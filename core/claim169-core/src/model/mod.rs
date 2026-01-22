//! Data models for Claim 169 identity credentials.
//!
//! This module contains the data structures that represent decoded Claim 169 data:
//!
//! - [`Claim169`]: The main identity data structure with demographic and biometric fields
//! - [`CwtMeta`]: Standard CWT claims (issuer, expiration, etc.)
//! - [`Biometric`]: Biometric data entries (fingerprints, iris, face, voice)
//!
//! # Field Mapping
//!
//! Claim 169 uses numeric CBOR keys for compactness. This library maps them to
//! human-readable field names:
//!
//! | Key | Field | Description |
//! |-----|-------|-------------|
//! | 1 | `id` | Unique identifier |
//! | 4 | `full_name` | Full name |
//! | 8 | `date_of_birth` | Date of birth (YYYYMMDD) |
//! | 9 | `gender` | Gender enum |
//! | 10 | `address` | Address (newline-separated) |
//! | 50-65 | Biometrics | Fingerprints, iris, face, voice |
//!
//! See the [Claim 169 specification](https://github.com/mosip/id-claim-169/tree/main) for the complete field mapping.
//!
//! # JSON Serialization
//!
//! All types implement `Serialize` and `Deserialize` with camelCase field names:
//!
//! ```rust,ignore
//! let claim = Claim169::minimal("123", "John Doe");
//! let json = serde_json::to_string(&claim)?;
//! // {"id":"123","fullName":"John Doe"}
//! ```
//!
//! Binary data (photos, biometrics) is base64-encoded in JSON output.

mod biometrics;
mod claim169;
mod cwt_meta;
mod enums;

pub use biometrics::Biometric;
pub use claim169::Claim169;
pub use cwt_meta::CwtMeta;
pub use enums::{
    BiometricFormat, BiometricSubFormat, Gender, ImageSubFormat, MaritalStatus, PhotoFormat,
    SoundSubFormat, TemplateSubFormat, VerificationStatus,
};
