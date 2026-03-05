use myid::types::BirthDate;

#[test]
fn valid_date_parses() {
    let d = BirthDate::parse("1990-05-15").unwrap();
    assert_eq!(d.to_string(), "1990-05-15");
}

#[test]
fn display_matches_formatted() {
    let d = BirthDate::parse("2000-01-01").unwrap();
    assert_eq!(d.to_string(), "2000-01-01");
}

#[test]
fn invalid_format_rejected() {
    assert!(BirthDate::parse("15-05-1990").is_err());
    assert!(BirthDate::parse("1990/05/15").is_err());
}

#[test]
fn invalid_month_rejected() {
    assert!(BirthDate::parse("1990-13-01").is_err());
}

#[test]
fn invalid_day_rejected() {
    assert!(BirthDate::parse("1990-01-32").is_err());
}

#[test]
fn leap_year_feb29_valid() {
    assert!(BirthDate::parse("2000-02-29").is_ok());
}

#[test]
fn non_leap_year_feb29_rejected() {
    assert!(BirthDate::parse("1999-02-29").is_err());
}

#[test]
fn copy_semantics() {
    let a = BirthDate::parse("1985-03-20").unwrap();
    let b = a; // Copy, not move
    assert_eq!(a, b);
}

#[test]
fn serde_roundtrip() {
    let d = BirthDate::parse("1990-05-15").unwrap();
    let json = serde_json::to_string(&d).unwrap();
    let back: BirthDate = serde_json::from_str(&json).unwrap();
    assert_eq!(d, back);
}

#[test]
fn into_string_conversion() {
    let d = BirthDate::parse("1990-05-15").unwrap();
    let s: String = d.into();
    assert_eq!(s, "1990-05-15");
}
