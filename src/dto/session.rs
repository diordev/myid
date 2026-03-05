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
    #[serde(flatten)]
    options: SessionOptions,
}

impl SessionWithPinfl {
    /// Yangi `SessionWithPinfl` yaratadi (faqat majburiy fieldlar).
    pub fn new(pinfl: Pinfl, birth_date: BirthDate) -> Self {
        Self {
            pinfl,
            birth_date,
            options: SessionOptions::default(),
        }
    }

    /// Telefon raqamini qo'shadi (ixtiyoriy).
    #[must_use]
    pub fn with_phone_number(mut self, phone_number: PhoneNumber) -> Self {
        self.options.phone_number = Some(phone_number);
        self
    }

    /// Rezidentlik belgisini o'rnatadi (ixtiyoriy).
    ///
    /// `true` — O'zbekiston rezidenti, `false` — xorijiy fuqaro.
    #[must_use]
    pub fn with_is_resident(mut self, is_resident: bool) -> Self {
        self.options.is_resident = Some(is_resident);
        self
    }

    /// Face matching aniqlik darajasini o'rnatadi (ixtiyoriy).
    ///
    /// Berilmasa server default qiymatini ishlatadi.
    #[must_use]
    pub fn with_threshold(mut self, threshold: Threshold) -> Self {
        self.options.threshold = Some(threshold);
        self
    }

    /// PINFL qiymatini qaytaradi.
    pub fn pinfl(&self) -> &Pinfl {
        &self.pinfl
    }

    /// Tug'ilgan sanani qaytaradi.
    pub fn birth_date(&self) -> BirthDate {
        self.birth_date
    }

    /// Telefon raqamini qaytaradi (agar o'rnatilgan bo'lsa).
    pub fn phone_number(&self) -> Option<&PhoneNumber> {
        self.options.phone_number.as_ref()
    }

    /// Rezidentlik belgisini qaytaradi (agar o'rnatilgan bo'lsa).
    pub fn is_resident(&self) -> Option<bool> {
        self.options.is_resident
    }

    /// Face matching threshold qiymatini qaytaradi (agar o'rnatilgan bo'lsa).
    pub fn threshold(&self) -> Option<Threshold> {
        self.options.threshold
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
    #[serde(flatten)]
    options: SessionOptions,
}

impl SessionWithPassport {
    /// Yangi `SessionWithPassport` yaratadi (faqat majburiy fieldlar).
    pub fn new(pass_data: PassportData, birth_date: BirthDate) -> Self {
        Self {
            pass_data,
            birth_date,
            options: SessionOptions::default(),
        }
    }

    /// Telefon raqamini qo'shadi (ixtiyoriy).
    #[must_use]
    pub fn with_phone_number(mut self, phone_number: PhoneNumber) -> Self {
        self.options.phone_number = Some(phone_number);
        self
    }

    /// Rezidentlik belgisini o'rnatadi (ixtiyoriy).
    #[must_use]
    pub fn with_is_resident(mut self, is_resident: bool) -> Self {
        self.options.is_resident = Some(is_resident);
        self
    }

    /// Face matching aniqlik darajasini o'rnatadi (ixtiyoriy).
    #[must_use]
    pub fn with_threshold(mut self, threshold: Threshold) -> Self {
        self.options.threshold = Some(threshold);
        self
    }

    /// Pasport seriya va raqamini qaytaradi.
    pub fn pass_data(&self) -> &PassportData {
        &self.pass_data
    }

    /// Tug'ilgan sanani qaytaradi.
    pub fn birth_date(&self) -> BirthDate {
        self.birth_date
    }

    /// Telefon raqamini qaytaradi (agar o'rnatilgan bo'lsa).
    pub fn phone_number(&self) -> Option<&PhoneNumber> {
        self.options.phone_number.as_ref()
    }

    /// Rezidentlik belgisini qaytaradi (agar o'rnatilgan bo'lsa).
    pub fn is_resident(&self) -> Option<bool> {
        self.options.is_resident
    }

