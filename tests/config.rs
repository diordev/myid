use std::{self, time::Duration};

use myid::config::{Config, DEFAULT_CONNECT_TIMEOUT_MS, DEFAULT_TIMEOUT_MS};
use myid::prelude::*;

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

// ===== Trailing slash normalization =====

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

// ===== Config::from_env() =====

/// Test uchun env o'zgaruvchilarini o'rnatadi va test tugaganda tozalaydi.
struct EnvGuard {
    keys: Vec<String>,
}

impl EnvGuard {
    fn new(prefix: &str, vars: &[(&str, &str)]) -> Self {
        let keys: Vec<String> = vars
            .iter()
            .map(|(k, v)| {
                let key = format!("{prefix}{k}");
                // Safety: testlar serial ishga tushiriladi,
                // har bir test noyob prefix ishlatadi (TEST1_, TEST2_...)
                unsafe { std::env::set_var(&key, v) };
                key
            })
            .collect();

        EnvGuard { keys }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for key in &self.keys {
            // Safety: faqat o'zimiz yaratgan env varlarni tozalaymiz
            unsafe { std::env::remove_var(key) };
        }
    }
}

#[test]
fn from_env_with_required_fields() -> MyIdResult<()> {
    let _guard = EnvGuard::new(
        "TEST1_",
        &[
            ("BASE_URL", "https://myid.example.uz"),
            ("CLIENT_ID", "env_app_id"),
            ("CLIENT_SECRET", "env_secret"),
        ],
    );

    let cfg = Config::from_env(Some("TEST1"))?;

    assert_eq!(cfg.base_url(), "https://myid.example.uz/");
    assert_eq!(cfg.client_id(), "env_app_id");
    assert_eq!(cfg.client_secret(), "env_secret");
    assert_eq!(cfg.timeout(), Duration::from_millis(DEFAULT_TIMEOUT_MS));
    assert_eq!(cfg.proxy_url(), None);
    Ok(())
}

#[test]
fn from_env_with_all_options() -> MyIdResult<()> {
    let _guard = EnvGuard::new(
        "TEST2_",
        &[
            ("BASE_URL", "https://myid.example.uz"),
            ("CLIENT_ID", "env_id"),
            ("CLIENT_SECRET", "env_secret"),
            ("CONNECT_TIMEOUT_MS", "5000"),
            ("TIMEOUT_MS", "30000"),
            ("USER_AGENT", "env-agent/1.0"),
            ("PROXY_URL", "http://proxy:8080"),
        ],
    );

    let cfg = Config::from_env(Some("TEST2"))?;

    assert_eq!(cfg.timeout(), Duration::from_secs(30));
    assert_eq!(cfg.connection_timeout(), Duration::from_secs(5));
    assert_eq!(cfg.user_agent(), "env-agent/1.0");
    assert_eq!(cfg.proxy_url(), Some("http://proxy:8080/"));
    Ok(())
}

#[test]
fn from_env_missing_required_var_returns_error() {
    let _guard = EnvGuard::new(
        "TEST3_",
        &[
            ("BASE_URL", "https://example.uz"),
            // CLIENT_ID va CLIENT_SECRET yo'q
        ],
    );

    assert!(Config::from_env(Some("TEST3")).is_err());
}

#[test]
fn from_env_empty_value_returns_error() {
    let _guard = EnvGuard::new(
        "TEST4_",
        &[
            ("BASE_URL", "   "),
            ("CLIENT_ID", "id"),
            ("CLIENT_SECRET", "secret"),
        ],
    );

    assert!(Config::from_env(Some("TEST4")).is_err());
}

#[test]
fn from_env_invalid_timeout_returns_error() {
    let _guard = EnvGuard::new(
        "TEST5_",
        &[
            ("BASE_URL", "https://example.uz"),
            ("CLIENT_ID", "id"),
            ("CLIENT_SECRET", "secret"),
            ("TIMEOUT_MS", "not_a_number"),
        ],
    );

    assert!(Config::from_env(Some("TEST5")).is_err());
}

#[test]
fn from_env_zero_timeout_returns_error() {
    let _guard = EnvGuard::new(
        "TEST6_",
        &[
            ("BASE_URL", "https://example.uz"),
            ("CLIENT_ID", "id"),
            ("CLIENT_SECRET", "secret"),
            ("TIMEOUT_MS", "0"),
        ],
    );

    assert!(Config::from_env(Some("TEST6")).is_err());
}

#[test]
fn from_env_invalid_url_returns_error() {
    let _guard = EnvGuard::new(
        "TEST7_",
        &[
            ("BASE_URL", "not-a-url"),
            ("CLIENT_ID", "id"),
            ("CLIENT_SECRET", "secret"),
        ],
    );

    assert!(Config::from_env(Some("TEST7")).is_err());
}

// ===== Error cases (Config::new) =====

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
