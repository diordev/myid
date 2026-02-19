use myid::types::Threshold;

#[test]
fn min_value_valid() {
    assert!(Threshold::parse(0.5).is_ok());
}

#[test]
fn max_value_valid() {
    assert!(Threshold::parse(0.99).is_ok());
}

#[test]
fn mid_value_valid() {
    let t = Threshold::parse(0.75).unwrap();
    assert_eq!(t.as_f64(), 0.75);
}

#[test]
fn below_min_rejected() {
    assert!(Threshold::parse(0.49).is_err());
}

#[test]
fn above_max_rejected() {
    assert!(Threshold::parse(1.0).is_err());
}

#[test]
fn nan_rejected() {
    assert!(Threshold::parse(f64::NAN).is_err());
}

#[test]
fn infinity_rejected() {
    assert!(Threshold::parse(f64::INFINITY).is_err());
}

#[test]
fn serde_roundtrip() {
    let t = Threshold::parse(0.85).unwrap();
    let json = serde_json::to_string(&t).unwrap();
    let back: Threshold = serde_json::from_str(&json).unwrap();
    assert_eq!(t, back);
}

#[test]
fn copy_semantics() {
    let a = Threshold::parse(0.9).unwrap();
    let b = a;
    assert_eq!(a, b);
}
