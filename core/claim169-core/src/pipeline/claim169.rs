use std::collections::HashMap;

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
}
