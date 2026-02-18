//! # MyID SDK — Rust Client
//!
//! O'zbekiston Respublikasi MyID identifikatsiya tizimi uchun
//! rasmiy bo'lmagan Rust SDK kutubxonasi.
//!
//! ## Imkoniyatlar
//!
//! - **OAuth 2.0** — MyID API bilan autentifikatsiya
//! - **Type-safe config** — compile-time'da URL va parametrlar validatsiyasi
//! - **Async/await** — `tokio` runtime bilan to'liq asinxron ishlash
//! - **Xavfsizlik** — `client_secret` `Debug` outputda yashiriladi
//! - **Thread-safe** — `Send + Sync` compile-time kafolati
//!
//! ## Tez boshlash
//!
//! ```rust
//! use myid::config::Config;
//! use myid::error::MyIdResult;
//!
//! fn main() -> MyIdResult<()> {
//!     // Minimal config — faqat majburiy parametrlar
//!     let config = Config::new(
//!         "https://myid.uz",
//!         "your_client_id",
//!         "your_client_secret",
//!     )?;
//!
//!     println!("Base URL: {}", config.base_url());
//!     Ok(())
//! }
//! ```
//!
//! ## Konfiguratsiya
//!
//! [`Config`](config::Config) — SDK ning asosiy konfiguratsiya strukturasi.
//! Barcha parametrlar `new()` + `with_*()` chaining pattern orqali sozlanadi:
//!
//! ```rust
//! use std::time::Duration;
//! use myid::config::Config;
//! # use myid::error::MyIdResult;
//!
//! # fn main() -> MyIdResult<()> {
//! let config = Config::new("https://myid.uz", "client_id", "client_secret")?
//!     .with_timeout(Duration::from_secs(30))
//!     .with_connect_timeout(Duration::from_secs(5))
//!     .with_user_agent("my-backend/1.0")
//!     .with_proxy("http://proxy.local:8080")?;
//! # Ok(())
//! # }
//! ```
//!
//! Batafsil ma'lumot uchun [`config`] moduli dokumentatsiyasini ko'ring.
//!
//! ## Xatolarni boshqarish
//!
//! SDK barcha xatolarni [`MyIdError`](error::MyIdError) enum orqali qaytaradi.
//! Qulay ishlatish uchun [`MyIdResult<T>`](error::MyIdResult) type aliasi mavjud:
//!
//! ```rust
//! use myid::config::Config;
//! use myid::error::{MyIdError, MyIdResult};
//!
//! fn create_config() -> MyIdResult<()> {
//!     let config = Config::new("https://myid.uz", "id", "secret")?;
//!     Ok(())
//! }
//!
//! // Xatoni ushlash
//! match Config::new("noto'g'ri-url", "id", "secret") {
//!     Ok(cfg) => println!("Muvaffaqiyatli: {}", cfg.base_url()),
//!     Err(MyIdError::Config { message }) => {
//!         eprintln!("Konfiguratsiya xatosi: {message}");
//!     },
//!     _ => unreachable!(),
//! }
//! ```
//! ## Features
//!
//! | Feature | Default | Tavsif |
//! |---------|---------|--------|
//! | `dotenvy` | ✅ Ha | `.env` fayldan konfiguratsiya yuklash ([`Config::from_env()`](config::Config::from_env)) |
//!
//! ### `dotenvy` ni o'chirish
//!
//! Agar `.env` fayl qo'llab-quvvatlash kerak bo'lmasa:
//!
//! ```toml
//! [dependencies]
//! myid = { version = "0.1.2", default-features = false }
//! ```
//!
//! ## Modullar
//!
//! | Modul | Tavsif |
//! |-------|--------|
//! | [`config`] | SDK konfiguratsiyasi — URL, timeout, proxy, user-agent |
//! | [`error`] | Xatolar tipi (`MyIdError`) va `Result` aliasi |
//! | [`client`] | MyID tizimiga backend orqali so'rovlar yuborish |
//!
//! ## Xavfsizlik eslatmalari
//!
//! - `client_secret` — **faqat backend muhitida** saqlang
//! - `Debug` output'da secret avtomatik `<redacted>` sifatida ko'rsatiladi
//! - Frontend yoki client-side kodda **ishlatmang**
//!
//! ## Minimal Rust versiyasi (MSRV)
//!
//! Rust **1.93.0** yoki undan yuqori talab qilinadi.

pub mod client;
pub mod config;
pub mod dto;
pub mod error;
pub mod prelude;
pub mod types;

pub use client::MyIdClient;
pub use config::Config;
pub use error::{MyIdError, MyIdResult};
