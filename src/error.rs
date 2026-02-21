//! MyID SDK xatoliklarni boshqarish moduli.
//!
//! Modul ikki asosiy tipdan iborat:
//!
//! - [`MyIdError`] — SDK ning yagona xatolik tipi. Barcha xatolar
//!   shu enum orqali qaytariladi.
//! - [`MyIdResult<T>`] — `Result<T, MyIdError>` uchun qulay type alias.
//!
//! # Xato kategoriyalari
//!
//! | Variant | Sababi | Misol |
//! |---------|--------|-------|
//! | [`Config`](MyIdError::Config) | Konfiguratsiya xatosi | Noto'g'ri URL, yo'q env var |
//! | [`Http`](MyIdError::Http) | Transport xatosi | Timeout, DNS, TLS |
//! | [`Api`](MyIdError::Api) | API 4xx/5xx javobi | 401 Unauthorized, 403 Forbidden |
//! | [`Internal`](MyIdError::Internal) | SDK ichki xatosi | Lock poisoning |
//!
//! # Ishlatish
//!
//! ```rust
//! use myid::config::Config;
//! use myid::error::{MyIdError, MyIdResult};
//!
//! fn create_config() -> MyIdResult<()> {
//!     let config = Config::new("https://myid.uz", "id", "secret")?;
//!     Ok(())
//! }
//!
//! // Xato turini tekshirish
//! match Config::new("noto'g'ri", "id", "secret") {
//!     Ok(_) => unreachable!(),
//!     Err(MyIdError::Config { message }) => {
//!         eprintln!("Config xatosi: {message}");
//!     }
//!     Err(e) => eprintln!("Boshqa xato: {e}"),
//! }
//! ```

/// MyID SDK ning asosiy xatolik tipi.
///
/// Barcha SDK operatsiyalari xato bo'lganda shu enum qaytariladi.
/// Har bir variant alohida xato kategoriyasini ifodalaydi.
///
/// # Misollar
///
/// ```rust
/// use myid::error::MyIdError;
///
/// // Config xatosi
/// let err = MyIdError::config("noto'g'ri URL formati");
/// println!("{err}"); // "config error: noto'g'ri URL formati"
///
/// // API xatosi
/// let err = MyIdError::api(401, "unauthorized");
/// println!("{err}"); // "api error 401: unauthorized"
/// ```
/// 
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum MyIdError {
    /// Konfiguratsiya xatosi.
    ///
    /// [`Config::new()`](crate::config::Config::new),
    /// [`Config::from_env()`](crate::config::Config::from_env), yoki
    /// `with_*()` metodlarida validatsiya muvaffaqiyatsiz bo'lganda qaytariladi.
    ///
    /// # Misol
    ///
    /// ```rust
    /// use myid::config::Config;
    /// use myid::error::MyIdError;
    ///
    /// let err = Config::new("ftp://example.uz", "id", "secret").unwrap_err();
    /// assert!(matches!(err, MyIdError::Config { .. }));
    /// ```
    #[error("config error: {message}")]
    Config {
        /// Xato haqida batafsil ma'lumot.
        message: String,
    },

    /// Ma'lumotlarni validatsiya xatosi.
    ///
    /// So'rov parametrlari noto'g'ri yoki yetishmagan bo'lganda qaytariladi.
    /// Masalan: majburiy field bo'sh, format noto'g'ri, qiymat chegaradan tashqarida.
    ///
    /// # Qachon yuz beradi
    ///
    /// | Sabab | Misol |
    /// |-------|-------|
    /// | Majburiy field bo'sh | `photo` berilmagan |
    /// | Format noto'g'ri | `birth_date` noto'g'ri format |
    /// | Qiymat chegaradan tashqari | `passport_series` 2 belgidan kam |
    ///
    /// # Misol
    ///
    /// ```rust
    /// use myid::error::MyIdError;
    ///
    /// let err = MyIdError::validation("photo maydoni majburiy");
    /// assert!(matches!(err, MyIdError::Validation { .. }));
    /// println!("{err}"); // "validation error: photo maydoni majburiy"
    /// ```
    #[error("validation error: {message}")]
    Validation {
        /// Qaysi field yoki qiymat noto'g'ri ekanligi haqida batafsil ma'lumot.
        message: String,
    },

    /// HTTP transport xatosi.
    ///
    /// `reqwest` kutubxonasi qaytargan xatolar avtomatik convert bo'ladi.
    /// Timeout, DNS resolution, TLS handshake, connection refused va
    /// boshqa tarmoq xatoliklari shu variantga tushadi.
    ///
    /// # Qachon yuz beradi
    ///
    /// - Server javob bermasa (timeout)
    /// - DNS nomi topilmasa
    /// - TLS sertifikat xatosi
    /// - Connection refused
    /// - Response body parse bo'lmasa
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// API javobidagi xato (4xx/5xx status kodlari).
    ///
    /// MyID serveri so'rovni qabul qildi, lekin xato javob qaytardi.
    /// `status` — HTTP status kodi, `message` — server javobi.
    ///
    /// # Tez-tez uchraydigan kodlar
    ///
    /// | Status | Sababi |
    /// |--------|--------|
    /// | 400 | Noto'g'ri so'rov parametrlari |
    /// | 401 | `client_id` yoki `client_secret` noto'g'ri |
    /// | 403 | Ruxsat berilmagan |
    /// | 429 | Rate limit — juda ko'p so'rov |
    /// | 500 | Server ichki xatosi |
    #[error("api error {status}: {message}")]
    Api {
        /// HTTP status kodi (masalan: 401, 403, 500).
        status: u16,
        /// Server qaytargan xato xabari.
        message: String,
    },

    /// SDK ichki xatosi.
    ///
    /// Odatiy foydalanishda yuz bermasligi kerak.
    /// Mutex lock poisoning yoki boshqa kutilmagan holatlar uchun.
    #[error("internal error: {message}")]
    Internal {
        /// Ichki xato haqida ma'lumot.
        message: String,
    },
}

