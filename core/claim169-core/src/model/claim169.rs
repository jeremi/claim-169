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
/// # Examples
///
/// Using the builder pattern (recommended):
///
/// ```rust
/// use claim169_core::model::{Claim169, Gender, MaritalStatus};
///
/// let claim = Claim169::new()
///     .with_id("ID-12345-ABCDE")
///     .with_full_name("Jane Marie Smith")
///     .with_first_name("Jane")
///     .with_last_name("Smith")
///     .with_date_of_birth("19900515")
///     .with_gender(Gender::Female)
///     .with_email("jane.smith@example.com")
///     .with_marital_status(MaritalStatus::Married);
/// ```
///
/// Using the minimal constructor:
///
/// ```rust
/// use claim169_core::model::Claim169;
///
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

    // ========== Builder Methods ==========

    /// Set the unique ID (key 1).
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the version (key 2).
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set the language code ISO 639-3 (key 3).
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Set the full name (key 4).
    pub fn with_full_name(mut self, full_name: impl Into<String>) -> Self {
        self.full_name = Some(full_name.into());
        self
    }

    /// Set the first name (key 5).
    pub fn with_first_name(mut self, first_name: impl Into<String>) -> Self {
        self.first_name = Some(first_name.into());
        self
    }

    /// Set the middle name (key 6).
    pub fn with_middle_name(mut self, middle_name: impl Into<String>) -> Self {
        self.middle_name = Some(middle_name.into());
        self
    }

    /// Set the last name (key 7).
    pub fn with_last_name(mut self, last_name: impl Into<String>) -> Self {
        self.last_name = Some(last_name.into());
        self
    }

    /// Set the date of birth in YYYYMMDD format (key 8).
    pub fn with_date_of_birth(mut self, date_of_birth: impl Into<String>) -> Self {
        self.date_of_birth = Some(date_of_birth.into());
        self
    }

    /// Set the gender (key 9).
    pub fn with_gender(mut self, gender: Gender) -> Self {
        self.gender = Some(gender);
        self
    }

    /// Set the address with \n separators (key 10).
    pub fn with_address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    /// Set the email address (key 11).
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Set the phone number in E.123 format (key 12).
    pub fn with_phone(mut self, phone: impl Into<String>) -> Self {
        self.phone = Some(phone.into());
        self
    }

    /// Set the nationality ISO 3166-1/2 (key 13).
    pub fn with_nationality(mut self, nationality: impl Into<String>) -> Self {
        self.nationality = Some(nationality.into());
        self
    }

    /// Set the marital status (key 14).
    pub fn with_marital_status(mut self, marital_status: MaritalStatus) -> Self {
        self.marital_status = Some(marital_status);
        self
    }

    /// Set the guardian name/id (key 15).
    pub fn with_guardian(mut self, guardian: impl Into<String>) -> Self {
        self.guardian = Some(guardian.into());
        self
    }

    /// Set the binary photo data (key 16).
    pub fn with_photo(mut self, photo: impl Into<Vec<u8>>) -> Self {
        self.photo = Some(photo.into());
        self
    }

    /// Set the photo format (key 17).
    pub fn with_photo_format(mut self, photo_format: PhotoFormat) -> Self {
        self.photo_format = Some(photo_format);
        self
    }

    /// Set the best quality fingers positions 0-10 (key 18).
    pub fn with_best_quality_fingers(mut self, fingers: impl Into<Vec<u8>>) -> Self {
        self.best_quality_fingers = Some(fingers.into());
        self
    }

    /// Set the full name in secondary language (key 19).
    pub fn with_secondary_full_name(mut self, name: impl Into<String>) -> Self {
        self.secondary_full_name = Some(name.into());
        self
    }

    /// Set the secondary language code ISO 639-3 (key 20).
    pub fn with_secondary_language(mut self, language: impl Into<String>) -> Self {
        self.secondary_language = Some(language.into());
        self
    }

    /// Set the geo location/code (key 21).
    pub fn with_location_code(mut self, location_code: impl Into<String>) -> Self {
        self.location_code = Some(location_code.into());
        self
    }

    /// Set the legal status of identity (key 22).
    pub fn with_legal_status(mut self, legal_status: impl Into<String>) -> Self {
        self.legal_status = Some(legal_status.into());
        self
    }

    /// Set the country of issuance (key 23).
    pub fn with_country_of_issuance(mut self, country: impl Into<String>) -> Self {
        self.country_of_issuance = Some(country.into());
        self
    }

    // ========== Biometric Builder Methods ==========

    /// Set right thumb biometrics (key 50).
    pub fn with_right_thumb(mut self, biometrics: Vec<Biometric>) -> Self {
        self.right_thumb = Some(biometrics);
        self
    }

    /// Set right pointer finger biometrics (key 51).
    pub fn with_right_pointer_finger(mut self, biometrics: Vec<Biometric>) -> Self {
        self.right_pointer_finger = Some(biometrics);
        self
    }

    /// Set right middle finger biometrics (key 52).
    pub fn with_right_middle_finger(mut self, biometrics: Vec<Biometric>) -> Self {
        self.right_middle_finger = Some(biometrics);
        self
    }

    /// Set right ring finger biometrics (key 53).
    pub fn with_right_ring_finger(mut self, biometrics: Vec<Biometric>) -> Self {
        self.right_ring_finger = Some(biometrics);
        self
    }

    /// Set right little finger biometrics (key 54).
    pub fn with_right_little_finger(mut self, biometrics: Vec<Biometric>) -> Self {
        self.right_little_finger = Some(biometrics);
        self
    }

    /// Set left thumb biometrics (key 55).
    pub fn with_left_thumb(mut self, biometrics: Vec<Biometric>) -> Self {
        self.left_thumb = Some(biometrics);
        self
    }

    /// Set left pointer finger biometrics (key 56).
    pub fn with_left_pointer_finger(mut self, biometrics: Vec<Biometric>) -> Self {
        self.left_pointer_finger = Some(biometrics);
        self
    }

    /// Set left middle finger biometrics (key 57).
    pub fn with_left_middle_finger(mut self, biometrics: Vec<Biometric>) -> Self {
        self.left_middle_finger = Some(biometrics);
        self
    }

    /// Set left ring finger biometrics (key 58).
    pub fn with_left_ring_finger(mut self, biometrics: Vec<Biometric>) -> Self {
        self.left_ring_finger = Some(biometrics);
        self
    }

    /// Set left little finger biometrics (key 59).
    pub fn with_left_little_finger(mut self, biometrics: Vec<Biometric>) -> Self {
        self.left_little_finger = Some(biometrics);
        self
    }

    /// Set right iris biometrics (key 60).
    pub fn with_right_iris(mut self, biometrics: Vec<Biometric>) -> Self {
        self.right_iris = Some(biometrics);
        self
    }

    /// Set left iris biometrics (key 61).
    pub fn with_left_iris(mut self, biometrics: Vec<Biometric>) -> Self {
        self.left_iris = Some(biometrics);
        self
    }

    /// Set face biometrics (key 62).
    pub fn with_face(mut self, biometrics: Vec<Biometric>) -> Self {
        self.face = Some(biometrics);
        self
    }

    /// Set right palm biometrics (key 63).
    pub fn with_right_palm(mut self, biometrics: Vec<Biometric>) -> Self {
        self.right_palm = Some(biometrics);
        self
    }

    /// Set left palm biometrics (key 64).
    pub fn with_left_palm(mut self, biometrics: Vec<Biometric>) -> Self {
        self.left_palm = Some(biometrics);
        self
    }

    /// Set voice biometrics (key 65).
    pub fn with_voice(mut self, biometrics: Vec<Biometric>) -> Self {
        self.voice = Some(biometrics);
        self
    }

    /// Create a clone of this claim with all biometric fields removed.
    ///
    /// Useful for encoding smaller QR codes that exclude biometric data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use claim169_core::model::{Claim169, Biometric};
    ///
    /// let mut claim = Claim169::minimal("USER-001", "Alice Smith");
    /// claim.face = Some(vec![Biometric::new(vec![1, 2, 3])]);
    /// assert!(claim.has_biometrics());
    ///
    /// let without_bio = claim.without_biometrics();
    /// assert!(!without_bio.has_biometrics());
    /// assert_eq!(without_bio.id, claim.id);
    /// ```
    pub fn without_biometrics(&self) -> Self {
        Self {
            id: self.id.clone(),
            version: self.version.clone(),
            language: self.language.clone(),
            full_name: self.full_name.clone(),
            first_name: self.first_name.clone(),
            middle_name: self.middle_name.clone(),
            last_name: self.last_name.clone(),
            date_of_birth: self.date_of_birth.clone(),
            gender: self.gender,
            address: self.address.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            nationality: self.nationality.clone(),
            marital_status: self.marital_status,
            guardian: self.guardian.clone(),
            photo: self.photo.clone(),
            photo_format: self.photo_format,
            best_quality_fingers: self.best_quality_fingers.clone(),
            secondary_full_name: self.secondary_full_name.clone(),
            secondary_language: self.secondary_language.clone(),
            location_code: self.location_code.clone(),
            legal_status: self.legal_status.clone(),
            country_of_issuance: self.country_of_issuance.clone(),
            unknown_fields: self.unknown_fields.clone(),
            // All biometric fields are set to None
            right_thumb: None,
            right_pointer_finger: None,
            right_middle_finger: None,
            right_ring_finger: None,
            right_little_finger: None,
            left_thumb: None,
            left_pointer_finger: None,
            left_middle_finger: None,
            left_ring_finger: None,
            left_little_finger: None,
            right_iris: None,
            left_iris: None,
            face: None,
            right_palm: None,
            left_palm: None,
            voice: None,
        }
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
    fn test_builder_pattern() {
        let claim = Claim169::new()
            .with_id("ID-12345-ABCDE")
            .with_full_name("Jane Marie Smith")
            .with_first_name("Jane")
            .with_middle_name("Marie")
            .with_last_name("Smith")
            .with_date_of_birth("19900515")
            .with_gender(Gender::Female)
            .with_email("jane.smith@example.com")
            .with_phone("+1 555 123 4567")
            .with_address("123 Main St\nNew York, NY 10001")
            .with_nationality("USA")
            .with_marital_status(MaritalStatus::Married);

        assert_eq!(claim.id, Some("ID-12345-ABCDE".to_string()));
        assert_eq!(claim.full_name, Some("Jane Marie Smith".to_string()));
        assert_eq!(claim.first_name, Some("Jane".to_string()));
        assert_eq!(claim.middle_name, Some("Marie".to_string()));
        assert_eq!(claim.last_name, Some("Smith".to_string()));
        assert_eq!(claim.date_of_birth, Some("19900515".to_string()));
        assert_eq!(claim.gender, Some(Gender::Female));
        assert_eq!(claim.email, Some("jane.smith@example.com".to_string()));
        assert_eq!(claim.phone, Some("+1 555 123 4567".to_string()));
        assert_eq!(
            claim.address,
            Some("123 Main St\nNew York, NY 10001".to_string())
        );
        assert_eq!(claim.nationality, Some("USA".to_string()));
        assert_eq!(claim.marital_status, Some(MaritalStatus::Married));
    }

    #[test]
    fn test_builder_with_owned_strings() {
        // Verify builder works with both &str and String
        let name = String::from("Owned Name");
        let claim = Claim169::new().with_id("literal-id").with_full_name(name);

        assert_eq!(claim.id, Some("literal-id".to_string()));
        assert_eq!(claim.full_name, Some("Owned Name".to_string()));
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

    // ========== Additional Builder Tests ==========
    #[test]
    fn test_builder_version_and_language() {
        let claim = Claim169::new().with_version("1.0").with_language("eng");

        assert_eq!(claim.version, Some("1.0".to_string()));
        assert_eq!(claim.language, Some("eng".to_string()));
    }

    #[test]
    fn test_builder_guardian() {
        let claim = Claim169::new().with_guardian("John Smith Sr.");

        assert_eq!(claim.guardian, Some("John Smith Sr.".to_string()));
    }

    #[test]
    fn test_builder_photo_and_format() {
        let photo_data = vec![0xFF, 0xD8, 0xFF, 0xE0];
        let claim = Claim169::new()
            .with_photo(photo_data.clone())
            .with_photo_format(PhotoFormat::Jpeg);

        assert_eq!(claim.photo, Some(photo_data));
        assert_eq!(claim.photo_format, Some(PhotoFormat::Jpeg));
    }

    #[test]
    fn test_builder_best_quality_fingers() {
        let fingers = vec![1, 6, 2, 7];
        let claim = Claim169::new().with_best_quality_fingers(fingers.clone());

        assert_eq!(claim.best_quality_fingers, Some(fingers));
    }

    #[test]
    fn test_builder_secondary_name_and_language() {
        let claim = Claim169::new()
            .with_secondary_full_name("मरियम जोसेफ")
            .with_secondary_language("hin");

        assert_eq!(claim.secondary_full_name, Some("मरियम जोसेफ".to_string()));
        assert_eq!(claim.secondary_language, Some("hin".to_string()));
    }

    #[test]
    fn test_builder_location_and_legal_status() {
        let claim = Claim169::new()
            .with_location_code("US-NY-NYC")
            .with_legal_status("citizen");

        assert_eq!(claim.location_code, Some("US-NY-NYC".to_string()));
        assert_eq!(claim.legal_status, Some("citizen".to_string()));
    }

    #[test]
    fn test_builder_country_of_issuance() {
        let claim = Claim169::new().with_country_of_issuance("IN");

        assert_eq!(claim.country_of_issuance, Some("IN".to_string()));
    }

    // ========== Biometric Builder Tests ==========
    #[test]
    fn test_builder_right_hand_fingers() {
        let bio = vec![Biometric::new(vec![1, 2, 3])];
        let claim = Claim169::new()
            .with_right_thumb(bio.clone())
            .with_right_pointer_finger(bio.clone())
            .with_right_middle_finger(bio.clone())
            .with_right_ring_finger(bio.clone())
            .with_right_little_finger(bio.clone());

        assert!(claim.right_thumb.is_some());
        assert!(claim.right_pointer_finger.is_some());
        assert!(claim.right_middle_finger.is_some());
        assert!(claim.right_ring_finger.is_some());
        assert!(claim.right_little_finger.is_some());
        assert_eq!(claim.biometric_count(), 5);
    }

    #[test]
    fn test_builder_left_hand_fingers() {
        let bio = vec![Biometric::new(vec![1, 2, 3])];
        let claim = Claim169::new()
            .with_left_thumb(bio.clone())
            .with_left_pointer_finger(bio.clone())
            .with_left_middle_finger(bio.clone())
            .with_left_ring_finger(bio.clone())
            .with_left_little_finger(bio.clone());

        assert!(claim.left_thumb.is_some());
        assert!(claim.left_pointer_finger.is_some());
        assert!(claim.left_middle_finger.is_some());
        assert!(claim.left_ring_finger.is_some());
        assert!(claim.left_little_finger.is_some());
        assert_eq!(claim.biometric_count(), 5);
    }

    #[test]
    fn test_builder_iris() {
        let bio = vec![Biometric::new(vec![1, 2, 3])];
        let claim = Claim169::new()
            .with_right_iris(bio.clone())
            .with_left_iris(bio.clone());

        assert!(claim.right_iris.is_some());
        assert!(claim.left_iris.is_some());
        assert_eq!(claim.biometric_count(), 2);
    }

    #[test]
    fn test_builder_face() {
        let bio = vec![Biometric::new(vec![1, 2, 3])];
        let claim = Claim169::new().with_face(bio.clone());

        assert!(claim.face.is_some());
        assert_eq!(claim.biometric_count(), 1);
    }

    #[test]
    fn test_builder_palms() {
        let bio = vec![Biometric::new(vec![1, 2, 3])];
        let claim = Claim169::new()
            .with_right_palm(bio.clone())
            .with_left_palm(bio.clone());

        assert!(claim.right_palm.is_some());
        assert!(claim.left_palm.is_some());
        assert_eq!(claim.biometric_count(), 2);
    }

    #[test]
    fn test_builder_voice() {
        let bio = vec![Biometric::new(vec![1, 2, 3])];
        let claim = Claim169::new().with_voice(bio.clone());

        assert!(claim.voice.is_some());
        assert_eq!(claim.biometric_count(), 1);
    }

    // ========== without_biometrics Tests ==========
    #[test]
    fn test_without_biometrics() {
        let original = Claim169::new()
            .with_id("12345")
            .with_full_name("Test User")
            .with_face(vec![Biometric::new(vec![1, 2, 3])])
            .with_right_thumb(vec![Biometric::new(vec![4, 5, 6])])
            .with_left_iris(vec![Biometric::new(vec![7, 8, 9])]);

        assert!(original.has_biometrics());
        assert_eq!(original.biometric_count(), 3);

        let without_bio = original.without_biometrics();

        // Demographics preserved
        assert_eq!(without_bio.id, original.id);
        assert_eq!(without_bio.full_name, original.full_name);

        // Biometrics removed
        assert!(!without_bio.has_biometrics());
        assert_eq!(without_bio.biometric_count(), 0);
        assert!(without_bio.face.is_none());
        assert!(without_bio.right_thumb.is_none());
        assert!(without_bio.left_iris.is_none());
    }

    #[test]
    fn test_without_biometrics_preserves_all_demographics() {
        let original = Claim169::new()
            .with_id("12345")
            .with_version("1.0")
            .with_language("eng")
            .with_full_name("Test User")
            .with_first_name("Test")
            .with_middle_name("Middle")
            .with_last_name("User")
            .with_date_of_birth("19900101")
            .with_gender(Gender::Male)
            .with_address("123 Main St")
            .with_email("test@example.com")
            .with_phone("+1 555 1234")
            .with_nationality("USA")
            .with_marital_status(MaritalStatus::Married)
            .with_guardian("Parent Name")
            .with_photo(vec![0xFF, 0xD8])
            .with_photo_format(PhotoFormat::Jpeg)
            .with_best_quality_fingers(vec![1, 2, 3])
            .with_secondary_full_name("Secondary Name")
            .with_secondary_language("hin")
            .with_location_code("US-NY")
            .with_legal_status("citizen")
            .with_country_of_issuance("USA")
            .with_face(vec![Biometric::new(vec![1, 2, 3])]);

        let without_bio = original.without_biometrics();

        assert_eq!(without_bio.id, original.id);
        assert_eq!(without_bio.version, original.version);
        assert_eq!(without_bio.language, original.language);
        assert_eq!(without_bio.full_name, original.full_name);
        assert_eq!(without_bio.first_name, original.first_name);
        assert_eq!(without_bio.middle_name, original.middle_name);
        assert_eq!(without_bio.last_name, original.last_name);
        assert_eq!(without_bio.date_of_birth, original.date_of_birth);
        assert_eq!(without_bio.gender, original.gender);
        assert_eq!(without_bio.address, original.address);
        assert_eq!(without_bio.email, original.email);
        assert_eq!(without_bio.phone, original.phone);
        assert_eq!(without_bio.nationality, original.nationality);
        assert_eq!(without_bio.marital_status, original.marital_status);
        assert_eq!(without_bio.guardian, original.guardian);
        assert_eq!(without_bio.photo, original.photo);
        assert_eq!(without_bio.photo_format, original.photo_format);
        assert_eq!(
            without_bio.best_quality_fingers,
            original.best_quality_fingers
        );
        assert_eq!(
            without_bio.secondary_full_name,
            original.secondary_full_name
        );
        assert_eq!(without_bio.secondary_language, original.secondary_language);
        assert_eq!(without_bio.location_code, original.location_code);
        assert_eq!(without_bio.legal_status, original.legal_status);
        assert_eq!(
            without_bio.country_of_issuance,
            original.country_of_issuance
        );
    }

    #[test]
    fn test_without_biometrics_preserves_unknown_fields() {
        let mut original = Claim169::new();
        original
            .unknown_fields
            .insert(42, serde_json::json!("test"));

        let without_bio = original.without_biometrics();

        assert_eq!(without_bio.unknown_fields.len(), 1);
        assert!(without_bio.unknown_fields.contains_key(&42));
    }

    // ========== Biometric Detection Edge Cases ==========
    #[test]
    fn test_has_biometrics_all_types() {
        // Test each biometric type individually triggers has_biometrics
        let bio = vec![Biometric::new(vec![1])];

        let test_cases = vec![
            Claim169::new().with_right_thumb(bio.clone()),
            Claim169::new().with_right_pointer_finger(bio.clone()),
            Claim169::new().with_right_middle_finger(bio.clone()),
            Claim169::new().with_right_ring_finger(bio.clone()),
            Claim169::new().with_right_little_finger(bio.clone()),
            Claim169::new().with_left_thumb(bio.clone()),
            Claim169::new().with_left_pointer_finger(bio.clone()),
            Claim169::new().with_left_middle_finger(bio.clone()),
            Claim169::new().with_left_ring_finger(bio.clone()),
            Claim169::new().with_left_little_finger(bio.clone()),
            Claim169::new().with_right_iris(bio.clone()),
            Claim169::new().with_left_iris(bio.clone()),
            Claim169::new().with_face(bio.clone()),
            Claim169::new().with_right_palm(bio.clone()),
            Claim169::new().with_left_palm(bio.clone()),
            Claim169::new().with_voice(bio.clone()),
        ];

        for (i, claim) in test_cases.iter().enumerate() {
            assert!(
                claim.has_biometrics(),
                "Test case {} should have biometrics",
                i
            );
            assert_eq!(
                claim.biometric_count(),
                1,
                "Test case {} should have 1 biometric",
                i
            );
        }
    }

    // ========== JSON Deserialization Tests ==========
    #[test]
    fn test_json_deserialization_with_base64_photo() {
        let json = r#"{
            "id": "12345",
            "fullName": "Test User",
            "photo": "SGVsbG8gV29ybGQ="
        }"#;

        let claim: Claim169 = serde_json::from_str(json).unwrap();
        assert_eq!(claim.id, Some("12345".to_string()));
        assert_eq!(claim.full_name, Some("Test User".to_string()));
        assert_eq!(claim.photo, Some(b"Hello World".to_vec()));
    }

    #[test]
    fn test_json_deserialization_with_null_photo() {
        // The photo field with custom deserializer handles null values
        let json = r#"{
            "id": "12345",
            "fullName": "Test User",
            "photo": null
        }"#;

        let claim: Claim169 = serde_json::from_str(json).unwrap();
        assert_eq!(claim.id, Some("12345".to_string()));
        assert!(claim.photo.is_none());
    }

    #[test]
    fn test_json_serialization_without_photo() {
        let claim = Claim169::new().with_id("12345").with_full_name("Test User");

        let json = serde_json::to_string(&claim).unwrap();
        assert!(!json.contains("photo"));
    }

    #[test]
    fn test_json_roundtrip_complete() {
        let original = Claim169::new()
            .with_id("12345")
            .with_full_name("Test User")
            .with_gender(Gender::Female)
            .with_marital_status(MaritalStatus::Unmarried)
            .with_photo(vec![1, 2, 3, 4, 5])
            .with_photo_format(PhotoFormat::Jpeg2000);

        let json = serde_json::to_string(&original).unwrap();
        let parsed: Claim169 = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.id, original.id);
        assert_eq!(parsed.full_name, original.full_name);
        assert_eq!(parsed.gender, original.gender);
        assert_eq!(parsed.marital_status, original.marital_status);
        assert_eq!(parsed.photo, original.photo);
        assert_eq!(parsed.photo_format, original.photo_format);
    }

    // ========== Default and New Tests ==========
    #[test]
    fn test_new_equals_default() {
        let new = Claim169::new();
        let default = Claim169::default();

        assert_eq!(new, default);
    }

    #[test]
    fn test_clone_and_partialeq() {
        let original = Claim169::new().with_id("12345").with_full_name("Test User");

        let cloned = original.clone();

        assert_eq!(original, cloned);
    }

    #[test]
    fn test_debug() {
        let claim = Claim169::new().with_id("12345");
        let debug_str = format!("{:?}", claim);
        assert!(debug_str.contains("12345"));
        assert!(debug_str.contains("Claim169"));
    }
}
