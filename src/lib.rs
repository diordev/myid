//! myid crate - initial release, API is under active development.

/// Crate version helper.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn version_test() {
        let version = version();
        assert!(!version.is_empty());
        assert_eq!(version, "0.1.0");
    }
    
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}