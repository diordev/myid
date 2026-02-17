use std::borrow::Cow;
use std::fmt;
use std::time::Duration;

/// TCP/TLS ulanish uchun default timeout — 2 soniya.
pub const DEFAULT_CONNECT_TIMEOUT_MS: u64 = 2_000;

/// Butun HTTP so'rov uchun default timeout — 15 soniya.
pub const DEFAULT_TIMEOUT_MS: u64 = 15_000;

/// Default User-Agent sarlavhasi.
pub(crate) const DEFAULT_USER_AGENT: &str = "myid-client-rust/0.1";

/// Environment o'zgaruvchilari uchun default prefiks.
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
    base_url: String,

    /// OAuth `client_id` — public identifikator.
    client_id: String,

    /// OAuth `client_secret` — faqat backend muhitida saqlanishi kerak.
    ///
    /// Debug output'da `<redacted>` sifatida ko'rsatiladi.
    client_secret: String,

    /// TCP/TLS ulanish bosqichi uchun connection timeout milsekundlarda.
    ///
    /// Default: 2 soniya. `with_connect_timeout_ms()` orqali o'zgartirish mumkin.
    connection_timeout_ms: Duration,

    /// HTTP so'rov uchun timeout milsekundlarda.
    ///
    /// Default: 15 soniya. `with_timeout_ms()` orqali o'zgartirish mumkin.
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
    proxy_url: Option<String>,
}

impl Config {
    #[must_use]
    pub fn new<T: Into<String>>(base_url: T, client_id: T, client_secret: T) -> Self {
        Self {
            base_url: base_url.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            connection_timeout_ms: Duration::from_millis(DEFAULT_CONNECT_TIMEOUT_MS),
            timeout_ms: Duration::from_millis(DEFAULT_TIMEOUT_MS),
            user_agent: Cow::Borrowed(DEFAULT_USER_AGENT),
            proxy_url: None,
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
