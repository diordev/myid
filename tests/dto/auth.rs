use myid::dto::{AccessTokenRequest, AccessTokenResponse};

// ===== AccessTokenRequest =====

#[test]
fn request_serializes_both_fields() {
    let req = AccessTokenRequest {
        client_id: "test_id",
        client_secret: "test_secret",
    };

    let json = serde_json::to_string(&req).unwrap();

    assert!(json.contains("\"client_id\":\"test_id\""));
    assert!(json.contains("\"client_secret\":\"test_secret\""));
}

#[test]
fn request_debug_redacts_secret() {
    let req = AccessTokenRequest {
        client_id: "my_app",
        client_secret: "super_secret_123",
    };

    let debug = format!("{:?}", req);

    assert!(debug.contains("my_app"));
    assert!(debug.contains("[REDACTED]"));
    assert!(!debug.contains("super_secret_123"));
}

#[test]
fn request_clone_works() {
    let req = AccessTokenRequest {
        client_id: "id",
        client_secret: "secret",
    };

    let cloned = req.clone();

    assert_eq!(cloned.client_id, "id");
    assert_eq!(cloned.client_secret, "secret");
}

// ===== AccessTokenResponse =====

#[test]
fn response_deserializes_from_json() {
    let json = r#"{
        "access_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9",
        "expires_in": 3600,
        "token_type": "Bearer"
    }"#;

    let resp: AccessTokenResponse = serde_json::from_str(json).unwrap();

    assert_eq!(resp.access_token, "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9");
    assert_eq!(resp.expires_in, 3600);
    assert_eq!(resp.token_type, "Bearer");
}

#[test]
fn response_debug_redacts_token() {
    let json = r#"{
        "access_token": "secret_token_value",
        "expires_in": 7200,
        "token_type": "Bearer"
    }"#;

    let resp: AccessTokenResponse = serde_json::from_str(json).unwrap();
    let debug = format!("{:?}", resp);

    assert!(debug.contains("[REDACTED]"));
    assert!(!debug.contains("secret_token_value"));
    assert!(debug.contains("7200"));
    assert!(debug.contains("Bearer"));
}

#[test]
fn response_ignores_unknown_fields() {
    let json = r#"{
        "access_token": "token123",
        "expires_in": 1800,
        "token_type": "Bearer",
        "scope": "read write",
        "extra_field": true
    }"#;

    let resp: AccessTokenResponse = serde_json::from_str(json).unwrap();

    assert_eq!(resp.access_token, "token123");
    assert_eq!(resp.expires_in, 1800);
}

#[test]
fn response_missing_field_fails() {
    // access_token yo'q
    let json = r#"{
        "expires_in": 3600,
        "token_type": "Bearer"
    }"#;

    assert!(serde_json::from_str::<AccessTokenResponse>(json).is_err());
}

#[test]
fn response_clone_works() {
    let json = r#"{
        "access_token": "abc",
        "expires_in": 60,
        "token_type": "Bearer"
    }"#;

    let resp: AccessTokenResponse = serde_json::from_str(json).unwrap();
    let cloned = resp.clone();

    assert_eq!(cloned.access_token, "abc");
    assert_eq!(cloned.expires_in, 60);
}
