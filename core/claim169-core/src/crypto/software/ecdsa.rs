#[cfg(feature = "software-crypto")]
use coset::iana;
#[cfg(feature = "software-crypto")]
use p256::{
    ecdsa::{
        signature::{Signer as EcdsaSigner, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    PublicKey, SecretKey,
};

#[cfg(feature = "software-crypto")]
use crate::crypto::traits::{SignatureVerifier, Signer};
#[cfg(feature = "software-crypto")]
use crate::error::{CryptoError, CryptoResult};

/// ECDSA P-256 signature verifier
#[cfg(feature = "software-crypto")]
pub struct EcdsaP256Verifier {
    verifying_key: VerifyingKey,
}

/// Check if SEC1 bytes represent the point at infinity or other weak points
#[cfg(feature = "software-crypto")]
fn is_weak_p256_key(bytes: &[u8]) -> Option<&'static str> {
    // Check for obvious weak patterns
    match bytes.len() {
        // Uncompressed format: 0x04 || x || y (65 bytes)
        65 => {
            if bytes[0] != 0x04 {
                return None; // Let the library handle format errors
            }
            // All zeros after prefix = weak
            if bytes[1..].iter().all(|&b| b == 0) {
                return Some("all-zeros coordinates");
            }
        }
        // Compressed format: 0x02/0x03 || x (33 bytes)
        33 => {
            if bytes[0] != 0x02 && bytes[0] != 0x03 {
                return None;
            }
            // All zeros x-coordinate = weak
            if bytes[1..].iter().all(|&b| b == 0) {
                return Some("zero x-coordinate");
            }
        }
        _ => {}
    }
    None
}

#[cfg(feature = "software-crypto")]
impl EcdsaP256Verifier {
    /// Create a verifier from uncompressed public key bytes (65 bytes starting with 0x04)
    ///
    /// Rejects weak keys including identity point and all-zeros coordinates.
    pub fn from_uncompressed_bytes(bytes: &[u8]) -> CryptoResult<Self> {
        // Check for weak keys before parsing
        if let Some(reason) = is_weak_p256_key(bytes) {
            return Err(CryptoError::InvalidKeyFormat(format!(
                "weak key rejected: {}",
                reason
            )));
        }

        let public_key = PublicKey::from_sec1_bytes(bytes)
            .map_err(|e| CryptoError::InvalidKeyFormat(e.to_string()))?;

        let verifying_key = VerifyingKey::from(public_key);
        Ok(Self { verifying_key })
    }

    /// Create a verifier from compressed public key bytes (33 bytes)
    ///
    /// Rejects weak keys including identity point and all-zeros coordinates.
    pub fn from_compressed_bytes(bytes: &[u8]) -> CryptoResult<Self> {
        Self::from_sec1_bytes(bytes) // p256 handles both formats
    }

    /// Create a verifier from SEC1-encoded bytes (compressed or uncompressed)
    ///
    /// Rejects weak keys including identity point and all-zeros coordinates.
    pub fn from_sec1_bytes(bytes: &[u8]) -> CryptoResult<Self> {
        // Check for weak keys before parsing
        if let Some(reason) = is_weak_p256_key(bytes) {
            return Err(CryptoError::InvalidKeyFormat(format!(
                "weak key rejected: {}",
                reason
            )));
        }

        let public_key = PublicKey::from_sec1_bytes(bytes)
            .map_err(|e| CryptoError::InvalidKeyFormat(e.to_string()))?;

        let verifying_key = VerifyingKey::from(public_key);
        Ok(Self { verifying_key })
    }

    /// Create a verifier from a PEM-encoded public key
    ///
    /// Supports both SPKI (SubjectPublicKeyInfo) format with "BEGIN PUBLIC KEY" headers
    /// and raw SEC1 encoded keys.
    ///
    /// Rejects weak keys including identity point and all-zeros coordinates.
    pub fn from_pem(pem: &str) -> CryptoResult<Self> {
        use p256::pkcs8::DecodePublicKey;
        use p256::EncodedPoint;

        let pem = pem.trim();

        // Try to parse as SPKI PEM using proper ASN.1 parsing
        if pem.contains("BEGIN PUBLIC KEY") {
            let public_key = PublicKey::from_public_key_pem(pem)
                .map_err(|e| CryptoError::InvalidKeyFormat(format!("Invalid SPKI PEM: {}", e)))?;

            // Validate against weak keys (same checks as from_sec1_bytes)
            let encoded_point: EncodedPoint = public_key.into();
            let sec1_bytes = encoded_point.as_bytes();

            if let Some(reason) = is_weak_p256_key(sec1_bytes) {
                return Err(CryptoError::InvalidKeyFormat(format!(
                    "weak key rejected: {}",
                    reason
                )));
            }

            let verifying_key = VerifyingKey::from(public_key);
            return Ok(Self { verifying_key });
        }

        // Fallback: try to decode as raw base64 SEC1 bytes
        use base64::Engine;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(pem)
            .map_err(|e| CryptoError::InvalidKeyFormat(format!("Invalid base64: {}", e)))?;

        Self::from_sec1_bytes(&bytes)
    }
}

#[cfg(feature = "software-crypto")]
impl SignatureVerifier for EcdsaP256Verifier {
    fn verify(
        &self,
        algorithm: iana::Algorithm,
        _key_id: Option<&[u8]>,
        data: &[u8],
        signature: &[u8],
    ) -> CryptoResult<()> {
        // Verify algorithm is ES256
        if algorithm != iana::Algorithm::ES256 {
            return Err(CryptoError::UnsupportedAlgorithm(format!(
                "{:?}",
                algorithm
            )));
        }

        // COSE signatures are in (r || s) format, 64 bytes total
        let signature = Signature::from_slice(signature)
            .map_err(|e| CryptoError::Other(format!("Invalid signature format: {}", e)))?;

        self.verifying_key
            .verify(data, &signature)
            .map_err(|_| CryptoError::VerificationFailed)
    }
}

/// ECDSA P-256 signer (for test vector generation)
///
/// Note: The p256 crate uses the elliptic-curve framework which handles
/// sensitive scalar data with care. For enhanced security, consider using
/// HSM-backed implementations via the `Signer` trait.
#[cfg(feature = "software-crypto")]
pub struct EcdsaP256Signer {
    signing_key: SigningKey,
}

#[cfg(feature = "software-crypto")]
impl EcdsaP256Signer {
    /// Create a signer from raw private key bytes (32 bytes)
    pub fn from_bytes(bytes: &[u8]) -> CryptoResult<Self> {
        let secret_key = SecretKey::from_slice(bytes)
            .map_err(|e| CryptoError::InvalidKeyFormat(e.to_string()))?;

        let signing_key = SigningKey::from(secret_key);
        Ok(Self { signing_key })
    }

    /// Generate a new random signing key
    pub fn generate() -> Self {
        use rand::rngs::OsRng;
        let signing_key = SigningKey::random(&mut OsRng);
        Self { signing_key }
    }

    /// Get the verifier for this signer
    pub fn verifying_key(&self) -> EcdsaP256Verifier {
        EcdsaP256Verifier {
            verifying_key: *self.signing_key.verifying_key(),
        }
    }

    /// Get the public key in SEC1 uncompressed format (65 bytes)
    pub fn public_key_uncompressed(&self) -> Vec<u8> {
        use p256::EncodedPoint;
        let point: EncodedPoint = self.signing_key.verifying_key().into();
        point.as_bytes().to_vec()
    }

    /// Get the public key in SEC1 compressed format (33 bytes)
    #[allow(dead_code)]
    pub fn public_key_compressed(&self) -> Vec<u8> {
        use p256::EncodedPoint;
        let point: EncodedPoint = self.signing_key.verifying_key().into();
        point.compress().as_bytes().to_vec()
    }

    /// Get the public key in SPKI PEM format
    pub fn public_key_pem(&self) -> String {
        use p256::pkcs8::EncodePublicKey;
        self.signing_key
            .verifying_key()
            .to_public_key_pem(p256::pkcs8::LineEnding::LF)
            .expect("PEM encoding should not fail")
    }
}

#[cfg(feature = "software-crypto")]
impl Signer for EcdsaP256Signer {
    fn sign(
        &self,
        algorithm: iana::Algorithm,
        _key_id: Option<&[u8]>,
        data: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        if algorithm != iana::Algorithm::ES256 {
            return Err(CryptoError::UnsupportedAlgorithm(format!(
                "{:?}",
                algorithm
            )));
        }

        let signature: Signature = self.signing_key.sign(data);
        Ok(signature.to_bytes().to_vec())
    }
}

#[cfg(all(test, feature = "software-crypto"))]
mod tests {
    use super::*;

    #[test]
    fn test_ecdsa_p256_sign_verify() {
        let signer = EcdsaP256Signer::generate();
        let verifier = signer.verifying_key();

        let data = b"test message to sign with ECDSA P-256";
        let signature = signer.sign(iana::Algorithm::ES256, None, data).unwrap();

        // ECDSA P-256 signatures are 64 bytes
        assert_eq!(signature.len(), 64);

        assert!(verifier
            .verify(iana::Algorithm::ES256, None, data, &signature)
            .is_ok());
    }

    #[test]
    fn test_ecdsa_p256_verify_wrong_data() {
        let signer = EcdsaP256Signer::generate();
        let verifier = signer.verifying_key();

        let data = b"original message";
        let wrong_data = b"tampered message";
        let signature = signer.sign(iana::Algorithm::ES256, None, data).unwrap();

        assert!(verifier
            .verify(iana::Algorithm::ES256, None, wrong_data, &signature)
            .is_err());
    }

    #[test]
    fn test_ecdsa_p256_wrong_algorithm() {
        let signer = EcdsaP256Signer::generate();
        let verifier = signer.verifying_key();

        let data = b"test message";
        let signature = signer.sign(iana::Algorithm::ES256, None, data).unwrap();

        // Try to verify with wrong algorithm
        let result = verifier.verify(iana::Algorithm::EdDSA, None, data, &signature);
        assert!(matches!(result, Err(CryptoError::UnsupportedAlgorithm(_))));
    }

    #[test]
    fn test_ecdsa_p256_from_sec1_bytes() {
        let signer = EcdsaP256Signer::generate();

        // Test uncompressed
        let uncompressed = signer.public_key_uncompressed();
        assert_eq!(uncompressed.len(), 65);
        assert_eq!(uncompressed[0], 0x04); // Uncompressed point marker

        let verifier = EcdsaP256Verifier::from_sec1_bytes(&uncompressed).unwrap();

        let data = b"test data";
        let signature = signer.sign(iana::Algorithm::ES256, None, data).unwrap();

        assert!(verifier
            .verify(iana::Algorithm::ES256, None, data, &signature)
            .is_ok());

        // Test compressed
        let compressed = signer.public_key_compressed();
        assert_eq!(compressed.len(), 33);
        assert!(compressed[0] == 0x02 || compressed[0] == 0x03);

        let verifier = EcdsaP256Verifier::from_sec1_bytes(&compressed).unwrap();
        assert!(verifier
            .verify(iana::Algorithm::ES256, None, data, &signature)
            .is_ok());
    }

    #[test]
    fn test_ecdsa_p256_from_pem_spki() {
        let signer = EcdsaP256Signer::generate();
        let pem = signer.public_key_pem();

        // Verify PEM has correct SPKI format
        assert!(pem.contains("-----BEGIN PUBLIC KEY-----"));
        assert!(pem.contains("-----END PUBLIC KEY-----"));

        // Parse from PEM and verify signature
        let verifier = EcdsaP256Verifier::from_pem(&pem).unwrap();

        let data = b"test data for PEM verification";
        let signature = signer.sign(iana::Algorithm::ES256, None, data).unwrap();

        assert!(verifier
            .verify(iana::Algorithm::ES256, None, data, &signature)
            .is_ok());
    }

    #[test]
    fn test_ecdsa_p256_from_pem_invalid() {
        // Invalid PEM should fail
        let result = EcdsaP256Verifier::from_pem("not a valid PEM");
        assert!(result.is_err());

        // Invalid SPKI PEM should fail with clear error
        let invalid_spki =
            "-----BEGIN PUBLIC KEY-----\nnotvalidbase64!!!\n-----END PUBLIC KEY-----";
        let result = EcdsaP256Verifier::from_pem(invalid_spki);
        assert!(result.is_err());
    }

    #[test]
    fn test_ecdsa_p256_rejects_all_zeros_uncompressed() {
        // Uncompressed format with all-zero coordinates
        let mut weak_key = vec![0x04]; // Uncompressed marker
        weak_key.extend_from_slice(&[0u8; 64]); // Zero x and y coordinates

        let result = EcdsaP256Verifier::from_uncompressed_bytes(&weak_key);

        assert!(result.is_err());
        match result {
            Err(CryptoError::InvalidKeyFormat(msg)) => {
                assert!(
                    msg.contains("weak key"),
                    "Error should mention weak key: {}",
                    msg
                );
            }
            _ => panic!("Expected InvalidKeyFormat error"),
        }
    }

    #[test]
    fn test_ecdsa_p256_rejects_zero_x_compressed() {
        // Compressed format with zero x-coordinate
        let mut weak_key = vec![0x02]; // Compressed marker (even y)
        weak_key.extend_from_slice(&[0u8; 32]); // Zero x coordinate

        let result = EcdsaP256Verifier::from_compressed_bytes(&weak_key);

        assert!(result.is_err());
        match result {
            Err(CryptoError::InvalidKeyFormat(msg)) => {
                assert!(
                    msg.contains("weak key") || msg.contains("zero"),
                    "Error should mention weak key or zero: {}",
                    msg
                );
            }
            _ => panic!("Expected InvalidKeyFormat error"),
        }
    }
}
