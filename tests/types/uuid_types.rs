use myid::types::{JobId, Reuid, SessionId};

#[test]
fn session_id_parses_hyphenated() {
    let id = SessionId::parse("9b7e597e-893e-4e11-92cf-f4e7d4f923b1").unwrap();
    assert_eq!(id.as_str(), "9b7e597e-893e-4e11-92cf-f4e7d4f923b1");
}

#[test]
fn session_id_parses_uppercase() {
    let lower = SessionId::parse("9b7e597e-893e-4e11-92cf-f4e7d4f923b1").unwrap();
    let upper = SessionId::parse("9B7E597E-893E-4E11-92CF-F4E7D4F923B1").unwrap();
    assert_eq!(lower, upper);
}

#[test]
fn session_id_generate_is_unique() {
    let a = SessionId::generate();
    let b = SessionId::generate();
    assert_ne!(a, b);
}

#[test]
fn reuid_parses_valid() {
    assert!(Reuid::parse("9b7e597e-893e-4e11-92cf-f4e7d4f923b1").is_ok());
}

#[test]
fn reuid_invalid_rejected() {
    assert!(Reuid::parse("not-a-uuid").is_err());
    assert!(Reuid::parse("").is_err());
}

#[test]
fn non_v4_uuid_rejected() {
    // UUID v1 namunasi
    let v1 = "6ba7b810-9dad-11d1-80b4-00c04fd430c8";
    assert!(SessionId::parse(v1).is_err());
    assert!(Reuid::parse(v1).is_err());
    assert!(JobId::parse(v1).is_err());
}

#[test]
fn job_id_generate_and_roundtrip() {
    let id = JobId::generate();
    let s = id.to_string();
    let back = JobId::parse(&s).unwrap();
    assert_eq!(id, back);
}

#[test]
fn uuid_serde_roundtrip() {
    let id = SessionId::generate();
    let json = serde_json::to_string(&id).unwrap();
    let back: SessionId = serde_json::from_str(&json).unwrap();
    assert_eq!(id, back);
}

#[test]
fn uuid_into_string() {
    let id = Reuid::generate();
    let s: String = id.into();
    assert_eq!(s.len(), 36); // hyphenated UUID
}
