use ciborium::Value;
use std::collections::HashSet;

use crate::error::{Claim169Error, Result};
use crate::model::CwtMeta;

/// Standard CWT claim keys
const CWT_ISS: i64 = 1;
const CWT_SUB: i64 = 2;
const CWT_EXP: i64 = 4;
const CWT_NBF: i64 = 5;
const CWT_IAT: i64 = 6;

/// Claim 169 key
pub const CLAIM_169_KEY: i64 = 169;

/// Maximum CBOR nesting depth to prevent stack overflow attacks
const MAX_CBOR_DEPTH: usize = 128;

/// Result of parsing CWT claims
#[derive(Debug)]
pub struct CwtParseResult {
    /// Standard CWT metadata (iss, sub, exp, nbf, iat)
    pub meta: CwtMeta,

    /// The raw Claim 169 CBOR value (map with integer keys)
    pub claim_169: Value,
}

/// Parse a CBOR timestamp claim (NumericDate) to i64.
///
/// Strict parsing: if a timestamp claim is present but invalid (wrong type or
/// outside i64 range), fail closed.
fn parse_timestamp_strict(val: &Value, field: &str) -> Result<i64> {
    match val {
        Value::Integer(i) => {
            let i128_val = i128::from(*i);
            // Only accept values within i64 range
            if i128_val >= i64::MIN as i128 && i128_val <= i64::MAX as i128 {
                Ok(i128_val as i64)
            } else {
                Err(Claim169Error::CwtParse(format!(
                    "{field} timestamp out of range: {i128_val}"
                )))
            }
        }
        _ => Err(Claim169Error::CwtParse(format!(
            "{field} timestamp must be an integer"
        ))),
    }
}

/// Check CBOR value nesting depth to prevent stack overflow DoS attacks.
fn check_cbor_depth(val: &Value, current: usize, max: usize) -> bool {
    if current > max {
        return false;
    }
    match val {
        Value::Array(arr) => arr.iter().all(|v| check_cbor_depth(v, current + 1, max)),
        Value::Map(map) => map.iter().all(|(k, v)| {
            check_cbor_depth(k, current + 1, max) && check_cbor_depth(v, current + 1, max)
        }),
        _ => true,
    }
}

/// Parse CWT payload bytes and extract standard claims and Claim 169
pub fn parse(payload: &[u8]) -> Result<CwtParseResult> {
    // Parse CBOR
    let value: Value =
        ciborium::from_reader(payload).map_err(|e| Claim169Error::CborParse(e.to_string()))?;

    // Check depth limit to prevent stack overflow attacks
    if !check_cbor_depth(&value, 0, MAX_CBOR_DEPTH) {
        return Err(Claim169Error::CborParse(
            "CBOR nesting depth exceeds maximum allowed limit".to_string(),
        ));
    }

    // CWT payload must be a map
    let map = match value {
        Value::Map(m) => m,
        _ => {
            return Err(Claim169Error::CwtParse(
                "CWT payload is not a CBOR map".to_string(),
            ))
        }
    };

    let mut meta = CwtMeta::default();
    let mut claim_169: Option<Value> = None;
    let mut seen_int_keys: HashSet<i64> = HashSet::new();

    for (key, val) in map {
        // Keys should be integers
        let key_int: i64 = match &key {
            Value::Integer(i) => {
                let i128_val = i128::from(*i);
                match i64::try_from(i128_val) {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(Claim169Error::CwtParse(format!(
                            "CWT key {} is out of valid range",
                            i128_val
                        )));
                    }
                }
            }
            _ => continue, // Skip non-integer keys
        };

        if !seen_int_keys.insert(key_int) {
            return Err(Claim169Error::CwtParse(format!(
                "duplicate CWT key: {key_int}"
            )));
        }

        match key_int {
            CWT_ISS => {
                meta.issuer = match val {
                    Value::Text(s) => Some(s),
                    _ => {
                        return Err(Claim169Error::CwtParse(
                            "iss claim must be a text string".to_string(),
                        ))
                    }
                };
            }
            CWT_SUB => {
                meta.subject = match val {
                    Value::Text(s) => Some(s),
                    _ => {
                        return Err(Claim169Error::CwtParse(
                            "sub claim must be a text string".to_string(),
                        ))
                    }
                };
            }
            CWT_EXP => {
                meta.expires_at = Some(parse_timestamp_strict(&val, "exp")?);
            }
            CWT_NBF => {
                meta.not_before = Some(parse_timestamp_strict(&val, "nbf")?);
            }
            CWT_IAT => {
                meta.issued_at = Some(parse_timestamp_strict(&val, "iat")?);
            }
            _ if key_int == CLAIM_169_KEY => {
                claim_169 = Some(val);
            }
            _ => {
                // Ignore other claims
            }
        }
    }

    let claim_169 = claim_169.ok_or(Claim169Error::Claim169NotFound)?;

    Ok(CwtParseResult { meta, claim_169 })
}

