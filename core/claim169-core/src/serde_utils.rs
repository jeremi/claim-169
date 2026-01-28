//! Serialization utilities for base64 encoding of binary data.
//!
//! This module provides serde helpers for serializing binary data as base64 strings
//! in JSON output, while preserving raw bytes in other formats.

use serde::{Deserialize, Deserializer, Serializer};

/// Serialize bytes as base64 string
pub mod base64_bytes {
    use super::*;
    use base64::Engine;

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let b64 = base64::engine::general_purpose::STANDARD.encode(bytes);
        serializer.serialize_str(&b64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        base64::engine::general_purpose::STANDARD
            .decode(&s)
            .map_err(serde::de::Error::custom)
    }
}

/// Serialize Option<Vec<Vec<u8>>> as array of base64 strings
pub mod option_vec_base64 {
    use super::*;
    use base64::Engine;
    use serde::ser::SerializeSeq;

    pub fn serialize<S>(value: &Option<Vec<Vec<u8>>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(vecs) => {
                let mut seq = serializer.serialize_seq(Some(vecs.len()))?;
                for bytes in vecs {
                    let b64 = base64::engine::general_purpose::STANDARD.encode(bytes);
                    seq.serialize_element(&b64)?;
                }
                seq.end()
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<Vec<u8>>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<Vec<String>> = Option::deserialize(deserializer)?;
        match opt {
            Some(strings) => {
                let mut result = Vec::with_capacity(strings.len());
                for s in strings {
                    let bytes = base64::engine::general_purpose::STANDARD
                        .decode(&s)
                        .map_err(serde::de::Error::custom)?;
                    result.push(bytes);
                }
                Ok(Some(result))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestStruct {
        #[serde(with = "base64_bytes")]
        data: Vec<u8>,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestOptVec {
        #[serde(skip_serializing_if = "Option::is_none", with = "option_vec_base64")]
        certs: Option<Vec<Vec<u8>>>,
    }

    #[test]
    fn test_base64_bytes_roundtrip() {
        let original = TestStruct {
            data: vec![0x48, 0x65, 0x6c, 0x6c, 0x6f], // "Hello"
        };

        let json = serde_json::to_string(&original).unwrap();
        assert!(json.contains("SGVsbG8=")); // Base64 of "Hello"

        let deserialized: TestStruct = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_option_vec_base64_some() {
        let original = TestOptVec {
            certs: Some(vec![vec![1, 2, 3], vec![4, 5, 6]]),
        };

        let json = serde_json::to_string(&original).unwrap();
        assert!(json.contains("certs"));

        let deserialized: TestOptVec = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_option_vec_base64_none() {
        let original = TestOptVec { certs: None };

        let json = serde_json::to_string(&original).unwrap();
        assert!(!json.contains("certs")); // skip_serializing_if

        // Deserialize explicitly null value
        let deserialized: TestOptVec = serde_json::from_str(r#"{"certs":null}"#).unwrap();
        assert_eq!(deserialized.certs, None);
    }
}
