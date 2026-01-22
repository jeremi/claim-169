//! # claim169-core
//!
//! A Rust library for decoding and verifying MOSIP Claim 169 QR codes.
//!
//! ## Overview
//!
//! [MOSIP Claim 169](https://docs.mosip.io/) defines a standard for encoding identity data
//! in QR codes, designed for offline verification of digital identity credentials. The format
//! uses a compact binary encoding optimized for QR code capacity constraints.
//!
//! The encoding pipeline:
//! ```text
//! Identity Data → CBOR → CWT → COSE_Sign1 → zlib → Base45 → QR Code
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
//! ### Basic Decoding (No Verification)
//!
//! ```rust,ignore
//! use claim169_core::{decode, DecodeOptions};
//!
//! let qr_content = "6BFQX..."; // Base45-encoded QR content
//!
//! // Decode without signature verification (INSECURE - for testing only)
//! let result = decode(qr_content, DecodeOptions::default().allow_unverified())?;
//!
//! println!("ID: {:?}", result.claim169.id);
//! println!("Name: {:?}", result.claim169.full_name);
//! println!("Issuer: {:?}", result.cwt_meta.issuer);
//! ```
//!
//! ### Verified Decoding (Recommended)
//!
//! ```rust,ignore
//! use claim169_core::{decode_with_verifier, DecodeOptions, Ed25519Verifier};
//!
//! // Load the issuer's public key
//! let public_key_pem = r#"
//! -----BEGIN PUBLIC KEY-----
//! MCowBQYDK2VwAyEA...
//! -----END PUBLIC KEY-----
//! "#;
//!
//! let verifier = Ed25519Verifier::from_pem(public_key_pem)?;
//! let result = decode_with_verifier(qr_content, &verifier, DecodeOptions::default())?;
//!
//! // Signature was verified - data is authentic
//! assert_eq!(result.verification_status, VerificationStatus::Verified);
//! ```
//!
//! ### Decoding Encrypted Credentials
//!
//! ```rust,ignore
//! use claim169_core::{decode_encrypted, AesGcmDecryptor, Ed25519Verifier, DecodeOptions};
//!
//! let decryptor = AesGcmDecryptor::new(&encryption_key);
//! let verifier = Ed25519Verifier::from_pem(public_key_pem)?;
//!
//! let result = decode_encrypted(
//!     qr_content,
//!     &decryptor,
//!     Some(&verifier),
//!     DecodeOptions::default()
//! )?;
//! ```
//!
//! ### Using a Key Resolver
//!
//! For systems with multiple issuers or key rotation:
//!
//! ```rust,ignore
//! use claim169_core::{decode_with_resolver, KeyResolver, DecodeOptions};
//!
//! // Implement KeyResolver to look up keys by key_id
//! struct MyKeyStore { /* ... */ }
//!
//! impl KeyResolver for MyKeyStore {
//!     fn resolve_verifier(&self, key_id: Option<&[u8]>, algorithm: Algorithm)
//!         -> CryptoResult<Box<dyn SignatureVerifier>> {
//!         // Look up the key and return the appropriate verifier
//!     }
//!     // ...
//! }
//!
//! let resolver = MyKeyStore::new();
//! let result = decode_with_resolver(qr_content, &resolver, DecodeOptions::default())?;
//! ```
//!
//! ## Decode Options
//!
//! Configure decoding behavior with [`DecodeOptions`]:
//!
//! ```rust,ignore
//! use claim169_core::DecodeOptions;
//!
//! // Strict settings (production)
//! let strict = DecodeOptions::strict();
//!
//! // Custom configuration
//! let options = DecodeOptions::new()
//!     .with_max_decompressed_bytes(32768)  // 32KB limit
//!     .skip_biometrics()                    // Don't parse biometric data
//!     .with_clock_skew_tolerance(60);       // 60 seconds tolerance
//!
//! // Permissive settings (development/debugging)
//! let permissive = DecodeOptions::permissive();
//! ```
//!
//! ## Security Considerations
//!
//! - **Always verify signatures** in production using `decode_with_verifier()` or `decode_with_resolver()`
//! - The default `DecodeOptions` requires verification - use `.allow_unverified()` explicitly to disable
//! - Decompression is limited to prevent zip bomb attacks (default: 64KB)
//! - Timestamps are validated by default; use `.without_timestamp_validation()` to disable
//! - Weak cryptographic keys (all-zeros, small-order points) are automatically rejected
//!
//! See the [SECURITY.md](https://github.com/mosip/claim169/SECURITY.md) for detailed security guidance.
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
//! Then implement the [`SignatureVerifier`], [`Decryptor`], or [`KeyResolver`] traits.
//!
//! ## Modules
//!
//! - [`crypto`]: Cryptographic traits and implementations
//! - [`error`]: Error types for decoding and crypto operations
//! - [`model`]: Data structures for Claim 169 identity data
//! - [`options`]: Configuration options for decoding
//! - [`pipeline`]: Low-level encoding/decoding pipeline functions

