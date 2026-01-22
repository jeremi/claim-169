#[cfg(feature = "software-crypto")]
use coset::iana;
#[cfg(feature = "software-crypto")]
use ed25519_dalek::{Signature, Signer as DalekSigner, SigningKey, Verifier, VerifyingKey};

#[cfg(feature = "software-crypto")]
use crate::crypto::traits::{SignatureVerifier, Signer};
#[cfg(feature = "software-crypto")]
use crate::error::{CryptoError, CryptoResult};

/// Ed25519 signature verifier using ed25519-dalek
#[cfg(feature = "software-crypto")]
pub struct Ed25519Verifier {
    public_key: VerifyingKey,
}

/// All-zeros key - identity point, provides no security
const ED25519_WEAK_KEY_ZEROS: [u8; 32] = [0u8; 32];

/// Small order points that should be rejected
/// These are the 8 points of small order on Curve25519
const ED25519_SMALL_ORDER_POINTS: [[u8; 32]; 8] = [
    // Identity point (neutral element)
    [
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
    // Point of order 2
    [
        0xec, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0x7f,
    ],
    // Points of order 4
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x80,
    ],
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
    // Points of order 8
    [
        0xc7, 0x17, 0x6a, 0x70, 0x3d, 0x4d, 0xd8, 0x4f, 0xba, 0x3c, 0x0b, 0x76, 0x0d, 0x10, 0x67,
        0x0f, 0x2a, 0x20, 0x53, 0xfa, 0x2c, 0x39, 0xcc, 0xc6, 0x4e, 0xc7, 0xfd, 0x77, 0x92, 0xac,
        0x03, 0x7a,
    ],
    [
        0xc7, 0x17, 0x6a, 0x70, 0x3d, 0x4d, 0xd8, 0x4f, 0xba, 0x3c, 0x0b, 0x76, 0x0d, 0x10, 0x67,
        0x0f, 0x2a, 0x20, 0x53, 0xfa, 0x2c, 0x39, 0xcc, 0xc6, 0x4e, 0xc7, 0xfd, 0x77, 0x92, 0xac,
        0x03, 0xfa,
    ],
    [
        0x26, 0xe8, 0x95, 0x8f, 0xc2, 0xb2, 0x27, 0xb0, 0x45, 0xc3, 0xf4, 0x89, 0xf2, 0xef, 0x98,
        0xf0, 0xd5, 0xdf, 0xac, 0x05, 0xd3, 0xc6, 0x33, 0x39, 0xb1, 0x38, 0x02, 0x88, 0x6d, 0x53,
        0xfc, 0x05,
    ],
    [
        0x26, 0xe8, 0x95, 0x8f, 0xc2, 0xb2, 0x27, 0xb0, 0x45, 0xc3, 0xf4, 0x89, 0xf2, 0xef, 0x98,
        0xf0, 0xd5, 0xdf, 0xac, 0x05, 0xd3, 0xc6, 0x33, 0x39, 0xb1, 0x38, 0x02, 0x88, 0x6d, 0x53,
        0xfc, 0x85,
    ],
];

#[cfg(feature = "software-crypto")]
impl Ed25519Verifier {
    /// Create a new verifier from raw public key bytes (32 bytes)
    ///
    /// Rejects weak keys including all-zeros and small-order points.
    pub fn from_bytes(bytes: &[u8]) -> CryptoResult<Self> {
        let bytes: [u8; 32] = bytes.try_into().map_err(|_| {
            CryptoError::InvalidKeyFormat("Ed25519 public key must be 32 bytes".to_string())
        })?;

        // Reject all-zeros key
        if bytes == ED25519_WEAK_KEY_ZEROS {
            return Err(CryptoError::InvalidKeyFormat(
                "weak key rejected: all-zeros public key".to_string(),
            ));
        }

        // Reject small-order points (potential small subgroup attack)
        for weak_point in &ED25519_SMALL_ORDER_POINTS {
            if bytes == *weak_point {
                return Err(CryptoError::InvalidKeyFormat(
                    "weak key rejected: small-order point".to_string(),
                ));
            }
        }

        let public_key = VerifyingKey::from_bytes(&bytes)
            .map_err(|e| CryptoError::InvalidKeyFormat(e.to_string()))?;

        Ok(Self { public_key })
    }

