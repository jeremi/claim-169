//! UniFFI bindings for MOSIP Claim 169 (Kotlin/Java SDK).
//!
//! This crate exposes the `claim169-core` library through UniFFI,
//! generating idiomatic Kotlin bindings for JVM and Android platforms.

use std::sync::Mutex;

use claim169_core as claim169;
use coset::iana::{self, EnumI64};

uniffi::setup_scaffolding!();

// ============================================================
// Error types — flat errors for UniFFI compatibility
// ============================================================

/// High-level errors from the Claim 169 decoding/encoding pipeline.
///
/// In Kotlin, this maps to a sealed exception hierarchy where each variant
/// is a subclass of `Claim169Exception`. The `message` field contains the
/// error details, available via `e.message` in Kotlin.
#[derive(Debug, thiserror::Error, uniffi::Error)]
#[uniffi(flat_error)]
pub enum Claim169Exception {
    #[error("invalid Base45 encoding: {0}")]
    Base45Decode(String),

    #[error("decompression failed: {0}")]
    Decompress(String),

    #[error("decompression limit exceeded: {0}")]
    DecompressLimitExceeded(String),

    #[error("invalid COSE structure: {0}")]
    CoseParse(String),

    #[error("unsupported COSE type: {0}")]
    UnsupportedCoseType(String),

    #[error("signature verification failed: {0}")]
    SignatureInvalid(String),

    #[error("decryption failed: {0}")]
    DecryptionFailed(String),

    #[error("invalid CBOR: {0}")]
    CborParse(String),

    #[error("CWT parsing failed: {0}")]
    CwtParse(String),

    #[error("claim 169 not found in CWT payload")]
    Claim169NotFound(String),

    #[error("invalid claim 169 structure: {0}")]
    Claim169Invalid(String),

    #[error("unsupported algorithm: {0}")]
    UnsupportedAlgorithm(String),

    #[error("key not found: {0}")]
    KeyNotFound(String),

    #[error("credential expired: {0}")]
    Expired(String),

    #[error("credential not yet valid: {0}")]
    NotYetValid(String),

    #[error("crypto error: {0}")]
    Crypto(String),

    #[error("I/O error: {0}")]
    Io(String),

    #[error("CBOR encoding failed: {0}")]
    CborEncode(String),

    #[error("signing failed: {0}")]
    SignatureFailed(String),

    #[error("encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("encoding configuration error: {0}")]
    EncodingConfig(String),

    #[error("decoding configuration error: {0}")]
    DecodingConfig(String),
}

impl From<claim169::Claim169Error> for Claim169Exception {
    fn from(err: claim169::Claim169Error) -> Self {
        match err {
            claim169::Claim169Error::Base45Decode(msg) => Claim169Exception::Base45Decode(msg),
            claim169::Claim169Error::Decompress(msg) => Claim169Exception::Decompress(msg),
            claim169::Claim169Error::DecompressLimitExceeded { max_bytes } => {
                Claim169Exception::DecompressLimitExceeded(format!("max {} bytes", max_bytes))
            }
            claim169::Claim169Error::CoseParse(msg) => Claim169Exception::CoseParse(msg),
            claim169::Claim169Error::UnsupportedCoseType(msg) => {
                Claim169Exception::UnsupportedCoseType(msg)
            }
            claim169::Claim169Error::SignatureInvalid(msg) => {
                Claim169Exception::SignatureInvalid(msg)
            }
            claim169::Claim169Error::DecryptionFailed(msg) => {
                Claim169Exception::DecryptionFailed(msg)
            }
            claim169::Claim169Error::CborParse(msg) => Claim169Exception::CborParse(msg),
            claim169::Claim169Error::CwtParse(msg) => Claim169Exception::CwtParse(msg),
            claim169::Claim169Error::Claim169NotFound => {
                Claim169Exception::Claim169NotFound("claim 169 not found".to_string())
            }
            claim169::Claim169Error::Claim169Invalid(msg) => {
                Claim169Exception::Claim169Invalid(msg)
            }
            claim169::Claim169Error::UnsupportedAlgorithm(msg) => {
                Claim169Exception::UnsupportedAlgorithm(msg)
            }
            claim169::Claim169Error::KeyNotFound(key_id) => {
                Claim169Exception::KeyNotFound(format!("{:?}", key_id))
            }
            claim169::Claim169Error::Expired(ts) => {
                Claim169Exception::Expired(format!("expired at timestamp {}", ts))
            }
            claim169::Claim169Error::NotYetValid(ts) => {
                Claim169Exception::NotYetValid(format!("not valid until timestamp {}", ts))
            }
            claim169::Claim169Error::Crypto(msg) => Claim169Exception::Crypto(msg),
            claim169::Claim169Error::Io(err) => Claim169Exception::Io(err.to_string()),
            claim169::Claim169Error::CborEncode(msg) => Claim169Exception::CborEncode(msg),
            claim169::Claim169Error::SignatureFailed(msg) => {
                Claim169Exception::SignatureFailed(msg)
            }
            claim169::Claim169Error::EncryptionFailed(msg) => {
                Claim169Exception::EncryptionFailed(msg)
            }
            claim169::Claim169Error::EncodingConfig(msg) => Claim169Exception::EncodingConfig(msg),
            claim169::Claim169Error::DecodingConfig(msg) => Claim169Exception::DecodingConfig(msg),
        }
    }
}

/// Low-level crypto error for callback interfaces.
#[derive(Debug, thiserror::Error, uniffi::Error)]
#[uniffi(flat_error)]
pub enum CryptoException {
    #[error("invalid key format: {0}")]
    InvalidKeyFormat(String),

    #[error("key not found")]
    KeyNotFound(String),

    #[error("signature verification failed")]
    VerificationFailed(String),

    #[error("decryption failed: {0}")]
    DecryptionFailed(String),

    #[error("signing failed: {0}")]
    SigningFailed(String),

