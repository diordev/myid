use myid::types::PhoneNumber;

#[test]
fn with_plus_prefix_parses() {
    let p = PhoneNumber::parse("+998901234567").unwrap();
    assert_eq!(p.as_str(), "998901234567");
}

#[test]
fn without_plus_parses() {
    let p = PhoneNumber::parse("998901234567").unwrap();
    assert_eq!(p.as_str(), "998901234567");
}

#[test]
fn international_format_adds_plus() {
    let p = PhoneNumber::parse("998901234567").unwrap();
    assert_eq!(p.as_international(), "+998901234567");
}

#[test]
fn short_number_rejected() {
    assert!(PhoneNumber::parse("99890123456").is_err());
}

#[test]
fn long_number_rejected() {
    assert!(PhoneNumber::parse("9989012345678").is_err());
}

#[test]
fn non_digit_rejected() {
    assert!(PhoneNumber::parse("+99890123456a").is_err());
}

#[test]
fn equality() {
    let a = PhoneNumber::parse("+998901234567").unwrap();
    let b = PhoneNumber::parse("998901234567").unwrap();
    assert_eq!(a, b);
}

#[test]
fn serde_roundtrip() {
    let p = PhoneNumber::parse("+998901234567").unwrap();
    let json = serde_json::to_string(&p).unwrap();
    let back: PhoneNumber = serde_json::from_str(&json).unwrap();
    assert_eq!(p, back);
}
