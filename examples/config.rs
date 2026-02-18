//! MyID SDK — Config yaratish misollari
//!
//! Ishga tushirish:
//! ```bash
//! cargo run --example config
//! ```

use std::time::Duration;

use myid::config::Config;
use myid::error::MyIdResult;

fn main() -> MyIdResult<()> {
    minimal_config()?;
    full_config()?;
    from_env_config()?;
    error_handling_examples();

    Ok(())
}

/// 1) Minimal config — faqat majburiy fieldlar, qolganlari default
fn minimal_config() -> MyIdResult<()> {
    println!("=== Minimal Config ===\n");

    let cfg = Config::new("https://myid.example.uz", "app_id", "secret_123")?;

    println!("Base URL:      {}", cfg.base_url());
    println!("Client ID:     {}", cfg.client_id());
    println!("Timeout:       {:?}", cfg.timeout());
    println!("Conn timeout:  {:?}", cfg.connection_timeout());
    println!("User-Agent:    {}", cfg.user_agent());
    println!("Proxy:         {:?}", cfg.proxy_url());
    println!();

    // Debug output — client_secret <redacted> sifatida ko'rinadi
    println!("Debug output (secret yashirin):");
    println!("{:#?}\n", cfg);

    Ok(())
}

/// 2) To'liq config — barcha parametrlar sozlangan
fn full_config() -> MyIdResult<()> {
    println!("=== Full Config ===\n");

    let cfg = Config::new("https://myid.example.uz", "app_id", "secret_123")?
        .with_timeout(Duration::from_secs(30))
        .with_connect_timeout(Duration::from_secs(5))
        .with_user_agent("my-service/2.0")
        .with_proxy("http://proxy.corp.local:8080")?;

    println!("{:#?}\n", cfg);

    Ok(())
}

/// 3) Environment o'zgaruvchilaridan yuklash
///
/// Ishga tushirishdan oldin .env fayl yarating yoki env var bering:
/// ```bash
/// MYID_BASE_URL=https://myid.example.uz
/// MYID_CLIENT_ID=env_app_id
/// MYID_CLIENT_SECRET=env_secret
/// MYID_TIMEOUT_MS=30000
/// MYID_PROXY_URL=http://proxy:8080
/// ```
fn from_env_config() -> MyIdResult<()> {
    println!("=== From Environment ===\n");

    // Default prefiks: MYID_
    match Config::from_env(None) {
        Ok(cfg) => {
            println!("Base URL:      {}", cfg.base_url());
            println!("Client ID:     {}", cfg.client_id());
            println!("Timeout:       {:?}", cfg.timeout());
            println!("Proxy:         {:?}", cfg.proxy_url());
        }
        Err(e) => {
            println!("Env config yuklanmadi (kutilgan): {e}");
            println!("MYID_BASE_URL, MYID_CLIENT_ID, MYID_CLIENT_SECRET o'rnating");
        }
    }

    // Custom prefiks: APP_BASE_URL, APP_CLIENT_ID, ...
    match Config::from_env(Some("APP")) {
        Ok(cfg) => println!("Custom prefix:  {}", cfg.base_url()),
        Err(e) => println!("APP_ prefix:    yuklanmadi (kutilgan): {e}"),
    }

    println!();
    Ok(())
}

/// 4) Xato holatlari — noto'g'ri kiritishlarni qanday ushlash
fn error_handling_examples() {
    println!("=== Error Handling ===\n");

    // Noto'g'ri URL
    let err = Config::new("not-a-url", "id", "secret");
    println!("Noto'g'ri URL:    {}", err.unwrap_err());

    // Ruxsat etilmagan scheme (faqat http/https)
    let err = Config::new("ftp://example.uz", "id", "secret");
    println!("FTP scheme:       {}", err.unwrap_err());

    // Bo'sh URL
    let err = Config::new("", "id", "secret");
    println!("Bo'sh URL:        {}", err.unwrap_err());

    // Noto'g'ri proxy URL
    let err = Config::new("https://example.uz", "id", "secret")
        .expect("base config xato bo'lmasligi kerak")
        .with_proxy("not-a-proxy");
    println!("Noto'g'ri proxy:  {}\n", err.unwrap_err());
}