impl MyIdError {
    /// [`MyIdError::Config`] variantini yaratadi.
    ///
    /// # Misollar
    ///
    /// ```rust
    /// use myid::error::MyIdError;
    ///
    /// let err = MyIdError::config("noto'g'ri URL");
    /// let err = MyIdError::config(format!("yo'q: {}", "MYID_BASE_URL"));
    /// ```    
    #[inline]
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// [`MyIdError::Validation`] variantini yaratadi.
    ///
    /// ```rust
    /// use myid::error::MyIdError;
    ///
    /// let err = MyIdError::validation("passport_series majburiy");
    /// ```
    #[inline]
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    /// [`MyIdError::Http`] variantini `reqwest::Error` dan yaratadi.
    ///
    /// Odatda `?` operatori orqali avtomatik convert bo'ladi (`#[from]`).
    /// Bu metod faqat qo'shimcha kontekst kerak bo'lganda ishlatiladi.
    pub fn http(source: reqwest::Error) -> Self {
        Self::Http(source)
    }

    /// [`MyIdError::Api`] variantini yaratadi.
    ///
    /// # Misollar
    ///
    /// ```rust
    /// use myid::error::MyIdError;
    ///
    /// let err = MyIdError::api(401, "unauthorized");
    /// let err = MyIdError::api(500, "internal server error");
    /// ```
    #[inline]
    pub fn api(status: u16, message: impl Into<String>) -> Self {
        Self::Api {
            status,
            message: message.into(),
        }
    }

    /// [`MyIdError::Internal`] variantini yaratadi.
    ///
    /// Faqat SDK ichida ishlatiladi — consumer uchun mo'ljallanmagan.
    #[inline]
    pub(crate) fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// Xato API xatosimi yoki yo'qligini tekshiradi.
    ///
    /// ```rust
    /// use myid::error::MyIdError;
    ///
    /// let err = MyIdError::api(401, "unauthorized");
    /// assert!(err.is_api());
    /// ```
    #[inline]
    pub fn is_api(&self) -> bool {
        matches!(self, Self::Api { .. })
    }

    /// API xato bo'lsa status kodini qaytaradi.
    ///
    /// ```rust
    /// use myid::error::MyIdError;
    ///
    /// let err = MyIdError::api(429, "rate limited");
    /// assert_eq!(err.api_status(), Some(429));
    /// ```
    #[inline]
    pub fn api_status(&self) -> Option<u16> {
        match self {
            Self::Api { status, .. } => Some(*status),
            _ => None,
        }
    }
}

/// MyID SDK ning standart `Result` tipi.
///
/// `Result<T, MyIdError>` uchun qulay alias. SDK ning barcha
/// public metodlari shu tipni qaytaradi.
///
/// # Misollar
///
/// ```rust
/// use myid::error::MyIdResult;
///
/// fn do_something() -> MyIdResult<String> {
///     Ok("muvaffaqiyatli".to_string())
/// }
/// ```
pub type MyIdResult<T> = Result<T, MyIdError>;