    #[error("encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("unsupported algorithm: {0}")]
    UnsupportedAlgorithm(String),

    #[error("{0}")]
    Other(String),
}

impl From<CryptoException> for claim169::CryptoError {
    fn from(err: CryptoException) -> Self {
        match err {
            CryptoException::InvalidKeyFormat(msg) => claim169::CryptoError::InvalidKeyFormat(msg),
            CryptoException::KeyNotFound(_) => claim169::CryptoError::KeyNotFound,
            CryptoException::VerificationFailed(_) => claim169::CryptoError::VerificationFailed,
            CryptoException::DecryptionFailed(msg) => claim169::CryptoError::DecryptionFailed(msg),
            CryptoException::SigningFailed(msg) => claim169::CryptoError::SigningFailed(msg),
            CryptoException::EncryptionFailed(msg) => claim169::CryptoError::EncryptionFailed(msg),
            CryptoException::UnsupportedAlgorithm(msg) => {
                claim169::CryptoError::UnsupportedAlgorithm(msg)
            }
            CryptoException::Other(msg) => claim169::CryptoError::Other(msg),
        }
    }
}

impl From<uniffi::UnexpectedUniFFICallbackError> for CryptoException {
    fn from(err: uniffi::UnexpectedUniFFICallbackError) -> Self {
        CryptoException::Other(err.reason)
    }
}

// ============================================================
// Data records (UniFFI value types)
// ============================================================

/// Biometric data entry.
#[derive(Debug, Clone, uniffi::Record)]
pub struct BiometricData {
    /// Raw biometric data bytes.
    pub data: Vec<u8>,
    /// Biometric format (0=Image, 1=Template, 2=Sound, 3=BioHash). None if not specified.
    pub format: Option<i64>,
    /// Biometric sub-format. None if not specified.
    pub sub_format: Option<i64>,
    /// Issuer of the biometric data.
    pub issuer: Option<String>,
}

impl From<&claim169::Biometric> for BiometricData {
    fn from(b: &claim169::Biometric) -> Self {
        Self {
            data: b.data.clone(),
            format: b.format.map(|f| f.into()),
            sub_format: b.sub_format.map(|sf| sf.to_value()),
            issuer: b.issuer.clone(),
        }
    }
}

impl From<&BiometricData> for claim169::Biometric {
    fn from(b: &BiometricData) -> Self {
        let format = b
            .format
            .and_then(|v| claim169::BiometricFormat::try_from(v).ok());
        let sub_format = match (format, b.sub_format) {
            (Some(f), Some(v)) => Some(claim169::BiometricSubFormat::from_format_and_value(f, v)),
            (None, Some(v)) => Some(claim169::BiometricSubFormat::Raw(v)),
            _ => None,
        };
        claim169::Biometric {
            data: b.data.clone(),
            format,
            sub_format,
            issuer: b.issuer.clone(),
        }
    }
}

/// X.509 certificate hash.
#[derive(Debug, Clone, uniffi::Record)]
pub struct CertificateHashData {
    /// Hash algorithm — numeric COSE algorithm ID (e.g., -16 for SHA-256) or None if named.
    pub algorithm_numeric: Option<i64>,
    /// Hash algorithm name (for non-numeric algorithms).
    pub algorithm_name: Option<String>,
    /// Hash value bytes.
    pub hash_value: Vec<u8>,
}

impl From<&claim169::CertificateHash> for CertificateHashData {
    fn from(h: &claim169::CertificateHash) -> Self {
        let (algorithm_numeric, algorithm_name) = match &h.algorithm {
            claim169::CertHashAlgorithm::Numeric(n) => (Some(*n), None),
            claim169::CertHashAlgorithm::Named(s) => (None, Some(s.clone())),
        };
        Self {
            algorithm_numeric,
            algorithm_name,
            hash_value: h.hash_value.clone(),
        }
    }
}

/// X.509 certificate headers from COSE structure.
#[derive(Debug, Clone, uniffi::Record)]
pub struct X509HeadersData {
    /// Unordered bag of DER-encoded X.509 certificates.
    pub x5bag: Option<Vec<Vec<u8>>>,
    /// Ordered chain of DER-encoded X.509 certificates.
    pub x5chain: Option<Vec<Vec<u8>>>,
    /// Certificate thumbprint hash.
    pub x5t: Option<CertificateHashData>,
    /// URI pointing to X.509 certificate.
    pub x5u: Option<String>,
}

impl From<&claim169::X509Headers> for X509HeadersData {
    fn from(h: &claim169::X509Headers) -> Self {
        Self {
            x5bag: h.x5bag.clone(),
            x5chain: h.x5chain.clone(),
            x5t: h.x5t.as_ref().map(CertificateHashData::from),
            x5u: h.x5u.clone(),
        }
    }
}

/// Warning generated during decoding.
#[derive(Debug, Clone, uniffi::Record)]
pub struct WarningData {
    /// Warning code string: "expiring_soon", "unknown_fields",
    /// "timestamp_validation_skipped", "biometrics_skipped".
    pub code: String,
    /// Human-readable warning message.
    pub message: String,
}

impl From<&claim169::Warning> for WarningData {
    fn from(w: &claim169::Warning) -> Self {
        let code = match w.code {
            claim169::WarningCode::ExpiringSoon => "expiring_soon",
            claim169::WarningCode::UnknownFields => "unknown_fields",
            claim169::WarningCode::TimestampValidationSkipped => "timestamp_validation_skipped",
            claim169::WarningCode::BiometricsSkipped => "biometrics_skipped",
        };
        Self {
            code: code.to_string(),
            message: w.message.clone(),
        }
    }
}

/// CWT (CBOR Web Token) metadata.
#[derive(Debug, Clone, uniffi::Record)]
pub struct CwtMetaData {
    /// Token issuer URI.
    pub issuer: Option<String>,
    /// Token subject.
    pub subject: Option<String>,
    /// Expiration time (Unix timestamp).
    pub expires_at: Option<i64>,
    /// Not-before time (Unix timestamp).
    pub not_before: Option<i64>,
    /// Issued-at time (Unix timestamp).
    pub issued_at: Option<i64>,
}

