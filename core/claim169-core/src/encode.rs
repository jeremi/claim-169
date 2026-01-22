//! Encoder builder for creating Claim 169 QR codes.
//!
//! The [`Encoder`] provides a fluent builder API for encoding identity credentials
//! into QR-ready Base45 strings.
//!
//! # Basic Usage
//!
//! ```rust,ignore
//! use claim169_core::{Encoder, Claim169, CwtMeta};
//!
//! // Create an unsigned credential (requires explicit opt-in)
//! let qr_data = Encoder::new(claim169, cwt_meta)
//!     .allow_unsigned()
//!     .encode()?;
//!
//! // Create a signed credential with Ed25519
//! let qr_data = Encoder::new(claim169, cwt_meta)
//!     .sign_with_ed25519(&private_key)?
//!     .encode()?;
//!
//! // Create a signed and encrypted credential
//! let qr_data = Encoder::new(claim169, cwt_meta)
//!     .sign_with_ed25519(&private_key)?
//!     .encrypt_with_aes256(&aes_key)?
//!     .encode()?;
//! ```
//!
//! # HSM Integration
//!
//! For hardware security modules, use the generic `sign_with()` method:
//!
//! ```rust,ignore
//! use claim169_core::{Encoder, Signer};
//!
//! struct HsmSigner { /* ... */ }
//! impl Signer for HsmSigner { /* ... */ }
//!
//! let qr_data = Encoder::new(claim169, cwt_meta)
//!     .sign_with(hsm_signer, iana::Algorithm::EdDSA)
//!     .encode()?;
//! ```

use coset::iana;

use crate::crypto::traits::{Encryptor, Signer};
use crate::error::{Claim169Error, Result};
use crate::model::{Claim169, CwtMeta};
use crate::pipeline::encode::{encode_signed, encode_signed_and_encrypted, EncodeConfig};

#[cfg(feature = "software-crypto")]
use crate::crypto::software::AesGcmEncryptor;

/// Configuration for encryption in the encoder
struct EncryptConfig {
    encryptor: Box<dyn Encryptor + Send + Sync>,
    algorithm: iana::Algorithm,
    nonce: Option<[u8; 12]>,
}

/// Builder for encoding Claim 169 credentials into QR-ready strings.
///
/// The encoder follows a builder pattern where configuration methods return `Self`
/// and the final `encode()` method consumes the builder to produce the result.
///
/// # Operation Order
///
/// When both signing and encryption are configured, the credential is always
/// **signed first, then encrypted** (sign-then-encrypt), regardless of the order
/// in which builder methods are called.
///
/// # Security
///
/// - Unsigned encoding requires explicit opt-in via [`allow_unsigned()`](Self::allow_unsigned)
/// - Nonces are generated randomly by default for encryption
/// - Use explicit nonce methods only for testing or deterministic scenarios
///
/// # Example
///
/// ```rust,ignore
/// use claim169_core::{Encoder, Claim169, CwtMeta};
///
/// let claim169 = Claim169::minimal("ID-001", "Jane Doe");
/// let cwt_meta = CwtMeta::new()
///     .with_issuer("https://issuer.example.com")
///     .with_expires_at(1800000000);
///
/// // Sign with Ed25519
/// let qr_data = Encoder::new(claim169, cwt_meta)
///     .sign_with_ed25519(&private_key)?
///     .encode()?;
/// ```
pub struct Encoder {
    claim169: Claim169,
    cwt_meta: CwtMeta,
    signer: Option<Box<dyn Signer + Send + Sync>>,
    sign_algorithm: Option<iana::Algorithm>,
    encrypt_config: Option<EncryptConfig>,
    allow_unsigned: bool,
    skip_biometrics: bool,
}

impl Encoder {
    /// Create a new encoder with the given claim and CWT metadata.
    ///
    /// # Arguments
    ///
    /// * `claim169` - The identity claim data to encode
    /// * `cwt_meta` - CWT metadata including issuer, expiration, etc.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let encoder = Encoder::new(claim169, cwt_meta);
    /// ```
    pub fn new(claim169: Claim169, cwt_meta: CwtMeta) -> Self {
        Self {
            claim169,
            cwt_meta,
            signer: None,
            sign_algorithm: None,
            encrypt_config: None,
            allow_unsigned: false,
            skip_biometrics: false,
        }
    }

