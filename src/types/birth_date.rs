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
/// assert_eq!(d.to_formatted_string(), "1990-05-15");
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
    pub fn to_formatted_string(&self) -> String {
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