impl From<&claim169::CwtMeta> for CwtMetaData {
    fn from(m: &claim169::CwtMeta) -> Self {
        Self {
            issuer: m.issuer.clone(),
            subject: m.subject.clone(),
            expires_at: m.expires_at,
            not_before: m.not_before,
            issued_at: m.issued_at,
        }
    }
}

impl From<&CwtMetaData> for claim169::CwtMeta {
    fn from(m: &CwtMetaData) -> Self {
        let mut cwt = claim169::CwtMeta::new();
        if let Some(ref issuer) = m.issuer {
            cwt = cwt.with_issuer(issuer);
        }
        if let Some(ref subject) = m.subject {
            cwt = cwt.with_subject(subject);
        }
        if let Some(expires_at) = m.expires_at {
            cwt = cwt.with_expires_at(expires_at);
        }
        if let Some(not_before) = m.not_before {
            cwt = cwt.with_not_before(not_before);
        }
        if let Some(issued_at) = m.issued_at {
            cwt = cwt.with_issued_at(issued_at);
        }
        cwt
    }
}

/// Claim 169 identity data.
#[derive(Debug, Clone, uniffi::Record)]
pub struct Claim169Data {
    // Demographics (keys 1-23)
    pub id: Option<String>,
    pub version: Option<String>,
    pub language: Option<String>,
    pub full_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<String>,
    pub gender: Option<i64>,
    pub address: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub nationality: Option<String>,
    pub marital_status: Option<i64>,
    pub guardian: Option<String>,
    pub photo: Option<Vec<u8>>,
    pub photo_format: Option<i64>,
    pub best_quality_fingers: Option<Vec<u8>>,
    pub secondary_full_name: Option<String>,
    pub secondary_language: Option<String>,
    pub location_code: Option<String>,
    pub legal_status: Option<String>,
    pub country_of_issuance: Option<String>,

    // Biometrics (keys 50-65)
    pub right_thumb: Option<Vec<BiometricData>>,
    pub right_pointer_finger: Option<Vec<BiometricData>>,
    pub right_middle_finger: Option<Vec<BiometricData>>,
    pub right_ring_finger: Option<Vec<BiometricData>>,
    pub right_little_finger: Option<Vec<BiometricData>>,
    pub left_thumb: Option<Vec<BiometricData>>,
    pub left_pointer_finger: Option<Vec<BiometricData>>,
    pub left_middle_finger: Option<Vec<BiometricData>>,
    pub left_ring_finger: Option<Vec<BiometricData>>,
    pub left_little_finger: Option<Vec<BiometricData>>,
    pub right_iris: Option<Vec<BiometricData>>,
    pub left_iris: Option<Vec<BiometricData>>,
    pub face: Option<Vec<BiometricData>>,
    pub right_palm: Option<Vec<BiometricData>>,
    pub left_palm: Option<Vec<BiometricData>>,
    pub voice: Option<Vec<BiometricData>>,

    // Unknown fields preserved for forward compatibility (JSON-encoded map)
    pub unknown_fields_json: Option<String>,
}

fn convert_biometrics(bio: &Option<Vec<claim169::Biometric>>) -> Option<Vec<BiometricData>> {
    bio.as_ref()
        .map(|v| v.iter().map(BiometricData::from).collect())
}

fn convert_biometrics_back(bio: &Option<Vec<BiometricData>>) -> Option<Vec<claim169::Biometric>> {
    bio.as_ref()
        .map(|v| v.iter().map(claim169::Biometric::from).collect())
}

impl From<&claim169::Claim169> for Claim169Data {
    fn from(c: &claim169::Claim169) -> Self {
        let unknown_fields_json = if c.unknown_fields.is_empty() {
            None
        } else {
            serde_json::to_string(&c.unknown_fields).ok()
        };

        Self {
            id: c.id.clone(),
            version: c.version.clone(),
            language: c.language.clone(),
            full_name: c.full_name.clone(),
            first_name: c.first_name.clone(),
            middle_name: c.middle_name.clone(),
            last_name: c.last_name.clone(),
            date_of_birth: c.date_of_birth.clone(),
            gender: c.gender.map(|g| g.into()),
            address: c.address.clone(),
            email: c.email.clone(),
            phone: c.phone.clone(),
            nationality: c.nationality.clone(),
            marital_status: c.marital_status.map(|m| m.into()),
            guardian: c.guardian.clone(),
            photo: c.photo.clone(),
            photo_format: c.photo_format.map(|f| f.into()),
            best_quality_fingers: c.best_quality_fingers.clone(),
            secondary_full_name: c.secondary_full_name.clone(),
            secondary_language: c.secondary_language.clone(),
            location_code: c.location_code.clone(),
            legal_status: c.legal_status.clone(),
            country_of_issuance: c.country_of_issuance.clone(),

            right_thumb: convert_biometrics(&c.right_thumb),
            right_pointer_finger: convert_biometrics(&c.right_pointer_finger),
            right_middle_finger: convert_biometrics(&c.right_middle_finger),
            right_ring_finger: convert_biometrics(&c.right_ring_finger),
            right_little_finger: convert_biometrics(&c.right_little_finger),
            left_thumb: convert_biometrics(&c.left_thumb),
            left_pointer_finger: convert_biometrics(&c.left_pointer_finger),
            left_middle_finger: convert_biometrics(&c.left_middle_finger),
            left_ring_finger: convert_biometrics(&c.left_ring_finger),
            left_little_finger: convert_biometrics(&c.left_little_finger),
            right_iris: convert_biometrics(&c.right_iris),
            left_iris: convert_biometrics(&c.left_iris),
            face: convert_biometrics(&c.face),
            right_palm: convert_biometrics(&c.right_palm),
            left_palm: convert_biometrics(&c.left_palm),
            voice: convert_biometrics(&c.voice),

            unknown_fields_json,
        }
    }
}

