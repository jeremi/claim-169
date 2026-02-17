//! # claim169-core
//!
//! A Rust library for encoding and decoding MOSIP Claim 169 QR codes.
//!
//! ## Overview
//!
//! [MOSIP Claim 169](https://github.com/mosip/id-claim-169/tree/main) defines a standard for encoding identity data
//! in QR codes, designed for offline verification of digital identity credentials. The format
//! uses a compact binary encoding optimized for QR code capacity constraints.
//!
//! The encoding pipeline:
//! ```text
//! Claim169 → CBOR → CWT → COSE_Sign1 → [COSE_Encrypt0] → zlib → Base45 → QR Code
//! ```
//!
//! Key technologies:
//! - **CBOR**: Compact binary encoding with numeric keys for minimal size
//! - **CWT**: CBOR Web Token for standard claims (issuer, expiration, etc.)
//! - **COSE_Sign1**: Digital signature for authenticity (Ed25519 or ECDSA P-256)
//! - **COSE_Encrypt0**: Optional encryption layer (AES-GCM)
//! - **zlib + Base45**: Compression and alphanumeric encoding for QR efficiency
//!
//! ## Quick Start
//!
//! ### Encoding (Creating QR Codes)
//!
//! ```rust,ignore
//! use claim169_core::{Encoder, Claim169, CwtMeta};
//!
//! let claim169 = Claim169 {
//!     id: Some("123456789".to_string()),
//!     full_name: Some("John Doe".to_string()),
//!     ..Default::default()
//! };
//!
//! let cwt_meta = CwtMeta::new()
//!     .with_issuer("https://issuer.example.com")
//!     .with_expires_at(1800000000);
//!
//! // Ed25519 signed (recommended)
//! let qr_data = Encoder::new(claim169, cwt_meta)
//!     .sign_with_ed25519(&private_key)?
//!     .encode()?;
//!
//! // Signed and encrypted
//! let qr_data = Encoder::new(claim169, cwt_meta)
//!     .sign_with_ed25519(&private_key)?
//!     .encrypt_with_aes256(&aes_key)?
//!     .encode()?;
//!
//! // Unsigned (testing only - requires explicit opt-in)
//! let qr_data = Encoder::new(claim169, cwt_meta)
//!     .allow_unsigned()
//!     .encode()?;
//! ```
//!
//! ### Decoding (Reading QR Codes)
//!
//! ```rust,ignore
//! use claim169_core::Decoder;
//!
//! // With Ed25519 verification (recommended)
//! let result = Decoder::new(qr_content)
//!     .verify_with_ed25519(&public_key)?
//!     .decode()?;
//!
//! println!("ID: {:?}", result.claim169.id);
//! println!("Name: {:?}", result.claim169.full_name);
//! println!("Issuer: {:?}", result.cwt_meta.issuer);
//!
//! // Decrypting encrypted credentials
//! let result = Decoder::new(qr_content)
//!     .decrypt_with_aes256(&aes_key)?
//!     .verify_with_ed25519(&public_key)?
//!     .decode()?;
//!
//! // Without verification (testing only - requires explicit opt-in)
//! let result = Decoder::new(qr_content)
//!     .allow_unverified()
//!     .decode()?;
//! ```
//!
//! ### Using Custom Cryptography (HSM Integration)
//!
//! For hardware security modules or custom cryptographic backends:
//!
//! ```rust,ignore
//! use claim169_core::{Encoder, Decoder, Signer, SignatureVerifier};
//!
//! // Implement the Signer trait for your HSM
//! struct HsmSigner { /* ... */ }
//! impl Signer for HsmSigner { /* ... */ }
//!
//! // Encoding with HSM
//! let qr_data = Encoder::new(claim169, cwt_meta)
//!     .sign_with(hsm_signer, iana::Algorithm::EdDSA)
//!     .encode()?;
//!
//! // Decoding with HSM
//! let result = Decoder::new(qr_content)
//!     .verify_with(hsm_verifier)
//!     .decode()?;
//! ```
//!
//! ## Security Considerations
//!
//! - **Always verify signatures** in production - use `.verify_with_*()` methods
//! - **Always sign credentials** in production - use `.sign_with_*()` methods
//! - Unsigned/unverified requires explicit opt-in with `.allow_unsigned()`/`.allow_unverified()`
//! - Decompression is limited to prevent zip bomb attacks (default: 64KB)
//! - Timestamps are validated by default; use `.without_timestamp_validation()` to disable
//! - Weak cryptographic keys (all-zeros, small-order points) are automatically rejected
//!
//! ## Features
//!
//! | Feature | Default | Description |
//! |---------|---------|-------------|
//! | `software-crypto` | ✓ | Software implementations of Ed25519, ECDSA P-256, and AES-GCM |
//!
//! Disable default features to integrate with HSMs or custom cryptographic backends:
//!
//! ```toml
//! [dependencies]
//! claim169-core = { version = "0.1", default-features = false }
//! ```
//!
//! Then implement the [`Signer`], [`SignatureVerifier`], [`Encryptor`], or [`Decryptor`] traits.
//!
//! ## Modules
//!
//! - [`crypto`]: Cryptographic traits and implementations
//! - [`error`]: Error types for encoding, decoding, and crypto operations
//! - [`model`]: Data structures for Claim 169 identity data
//! - [`pipeline`]: Low-level encoding/decoding pipeline functions

