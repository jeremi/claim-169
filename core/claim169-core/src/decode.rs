//! Decoder builder for parsing Claim 169 QR codes.
//!
//! The [`Decoder`] provides a fluent builder API for decoding identity credentials
//! from QR-scanned Base45 strings.
//!
//! # Basic Usage
//!
//! ```rust,ignore
//! use claim169_core::Decoder;
//!
//! // Decode without verification (testing only)
//! let result = Decoder::new(qr_text)
//!     .allow_unverified()
//!     .decode()?;
//!
//! // Decode with Ed25519 verification
//! let result = Decoder::new(qr_text)
//!     .verify_with_ed25519(&public_key)?
//!     .decode()?;
//!
//! // Decode encrypted payload
//! let result = Decoder::new(qr_text)
//!     .decrypt_with_aes256(&aes_key)?
//!     .verify_with_ed25519(&public_key)?
//!     .decode()?;
//! ```
//!
//! # HSM Integration
//!
//! For hardware security modules, use the generic `verify_with()` method:
//!
//! ```rust,ignore
//! use claim169_core::{Decoder, SignatureVerifier};
//!
//! struct HsmVerifier { /* ... */ }
//! impl SignatureVerifier for HsmVerifier { /* ... */ }
//!
//! let result = Decoder::new(qr_text)
//!     .verify_with(hsm_verifier)
//!     .decode()?;
//! ```

use crate::crypto::traits::{Decryptor, SignatureVerifier};
use crate::error::{Claim169Error, Result};
use crate::model::VerificationStatus;
use crate::pipeline;
use crate::{DecodeResult, Warning, WarningCode};

#[cfg(feature = "software-crypto")]
use crate::crypto::software::{AesGcmDecryptor, EcdsaP256Verifier, Ed25519Verifier};

use std::time::{SystemTime, UNIX_EPOCH};

/// Default maximum decompressed size (64KB)
const DEFAULT_MAX_DECOMPRESSED_BYTES: usize = 65536;

/// Builder for decoding Claim 169 credentials from QR strings.
///
/// The decoder follows a builder pattern where configuration methods return `Self`
/// and the final `decode()` method consumes the builder to produce the result.
///
/// # Operation Order
///
/// When both decryption and verification are configured, the credential is always
/// **decrypted first, then verified** (reverse of encoding), regardless of the order
/// in which builder methods are called.
///
/// # Security
///
/// - By default, decoding requires signature verification
/// - Use [`allow_unverified()`](Self::allow_unverified) to explicitly opt out
/// - Decompression is limited to prevent zip bomb attacks (default: 64KB)
///
/// # Example
///
/// ```rust,ignore
/// use claim169_core::Decoder;
///
/// let result = Decoder::new("6BF5YZB2...")
///     .verify_with_ed25519(&public_key)?
///     .decode()?;
///
/// println!("ID: {:?}", result.claim169.id);
/// println!("Verified: {:?}", result.verification_status);
/// ```
pub struct Decoder {
    qr_text: String,
    verifier: Option<Box<dyn SignatureVerifier + Send + Sync>>,
    decryptor: Option<Box<dyn Decryptor + Send + Sync>>,
    allow_unverified: bool,
    skip_biometrics: bool,
    validate_timestamps: bool,
    clock_skew_tolerance_seconds: i64,
    max_decompressed_bytes: usize,
}

impl Decoder {
    /// Create a new decoder with the given QR text.
    ///
    /// # Arguments
    ///
    /// * `qr_text` - The Base45-encoded QR code content (accepts `&str` or `String`)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let decoder = Decoder::new("6BF5YZB2...");
    /// let decoder = Decoder::new(qr_string);
    /// ```
    pub fn new(qr_text: impl Into<String>) -> Self {
        Self {
            qr_text: qr_text.into(),
            verifier: None,
            decryptor: None,
            allow_unverified: false,
            skip_biometrics: false,
            validate_timestamps: true,
            clock_skew_tolerance_seconds: 0,
            max_decompressed_bytes: DEFAULT_MAX_DECOMPRESSED_BYTES,
        }
    }

