//! Cryptographic traits for pluggable crypto backends.
//!
//! These traits define the interface between the Claim 169 decoder and
//! cryptographic operations. Implement these traits to integrate with
//! external crypto providers such as:
//!
//! - Hardware Security Modules (HSMs)
//! - Cloud KMS (AWS KMS, Google Cloud KMS, Azure Key Vault)
//! - Remote signing services
//! - Smart cards and TPMs
//! - Custom software keystores
//!
//! # Verification vs Signing
//!
//! - [`SignatureVerifier`] and [`Decryptor`]: Used during credential verification
//! - [`Signer`] and [`Encryptor`]: Used for credential issuance
//!
//! # Thread Safety
//!
//! All traits require `Send + Sync` for use in multi-threaded contexts.

use coset::iana;

use crate::error::CryptoResult;

/// Trait for signature verification.
///
/// Implement this trait to provide custom signature verification
/// using external crypto providers (HSM, cloud KMS, etc.).
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

/// Trait for decryption.
///
/// Implement this trait to provide custom decryption
/// using external crypto providers (HSM, cloud KMS, etc.).
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

/// Trait for signing.
///
/// Implement this trait to provide custom signing
/// using external crypto providers (HSM, cloud KMS, etc.).
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

/// Trait for encryption.
///
/// Implement this trait to provide custom encryption
/// using external crypto providers (HSM, cloud KMS, etc.).
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

/// Key resolver trait for looking up keys by key ID.
///
/// Implement this trait to provide key lookup functionality
/// using external key management systems (HSM, cloud KMS, etc.).
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CryptoError;

    /// Mock verifier for testing blanket implementations
    struct MockVerifier {
        expected_result: bool,
    }

    impl SignatureVerifier for MockVerifier {
        fn verify(
            &self,
            _algorithm: iana::Algorithm,
            _key_id: Option<&[u8]>,
            _data: &[u8],
            _signature: &[u8],
        ) -> CryptoResult<()> {
            if self.expected_result {
                Ok(())
            } else {
                Err(CryptoError::VerificationFailed)
            }
        }
    }

    /// Mock decryptor for testing blanket implementations
    struct MockDecryptor {
        plaintext: Vec<u8>,
    }

    impl Decryptor for MockDecryptor {
        fn decrypt(
            &self,
            _algorithm: iana::Algorithm,
            _key_id: Option<&[u8]>,
            _nonce: &[u8],
            _aad: &[u8],
            _ciphertext: &[u8],
        ) -> CryptoResult<Vec<u8>> {
            Ok(self.plaintext.clone())
        }
    }

    /// Mock signer for testing default key_id implementation
    struct MockSigner;

    impl Signer for MockSigner {
        fn sign(
            &self,
            _algorithm: iana::Algorithm,
            _key_id: Option<&[u8]>,
            _data: &[u8],
        ) -> CryptoResult<Vec<u8>> {
            Ok(vec![1, 2, 3, 4])
        }
        // Uses default key_id() implementation
    }

    #[test]
    fn test_signature_verifier_ref_blanket_impl() {
        let verifier = MockVerifier {
            expected_result: true,
        };
        let verifier_ref: &dyn SignatureVerifier = &verifier;

        let result = verifier_ref.verify(iana::Algorithm::EdDSA, None, b"data", b"sig");
        assert!(result.is_ok());
    }

    #[test]
    fn test_signature_verifier_box_blanket_impl() {
        let verifier = MockVerifier {
            expected_result: true,
        };
        let boxed: Box<dyn SignatureVerifier> = Box::new(verifier);

        let result = boxed.verify(iana::Algorithm::EdDSA, None, b"data", b"sig");
        assert!(result.is_ok());
    }

    #[test]
    fn test_signature_verifier_box_failure() {
        let verifier = MockVerifier {
            expected_result: false,
        };
        let boxed: Box<dyn SignatureVerifier> = Box::new(verifier);

        let result = boxed.verify(iana::Algorithm::EdDSA, None, b"data", b"sig");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CryptoError::VerificationFailed
        ));
    }

    #[test]
    fn test_decryptor_ref_blanket_impl() {
        let decryptor = MockDecryptor {
            plaintext: vec![1, 2, 3],
        };
        let decryptor_ref: &dyn Decryptor = &decryptor;

        let result = decryptor_ref.decrypt(iana::Algorithm::A256GCM, None, b"nonce", b"aad", b"ct");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_decryptor_box_blanket_impl() {
        let decryptor = MockDecryptor {
            plaintext: vec![4, 5, 6],
        };
        let boxed: Box<dyn Decryptor> = Box::new(decryptor);

        let result = boxed.decrypt(iana::Algorithm::A256GCM, None, b"nonce", b"aad", b"ct");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![4, 5, 6]);
    }

    #[test]
    fn test_signer_default_key_id() {
        let signer = MockSigner;
        assert!(signer.key_id().is_none());
    }

    #[test]
    fn test_signature_verifier_with_key_id() {
        let verifier = MockVerifier {
            expected_result: true,
        };
        let key_id = b"test-key-id";

        let result = verifier.verify(iana::Algorithm::EdDSA, Some(key_id), b"data", b"sig");
        assert!(result.is_ok());
    }

    #[test]
    fn test_decryptor_with_key_id() {
        let decryptor = MockDecryptor {
            plaintext: vec![7, 8, 9],
        };
        let key_id = b"decrypt-key-id";

        let result = decryptor.decrypt(
            iana::Algorithm::A256GCM,
            Some(key_id),
            b"nonce",
            b"aad",
            b"ciphertext",
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![7, 8, 9]);
    }
}
