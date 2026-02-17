//! # MyID SDK — O'zbekiston MyID identifikatsiya tizimi bilan ishlash uchun Rust kutubxonasi.
//! ## myid crate - dastlabki versiya, API faol ishlab chiqilmoqda.

/// Crate version helper.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}


pub mod config;
pub mod error;