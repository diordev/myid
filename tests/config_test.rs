use std::time::Duration;

use myid::config::{Config, DEFAULT_CONNECT_TIMEOUT_MS, DEFAULT_TIMEOUT_MS};
use myid::error::MyIdResult;

// ===== Happy path =====

#[test]
fn create_config_with_required_fields() -> MyIdResult<()> {
    let cfg = Config::new("https://myid.example.uz", "app_id", "secret_123")?;

    assert_eq!(cfg.base_url(), "https://myid.example.uz/");
    assert_eq!(cfg.client_id(), "app_id");
    assert_eq!(cfg.client_secret(), "secret_123");
    Ok(())
}

#[test]
fn default_values_are_correct() -> MyIdResult<()> {
    let cfg = Config::new("https://example.uz", "id", "secret")?;

    assert_eq!(cfg.timeout(), Duration::from_millis(DEFAULT_TIMEOUT_MS));
    assert_eq!(
        cfg.connection_timeout(),
        Duration::from_millis(DEFAULT_CONNECT_TIMEOUT_MS)
    );
    assert_eq!(cfg.user_agent(), "myid-client-rust/0.1");
    assert_eq!(cfg.proxy_url(), None);
    Ok(())
}

#[test]
fn trailing_slash_appended_when_missing() -> MyIdResult<()> {
    let cfg = Config::new("https://example.uz", "id", "secret")?;
    assert_eq!(cfg.base_url(), "https://example.uz/");
    Ok(())
}

#[test]
fn trailing_slash_preserved_when_present() -> MyIdResult<()> {
    let cfg = Config::new("https://example.uz/", "id", "secret")?;
    assert_eq!(cfg.base_url(), "https://example.uz/");
    Ok(())
}

#[test]
fn trailing_slash_with_path() -> MyIdResult<()> {
    let cfg = Config::new("https://example.uz/api/v1", "id", "secret")?;
    assert_eq!(cfg.base_url(), "https://example.uz/api/v1/");
    Ok(())
}

// ===== Builder methods (with_*) =====

#[test]
fn with_timeout_overrides_default() -> MyIdResult<()> {
    let cfg =
        Config::new("https://example.uz", "id", "secret")?.with_timeout(Duration::from_secs(60));

    assert_eq!(cfg.timeout(), Duration::from_secs(60));
    Ok(())
}

#[test]
fn with_connect_timeout_overrides_default() -> MyIdResult<()> {
    let cfg = Config::new("https://example.uz", "id", "secret")?
        .with_connect_timeout(Duration::from_secs(10));

    assert_eq!(cfg.connection_timeout(), Duration::from_secs(10));
    Ok(())
}

#[test]
fn with_user_agent_overrides_default() -> MyIdResult<()> {
    let cfg =
        Config::new("https://example.uz", "id", "secret")?.with_user_agent("custom-agent/1.0");

    assert_eq!(cfg.user_agent(), "custom-agent/1.0");
    Ok(())
}

#[test]
fn with_proxy_sets_proxy_url() -> MyIdResult<()> {
    let cfg =
        Config::new("https://example.uz", "id", "secret")?.with_proxy("http://proxy.local:8080")?;

    // assert_eq! — None bo'lsa test FAIL bo'ladi
    assert_eq!(cfg.proxy_url(), Some("http://proxy.local:8080/"));
    Ok(())
}

#[test]
fn full_config_with_all_options() -> MyIdResult<()> {
    let cfg = Config::new("https://myid.example.uz", "app_id", "secret_123")?
        .with_timeout(Duration::from_secs(30))
        .with_connect_timeout(Duration::from_secs(5))
        .with_user_agent("my-service/2.0")
        .with_proxy("http://proxy.corp.local:8080")?;

    assert_eq!(cfg.base_url(), "https://myid.example.uz/");
    assert_eq!(cfg.client_id(), "app_id");
    assert_eq!(cfg.timeout(), Duration::from_secs(30));
    assert_eq!(cfg.connection_timeout(), Duration::from_secs(5));
    assert_eq!(cfg.user_agent(), "my-service/2.0");
    assert_eq!(cfg.proxy_url(), Some("http://proxy.corp.local:8080/"));
    Ok(())
}

// ===== Error cases =====

#[test]
fn empty_url_returns_error() {
    assert!(Config::new("", "id", "secret").is_err());
}

#[test]
fn invalid_url_returns_error() {
    assert!(Config::new("not-a-url", "id", "secret").is_err());
}

#[test]
fn ftp_scheme_rejected() {
    assert!(Config::new("ftp://example.uz", "id", "secret").is_err());
}

#[test]
fn invalid_proxy_url_returns_error() -> MyIdResult<()> {
    let result = Config::new("https://example.uz", "id", "secret")?.with_proxy("not-a-url");

    assert!(result.is_err());
    Ok(())
}

#[test]
fn ftp_proxy_rejected() -> MyIdResult<()> {
    let result =
        Config::new("https://example.uz", "id", "secret")?.with_proxy("ftp://proxy.local:8080");

    assert!(result.is_err());
    Ok(())
}
