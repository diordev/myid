use myid::version;

#[test]
fn version_test() {
    let v = version();
    assert!(!v.is_empty());
    assert_eq!(v, "0.1.1");
}
