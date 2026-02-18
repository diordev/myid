//! Session yaratish va boshqarish uchun DTO'lar.
//!
//! MyID API 3 xil usulda session yaratish imkonini beradi:
//!
//! | Usul | Struct | Majburiy fieldlar |
//! |------|--------|-------------------|
//! | PINFL orqali | [`SessionWithPinfl`] | `pinfl` + `birth_date` |
//! | Passport orqali | [`SessionWithPassport`] | `pass_data` + `birth_date` |
//! | REUID orqali | [`SessionWithReuid`] | `reuid` |

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{
    BirthDate, JobId, PassportData, PhoneNumber, Pinfl, Reuid, SessionId, Threshold,
};

// ─── Request: Session yaratish ───

/// PINFL orqali session yaratish.
///
/// # Misollar
///
/// ```
/// # use myid::dto::SessionWithPinfl;
/// # use myid::types::{Pinfl, BirthDate, PhoneNumber, Threshold};
/// # use myid::error::MyIdResult;
/// # fn main() -> MyIdResult<()> {
/// // Minimal
/// let req = SessionWithPinfl::new(
///     Pinfl::parse("12345678901234")?,
///     BirthDate::parse("1990-05-15")?,
/// );
///
/// // To'liq
/// let req = SessionWithPinfl::new(
///     Pinfl::parse("12345678901234")?,
///     BirthDate::parse("1990-05-15")?,
/// )
///     .with_phone_number(PhoneNumber::parse("+998901234567")?)
///     .with_threshold(Threshold::parse(0.85)?)
///     .with_is_resident(true);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Serialize)]
pub struct SessionWithPinfl {
    pinfl: Pinfl,
    birth_date: BirthDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<PhoneNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_resident: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold: Option<Threshold>,
}

impl SessionWithPinfl {
    pub fn new(pinfl: Pinfl, birth_date: BirthDate) -> Self {
        Self {
            pinfl,
            birth_date,
            phone_number: None,
            is_resident: None,
            threshold: None,
        }
    }

    #[must_use]
    pub fn with_phone_number(mut self, phone_number: PhoneNumber) -> Self {
        self.phone_number = Some(phone_number);
        self
    }

    #[must_use]
    pub fn with_is_resident(mut self, is_resident: bool) -> Self {
        self.is_resident = Some(is_resident);
        self
    }

    #[must_use]
    pub fn with_threshold(mut self, threshold: Threshold) -> Self {
        self.threshold = Some(threshold);
        self
    }

    pub fn pinfl(&self) -> &Pinfl {
        &self.pinfl
    }

    pub fn birth_date(&self) -> BirthDate {
        self.birth_date
    }

    pub fn phone_number(&self) -> Option<&PhoneNumber> {
        self.phone_number.as_ref()
    }

    pub fn is_resident(&self) -> Option<bool> {
        self.is_resident
    }

    pub fn threshold(&self) -> Option<Threshold> {
        self.threshold
    }
}

/// Passport orqali session yaratish.
///
/// # Misollar
///
/// ```
/// # use myid::dto::SessionWithPassport;
/// # use myid::types::{PassportData, BirthDate};
/// # use myid::error::MyIdResult;
/// # fn main() -> MyIdResult<()> {
/// let req = SessionWithPassport::new(
///     PassportData::parse("AB1234567")?,
///     BirthDate::parse("1990-05-15")?,
/// );
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Serialize)]
pub struct SessionWithPassport {
    pass_data: PassportData,
    birth_date: BirthDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<PhoneNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_resident: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold: Option<Threshold>,
}

impl SessionWithPassport {
    pub fn new(pass_data: PassportData, birth_date: BirthDate) -> Self {
        Self {
            pass_data,
            birth_date,
            phone_number: None,
            is_resident: None,
            threshold: None,
        }
    }

    #[must_use]
    pub fn with_phone_number(mut self, phone_number: PhoneNumber) -> Self {
        self.phone_number = Some(phone_number);
        self
    }

    #[must_use]
    pub fn with_is_resident(mut self, is_resident: bool) -> Self {
        self.is_resident = Some(is_resident);
        self
    }

