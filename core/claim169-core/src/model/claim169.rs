use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::biometrics::Biometric;
use super::enums::{Gender, MaritalStatus, PhotoFormat};

/// The main Claim 169 identity data structure.
///
/// This represents the canonical form of MOSIP Claim 169 data, with numeric CBOR
/// keys mapped to human-readable field names. All fields are optional as the
/// specification allows partial credentials.
///
/// # Field Categories
///
/// - **Demographics** (keys 1-23): Basic identity information like name, DOB, address
/// - **Biometrics** (keys 50-65): Fingerprints, iris scans, face images, voice samples
/// - **Unknown** (other keys): Forward-compatible storage for new fields
///
/// # Example
///
/// ```rust
/// use claim169_core::model::Claim169;
///
/// // Create a minimal claim
/// let claim = Claim169::minimal("ID-12345", "Jane Doe");
/// assert_eq!(claim.id, Some("ID-12345".to_string()));
///
/// // Check for biometrics
/// if claim.has_biometrics() {
///     println!("Contains {} biometric entries", claim.biometric_count());
/// }
/// ```
///
/// # JSON Representation
///
/// When serialized to JSON, fields use camelCase naming and binary data is
/// base64-encoded:
///
/// ```json
/// {
///   "id": "12345",
///   "fullName": "Jane Doe",
///   "dateOfBirth": "19900115",
///   "gender": 2,
///   "photo": "base64-encoded-data..."
/// }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim169 {
    // ========== Demographics (keys 1-23) ==========
    /// Unique ID (key 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Version of the ID data (key 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Language code ISO 639-3 (key 3)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Full name (key 4)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,

    /// First name (key 5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// Middle name (key 6)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,

    /// Last name (key 7)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// Date of birth in YYYYMMDD format (key 8)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,

    /// Gender (key 9)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,

    /// Address with \n separators (key 10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// Email address (key 11)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Phone number in E.123 format (key 12)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Nationality ISO 3166-1/2 (key 13)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,

    /// Marital status (key 14)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<MaritalStatus>,

    /// Guardian name/id (key 15)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardian: Option<String>,

    /// Binary photo data (key 16)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "optional_bytes_base64")]
    pub photo: Option<Vec<u8>>,

    /// Photo format (key 17)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_format: Option<PhotoFormat>,

    /// Best quality fingers positions 0-10 (key 18)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_quality_fingers: Option<Vec<u8>>,

    /// Full name in secondary language (key 19)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_full_name: Option<String>,

    /// Secondary language code ISO 639-3 (key 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_language: Option<String>,

    /// Geo location/code (key 21)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<String>,

    /// Legal status of identity (key 22)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_status: Option<String>,

    /// Country of issuance (key 23)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_of_issuance: Option<String>,

    // ========== Biometrics (keys 50-65) ==========
    /// Right thumb biometrics (key 50)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_thumb: Option<Vec<Biometric>>,

    /// Right pointer finger biometrics (key 51)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_pointer_finger: Option<Vec<Biometric>>,

    /// Right middle finger biometrics (key 52)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_middle_finger: Option<Vec<Biometric>>,

    /// Right ring finger biometrics (key 53)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_ring_finger: Option<Vec<Biometric>>,

    /// Right little finger biometrics (key 54)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_little_finger: Option<Vec<Biometric>>,

    /// Left thumb biometrics (key 55)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_thumb: Option<Vec<Biometric>>,

    /// Left pointer finger biometrics (key 56)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_pointer_finger: Option<Vec<Biometric>>,

    /// Left middle finger biometrics (key 57)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_middle_finger: Option<Vec<Biometric>>,

    /// Left ring finger biometrics (key 58)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_ring_finger: Option<Vec<Biometric>>,

    /// Left little finger biometrics (key 59)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_little_finger: Option<Vec<Biometric>>,

    /// Right iris biometrics (key 60)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_iris: Option<Vec<Biometric>>,

    /// Left iris biometrics (key 61)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_iris: Option<Vec<Biometric>>,

    /// Face biometrics (key 62)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<Vec<Biometric>>,

    /// Right palm biometrics (key 63)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_palm: Option<Vec<Biometric>>,

    /// Left palm biometrics (key 64)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_palm: Option<Vec<Biometric>>,

    /// Voice biometrics (key 65)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Vec<Biometric>>,

    // ========== Unknown/future fields ==========
    /// Unknown fields (keys 24-49, 66-99, or any unrecognized)
    /// Preserved for forward compatibility
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub unknown_fields: HashMap<i64, serde_json::Value>,
}

