use myid::error::{MyIdError, MyIdResult};

#[test]
fn config_error_display() {
    let err = MyIdError::config("noto'g'ri URL");
    assert_eq!(err.to_string(), "config error: noto'g'ri URL");
}

#[test]
fn validation_error_display() {
    let err = MyIdError::validation("maydon bo'sh");
    assert_eq!(err.to_string(), "validation error: maydon bo'sh");
}

#[test]
fn api_error_display() {
    let err = MyIdError::api(401, "unauthorized");
    assert_eq!(err.to_string(), "api error 401: unauthorized");
}

#[test]
fn internal_error_display() {
    let err = MyIdError::Internal {
        message: "lock poisoned".into(),
    };
    assert_eq!(err.to_string(), "internal error: lock poisoned");
}

#[test]
fn is_api_true_for_api_variant() {
    let err = MyIdError::api(403, "forbidden");
    assert!(err.is_api());
}

#[test]
fn is_api_false_for_other_variants() {
    assert!(!MyIdError::config("x").is_api());
    assert!(!MyIdError::validation("x").is_api());
    assert!(
        !MyIdError::Internal {
            message: "x".into()
        }
        .is_api()
    );
}

#[test]
fn api_status_returns_code() {
    let err = MyIdError::api(429, "rate limited");
    assert_eq!(err.api_status(), Some(429));
}

#[test]
fn api_status_none_for_non_api() {
    assert_eq!(MyIdError::config("x").api_status(), None);
    assert_eq!(
        MyIdError::Internal {
            message: "x".into()
        }
        .api_status(),
        None
    );
}

#[test]
fn result_alias_works() {
    fn ok_fn() -> MyIdResult<i32> {
        Ok(42)
    }
    fn err_fn() -> MyIdResult<i32> {
        Err(MyIdError::config("fail"))
    }
    assert_eq!(ok_fn().unwrap(), 42);
    assert!(err_fn().is_err());
}
