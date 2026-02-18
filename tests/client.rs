use std::time::Duration;

use myid::client::MyIdClient;
use myid::config::Config;
use myid::error::{MyIdError, MyIdResult};

// ===== MyIdClient::new() =====

#[test]
fn new_with_valid_config() -> MyIdResult<()> {
    let config = Config::new("https://myid.uz", "id", "secret")?;
    let client = MyIdClient::new(config);

    assert!(client.is_ok());
    Ok(())
}

#[test]
fn new_with_custom_timeouts() -> MyIdResult<()> {
    let config = Config::new("https://myid.uz", "id", "secret")?
        .with_timeout(Duration::from_secs(60))
        .with_connect_timeout(Duration::from_secs(10));

    assert!(MyIdClient::new(config).is_ok());
    Ok(())
}

#[test]
fn new_with_proxy() -> MyIdResult<()> {
    let config =
        Config::new("https://myid.uz", "id", "secret")?.with_proxy("http://proxy.local:8080")?;

    assert!(MyIdClient::new(config).is_ok());
    Ok(())
}

// ===== Clone — shared cache =====

#[test]
fn clone_shares_token_cache() -> MyIdResult<()> {
    let config = Config::new("https://myid.uz", "id", "secret")?;
    let client = MyIdClient::new(config)?;

    let cloned = client.clone();

    // Ikkalasi ham ishlaydi — panic bo'lmaydi
    drop(cloned);
    drop(client);

    Ok(())
}

// ===== get_token — cache bo'sh, API kerak =====

#[tokio::test]
async fn get_token_without_server_returns_http_error() -> MyIdResult<()> {
    // Mavjud bo'lmagan serverga ulanish — Http xato kutamiz
    let config = Config::new("http://127.0.0.1:1", "id", "secret")?
        .with_timeout(Duration::from_millis(100))
        .with_connect_timeout(Duration::from_millis(100));

    let client = MyIdClient::new(config)?;
    let result = client.get_token().await;

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), MyIdError::Http(_)));

    Ok(())
}

// ===== create_session — API kerak =====

#[tokio::test]
async fn create_session_without_server_returns_error() -> MyIdResult<()> {
    let config = Config::new("http://127.0.0.1:1", "id", "secret")?
        .with_timeout(Duration::from_millis(100))
        .with_connect_timeout(Duration::from_millis(100));

    let client = MyIdClient::new(config)?;

    let request = myid::dto::CreateSessionRequest::Empty {};
    let result = client.create_session(&request).await;

    // Token olish bosqichida Http xato
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), MyIdError::Http(_)));

    Ok(())
}
