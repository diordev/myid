//! MyID SDK konfiguratsiya moduli.
//!
//! Ushbu modul MyID klientini ishga tushirish uchun kerak bo'ladigan
//! konfiguratsiyani boshqaradi. Barcha parametrlar [`Config::new()`] orqali
//! yaratiladi va `with_*()` metodlari bilan sozlanadi.
//!
//! # Arxitektura
//!
//! ```text
//! Config::new(base_url, client_id, client_secret)
//!     │
//!     ├── parse_url()  ← URL validatsiya (faqat http/https)
//!     ├── trailing slash normalizatsiya
//!     └── default qiymatlar (timeout, user-agent)
//!           │
//!           ├── .with_timeout()           ← ixtiyoriy
//!           ├── .with_connect_timeout()   ← ixtiyoriy
//!           ├── .with_user_agent()        ← ixtiyoriy
//!           └── .with_proxy()             ← ixtiyoriy
//! ```
//!
//! # Misollar
//!
//! ## Minimal konfiguratsiya
//!
//! ```rust
//! use myid::config::Config;
//! # use myid::error::MyIdResult;
//!
//! # fn main() -> MyIdResult<()> {
//! let config = Config::new(
//!     "https://myid.uz",
//!     "your_client_id",
//!     "your_client_secret",
//! )?;
//!
//! assert_eq!(config.base_url(), "https://myid.uz/");
//! assert_eq!(config.user_agent(), "myid-client-rust/0.1");
//! # Ok(())
//! # }
//! ```
//!
//! ## To'liq konfiguratsiya
//!
//! ```rust
//! use std::time::Duration;
//! use myid::config::Config;
//! # use myid::error::MyIdResult;
//!
//! # fn main() -> MyIdResult<()> {
//! let config = Config::new("https://myid.uz", "client_id", "client_secret")?
//!     .with_timeout(Duration::from_secs(30))
//!     .with_connect_timeout(Duration::from_secs(5))
//!     .with_user_agent("my-backend/1.0")
//!     .with_proxy("http://proxy.corp.local:8080")?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Xato holatlari
//!
//! ```rust
//! use myid::config::Config;
//!
//! // Noto'g'ri URL — xato qaytaradi
//! assert!(Config::new("not-a-url", "id", "secret").is_err());
//!
//! // FTP scheme — faqat http/https qabul qilinadi
//! assert!(Config::new("ftp://example.uz", "id", "secret").is_err());
//! ```

use std::borrow::Cow;
use std::fmt;
use std::time::Duration;
use url::Url;

use crate::error::{MyIdError, MyIdResult};

/// TCP/TLS ulanish uchun default timeout — **2 soniya** (2000 ms).
///
/// Agar server 2 soniya ichida TCP/TLS handshake'ni tugatmasa,
/// ulanish bekor qilinadi. [`Config::with_connect_timeout()`] orqali o'zgartirish mumkin.
pub const DEFAULT_CONNECT_TIMEOUT_MS: u64 = 2_000;

/// Butun HTTP so'rov uchun default timeout — **15 soniya** (15000 ms).
///
/// Bu vaqt ichida server javob bermasa, so'rov bekor qilinadi.
/// [`Config::with_timeout()`] orqali o'zgartirish mumkin.
pub const DEFAULT_TIMEOUT_MS: u64 = 15_000;

/// Default User-Agent sarlavhasi.
///
/// HTTP so'rovlarda `User-Agent` header sifatida yuboriladi.
/// Observability va diagnostika uchun ishlatiladi.
pub(crate) const DEFAULT_USER_AGENT: &str = "myid-client-rust/0.1";

/// Environment o'zgaruvchilari uchun default prefiks.
///
/// Kelajakda `Config::from_env()` metodi uchun ishlatiladi.
/// Masalan: `MYID_CLIENT_ID`, `MYID_CLIENT_SECRET`.
#[allow(dead_code)]
pub(crate) const DEFAULT_PREFIX: &str = "MYID_";

// Compile-time kafolat: Config xavfsiz tarzda threadlar orasida
// share qilinishi mumkin. Agar kelajakda `Rc` yoki boshqa
// `!Send` tur qo'shilsa, kompilatsiya xato beradi.
const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Config>();
};

