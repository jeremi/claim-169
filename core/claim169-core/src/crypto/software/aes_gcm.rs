#[cfg(feature = "software-crypto")]
use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes128Gcm, Aes256Gcm,
};
#[cfg(feature = "software-crypto")]
use coset::iana;

#[cfg(feature = "software-crypto")]
use crate::crypto::traits::{Decryptor, Encryptor};
#[cfg(feature = "software-crypto")]
use crate::error::{CryptoError, CryptoResult};

/// AES-GCM decryptor supporting A128GCM and A256GCM
///
/// Key material is automatically zeroized on drop via the `zeroize` feature
/// of the underlying `aes-gcm` crate.
#[cfg(feature = "software-crypto")]
pub struct AesGcmDecryptor {
    key: AesGcmKey,
}

#[cfg(feature = "software-crypto")]
enum AesGcmKey {
    // The underlying `aes` crate with zeroize feature handles key material cleanup
    Aes128(Box<Aes128Gcm>),
    Aes256(Box<Aes256Gcm>),
}

#[cfg(feature = "software-crypto")]
impl AesGcmDecryptor {
    /// Create a new AES-128-GCM decryptor (16-byte key)
    pub fn aes128(key: &[u8]) -> CryptoResult<Self> {
        if key.len() != 16 {
            return Err(CryptoError::InvalidKeyFormat(
                "AES-128-GCM key must be 16 bytes".to_string(),
            ));
        }

        let key_array: [u8; 16] = key.try_into().unwrap();
        let cipher = Aes128Gcm::new(&key_array.into());

        Ok(Self {
            key: AesGcmKey::Aes128(Box::new(cipher)),
        })
    }

    /// Create a new AES-256-GCM decryptor (32-byte key)
    pub fn aes256(key: &[u8]) -> CryptoResult<Self> {
        if key.len() != 32 {
            return Err(CryptoError::InvalidKeyFormat(
                "AES-256-GCM key must be 32 bytes".to_string(),
            ));
        }

        let key_array: [u8; 32] = key.try_into().unwrap();
        let cipher = Aes256Gcm::new(&key_array.into());

        Ok(Self {
            key: AesGcmKey::Aes256(Box::new(cipher)),
        })
    }

    /// Create a decryptor from key bytes, auto-detecting key size
    pub fn from_bytes(key: &[u8]) -> CryptoResult<Self> {
        match key.len() {
            16 => Self::aes128(key),
            32 => Self::aes256(key),
            _ => Err(CryptoError::InvalidKeyFormat(
                "AES-GCM key must be 16 or 32 bytes".to_string(),
            )),
        }
    }
}

#[cfg(feature = "software-crypto")]
impl Decryptor for AesGcmDecryptor {
    fn decrypt(
        &self,
        algorithm: iana::Algorithm,
        _key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        ciphertext: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        // Verify algorithm matches key type
        match (&self.key, algorithm) {
            (AesGcmKey::Aes128(_), iana::Algorithm::A128GCM) => {}
            (AesGcmKey::Aes256(_), iana::Algorithm::A256GCM) => {}
            _ => {
                return Err(CryptoError::UnsupportedAlgorithm(format!(
                    "Algorithm {:?} does not match key type",
                    algorithm
                )));
            }
        }

        // AES-GCM nonce is 12 bytes
        if nonce.len() != 12 {
            return Err(CryptoError::DecryptionFailed(format!(
                "AES-GCM nonce must be 12 bytes, got {}",
                nonce.len()
            )));
        }

        // Convert validated slice to fixed-size array for Nonce
        let nonce_array: &[u8; 12] = nonce.try_into().unwrap();
        let payload = Payload {
            msg: ciphertext,
            aad,
        };

        match &self.key {
            AesGcmKey::Aes128(cipher) => {
                cipher.decrypt(nonce_array.into(), payload).map_err(|_| {
                    CryptoError::DecryptionFailed("AES-128-GCM decryption failed".to_string())
                })
            }
            AesGcmKey::Aes256(cipher) => {
                cipher.decrypt(nonce_array.into(), payload).map_err(|_| {
                    CryptoError::DecryptionFailed("AES-256-GCM decryption failed".to_string())
                })
            }
        }
    }
}

