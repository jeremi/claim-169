//! Cryptographic traits and implementations for Claim 169.
//!
//! This module provides the cryptographic primitives needed to verify signatures
//! and decrypt encrypted credentials.
//!
//! # Architecture
//!
//! The crypto module is designed around traits that allow pluggable implementations:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                     Your Application                        │
//! └─────────────────────────────────────────────────────────────┘
//!                              │
//!                              ▼
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    Crypto Traits                            │
//! │  SignatureVerifier, Decryptor, Signer, Encryptor, KeyResolver│
//! └─────────────────────────────────────────────────────────────┘
//!                              │
//!              ┌───────────────┴───────────────┐
//!              ▼                               ▼
//! ┌─────────────────────────┐   ┌─────────────────────────┐
//! │   Software Crypto       │   │   Custom Backend        │
//! │   (feature flag)        │   │   (HSM, Cloud KMS)      │
//! │                         │   │                         │
//! │   Ed25519Verifier       │   │   YourHsmVerifier       │
//! │   EcdsaP256Verifier     │   │   YourKmsDecryptor      │
//! │   AesGcmDecryptor       │   │                         │
//! └─────────────────────────┘   └─────────────────────────┘
//! ```
//!
//! # Traits
//!
//! | Trait | Purpose |
//! |-------|---------|
//! | [`SignatureVerifier`] | Verify COSE_Sign1 signatures |
//! | [`Decryptor`] | Decrypt COSE_Encrypt0 payloads |
//! | [`Signer`] | Create signatures (for testing/issuance) |
//! | [`Encryptor`] | Encrypt payloads (for testing/issuance) |
//! | [`KeyResolver`] | Look up keys by key ID |
//!
//! # Software Implementations
//!
//! When the `software-crypto` feature is enabled (default), these implementations
//! are available:
//!
//! | Type | Algorithm | Use |
//! |------|-----------|-----|
//! | [`Ed25519Verifier`] | Ed25519 | Signature verification |
//! | [`Ed25519Signer`] | Ed25519 | Signing (test vectors) |
//! | [`EcdsaP256Verifier`] | ECDSA P-256 | Signature verification |
//! | [`EcdsaP256Signer`] | ECDSA P-256 | Signing (test vectors) |
//! | [`AesGcmDecryptor`] | AES-128/256-GCM | Decryption |
//! | [`AesGcmEncryptor`] | AES-128/256-GCM | Encryption (test vectors) |
//!
//! # Example: Using Software Crypto
//!
//! ```rust,ignore
//! use claim169_core::Decoder;
//!
//! // Using Ed25519 public key in PEM format
//! let result = Decoder::new(qr_content)
//!     .verify_with_ed25519_pem(r#"
//! -----BEGIN PUBLIC KEY-----
//! MCowBQYDK2VwAyEAExample...
//! -----END PUBLIC KEY-----
//! "#)?
//!     .decode()?;
//! ```
//!
//! # Example: Custom HSM Integration
//!
//! ```rust,ignore
//! use claim169_core::{SignatureVerifier, CryptoResult, CryptoError};
//! use coset::iana::Algorithm;
//!
//! struct HsmVerifier {
//!     hsm_client: HsmClient,
//!     key_label: String,
//! }
//!
//! impl SignatureVerifier for HsmVerifier {
//!     fn verify(&self, algorithm: Algorithm, key_id: Option<&[u8]>,
//!               data: &[u8], signature: &[u8]) -> CryptoResult<()> {
//!         // Delegate to HSM
//!         self.hsm_client.verify(&self.key_label, data, signature)
//!             .map_err(|e| CryptoError::VerificationFailed)
//!     }
//! }
//! ```
//!
//! # Security
//!
//! - Weak keys (all-zeros, small-order points) are automatically rejected
//! - Software signing keys are zeroized on drop (where supported by underlying crates)
//! - For production use, consider HSM-backed implementations

pub mod software;
pub mod traits;

pub use traits::{Decryptor, Encryptor, KeyResolver, SignatureVerifier, Signer};

#[cfg(feature = "software-crypto")]
pub use software::{
    AesGcmDecryptor, AesGcmEncryptor, EcdsaP256Signer, EcdsaP256Verifier, Ed25519Signer,
    Ed25519Verifier,
};