impl TryFrom<&Claim169Data> for claim169::Claim169 {
    type Error = Claim169Exception;

    fn try_from(c: &Claim169Data) -> Result<Self, Self::Error> {
        let unknown_fields = match &c.unknown_fields_json {
            Some(s) => serde_json::from_str(s).map_err(|e| {
                Claim169Exception::Claim169Invalid(format!("invalid unknown_fields_json: {}", e))
            })?,
            None => Default::default(),
        };

        Ok(claim169::Claim169 {
            id: c.id.clone(),
            version: c.version.clone(),
            language: c.language.clone(),
            full_name: c.full_name.clone(),
            first_name: c.first_name.clone(),
            middle_name: c.middle_name.clone(),
            last_name: c.last_name.clone(),
            date_of_birth: c.date_of_birth.clone(),
            gender: c.gender.and_then(|v| claim169::Gender::try_from(v).ok()),
            address: c.address.clone(),
            email: c.email.clone(),
            phone: c.phone.clone(),
            nationality: c.nationality.clone(),
            marital_status: c
                .marital_status
                .and_then(|v| claim169::MaritalStatus::try_from(v).ok()),
            guardian: c.guardian.clone(),
            photo: c.photo.clone(),
            photo_format: c
                .photo_format
                .and_then(|v| claim169::PhotoFormat::try_from(v).ok()),
            best_quality_fingers: c.best_quality_fingers.clone(),
            secondary_full_name: c.secondary_full_name.clone(),
            secondary_language: c.secondary_language.clone(),
            location_code: c.location_code.clone(),
            legal_status: c.legal_status.clone(),
            country_of_issuance: c.country_of_issuance.clone(),

            right_thumb: convert_biometrics_back(&c.right_thumb),
            right_pointer_finger: convert_biometrics_back(&c.right_pointer_finger),
            right_middle_finger: convert_biometrics_back(&c.right_middle_finger),
            right_ring_finger: convert_biometrics_back(&c.right_ring_finger),
            right_little_finger: convert_biometrics_back(&c.right_little_finger),
            left_thumb: convert_biometrics_back(&c.left_thumb),
            left_pointer_finger: convert_biometrics_back(&c.left_pointer_finger),
            left_middle_finger: convert_biometrics_back(&c.left_middle_finger),
            left_ring_finger: convert_biometrics_back(&c.left_ring_finger),
            left_little_finger: convert_biometrics_back(&c.left_little_finger),
            right_iris: convert_biometrics_back(&c.right_iris),
            left_iris: convert_biometrics_back(&c.left_iris),
            face: convert_biometrics_back(&c.face),
            right_palm: convert_biometrics_back(&c.right_palm),
            left_palm: convert_biometrics_back(&c.left_palm),
            voice: convert_biometrics_back(&c.voice),

            unknown_fields,
        })
    }
}

/// Result of decoding a Claim 169 QR code.
#[derive(Debug, Clone, uniffi::Record)]
pub struct DecodeResultData {
    /// The extracted Claim 169 identity data.
    pub claim169: Claim169Data,
    /// CWT metadata (issuer, expiration, etc.).
    pub cwt_meta: CwtMetaData,
    /// Verification status: "verified", "failed", or "skipped".
    pub verification_status: String,
    /// X.509 certificate headers from the COSE structure.
    pub x509_headers: X509HeadersData,
    /// Warnings generated during decoding.
    pub warnings: Vec<WarningData>,
}

// ============================================================
// Callback interfaces for custom crypto
// ============================================================

/// Callback interface for custom signature verification (HSM, KMS, etc.).
#[uniffi::export(callback_interface)]
pub trait SignatureVerifierCallback: Send + Sync {
    /// Verify a digital signature.
    ///
    /// # Arguments
    /// * `algorithm` - COSE algorithm name (e.g., "EdDSA", "ES256")
    /// * `key_id` - Optional key identifier bytes
    /// * `data` - The data that was signed (Sig_structure)
    /// * `signature` - The signature bytes to verify
    ///
    /// # Errors
    /// Throw `CryptoException` if verification fails.
    fn verify(
        &self,
        algorithm: String,
        key_id: Option<Vec<u8>>,
        data: Vec<u8>,
        signature: Vec<u8>,
    ) -> Result<(), CryptoException>;
}

/// Callback interface for custom decryption (HSM, KMS, etc.).
#[uniffi::export(callback_interface)]
pub trait DecryptorCallback: Send + Sync {
    /// Decrypt ciphertext using AEAD.
    fn decrypt(
        &self,
        algorithm: String,
        key_id: Option<Vec<u8>>,
        nonce: Vec<u8>,
        aad: Vec<u8>,
        ciphertext: Vec<u8>,
    ) -> Result<Vec<u8>, CryptoException>;
}

/// Callback interface for custom signing (HSM, KMS, etc.).
#[uniffi::export(callback_interface)]
pub trait SignerCallback: Send + Sync {
    /// Sign data and return the signature.
    fn sign(
        &self,
        algorithm: String,
        key_id: Option<Vec<u8>>,
        data: Vec<u8>,
    ) -> Result<Vec<u8>, CryptoException>;

    /// Get the key ID for this signer. Returns None if no key ID.
    fn key_id(&self) -> Option<Vec<u8>>;
}

/// Callback interface for custom encryption (HSM, KMS, etc.).
#[uniffi::export(callback_interface)]
pub trait EncryptorCallback: Send + Sync {
    /// Encrypt plaintext using AEAD.
    fn encrypt(
        &self,
        algorithm: String,
        key_id: Option<Vec<u8>>,
        nonce: Vec<u8>,
        aad: Vec<u8>,
        plaintext: Vec<u8>,
    ) -> Result<Vec<u8>, CryptoException>;
}

// ============================================================
// Algorithm string <-> iana::Algorithm conversion
// ============================================================

