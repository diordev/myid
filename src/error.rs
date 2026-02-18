//! MyID SDK xatoliklarni boshqarish moduli.
//!
//! Modul ikki asosiy tipdan iborat:
//!
//! - [`MyIdError`] — SDK ning yagona xatolik tipi. Barcha xatolar
//!   shu enum orqali qaytariladi.
//! - [`MyIdResult<T>`] — `Result<T, MyIdError>` uchun qulay type alias.
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
//!         println!("Config xatosi: {message}");
//!     }
//! }
//! ```

/// MyID SDK ning asosiy xatolik tipi.
///
/// Barcha SDK operatsiyalari xato bo'lganda shu enum qaytariladi.
/// Har bir variant alohida xato kategoriyasini ifodalaydi.
///
/// # Variantlar
///
/// | Variant | Sababi |
/// |---------|--------|
/// | [`Config`](MyIdError::Config) | Noto'g'ri URL, yo'q maydon, validatsiya xatosi |
///
/// # Misollar
///
/// ```rust
/// use myid::error::MyIdError;
///
/// let err = MyIdError::config("noto'g'ri URL formati");
/// println!("{err}"); // "config error: noto'g'ri URL formati"
/// ```
#[derive(Debug, thiserror::Error)]
pub enum MyIdError {
    /// Konfiguratsiya xatosi.
    ///
    /// [`Config::new()`](crate::config::Config::new) yoki
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
}

impl MyIdError {
    /// [`MyIdError::Config`] variantini yaratish uchun helper metod.
    ///
    /// `impl Into<String>` qabul qiladi — `&str` ham, `String` ham,
    /// `format!()` natijasi ham to'g'ridan-to'g'ri berilishi mumkin.
    ///
    /// # Misollar
    ///
    /// ```rust
    /// use myid::error::MyIdError;
    ///
    /// // &str bilan
    /// let err = MyIdError::config("noto'g'ri URL");
    ///
    /// // format! bilan
    /// let url = "ftp://example.uz";
    /// let err = MyIdError::config(format!("ruxsat etilmagan scheme: {url}"));
    /// ```
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
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