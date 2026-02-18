use crate::error::{MyIdError, MyIdResult};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Tug'ilgan sana (`YYYY-MM-DD` formatda).
///
/// Ichki saqlashda [`NaiveDate`] ishlatiladi — string emas.
/// Bu solishtirish va hisoblash imkonini beradi.
///
/// # Format
///
/// - Kirishda: `YYYY-MM-DD` (masalan: `1990-05-15`)
/// - Kabisa yillar hisobga olinadi
///
/// # Misollar
///
/// ```
/// use myid::types::BirthDate;
///
/// let d = BirthDate::parse("1990-05-15").unwrap();
/// assert_eq!(d.as_str(), "1990-05-15");
///
/// // Kabisa yili
/// assert!(BirthDate::parse("2000-02-29").is_ok());
/// assert!(BirthDate::parse("2023-02-29").is_err());
///
/// // Noto'g'ri format
/// assert!(BirthDate::parse("15-05-1990").is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BirthDate(NaiveDate);

impl BirthDate {
    const FORMAT: &str = "%Y-%m-%d";
    const EXPECTED_LEN: usize = 10; // "YYYY-MM-DD"

    /// String qiymatdan `BirthDate` yaratadi.
    ///
    /// # Xatolar
    ///
    /// [`MyIdError::Validation`] qaytaradi agar:
    /// - Format `YYYY-MM-DD` bo'lmasa
    /// - Sana mavjud bo'lmasa (masalan: `2023-02-30`)
    pub fn parse(value: impl AsRef<str>) -> MyIdResult<Self> {
        let raw = value.as_ref().trim();

        if raw.len() != Self::EXPECTED_LEN {
            return Err(MyIdError::validation(format!(
                "birth_date must be YYYY-MM-DD format, got: {raw}"
            )));
        }

        let date = NaiveDate::parse_from_str(raw, Self::FORMAT)
            .map_err(|_| MyIdError::validation(format!("birth_date is not a valid date: {raw}")))?;

        Ok(Self(date))
    }

    /// Canonical string ko'rinishini qaytaradi (`YYYY-MM-DD`).
    pub fn as_str(&self) -> String {
        self.0.format(Self::FORMAT).to_string()
    }

    /// Ichki [`NaiveDate`] qiymatni qaytaradi.
    ///
    /// Yosh hisoblash yoki sanalar solishtirish uchun foydali.
    pub fn as_date(&self) -> NaiveDate {
        self.0
    }
}

impl std::fmt::Display for BirthDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(Self::FORMAT))
    }
}

impl TryFrom<String> for BirthDate {
    type Error = MyIdError;

    fn try_from(value: String) -> MyIdResult<Self> {
        Self::parse(value)
    }
}

impl From<BirthDate> for String {
    fn from(value: BirthDate) -> Self {
        value.to_string()
    }
}

impl Serialize for BirthDate {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for BirthDate {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::parse(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::BirthDate;
    use chrono::Datelike;

    #[test]
    fn valid_date() {
        let d = BirthDate::parse("2000-05-15").unwrap();
        assert_eq!(d.as_str(), "2000-05-15");
    }

    #[test]
    fn leap_year_accepted() {
        assert!(BirthDate::parse("2024-02-29").is_ok());
        assert!(BirthDate::parse("2000-02-29").is_ok());
    }

    #[test]
    fn non_leap_year_feb29_rejected() {
        assert!(BirthDate::parse("2023-02-29").is_err());
        assert!(BirthDate::parse("1900-02-29").is_err());
    }

    #[test]
    fn invalid_day_rejected() {
        assert!(BirthDate::parse("2024-04-31").is_err());
        assert!(BirthDate::parse("2024-11-31").is_err());
    }

    #[test]
    fn invalid_month_rejected() {
        assert!(BirthDate::parse("2024-13-10").is_err());
        assert!(BirthDate::parse("2024-00-10").is_err());
    }

    #[test]
    fn wrong_format_rejected() {
        assert!(BirthDate::parse("2024/02/28").is_err());
        assert!(BirthDate::parse("28-02-2024").is_err());
        assert!(BirthDate::parse("2024-2-8").is_err());
    }

    #[test]
    fn whitespace_trimmed() {
        let d = BirthDate::parse("  2024-05-15  ").unwrap();
        assert_eq!(d.as_str(), "2024-05-15");
    }

    #[test]
    fn empty_rejected() {
        assert!(BirthDate::parse("").is_err());
        assert!(BirthDate::parse("   ").is_err());
    }

    #[test]
    fn as_date_works() {
        let d = BirthDate::parse("2000-06-15").unwrap();
        assert_eq!(d.as_date().year(), 2000);
        assert_eq!(d.as_date().month(), 6);
        assert_eq!(d.as_date().day(), 15);
    }

    #[test]
    fn copy_semantics() {
        let a = BirthDate::parse("2000-01-01").unwrap();
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn display_matches_as_str() {
        let d = BirthDate::parse("2000-05-15").unwrap();
        assert_eq!(d.to_string(), d.as_str());
    }

    #[test]
    fn serde_round_trip() {
        let d = BirthDate::parse("2000-05-15").unwrap();
        let json = serde_json::to_string(&d).unwrap();
        assert_eq!(json, "\"2000-05-15\"");

        let back: BirthDate = serde_json::from_str(&json).unwrap();
        assert_eq!(d, back);
    }

    #[test]
    fn serde_invalid_rejected() {
        assert!(serde_json::from_str::<BirthDate>("\"not-a-date\"").is_err());
        assert!(serde_json::from_str::<BirthDate>("\"2024-13-01\"").is_err());
    }
}