fn algorithm_to_string(algorithm: iana::Algorithm) -> String {
    match algorithm {
        iana::Algorithm::EdDSA => "EdDSA".to_string(),
        iana::Algorithm::ES256 => "ES256".to_string(),
        iana::Algorithm::ES384 => "ES384".to_string(),
        iana::Algorithm::ES512 => "ES512".to_string(),
        iana::Algorithm::A128GCM => "A128GCM".to_string(),
        iana::Algorithm::A192GCM => "A192GCM".to_string(),
        iana::Algorithm::A256GCM => "A256GCM".to_string(),
        other => format!("COSE_ALG_{}", other.to_i64()),
    }
}

fn algorithm_from_string(s: &str) -> Result<iana::Algorithm, Claim169Exception> {
    match s {
        "EdDSA" => Ok(iana::Algorithm::EdDSA),
        "ES256" => Ok(iana::Algorithm::ES256),
        "ES384" => Ok(iana::Algorithm::ES384),
        "ES512" => Ok(iana::Algorithm::ES512),
        "A128GCM" => Ok(iana::Algorithm::A128GCM),
        "A192GCM" => Ok(iana::Algorithm::A192GCM),
        "A256GCM" => Ok(iana::Algorithm::A256GCM),
        _ => {
            if let Some(id_str) = s.strip_prefix("COSE_ALG_") {
                let id: i64 = id_str.parse().map_err(|_| {
                    Claim169Exception::UnsupportedAlgorithm(format!(
                        "invalid numeric algorithm ID: {}",
                        s
                    ))
                })?;
                iana::Algorithm::from_i64(id).ok_or_else(|| {
                    Claim169Exception::UnsupportedAlgorithm(format!(
                        "unknown COSE algorithm ID: {}",
                        id
                    ))
                })
            } else {
                Err(Claim169Exception::UnsupportedAlgorithm(s.to_string()))
            }
        }
    }
}

// ============================================================
// Callback → core trait adapters
// ============================================================

struct CallbackVerifier {
    callback: Box<dyn SignatureVerifierCallback>,
}

impl claim169::SignatureVerifier for CallbackVerifier {
    fn verify(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
        signature: &[u8],
    ) -> claim169::CryptoResult<()> {
        self.callback
            .verify(
                algorithm_to_string(algorithm),
                key_id.map(|k| k.to_vec()),
                data.to_vec(),
                signature.to_vec(),
            )
            .map_err(claim169::CryptoError::from)
    }
}

struct CallbackDecryptor {
    callback: Box<dyn DecryptorCallback>,
}

impl claim169::Decryptor for CallbackDecryptor {
    fn decrypt(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        ciphertext: &[u8],
    ) -> claim169::CryptoResult<Vec<u8>> {
        self.callback
            .decrypt(
                algorithm_to_string(algorithm),
                key_id.map(|k| k.to_vec()),
                nonce.to_vec(),
                aad.to_vec(),
                ciphertext.to_vec(),
            )
            .map_err(claim169::CryptoError::from)
    }
}

struct CallbackSigner {
    callback: Box<dyn SignerCallback>,
    cached_key_id: Option<Vec<u8>>,
}

impl claim169::Signer for CallbackSigner {
    fn sign(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
    ) -> claim169::CryptoResult<Vec<u8>> {
        self.callback
            .sign(
                algorithm_to_string(algorithm),
                key_id.map(|k| k.to_vec()),
                data.to_vec(),
            )
            .map_err(claim169::CryptoError::from)
    }

    fn key_id(&self) -> Option<&[u8]> {
        self.cached_key_id.as_deref()
    }
}

struct CallbackEncryptor {
    callback: Box<dyn EncryptorCallback>,
}

impl claim169::Encryptor for CallbackEncryptor {
    fn encrypt(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        plaintext: &[u8],
    ) -> claim169::CryptoResult<Vec<u8>> {
        self.callback
            .encrypt(
                algorithm_to_string(algorithm),
                key_id.map(|k| k.to_vec()),
                nonce.to_vec(),
                aad.to_vec(),
                plaintext.to_vec(),
            )
            .map_err(claim169::CryptoError::from)
    }
}

// ============================================================
// Decoder object (UniFFI Object with interior mutability)
// ============================================================

/// Builder for decoding Claim 169 QR codes.
#[derive(uniffi::Object)]
pub struct Claim169Decoder {
    inner: Mutex<Option<claim169::Decoder>>,
}

impl Claim169Decoder {
    /// Lock the inner mutex, take the decoder, apply a transformation, and put it back.
    fn with_decoder<F>(&self, f: F) -> Result<(), Claim169Exception>
    where
        F: FnOnce(claim169::Decoder) -> Result<claim169::Decoder, Claim169Exception>,
    {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let decoder = guard.take().ok_or_else(|| {
            Claim169Exception::DecodingConfig("decoder already consumed".to_string())
        })?;
        *guard = Some(f(decoder)?);
        Ok(())
    }

    /// Take the decoder from the mutex without putting it back (for terminal operations).
    fn take_decoder(&self) -> Result<claim169::Decoder, Claim169Exception> {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        guard.take().ok_or_else(|| {
            Claim169Exception::DecodingConfig("decoder already consumed".to_string())
        })
    }
}

#[uniffi::export]
impl Claim169Decoder {
    /// Create a decoder for the given QR text string.
    #[uniffi::constructor]
    pub fn new(qr_text: String) -> Self {
        Self {
            inner: Mutex::new(Some(claim169::Decoder::new(qr_text))),
        }
    }

    /// Verify with an Ed25519 public key (32 raw bytes).
    pub fn verify_with_ed25519(&self, public_key: Vec<u8>) -> Result<(), Claim169Exception> {
        self.with_decoder(|d| {
            d.verify_with_ed25519(&public_key)
                .map_err(Claim169Exception::from)
        })
    }

