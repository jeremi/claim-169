//! Error types for Claim 169 decoding and cryptographic operations.
//!
//! This module provides two main error types:
//!
//! - [`Claim169Error`]: High-level errors from the decoding pipeline
//! - [`CryptoError`]: Lower-level cryptographic operation errors
//!
//! # Error Handling
//!
//! All public functions return [`Result<T>`] which uses [`Claim169Error`]:
//!
//! ```rust,ignore
//! use claim169_core::{Decoder, Claim169Error};
//!
//! match Decoder::new(qr_content).allow_unverified().decode() {
//!     Ok(result) => println!("Decoded: {:?}", result.claim169.full_name),
//!     Err(Claim169Error::Expired(ts)) => println!("Credential expired at {}", ts),
//!     Err(Claim169Error::SignatureInvalid(msg)) => println!("Bad signature: {}", msg),
//!     Err(e) => println!("Error: {}", e),
//! }
//! ```
//!
//! # Error Categories
//!
//! | Error Type | Description |
//! |------------|-------------|
//! | `Base45Decode` | Invalid Base45 encoding in QR string |
//! | `Decompress*` | Decompression failure or limit exceeded |
//! | `CoseParse`, `CborParse` | Invalid binary structure |
//! | `SignatureInvalid` | Signature verification failed |
//! | `DecryptionFailed` | Decryption failed |
//! | `Expired`, `NotYetValid` | Timestamp validation failed |
//! | `Claim169NotFound` | Required claim not in payload |

use thiserror::Error;

/// Errors that can occur during Claim 169 QR decoding.
///
/// This enum covers all errors in the decoding pipeline, from Base45 decoding
/// through CBOR parsing to signature verification and timestamp validation.
///
/// # Converting from CryptoError
///
/// [`CryptoError`] automatically converts to `Claim169Error` via the `From` trait:
///
/// ```rust,ignore
/// let crypto_err = CryptoError::VerificationFailed;
/// let claim_err: Claim169Error = crypto_err.into();
/// ```
#[derive(Debug, Error)]
pub enum Claim169Error {
    /// Invalid Base45 encoding in QR string
    #[error("invalid Base45 encoding: {0}")]
    Base45Decode(String),

    /// Failed to decompress zlib data
    #[error("decompression failed: {0}")]
    Decompress(String),

    /// Decompressed data exceeds safety limit
    #[error("decompression limit exceeded: max {max_bytes} bytes")]
    DecompressLimitExceeded { max_bytes: usize },

    /// Invalid COSE structure
    #[error("invalid COSE structure: {0}")]
    CoseParse(String),

    /// Unsupported COSE message type
    #[error("unsupported COSE type: expected Sign1, Sign, Encrypt0, or Encrypt, got {0}")]
    UnsupportedCoseType(String),

    /// COSE signature verification failed
    #[error("signature verification failed: {0}")]
    SignatureInvalid(String),

    /// COSE decryption failed
    #[error("decryption failed: {0}")]
    DecryptionFailed(String),

    /// Invalid CBOR structure
    #[error("invalid CBOR: {0}")]
    CborParse(String),

    /// CWT parsing failed
    #[error("CWT parsing failed: {0}")]
    CwtParse(String),

    /// Claim 169 not found in CWT payload
    #[error("claim 169 not found in CWT payload")]
    Claim169NotFound,

    /// Invalid Claim 169 structure
    #[error("invalid claim 169 structure: {0}")]
    Claim169Invalid(String),

    /// Unsupported cryptographic algorithm
    #[error("unsupported algorithm: {0}")]
    UnsupportedAlgorithm(String),

    /// Key not found for the given key ID
    #[error("key not found for kid: {0:?}")]
    KeyNotFound(Option<Vec<u8>>),

    /// Credential has expired
    #[error("credential expired at timestamp {0}")]
    Expired(i64),

    /// Credential is not yet valid
    #[error("credential not valid until timestamp {0}")]
    NotYetValid(i64),

