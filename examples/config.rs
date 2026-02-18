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
    corporate_proxy_config()?;
    error_handling_examples();

    Ok(())
}

/// 1) Minimal config — faqat majburiy fieldlar, qolganlari default
fn minimal_config() -> MyIdResult<()> {
    println!("=== Minimal Config ===\n");

    let cfg = Config::new("https://myid.example.uz", "app_id", "secret_123")?;

    println!("Base URL:    {}", cfg.base_url());
    println!("Client ID:   {}", cfg.client_id());
    println!("Timeout:     {:?}", cfg.timeout());
    println!("Conn timeout:{:?}", cfg.connection_timeout());
    println!("User-Agent:  {}", cfg.user_agent());
    println!("Proxy:       {:?}", cfg.proxy_url());
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

/// 3) Korporativ muhit — proxy va katta timeout
fn corporate_proxy_config() -> MyIdResult<()> {
    println!("=== Corporate Proxy Config ===\n");

    // Korporativ tarmoqlarda proxy va uzoq timeout odatiy holat
    let cfg = Config::new("https://myid.prod.example.uz", "corp_app", "corp_secret")?
        .with_timeout(Duration::from_secs(60))
        .with_connect_timeout(Duration::from_secs(10))
        .with_user_agent("corp-backend/1.0")
        .with_proxy("https://egress-proxy.corp.local:3128")?;

    println!("Proxy: {:?}\n", cfg.proxy_url());

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

    // Noto'g'ri proxy URL
    let err = Config::new("https://example.uz", "id", "secret")
        .expect("base config xato bo'lmasligi kerak")
        .with_proxy("not-a-proxy");
    println!("Noto'g'ri proxy:  {}", err.unwrap_err());

    // Bo'sh URL
    let err = Config::new("", "id", "secret");
    println!("Bo'sh URL:        {}\n", err.unwrap_err());
}