    /// Verify with an Ed25519 public key in PEM format.
    pub fn verify_with_ed25519_pem(&self, pem: String) -> Result<(), Claim169Exception> {
        self.with_decoder(|d| {
            d.verify_with_ed25519_pem(&pem)
                .map_err(Claim169Exception::from)
        })
    }

    /// Verify with an ECDSA P-256 public key (SEC1-encoded, 33 or 65 bytes).
    pub fn verify_with_ecdsa_p256(&self, public_key: Vec<u8>) -> Result<(), Claim169Exception> {
        self.with_decoder(|d| {
            d.verify_with_ecdsa_p256(&public_key)
                .map_err(Claim169Exception::from)
        })
    }

    /// Verify with an ECDSA P-256 public key in PEM format.
    pub fn verify_with_ecdsa_p256_pem(&self, pem: String) -> Result<(), Claim169Exception> {
        self.with_decoder(|d| {
            d.verify_with_ecdsa_p256_pem(&pem)
                .map_err(Claim169Exception::from)
        })
    }

    /// Verify with a custom verifier callback (for HSM/KMS integration).
    pub fn verify_with_callback(
        &self,
        verifier: Box<dyn SignatureVerifierCallback>,
    ) -> Result<(), Claim169Exception> {
        self.with_decoder(|d| {
            let adapter = CallbackVerifier { callback: verifier };
            Ok(d.verify_with(adapter))
        })
    }

    /// Allow decoding without signature verification.
    pub fn allow_unverified(&self) -> Result<(), Claim169Exception> {
        self.with_decoder(|d| Ok(d.allow_unverified()))
    }

    /// Decrypt with AES-256-GCM (32-byte key).
    pub fn decrypt_with_aes256(&self, key: Vec<u8>) -> Result<(), Claim169Exception> {
        self.with_decoder(|d| d.decrypt_with_aes256(&key).map_err(Claim169Exception::from))
    }

    /// Decrypt with AES-128-GCM (16-byte key).
    pub fn decrypt_with_aes128(&self, key: Vec<u8>) -> Result<(), Claim169Exception> {
        self.with_decoder(|d| d.decrypt_with_aes128(&key).map_err(Claim169Exception::from))
    }

    /// Decrypt with a custom decryptor callback (for HSM/KMS integration).
    pub fn decrypt_with_callback(
        &self,
        decryptor: Box<dyn DecryptorCallback>,
    ) -> Result<(), Claim169Exception> {
        self.with_decoder(|d| {
            let adapter = CallbackDecryptor {
                callback: decryptor,
            };
            Ok(d.decrypt_with(adapter))
        })
    }

    /// Skip biometric data parsing for faster decoding.
    pub fn skip_biometrics(&self) -> Result<(), Claim169Exception> {
        self.with_decoder(|d| Ok(d.skip_biometrics()))
    }

    /// Disable timestamp validation (expiration and not-before checks).
    pub fn without_timestamp_validation(&self) -> Result<(), Claim169Exception> {
        self.with_decoder(|d| Ok(d.without_timestamp_validation()))
    }

    /// Set clock skew tolerance for timestamp validation (in seconds).
    pub fn clock_skew_tolerance(&self, seconds: i64) -> Result<(), Claim169Exception> {
        if seconds < 0 {
            return Err(Claim169Exception::DecodingConfig(format!(
                "clock skew tolerance must be non-negative, got {}",
                seconds
            )));
        }
        self.with_decoder(|d| Ok(d.clock_skew_tolerance(seconds)))
    }

    /// Set maximum decompressed size in bytes (default: 65536).
    pub fn max_decompressed_bytes(&self, max_bytes: u64) -> Result<(), Claim169Exception> {
        let max_bytes_usize = usize::try_from(max_bytes).map_err(|_| {
            Claim169Exception::DecodingConfig(format!(
                "max_decompressed_bytes value {} exceeds platform limit of {}",
                max_bytes,
                usize::MAX
            ))
        })?;
        self.with_decoder(|d| Ok(d.max_decompressed_bytes(max_bytes_usize)))
    }

    /// Execute the decode operation and return the result.
    ///
    /// This consumes the decoder — it cannot be reused after calling this method.
    pub fn execute(&self) -> Result<DecodeResultData, Claim169Exception> {
        let decoder = self.take_decoder()?;
        let result = decoder.decode().map_err(Claim169Exception::from)?;

        Ok(DecodeResultData {
            claim169: Claim169Data::from(&result.claim169),
            cwt_meta: CwtMetaData::from(&result.cwt_meta),
            verification_status: result.verification_status.to_string(),
            x509_headers: X509HeadersData::from(&result.x509_headers),
            warnings: result.warnings.iter().map(WarningData::from).collect(),
        })
    }
}

// ============================================================
// Encoder object (UniFFI Object with interior mutability)
// ============================================================

/// Builder for encoding Claim 169 credentials into QR-ready strings.
#[derive(uniffi::Object)]
pub struct Claim169Encoder {
    inner: Mutex<Option<claim169::Encoder>>,
}

impl Claim169Encoder {
    /// Lock the inner mutex, take the encoder, apply a transformation, and put it back.
    fn with_encoder<F>(&self, f: F) -> Result<(), Claim169Exception>
    where
        F: FnOnce(claim169::Encoder) -> Result<claim169::Encoder, Claim169Exception>,
    {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let encoder = guard.take().ok_or_else(|| {
            Claim169Exception::EncodingConfig("encoder already consumed".to_string())
        })?;
        *guard = Some(f(encoder)?);
        Ok(())
    }

    /// Take the encoder from the mutex without putting it back (for terminal operations).
    fn take_encoder(&self) -> Result<claim169::Encoder, Claim169Exception> {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        guard.take().ok_or_else(|| {
            Claim169Exception::EncodingConfig("encoder already consumed".to_string())
        })
    }
}

