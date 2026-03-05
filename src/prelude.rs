//! SDK ning barcha tez-tez ishlatiladigan turlarini bitta import bilan yuklash.
//!
//! Har bir moduldan alohida import o'rniga shu modul yetarli:
//!
//! ```rust
//! use myid::prelude::*;
//! ```
//!
//! # Eksport qilinadi
//!
//! | Kategoriya | Turlar |
//! |------------|--------|
//! | Core | [`MyIdClient`], [`Config`], [`MyIdError`], [`MyIdResult`] |
//! | Types | [`Pinfl`], [`BirthDate`], [`PassportData`], [`PhoneNumber`], [`Threshold`], [`Reuid`], [`SessionId`], [`JobId`] |
//! | DTO | [`CreateSessionRequest`], [`SessionWithPinfl`], [`SessionWithPassport`], [`SessionWithReuid`], [`SessionResponse`], [`SessionStatusResponse`], [`SessionStatus`], [`SessionAttempt`], [`AccessTokenResponse`], [`UserDataResponse`], [`ReuId`] |

// Core
pub use crate::client::MyIdClient;
pub use crate::config::Config;
pub use crate::error::{MyIdError, MyIdResult};

// Types
pub use crate::types::{
    BirthDate, JobId, PassportData, PhoneNumber, Pinfl, Reuid, SessionId, Threshold,
};

// DTO
pub use crate::dto::{
    // AccessTokenResponse olib tashlandi — faqat internal cache uchun ishlatilgani uchun
    CreateSessionRequest,
    ReuId,
    SessionAttempt,
    SessionResponse,
    SessionStatus,
    SessionStatusResponse,
    SessionWithPassport,
    SessionWithPinfl,
    SessionWithReuid,
    UserDataResponse,
};
