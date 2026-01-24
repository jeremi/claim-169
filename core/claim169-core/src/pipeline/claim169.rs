use std::collections::HashMap;
use std::collections::HashSet;

use ciborium::Value;

use crate::error::{Claim169Error, Result};
use crate::model::{
    Biometric, BiometricFormat, BiometricSubFormat, Claim169, Gender, MaritalStatus, PhotoFormat,
};

/// Transform a Claim 169 CBOR map (with integer keys) into the canonical Claim169 struct
pub fn transform(value: Value, skip_biometrics: bool) -> Result<Claim169> {
    let map = match value {
        Value::Map(m) => m,
        _ => {
            return Err(Claim169Error::Claim169Invalid(
                "claim 169 is not a CBOR map".to_string(),
            ))
        }
    };

    let mut claim = Claim169::default();
    let mut unknown_fields: HashMap<i64, serde_json::Value> = HashMap::new();
    let mut seen_keys: HashSet<i64> = HashSet::new();

    for (key, val) in map {
        let key_int = match &key {
            Value::Integer(i) => {
                let i128_val = i128::from(*i);
                i64::try_from(i128_val).map_err(|_| {
                    Claim169Error::Claim169Invalid(format!(
                        "claim 169 key {} is out of valid range",
                        i128_val
                    ))
                })?
            }
            _ => {
                // Non-integer keys are invalid per spec
                return Err(Claim169Error::Claim169Invalid(
                    "claim 169 keys must be integers".to_string(),
                ));
            }
        };

        if !seen_keys.insert(key_int) {
            return Err(Claim169Error::Claim169Invalid(format!(
                "duplicate claim 169 key: {key_int}"
            )));
        }

        match key_int {
            // Demographics
            1 => claim.id = extract_string(&val),
            2 => claim.version = extract_string(&val),
            3 => claim.language = extract_string(&val),
            4 => claim.full_name = extract_string(&val),
            5 => claim.first_name = extract_string(&val),
            6 => claim.middle_name = extract_string(&val),
            7 => claim.last_name = extract_string(&val),
            8 => claim.date_of_birth = extract_string(&val),
            9 => claim.gender = extract_int(&val).and_then(|i| Gender::try_from(i).ok()),
            10 => claim.address = extract_string(&val),
            11 => claim.email = extract_string(&val),
            12 => claim.phone = extract_string(&val),
            13 => claim.nationality = extract_string(&val),
            14 => {
                claim.marital_status =
                    extract_int(&val).and_then(|i| MaritalStatus::try_from(i).ok())
            }
            15 => claim.guardian = extract_string(&val),
            16 => claim.photo = extract_bytes(&val),
            17 => {
                claim.photo_format = extract_int(&val).and_then(|i| PhotoFormat::try_from(i).ok())
            }
            18 => claim.best_quality_fingers = extract_best_quality_fingers(&val),
            19 => claim.secondary_full_name = extract_string(&val),
            20 => claim.secondary_language = extract_string(&val),
            21 => claim.location_code = extract_string(&val),
            22 => claim.legal_status = extract_string(&val),
            23 => claim.country_of_issuance = extract_string(&val),

            // Biometrics (keys 50-65)
            50 if !skip_biometrics => claim.right_thumb = extract_biometrics(&val),
            51 if !skip_biometrics => claim.right_pointer_finger = extract_biometrics(&val),
            52 if !skip_biometrics => claim.right_middle_finger = extract_biometrics(&val),
            53 if !skip_biometrics => claim.right_ring_finger = extract_biometrics(&val),
            54 if !skip_biometrics => claim.right_little_finger = extract_biometrics(&val),
            55 if !skip_biometrics => claim.left_thumb = extract_biometrics(&val),
            56 if !skip_biometrics => claim.left_pointer_finger = extract_biometrics(&val),
            57 if !skip_biometrics => claim.left_middle_finger = extract_biometrics(&val),
            58 if !skip_biometrics => claim.left_ring_finger = extract_biometrics(&val),
            59 if !skip_biometrics => claim.left_little_finger = extract_biometrics(&val),
            60 if !skip_biometrics => claim.right_iris = extract_biometrics(&val),
            61 if !skip_biometrics => claim.left_iris = extract_biometrics(&val),
            62 if !skip_biometrics => claim.face = extract_biometrics(&val),
            63 if !skip_biometrics => claim.right_palm = extract_biometrics(&val),
            64 if !skip_biometrics => claim.left_palm = extract_biometrics(&val),
            65 if !skip_biometrics => claim.voice = extract_biometrics(&val),

            // Skip biometric keys when skip_biometrics is true
            50..=65 if skip_biometrics => {}

            // Unknown/future fields - preserve them
            _ => {
                if let Some(json_val) = cbor_to_json(&val) {
                    unknown_fields.insert(key_int, json_val);
                }
            }
        }
    }

    claim.unknown_fields = unknown_fields;
    Ok(claim)
}