#[uniffi::export]
impl Claim169Encoder {
    /// Create an encoder with the given claim data and CWT metadata.
    #[uniffi::constructor]
    pub fn new(
        claim169_data: Claim169Data,
        cwt_meta: CwtMetaData,
    ) -> Result<Self, Claim169Exception> {
        let core_claim = claim169::Claim169::try_from(&claim169_data)?;
        let core_meta = claim169::CwtMeta::from(&cwt_meta);
        Ok(Self {
            inner: Mutex::new(Some(claim169::Encoder::new(core_claim, core_meta))),
        })
    }

    /// Sign with an Ed25519 private key (32 raw bytes).
    pub fn sign_with_ed25519(&self, private_key: Vec<u8>) -> Result<(), Claim169Exception> {
        self.with_encoder(|e| {
            e.sign_with_ed25519(&private_key)
                .map_err(Claim169Exception::from)
        })
    }

    /// Sign with an ECDSA P-256 private key (32-byte scalar).
    pub fn sign_with_ecdsa_p256(&self, private_key: Vec<u8>) -> Result<(), Claim169Exception> {
        self.with_encoder(|e| {
            e.sign_with_ecdsa_p256(&private_key)
                .map_err(Claim169Exception::from)
        })
    }

    /// Sign with a custom signer callback (for HSM/KMS integration).
    pub fn sign_with_callback(
        &self,
        signer: Box<dyn SignerCallback>,
        algorithm: String,
    ) -> Result<(), Claim169Exception> {
        let alg = algorithm_from_string(&algorithm)?;
        self.with_encoder(|e| {
            let cached_key_id = signer.key_id();
            let adapter = CallbackSigner {
                callback: signer,
                cached_key_id,
            };
            Ok(e.sign_with(adapter, alg))
        })
    }

    /// Allow encoding without a signature.
    pub fn allow_unsigned(&self) -> Result<(), Claim169Exception> {
        self.with_encoder(|e| Ok(e.allow_unsigned()))
    }

    /// Encrypt with AES-256-GCM (32-byte key). Nonce is generated randomly.
    pub fn encrypt_with_aes256(&self, key: Vec<u8>) -> Result<(), Claim169Exception> {
        self.with_encoder(|e| e.encrypt_with_aes256(&key).map_err(Claim169Exception::from))
    }

    /// Encrypt with AES-128-GCM (16-byte key). Nonce is generated randomly.
    pub fn encrypt_with_aes128(&self, key: Vec<u8>) -> Result<(), Claim169Exception> {
        self.with_encoder(|e| e.encrypt_with_aes128(&key).map_err(Claim169Exception::from))
    }

    /// Encrypt with a custom encryptor callback (for HSM/KMS integration).
    pub fn encrypt_with_callback(
        &self,
        encryptor: Box<dyn EncryptorCallback>,
        algorithm: String,
    ) -> Result<(), Claim169Exception> {
        let alg = algorithm_from_string(&algorithm)?;
        self.with_encoder(|e| {
            let adapter = CallbackEncryptor {
                callback: encryptor,
            };
            Ok(e.encrypt_with(adapter, alg))
        })
    }

    /// Skip biometric data during encoding.
    pub fn skip_biometrics(&self) -> Result<(), Claim169Exception> {
        self.with_encoder(|e| Ok(e.skip_biometrics()))
    }

    /// Execute the encode operation and return the Base45-encoded QR string.
    pub fn execute(&self) -> Result<String, Claim169Exception> {
        let encoder = self.take_encoder()?;
        encoder.encode().map_err(Claim169Exception::from)
    }
}

// ============================================================
// Convenience free functions
// ============================================================