pub mod crypto;
pub mod error;
pub mod model;
pub mod options;
pub mod pipeline;

pub use crypto::traits::{Decryptor, Encryptor, KeyResolver, SignatureVerifier, Signer};
pub use error::{Claim169Error, CryptoError, CryptoResult, Result};
pub use model::{
    Biometric, BiometricFormat, BiometricSubFormat, Claim169, CwtMeta, Gender, MaritalStatus,
    PhotoFormat, VerificationStatus,
};
pub use options::DecodeOptions;

#[cfg(feature = "software-crypto")]
pub use crypto::{
    AesGcmDecryptor, AesGcmEncryptor, EcdsaP256Signer, EcdsaP256Verifier, Ed25519Signer,
    Ed25519Verifier,
};

use std::time::{SystemTime, UNIX_EPOCH};

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
/// let result = decode_with_verifier(qr_content, &verifier, options)?;
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

    /// Warnings generated during decoding.
    ///
    /// Non-fatal issues that don't prevent decoding but may warrant attention,
    /// such as unknown fields (forward compatibility) or skipped validations.
    pub warnings: Vec<Warning>,
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
}

/// Decode a Claim 169 QR code without signature verification
///
/// This function decodes the QR payload but does NOT verify the signature.
/// The `verification_status` in the result will be `Skipped`.
///
/// # Arguments
/// * `qr_text` - The Base45-encoded QR code content
/// * `options` - Decoding options (limits, validation settings)
///
/// # Returns
/// * `Ok(DecodeResult)` - Successfully decoded claim
/// * `Err(Claim169Error)` - Decoding failed
pub fn decode(qr_text: &str, options: DecodeOptions) -> Result<DecodeResult> {
    decode_internal(qr_text, None, None, options)
}

/// Decode a Claim 169 QR code with signature verification
///
/// This function decodes the QR payload and verifies the signature
/// using the provided verifier.
///
/// # Arguments
/// * `qr_text` - The Base45-encoded QR code content
/// * `verifier` - Implementation of SignatureVerifier trait
/// * `options` - Decoding options
///
/// # Returns
/// * `Ok(DecodeResult)` - Successfully decoded and verified claim
/// * `Err(Claim169Error)` - Decoding or verification failed
pub fn decode_with_verifier<V: SignatureVerifier>(
    qr_text: &str,
    verifier: &V,
    options: DecodeOptions,
) -> Result<DecodeResult> {
    decode_internal(qr_text, Some(verifier), None, options)
}

/// Decode an encrypted Claim 169 QR code
///
/// This function decodes an encrypted QR payload using the provided decryptor,
/// and optionally verifies the signature.
///
/// # Arguments
/// * `qr_text` - The Base45-encoded QR code content
/// * `decryptor` - Implementation of Decryptor trait
/// * `verifier` - Optional signature verifier (if the decrypted payload is signed)
/// * `options` - Decoding options
///
/// # Returns
/// * `Ok(DecodeResult)` - Successfully decrypted (and verified) claim
/// * `Err(Claim169Error)` - Decryption or decoding failed
pub fn decode_encrypted<D: Decryptor, V: SignatureVerifier>(
    qr_text: &str,
    decryptor: &D,
    verifier: Option<&V>,
    options: DecodeOptions,
) -> Result<DecodeResult> {
    decode_internal(
        qr_text,
        verifier.map(|v| v as &dyn SignatureVerifier),
        Some(decryptor as &dyn Decryptor),
        options,
    )
}

