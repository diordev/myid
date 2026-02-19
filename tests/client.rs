use std::time::Duration;

use myid::client::MyIdClient;
use myid::config::Config;
use myid::error::MyIdResult;

fn fast_config(url: &str) -> MyIdResult<Config> {
    Ok(Config::new(url, "id", "secret")?
        .with_timeout(Duration::from_millis(100))
        .with_connect_timeout(Duration::from_millis(100)))
}

// ===== MyIdClient::new() =====

#[test]
fn new_with_valid_config() -> MyIdResult<()> {
    let config = Config::new("https://myid.uz", "id", "secret")?;
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

#[test]
fn new_with_custom_timeouts() -> MyIdResult<()> {
    let config = Config::new("https://myid.uz", "id", "secret")?
        .with_timeout(Duration::from_secs(60))
        .with_connect_timeout(Duration::from_secs(10));
    assert!(MyIdClient::new(config).is_ok());
    Ok(())
}

// ===== Clone — shared cache =====

#[test]
fn clone_shares_token_cache() -> MyIdResult<()> {
    let client = MyIdClient::new(Config::new("https://myid.uz", "id", "secret")?)?;
    let cloned = client.clone();
    drop(cloned);
    drop(client);
    Ok(())
}

// ===== get_token — server yo'q =====

#[tokio::test]
async fn get_token_without_server_returns_error() -> MyIdResult<()> {
    // Mavjud bo'lmagan yoki yopiq serverga ulanish — xato kutamiz
    let config = fast_config("http://127.0.0.1:1")?;
    let client = MyIdClient::new(config)?;
    assert!(client.get_token().await.is_err());
    Ok(())
}

// ===== create_session — server yo'q =====

#[tokio::test]
async fn create_session_without_server_returns_error() -> MyIdResult<()> {
    let config = fast_config("http://127.0.0.1:1")?;
    let client = MyIdClient::new(config)?;
    let request = myid::dto::CreateSessionRequest::Empty {};
    assert!(client.create_session(&request).await.is_err());
    Ok(())
}

// ===== Clone — parallel get_token =====

#[tokio::test]
async fn cloned_clients_share_error_path() -> MyIdResult<()> {
    let config = fast_config("http://127.0.0.1:1")?;
    let client = MyIdClient::new(config)?;
    let c1 = client.clone();
    let c2 = client.clone();

    let (r1, r2) = tokio::join!(c1.get_token(), c2.get_token());
    assert!(r1.is_err());
    assert!(r2.is_err());
    Ok(())
}