pub mod crypto;
pub mod decode;
pub mod encode;
pub mod error;
pub mod model;
pub mod pipeline;
pub mod serde_utils;

// Re-export builder pattern API (primary interface)
pub use decode::Decoder;
pub use encode::{EncodeResult, Encoder};

// Re-export nonce generation when software-crypto is enabled
#[cfg(feature = "software-crypto")]
pub use encode::generate_random_nonce;

// Re-export cryptographic traits (for HSM integration)
pub use crypto::traits::{Decryptor, Encryptor, KeyResolver, SignatureVerifier, Signer};

// Re-export error types
pub use error::{Claim169Error, CryptoError, CryptoResult, Result};

// Re-export model types
pub use model::{
    Biometric, BiometricFormat, BiometricSubFormat, CertHashAlgorithm, CertificateHash, Claim169,
    CwtMeta, Gender, MaritalStatus, PhotoFormat, VerificationStatus, X509Headers,
};

// Re-export compression types
pub use pipeline::{Compression, DetectedCompression};

// Re-export software crypto implementations when feature is enabled
#[cfg(feature = "software-crypto")]
pub use crypto::{
    AesGcmDecryptor, AesGcmEncryptor, EcdsaP256Signer, EcdsaP256Verifier, Ed25519Signer,
    Ed25519Verifier,
};

/// Result of successfully decoding a Claim 169 QR code.
///
/// This struct contains all the data extracted from the QR code:
/// - The identity data ([`Claim169`])
/// - CWT metadata like issuer and expiration ([`CwtMeta`])
/// - The signature verification status
/// - Any warnings generated during decoding
///
/// # Example
///
/// ```rust,ignore
/// let result = Decoder::new(qr_content)
///     .verify_with_ed25519(&public_key)?
///     .decode()?;
///
/// // Access identity data
/// if let Some(name) = &result.claim169.full_name {
///     println!("Welcome, {}!", name);
/// }
///
/// // Check verification status
/// match result.verification_status {
///     VerificationStatus::Verified => println!("Signature verified"),
///     VerificationStatus::Skipped => println!("Verification skipped"),
///     VerificationStatus::Failed => println!("Verification failed"),
/// }
///
/// // Check for warnings
/// for warning in &result.warnings {
///     println!("Warning: {}", warning.message);
/// }
/// ```
#[non_exhaustive]
#[derive(Debug)]
pub struct DecodeResult {
    /// The extracted Claim 169 identity data.
    ///
    /// Contains demographic information (name, date of birth, address, etc.)
    /// and optionally biometric data (fingerprints, iris scans, face images).
    pub claim169: Claim169,

    /// CWT (CBOR Web Token) metadata.
    ///
    /// Contains standard claims like issuer, subject, expiration time,
    /// and issued-at timestamp.
    pub cwt_meta: CwtMeta,

    /// Signature verification status.
    ///
    /// - `Verified`: Signature was checked and is valid
    /// - `Skipped`: No verifier was provided (only if `allow_unverified` was set)
    /// - `Failed`: Signature verification failed (this typically returns an error instead)
    pub verification_status: VerificationStatus,

    /// X.509 certificate headers from the COSE structure.
    ///
    /// Contains any X.509 certificate information present in the COSE
    /// protected/unprotected headers (x5bag, x5chain, x5t, x5u).
    pub x509_headers: X509Headers,

    /// The compression format detected during decoding.
    ///
    /// Indicates which compression format was auto-detected and used:
    /// `Zlib` (spec-compliant), `Brotli` (non-standard), or `None` (raw).
    pub detected_compression: DetectedCompression,