/// Decode a Claim 169 QR code using a KeyResolver
///
/// This function decodes the QR payload and resolves the appropriate
/// verifier and/or decryptor based on the key_id in the COSE header.
/// This is useful when multiple issuers or key management systems are involved.
///
/// # Arguments
/// * `qr_text` - The Base45-encoded QR code content
/// * `resolver` - Implementation of KeyResolver trait
/// * `options` - Decoding options
///
/// # Returns
/// * `Ok(DecodeResult)` - Successfully decoded and verified claim
/// * `Err(Claim169Error)` - Decoding, resolution, or verification failed
///
/// # Example
///
/// ```rust,ignore
/// use claim169_core::{decode_with_resolver, KeyResolver, DecodeOptions};
///
/// struct MyKeyResolver {
///     // Maps key IDs to public keys
///     keys: HashMap<Vec<u8>, Ed25519Verifier>,
/// }
///
/// impl KeyResolver for MyKeyResolver {
///     fn resolve_verifier(&self, key_id: Option<&[u8]>, algorithm: Algorithm)
///         -> CryptoResult<Box<dyn SignatureVerifier>> {
///         let kid = key_id.ok_or(CryptoError::KeyNotFound)?;
///         let verifier = self.keys.get(kid).ok_or(CryptoError::KeyNotFound)?;
///         Ok(Box::new(verifier.clone()))
///     }
///     // ... resolve_decryptor implementation
/// }
///
/// let resolver = MyKeyResolver { keys: my_key_map };
/// let result = decode_with_resolver("QR_DATA", &resolver, DecodeOptions::default())?;
/// ```
pub fn decode_with_resolver<R: KeyResolver>(
    qr_text: &str,
    resolver: &R,
    options: DecodeOptions,
) -> Result<DecodeResult> {
    decode_with_resolver_internal(qr_text, resolver, options)
}

/// Internal decode implementation using KeyResolver
fn decode_with_resolver_internal<R: KeyResolver>(
    qr_text: &str,
    resolver: &R,
    options: DecodeOptions,
) -> Result<DecodeResult> {
    let mut warnings = Vec::new();

    // Step 1: Base45 decode
    let compressed = pipeline::base45_decode(qr_text)?;

    // Step 2: zlib decompress
    let cose_bytes = pipeline::decompress(&compressed, options.max_decompressed_bytes)?;

    // Step 3-4: Parse COSE with resolver-based verification
    let cose_result = pipeline::cose_parse_with_resolver(&cose_bytes, resolver)?;

    // Check if verification was required but skipped
    if !options.allow_unverified && cose_result.verification_status == VerificationStatus::Skipped {
        return Err(Claim169Error::SignatureInvalid(
            "verification required but no verifier provided".to_string(),
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
    if options.validate_timestamps {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        let skew = options.clock_skew_tolerance_seconds;

        if let Some(exp) = cwt_result.meta.expires_at {
            // Allow clock skew tolerance for expiration
            if now > exp + skew {
                return Err(Claim169Error::Expired(exp));
            }
        }

        if let Some(nbf) = cwt_result.meta.not_before {
            // Allow clock skew tolerance for not-before
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
    let claim169 = pipeline::claim169_transform(cwt_result.claim_169, options.skip_biometrics)?;

    if options.skip_biometrics {
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

/// Internal decode implementation
fn decode_internal(
    qr_text: &str,
    verifier: Option<&dyn SignatureVerifier>,
    decryptor: Option<&dyn Decryptor>,
    options: DecodeOptions,
) -> Result<DecodeResult> {
    let mut warnings = Vec::new();

    // Step 1: Base45 decode
    let compressed = pipeline::base45_decode(qr_text)?;

    // Step 2: zlib decompress
    let cose_bytes = pipeline::decompress(&compressed, options.max_decompressed_bytes)?;

    // Step 3-4: Parse COSE and verify/decrypt
    let cose_result = pipeline::cose_parse(&cose_bytes, verifier, decryptor)?;

    // Check if verification was required but skipped
    if !options.allow_unverified && cose_result.verification_status == VerificationStatus::Skipped {
        return Err(Claim169Error::SignatureInvalid(
            "verification required but no verifier provided".to_string(),
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
    if options.validate_timestamps {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        let skew = options.clock_skew_tolerance_seconds;

        if let Some(exp) = cwt_result.meta.expires_at {
            // Allow clock skew tolerance for expiration
            if now > exp + skew {
                return Err(Claim169Error::Expired(exp));
            }
        }

        if let Some(nbf) = cwt_result.meta.not_before {
            // Allow clock skew tolerance for not-before
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
    let claim169 = pipeline::claim169_transform(cwt_result.claim_169, options.skip_biometrics)?;

    if options.skip_biometrics {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_options_strict() {
        let opts = DecodeOptions::strict();
        assert!(!opts.allow_unverified);
        assert!(opts.validate_timestamps);
    }

    #[test]
    fn test_decode_options_permissive() {
        let opts = DecodeOptions::permissive();
        assert!(opts.allow_unverified);
        assert!(!opts.validate_timestamps);
    }
}
