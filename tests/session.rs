use myid::prelude::*;

// ─── Serialize ───

#[test]
fn pinfl_serializes_required_fields() {
    let req = CreateSessionRequest::WithPinfl(SessionWithPinfl::new(
        Pinfl::parse("12345678901234").unwrap(),
        BirthDate::parse("1990-05-15").unwrap(),
    ));
    let json = serde_json::to_string(&req).unwrap();

    assert!(json.contains("\"pinfl\""));
    assert!(json.contains("\"birth_date\""));
    // Optional fieldlar yo'q
    assert!(!json.contains("\"phone_number\""));
    assert!(!json.contains("\"threshold\""));
}

#[test]
fn passport_serializes_required_fields() {
    let req = CreateSessionRequest::WithPassport(SessionWithPassport::new(
        PassportData::parse("AB1234567").unwrap(),
        BirthDate::parse("1990-05-15").unwrap(),
    ));
    let json = serde_json::to_string(&req).unwrap();

    assert!(json.contains("\"pass_data\""));
    assert!(json.contains("\"birth_date\""));
}

#[test]
fn optional_fields_appear_when_set() {
    let req = SessionWithPinfl::new(
        Pinfl::parse("12345678901234").unwrap(),
        BirthDate::parse("1990-05-15").unwrap(),
    )
    .with_phone_number(PhoneNumber::parse("+998901234567").unwrap())
    .with_threshold(Threshold::parse(0.85).unwrap())
    .with_is_resident(true);

    let json = serde_json::to_string(&req).unwrap();

    assert!(json.contains("\"phone_number\""));
    assert!(json.contains("\"threshold\""));
    assert!(json.contains("\"is_resident\""));
}

#[test]
fn reuid_serializes_minimal() {
    let req = CreateSessionRequest::WithReuid(SessionWithReuid::new(
        Reuid::parse("9b7e597e-893e-4e11-92cf-f4e7d4f923b1").unwrap(),
    ));
    let json = serde_json::to_string(&req).unwrap();

    assert!(json.contains("\"reuid\""));
    assert!(!json.contains("\"phone_number\""));
}

#[test]
fn empty_serializes_as_empty_object() {
    let req = CreateSessionRequest::Empty {};
    let json = serde_json::to_string(&req).unwrap();
    assert_eq!(json, "{}");
}

// ─── Getters ───

#[test]
fn pinfl_getters() {
    let pinfl = Pinfl::parse("12345678901234").unwrap();
    let birth = BirthDate::parse("1990-05-15").unwrap();
    let phone = PhoneNumber::parse("+998901234567").unwrap();

    let req = SessionWithPinfl::new(pinfl.clone(), birth)
        .with_phone_number(phone)
        .with_is_resident(true)
        .with_threshold(Threshold::parse(0.85).unwrap());

    assert_eq!(req.pinfl().as_str(), "12345678901234");
    assert_eq!(req.birth_date().as_str(), "1990-05-15");
    assert_eq!(
        req.phone_number().map(PhoneNumber::as_str),
        Some("998901234567")
    );
    assert_eq!(req.is_resident(), Some(true));
    assert_eq!(req.threshold().map(Threshold::as_f64), Some(0.85));
}

#[test]
fn passport_getters() {
    let req = SessionWithPassport::new(
        PassportData::parse("AB1234567").unwrap(),
        BirthDate::parse("2000-01-01").unwrap(),
    );

    assert_eq!(req.pass_data().as_str(), "AB1234567");
    assert_eq!(req.birth_date().as_str(), "2000-01-01");
    assert_eq!(req.phone_number(), None);
}

// ─── Response deserialization ───

#[test]
fn session_status_parses_snake_and_upper() {
    use myid::dto::SessionStatus;

    let snake: SessionStatus = serde_json::from_str("\"in_progress\"").unwrap();
    let upper: SessionStatus = serde_json::from_str("\"IN_PROGRESS\"").unwrap();

    assert_eq!(snake, SessionStatus::InProgress);
    assert_eq!(upper, SessionStatus::InProgress);
    assert!(snake.is_in_progress());
}

#[test]
fn session_status_all_variants() {
    use myid::dto::SessionStatus;
    
    let closed: SessionStatus = serde_json::from_str("\"closed\"").unwrap();
    let expired: SessionStatus = serde_json::from_str("\"expired\"").unwrap();

    assert!(closed.is_closed());
    assert!(expired.is_expired());
}