    /// Verify with a custom verifier implementation.
    ///
    /// Use this method for HSM integration or custom cryptographic backends.
    ///
    /// # Arguments
    ///
    /// * `verifier` - A type implementing the [`SignatureVerifier`] trait
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let decoder = Decoder::new(qr_text)
    ///     .verify_with(hsm_verifier);
    /// ```
    pub fn verify_with<V: SignatureVerifier + 'static>(mut self, verifier: V) -> Self {
        self.verifier = Some(Box::new(verifier));
        self
    }

    /// Verify with an Ed25519 public key.
    ///
    /// The key is validated immediately and an error is returned if invalid.
    ///
    /// # Arguments
    ///
    /// * `public_key` - 32-byte Ed25519 public key
    ///
    /// # Errors
    ///
    /// Returns an error if the public key is invalid or represents a weak key.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let decoder = Decoder::new(qr_text)
    ///     .verify_with_ed25519(&public_key)?;
    /// ```
    #[cfg(feature = "software-crypto")]
    pub fn verify_with_ed25519(self, public_key: &[u8]) -> Result<Self> {
        let verifier = Ed25519Verifier::from_bytes(public_key)
            .map_err(|e| Claim169Error::Crypto(e.to_string()))?;

        Ok(self.verify_with(verifier))
    }

    /// Verify with an Ed25519 public key in PEM format.
    ///
    /// Supports SPKI format with "BEGIN PUBLIC KEY" headers.
    ///
    /// # Arguments
    ///
    /// * `pem` - PEM-encoded Ed25519 public key
    ///
    /// # Errors
    ///
    /// Returns an error if the PEM is invalid or the key is weak.
    #[cfg(feature = "software-crypto")]
    pub fn verify_with_ed25519_pem(self, pem: &str) -> Result<Self> {
        let verifier =
            Ed25519Verifier::from_pem(pem).map_err(|e| Claim169Error::Crypto(e.to_string()))?;

        Ok(self.verify_with(verifier))
    }

    /// Verify with an ECDSA P-256 public key.
    ///
    /// The key is validated immediately and an error is returned if invalid.
    ///
    /// # Arguments
    ///
    /// * `public_key` - SEC1-encoded P-256 public key (33 or 65 bytes)
    ///
    /// # Errors
    ///
    /// Returns an error if the public key format is invalid or represents a weak key.
    #[cfg(feature = "software-crypto")]
    pub fn verify_with_ecdsa_p256(self, public_key: &[u8]) -> Result<Self> {
        let verifier = EcdsaP256Verifier::from_sec1_bytes(public_key)
            .map_err(|e| Claim169Error::Crypto(e.to_string()))?;

        Ok(self.verify_with(verifier))
    }

    /// Verify with an ECDSA P-256 public key in PEM format.
    ///
    /// Supports SPKI format with "BEGIN PUBLIC KEY" headers.
    ///
    /// # Arguments
    ///
    /// * `pem` - PEM-encoded P-256 public key
    ///
    /// # Errors
    ///
    /// Returns an error if the PEM is invalid or the key is weak.
    #[cfg(feature = "software-crypto")]
    pub fn verify_with_ecdsa_p256_pem(self, pem: &str) -> Result<Self> {
        let verifier =
            EcdsaP256Verifier::from_pem(pem).map_err(|e| Claim169Error::Crypto(e.to_string()))?;

        Ok(self.verify_with(verifier))
    }

    /// Decrypt with a custom decryptor implementation.
    ///
    /// Use this method for HSM integration or custom cryptographic backends.
    ///
    /// # Arguments
    ///
    /// * `decryptor` - A type implementing the [`Decryptor`] trait
    pub fn decrypt_with<D: Decryptor + 'static>(mut self, decryptor: D) -> Self {
        self.decryptor = Some(Box::new(decryptor));
        self
    }

    /// Decrypt with AES-256-GCM.
    ///
    /// # Arguments
    ///
    /// * `key` - 32-byte AES-256 decryption key
    ///
    /// # Errors
    ///
    /// Returns an error if the key is not exactly 32 bytes.
    #[cfg(feature = "software-crypto")]
    pub fn decrypt_with_aes256(self, key: &[u8]) -> Result<Self> {
        let decryptor =
            AesGcmDecryptor::aes256(key).map_err(|e| Claim169Error::Crypto(e.to_string()))?;

        Ok(self.decrypt_with(decryptor))
    }

    /// Decrypt with AES-128-GCM.
    ///
    /// # Arguments
    ///
    /// * `key` - 16-byte AES-128 decryption key
    ///
    /// # Errors
    ///
    /// Returns an error if the key is not exactly 16 bytes.
    #[cfg(feature = "software-crypto")]
    pub fn decrypt_with_aes128(self, key: &[u8]) -> Result<Self> {
        let decryptor =
            AesGcmDecryptor::aes128(key).map_err(|e| Claim169Error::Crypto(e.to_string()))?;

        Ok(self.decrypt_with(decryptor))
    }

    /// Allow decoding without signature verification.
    ///
    /// **Security Warning**: Unverified credentials cannot be trusted for authenticity.
    /// Only use this for testing or when verification is handled externally.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let result = Decoder::new(qr_text)
    ///     .allow_unverified()
    ///     .decode()?;
    /// ```
    pub fn allow_unverified(mut self) -> Self {
        self.allow_unverified = true;
        self
    }

    /// Skip biometric fields during decoding.
    ///
    /// This speeds up decoding by not parsing fingerprint, iris, face,
    /// palm, and voice biometric data.
    pub fn skip_biometrics(mut self) -> Self {
        self.skip_biometrics = true;
        self
    }

    /// Disable timestamp validation.
    ///
    /// By default, the decoder validates that:
    /// - The credential has not expired (`exp` claim)
    /// - The credential is valid for use (`nbf` claim)
    ///
    /// Use this method to skip these checks (e.g., for offline scenarios).
    pub fn without_timestamp_validation(mut self) -> Self {
        self.validate_timestamps = false;
        self
    }

    /// Set the clock skew tolerance for timestamp validation.
    ///
    /// This allows for some difference between the system clock and the
    /// issuer's clock when validating `exp` and `nbf` claims.
    ///
    /// # Arguments
    ///
    /// * `seconds` - Number of seconds to tolerate (default: 0)
    pub fn clock_skew_tolerance(mut self, seconds: i64) -> Self {
        self.clock_skew_tolerance_seconds = seconds;
        self
    }

    /// Set the maximum decompressed size.
    ///
    /// This protects against zip bomb attacks by limiting how much data
    /// can be decompressed from the QR payload.
    ///
    /// # Arguments
    ///
    /// * `bytes` - Maximum decompressed size in bytes (default: 65536)
    pub fn max_decompressed_bytes(mut self, bytes: usize) -> Self {
        self.max_decompressed_bytes = bytes;
        self
    }

    /// Decode the QR text and return the result.
    ///
    /// This method consumes the decoder and performs the full decoding pipeline:
    ///
    /// ```text
    /// Base45 → zlib → COSE_Encrypt0 → COSE_Sign1 → CWT → Claim169
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Neither a verifier nor `allow_unverified()` was configured
    /// - Base45 decoding fails
    /// - Decompression fails or exceeds the limit
    /// - COSE structure is invalid
    /// - Signature verification fails
    /// - Decryption fails
    /// - CWT parsing fails
    /// - Timestamp validation fails
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let result = Decoder::new(qr_text)
    ///     .verify_with_ed25519(&public_key)?
    ///     .decode()?;
    ///
    /// println!("Name: {:?}", result.claim169.full_name);
    /// ```
    pub fn decode(self) -> Result<DecodeResult> {
        let mut warnings = Vec::new();

        // Convert trait objects for pipeline functions
        let verifier_ref: Option<&dyn SignatureVerifier> = self
            .verifier
            .as_ref()
            .map(|v| v.as_ref() as &dyn SignatureVerifier);
        let decryptor_ref: Option<&dyn Decryptor> = self
            .decryptor
            .as_ref()
            .map(|d| d.as_ref() as &dyn Decryptor);

        // Step 1: Base45 decode
        let compressed = pipeline::base45_decode(&self.qr_text)?;

        // Step 2: zlib decompress
        let cose_bytes = pipeline::decompress(&compressed, self.max_decompressed_bytes)?;

        // Step 3-4: Parse COSE and verify/decrypt
        let cose_result = pipeline::cose_parse(&cose_bytes, verifier_ref, decryptor_ref)?;

        // Check if verification was required but skipped
        if !self.allow_unverified && cose_result.verification_status == VerificationStatus::Skipped
        {
            return Err(Claim169Error::DecodingConfig(
                "verification required but no verifier provided - use allow_unverified() to skip"
                    .to_string(),
            ));
        }

        // Check if verification failed
        if cose_result.verification_status == VerificationStatus::Failed {
            return Err(Claim169Error::SignatureInvalid(
                "signature verification failed".to_string(),
            ));
        }

        // Step 5: Parse CWT
        let cwt_result = pipeline::cwt_parse(&cose_result.payload)?;

        // Step 6: Validate timestamps
        if self.validate_timestamps {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);

            let skew = self.clock_skew_tolerance_seconds;

            if let Some(exp) = cwt_result.meta.expires_at {
                if now > exp + skew {
                    return Err(Claim169Error::Expired(exp));
                }
            }

            if let Some(nbf) = cwt_result.meta.not_before {
                if now + skew < nbf {
                    return Err(Claim169Error::NotYetValid(nbf));
                }
            }
        } else {
            warnings.push(Warning {
                code: WarningCode::TimestampValidationSkipped,
                message: "Timestamp validation was disabled".to_string(),
            });
        }

        // Step 7: Transform claim 169
        let claim169 = pipeline::claim169_transform(cwt_result.claim_169, self.skip_biometrics)?;

        if self.skip_biometrics {
            warnings.push(Warning {
                code: WarningCode::BiometricsSkipped,
                message: "Biometric data was skipped".to_string(),
            });
        }

        if !claim169.unknown_fields.is_empty() {
            warnings.push(Warning {
                code: WarningCode::UnknownFields,
                message: format!(
                    "Found {} unknown fields (keys: {:?})",
                    claim169.unknown_fields.len(),
                    claim169.unknown_fields.keys().collect::<Vec<_>>()
                ),
            });
        }

        Ok(DecodeResult {
            claim169,
            cwt_meta: cwt_result.meta,
            verification_status: cose_result.verification_status,
            warnings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Claim169, CwtMeta, VerificationStatus};

    // Create a minimal test QR payload (unsigned, for testing basic functionality)
    fn create_test_qr() -> String {
        use crate::Encoder;

        let claim169 = Claim169 {
            id: Some("test-123".to_string()),
            full_name: Some("Test User".to_string()),
            ..Default::default()
        };

        let cwt_meta = CwtMeta::new()
            .with_issuer("https://test.issuer")
            .with_expires_at(i64::MAX); // Far future expiration

        Encoder::new(claim169, cwt_meta)
            .allow_unsigned()
            .encode()
            .unwrap()
    }

    #[test]
    fn test_decoder_requires_verifier_or_allow_unverified() {
        let qr_text = create_test_qr();

        let result = Decoder::new(&qr_text).decode();

        assert!(result.is_err());
        match result.unwrap_err() {
            Claim169Error::DecodingConfig(msg) => {
                assert!(msg.contains("allow_unverified"));
            }
            e => panic!("Expected DecodingConfig error, got: {:?}", e),
        }
    }

    #[test]
    fn test_decoder_allow_unverified() {
        let qr_text = create_test_qr();

        let result = Decoder::new(&qr_text).allow_unverified().decode();

        assert!(result.is_ok());
        let decoded = result.unwrap();
        assert_eq!(decoded.claim169.id, Some("test-123".to_string()));
        assert_eq!(decoded.claim169.full_name, Some("Test User".to_string()));
        assert_eq!(decoded.verification_status, VerificationStatus::Skipped);
    }

    #[test]
    fn test_decoder_accepts_string() {
        let qr_text = create_test_qr();

        // Test with String
        let result = Decoder::new(qr_text.clone()).allow_unverified().decode();
        assert!(result.is_ok());

        // Test with &str
        let result = Decoder::new(&qr_text).allow_unverified().decode();
        assert!(result.is_ok());
    }

    #[test]
    fn test_decoder_skip_biometrics() {
        let qr_text = create_test_qr();

        let result = Decoder::new(&qr_text)
            .allow_unverified()
            .skip_biometrics()
            .decode()
            .unwrap();

        // Should have a warning about skipped biometrics
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == WarningCode::BiometricsSkipped));
    }

    #[test]
    fn test_decoder_without_timestamp_validation() {
        let qr_text = create_test_qr();

        let result = Decoder::new(&qr_text)
            .allow_unverified()
            .without_timestamp_validation()
            .decode()
            .unwrap();

        // Should have a warning about skipped validation
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == WarningCode::TimestampValidationSkipped));
    }

    #[cfg(feature = "software-crypto")]
    #[test]
    fn test_decoder_roundtrip_ed25519() {
        use crate::crypto::software::Ed25519Signer;
        use crate::Encoder;
        use coset::iana;

        let claim169 = Claim169 {
            id: Some("signed-test".to_string()),
            full_name: Some("Signed User".to_string()),
            email: Some("signed@example.com".to_string()),
            ..Default::default()
        };

        let cwt_meta = CwtMeta::new()
            .with_issuer("https://signed.test")
            .with_expires_at(i64::MAX);

        // Generate key pair
        let signer = Ed25519Signer::generate();
        let public_key = signer.public_key_bytes();

        // Encode
        let qr_data = Encoder::new(claim169.clone(), cwt_meta)
            .sign_with(signer, iana::Algorithm::EdDSA)
            .encode()
            .unwrap();

        // Decode with verification
        let result = Decoder::new(&qr_data)
            .verify_with_ed25519(&public_key)
            .unwrap()
            .decode()
            .unwrap();

        assert_eq!(result.verification_status, VerificationStatus::Verified);
        assert_eq!(result.claim169.id, claim169.id);
        assert_eq!(result.claim169.full_name, claim169.full_name);
        assert_eq!(result.claim169.email, claim169.email);
    }

    #[cfg(feature = "software-crypto")]
    #[test]
    fn test_decoder_roundtrip_encrypted() {
        use crate::crypto::software::Ed25519Signer;
        use crate::Encoder;
        use coset::iana;

        let claim169 = Claim169 {
            id: Some("encrypted-test".to_string()),
            full_name: Some("Encrypted User".to_string()),
            ..Default::default()
        };

        let cwt_meta = CwtMeta::new()
            .with_issuer("https://encrypted.test")
            .with_expires_at(i64::MAX);

        // Generate keys
        let signer = Ed25519Signer::generate();
        let public_key = signer.public_key_bytes();
        let aes_key = [42u8; 32];
        let nonce = [7u8; 12];

        // Encode with signing and encryption
        let qr_data = Encoder::new(claim169.clone(), cwt_meta)
            .sign_with(signer, iana::Algorithm::EdDSA)
            .encrypt_with_aes256_nonce(&aes_key, &nonce)
            .unwrap()
            .encode()
            .unwrap();

        // Decode with decryption and verification
        let result = Decoder::new(&qr_data)
            .decrypt_with_aes256(&aes_key)
            .unwrap()
            .verify_with_ed25519(&public_key)
            .unwrap()
            .decode()
            .unwrap();

        assert_eq!(result.verification_status, VerificationStatus::Verified);
        assert_eq!(result.claim169.id, claim169.id);
        assert_eq!(result.claim169.full_name, claim169.full_name);
    }

    #[cfg(feature = "software-crypto")]
    #[test]
    fn test_decoder_wrong_key_fails() {
        use crate::crypto::software::Ed25519Signer;
        use crate::Encoder;
        use coset::iana;

        let claim169 = Claim169::minimal("test", "Test");
        let cwt_meta = CwtMeta::default();

        let signer = Ed25519Signer::generate();
        let wrong_signer = Ed25519Signer::generate();
        let wrong_public_key = wrong_signer.public_key_bytes();

        let qr_data = Encoder::new(claim169, cwt_meta)
            .sign_with(signer, iana::Algorithm::EdDSA)
            .encode()
            .unwrap();

        // Try to decode with wrong key
        let result = Decoder::new(&qr_data)
            .verify_with_ed25519(&wrong_public_key)
            .unwrap()
            .decode();

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Claim169Error::SignatureInvalid(_)
        ));
    }

    #[test]
    fn test_decoder_invalid_base45() {
        let result = Decoder::new("!!!invalid base45!!!")
            .allow_unverified()
            .decode();

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Claim169Error::Base45Decode(_)
        ));
    }

    #[test]
    fn test_decoder_max_decompressed_bytes() {
        let qr_text = create_test_qr();

        // Set a very small limit that will be exceeded
        let result = Decoder::new(&qr_text)
            .allow_unverified()
            .max_decompressed_bytes(10)
            .decode();

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Claim169Error::DecompressLimitExceeded { .. }
        ));
    }
}
