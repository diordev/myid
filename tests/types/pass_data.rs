use myid::types::PassportData;

#[test]
fn valid_passport_parses() {
    let p = PassportData::parse("AB1234567").unwrap();
    assert_eq!(p.as_str(), "AB1234567");
}

#[test]
fn lowercase_normalized_to_uppercase() {
    let p = PassportData::parse("ab1234567").unwrap();
    assert_eq!(p.as_str(), "AB1234567");
}

#[test]
fn series_and_number_split() {
    let p = PassportData::parse("AB1234567").unwrap();
    assert_eq!(p.series(), "AB");
    assert_eq!(p.number(), "1234567");
}

#[test]
fn too_short_rejected() {
    assert!(PassportData::parse("AB123456").is_err());
}

#[test]
fn too_long_rejected() {
    assert!(PassportData::parse("AB12345678").is_err());
}

#[test]
fn digit_in_series_rejected() {
    assert!(PassportData::parse("1B1234567").is_err());
}

#[test]
fn letter_in_number_rejected() {
    assert!(PassportData::parse("AB123456A").is_err());
}

#[test]
fn serde_roundtrip() {
    let p = PassportData::parse("CD9876543").unwrap();
    let json = serde_json::to_string(&p).unwrap();
    let back: PassportData = serde_json::from_str(&json).unwrap();
    assert_eq!(p, back);
}

#[test]
fn into_string_conversion() {
    let p = PassportData::parse("AB1234567").unwrap();
    let s: String = p.into();
    assert_eq!(s, "AB1234567");
}