/// Encode CWT claims to CBOR bytes (used for test vector generation)
pub fn encode(meta: &CwtMeta, claim_169: &Value) -> Vec<u8> {
    let mut map: Vec<(Value, Value)> = Vec::new();

    if let Some(ref iss) = meta.issuer {
        map.push((Value::Integer(CWT_ISS.into()), Value::Text(iss.clone())));
    }
    if let Some(ref sub) = meta.subject {
        map.push((Value::Integer(CWT_SUB.into()), Value::Text(sub.clone())));
    }
    if let Some(exp) = meta.expires_at {
        map.push((Value::Integer(CWT_EXP.into()), Value::Integer(exp.into())));
    }
    if let Some(nbf) = meta.not_before {
        map.push((Value::Integer(CWT_NBF.into()), Value::Integer(nbf.into())));
    }
    if let Some(iat) = meta.issued_at {
        map.push((Value::Integer(CWT_IAT.into()), Value::Integer(iat.into())));
    }

    map.push((Value::Integer(CLAIM_169_KEY.into()), claim_169.clone()));

    let cwt = Value::Map(map);
    let mut bytes = Vec::new();
    ciborium::into_writer(&cwt, &mut bytes).expect("CBOR encoding should not fail");
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_cwt() -> Vec<u8> {
        let meta = CwtMeta::new()
            .with_issuer("https://mosip.io")
            .with_expires_at(1787912445)
            .with_issued_at(1756376445);

        // Minimal claim 169 map
        let claim_169 = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("12345".to_string())),
            (
                Value::Integer(4.into()),
                Value::Text("Test User".to_string()),
            ),
        ]);

        encode(&meta, &claim_169)
    }

    #[test]
    fn test_parse_valid_cwt() {
        let cwt_bytes = create_test_cwt();
        let result = parse(&cwt_bytes).unwrap();

        assert_eq!(result.meta.issuer, Some("https://mosip.io".to_string()));
        assert_eq!(result.meta.expires_at, Some(1787912445));
        assert_eq!(result.meta.issued_at, Some(1756376445));

        // Verify claim 169 is a map
        assert!(matches!(result.claim_169, Value::Map(_)));
    }

    #[test]
    fn test_parse_missing_claim_169() {
        // Create CWT without claim 169
        let cwt = Value::Map(vec![
            (Value::Integer(1.into()), Value::Text("issuer".to_string())),
            (Value::Integer(4.into()), Value::Integer(12345.into())),
        ]);

        let mut bytes = Vec::new();
        ciborium::into_writer(&cwt, &mut bytes).unwrap();

        let result = parse(&bytes);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Claim169Error::Claim169NotFound
        ));
    }

    #[test]
    fn test_parse_not_a_map() {
        // Array instead of map
        let cwt = Value::Array(vec![Value::Integer(1.into())]);
        let mut bytes = Vec::new();
        ciborium::into_writer(&cwt, &mut bytes).unwrap();

        let result = parse(&bytes);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Claim169Error::CwtParse(_)));
    }

    #[test]
    fn test_roundtrip() {
        let cwt_bytes = create_test_cwt();
        let result = parse(&cwt_bytes).unwrap();

        // Re-encode
        let re_encoded = encode(&result.meta, &result.claim_169);

        // Parse again
        let result2 = parse(&re_encoded).unwrap();

        assert_eq!(result.meta.issuer, result2.meta.issuer);
        assert_eq!(result.meta.expires_at, result2.meta.expires_at);
    }

    #[test]
    fn test_parse_i64_max_timestamp() {
        // i64::MAX is a valid timestamp (far future)
        let cwt = Value::Map(vec![
            (
                Value::Integer(CWT_EXP.into()),
                Value::Integer(i64::MAX.into()),
            ),
            (
                Value::Integer(CLAIM_169_KEY.into()),
                Value::Map(vec![(
                    Value::Integer(1.into()),
                    Value::Text("id".to_string()),
                )]),
            ),
        ]);

        let mut bytes = Vec::new();
        ciborium::into_writer(&cwt, &mut bytes).unwrap();

        let result = parse(&bytes).unwrap();
        assert_eq!(result.meta.expires_at, Some(i64::MAX));
    }

    #[test]
    fn test_parse_i64_min_timestamp() {
        // i64::MIN is a valid timestamp (distant past)
        let cwt = Value::Map(vec![
            (
                Value::Integer(CWT_NBF.into()),
                Value::Integer(i64::MIN.into()),
            ),
            (
                Value::Integer(CLAIM_169_KEY.into()),
                Value::Map(vec![(
                    Value::Integer(1.into()),
                    Value::Text("id".to_string()),
                )]),
            ),
        ]);

        let mut bytes = Vec::new();
        ciborium::into_writer(&cwt, &mut bytes).unwrap();

        let result = parse(&bytes).unwrap();
        assert_eq!(result.meta.not_before, Some(i64::MIN));
    }

    #[test]
    fn test_parse_timestamp_non_integer_rejected() {
        // Timestamp that is not an integer should be rejected (fail closed)
        let cwt = Value::Map(vec![
            (
                Value::Integer(CWT_EXP.into()),
                Value::Text("not-a-timestamp".to_string()), // Wrong type
            ),
            (
                Value::Integer(CLAIM_169_KEY.into()),
                Value::Map(vec![(
                    Value::Integer(1.into()),
                    Value::Text("id".to_string()),
                )]),
            ),
        ]);

        let mut bytes = Vec::new();
        ciborium::into_writer(&cwt, &mut bytes).unwrap();

        let result = parse(&bytes);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Claim169Error::CwtParse(_)));
    }

    #[test]
    fn test_rejects_duplicate_integer_keys() {
        // Duplicate keys are ambiguous; fail closed.
        let cwt = Value::Map(vec![
            (
                Value::Integer(CWT_EXP.into()),
                Value::Integer(1700000000i64.into()),
            ),
            (
                Value::Integer(CWT_EXP.into()),
                Value::Integer(2000000000i64.into()),
            ),
            (
                Value::Integer(CLAIM_169_KEY.into()),
                Value::Map(vec![(
                    Value::Integer(1.into()),
                    Value::Text("id".to_string()),
                )]),
            ),
        ]);

        let mut bytes = Vec::new();
        ciborium::into_writer(&cwt, &mut bytes).unwrap();

        let result = parse(&bytes);
        assert!(result.is_err());
        match result.unwrap_err() {
            Claim169Error::CwtParse(msg) => assert!(msg.contains("duplicate")),
            e => panic!("Expected CwtParse duplicate error, got: {:?}", e),
        }
    }

    #[test]
    fn test_parse_zero_timestamp() {
        // Timestamp of 0 (Unix epoch) is valid
        let cwt = Value::Map(vec![
            (Value::Integer(CWT_EXP.into()), Value::Integer(0.into())),
            (
                Value::Integer(CLAIM_169_KEY.into()),
                Value::Map(vec![(
                    Value::Integer(1.into()),
                    Value::Text("id".to_string()),
                )]),
            ),
        ]);

        let mut bytes = Vec::new();
        ciborium::into_writer(&cwt, &mut bytes).unwrap();

        let result = parse(&bytes).unwrap();
        assert_eq!(result.meta.expires_at, Some(0));
    }

    #[test]
    fn test_parse_negative_timestamp() {
        // Negative timestamps (before 1970) should be valid
        let cwt = Value::Map(vec![
            (
                Value::Integer(CWT_NBF.into()),
                Value::Integer((-86400i64).into()),
            ), // -1 day
            (
                Value::Integer(CLAIM_169_KEY.into()),
                Value::Map(vec![(
                    Value::Integer(1.into()),
                    Value::Text("id".to_string()),
                )]),
            ),
        ]);

        let mut bytes = Vec::new();
        ciborium::into_writer(&cwt, &mut bytes).unwrap();

        let result = parse(&bytes).unwrap();
        assert_eq!(result.meta.not_before, Some(-86400));
    }

    #[test]
    fn test_parse_rejects_deeply_nested_cbor() {
        // Create deeply nested CBOR structure that exceeds depth limit
        fn create_nested(depth: usize) -> Value {
            if depth == 0 {
                Value::Integer(1.into())
            } else {
                Value::Array(vec![create_nested(depth - 1)])
            }
        }

        let deeply_nested = Value::Map(vec![(
            Value::Integer(CLAIM_169_KEY.into()),
            create_nested(150), // Exceeds MAX_CBOR_DEPTH of 128
        )]);

        let mut bytes = Vec::new();
        ciborium::into_writer(&deeply_nested, &mut bytes).unwrap();

        let result = parse(&bytes);
        assert!(result.is_err());
        match result.unwrap_err() {
            Claim169Error::CborParse(msg) => {
                assert!(msg.contains("depth"));
            }
            _ => panic!("Expected CborParse error about depth"),
        }
    }

    #[test]
    fn test_parse_accepts_reasonable_nesting() {
        // Normal nesting depth (10 levels) should work fine
        fn create_nested_map(depth: usize) -> Value {
            if depth == 0 {
                Value::Text("leaf".to_string())
            } else {
                Value::Map(vec![(
                    Value::Integer(0.into()),
                    create_nested_map(depth - 1),
                )])
            }
        }

        let cwt = Value::Map(vec![
            (
                Value::Integer(CWT_ISS.into()),
                Value::Text("issuer".to_string()),
            ),
            (Value::Integer(CLAIM_169_KEY.into()), create_nested_map(10)),
        ]);

        let mut bytes = Vec::new();
        ciborium::into_writer(&cwt, &mut bytes).unwrap();

        let result = parse(&bytes);
        assert!(result.is_ok());
    }
}