/// AES-GCM encryptor (for test vector generation)
///
/// Key material is automatically zeroized on drop via the `zeroize` feature
/// of the underlying `aes-gcm` crate.
#[cfg(feature = "software-crypto")]
pub struct AesGcmEncryptor {
    key: AesGcmKey,
}

#[cfg(feature = "software-crypto")]
impl AesGcmEncryptor {
    /// Create a new AES-128-GCM encryptor (16-byte key)
    pub fn aes128(key: &[u8]) -> CryptoResult<Self> {
        if key.len() != 16 {
            return Err(CryptoError::InvalidKeyFormat(
                "AES-128-GCM key must be 16 bytes".to_string(),
            ));
        }

        let key_array: [u8; 16] = key.try_into().unwrap();
        let cipher = Aes128Gcm::new(&key_array.into());

        Ok(Self {
            key: AesGcmKey::Aes128(Box::new(cipher)),
        })
    }

    /// Create a new AES-256-GCM encryptor (32-byte key)
    pub fn aes256(key: &[u8]) -> CryptoResult<Self> {
        if key.len() != 32 {
            return Err(CryptoError::InvalidKeyFormat(
                "AES-256-GCM key must be 32 bytes".to_string(),
            ));
        }

        let key_array: [u8; 32] = key.try_into().unwrap();
        let cipher = Aes256Gcm::new(&key_array.into());

        Ok(Self {
            key: AesGcmKey::Aes256(Box::new(cipher)),
        })
    }

    /// Create an encryptor from key bytes, auto-detecting key size
    pub fn from_bytes(key: &[u8]) -> CryptoResult<Self> {
        match key.len() {
            16 => Self::aes128(key),
            32 => Self::aes256(key),
            _ => Err(CryptoError::InvalidKeyFormat(
                "AES-GCM key must be 16 or 32 bytes".to_string(),
            )),
        }
    }

    /// Generate a random 12-byte nonce
    pub fn generate_nonce() -> [u8; 12] {
        use rand::RngCore;
        let mut nonce = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce);
        nonce
    }
}

#[cfg(feature = "software-crypto")]
impl Encryptor for AesGcmEncryptor {
    fn encrypt(
        &self,
        algorithm: iana::Algorithm,
        _key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        plaintext: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        // Verify algorithm matches key type
        match (&self.key, algorithm) {
            (AesGcmKey::Aes128(_), iana::Algorithm::A128GCM) => {}
            (AesGcmKey::Aes256(_), iana::Algorithm::A256GCM) => {}
            _ => {
                return Err(CryptoError::UnsupportedAlgorithm(format!(
                    "Algorithm {:?} does not match key type",
                    algorithm
                )));
            }
        }

        if nonce.len() != 12 {
            return Err(CryptoError::Other(format!(
                "AES-GCM nonce must be 12 bytes, got {}",
                nonce.len()
            )));
        }

        // Convert validated slice to fixed-size array for Nonce
        let nonce_array: &[u8; 12] = nonce.try_into().unwrap();
        let payload = Payload {
            msg: plaintext,
            aad,
        };

        match &self.key {
            AesGcmKey::Aes128(cipher) => cipher
                .encrypt(nonce_array.into(), payload)
                .map_err(|_| CryptoError::Other("AES-128-GCM encryption failed".to_string())),
            AesGcmKey::Aes256(cipher) => cipher
                .encrypt(nonce_array.into(), payload)
                .map_err(|_| CryptoError::Other("AES-256-GCM encryption failed".to_string())),
        }
    }
}

#[cfg(all(test, feature = "software-crypto"))]
mod tests {
    use super::*;

