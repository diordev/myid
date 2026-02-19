//! Type-safe qiymat turlari.
//!
//! Har bir tur compile-time'da validatsiya qilinadi —
//! noto'g'ri qiymat yaratib bo'lmaydi.
//!
//! | Tur | Tavsif |
//! |-----|--------|
//! | [`Pinfl`] | 14 raqamli shaxsiy identifikatsiya raqami |
//! | [`PassportData`] | Pasport seriyasi va raqami (`AB1234567`) |
//! | [`BirthDate`] | Tug'ilgan sana (`YYYY-MM-DD`) |
//! | [`PhoneNumber`] | O'zbekiston telefon raqami (`+998XXXXXXXXX`) |
//! | [`Threshold`] | Face matching aniqlik darajasi (`0.0–1.0`) |
//! | [`SessionId`] | Session UUID identifikatori |
//! | [`Reuid`] | Secondary flow UUID identifikatori |
//! | [`JobId`] | Verifikatsiya urinishi UUID identifikatori |

mod birth_date;
mod job_id;
mod pass_data;
mod phone_number;
mod pinfl;
mod reuid;
mod session_id;
mod threshold;
pub(crate) mod uuid_type;

pub use birth_date::BirthDate;
pub use job_id::JobId;
pub use pass_data::PassportData;
pub use phone_number::PhoneNumber;
pub use pinfl::Pinfl;
pub use reuid::Reuid;
pub use session_id::SessionId;
pub use threshold::Threshold;