    /// Warnings generated during decoding.
    ///
    /// Non-fatal issues that don't prevent decoding but may warrant attention,
    /// such as unknown fields (forward compatibility) or skipped validations.
    pub warnings: Vec<Warning>,

    /// Key ID from the COSE protected header, if present.
    ///
    /// Useful for identifying which key was used for signing in multi-issuer
    /// or key-rotation scenarios.
    pub key_id: Option<Vec<u8>>,

    /// COSE algorithm used for signing or encryption.
    ///
    /// Reflects the algorithm declared in the COSE protected header (e.g., EdDSA, ES256).
    pub algorithm: Option<coset::iana::Algorithm>,
}

/// A warning generated during the decoding process.
///
/// Warnings represent non-fatal issues that don't prevent successful decoding
/// but may be relevant for logging or auditing purposes.
#[derive(Debug, Clone)]
pub struct Warning {
    /// The type of warning.
    pub code: WarningCode,
    /// Human-readable description of the warning.
    pub message: String,
}

/// Types of warnings that can be generated during decoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarningCode {
    /// The credential will expire soon (within a configurable threshold).
    ExpiringSoon,
    /// Unknown fields were found in the Claim 169 data.
    ///
    /// This supports forward compatibility - new fields added to the spec
    /// won't break older decoders. The unknown fields are preserved in
    /// `Claim169::unknown_fields`.
    UnknownFields,
    /// Timestamp validation was explicitly disabled via options.
    TimestampValidationSkipped,
    /// Biometric data parsing was skipped via options.
    BiometricsSkipped,
    /// Non-standard compression was detected during decoding or used during encoding.
    ///
    /// The Claim 169 spec mandates zlib compression. This warning indicates
    /// that a different compression format (brotli, none) was used.
    NonStandardCompression,
}

/// Metadata extracted from a credential without full verification or decoding.
///
/// Useful for determining which key to use before calling `Decoder::decode()`.
/// This allows verifiers in multi-issuer scenarios to:
/// 1. Inspect the credential to find the issuer and key ID
/// 2. Look up the correct verification key
/// 3. Perform full decoding with the appropriate key
///
/// # Example
///
/// ```rust,ignore
/// use claim169_core::inspect;
///
/// let info = inspect(qr_text)?;
/// println!("Issuer: {:?}, Key ID: {:?}", info.issuer, info.key_id);
///
/// // Use the metadata to select the right verification key
/// let public_key = key_store.get(&info.issuer, &info.key_id);
/// let result = Decoder::new(qr_text)
///     .verify_with_ed25519(&public_key)?
///     .decode()?;
/// ```
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct InspectResult {
    /// Issuer from CWT claims (claim key 1).
    pub issuer: Option<String>,
    /// Subject from CWT claims (claim key 2).
    pub subject: Option<String>,
    /// Key ID from the COSE header.
    pub key_id: Option<Vec<u8>>,
    /// COSE algorithm declared in the protected header.
    pub algorithm: Option<coset::iana::Algorithm>,
    /// X.509 certificate headers from the COSE structure.
    pub x509_headers: X509Headers,
    /// Expiration time from CWT claims (Unix epoch seconds).
    pub expires_at: Option<i64>,
    /// COSE structure type (Sign1 or Encrypt0).
    pub cose_type: pipeline::CoseType,
}

