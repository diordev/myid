use serde::{Deserialize, Serialize};

use crate::error::{MyIdError, MyIdResult};

/// O'zbekiston telefon raqami.
///
/// Ichki saqlashda faqat raqamlar saqlanadi (`+` olib tashlanadi).
/// Bu normalizatsiya API ga yuborishda izchillikni ta'minlaydi.
///
/// # Format
///
/// - Kirish: `+998901234567` yoki `998901234567`
/// - Ichki saqlash: `998901234567` (har doim 12 raqam)
///
/// # Misollar
///
/// ```
/// use myid::types::PhoneNumber;
///
/// // + bilan va + siz — ikkalasi ham qabul qilinadi
/// let p1 = PhoneNumber::parse("+998901234567").unwrap();
/// let p2 = PhoneNumber::parse("998901234567").unwrap();
///
/// // Ichki saqlashda + yo'q
/// assert_eq!(p1.as_str(), "998901234567");
/// assert_eq!(p1, p2);
///
/// assert!(PhoneNumber::parse("123").is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PhoneNumber(String);

impl PhoneNumber {
    /// Telefon raqam uzunligi (`+` siz) — aynan 12 raqam.
    pub const DIGIT_LEN: usize = 12;

    /// String qiymatdan `PhoneNumber` yaratadi.
    ///
    /// Boshidagi `+` belgisi qabul qilinadi va olib tashlanadi.
    ///
    /// # Xatolar
    ///
    /// [`MyIdError::Validation`] qaytaradi agar:
    /// - Raqamlar soni 12 ta bo'lmasa
    /// - Raqamdan boshqa belgi mavjud bo'lsa (`+` dan tashqari)
    pub fn parse(value: impl AsRef<str>) -> MyIdResult<Self> {
        let raw = value.as_ref().trim();
        let digits = raw.strip_prefix('+').unwrap_or(raw);

        if digits.len() != Self::DIGIT_LEN {
            return Err(MyIdError::validation(format!(
                "phone number must be exactly {} digits, got {}: {raw}",
                Self::DIGIT_LEN,
                digits.len(),
            )));
        }

        if !digits.bytes().all(|b| b.is_ascii_digit()) {
            return Err(MyIdError::validation(format!(
                "phone number must contain only digits, got: {raw}"
            )));
        }

        Ok(Self(digits.to_owned()))
    }

    /// Raqamni `&str` sifatida qaytaradi (`+` siz, faqat 12 raqam).
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Raqamni `+` bilan qaytaradi (masalan: `+998901234567`).
    #[inline]
    pub fn as_international(&self) -> String {
        format!("+{}", self.0)
    }
}

impl AsRef<str> for PhoneNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+{}", self.0)
    }
}

impl TryFrom<String> for PhoneNumber {
    type Error = MyIdError;

    fn try_from(value: String) -> MyIdResult<Self> {
        Self::parse(value)
    }
}

impl From<PhoneNumber> for String {
    fn from(value: PhoneNumber) -> Self {
        value.0
    }
}