    /// Crypto operation error
    #[error("crypto error: {0}")]
    Crypto(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    // ========== Encoding errors ==========
    /// CBOR encoding failed
    #[error("CBOR encoding failed: {0}")]
    CborEncode(String),

    /// Signing failed
    #[error("signing failed: {0}")]
    SignatureFailed(String),

    /// Encryption failed
    #[error("encryption failed: {0}")]
    EncryptionFailed(String),

    /// Encoding configuration error
    #[error("encoding configuration error: {0}")]
    EncodingConfig(String),

    /// Decoding configuration error
    #[error("decoding configuration error: {0}")]
    DecodingConfig(String),
}

/// Errors specific to cryptographic operations.
///
/// These errors occur within [`SignatureVerifier`](crate::SignatureVerifier),
/// [`Decryptor`](crate::Decryptor), and related crypto implementations.
///
/// `CryptoError` automatically converts to [`Claim169Error`] when returned
/// from the main decoding functions.
///
/// # Implementing Custom Crypto
///
/// When implementing custom cryptographic backends, return appropriate
/// `CryptoError` variants:
///
/// ```rust,ignore
/// impl SignatureVerifier for MyHsmVerifier {
///     fn verify(&self, algorithm: Algorithm, key_id: Option<&[u8]>,
///               data: &[u8], signature: &[u8]) -> CryptoResult<()> {
///         if !self.has_key(key_id) {
///             return Err(CryptoError::KeyNotFound);
///         }
///         if !self.supports_algorithm(algorithm) {
///             return Err(CryptoError::UnsupportedAlgorithm(format!("{:?}", algorithm)));
///         }
///         // ... perform verification
///         if !valid {
///             return Err(CryptoError::VerificationFailed);
///         }
///         Ok(())
///     }
/// }
/// ```
#[derive(Debug, Error)]
pub enum CryptoError {
    /// Invalid key format
    #[error("invalid key format: {0}")]
    InvalidKeyFormat(String),

    /// Key not found
    #[error("key not found")]
    KeyNotFound,

    /// Signature verification failed
    #[error("signature verification failed")]
    VerificationFailed,

    /// Decryption failed
    #[error("decryption failed: {0}")]
    DecryptionFailed(String),

    /// Signing failed (for custom signer callbacks)
    #[error("signing failed: {0}")]
    SigningFailed(String),

    /// Encryption failed (for custom encryptor callbacks)
    #[error("encryption failed: {0}")]
    EncryptionFailed(String),

    /// Unsupported algorithm
    #[error("unsupported algorithm: {0}")]
    UnsupportedAlgorithm(String),

    /// Generic crypto error
    #[error("{0}")]
    Other(String),
}

impl From<CryptoError> for Claim169Error {
    fn from(err: CryptoError) -> Self {
        match err {
            CryptoError::VerificationFailed => {
                Claim169Error::SignatureInvalid("verification failed".to_string())
            }
            CryptoError::DecryptionFailed(msg) => Claim169Error::DecryptionFailed(msg),
            CryptoError::SigningFailed(msg) => Claim169Error::SignatureFailed(msg),
            CryptoError::EncryptionFailed(msg) => Claim169Error::EncryptionFailed(msg),
            CryptoError::UnsupportedAlgorithm(alg) => Claim169Error::UnsupportedAlgorithm(alg),
            CryptoError::KeyNotFound => Claim169Error::KeyNotFound(None),
            other => Claim169Error::Crypto(other.to_string()),
        }
    }
}

/// Result type for Claim 169 operations
pub type Result<T> = std::result::Result<T, Claim169Error>;

/// Result type for crypto operations
pub type CryptoResult<T> = std::result::Result<T, CryptoError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Claim169Error::Base45Decode("invalid character at position 5".to_string());
        assert!(err.to_string().contains("Base45"));
        assert!(err.to_string().contains("position 5"));
    }

    #[test]
    fn test_crypto_error_conversion() {
        let crypto_err = CryptoError::VerificationFailed;
        let claim_err: Claim169Error = crypto_err.into();

        assert!(matches!(claim_err, Claim169Error::SignatureInvalid(_)));
    }

    #[test]
    fn test_decompression_limit_error() {
        let err = Claim169Error::DecompressLimitExceeded { max_bytes: 65536 };
        assert!(err.to_string().contains("65536"));
    }
}