    /// Create a new verifier from a PEM-encoded public key
    ///
    /// Supports both SPKI (SubjectPublicKeyInfo) format with "BEGIN PUBLIC KEY" headers
    /// and raw base64-encoded 32-byte keys.
    ///
    /// Rejects weak keys including all-zeros and small-order points.
    pub fn from_pem(pem: &str) -> CryptoResult<Self> {
        use ed25519_dalek::pkcs8::DecodePublicKey;

        let pem = pem.trim();

        // Try to parse as SPKI PEM using proper ASN.1 parsing
        if pem.contains("BEGIN PUBLIC KEY") {
            let public_key = VerifyingKey::from_public_key_pem(pem)
                .map_err(|e| CryptoError::InvalidKeyFormat(format!("Invalid SPKI PEM: {}", e)))?;

            // Validate against weak keys (same checks as from_bytes)
            let bytes = public_key.to_bytes();

            if bytes == ED25519_WEAK_KEY_ZEROS {
                return Err(CryptoError::InvalidKeyFormat(
                    "weak key rejected: all-zeros public key".to_string(),
                ));
            }

            for weak_point in &ED25519_SMALL_ORDER_POINTS {
                if bytes == *weak_point {
                    return Err(CryptoError::InvalidKeyFormat(
                        "weak key rejected: small-order point".to_string(),
                    ));
                }
            }

            return Ok(Self { public_key });
        }

        // Fallback: try to decode as raw base64 32-byte key
        use base64::Engine;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(pem)
            .map_err(|e| CryptoError::InvalidKeyFormat(format!("Invalid base64: {}", e)))?;

        Self::from_bytes(&bytes)
    }
}

#[cfg(feature = "software-crypto")]
impl SignatureVerifier for Ed25519Verifier {
    fn verify(
        &self,
        algorithm: iana::Algorithm,
        _key_id: Option<&[u8]>,
        data: &[u8],
        signature: &[u8],
    ) -> CryptoResult<()> {
        // Verify algorithm is EdDSA
        if algorithm != iana::Algorithm::EdDSA {
            return Err(CryptoError::UnsupportedAlgorithm(format!(
                "{:?}",
                algorithm
            )));
        }

        let signature: [u8; 64] = signature
            .try_into()
            .map_err(|_| CryptoError::VerificationFailed)?;

        let signature = Signature::from_bytes(&signature);

        self.public_key
            .verify(data, &signature)
            .map_err(|_| CryptoError::VerificationFailed)
    }
}

/// Ed25519 signer using ed25519-dalek (for test vector generation)
///
/// The underlying `SigningKey` implements `ZeroizeOnDrop` (when the zeroize feature
/// is enabled on ed25519-dalek), which securely clears the private key from memory
/// when the signer is dropped.
#[cfg(feature = "software-crypto")]
pub struct Ed25519Signer {
    signing_key: SigningKey,
}

#[cfg(feature = "software-crypto")]
impl Ed25519Signer {
    /// Create a new signer from raw private key bytes (32 bytes)
    pub fn from_bytes(bytes: &[u8]) -> CryptoResult<Self> {
        let bytes: [u8; 32] = bytes.try_into().map_err(|_| {
            CryptoError::InvalidKeyFormat("Ed25519 private key must be 32 bytes".to_string())
        })?;

        let signing_key = SigningKey::from_bytes(&bytes);
        Ok(Self { signing_key })
    }

    /// Generate a new random signing key
    pub fn generate() -> Self {
        use rand::rngs::OsRng;
        let signing_key = SigningKey::generate(&mut OsRng);
        Self { signing_key }
    }

    /// Get the verifying (public) key
    pub fn verifying_key(&self) -> Ed25519Verifier {
        Ed25519Verifier {
            public_key: self.signing_key.verifying_key(),
        }
    }

    /// Get the public key bytes
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.signing_key.verifying_key().to_bytes()
    }
}

#[cfg(feature = "software-crypto")]
impl Signer for Ed25519Signer {
    fn sign(
        &self,
        algorithm: iana::Algorithm,
        _key_id: Option<&[u8]>,
        data: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        if algorithm != iana::Algorithm::EdDSA {
            return Err(CryptoError::UnsupportedAlgorithm(format!(
                "{:?}",
                algorithm
            )));
        }

        let signature = self.signing_key.sign(data);
        Ok(signature.to_bytes().to_vec())
    }
}

#[cfg(all(test, feature = "software-crypto"))]
mod tests {
    use super::*;

    #[test]
    fn test_ed25519_sign_verify() {
        let signer = Ed25519Signer::generate();
        let verifier = signer.verifying_key();

        let data = b"test message to sign";
        let signature = signer.sign(iana::Algorithm::EdDSA, None, data).unwrap();

        assert!(verifier
            .verify(iana::Algorithm::EdDSA, None, data, &signature)
            .is_ok());
    }

