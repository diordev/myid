//! MyID SDK xatoliklarni boshqarish moduli.
//!
//! Modul ikki asosiy tipdan iborat:
//! - [`MyIdError`] — MyID SDK ning asosiy error tipi (`thiserror::Error`)
//! - [`MyIdResult`] — MyID SDK ning asosiy `Result` tipi (`MyIdError`)

/// MyID SDK ning asosiy xatolik tipi.
#[derive(Debug, thiserror::Error)]
pub enum MyIdError {
    /// Konfiguratsiya xatosi.
    #[error("config error: {message}")]
    Config { message: String },
}

impl MyIdError {
    /// Config xatosi yaratish uchun helper.
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }
}

/// MyID SDK ning standart `Result` tipi.
pub type MyIdResult<T> = Result<T, MyIdError>;
