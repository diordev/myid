use serde::{Deserialize, Serialize};

use crate::error::{MyIdError, MyIdResult};

/// O'zbekiston pasport seriya va raqami.
///
/// # Format
///
/// - Aynan **9 ta** belgi: **2 katta lotin harfi** + **7 raqam**
/// - Ichki saqlashda seriya uppercase ga normalizatsiya qilinadi
///
/// # Misollar
///
/// ```
/// use myid::types::PassportData;
///
/// let p = PassportData::parse("AA1234567").unwrap();
/// assert_eq!(p.as_str(), "AA1234567");
/// assert_eq!(p.series(), "AA");
/// assert_eq!(p.number(), "1234567");
///
/// // Kichik harf avtomatik uppercase bo'ladi
/// let p = PassportData::parse("ab1234567").unwrap();
/// assert_eq!(p.as_str(), "AB1234567");
///
/// assert!(PassportData::parse("A1234567").is_err());  // qisqa
/// assert!(PassportData::parse("123456789").is_err());  // harf yo'q
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PassportData(String);

impl PassportData {
    /// Umumiy uzunlik: 2 harf + 7 raqam = 9.
    pub const LEN: usize = 9;
    /// Seriya uzunligi — 2 harf.
    pub const SERIES_LEN: usize = 2;
    /// Raqam uzunligi — 7 raqam.
    pub const NUMBER_LEN: usize = 7;

    /// String qiymatdan `PassportData` yaratadi.
    ///
    /// Kichik harflar avtomatik uppercase ga o'tkaziladi.
    ///
    /// # Xatolar
    ///
    /// [`MyIdError::Validation`] qaytaradi agar:
    /// - Uzunlik 9 ta belgidan farqli bo'lsa
    /// - Birinchi 2 belgi lotin harfi bo'lmasa
    /// - Oxirgi 7 belgi raqam bo'lmasa
    pub fn parse(value: impl AsRef<str>) -> MyIdResult<Self> {
        let raw = value.as_ref().trim();

        if raw.len() != Self::LEN {
            return Err(MyIdError::validation(format!(
                "passport must be exactly {} chars (2 letters + 7 digits), got {}: {raw}",
                Self::LEN,
                raw.len(),
            )));
        }

        let bytes = raw.as_bytes();

        if !bytes[..Self::SERIES_LEN]
            .iter()
            .all(|b| b.is_ascii_alphabetic())
        {
            return Err(MyIdError::validation(format!(
                "passport series must be 2 latin letters, got: {raw}"
            )));
        }

        if !bytes[Self::SERIES_LEN..].iter().all(|b| b.is_ascii_digit()) {
            return Err(MyIdError::validation(format!(
                "passport number must be 7 digits, got: {raw}"
            )));
        }

        // Uppercase normalizatsiya: "ab1234567" → "AB1234567"
        Ok(Self(
            raw[..Self::SERIES_LEN].to_ascii_uppercase() + &raw[Self::SERIES_LEN..],
        ))
    }

    /// To'liq pasport ma'lumotini qaytaradi (masalan: `AB1234567`).
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Faqat seriya qismini qaytaradi (masalan: `AB`).
    pub fn series(&self) -> &str {
        &self.0[..Self::SERIES_LEN]
    }

    /// Faqat raqam qismini qaytaradi (masalan: `1234567`).
    pub fn number(&self) -> &str {
        &self.0[Self::SERIES_LEN..]
    }
}

impl AsRef<str> for PassportData {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for PassportData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<String> for PassportData {
    type Error = MyIdError;

    fn try_from(value: String) -> MyIdResult<Self> {
        Self::parse(value)
    }
}

impl From<PassportData> for String {
    fn from(value: PassportData) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::PassportData;

    #[test]
    fn valid_passport() {
        let p = PassportData::parse("AA1234567").unwrap();
        assert_eq!(p.as_str(), "AA1234567");
    }

    #[test]
    fn lowercase_normalized_to_uppercase() {
        let p = PassportData::parse("ab1234567").unwrap();
        assert_eq!(p.as_str(), "AB1234567");

        let p = PassportData::parse("aB1234567").unwrap();
        assert_eq!(p.as_str(), "AB1234567");
    }

    #[test]
    fn series_and_number() {
        let p = PassportData::parse("AB1234567").unwrap();
        assert_eq!(p.series(), "AB");
        assert_eq!(p.number(), "1234567");
    }

    #[test]
    fn whitespace_trimmed() {
        let p = PassportData::parse("  AA1234567  ").unwrap();
        assert_eq!(p.as_str(), "AA1234567");
    }

    #[test]
    fn too_short_rejected() {
        assert!(PassportData::parse("AA123456").is_err());
    }

    #[test]
    fn too_long_rejected() {
        assert!(PassportData::parse("AA12345678").is_err());
    }

    #[test]
    fn digit_series_rejected() {
        assert!(PassportData::parse("121234567").is_err());
        assert!(PassportData::parse("A11234567").is_err());
    }

    #[test]
    fn letter_in_number_rejected() {
        assert!(PassportData::parse("AA12345A7").is_err());
        assert!(PassportData::parse("AA1234-67").is_err());
    }

    #[test]
    fn empty_rejected() {
        assert!(PassportData::parse("").is_err());
        assert!(PassportData::parse("   ").is_err());
    }

    #[test]
    fn both_formats_equal() {
        let a = PassportData::parse("AB1234567").unwrap();
        let b = PassportData::parse("ab1234567").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn display_and_as_ref() {
        let p = PassportData::parse("AB1234567").unwrap();
        assert_eq!(p.to_string(), "AB1234567");
        assert_eq!(p.as_ref(), "AB1234567");
    }

    #[test]
    fn serde_round_trip() {
        let p = PassportData::parse("AB1234567").unwrap();
        let json = serde_json::to_string(&p).unwrap();
        assert_eq!(json, "\"AB1234567\"");

        let back: PassportData = serde_json::from_str(&json).unwrap();
        assert_eq!(p, back);
    }

    #[test]
    fn serde_lowercase_normalized() {
        let p: PassportData = serde_json::from_str("\"ab1234567\"").unwrap();
        assert_eq!(p.as_str(), "AB1234567");
    }

    #[test]
    fn serde_invalid_rejected() {
        assert!(serde_json::from_str::<PassportData>("\"123\"").is_err());
    }
}
