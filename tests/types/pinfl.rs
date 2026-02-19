use myid::error::MyIdError;
use myid::types::Pinfl;

#[test]
fn valid_pinfl_parses() {
    let p = Pinfl::parse("12345678901234").unwrap();
    assert_eq!(p.as_str(), "12345678901234");
}

#[test]
fn display_equals_as_str() {
    let p = Pinfl::parse("12345678901234").unwrap();
    assert_eq!(p.to_string(), p.as_str());
}

#[test]
fn short_pinfl_rejected() {
    assert!(matches!(
        Pinfl::parse("1234567890123").unwrap_err(),
        MyIdError::Validation { .. }
    ));
}

#[test]
fn long_pinfl_rejected() {
    assert!(Pinfl::parse("123456789012345").is_err());
}

#[test]
fn non_digit_rejected() {
    assert!(Pinfl::parse("1234567890123a").is_err());
}

#[test]
fn equality_and_hash() {
    let a = Pinfl::parse("12345678901234").unwrap();
    let b = Pinfl::parse("12345678901234").unwrap();
    assert_eq!(a, b);
}

#[test]
fn serde_roundtrip() {
    let p = Pinfl::parse("12345678901234").unwrap();
    let json = serde_json::to_string(&p).unwrap();
    let back: Pinfl = serde_json::from_str(&json).unwrap();
    assert_eq!(p, back);
}

#[test]
fn into_string_conversion() {
    let p = Pinfl::parse("12345678901234").unwrap();
    let s: String = p.into();
    assert_eq!(s, "12345678901234");
}
