use serde::{Deserialize, Serialize};

use super::enums::{BiometricFormat, BiometricSubFormat};

/// A single biometric data entry
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Biometric {
    /// Binary biometric data (key 0)
    #[serde(with = "serde_bytes_base64")]
    pub data: Vec<u8>,

    /// Data format: Image, Template, Sound, BioHash (key 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<BiometricFormat>,

    /// Data sub-format depending on format type (key 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_format: Option<BiometricSubFormat>,

    /// Biometric data issuer (key 3)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
}

impl Biometric {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            format: None,
            sub_format: None,
            issuer: None,
        }
    }

    pub fn with_format(mut self, format: BiometricFormat) -> Self {
        self.format = Some(format);
        self
    }

    pub fn with_sub_format(mut self, sub_format: BiometricSubFormat) -> Self {
        self.sub_format = Some(sub_format);
        self
    }

    pub fn with_issuer(mut self, issuer: impl Into<String>) -> Self {
        self.issuer = Some(issuer.into());
        self
    }
}

/// Custom serde module for base64-encoding byte arrays in JSON
mod serde_bytes_base64 {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use base64::Engine;
        let b64 = base64::engine::general_purpose::STANDARD.encode(bytes);
        serializer.serialize_str(&b64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use base64::Engine;
        let s = String::deserialize(deserializer)?;
        base64::engine::general_purpose::STANDARD
            .decode(&s)
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biometric_builder() {
        let bio = Biometric::new(vec![1, 2, 3])
            .with_format(BiometricFormat::Image)
            .with_sub_format(BiometricSubFormat::Image(
                super::super::enums::ImageSubFormat::Jpeg,
            ))
            .with_issuer("VendorA");

        assert_eq!(bio.data, vec![1, 2, 3]);
        assert_eq!(bio.format, Some(BiometricFormat::Image));
        assert_eq!(bio.issuer, Some("VendorA".to_string()));
    }

    #[test]
    fn test_biometric_json_serialization() {
        let bio = Biometric::new(vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]); // "Hello"

        let json = serde_json::to_string(&bio).unwrap();
        assert!(json.contains("SGVsbG8=")); // Base64 of "Hello"

        let deserialized: Biometric = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.data, bio.data);
    }
}
