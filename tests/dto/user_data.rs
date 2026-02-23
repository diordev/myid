use myid::dto::UserDataResponse;
use serde_json::json;

#[test]
fn primary_response_parses_encrypted_pass_data_and_extended_common_fields() {
    let payload = json!({
        "data": {
            "comparison_value": 0.92,
            "pass_data": "encrypted-pass-data-blob",
            "job_id": "550e8400-e29b-41d4-a716-446655440000",
            "profile": {
                "common_data": {
                    "first_name": "ALI",
                    "middle_name": "VALIYEVICH",
                    "last_name": "VALIYEV",
                    "first_name_en": "ALI",
                    "last_name_en": "VALIYEV",
                    "pinfl": "12345678901234",
                    "gender": "M",
                    "birth_place": "Tashkent",
                    "birth_country": "Uzbekistan",
                    "birth_country_id": "860",
                    "birth_country_id_cbu": "860",
                    "birth_date": "1990-05-15",
                    "nationality": "Uzbek",
                    "nationality_id": "001",
                    "nationality_id_cbu": "001",
                    "citizenship": "Uzbekistan",
                    "citizenship_id": "860",
                    "citizenship_id_cbu": "860",
                    "sdk_hash": "hash-value",
                    "last_update_pass_data": "2025-01-01T00:00:00Z",
                    "last_update_address": "2025-01-01T00:00:00Z"
                },
                "doc_data": {
                    "pass_data": "AB1234567",
                    "issued_by": "IIB",
                    "issued_by_id": "123",
                    "issued_date": "2020-01-01",
                    "expiry_date": "2030-01-01",
                    "doc_type": "passport",
                    "doc_type_id": "46",
                    "doc_type_id_cbu": "46"
                },
                "contacts": null,
                "address": null
            }
        },
        "reuid": {
            "expires_at": 2147483648i64,
            "value": "9b7e597e-893e-4e11-92cf-f4e7d4f923b1"
        }
    });

    let resp: UserDataResponse = serde_json::from_value(payload).unwrap();

    assert_eq!(resp.data.pass_data, "encrypted-pass-data-blob");
    let profile = resp.data.profile.as_ref().expect("profile bo'lishi kerak");
    assert_eq!(
        profile.common_data.birth_country.as_deref(),
        Some("Uzbekistan")
    );
    assert_eq!(profile.common_data.nationality_id.as_deref(), Some("001"));
    assert_eq!(
        profile.common_data.citizenship_id_cbu.as_deref(),
        Some("860")
    );
    assert_eq!(resp.reuid.as_ref().unwrap().expires_at, 2147483648i64);
}

#[test]
fn primary_response_parses_when_new_optional_fields_are_missing() {
    let payload = json!({
        "data": {
            "comparison_value": 0.91,
            "pass_data": "encrypted-pass-data",
            "job_id": "550e8400-e29b-41d4-a716-446655440000",
            "profile": {
                "common_data": {
                    "first_name": "ALI",
                    "middle_name": null,
                    "last_name": "VALIYEV",
                    "first_name_en": null,
                    "last_name_en": null,
                    "pinfl": "12345678901234",
                    "gender": "M",
                    "birth_place": "Tashkent",
                    "birth_date": "1990-05-15",
                    "nationality": "Uzbek",
                    "citizenship": "Uzbekistan",
                    "sdk_hash": "hash-value",
                    "last_update_pass_data": "2025-01-01T00:00:00Z",
                    "last_update_address": "2025-01-01T00:00:00Z"
                },
                "doc_data": {
                    "pass_data": "AB1234567",
                    "issued_by": "IIB",
                    "issued_by_id": "123",
                    "issued_date": "2020-01-01",
                    "expiry_date": "2030-01-01",
                    "doc_type": "passport",
                    "doc_type_id": "46",
                    "doc_type_id_cbu": "46"
                },
                "contacts": null,
                "address": null
            }
        },
        "reuid": null
    });

    let resp: UserDataResponse = serde_json::from_value(payload).unwrap();

    let profile = resp.data.profile.as_ref().expect("profile bo'lishi kerak");
    assert_eq!(profile.common_data.birth_country, None);
    assert_eq!(profile.common_data.birth_country_id, None);
    assert_eq!(profile.common_data.birth_country_id_cbu, None);
    assert_eq!(profile.common_data.nationality_id, None);
    assert_eq!(profile.common_data.nationality_id_cbu, None);
    assert_eq!(profile.common_data.citizenship_id, None);
    assert_eq!(profile.common_data.citizenship_id_cbu, None);
}

#[test]
fn secondary_response_parses_with_profile_and_reuid_null() {
    let payload = json!({
        "data": {
            "comparison_value": 0.77,
            "pass_data": "RAW-SECONDARY-DATA",
            "job_id": "550e8400-e29b-41d4-a716-446655440000",
            "profile": null
        },
        "reuid": null
    });

    let resp: UserDataResponse = serde_json::from_value(payload).unwrap();

    assert_eq!(resp.data.pass_data, "RAW-SECONDARY-DATA");
    assert!(resp.data.profile.is_none());
    assert!(resp.reuid.is_none());
}