/// Extract a string from a CBOR value
fn extract_string(val: &Value) -> Option<String> {
    match val {
        Value::Text(s) => Some(s.clone()),
        _ => None,
    }
}

/// Extract an integer from a CBOR value
fn extract_int(val: &Value) -> Option<i64> {
    match val {
        Value::Integer(i) => i64::try_from(i128::from(*i)).ok(),
        _ => None,
    }
}

/// Extract bytes from a CBOR value
fn extract_bytes(val: &Value) -> Option<Vec<u8>> {
    match val {
        Value::Bytes(b) => Some(b.clone()),
        // Some implementations encode as hex string
        Value::Text(s) => hex::decode(s).ok(),
        _ => None,
    }
}

/// Extract best quality fingers array with range validation (0-10)
/// Per Claim 169 spec: 0=Unknown, 1-5=Right thumb to little finger, 6-10=Left thumb to little finger
fn extract_best_quality_fingers(val: &Value) -> Option<Vec<u8>> {
    match val {
        Value::Array(arr) => {
            let mut result = Vec::new();
            for item in arr {
                if let Value::Integer(i) = item {
                    if let Ok(v) = u8::try_from(i128::from(*i)) {
                        // Only accept values 0-10 per spec
                        if v <= 10 {
                            result.push(v);
                        }
                        // Invalid values (> 10) are silently dropped
                    }
                }
            }
            if result.is_empty() {
                None
            } else {
                Some(result)
            }
        }
        _ => None,
    }
}

/// Extract biometrics array from a CBOR value
fn extract_biometrics(val: &Value) -> Option<Vec<Biometric>> {
    match val {
        // Single biometric as a map
        Value::Map(_) => extract_single_biometric(val).map(|b| vec![b]),
        // Array of biometrics
        Value::Array(arr) => {
            let biometrics: Vec<Biometric> =
                arr.iter().filter_map(extract_single_biometric).collect();
            if biometrics.is_empty() {
                None
            } else {
                Some(biometrics)
            }
        }
        _ => None,
    }
}

/// Extract a single biometric entry from a CBOR map
fn extract_single_biometric(val: &Value) -> Option<Biometric> {
    let map = match val {
        Value::Map(m) => m,
        _ => return None,
    };

    let mut data: Option<Vec<u8>> = None;
    let mut format: Option<BiometricFormat> = None;
    let mut sub_format_raw: Option<i64> = None;
    let mut issuer: Option<String> = None;

    for (key, val) in map {
        let key_int = match key {
            Value::Integer(i) => i64::try_from(i128::from(*i)).ok()?,
            _ => continue,
        };

        match key_int {
            0 => data = extract_bytes(val),
            1 => format = extract_int(val).and_then(|i| BiometricFormat::try_from(i).ok()),
            2 => sub_format_raw = extract_int(val),
            3 => issuer = extract_string(val),
            _ => {}
        }
    }

    let data = data?;

    let sub_format = match (format, sub_format_raw) {
        (Some(f), Some(raw)) => Some(BiometricSubFormat::from_format_and_value(f, raw)),
        _ => None,
    };

    Some(Biometric {
        data,
        format,
        sub_format,
        issuer,
    })
}

/// Convert a CBOR value to JSON for unknown fields
fn cbor_to_json(val: &Value) -> Option<serde_json::Value> {
    match val {
        Value::Integer(i) => Some(serde_json::Value::Number(serde_json::Number::from(
            i64::try_from(i128::from(*i)).ok()?,
        ))),
        Value::Text(s) => Some(serde_json::Value::String(s.clone())),
        Value::Bool(b) => Some(serde_json::Value::Bool(*b)),
        Value::Null => Some(serde_json::Value::Null),
        Value::Float(f) => serde_json::Number::from_f64(*f).map(serde_json::Value::Number),
        Value::Bytes(b) => {
            use base64::Engine;
            Some(serde_json::Value::String(
                base64::engine::general_purpose::STANDARD.encode(b),
            ))
        }
        Value::Array(arr) => {
            let json_arr: Vec<serde_json::Value> = arr.iter().filter_map(cbor_to_json).collect();
            Some(serde_json::Value::Array(json_arr))
        }
        Value::Map(map) => {
            let mut json_map = serde_json::Map::new();
            for (k, v) in map {
                let key_str = match k {
                    Value::Text(s) => s.clone(),
                    Value::Integer(i) => i128::from(*i).to_string(),
                    _ => continue,
                };
                if let Some(json_val) = cbor_to_json(v) {
                    json_map.insert(key_str, json_val);
                }
            }
            Some(serde_json::Value::Object(json_map))
        }
        _ => None,
    }
}