    #[test]
    fn test_aes128_gcm_roundtrip() {
        let key = [0u8; 16];
        let nonce = [1u8; 12];
        let aad = b"additional authenticated data";
        let plaintext = b"secret message";

        let encryptor = AesGcmEncryptor::aes128(&key).unwrap();
        let decryptor = AesGcmDecryptor::aes128(&key).unwrap();

        let ciphertext = encryptor
            .encrypt(iana::Algorithm::A128GCM, None, &nonce, aad, plaintext)
            .unwrap();

        // Ciphertext should be plaintext + 16 byte auth tag
        assert_eq!(ciphertext.len(), plaintext.len() + 16);

        let decrypted = decryptor
            .decrypt(iana::Algorithm::A128GCM, None, &nonce, aad, &ciphertext)
            .unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_aes256_gcm_roundtrip() {
        let key = [0u8; 32];
        let nonce = [2u8; 12];
        let aad = b"more aad";
        let plaintext = b"another secret message";

        let encryptor = AesGcmEncryptor::aes256(&key).unwrap();
        let decryptor = AesGcmDecryptor::aes256(&key).unwrap();

        let ciphertext = encryptor
            .encrypt(iana::Algorithm::A256GCM, None, &nonce, aad, plaintext)
            .unwrap();

        let decrypted = decryptor
            .decrypt(iana::Algorithm::A256GCM, None, &nonce, aad, &ciphertext)
            .unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_aes_gcm_tampered_ciphertext() {
        let key = [0u8; 16];
        let nonce = [1u8; 12];
        let aad = b"aad";
        let plaintext = b"secret";

        let encryptor = AesGcmEncryptor::aes128(&key).unwrap();
        let decryptor = AesGcmDecryptor::aes128(&key).unwrap();

        let mut ciphertext = encryptor
            .encrypt(iana::Algorithm::A128GCM, None, &nonce, aad, plaintext)
            .unwrap();

        // Tamper with ciphertext
        ciphertext[0] ^= 0xFF;

        let result = decryptor.decrypt(iana::Algorithm::A128GCM, None, &nonce, aad, &ciphertext);
        assert!(result.is_err());
    }

    #[test]
    fn test_aes_gcm_wrong_aad() {
        let key = [0u8; 16];
        let nonce = [1u8; 12];
        let aad = b"correct aad";
        let wrong_aad = b"wrong aad";
        let plaintext = b"secret";

        let encryptor = AesGcmEncryptor::aes128(&key).unwrap();
        let decryptor = AesGcmDecryptor::aes128(&key).unwrap();

        let ciphertext = encryptor
            .encrypt(iana::Algorithm::A128GCM, None, &nonce, aad, plaintext)
            .unwrap();

        let result = decryptor.decrypt(
            iana::Algorithm::A128GCM,
            None,
            &nonce,
            wrong_aad,
            &ciphertext,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_aes_gcm_wrong_algorithm() {
        let key = [0u8; 16];
        let decryptor = AesGcmDecryptor::aes128(&key).unwrap();

        // Try to use A256GCM with 128-bit key
        let result = decryptor.decrypt(iana::Algorithm::A256GCM, None, &[0u8; 12], &[], &[]);
        assert!(matches!(result, Err(CryptoError::UnsupportedAlgorithm(_))));
    }

    #[test]
    fn test_aes_gcm_invalid_nonce_length() {
        let key = [0u8; 16];
        let decryptor = AesGcmDecryptor::aes128(&key).unwrap();

        // Wrong nonce length
        let result = decryptor.decrypt(iana::Algorithm::A128GCM, None, &[0u8; 8], &[], &[]);
        assert!(matches!(result, Err(CryptoError::DecryptionFailed(_))));
    }

    #[test]
    fn test_generate_nonce() {
        let nonce1 = AesGcmEncryptor::generate_nonce();
        let nonce2 = AesGcmEncryptor::generate_nonce();

        assert_eq!(nonce1.len(), 12);
        assert_eq!(nonce2.len(), 12);
        assert_ne!(nonce1, nonce2); // Should be random
    }
}