    #[test]
    fn test_ed25519_verify_wrong_data() {
        let signer = Ed25519Signer::generate();
        let verifier = signer.verifying_key();

        let data = b"original message";
        let wrong_data = b"tampered message";
        let signature = signer.sign(iana::Algorithm::EdDSA, None, data).unwrap();

        assert!(verifier
            .verify(iana::Algorithm::EdDSA, None, wrong_data, &signature)
            .is_err());
    }

    #[test]
    fn test_ed25519_wrong_algorithm() {
        let signer = Ed25519Signer::generate();
        let verifier = signer.verifying_key();

        let data = b"test message";
        let signature = signer.sign(iana::Algorithm::EdDSA, None, data).unwrap();

        // Try to verify with wrong algorithm
        let result = verifier.verify(iana::Algorithm::ES256, None, data, &signature);
        assert!(matches!(result, Err(CryptoError::UnsupportedAlgorithm(_))));
    }

    #[test]
    fn test_ed25519_from_bytes() {
        let signer = Ed25519Signer::generate();
        let public_bytes = signer.public_key_bytes();

        let verifier = Ed25519Verifier::from_bytes(&public_bytes).unwrap();

        let data = b"test data";
        let signature = signer.sign(iana::Algorithm::EdDSA, None, data).unwrap();

        assert!(verifier
            .verify(iana::Algorithm::EdDSA, None, data, &signature)
            .is_ok());
    }

    #[test]
    fn test_ed25519_invalid_key_length() {
        let result = Ed25519Verifier::from_bytes(&[0u8; 16]);
        assert!(matches!(result, Err(CryptoError::InvalidKeyFormat(_))));
    }

    #[test]
    fn test_ed25519_from_pem_spki() {
        use ed25519_dalek::pkcs8::{spki::der::pem::LineEnding, EncodePublicKey};

        let signer = Ed25519Signer::generate();
        let pem = signer
            .signing_key
            .verifying_key()
            .to_public_key_pem(LineEnding::LF)
            .expect("PEM encoding should not fail");

        // Verify PEM has correct SPKI format
        assert!(pem.contains("-----BEGIN PUBLIC KEY-----"));
        assert!(pem.contains("-----END PUBLIC KEY-----"));

        // Parse from PEM and verify signature
        let verifier = Ed25519Verifier::from_pem(&pem).unwrap();

        let data = b"test data for PEM verification";
        let signature = signer.sign(iana::Algorithm::EdDSA, None, data).unwrap();

        assert!(verifier
            .verify(iana::Algorithm::EdDSA, None, data, &signature)
            .is_ok());
    }

    #[test]
    fn test_ed25519_from_pem_invalid() {
        // Invalid PEM should fail
        let result = Ed25519Verifier::from_pem("not a valid PEM");
        assert!(result.is_err());

        // Invalid SPKI PEM should fail with clear error
        let invalid_spki =
            "-----BEGIN PUBLIC KEY-----\nnotvalidbase64!!!\n-----END PUBLIC KEY-----";
        let result = Ed25519Verifier::from_pem(invalid_spki);
        assert!(result.is_err());
    }

    #[test]
    fn test_ed25519_from_pem_raw_base64() {
        // Test fallback to raw base64-encoded 32-byte key
        let signer = Ed25519Signer::generate();
        let public_bytes = signer.public_key_bytes();

        use base64::Engine;
        let raw_base64 = base64::engine::general_purpose::STANDARD.encode(public_bytes);

        let verifier = Ed25519Verifier::from_pem(&raw_base64).unwrap();

        let data = b"test data";
        let signature = signer.sign(iana::Algorithm::EdDSA, None, data).unwrap();

        assert!(verifier
            .verify(iana::Algorithm::EdDSA, None, data, &signature)
            .is_ok());
    }

    #[test]
    fn test_ed25519_rejects_all_zeros_key() {
        let zero_key = [0u8; 32];
        let result = Ed25519Verifier::from_bytes(&zero_key);

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
    fn test_ed25519_rejects_small_order_points() {
        // Test identity point (order 1)
        let identity = [
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];

        let result = Ed25519Verifier::from_bytes(&identity);
        assert!(result.is_err());
        match result {
            Err(CryptoError::InvalidKeyFormat(msg)) => {
                assert!(
                    msg.contains("small-order"),
                    "Error should mention small-order: {}",
                    msg
                );
            }
            _ => panic!("Expected InvalidKeyFormat error for identity point"),
        }

        // Test point of order 2
        let order_2 = [
            0xec, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0x7f,
        ];

        let result = Ed25519Verifier::from_bytes(&order_2);
        assert!(result.is_err());
    }
}