    /// Sign with a custom signer implementation.
    ///
    /// Use this method for HSM integration or custom cryptographic backends.
    ///
    /// # Arguments
    ///
    /// * `signer` - A type implementing the [`Signer`] trait
    /// * `algorithm` - The COSE algorithm to use for signing
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let encoder = Encoder::new(claim169, cwt_meta)
    ///     .sign_with(hsm_signer, iana::Algorithm::EdDSA);
    /// ```
    pub fn sign_with<S: Signer + 'static>(mut self, signer: S, algorithm: iana::Algorithm) -> Self {
        self.signer = Some(Box::new(signer));
        self.sign_algorithm = Some(algorithm);
        self
    }

    /// Sign with an Ed25519 private key.
    ///
    /// The key is validated immediately and an error is returned if invalid.
    ///
    /// # Arguments
    ///
    /// * `private_key` - 32-byte Ed25519 private key
    ///
    /// # Errors
    ///
    /// Returns an error if the private key is not exactly 32 bytes.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let encoder = Encoder::new(claim169, cwt_meta)
    ///     .sign_with_ed25519(&private_key)?;
    /// ```
    #[cfg(feature = "software-crypto")]
    pub fn sign_with_ed25519(self, private_key: &[u8]) -> Result<Self> {
        use crate::crypto::software::Ed25519Signer;

        let signer = Ed25519Signer::from_bytes(private_key)
            .map_err(|e| Claim169Error::Crypto(e.to_string()))?;

        Ok(self.sign_with(signer, iana::Algorithm::EdDSA))
    }

    /// Sign with an ECDSA P-256 private key.
    ///
    /// The key is validated immediately and an error is returned if invalid.
    ///
    /// # Arguments
    ///
    /// * `private_key` - 32-byte ECDSA P-256 private key (scalar)
    ///
    /// # Errors
    ///
    /// Returns an error if the private key format is invalid.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let encoder = Encoder::new(claim169, cwt_meta)
    ///     .sign_with_ecdsa_p256(&private_key)?;
    /// ```
    #[cfg(feature = "software-crypto")]
    pub fn sign_with_ecdsa_p256(self, private_key: &[u8]) -> Result<Self> {
        use crate::crypto::software::EcdsaP256Signer;

        let signer = EcdsaP256Signer::from_bytes(private_key)
            .map_err(|e| Claim169Error::Crypto(e.to_string()))?;

        Ok(self.sign_with(signer, iana::Algorithm::ES256))
    }

    /// Encrypt with a custom encryptor implementation.
    ///
    /// Use this method for HSM integration or custom cryptographic backends.
    /// A random 12-byte nonce is generated automatically.
    ///
    /// # Arguments
    ///
    /// * `encryptor` - A type implementing the [`Encryptor`] trait
    /// * `algorithm` - The COSE algorithm to use for encryption
    pub fn encrypt_with<E: Encryptor + 'static>(
        mut self,
        encryptor: E,
        algorithm: iana::Algorithm,
    ) -> Self {
        self.encrypt_config = Some(EncryptConfig {
            encryptor: Box::new(encryptor),
            algorithm,
            nonce: None, // Generate random nonce during encode
        });
        self
    }

    /// Encrypt with AES-256-GCM.
    ///
    /// A random 12-byte nonce is generated automatically during encoding.
    ///
    /// # Arguments
    ///
    /// * `key` - 32-byte AES-256 encryption key
    ///
    /// # Errors
    ///
    /// Returns an error if the key is not exactly 32 bytes.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let encoder = Encoder::new(claim169, cwt_meta)
    ///     .sign_with_ed25519(&sign_key)?
    ///     .encrypt_with_aes256(&aes_key)?;
    /// ```
    #[cfg(feature = "software-crypto")]
    pub fn encrypt_with_aes256(self, key: &[u8]) -> Result<Self> {
        let encryptor =
            AesGcmEncryptor::aes256(key).map_err(|e| Claim169Error::Crypto(e.to_string()))?;

        Ok(self.encrypt_with(encryptor, iana::Algorithm::A256GCM))
    }

    /// Encrypt with AES-128-GCM.
    ///
    /// A random 12-byte nonce is generated automatically during encoding.
    ///
    /// # Arguments
    ///
    /// * `key` - 16-byte AES-128 encryption key
    ///
    /// # Errors
    ///
    /// Returns an error if the key is not exactly 16 bytes.
    #[cfg(feature = "software-crypto")]
    pub fn encrypt_with_aes128(self, key: &[u8]) -> Result<Self> {
        let encryptor =
            AesGcmEncryptor::aes128(key).map_err(|e| Claim169Error::Crypto(e.to_string()))?;

        Ok(self.encrypt_with(encryptor, iana::Algorithm::A128GCM))
    }

    /// Encrypt with AES-256-GCM using an explicit nonce.
    ///
    /// **Warning**: Only use this for testing or deterministic scenarios.
    /// Reusing nonces with the same key is a critical security vulnerability.
    ///
    /// # Arguments
    ///
    /// * `key` - 32-byte AES-256 encryption key
    /// * `nonce` - 12-byte nonce/IV (must be unique per encryption)
    #[cfg(feature = "software-crypto")]
    pub fn encrypt_with_aes256_nonce(mut self, key: &[u8], nonce: &[u8; 12]) -> Result<Self> {
        let encryptor =
            AesGcmEncryptor::aes256(key).map_err(|e| Claim169Error::Crypto(e.to_string()))?;

        self.encrypt_config = Some(EncryptConfig {
            encryptor: Box::new(encryptor),
            algorithm: iana::Algorithm::A256GCM,
            nonce: Some(*nonce),
        });
        Ok(self)
    }

    /// Encrypt with AES-128-GCM using an explicit nonce.
    ///
    /// **Warning**: Only use this for testing or deterministic scenarios.
    /// Reusing nonces with the same key is a critical security vulnerability.
    ///
    /// # Arguments
    ///
    /// * `key` - 16-byte AES-128 encryption key
    /// * `nonce` - 12-byte nonce/IV (must be unique per encryption)
    #[cfg(feature = "software-crypto")]
    pub fn encrypt_with_aes128_nonce(mut self, key: &[u8], nonce: &[u8; 12]) -> Result<Self> {
        let encryptor =
            AesGcmEncryptor::aes128(key).map_err(|e| Claim169Error::Crypto(e.to_string()))?;

        self.encrypt_config = Some(EncryptConfig {
            encryptor: Box::new(encryptor),
            algorithm: iana::Algorithm::A128GCM,
            nonce: Some(*nonce),
        });
        Ok(self)
    }

    /// Allow encoding without a signature.
    ///
    /// **Security Warning**: Unsigned credentials cannot be verified for authenticity.
    /// Only use this for testing or scenarios where signatures are not required.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let encoder = Encoder::new(claim169, cwt_meta)
    ///     .allow_unsigned()
    ///     .encode()?;
    /// ```
    pub fn allow_unsigned(mut self) -> Self {
        self.allow_unsigned = true;
        self
    }

    /// Skip biometric fields during encoding.
    ///
    /// This reduces the QR code size by excluding fingerprint, iris, face,
    /// palm, and voice biometric data.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let encoder = Encoder::new(claim169, cwt_meta)
    ///     .skip_biometrics()
    ///     .sign_with_ed25519(&key)?
    ///     .encode()?;
    /// ```
    pub fn skip_biometrics(mut self) -> Self {
        self.skip_biometrics = true;
        self
    }

    /// Encode the credential to a Base45 QR string.
    ///
    /// This method consumes the encoder and produces the final QR-ready string.
    ///
    /// # Pipeline
    ///
    /// ```text
    /// Claim169 → CBOR → CWT → COSE_Sign1 → [COSE_Encrypt0] → zlib → Base45
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Neither a signer nor `allow_unsigned()` was configured
    /// - Signing fails
    /// - Encryption fails
    /// - CBOR encoding fails
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let qr_data = Encoder::new(claim169, cwt_meta)
    ///     .sign_with_ed25519(&key)?
    ///     .encode()?;
    ///
    /// println!("QR content: {}", qr_data);
    /// ```
    pub fn encode(self) -> Result<String> {
        // Validate configuration
        if self.signer.is_none() && !self.allow_unsigned {
            return Err(Claim169Error::EncodingConfig(
                "either call sign_with_*() or allow_unsigned() before encode()".to_string(),
            ));
        }

        let config = EncodeConfig {
            skip_biometrics: self.skip_biometrics,
        };

        // Convert the boxed signer to a trait object reference
        // The cast is needed because Box<dyn Signer + Send + Sync> doesn't automatically
        // coerce to &dyn Signer, even though Signer: Send + Sync
        let signer_ref: Option<&dyn Signer> =
            self.signer.as_ref().map(|s| s.as_ref() as &dyn Signer);

        match self.encrypt_config {
            Some(encrypt_config) => {
                // Get nonce - auto-generate if software-crypto is enabled, otherwise require explicit
                #[cfg(feature = "software-crypto")]
                let nonce = encrypt_config.nonce.unwrap_or_else(generate_nonce);

                #[cfg(not(feature = "software-crypto"))]
                let nonce = encrypt_config.nonce.ok_or_else(|| {
                    Claim169Error::EncodingConfig(
                        "explicit nonce required when software-crypto feature is disabled"
                            .to_string(),
                    )
                })?;

                encode_signed_and_encrypted(
                    &self.claim169,
                    &self.cwt_meta,
                    signer_ref,
                    self.sign_algorithm,
                    encrypt_config.encryptor.as_ref(),
                    encrypt_config.algorithm,
                    &nonce,
                    &config,
                )
            }
            None => encode_signed(
                &self.claim169,
                &self.cwt_meta,
                signer_ref,
                self.sign_algorithm,
                &config,
            ),
        }
    }
}

