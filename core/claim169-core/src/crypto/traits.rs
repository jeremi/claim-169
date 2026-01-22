//! Cryptographic traits for pluggable crypto backends.
//!
//! These traits define the interface between the Claim 169 decoder and
//! cryptographic operations. Implement these traits to integrate with
//! custom backends like HSMs, cloud KMS, or hardware security modules.
//!
//! # Verification vs Signing
//!
//! - [`SignatureVerifier`] and [`Decryptor`]: Used during credential verification
//! - [`Signer`] and [`Encryptor`]: Used for credential issuance and test vector generation
//!
//! # Thread Safety
//!
//! All traits require `Send + Sync` for use in multi-threaded contexts.

use coset::iana;

use crate::error::CryptoResult;

/// Trait for signature verification.
///
/// Implement this trait to provide custom signature verification,
/// such as delegating to an HSM or other hardware security module.
pub trait SignatureVerifier: Send + Sync {
    /// Verify a signature over the given data
    ///
    /// # Arguments
    /// * `algorithm` - The COSE algorithm identifier
    /// * `key_id` - Optional key identifier from the COSE header
    /// * `data` - The data that was signed (Sig_structure)
    /// * `signature` - The signature bytes to verify
    ///
    /// # Returns
    /// * `Ok(())` if the signature is valid
    /// * `Err(CryptoError::VerificationFailed)` if the signature is invalid
    /// * `Err(...)` for other errors (key not found, unsupported algorithm, etc.)
    fn verify(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
        signature: &[u8],
    ) -> CryptoResult<()>;
}

/// Trait for decryption
///
/// Implement this trait to provide custom decryption,
/// such as delegating to an HSM or other hardware security module.
pub trait Decryptor: Send + Sync {
    /// Decrypt ciphertext using AEAD
    ///
    /// # Arguments
    /// * `algorithm` - The COSE algorithm identifier
    /// * `key_id` - Optional key identifier from the COSE header
    /// * `nonce` - The IV/nonce for decryption
    /// * `aad` - Additional authenticated data (Enc_structure)
    /// * `ciphertext` - The ciphertext to decrypt (includes auth tag for AEAD)
    ///
    /// # Returns
    /// * `Ok(plaintext)` if decryption succeeds
    /// * `Err(CryptoError::DecryptionFailed)` if decryption fails
    fn decrypt(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        ciphertext: &[u8],
    ) -> CryptoResult<Vec<u8>>;
}

/// Trait for signing (used in test vector generation)
///
/// Implement this trait to provide custom signing,
/// such as delegating to an HSM for key protection.
pub trait Signer: Send + Sync {
    /// Sign data
    ///
    /// # Arguments
    /// * `algorithm` - The COSE algorithm identifier
    /// * `key_id` - Optional key identifier
    /// * `data` - The data to sign (Sig_structure)
    ///
    /// # Returns
    /// * `Ok(signature)` containing the signature bytes
    fn sign(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
    ) -> CryptoResult<Vec<u8>>;

    /// Get the key ID for this signer
    fn key_id(&self) -> Option<&[u8]> {
        None
    }
}

/// Trait for encryption (used in test vector generation)
///
/// Implement this trait to provide custom encryption.
pub trait Encryptor: Send + Sync {
    /// Encrypt plaintext using AEAD
    ///
    /// # Arguments
    /// * `algorithm` - The COSE algorithm identifier
    /// * `key_id` - Optional key identifier
    /// * `nonce` - The IV/nonce for encryption
    /// * `aad` - Additional authenticated data
    /// * `plaintext` - The plaintext to encrypt
    ///
    /// # Returns
    /// * `Ok(ciphertext)` containing ciphertext with auth tag
    fn encrypt(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        plaintext: &[u8],
    ) -> CryptoResult<Vec<u8>>;
}

/// Key resolver trait for looking up keys by key ID
///
/// Implement this trait to provide key lookup functionality,
/// which can delegate to key management systems, HSMs, or local storage.
pub trait KeyResolver: Send + Sync {
    /// Resolve a verifier for the given key ID and algorithm
    fn resolve_verifier(
        &self,
        key_id: Option<&[u8]>,
        algorithm: iana::Algorithm,
    ) -> CryptoResult<Box<dyn SignatureVerifier>>;

    /// Resolve a decryptor for the given key ID and algorithm
    fn resolve_decryptor(
        &self,
        key_id: Option<&[u8]>,
        algorithm: iana::Algorithm,
    ) -> CryptoResult<Box<dyn Decryptor>>;
}

/// Blanket implementation allowing `&T` where `T: SignatureVerifier`.
impl<T: SignatureVerifier + ?Sized> SignatureVerifier for &T {
    fn verify(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
        signature: &[u8],
    ) -> CryptoResult<()> {
        (*self).verify(algorithm, key_id, data, signature)
    }
}

/// Blanket implementation allowing `Box<T>` where `T: SignatureVerifier`.
impl<T: SignatureVerifier + ?Sized> SignatureVerifier for Box<T> {
    fn verify(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
        signature: &[u8],
    ) -> CryptoResult<()> {
        self.as_ref().verify(algorithm, key_id, data, signature)
    }
}

/// Blanket implementation allowing `&T` where `T: Decryptor`.
impl<T: Decryptor + ?Sized> Decryptor for &T {
    fn decrypt(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        ciphertext: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        (*self).decrypt(algorithm, key_id, nonce, aad, ciphertext)
    }
}

/// Blanket implementation allowing `Box<T>` where `T: Decryptor`.
impl<T: Decryptor + ?Sized> Decryptor for Box<T> {
    fn decrypt(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        ciphertext: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        self.as_ref()
            .decrypt(algorithm, key_id, nonce, aad, ciphertext)
    }
}