    #[must_use]
    pub fn with_threshold(mut self, threshold: Threshold) -> Self {
        self.threshold = Some(threshold);
        self
    }

    pub fn pass_data(&self) -> &PassportData {
        &self.pass_data
    }

    pub fn birth_date(&self) -> BirthDate {
        self.birth_date
    }

    pub fn phone_number(&self) -> Option<&PhoneNumber> {
        self.phone_number.as_ref()
    }

    pub fn is_resident(&self) -> Option<bool> {
        self.is_resident
    }

    pub fn threshold(&self) -> Option<Threshold> {
        self.threshold
    }
}

/// REUID orqali session yaratish (secondary flow).
///
/// # Misollar
///
/// ```
/// # use myid::dto::SessionWithReuid;
/// # use myid::types::Reuid;
/// # use myid::error::MyIdResult;
/// # fn main() -> MyIdResult<()> {
/// let req = SessionWithReuid::new(
///     Reuid::parse("9b7e597e-893e-4e11-92cf-f4e7d4f923b1")?,
/// );
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Serialize)]
pub struct SessionWithReuid {
    reuid: Reuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<PhoneNumber>,
}

impl SessionWithReuid {
    pub fn new(reuid: Reuid) -> Self {
        Self {
            reuid,
            phone_number: None,
        }
    }

    #[must_use]
    pub fn with_phone_number(mut self, phone_number: PhoneNumber) -> Self {
        self.phone_number = Some(phone_number);
        self
    }

    pub fn reuid(&self) -> Reuid {
        self.reuid
    }

    pub fn phone_number(&self) -> Option<&PhoneNumber> {
        self.phone_number.as_ref()
    }
}

/// Barcha session yaratish variantlarini birlashtiruvchi enum.
///
/// `#[serde(untagged)]` — JSON'da wrapper key bo'lmaydi,
/// ichki struct to'g'ridan-to'g'ri serialize bo'ladi.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CreateSessionRequest {
    /// PINFL orqali (primary flow)
    WithPinfl(SessionWithPinfl),
    /// Passport orqali (primary flow)
    WithPassport(SessionWithPassport),
    /// REUID orqali (secondary flow)
    WithReuid(SessionWithReuid),
    /// Bo'sh session — foydalanuvchi o'zi kiritadi
    Empty {},
}

// ─── Response ───

/// Session yaratish javobi.
#[derive(Debug, Deserialize)]
pub struct SessionResponse {
    session_id: SessionId,
}

impl SessionResponse {
    pub fn session_id(&self) -> SessionId {
        self.session_id
    }
}

/// Session holati javobi.
#[derive(Debug, Deserialize)]
pub struct SessionStatusResponse {
    code: Option<String>,
    status: SessionStatus,
    attempts: Vec<SessionAttempt>,
}

impl SessionStatusResponse {
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    pub fn status(&self) -> &SessionStatus {
        &self.status
    }

    pub fn attempts(&self) -> &[SessionAttempt] {
        &self.attempts
    }
}

/// Session holat turlari.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    #[serde(alias = "IN_PROGRESS")]
    InProgress,
    #[serde(alias = "CLOSED")]
    Closed,
    #[serde(alias = "EXPIRED")]
    Expired,
}

impl SessionStatus {
    pub fn is_in_progress(&self) -> bool {
        matches!(self, Self::InProgress)
    }

    pub fn is_closed(&self) -> bool {
        matches!(self, Self::Closed)
    }

    pub fn is_expired(&self) -> bool {
        matches!(self, Self::Expired)
    }
}

/// Session urinishi (attempt) haqida ma'lumot.
#[derive(Debug, Deserialize)]
pub struct SessionAttempt {
    job_id: JobId,
    timestamp: DateTime<Utc>,
    reason: Option<String>,
    reason_code: Option<i32>,
}

impl SessionAttempt {
    pub fn job_id(&self) -> JobId {
        self.job_id
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn reason_code(&self) -> Option<i32> {
        self.reason_code
    }
}

