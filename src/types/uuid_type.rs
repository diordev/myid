// UUID asosidagi newtype wrapper'lar uchun umumiy macro.

/// UUID asosidagi newtype yaratish uchun macro.
///
/// Yaratilgan struct: `Copy`, `Eq`, `Hash`, `Serialize`, `Deserialize`,
/// `Display`, `TryFrom<String>`, `Into<String>`.
macro_rules! define_uuid_type {
    (
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident;
        error_field = $field:literal;
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $vis struct $Name(uuid::Uuid);

        impl $Name {
            /// String qiymatdan parse qiladi.
            ///
            /// Hyphenated, simple va uppercase formatlar qabul qilinadi.
            /// Natija canonical ko'rinishda saqlanadi (lowercase + hyphenated).
            pub fn parse(value: impl AsRef<str>) -> $crate::error::MyIdResult<Self> {
                let value = value.as_ref().trim();
                let parsed = uuid::Uuid::parse_str(value).map_err(|_| {
                    $crate::error::MyIdError::validation(format!(
                        "{} must be a valid UUID, got: {value}",
                        $field,
                    ))
                })?;
                Ok(Self(parsed))
            }

            /// Yangi random UUID v4 generatsiya qiladi.
            pub fn generate() -> Self {
                Self(uuid::Uuid::new_v4())
            }

            /// Canonical string ko'rinishini qaytaradi (lowercase, hyphenated).
            pub fn as_str(&self) -> String {
                self.0.as_hyphenated().to_string()
            }

            /// Ichki [`Uuid`](uuid::Uuid) qiymatni qaytaradi.
            pub fn as_uuid(&self) -> &uuid::Uuid {
                &self.0
            }
        }

        impl std::fmt::Display for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0.as_hyphenated())
            }
        }

        impl TryFrom<String> for $Name {
            type Error = $crate::error::MyIdError;
            fn try_from(value: String) -> $crate::error::MyIdResult<Self> {
                Self::parse(value)
            }
        }

        impl From<$Name> for String {
            fn from(value: $Name) -> Self {
                value.to_string()
            }
        }

        impl serde::Serialize for $Name {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_str(&self.to_string())
            }
        }

        impl<'de> serde::Deserialize<'de> for $Name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                let s = String::deserialize(d)?;
                Self::parse(&s).map_err(serde::de::Error::custom)
            }
        }
    };
}

pub(crate) use define_uuid_type;

#[cfg(test)]
macro_rules! uuid_type_tests {
    ($Type:ident) => {
        const VALID: &str = "9b7e597e-893e-4e11-92cf-f4e7d4f923b1";
        const VALID_UPPER: &str = "9B7E597E-893E-4E11-92CF-F4E7D4F923B1";
        const VALID_SIMPLE: &str = "9b7e597e893e4e1192cff4e7d4f923b1";

        #[test]
        fn valid_hyphenated() {
            assert!($Type::parse(VALID).is_ok());
        }

        #[test]
        fn valid_simple() {
            let id = $Type::parse(VALID_SIMPLE).unwrap();
            assert_eq!(id.as_str(), VALID);
        }

        #[test]
        fn uppercase_normalized() {
            let id = $Type::parse(VALID_UPPER).unwrap();
            assert_eq!(id.as_str(), VALID);
        }

        #[test]
        fn whitespace_trimmed() {
            let id = $Type::parse(format!("  {VALID}  ")).unwrap();
            assert_eq!(id.as_str(), VALID);
        }

        #[test]
        fn invalid_rejected() {
            assert!($Type::parse("not-a-uuid").is_err());
            assert!($Type::parse("").is_err());
            assert!($Type::parse("   ").is_err());
            assert!($Type::parse("9b7e597e-893e-4e11-92cf").is_err());
        }

        #[test]
        fn generate_unique() {
            assert_ne!($Type::generate(), $Type::generate());
        }

        #[test]
        fn display_matches_as_str() {
            let id = $Type::parse(VALID).unwrap();
            assert_eq!(id.to_string(), id.as_str());
        }

        #[test]
        fn copy_semantics() {
            let a = $Type::parse(VALID).unwrap();
            let b = a;
            assert_eq!(a, b);
        }

        #[test]
        fn serde_round_trip() {
            let id = $Type::parse(VALID).unwrap();
            let json = serde_json::to_string(&id).unwrap();
            assert_eq!(json, format!("\"{VALID}\""));
            let back: $Type = serde_json::from_str(&json).unwrap();
            assert_eq!(id, back);
        }

        #[test]
        fn serde_invalid_rejected() {
            assert!(serde_json::from_str::<$Type>("\"invalid\"").is_err());
        }
    };
}

#[cfg(test)]
pub(crate) use uuid_type_tests;
