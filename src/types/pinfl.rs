use serde::{Deserialize, Serialize};

use crate::error::{MyIdError, MyIdResult};

/// Jismoniy shaxsning Yagona Identifikatsiya Raqami (PINFL).
///
/// O'zbekiston fuqarolarining 14 xonali identifikatsiya raqami.
///
/// # Format
///
/// - Aynan **14 ta** ASCII raqam
/// - Harflar, bo'shliqlar yoki maxsus belgilar qabul qilinmaydi
///
/// # Misollar
///
/// ```
/// use myid::types::Pinfl;
///
/// let p = Pinfl::parse("12345678901234").unwrap();
/// assert_eq!(p.as_str(), "12345678901234");
///
/// assert!(Pinfl::parse("123456789012").is_err());   // qisqa
/// assert!(Pinfl::parse("1234567890123a").is_err()); // harf bor
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Pinfl(String);

impl Pinfl {
    /// PINFL uzunligi — aynan 14 raqam.
    pub const LEN: usize = 14;

    /// String qiymatdan `Pinfl` yaratadi.
    ///
    /// # Xatolar
    ///
    /// [`MyIdError::Validation`] qaytaradi agar:
    /// - Uzunlik 14 ta belgidan farqli bo'lsa
    /// - Raqamdan boshqa belgi mavjud bo'lsa
    pub fn parse(value: impl AsRef<str>) -> MyIdResult<Self> {
        let pinfl = value.as_ref().trim();

        if pinfl.len() != Self::LEN {
            return Err(MyIdError::validation(format!(
                "pinfl must be exactly {} digits, got {} chars: {pinfl}",
                Self::LEN,
                pinfl.len(),
            )));
        }

        if !pinfl.bytes().all(|b| b.is_ascii_digit()) {
            return Err(MyIdError::validation(format!(
                "pinfl must contain only digits, got: {pinfl}"
            )));
        }

        Ok(Self(pinfl.to_owned()))
    }

    /// PINFL qiymatini `&str` sifatida qaytaradi.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Pinfl {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Pinfl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<String> for Pinfl {
    type Error = MyIdError;

    fn try_from(value: String) -> MyIdResult<Self> {
        Self::parse(value)
    }
}

impl From<Pinfl> for String {
    fn from(value: Pinfl) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::Pinfl;

    #[test]
    fn valid_pinfl() {
        let p = Pinfl::parse("12345678901234").unwrap();
        assert_eq!(p.as_str(), "12345678901234");
    }

    #[test]
    fn whitespace_trimmed() {
        let p = Pinfl::parse("  12345678901234  ").unwrap();
        assert_eq!(p.as_str(), "12345678901234");
    }

    #[test]
    fn too_short_rejected() {
        assert!(Pinfl::parse("123").is_err());
        assert!(Pinfl::parse("1234567890123").is_err()); // 13
    }

    #[test]
    fn too_long_rejected() {
        assert!(Pinfl::parse("123456789012345").is_err()); // 15
    }

    #[test]
    fn non_digit_rejected() {
        assert!(Pinfl::parse("1234567890123a").is_err());
        assert!(Pinfl::parse("1234 678901234").is_err());
        assert!(Pinfl::parse("1234567890123+").is_err());
    }

    #[test]
    fn empty_rejected() {
        assert!(Pinfl::parse("").is_err());
        assert!(Pinfl::parse("   ").is_err());
    }

    #[test]
    fn display_and_as_ref() {
        let p = Pinfl::parse("12345678901234").unwrap();
        assert_eq!(p.to_string(), "12345678901234");
        assert_eq!(p.as_ref(), "12345678901234");
    }

    #[test]
    fn serde_round_trip() {
        let p = Pinfl::parse("12345678901234").unwrap();
        let json = serde_json::to_string(&p).unwrap();
        assert_eq!(json, "\"12345678901234\"");

        let back: Pinfl = serde_json::from_str(&json).unwrap();
        assert_eq!(p, back);
    }

    #[test]
    fn serde_invalid_rejected() {
        assert!(serde_json::from_str::<Pinfl>("\"123\"").is_err());
        assert!(serde_json::from_str::<Pinfl>("12345678901234").is_err()); // raqam, string emas
    }
}
