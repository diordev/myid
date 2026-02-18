//! MyID SDK konfiguratsiya moduli.
//!
//! Ushbu modul MyID klientini ishga tushirish uchun kerak bo'ladigan
//! konfiguratsiyani boshqaradi.

use std::borrow::Cow;
use std::fmt;
use std::time::Duration;
use url::Url;

use crate::error::{MyIdError, MyIdResult};

/// TCP/TLS ulanish uchun default timeout — 2 soniya.
pub const DEFAULT_CONNECT_TIMEOUT_MS: u64 = 2_000;

/// Butun HTTP so'rov uchun default timeout — 15 soniya.
pub const DEFAULT_TIMEOUT_MS: u64 = 15_000;

/// Default User-Agent sarlavhasi.
pub(crate) const DEFAULT_USER_AGENT: &str = "myid-client-rust/0.1";

/// Environment o'zgaruvchilari uchun default prefiks.
#[allow(dead_code)]
pub(crate) const DEFAULT_PREFIX: &str = "MYID_";

// Compile-time kafolat: Config xavfsiz tarzda threadlar orasida
// share qilinishi mumkin. Agar kelajakda `Rc` yoki boshqa
// `!Send` tur qo'shilsa, kompilatsiya xato beradi.
const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Config>();
};

#[derive(Clone)]
pub struct Config {
    /// API bazaviy URL. Trailing slash avtomatik qo'shiladi.
    ///
    /// Misol: `https://myid.example.uz/`
    /// Faqat `http` va `https` scheme qabul qilinadi.
    base_url: Url,

    /// OAuth `client_id` — public identifikator.
    client_id: String,

    /// OAuth `client_secret` — faqat backend muhitida saqlanishi kerak.
    ///
    /// Debug output'da `<redacted>` sifatida ko'rsatiladi.
    client_secret: String,

    /// TCP/TLS ulanish bosqichi uchun connection timeout milsekundlarda.
    ///
    /// Default: 2 soniya. `with_connect_timeout()` orqali o'zgartirish mumkin.
    connection_timeout_ms: Duration,

    /// HTTP so'rov uchun timeout milsekundlarda.
    ///
    /// Default: 15 soniya. `with_timeout()` orqali o'zgartirish mumkin.
    timeout_ms: Duration,

    /// HTTP `User-Agent` sarlavhasi — observability va diagnostika uchun.
    ///
    /// Default qiymatda heap allokatsiya bo'lmaydi (`Cow::Borrowed`).
    /// Custom qiymat berilsa `Cow::Owned` ga o'tadi.
    user_agent: Cow<'static, str>,

    /// Ixtiyoriy outbound HTTP/HTTPS proxy URL.
    ///
    /// Agar korporativ tarmoqda proxy orqali chiqish kerak bo'lsa ishlatiladi.
    /// Faqat `http` va `https` scheme qabul qilinadi.
    proxy_url: Option<Url>,
}

impl Config {
    pub fn new(
        base_url: impl AsRef<str>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> MyIdResult<Self> {
        let mut url = Self::parse_url(&base_url)?;

        if !url.path().ends_with('/') {
            url.set_path(&format!("{}/", url.path()));
        }
        Ok(Self {
            base_url: url,
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            connection_timeout_ms: Duration::from_millis(DEFAULT_CONNECT_TIMEOUT_MS),
            timeout_ms: Duration::from_millis(DEFAULT_TIMEOUT_MS),
            user_agent: Cow::Borrowed(DEFAULT_USER_AGENT),
            proxy_url: None,
        })
    }

    // --- Builder methods (with_*) ---

    /// Connection timeout o'zgartirish
    #[must_use]
    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout_ms = timeout;
        self
    }

    ///Request timeout o'zgartirish
    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout_ms = timeout;
        self
    }

    /// Custom User-Agent
    #[must_use]
    pub fn with_user_agent(mut self, agent: impl Into<String>) -> Self {
        self.user_agent = Cow::Owned(agent.into());
        self
    }

    /// Proxy URL
    pub fn with_proxy(mut self, url: impl AsRef<str>) -> MyIdResult<Self> {
        self.proxy_url = Some(Self::parse_url(&url)?);
        Ok(self)
    }

    // --- Getter methods ---

    /// Getter — consumer API bazaviy URL'ini qaytaradi.
    pub fn base_url(&self) -> &str {
        self.base_url.as_str()
    }

    /// OAuth `client_id` qiymatini qaytaradi.
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// OAuth `client_secret` qiymatini qaytaradi.
    ///
    /// Eslatma: bu qiymat maxfiy bo'lgani uchun logga chiqarmang.
    pub fn client_secret(&self) -> &str {
        &self.client_secret
    }

    /// TCP/TLS ulanish timeout'ini qaytaradi.
    pub fn connection_timeout(&self) -> Duration {
        self.connection_timeout_ms
    }

    /// Butun so'rov timeout'ini qaytaradi.
    pub fn timeout(&self) -> Duration {
        self.timeout_ms
    }

    /// HTTP `User-Agent` qiymatini qaytaradi.
    pub fn user_agent(&self) -> &str {
        self.user_agent.as_ref()
    }

    /// Outbound proxy URL'ni qaytaradi (agar o'rnatilgan bo'lsa).
    pub fn proxy_url(&self) -> Option<&str> {
        self.proxy_url.as_ref().map(Url::as_str)
    }

    // ---Crate ichida ishlatiladigan metodlar---

    /// Crate ichida — endpoint join uchun
    pub(crate) fn base_url_parsed(&self) -> &Url {
        &self.base_url
    }

    /// Crate ichida — Url kerak bo'lganda
    pub(crate) fn proxy_url_parsed(&self) -> Option<&Url> {
        self.proxy_url.as_ref()
    }

    // ---Private methods---

    /// URL stringni parse va validate qiladi.
    /// Faqat `http` va `https` scheme qabul qiladi.
    fn parse_url(raw: impl AsRef<str>) -> MyIdResult<Url> {
        let url = Url::parse(raw.as_ref())
            .map_err(|e| MyIdError::config(format!("invalid URL `{}`: {e}", raw.as_ref())))?;

        match url.scheme() {
            "http" | "https" => Ok(url),
            other => Err(MyIdError::config(format!(
                "only http/https are accepted, given: {other}"
            ))),
        }
    }
}

/// `Debug` implementatsiyasi `client_secret` ni yashiradi.
///
/// Log yoki panic output'da credential'lar sizib chiqishining oldini oladi.
/// `client_id` ochiq ko'rsatiladi — bu public identifikator.
impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("base_url", &self.base_url.as_str())
            .field("client_id", &self.client_id.as_str())
            .field("client_secret", &"<redacted>")
            .field("connection_timeout_ms", &self.connection_timeout_ms)
            .field("timeout_ms", &self.timeout_ms)
            .field("user_agent", &self.user_agent.as_ref())
            .field("proxy_url", &self.proxy_url.as_ref().map(|p| p.as_str()))
            .finish()
    }
}
