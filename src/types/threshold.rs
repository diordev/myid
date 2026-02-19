use serde::{Deserialize, Serialize};

use crate::error::{MyIdError, MyIdResult};

/// Face matching threshold qiymatini ifodalaydi (Newtype pattern).
///
/// Foydalanuvchi yuzini solishtirish aniqlik darajasini belgilaydi.
/// Qiymat `0.5` dan `0.99` gacha bo'lishi kerak.
///
/// # Chegaralar
///
/// | Const | Qiymat | Tavsif |
/// |-------|--------|--------|
/// | [`MIN`](Self::MIN) | `0.5` | Eng past aniqlik |
/// | [`MAX`](Self::MAX) | `0.99` | Eng yuqori aniqlik |
/// | [`DEFAULT`](Self::DEFAULT) | `0.75` | O'rtacha aniqlik |
///
/// # Misollar
///
/// ```
/// use myid::types::Threshold;
///
/// // parse orqali
/// let t = Threshold::parse(0.85).unwrap();
/// assert_eq!(t.as_f64(), 0.85);
///
/// // Default qiymat
/// assert_eq!(Threshold::DEFAULT.as_f64(), 0.75);
///
/// // Xato holatlar
/// assert!(Threshold::parse(0.49).is_err());
/// assert!(Threshold::parse(1.0).is_err());
/// assert!(Threshold::parse(f64::NAN).is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "f64", into = "f64")]
pub struct Threshold(f64);

impl Threshold {
    /// Minimal qiymat — `0.5`.
    pub const MIN: f64 = 0.5;

    /// Maksimal qiymat — `0.99`.
    pub const MAX: f64 = 0.99;

    /// Default qiymat — `0.75` (o'rtacha aniqlik).
    ///
    /// ```
    /// use myid::types::Threshold;
    /// assert_eq!(Threshold::DEFAULT.as_f64(), 0.75);
    /// ```
    pub const DEFAULT: Self = Self(0.75);

    /// `f64` qiymatdan `Threshold` yaratadi.
    ///
    /// # Xatolar
    ///
    /// [`MyIdError::Validation`] qaytaradi agar:
    /// - Qiymat `NaN` yoki `Infinity` bo'lsa
    /// - Qiymat `0.5..=0.99` diapazondan tashqarida bo'lsa
    pub fn parse(value: f64) -> MyIdResult<Self> {
        if !value.is_finite() {
            return Err(MyIdError::validation(format!(
                "threshold must be a finite number, got: {value}"
            )));
        }

        if !(Self::MIN..=Self::MAX).contains(&value) {
            return Err(MyIdError::validation(format!(
                "threshold must be between {} and {}, got: {value}",
                Self::MIN,
                Self::MAX,
            )));
        }

        Ok(Self(value))
    }

    /// Ichki `f64` qiymatni qaytaradi.
    pub fn as_f64(self) -> f64 {
        self.0
    }
}

impl std::fmt::Display for Threshold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<f64> for Threshold {
    type Error = MyIdError;

    fn try_from(value: f64) -> MyIdResult<Self> {
        Self::parse(value)
    }
}

impl From<Threshold> for f64 {
    fn from(value: Threshold) -> Self {
        value.0
    }
}
