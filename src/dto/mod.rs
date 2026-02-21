//! API so'rov va javob DTO strukturalari.
//!
//! | Modul | Tavsif |
//! |-------|--------|
//! | [`auth`] | OAuth 2.0 token so'rov/javob strukturalari |
//! | [`session`] | Session yaratish so'rov/javob strukturalari |
//! | [`user_data`] | Foydalanuvchi profil javob strukturalari |
//!
//! # Session yaratish
//!
//! [`CreateSessionRequest`] enum orqali 3 xil usul:
//!
//! | Variant | Struct | Majburiy |
//! |---------|--------|----------|
//! | `WithPinfl` | [`SessionWithPinfl`] | `pinfl` + `birth_date` |
//! | `WithPassport` | [`SessionWithPassport`] | `pass_data` + `birth_date` |
//! | `WithReuid` | [`SessionWithReuid`] | `reuid` |

mod auth;
mod session;
mod user_data;

// Consumer ko'radigan structlar
pub use auth::{AccessTokenRequest, AccessTokenResponse};
pub use session::{
    CreateSessionRequest, SessionResponse, SessionStatus, SessionStatusResponse,
    SessionWithPassport, SessionWithPinfl, SessionWithReuid,
};
pub use user_data::UserDataResponse;