/// Encode a Claim169 struct back to CBOR Value (for test vector generation)
pub fn to_cbor(claim: &Claim169) -> Value {
    let mut map: Vec<(Value, Value)> = Vec::new();

    macro_rules! add_string {
        ($key:expr, $field:expr) => {
            if let Some(ref v) = $field {
                map.push((Value::Integer($key.into()), Value::Text(v.clone())));
            }
        };
    }

    macro_rules! add_int {
        ($key:expr, $field:expr) => {
            if let Some(v) = $field {
                map.push((
                    Value::Integer($key.into()),
                    Value::Integer((v as i64).into()),
                ));
            }
        };
    }

    macro_rules! add_bytes {
        ($key:expr, $field:expr) => {
            if let Some(ref v) = $field {
                map.push((Value::Integer($key.into()), Value::Bytes(v.clone())));
            }
        };
    }

    // Demographics
    add_string!(1, claim.id);
    add_string!(2, claim.version);
    add_string!(3, claim.language);
    add_string!(4, claim.full_name);
    add_string!(5, claim.first_name);
    add_string!(6, claim.middle_name);
    add_string!(7, claim.last_name);
    add_string!(8, claim.date_of_birth);
    add_int!(9, claim.gender.map(|g| g as i64));
    add_string!(10, claim.address);
    add_string!(11, claim.email);
    add_string!(12, claim.phone);
    add_string!(13, claim.nationality);
    add_int!(14, claim.marital_status.map(|m| m as i64));
    add_string!(15, claim.guardian);
    add_bytes!(16, claim.photo);
    add_int!(17, claim.photo_format.map(|f| f as i64));

    if let Some(ref fingers) = claim.best_quality_fingers {
        let arr: Vec<Value> = fingers
            .iter()
            .map(|&f| Value::Integer((f as i64).into()))
            .collect();
        map.push((Value::Integer(18.into()), Value::Array(arr)));
    }

    add_string!(19, claim.secondary_full_name);
    add_string!(20, claim.secondary_language);
    add_string!(21, claim.location_code);
    add_string!(22, claim.legal_status);
    add_string!(23, claim.country_of_issuance);

    // Biometrics
    macro_rules! add_biometrics {
        ($key:expr, $field:expr) => {
            if let Some(ref biometrics) = $field {
                let arr: Vec<Value> = biometrics.iter().map(biometric_to_cbor).collect();
                map.push((Value::Integer($key.into()), Value::Array(arr)));
            }
        };
    }

    add_biometrics!(50, claim.right_thumb);
    add_biometrics!(51, claim.right_pointer_finger);
    add_biometrics!(52, claim.right_middle_finger);
    add_biometrics!(53, claim.right_ring_finger);
    add_biometrics!(54, claim.right_little_finger);
    add_biometrics!(55, claim.left_thumb);
    add_biometrics!(56, claim.left_pointer_finger);
    add_biometrics!(57, claim.left_middle_finger);
    add_biometrics!(58, claim.left_ring_finger);
    add_biometrics!(59, claim.left_little_finger);
    add_biometrics!(60, claim.right_iris);
    add_biometrics!(61, claim.left_iris);
    add_biometrics!(62, claim.face);
    add_biometrics!(63, claim.right_palm);
    add_biometrics!(64, claim.left_palm);
    add_biometrics!(65, claim.voice);

    Value::Map(map)
}