/// MyID SDK ning asosiy konfiguratsiya strukturasi.
///
/// `Config` MyID API bilan ishlash uchun kerakli barcha parametrlarni
/// o'z ichiga oladi: API URL, OAuth credential'lar, timeout'lar,
/// va ixtiyoriy proxy sozlamalari.
///
/// # Yaratish
///
/// `Config` faqat [`Config::new()`] orqali yaratiladi. 3 ta majburiy
/// parametr talab qilinadi, qolganlari sensible default qiymatlarga ega:
///
/// ```rust
/// # use myid::config::Config;
/// # use myid::error::MyIdResult;
/// # fn main() -> MyIdResult<()> {
/// let config = Config::new("https://myid.uz", "client_id", "secret")?;
/// # Ok(())
/// # }
/// ```
///
/// # Ixtiyoriy parametrlar
///
/// `with_*()` metodlari orqali chaining pattern bilan sozlanadi:
///
/// | Metod | Default qiymat | Tavsif |
/// |-------|---------------|--------|
/// | [`with_timeout()`](Config::with_timeout) | 15 soniya | HTTP so'rov timeout |
/// | [`with_connect_timeout()`](Config::with_connect_timeout) | 2 soniya | TCP/TLS ulanish timeout |
/// | [`with_user_agent()`](Config::with_user_agent) | `myid-client-rust/0.1` | HTTP User-Agent header |
/// | [`with_proxy()`](Config::with_proxy) | `None` | Outbound HTTP/HTTPS proxy |
///
/// # Thread-safety
///
/// `Config` `Send + Sync` traitlarini implement qiladi va
/// xavfsiz tarzda threadlar orasida share qilinishi mumkin.
/// Bu compile-time'da kafolatlanadi.
///
/// # Xavfsizlik
///
/// - `client_secret` [`Debug`] output'da `<redacted>` sifatida ko'rsatiladi
/// - Secret faqat [`Config::client_secret()`] orqali olinadi
/// - Production'da secret'ni environment variable orqali bering, kodni hardcode qilmang
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

    /// TCP/TLS ulanish bosqichi uchun connection timeout.
    ///
    /// Default: 2 soniya. `with_connect_timeout()` orqali o'zgartirish mumkin.
    connection_timeout_ms: Duration,

    /// HTTP so'rov uchun timeout.
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
    /// Yangi `Config` instansini yaratadi.
    ///
    /// 3 ta majburiy parametr talab qilinadi. Qolgan barcha parametrlar
    /// default qiymatlarga ega va `with_*()` metodlari orqali o'zgartirilishi mumkin.
    ///
    /// # Parametrlar
    ///
    /// - `base_url` — MyID API bazaviy URL (masalan: `https://myid.uz`).
    ///   Trailing slash avtomatik qo'shiladi. Faqat `http` va `https` qabul qilinadi.
    /// - `client_id` — OAuth 2.0 client identifikator (public).
    /// - `client_secret` — OAuth 2.0 client secret (**maxfiy**, faqat backend'da saqlang).
    ///
    /// # Xatolar
    ///
    /// [`MyIdError::Config`] qaytaradi agar:
    /// - `base_url` noto'g'ri URL formatida bo'lsa
    /// - URL scheme `http` yoki `https` dan farqli bo'lsa
    ///
    /// # Misollar
    ///
    /// ```rust
    /// use myid::config::Config;
    /// # use myid::error::MyIdResult;
    ///
    /// # fn main() -> MyIdResult<()> {
    /// // Minimal
    /// let config = Config::new("https://myid.uz", "app_id", "secret")?;
    /// assert_eq!(config.base_url(), "https://myid.uz/");
    ///
    /// // Trailing slash mavjud bo'lsa ham to'g'ri ishlaydi
    /// let config = Config::new("https://myid.uz/", "app_id", "secret")?;
    /// assert_eq!(config.base_url(), "https://myid.uz/");
    ///
    /// // Noto'g'ri URL xato qaytaradi
    /// assert!(Config::new("not-a-url", "id", "secret").is_err());
    /// # Ok(())
    /// # }
    /// ```
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

    /// TCP/TLS ulanish timeout'ini o'zgartiradi.
    ///
    /// Bu faqat ulanish bosqichi (TCP handshake + TLS negotiation) uchun.
    /// Server javob vaqti uchun [`Config::with_timeout()`] ishlatiladi.
    ///
    /// Default: **2 soniya**.
    ///
    /// # Misollar
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use myid::config::Config;
    /// # use myid::error::MyIdResult;
    ///
    /// # fn main() -> MyIdResult<()> {
    /// let config = Config::new("https://myid.uz", "id", "secret")?
    ///     .with_connect_timeout(Duration::from_secs(10));
    ///
    /// assert_eq!(config.connection_timeout(), Duration::from_secs(10));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout_ms = timeout;
        self
    }

    /// HTTP so'rov timeout'ini o'zgartiradi.
    ///
    /// Bu butun so'rov davomiyligi uchun — ulanish, yuborish va javob qabul qilish.
    /// Agar server shu vaqt ichida javob bermasa, so'rov bekor qilinadi.
    ///
    /// Default: **15 soniya**.
    ///
    /// # Misollar
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use myid::config::Config;
    /// # use myid::error::MyIdResult;
    ///
    /// # fn main() -> MyIdResult<()> {
    /// let config = Config::new("https://myid.uz", "id", "secret")?
    ///     .with_timeout(Duration::from_secs(60));
    ///
    /// assert_eq!(config.timeout(), Duration::from_secs(60));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout_ms = timeout;
        self
    }

    /// HTTP `User-Agent` sarlavhasini o'zgartiradi.
    ///
    /// `User-Agent` header har bir HTTP so'rovda yuboriladi.
    /// Server tomonida so'rovlarni identifikatsiya qilish va
    /// monitoring uchun foydali.
    ///
    /// Default: `myid-client-rust/0.1`.
    ///
    /// # Misollar
    ///
    /// ```rust
    /// use myid::config::Config;
    /// # use myid::error::MyIdResult;
    ///
    /// # fn main() -> MyIdResult<()> {
    /// let config = Config::new("https://myid.uz", "id", "secret")?
    ///     .with_user_agent("my-backend/2.0");
    ///
    /// assert_eq!(config.user_agent(), "my-backend/2.0");
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn with_user_agent(mut self, agent: impl Into<String>) -> Self {
        self.user_agent = Cow::Owned(agent.into());
        self
    }

    /// Outbound HTTP/HTTPS proxy URL'ni sozlaydi.
    ///
    /// Korporativ tarmoqlarda internet chiqish faqat proxy orqali
    /// bo'lishi mumkin. Bu holda shu metod orqali proxy URL beriladi.
    ///
    /// Faqat `http` va `https` scheme qabul qilinadi.
    ///
    /// # Xatolar
    ///
    /// [`MyIdError::Config`] qaytaradi agar:
    /// - URL noto'g'ri formatda bo'lsa
    /// - Scheme `http` yoki `https` dan farqli bo'lsa
    ///
    /// # Misollar
    ///
    /// ```rust
    /// use myid::config::Config;
    /// # use myid::error::MyIdResult;
    ///
    /// # fn main() -> MyIdResult<()> {
    /// let config = Config::new("https://myid.uz", "id", "secret")?
    ///     .with_proxy("http://proxy.corp.local:8080")?;
    ///
    /// assert!(config.proxy_url().is_some());
    ///
    /// // FTP proxy qabul qilinmaydi
    /// let result = Config::new("https://myid.uz", "id", "secret")?
    ///     .with_proxy("ftp://proxy.local");
    /// assert!(result.is_err());
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_proxy(mut self, url: impl AsRef<str>) -> MyIdResult<Self> {
        self.proxy_url = Some(Self::parse_url(&url)?);
        Ok(self)
    }

    // --- Getter methods ---

    /// API bazaviy URL'ni `&str` sifatida qaytaradi.
    ///
    /// Qaytariladigan URL har doim trailing slash (`/`) bilan tugaydi.
    ///
    /// # Misollar
    ///
    /// ```rust
    /// # use myid::config::Config;
    /// # use myid::error::MyIdResult;
    /// # fn main() -> MyIdResult<()> {
    /// let config = Config::new("https://myid.uz", "id", "secret")?;
    /// assert_eq!(config.base_url(), "https://myid.uz/");
    /// # Ok(())
    /// # }
    /// ```
    pub fn base_url(&self) -> &str {
        self.base_url.as_str()
    }

    /// OAuth `client_id` qiymatini qaytaradi.
    ///
    /// Bu public identifikator — logga chiqarish xavfsiz.
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// OAuth `client_secret` qiymatini qaytaradi.
    ///
    /// ⚠️ **Ogohlantirish:** bu qiymat maxfiy. Logga, stdout'ga yoki
    /// tashqi tizimlarga **chiqarmang**. [`Debug`] output'da avtomatik
    /// `<redacted>` sifatida ko'rsatiladi.
    pub fn client_secret(&self) -> &str {
        &self.client_secret
    }

    /// TCP/TLS ulanish timeout qiymatini qaytaradi.
    ///
    /// Default: 2 soniya. [`Config::with_connect_timeout()`] orqali o'zgartiriladi.
    pub fn connection_timeout(&self) -> Duration {
        self.connection_timeout_ms
    }

    /// HTTP so'rov timeout qiymatini qaytaradi.
    ///
    /// Default: 15 soniya. [`Config::with_timeout()`] orqali o'zgartiriladi.
    pub fn timeout(&self) -> Duration {
        self.timeout_ms
    }

    /// HTTP `User-Agent` header qiymatini qaytaradi.
    ///
    /// Default: `myid-client-rust/0.1`.
    pub fn user_agent(&self) -> &str {
        self.user_agent.as_ref()
    }

    /// Proxy URL'ni `&str` sifatida qaytaradi (agar o'rnatilgan bo'lsa).
    ///
    /// Proxy sozlanmagan bo'lsa `None` qaytaradi.
    ///
    /// # Misollar
    ///
    /// ```rust
    /// # use myid::config::Config;
    /// # use myid::error::MyIdResult;
    /// # fn main() -> MyIdResult<()> {
    /// // Proxy yo'q
    /// let config = Config::new("https://myid.uz", "id", "secret")?;
    /// assert_eq!(config.proxy_url(), None);
    ///
    /// // Proxy bor
    /// let config = Config::new("https://myid.uz", "id", "secret")?
    ///     .with_proxy("http://proxy:8080")?;
    /// assert!(config.proxy_url().is_some());
    /// # Ok(())
    /// # }
    /// ```
    pub fn proxy_url(&self) -> Option<&str> {
        self.proxy_url.as_ref().map(Url::as_str)
    }

    // --- Crate-internal methods ---

    /// API bazaviy URL'ni [`Url`] sifatida qaytaradi.
    ///
    /// Crate ichida endpoint yaratish uchun ishlatiladi:
    ///
    /// ```rust,ignore
    /// let endpoint = config.base_url_parsed().join("api/v1/verify")?;
    /// ```
    #[allow(dead_code)]
    pub(crate) fn base_url_parsed(&self) -> &Url {
        &self.base_url
    }

    /// Proxy URL'ni [`Url`] sifatida qaytaradi.
    ///
    /// Crate ichida HTTP client proxy sozlamalari uchun ishlatiladi.
    #[allow(dead_code)]
    pub(crate) fn proxy_url_parsed(&self) -> Option<&Url> {
        self.proxy_url.as_ref()
    }

    // --- Private methods ---

    /// URL stringni parse va validate qiladi.
    ///
    /// Faqat `http` va `https` scheme qabul qiladi.
    /// Boshqa schemalar (ftp, ws, va h.k.) rad etiladi.
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

/// [`Debug`] implementatsiyasi `client_secret` ni yashiradi.
///
/// Log yoki panic output'da credential'lar sizib chiqishining oldini oladi.
/// `client_id` ochiq ko'rsatiladi — bu public identifikator.
///
/// # Misol
///
/// ```rust
/// # use myid::config::Config;
/// # use myid::error::MyIdResult;
/// # fn main() -> MyIdResult<()> {
/// let config = Config::new("https://myid.uz", "my_app", "super_secret")?;
/// let debug = format!("{:?}", config);
///
/// // Secret ko'rinmaydi
/// assert!(debug.contains("<redacted>"));
/// assert!(!debug.contains("super_secret"));
///
/// // Client ID ko'rinadi
/// assert!(debug.contains("my_app"));
/// # Ok(())
/// # }
/// ```
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
