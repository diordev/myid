//! MyID SDK — Config yaratish misollari
//!
//! Ishga tushirish:
//! ```bash
//! cargo run --example config
//! ```

use std::time::Duration;

use myid::error::MyIdError;
use myid::Config;
use myid::MyIdResult;

fn main() -> MyIdResult<()> {
    minimal_config()?;
    full_config()?;
    from_env_config();
    error_handling_examples();
    Ok(())
}

/// 1) Minimal config — faqat majburiy fieldlar
fn minimal_config() -> MyIdResult<()> {
    println!("=== Minimal Config ===");

    let cfg = Config::new("https://myid.example.uz", "app_id", "secret_123")?;

    println!("Base URL:   {}", cfg.base_url());
    println!("Client ID:  {}", cfg.client_id());
    println!("Timeout:    {:?}", cfg.timeout());
    println!("User-Agent: {}", cfg.user_agent());
    // Debug chiqishida client_secret <redacted> ko'rinadi
    println!("Debug: {cfg:#?}\n");

    Ok(())
}

/// 2) To'liq config — barcha parametrlar sozlangan
fn full_config() -> MyIdResult<()> {
    println!("=== Full Config ===");

    let cfg = Config::new("https://myid.example.uz", "app_id", "secret_123")?
        .with_timeout(Duration::from_secs(30))
        .with_connect_timeout(Duration::from_secs(5))
        .with_user_agent("my-service/2.0")
        .with_proxy("http://proxy.corp.local:8080")?;

    println!("{cfg:#?}\n");
    Ok(())
}

/// 3) Environment o'zgaruvchilaridan yuklash
///
/// Kerakli env varlar:
/// ```bash
/// MYID_BASE_URL=https://myid.example.uz
/// MYID_CLIENT_ID=app_id
/// MYID_CLIENT_SECRET=secret
/// MYID_TIMEOUT_MS=30000          # ixtiyoriy
/// MYID_PROXY_URL=http://proxy:8080  # ixtiyoriy
/// ```
fn from_env_config() {
    println!("=== From Environment ===");

    match Config::from_env(None) {
        Ok(cfg) => {
            println!("Yuklandi: base_url={}, client_id={}", cfg.base_url(), cfg.client_id());
        }
        Err(MyIdError::Config { message }) => {
            println!("Config xatosi (kutilgan): {message}");
            println!("MYID_BASE_URL, MYID_CLIENT_ID, MYID_CLIENT_SECRET o'rnating\n");
        }
        Err(e) => println!("Xato: {e}\n"),
    }
}

/// 4) Xato holatlari — validatsiya
fn error_handling_examples() {
    println!("=== Xato holatlari ===");

    let cases: &[(&str, &str, &str)] = &[
        ("not-a-url", "id", "secret"),   // noto'g'ri URL
        ("ftp://x.uz", "id", "secret"),   // ruxsatsiz scheme
        ("https://x.uz", "", "secret"),   // bo'sh client_id
        ("https://x.uz", "id", ""),       // bo'sh client_secret
        ("", "id", "secret"),             // bo'sh URL
    ];

    for (url, id, secret) in cases {
        match Config::new(*url, *id, *secret) {
            Ok(_) => println!("ok (kutilmagan)"),
            Err(e) => println!("  {e}"),
        }
    }

    // Noto'g'ri proxy
    if let Err(e) = Config::new("https://x.uz", "id", "secret")
        .expect("to'g'ri config")
        .with_proxy("not-a-proxy")
    {
        println!("  {e}");
    }

    println!();
}