    /// Face matching threshold qiymatini qaytaradi (agar o'rnatilgan bo'lsa).
    pub fn threshold(&self) -> Option<Threshold> {
        self.options.threshold
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
    /// Yangi `SessionWithReuid` yaratadi.
    pub fn new(reuid: Reuid) -> Self {
        Self {
            reuid,
            phone_number: None,
        }
    }

    /// Telefon raqamini qo'shadi (ixtiyoriy).
    #[must_use]
    pub fn with_phone_number(mut self, phone_number: PhoneNumber) -> Self {
        self.phone_number = Some(phone_number);
        self
    }

    /// REUID qiymatini qaytaradi.
    pub fn reuid(&self) -> Reuid {
        self.reuid
    }

    /// Telefon raqamini qaytaradi (agar o'rnatilgan bo'lsa).
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

/// Session yaratish muvaffaqiyatli bo'lganda qaytariladigan javob.
///
/// `session_id` — keyingi so'rovlarda (status, natija) ishlatiladi.
#[derive(Debug, Deserialize)]
pub struct SessionResponse {
    session_id: SessionId,
}

impl SessionResponse {
    /// Yaratilgan session identifikatorini qaytaradi.
    pub fn session_id(&self) -> SessionId {
        self.session_id
    }
}

/// Session holati so'roviga javob.
///
/// Session tugagandan so'ng `code` field paydo bo'ladi —
/// bu verifikatsiya natijasini bildiruvchi kod.
#[derive(Debug, Deserialize)]
pub struct SessionStatusResponse {
    code: Option<String>,
    status: SessionStatus,
    attempts: Vec<SessionAttempt>,
}

impl SessionStatusResponse {
    /// Verifikatsiya natija kodini qaytaradi.
    ///
    /// Session `Closed` holatida bo'lsagina qiymat bo'ladi.
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// Joriy session holatini qaytaradi.
    pub fn status(&self) -> &SessionStatus {
        &self.status
    }

    /// Barcha verifikatsiya urinishlarini qaytaradi.
    pub fn attempts(&self) -> &[SessionAttempt] {
        &self.attempts
    }
}

/// Session holat turlari.
///
/// API `snake_case` va `UPPERCASE` formatlarini qaytarishi mumkin —
/// ikkalasi ham qabul qilinadi.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    /// Verifikatsiya jarayoni davom etmoqda.
    #[serde(alias = "IN_PROGRESS")]
    InProgress,
    /// Verifikatsiya yakunlandi (muvaffaqiyatli yoki muvaffaqiyatsiz).
    #[serde(alias = "CLOSED")]
    Closed,
    /// Session muddati tugadi.
    #[serde(alias = "EXPIRED")]
    Expired,
}

impl SessionStatus {
    /// Session hali jarayonda ekanligini tekshiradi.
    pub fn is_in_progress(&self) -> bool {
        matches!(self, Self::InProgress)
    }

    /// Session yopilganligini tekshiradi.
    pub fn is_closed(&self) -> bool {
        matches!(self, Self::Closed)
    }

    /// Session muddati o'tganligini tekshiradi.
    pub fn is_expired(&self) -> bool {
        matches!(self, Self::Expired)
    }
}

/// Bitta verifikatsiya urinishi haqida ma'lumot.
#[derive(Debug, Deserialize)]
pub struct SessionAttempt {
    job_id: JobId,
    timestamp: DateTime<Utc>,
    reason: Option<String>,
    reason_code: Option<i32>,
}

impl SessionAttempt {
    /// Ushbu urinishga tegishli job identifikatorini qaytaradi.
    pub fn job_id(&self) -> JobId {
        self.job_id
    }

    /// Urinish amalga oshirilgan vaqtni qaytaradi (UTC).
    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    /// Muvaffaqiyatsizlik sababini qaytaradi (agar mavjud bo'lsa).
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    /// Muvaffaqiyatsizlik sabab kodini qaytaradi (agar mavjud bo'lsa).
    pub fn reason_code(&self) -> Option<i32> {
        self.reason_code
    }
}

/// PINFL va Passport sessiyalari uchun umumiy ixtiyoriy
/// parametrlar.
#[derive(Debug, Default, Serialize)]
struct SessionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<PhoneNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_resident: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold: Option<Threshold>,
}
