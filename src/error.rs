//! MyID SDK xatoliklarni boshqarish moduli.
//!
//! Modul ikki asosiy tipdan iborat:
//! - [`MyIdError`] — MyID SDK ning asosiy error tipi (`thiserror::Error`)
//! - [`MyIdResult`] — MyID SDK ning asosiy `Result` tipi (`MyIdError`)

/// MyID SDK xatoliklarni boshqarish moduli.
#[derive(Debug, thiserror::Error)]
pub enum MyIdError {
    #[error("config error at{message}")]
    Config { message: String },
}

impl MyIdError {
    #[must_use]
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }
}

/// MyID SDK ning asosiy standart `Result` alias.
pub type MyIdResult<T, E = MyIdError> = Result<T, E>;