/// Get the library version.
#[uniffi::export]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!version().is_empty());
    }

    #[test]
    fn test_decode_unverified() {
        let claim = claim169::Claim169::minimal("test-123", "Test User");
        let cwt = claim169::CwtMeta::new()
            .with_issuer("https://test.issuer")
            .with_expires_at(i64::MAX);
        let qr_data = claim169::Encoder::new(claim, cwt)
            .allow_unsigned()
            .encode()
            .unwrap();

        let decoder = Claim169Decoder::new(qr_data);
        decoder.allow_unverified().unwrap();
        let result = decoder.execute().unwrap();

        assert_eq!(result.claim169.id.as_deref(), Some("test-123"));
        assert_eq!(result.claim169.full_name.as_deref(), Some("Test User"));
        assert_eq!(result.verification_status, "skipped");
    }

    #[test]
    fn test_decode_with_ed25519() {
        let signer = claim169::Ed25519Signer::generate();
        let public_key = signer.public_key_bytes();

        let claim = claim169::Claim169::minimal("signed-test", "Signed User");
        let cwt = claim169::CwtMeta::new()
            .with_issuer("https://test.issuer")
            .with_expires_at(i64::MAX);
        let qr_data = claim169::Encoder::new(claim, cwt)
            .sign_with(signer, iana::Algorithm::EdDSA)
            .encode()
            .unwrap();

        let decoder = Claim169Decoder::new(qr_data);
        decoder.verify_with_ed25519(public_key.to_vec()).unwrap();
        let result = decoder.execute().unwrap();

        assert_eq!(result.claim169.id.as_deref(), Some("signed-test"));
        assert_eq!(result.verification_status, "verified");
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let claim_data = Claim169Data {
            id: Some("encode-test".to_string()),
            full_name: Some("Encode User".to_string()),
            version: None,
            language: None,
            first_name: None,
            middle_name: None,
            last_name: None,
            date_of_birth: Some("19900115".to_string()),
            gender: Some(2),
            address: None,
            email: Some("user@example.com".to_string()),
            phone: None,
            nationality: None,
            marital_status: None,
            guardian: None,
            photo: None,
            photo_format: None,
            best_quality_fingers: None,
            secondary_full_name: None,
            secondary_language: None,
            location_code: None,
            legal_status: None,
            country_of_issuance: None,
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
            unknown_fields_json: None,
        };
        let cwt_data = CwtMetaData {
            issuer: Some("https://test.issuer".to_string()),
            subject: None,
            expires_at: Some(i64::MAX),
            not_before: None,
            issued_at: None,
        };

        let encoder = Claim169Encoder::new(claim_data, cwt_data).unwrap();
        encoder.allow_unsigned().unwrap();
        let qr_data = encoder.execute().unwrap();

        let decoder = Claim169Decoder::new(qr_data);
        decoder.allow_unverified().unwrap();
        let result = decoder.execute().unwrap();

        assert_eq!(result.claim169.id.as_deref(), Some("encode-test"));
        assert_eq!(result.claim169.full_name.as_deref(), Some("Encode User"));
        assert_eq!(result.claim169.date_of_birth.as_deref(), Some("19900115"));
        assert_eq!(result.claim169.gender, Some(2));
        assert_eq!(result.claim169.email.as_deref(), Some("user@example.com"));
    }

    #[test]
    fn test_decoder_already_consumed() {
        let claim = claim169::Claim169::minimal("test", "Test");
        let cwt = claim169::CwtMeta::default();
        let qr_data = claim169::Encoder::new(claim, cwt)
            .allow_unsigned()
            .encode()
            .unwrap();

        let decoder = Claim169Decoder::new(qr_data);
        decoder.allow_unverified().unwrap();
        decoder.execute().unwrap();

        let result = decoder.execute();
        assert!(result.is_err());
    }

    #[test]
    fn test_clock_skew_tolerance_rejects_negative() {
        let decoder = Claim169Decoder::new("dummy".to_string());
        let result = decoder.clock_skew_tolerance(-1);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Claim169Exception::DecodingConfig(_)
        ));
    }

    #[test]
    fn test_clock_skew_tolerance_accepts_zero() {
        let decoder = Claim169Decoder::new("dummy".to_string());
        let result = decoder.clock_skew_tolerance(0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_conversion() {
        let err = claim169::Claim169Error::Base45Decode("test error".to_string());
        let exc = Claim169Exception::from(err);
        assert!(matches!(exc, Claim169Exception::Base45Decode(_)));
    }

    #[test]
    fn test_algorithm_roundtrip_known() {
        let known = vec![
            (iana::Algorithm::EdDSA, "EdDSA"),
            (iana::Algorithm::ES256, "ES256"),
            (iana::Algorithm::ES384, "ES384"),
            (iana::Algorithm::ES512, "ES512"),
            (iana::Algorithm::A128GCM, "A128GCM"),
            (iana::Algorithm::A192GCM, "A192GCM"),
            (iana::Algorithm::A256GCM, "A256GCM"),
        ];
        for (alg, expected_str) in known {
            let s = algorithm_to_string(alg);
            assert_eq!(s, expected_str);
            let roundtripped = algorithm_from_string(&s).unwrap();
            assert_eq!(roundtripped, alg);
        }
    }

    #[test]
    fn test_algorithm_roundtrip_unknown() {
        // PS256 is COSE algorithm -37, not in our named mapping
        let alg = iana::Algorithm::PS256;
        let s = algorithm_to_string(alg);
        assert_eq!(s, "COSE_ALG_-37");
        let roundtripped = algorithm_from_string(&s).unwrap();
        assert_eq!(roundtripped, alg);
    }

    #[test]
    fn test_algorithm_from_string_invalid_prefix() {
        let result = algorithm_from_string("COSE_ALG_not_a_number");
        assert!(result.is_err());
    }

    #[test]
    fn test_algorithm_from_string_unknown_name() {
        let result = algorithm_from_string("UnknownAlg");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Claim169Exception::UnsupportedAlgorithm(_)
        ));
    }

    #[test]
    fn test_malformed_unknown_fields_json_returns_error() {
        let claim_data = Claim169Data {
            id: Some("test".to_string()),
            full_name: Some("Test".to_string()),
            version: None,
            language: None,
            first_name: None,
            middle_name: None,
            last_name: None,
            date_of_birth: None,
            gender: None,
            address: None,
            email: None,
            phone: None,
            nationality: None,
            marital_status: None,
            guardian: None,
            photo: None,
            photo_format: None,
            best_quality_fingers: None,
            secondary_full_name: None,
            secondary_language: None,
            location_code: None,
            legal_status: None,
            country_of_issuance: None,
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
            unknown_fields_json: Some("not valid json{{{".to_string()),
        };
        let cwt_data = CwtMetaData {
            issuer: None,
            subject: None,
            expires_at: None,
            not_before: None,
            issued_at: None,
        };

        let result = Claim169Encoder::new(claim_data, cwt_data);
        match result {
            Err(Claim169Exception::Claim169Invalid(msg)) => {
                assert!(
                    msg.contains("unknown_fields_json"),
                    "error should mention unknown_fields_json: {}",
                    msg
                );
            }
            Err(other) => panic!("expected Claim169Invalid, got: {:?}", other),
            Ok(_) => panic!("expected error for malformed JSON, but got Ok"),
        }
    }

    #[test]
    fn test_valid_unknown_fields_json_accepted() {
        let claim_data = Claim169Data {
            id: Some("test".to_string()),
            full_name: Some("Test".to_string()),
            version: None,
            language: None,
            first_name: None,
            middle_name: None,
            last_name: None,
            date_of_birth: None,
            gender: None,
            address: None,
            email: None,
            phone: None,
            nationality: None,
            marital_status: None,
            guardian: None,
            photo: None,
            photo_format: None,
            best_quality_fingers: None,
            secondary_full_name: None,
            secondary_language: None,
            location_code: None,
            legal_status: None,
            country_of_issuance: None,
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
            unknown_fields_json: Some(r#"{"100":"extra_value"}"#.to_string()),
        };
        let cwt_data = CwtMetaData {
            issuer: None,
            subject: None,
            expires_at: None,
            not_before: None,
            issued_at: None,
        };

        let result = Claim169Encoder::new(claim_data, cwt_data);
        assert!(result.is_ok());
    }
}