/// Generate a random 12-byte nonce for AES-GCM encryption.
#[cfg(feature = "software-crypto")]
fn generate_nonce() -> [u8; 12] {
    use rand::RngCore;
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
}

/// Generate a random nonce.
///
/// Returns a 12-byte random nonce suitable for AES-GCM encryption.
///
/// # Example
///
/// ```rust
/// use claim169_core::generate_random_nonce;
///
/// let nonce = generate_random_nonce();
/// assert_eq!(nonce.len(), 12);
/// ```
#[cfg(feature = "software-crypto")]
pub fn generate_random_nonce() -> [u8; 12] {
    generate_nonce()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_requires_signer_or_allow_unsigned() {
        let claim169 = Claim169::minimal("test-id", "Test User");
        let cwt_meta = CwtMeta::default();

        let result = Encoder::new(claim169, cwt_meta).encode();

        assert!(result.is_err());
        match result.unwrap_err() {
            Claim169Error::EncodingConfig(msg) => {
                assert!(msg.contains("allow_unsigned"));
            }
            e => panic!("Expected EncodingConfig error, got: {:?}", e),
        }
    }

    #[test]
    fn test_encoder_unsigned() {
        let claim169 = Claim169::minimal("test-id", "Test User");
        let cwt_meta = CwtMeta::new().with_issuer("test-issuer");

        let result = Encoder::new(claim169, cwt_meta).allow_unsigned().encode();

        assert!(result.is_ok());
        let qr_data = result.unwrap();
        assert!(!qr_data.is_empty());
    }

    #[cfg(feature = "software-crypto")]
    #[test]
    fn test_encoder_ed25519_signed() {
        use crate::crypto::software::Ed25519Signer;

        let claim169 = Claim169::minimal("signed-test", "Signed User");
        let cwt_meta = CwtMeta::new()
            .with_issuer("https://test.issuer")
            .with_expires_at(1800000000);

        let private_key = [0u8; 32]; // For test - generate real key in production
        let test_signer = Ed25519Signer::from_bytes(&private_key).unwrap();

        let result = Encoder::new(claim169, cwt_meta)
            .sign_with(test_signer, iana::Algorithm::EdDSA)
            .encode();

        assert!(result.is_ok());
    }

    #[cfg(feature = "software-crypto")]
    #[test]
    fn test_encoder_ed25519_convenience() {
        let claim169 = Claim169::minimal("signed-test", "Signed User");
        let cwt_meta = CwtMeta::default();

        // Use a fixed test key
        let private_key = [1u8; 32];

        let result = Encoder::new(claim169, cwt_meta)
            .sign_with_ed25519(&private_key)
            .and_then(|e| e.encode());

        assert!(result.is_ok());
    }

    #[cfg(feature = "software-crypto")]
    #[test]
    fn test_encoder_with_encryption() {
        let claim169 = Claim169::minimal("encrypted-test", "Encrypted User");
        let cwt_meta = CwtMeta::new().with_issuer("test");

        let sign_key = [2u8; 32];
        let encrypt_key = [3u8; 32];
        let nonce = [4u8; 12];

        let result = Encoder::new(claim169, cwt_meta)
            .sign_with_ed25519(&sign_key)
            .and_then(|e| e.encrypt_with_aes256_nonce(&encrypt_key, &nonce))
            .and_then(|e| e.encode());

        assert!(result.is_ok());
    }

    #[cfg(feature = "software-crypto")]
    #[test]
    fn test_encoder_skip_biometrics() {
        use crate::model::Biometric;

        let mut claim169 = Claim169::minimal("bio-test", "Bio User");
        claim169.face = Some(vec![Biometric::new(vec![1, 2, 3, 4, 5])]);
        assert!(claim169.has_biometrics());

        let cwt_meta = CwtMeta::default();
        let sign_key = [5u8; 32];

        // Encode with biometrics
        let result_with_bio = Encoder::new(claim169.clone(), cwt_meta.clone())
            .sign_with_ed25519(&sign_key)
            .and_then(|e| e.encode())
            .unwrap();

        // Encode without biometrics
        let result_without_bio = Encoder::new(claim169, cwt_meta)
            .skip_biometrics()
            .sign_with_ed25519(&sign_key)
            .and_then(|e| e.encode())
            .unwrap();

        // Without biometrics should be smaller
        assert!(result_without_bio.len() < result_with_bio.len());
    }

    #[cfg(feature = "software-crypto")]
    #[test]
    fn test_encoder_roundtrip() {
        use crate::crypto::software::{AesGcmDecryptor, Ed25519Signer};
        use crate::model::VerificationStatus;
        use crate::pipeline::claim169::transform;
        use crate::pipeline::{base45_decode, cose_parse, cwt_parse, decompress};

        let original_claim = Claim169 {
            id: Some("roundtrip-builder".to_string()),
            full_name: Some("Builder Roundtrip".to_string()),
            email: Some("builder@test.com".to_string()),
            ..Default::default()
        };

        let cwt_meta = CwtMeta::new()
            .with_issuer("https://builder.test")
            .with_expires_at(1800000000);

        // Generate keys
        let signer = Ed25519Signer::generate();
        let verifier = signer.verifying_key();

        let encrypt_key = [10u8; 32];
        let nonce = [11u8; 12];

        // Encode using builder
        let qr_data = Encoder::new(original_claim.clone(), cwt_meta.clone())
            .sign_with(signer, iana::Algorithm::EdDSA)
            .encrypt_with_aes256_nonce(&encrypt_key, &nonce)
            .unwrap()
            .encode()
            .unwrap();

        // Decode and verify
        let compressed = base45_decode(&qr_data).unwrap();
        let cose_bytes = decompress(&compressed, 65536).unwrap();
        let decryptor = AesGcmDecryptor::aes256(&encrypt_key).unwrap();
        let cose_result = cose_parse(&cose_bytes, Some(&verifier), Some(&decryptor)).unwrap();

        assert_eq!(
            cose_result.verification_status,
            VerificationStatus::Verified
        );

        let cwt_result = cwt_parse(&cose_result.payload).unwrap();
        let decoded_claim = transform(cwt_result.claim_169, false).unwrap();

        assert_eq!(decoded_claim.id, original_claim.id);
        assert_eq!(decoded_claim.full_name, original_claim.full_name);
        assert_eq!(decoded_claim.email, original_claim.email);
    }

    #[test]
    fn test_generate_random_nonce() {
        let nonce1 = generate_random_nonce();
        let nonce2 = generate_random_nonce();

        assert_eq!(nonce1.len(), 12);
        assert_eq!(nonce2.len(), 12);
        assert_ne!(nonce1, nonce2);
    }
}