impl Claim169 {
    /// Create a new empty Claim169 with all fields set to None.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a minimal claim with just ID and full name.
    ///
    /// Useful for testing or creating placeholder credentials.
    ///
    /// # Example
    ///
    /// ```rust
    /// use claim169_core::model::Claim169;
    ///
    /// let claim = Claim169::minimal("USER-001", "Alice Smith");
    /// assert!(claim.id.is_some());
    /// assert!(claim.full_name.is_some());
    /// assert!(claim.date_of_birth.is_none());
    /// ```
    pub fn minimal(id: impl Into<String>, full_name: impl Into<String>) -> Self {
        Self {
            id: Some(id.into()),
            full_name: Some(full_name.into()),
            ..Default::default()
        }
    }

    /// Check if this claim has any biometric data
    pub fn has_biometrics(&self) -> bool {
        self.right_thumb.is_some()
            || self.right_pointer_finger.is_some()
            || self.right_middle_finger.is_some()
            || self.right_ring_finger.is_some()
            || self.right_little_finger.is_some()
            || self.left_thumb.is_some()
            || self.left_pointer_finger.is_some()
            || self.left_middle_finger.is_some()
            || self.left_ring_finger.is_some()
            || self.left_little_finger.is_some()
            || self.right_iris.is_some()
            || self.left_iris.is_some()
            || self.face.is_some()
            || self.right_palm.is_some()
            || self.left_palm.is_some()
            || self.voice.is_some()
    }

    /// Get total count of biometric entries
    pub fn biometric_count(&self) -> usize {
        let count_opt = |opt: &Option<Vec<Biometric>>| opt.as_ref().map(|v| v.len()).unwrap_or(0);

        count_opt(&self.right_thumb)
            + count_opt(&self.right_pointer_finger)
            + count_opt(&self.right_middle_finger)
            + count_opt(&self.right_ring_finger)
            + count_opt(&self.right_little_finger)
            + count_opt(&self.left_thumb)
            + count_opt(&self.left_pointer_finger)
            + count_opt(&self.left_middle_finger)
            + count_opt(&self.left_ring_finger)
            + count_opt(&self.left_little_finger)
            + count_opt(&self.right_iris)
            + count_opt(&self.left_iris)
            + count_opt(&self.face)
            + count_opt(&self.right_palm)
            + count_opt(&self.left_palm)
            + count_opt(&self.voice)
    }
}

/// Custom serde module for optional base64-encoded byte arrays
mod optional_bytes_base64 {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(bytes: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match bytes {
            Some(b) => {
                use base64::Engine;
                let b64 = base64::engine::general_purpose::STANDARD.encode(b);
                b64.serialize(serializer)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = Option::deserialize(deserializer)?;
        match opt {
            Some(s) => {
                use base64::Engine;
                let bytes = base64::engine::general_purpose::STANDARD
                    .decode(&s)
                    .map_err(serde::de::Error::custom)?;
                Ok(Some(bytes))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal_claim() {
        let claim = Claim169::minimal("12345", "John Doe");
        assert_eq!(claim.id, Some("12345".to_string()));
        assert_eq!(claim.full_name, Some("John Doe".to_string()));
        assert!(!claim.has_biometrics());
    }

    #[test]
    fn test_biometric_detection() {
        let mut claim = Claim169::new();
        assert!(!claim.has_biometrics());
        assert_eq!(claim.biometric_count(), 0);

        claim.face = Some(vec![Biometric::new(vec![1, 2, 3])]);
        assert!(claim.has_biometrics());
        assert_eq!(claim.biometric_count(), 1);

        claim.right_thumb = Some(vec![Biometric::new(vec![1]), Biometric::new(vec![2])]);
        assert_eq!(claim.biometric_count(), 3);
    }

    #[test]
    fn test_json_serialization() {
        let claim = Claim169 {
            id: Some("123".to_string()),
            full_name: Some("Test User".to_string()),
            gender: Some(Gender::Male),
            photo: Some(vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]), // "Hello"
            ..Default::default()
        };

        let json = serde_json::to_string_pretty(&claim).unwrap();

        // Verify camelCase
        assert!(json.contains("\"fullName\""));
        // Verify photo is base64 (with possible spacing from pretty print)
        assert!(json.contains("SGVsbG8=")); // Base64 of "Hello"
                                            // Verify gender is integer
        assert!(json.contains("\"gender\": 1") || json.contains("\"gender\":1"));

        // Deserialize back
        let parsed: Claim169 = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, claim.id);
        assert_eq!(parsed.photo, claim.photo);
    }

    #[test]
    fn test_unknown_fields_in_cbor_transform() {
        // Unknown fields are preserved during CBOR->Claim169 transformation,
        // not during JSON deserialization. This test verifies the HashMap is usable.
        let mut claim = Claim169::new();
        claim.id = Some("123".to_string());
        claim
            .unknown_fields
            .insert(42, serde_json::json!("future field"));
        claim.unknown_fields.insert(99, serde_json::json!(999));

        assert!(claim.unknown_fields.contains_key(&42));
        assert!(claim.unknown_fields.contains_key(&99));
        assert_eq!(claim.unknown_fields.len(), 2);
    }
}