/// Convert a Biometric to CBOR
fn biometric_to_cbor(bio: &Biometric) -> Value {
    let mut map: Vec<(Value, Value)> = Vec::new();

    map.push((Value::Integer(0.into()), Value::Bytes(bio.data.clone())));

    if let Some(format) = bio.format {
        map.push((
            Value::Integer(1.into()),
            Value::Integer((format as i64).into()),
        ));
    }

    if let Some(ref sub_format) = bio.sub_format {
        map.push((
            Value::Integer(2.into()),
            Value::Integer(sub_format.to_value().into()),
        ));
    }

    if let Some(ref issuer) = bio.issuer {
        map.push((Value::Integer(3.into()), Value::Text(issuer.clone())));
    }

    Value::Map(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_claim169_cbor() -> Value {
        Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (
                Value::Integer(4.into()),
                Value::Text("Test User".to_string()),
            ),
            (Value::Integer(9.into()), Value::Integer(1.into())), // Male
            (
                Value::Integer(8.into()),
                Value::Text("19880102".to_string()),
            ),
        ])
    }

    #[test]
    fn test_transform_minimal() {
        let cbor = create_test_claim169_cbor();
        let claim = transform(cbor, false).unwrap();

        assert_eq!(claim.id, Some("12345".to_string()));
        assert_eq!(claim.full_name, Some("Test User".to_string()));
        assert_eq!(claim.gender, Some(Gender::Male));
        assert_eq!(claim.date_of_birth, Some("19880102".to_string()));
    }

    #[test]
    fn test_transform_with_biometrics() {
        let bio_map = Value::Map(vec![
            (Value::Integer(0.into()), Value::Bytes(vec![1, 2, 3, 4])),
            (Value::Integer(1.into()), Value::Integer(0.into())), // Image
            (Value::Integer(2.into()), Value::Integer(1.into())), // JPEG
        ]);

        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (Value::Integer(62.into()), Value::Array(vec![bio_map])),
        ]);

        let claim = transform(cbor, false).unwrap();
        assert!(claim.face.is_some());
        let face = claim.face.unwrap();
        assert_eq!(face.len(), 1);
        assert_eq!(face[0].data, vec![1, 2, 3, 4]);
        assert_eq!(face[0].format, Some(BiometricFormat::Image));
    }

    #[test]
    fn test_transform_skip_biometrics() {
        let bio_map = Value::Map(vec![(
            Value::Integer(0.into()),
            Value::Bytes(vec![1, 2, 3, 4]),
        )]);

        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (Value::Integer(62.into()), Value::Array(vec![bio_map])),
        ]);

        let claim = transform(cbor, true).unwrap();
        assert!(claim.face.is_none()); // Skipped
        assert_eq!(claim.id, Some("12345".to_string())); // Demographics still parsed
    }

    #[test]
    fn test_transform_unknown_fields() {
        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (
                Value::Integer(42.into()),
                Value::Text("future field".to_string()),
            ),
            (Value::Integer(99.into()), Value::Integer(999.into())),
        ]);

        let claim = transform(cbor, false).unwrap();
        assert!(claim.unknown_fields.contains_key(&42));
        assert!(claim.unknown_fields.contains_key(&99));
        assert_eq!(
            claim.unknown_fields.get(&42),
            Some(&serde_json::Value::String("future field".to_string()))
        );
    }

    #[test]
    fn test_transform_not_a_map() {
        let cbor = Value::Array(vec![Value::Integer(1.into())]);
        let result = transform(cbor, false);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Claim169Error::Claim169Invalid(_)
        ));
    }

    #[test]
    fn test_roundtrip() {
        let original = Claim169 {
            id: Some("12345".to_string()),
            full_name: Some("Test User".to_string()),
            gender: Some(Gender::Female),
            date_of_birth: Some("19900315".to_string()),
            email: Some("test@example.com".to_string()),
            face: Some(vec![
                Biometric::new(vec![1, 2, 3]).with_format(BiometricFormat::Image)
            ]),
            ..Default::default()
        };

        let cbor = to_cbor(&original);
        let parsed = transform(cbor, false).unwrap();

        assert_eq!(parsed.id, original.id);
        assert_eq!(parsed.full_name, original.full_name);
        assert_eq!(parsed.gender, original.gender);
        assert_eq!(parsed.email, original.email);
        assert!(parsed.face.is_some());
    }

    #[test]
    fn test_extract_best_quality_fingers_filters_invalid() {
        // Valid values: 0-10, Invalid values: > 10
        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (
                Value::Integer(18.into()),
                Value::Array(vec![
                    Value::Integer(1.into()),   // Valid: right thumb
                    Value::Integer(5.into()),   // Valid: right little finger
                    Value::Integer(10.into()),  // Valid: left little finger
                    Value::Integer(11.into()),  // Invalid: should be filtered
                    Value::Integer(255.into()), // Invalid: should be filtered
                    Value::Integer(0.into()),   // Valid: unknown
                ]),
            ),
        ]);

        let claim = transform(cbor, false).unwrap();
        let fingers = claim.best_quality_fingers.expect("should have fingers");

        // Only valid values (0-10) should be kept
        assert_eq!(fingers, vec![1, 5, 10, 0]);
        assert!(!fingers.contains(&11));
        assert!(!fingers.contains(&255));
    }

    #[test]
    fn test_extract_best_quality_fingers_empty_when_all_invalid() {
        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (
                Value::Integer(18.into()),
                Value::Array(vec![
                    Value::Integer(11.into()),  // Invalid
                    Value::Integer(100.into()), // Invalid
                ]),
            ),
        ]);

        let claim = transform(cbor, false).unwrap();
        // All values filtered out, so should be None
        assert!(claim.best_quality_fingers.is_none());
    }

    // ========== Non-integer Key Tests ==========
    #[test]
    fn test_transform_non_integer_key_returns_error() {
        let cbor = Value::Map(vec![(
            Value::Text("id".to_string()), // String key instead of integer
            Value::Text("12345".to_string()),
        )]);

        let result = transform(cbor, false);
        assert!(result.is_err());
        match result.unwrap_err() {
            Claim169Error::Claim169Invalid(msg) => {
                assert!(
                    msg.contains("integer"),
                    "Error should mention integer keys: {}",
                    msg
                );
            }
            e => panic!("Expected Claim169Invalid error, got: {:?}", e),
        }
    }

    #[test]
    fn test_transform_bytes_key_returns_error() {
        let cbor = Value::Map(vec![(
            Value::Bytes(vec![1, 2, 3]), // Bytes key instead of integer
            Value::Text("12345".to_string()),
        )]);

        let result = transform(cbor, false);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Claim169Error::Claim169Invalid(_)
        ));
    }

    // ========== Photo from Hex String Test ==========
    #[test]
    fn test_transform_photo_from_hex_string() {
        let hex_photo = "48656c6c6f"; // "Hello" in hex

        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (
                Value::Integer(16.into()),
                Value::Text(hex_photo.to_string()),
            ),
        ]);

        let claim = transform(cbor, false).unwrap();
        assert_eq!(claim.photo, Some(b"Hello".to_vec()));
    }

    #[test]
    fn test_transform_photo_from_bytes() {
        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (
                Value::Integer(16.into()),
                Value::Bytes(b"Photo data".to_vec()),
            ),
        ]);

        let claim = transform(cbor, false).unwrap();
        assert_eq!(claim.photo, Some(b"Photo data".to_vec()));
    }

    // ========== Single Biometric as Map Test ==========
    #[test]
    fn test_transform_single_biometric_as_map() {
        // Some implementations may encode a single biometric as a map directly
        // instead of an array containing a map
        let bio_map = Value::Map(vec![
            (Value::Integer(0.into()), Value::Bytes(vec![1, 2, 3, 4])),
            (Value::Integer(1.into()), Value::Integer(0.into())), // Image
        ]);

        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (Value::Integer(62.into()), bio_map), // Single map, not array
        ]);

        let claim = transform(cbor, false).unwrap();
        assert!(claim.face.is_some());
        let face = claim.face.unwrap();
        assert_eq!(face.len(), 1);
        assert_eq!(face[0].data, vec![1, 2, 3, 4]);
    }

    // ========== Biometric with Issuer Field Test ==========
    #[test]
    fn test_transform_biometric_with_issuer() {
        let bio_map = Value::Map(vec![
            (Value::Integer(0.into()), Value::Bytes(vec![1, 2, 3, 4])),
            (Value::Integer(1.into()), Value::Integer(0.into())), // Image
            (Value::Integer(2.into()), Value::Integer(1.into())), // JPEG
            (Value::Integer(3.into()), Value::Text("MOSIP".to_string())), // Issuer
        ]);

        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (Value::Integer(62.into()), Value::Array(vec![bio_map])),
        ]);

        let claim = transform(cbor, false).unwrap();
        let face = claim.face.unwrap();
        assert_eq!(face[0].issuer, Some("MOSIP".to_string()));
    }

    // ========== cbor_to_json Tests ==========
    #[test]
    fn test_cbor_to_json_bool() {
        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (Value::Integer(42.into()), Value::Bool(true)),
            (Value::Integer(43.into()), Value::Bool(false)),
        ]);

        let claim = transform(cbor, false).unwrap();
        assert_eq!(
            claim.unknown_fields.get(&42),
            Some(&serde_json::Value::Bool(true))
        );
        assert_eq!(
            claim.unknown_fields.get(&43),
            Some(&serde_json::Value::Bool(false))
        );
    }

    #[test]
    fn test_cbor_to_json_null() {
        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (Value::Integer(42.into()), Value::Null),
        ]);

        let claim = transform(cbor, false).unwrap();
        assert_eq!(
            claim.unknown_fields.get(&42),
            Some(&serde_json::Value::Null)
        );
    }

    #[test]
    fn test_cbor_to_json_float() {
        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (Value::Integer(42.into()), Value::Float(2.5)),
        ]);

        let claim = transform(cbor, false).unwrap();
        let val = claim.unknown_fields.get(&42).unwrap();
        assert!(val.is_number());
        assert!((val.as_f64().unwrap() - 2.5).abs() < 0.001);
    }

    #[test]
    fn test_cbor_to_json_bytes() {
        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (Value::Integer(42.into()), Value::Bytes(b"test".to_vec())),
        ]);

        let claim = transform(cbor, false).unwrap();
        // Bytes are base64 encoded
        let val = claim.unknown_fields.get(&42).unwrap();
        assert!(val.is_string());
        assert_eq!(val.as_str().unwrap(), "dGVzdA=="); // base64 of "test"
    }

    #[test]
    fn test_cbor_to_json_array() {
        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (
                Value::Integer(42.into()),
                Value::Array(vec![
                    Value::Integer(1.into()),
                    Value::Text("two".to_string()),
                    Value::Bool(true),
                ]),
            ),
        ]);

        let claim = transform(cbor, false).unwrap();
        let val = claim.unknown_fields.get(&42).unwrap();
        assert!(val.is_array());
        let arr = val.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], serde_json::json!(1));
        assert_eq!(arr[1], serde_json::json!("two"));
        assert_eq!(arr[2], serde_json::json!(true));
    }

    #[test]
    fn test_cbor_to_json_nested_map() {
        let inner_map = Value::Map(vec![
            (
                Value::Text("name".to_string()),
                Value::Text("inner".to_string()),
            ),
            (Value::Integer(123.into()), Value::Integer(456.into())),
        ]);

        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (Value::Integer(42.into()), inner_map),
        ]);

        let claim = transform(cbor, false).unwrap();
        let val = claim.unknown_fields.get(&42).unwrap();
        assert!(val.is_object());
        let obj = val.as_object().unwrap();
        assert_eq!(obj.get("name"), Some(&serde_json::json!("inner")));
        assert_eq!(obj.get("123"), Some(&serde_json::json!(456)));
    }

    // ========== All Demographic Fields Test ==========
    #[test]
    fn test_transform_all_demographic_fields() {
        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("id123".to_string())),
            (Value::Integer(2.into()), Value::Text("1.0".to_string())),
            (Value::Integer(3.into()), Value::Text("eng".to_string())),
            (
                Value::Integer(4.into()),
                Value::Text("Full Name".to_string()),
            ),
            (Value::Integer(5.into()), Value::Text("First".to_string())),
            (Value::Integer(6.into()), Value::Text("Middle".to_string())),
            (Value::Integer(7.into()), Value::Text("Last".to_string())),
            (
                Value::Integer(8.into()),
                Value::Text("19900101".to_string()),
            ),
            (Value::Integer(9.into()), Value::Integer(2.into())), // Female
            (
                Value::Integer(10.into()),
                Value::Text("123 Main St".to_string()),
            ),
            (
                Value::Integer(11.into()),
                Value::Text("test@example.com".to_string()),
            ),
            (
                Value::Integer(12.into()),
                Value::Text("+1234567890".to_string()),
            ),
            (Value::Integer(13.into()), Value::Text("USA".to_string())),
            (Value::Integer(14.into()), Value::Integer(2.into())), // Married
            (
                Value::Integer(15.into()),
                Value::Text("Guardian Name".to_string()),
            ),
            (Value::Integer(16.into()), Value::Bytes(vec![0xFF, 0xD8])),
            (Value::Integer(17.into()), Value::Integer(1.into())), // JPEG
            (
                Value::Integer(19.into()),
                Value::Text("Secondary Name".to_string()),
            ),
            (Value::Integer(20.into()), Value::Text("hin".to_string())),
            (Value::Integer(21.into()), Value::Text("US-NY".to_string())),
            (
                Value::Integer(22.into()),
                Value::Text("citizen".to_string()),
            ),
            (Value::Integer(23.into()), Value::Text("IN".to_string())),
        ]);

        let claim = transform(cbor, false).unwrap();

        assert_eq!(claim.id, Some("id123".to_string()));
        assert_eq!(claim.version, Some("1.0".to_string()));
        assert_eq!(claim.language, Some("eng".to_string()));
        assert_eq!(claim.full_name, Some("Full Name".to_string()));
        assert_eq!(claim.first_name, Some("First".to_string()));
        assert_eq!(claim.middle_name, Some("Middle".to_string()));
        assert_eq!(claim.last_name, Some("Last".to_string()));
        assert_eq!(claim.date_of_birth, Some("19900101".to_string()));
        assert_eq!(claim.gender, Some(Gender::Female));
        assert_eq!(claim.address, Some("123 Main St".to_string()));
        assert_eq!(claim.email, Some("test@example.com".to_string()));
        assert_eq!(claim.phone, Some("+1234567890".to_string()));
        assert_eq!(claim.nationality, Some("USA".to_string()));
        assert_eq!(claim.marital_status, Some(MaritalStatus::Married));
        assert_eq!(claim.guardian, Some("Guardian Name".to_string()));
        assert_eq!(claim.photo, Some(vec![0xFF, 0xD8]));
        assert_eq!(claim.photo_format, Some(PhotoFormat::Jpeg));
        assert_eq!(
            claim.secondary_full_name,
            Some("Secondary Name".to_string())
        );
        assert_eq!(claim.secondary_language, Some("hin".to_string()));
        assert_eq!(claim.location_code, Some("US-NY".to_string()));
        assert_eq!(claim.legal_status, Some("citizen".to_string()));
        assert_eq!(claim.country_of_issuance, Some("IN".to_string()));
    }

    // ========== All Biometric Fields Test ==========
    #[test]
    fn test_transform_all_biometric_fields() {
        let bio = |data: u8| Value::Map(vec![(Value::Integer(0.into()), Value::Bytes(vec![data]))]);

        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (Value::Integer(50.into()), Value::Array(vec![bio(50)])), // right_thumb
            (Value::Integer(51.into()), Value::Array(vec![bio(51)])), // right_pointer_finger
            (Value::Integer(52.into()), Value::Array(vec![bio(52)])), // right_middle_finger
            (Value::Integer(53.into()), Value::Array(vec![bio(53)])), // right_ring_finger
            (Value::Integer(54.into()), Value::Array(vec![bio(54)])), // right_little_finger
            (Value::Integer(55.into()), Value::Array(vec![bio(55)])), // left_thumb
            (Value::Integer(56.into()), Value::Array(vec![bio(56)])), // left_pointer_finger
            (Value::Integer(57.into()), Value::Array(vec![bio(57)])), // left_middle_finger
            (Value::Integer(58.into()), Value::Array(vec![bio(58)])), // left_ring_finger
            (Value::Integer(59.into()), Value::Array(vec![bio(59)])), // left_little_finger
            (Value::Integer(60.into()), Value::Array(vec![bio(60)])), // right_iris
            (Value::Integer(61.into()), Value::Array(vec![bio(61)])), // left_iris
            (Value::Integer(62.into()), Value::Array(vec![bio(62)])), // face
            (Value::Integer(63.into()), Value::Array(vec![bio(63)])), // right_palm
            (Value::Integer(64.into()), Value::Array(vec![bio(64)])), // left_palm
            (Value::Integer(65.into()), Value::Array(vec![bio(65)])), // voice
        ]);

        let claim = transform(cbor, false).unwrap();

        assert!(claim.right_thumb.is_some());
        assert!(claim.right_pointer_finger.is_some());
        assert!(claim.right_middle_finger.is_some());
        assert!(claim.right_ring_finger.is_some());
        assert!(claim.right_little_finger.is_some());
        assert!(claim.left_thumb.is_some());
        assert!(claim.left_pointer_finger.is_some());
        assert!(claim.left_middle_finger.is_some());
        assert!(claim.left_ring_finger.is_some());
        assert!(claim.left_little_finger.is_some());
        assert!(claim.right_iris.is_some());
        assert!(claim.left_iris.is_some());
        assert!(claim.face.is_some());
        assert!(claim.right_palm.is_some());
        assert!(claim.left_palm.is_some());
        assert!(claim.voice.is_some());

        assert_eq!(claim.biometric_count(), 16);
    }

    // ========== Skip All Biometric Fields Test ==========
    #[test]
    fn test_transform_skip_all_biometric_fields() {
        let bio = |data: u8| Value::Map(vec![(Value::Integer(0.into()), Value::Bytes(vec![data]))]);

        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (Value::Integer(50.into()), Value::Array(vec![bio(50)])),
            (Value::Integer(55.into()), Value::Array(vec![bio(55)])),
            (Value::Integer(60.into()), Value::Array(vec![bio(60)])),
            (Value::Integer(62.into()), Value::Array(vec![bio(62)])),
            (Value::Integer(65.into()), Value::Array(vec![bio(65)])),
        ]);

        let claim = transform(cbor, true).unwrap(); // skip_biometrics = true

        assert!(claim.right_thumb.is_none());
        assert!(claim.left_thumb.is_none());
        assert!(claim.right_iris.is_none());
        assert!(claim.face.is_none());
        assert!(claim.voice.is_none());
        assert_eq!(claim.biometric_count(), 0);
        assert_eq!(claim.id, Some("12345".to_string())); // Demographics preserved
    }

    // ========== Invalid Value Types Silently Ignored ==========
    #[test]
    fn test_transform_invalid_gender_returns_none() {
        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (Value::Integer(9.into()), Value::Integer(99.into())), // Invalid gender
        ]);

        let claim = transform(cbor, false).unwrap();
        assert!(claim.gender.is_none());
    }

    #[test]
    fn test_transform_invalid_marital_status_returns_none() {
        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (Value::Integer(14.into()), Value::Integer(99.into())), // Invalid marital status
        ]);

        let claim = transform(cbor, false).unwrap();
        assert!(claim.marital_status.is_none());
    }

    #[test]
    fn test_transform_invalid_photo_format_returns_none() {
        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (Value::Integer(17.into()), Value::Integer(99.into())), // Invalid photo format
        ]);

        let claim = transform(cbor, false).unwrap();
        assert!(claim.photo_format.is_none());
    }

    // ========== to_cbor Roundtrip Tests ==========
    #[test]
    fn test_to_cbor_with_all_demographics() {
        let original = Claim169 {
            id: Some("id123".to_string()),
            version: Some("1.0".to_string()),
            language: Some("eng".to_string()),
            full_name: Some("Full Name".to_string()),
            first_name: Some("First".to_string()),
            middle_name: Some("Middle".to_string()),
            last_name: Some("Last".to_string()),
            date_of_birth: Some("19900101".to_string()),
            gender: Some(Gender::Female),
            address: Some("123 Main St".to_string()),
            email: Some("test@example.com".to_string()),
            phone: Some("+1234567890".to_string()),
            nationality: Some("USA".to_string()),
            marital_status: Some(MaritalStatus::Married),
            guardian: Some("Guardian".to_string()),
            photo: Some(vec![0xFF, 0xD8]),
            photo_format: Some(PhotoFormat::Jpeg),
            best_quality_fingers: Some(vec![1, 2, 3]),
            secondary_full_name: Some("Secondary".to_string()),
            secondary_language: Some("hin".to_string()),
            location_code: Some("US-NY".to_string()),
            legal_status: Some("citizen".to_string()),
            country_of_issuance: Some("IN".to_string()),
            ..Default::default()
        };

        let cbor = to_cbor(&original);
        let parsed = transform(cbor, false).unwrap();

        assert_eq!(parsed.id, original.id);
        assert_eq!(parsed.version, original.version);
        assert_eq!(parsed.language, original.language);
        assert_eq!(parsed.full_name, original.full_name);
        assert_eq!(parsed.first_name, original.first_name);
        assert_eq!(parsed.middle_name, original.middle_name);
        assert_eq!(parsed.last_name, original.last_name);
        assert_eq!(parsed.date_of_birth, original.date_of_birth);
        assert_eq!(parsed.gender, original.gender);
        assert_eq!(parsed.address, original.address);
        assert_eq!(parsed.email, original.email);
        assert_eq!(parsed.phone, original.phone);
        assert_eq!(parsed.nationality, original.nationality);
        assert_eq!(parsed.marital_status, original.marital_status);
        assert_eq!(parsed.guardian, original.guardian);
        assert_eq!(parsed.photo, original.photo);
        assert_eq!(parsed.photo_format, original.photo_format);
        assert_eq!(parsed.best_quality_fingers, original.best_quality_fingers);
        assert_eq!(parsed.secondary_full_name, original.secondary_full_name);
        assert_eq!(parsed.secondary_language, original.secondary_language);
        assert_eq!(parsed.location_code, original.location_code);
        assert_eq!(parsed.legal_status, original.legal_status);
        assert_eq!(parsed.country_of_issuance, original.country_of_issuance);
    }

    #[test]
    fn test_to_cbor_with_biometrics() {
        use crate::model::BiometricSubFormat;
        use crate::model::ImageSubFormat;

        let original = Claim169 {
            id: Some("12345".to_string()),
            right_thumb: Some(vec![Biometric::new(vec![1, 2, 3])
                .with_format(BiometricFormat::Image)
                .with_sub_format(BiometricSubFormat::Image(ImageSubFormat::Jpeg))
                .with_issuer("Test Issuer")]),
            face: Some(vec![Biometric::new(vec![4, 5, 6])]),
            ..Default::default()
        };

        let cbor = to_cbor(&original);
        let parsed = transform(cbor, false).unwrap();

        assert_eq!(parsed.id, original.id);
        assert!(parsed.right_thumb.is_some());
        assert!(parsed.face.is_some());

        let thumb = &parsed.right_thumb.unwrap()[0];
        assert_eq!(thumb.data, vec![1, 2, 3]);
        assert_eq!(thumb.format, Some(BiometricFormat::Image));
        assert_eq!(thumb.issuer, Some("Test Issuer".to_string()));
    }

    // ========== Biometric Empty Array Test ==========
    #[test]
    fn test_transform_empty_biometric_array() {
        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (Value::Integer(62.into()), Value::Array(vec![])), // Empty array
        ]);

        let claim = transform(cbor, false).unwrap();
        assert!(claim.face.is_none()); // Empty array becomes None
    }

    // ========== Biometric Missing Data Returns None ==========
    #[test]
    fn test_transform_biometric_missing_data() {
        // Biometric map without the required data field (key 0)
        let bio_map = Value::Map(vec![
            (Value::Integer(1.into()), Value::Integer(0.into())), // Only format, no data
        ]);

        let cbor = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (Value::Integer(62.into()), Value::Array(vec![bio_map])),
        ]);

        let claim = transform(cbor, false).unwrap();
        assert!(claim.face.is_none()); // Biometric without data is skipped
    }
}