/// Inspect credential metadata without full decoding or verification.
///
/// Runs Base45 → decompress → COSE header parse → CWT parse, but skips
/// signature verification. For encrypted credentials (COSE_Encrypt0), only
/// the outer COSE headers are accessible since the CWT payload is encrypted;
/// CWT-level fields (issuer, subject, expires_at) will be `None`.
///
/// This is useful for multi-issuer or key-rotation scenarios where you need
/// to determine which verification key to use before decoding.
pub fn inspect(qr_text: &str) -> error::Result<InspectResult> {
    let compressed = pipeline::base45_decode(qr_text)?;
    let (cose_bytes, _) =
        pipeline::decompress(&compressed, decode::DEFAULT_MAX_DECOMPRESSED_BYTES)?;

    // Try full path: parse COSE headers + CWT payload (works for Sign1)
    match pipeline::cose_parse(&cose_bytes, None, None) {
        Ok(cose_result) => {
            let cwt_result = pipeline::cwt_parse(&cose_result.payload)?;
            Ok(InspectResult {
                issuer: cwt_result.meta.issuer,
                subject: cwt_result.meta.subject,
                key_id: cose_result.key_id,
                algorithm: cose_result.algorithm,
                x509_headers: cose_result.x509_headers,
                expires_at: cwt_result.meta.expires_at,
                cose_type: pipeline::CoseType::Sign1,
            })
        }
        Err(Claim169Error::DecryptionFailed(_) | Claim169Error::UnsupportedCoseType(_)) => {
            // Expected for Encrypt0 payloads or unsupported COSE types -
            // fall back to header-only inspection
            let inspect_result = pipeline::cose_inspect(&cose_bytes)?;
            Ok(InspectResult {
                issuer: None,
                subject: None,
                key_id: inspect_result.key_id,
                algorithm: inspect_result.algorithm,
                x509_headers: inspect_result.x509_headers,
                expires_at: None,
                cose_type: inspect_result.cose_type,
            })
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warning_code_equality() {
        assert_eq!(WarningCode::ExpiringSoon, WarningCode::ExpiringSoon);
        assert_ne!(WarningCode::ExpiringSoon, WarningCode::UnknownFields);
    }

    #[test]
    fn test_warning_clone() {
        let warning = Warning {
            code: WarningCode::BiometricsSkipped,
            message: "Test warning".to_string(),
        };
        let cloned = warning.clone();
        assert_eq!(cloned.code, warning.code);
        assert_eq!(cloned.message, warning.message);
    }

    #[cfg(feature = "software-crypto")]
    #[test]
    fn test_inspect_returns_issuer_kid_algorithm() {
        use crypto::software::Ed25519Signer;

        let claim = model::Claim169::minimal("inspect-test", "Test User");
        let cwt = model::CwtMeta::new()
            .with_issuer("https://inspect.issuer.io")
            .with_subject("subject-456")
            .with_expires_at(1900000000);

        let mut signer = Ed25519Signer::generate();
        signer.set_key_id(b"inspect-key-1".to_vec());

        let qr_data = Encoder::new(claim, cwt)
            .sign_with(signer, coset::iana::Algorithm::EdDSA)
            .encode()
            .unwrap()
            .qr_data;

        let info = inspect(&qr_data).unwrap();

        assert_eq!(info.issuer.as_deref(), Some("https://inspect.issuer.io"));
        assert_eq!(info.subject.as_deref(), Some("subject-456"));
        assert_eq!(info.key_id, Some(b"inspect-key-1".to_vec()));
        assert_eq!(info.algorithm, Some(coset::iana::Algorithm::EdDSA));
        assert_eq!(info.expires_at, Some(1900000000));
        assert_eq!(info.cose_type, pipeline::CoseType::Sign1);
    }

    #[test]
    fn test_inspect_unsigned_credential() {
        let claim = model::Claim169::minimal("unsigned-inspect", "Unsigned User");
        let cwt = model::CwtMeta::new()
            .with_issuer("https://unsigned.issuer")
            .with_expires_at(i64::MAX);

        let qr_data = Encoder::new(claim, cwt)
            .allow_unsigned()
            .encode()
            .unwrap()
            .qr_data;

        let info = inspect(&qr_data).unwrap();

        assert_eq!(info.issuer.as_deref(), Some("https://unsigned.issuer"));
        assert_eq!(info.key_id, None);
        assert_eq!(info.cose_type, pipeline::CoseType::Sign1);
    }

    #[cfg(feature = "software-crypto")]
    #[test]
    fn test_inspect_encrypted_credential_returns_header_info() {
        use crypto::software::Ed25519Signer;

        let claim = model::Claim169::minimal("encrypted-inspect", "Encrypted User");
        let cwt = model::CwtMeta::new()
            .with_issuer("https://encrypted.issuer")
            .with_expires_at(i64::MAX);

        let signer = Ed25519Signer::generate();

        // Generate AES-256 key (32 bytes)
        let aes_key = [0xABu8; 32];

        let qr_data = Encoder::new(claim, cwt)
            .sign_with(signer, coset::iana::Algorithm::EdDSA)
            .encrypt_with_aes256(&aes_key)
            .unwrap()
            .encode()
            .unwrap()
            .qr_data;

        let info = inspect(&qr_data).unwrap();

        // For encrypted credentials, CWT-level fields are not accessible
        assert_eq!(info.cose_type, pipeline::CoseType::Encrypt0);
        // Algorithm should be the encryption algorithm
        assert!(info.algorithm.is_some());
    }

    #[test]
    fn test_inspect_invalid_base45() {
        let result = inspect("!!!INVALID_BASE45!!!");
        assert!(result.is_err());
    }
}
