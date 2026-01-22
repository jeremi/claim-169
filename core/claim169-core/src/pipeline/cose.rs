use coset::{iana, CborSerializable, CoseEncrypt0, CoseSign1, Header, TaggedCborSerializable};

use crate::crypto::traits::{Decryptor, KeyResolver, SignatureVerifier};
use crate::error::{Claim169Error, CryptoError, Result};
use crate::model::VerificationStatus;

/// Result of parsing and optionally verifying/decrypting a COSE structure
#[derive(Debug)]
pub struct CoseResult {
    /// The CWT payload bytes (after signature verification and/or decryption)
    pub payload: Vec<u8>,

    /// Verification status
    pub verification_status: VerificationStatus,

    /// COSE algorithm used (if available)
    pub algorithm: Option<iana::Algorithm>,

    /// Key ID from COSE header (if present)
    pub key_id: Option<Vec<u8>>,
}

/// Detected COSE message type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoseType {
    Sign1,
    Encrypt0,
}

/// Parse COSE data and detect its type
///
/// MOSIP Claim 169 uses CWT (CBOR Web Token) which is wrapped in COSE.
/// The structure can be COSE_Sign1 (signed) or COSE_Encrypt0 (encrypted).
pub fn parse_and_verify(
    data: &[u8],
    verifier: Option<&dyn SignatureVerifier>,
    decryptor: Option<&dyn Decryptor>,
) -> Result<CoseResult> {
    // Try to detect COSE type by parsing
    // COSE_Sign1 tag is 18, COSE_Encrypt0 tag is 16

    // First try COSE_Sign1 (most common case)
    if let Ok(sign1) = CoseSign1::from_tagged_slice(data) {
        return process_sign1(sign1, verifier);
    }

    // Try COSE_Encrypt0
    if let Ok(encrypt0) = CoseEncrypt0::from_tagged_slice(data) {
        return process_encrypt0(encrypt0, decryptor, verifier);
    }

    // Also try without tags (some implementations omit them)
    if let Ok(sign1) = CoseSign1::from_slice(data) {
        return process_sign1(sign1, verifier);
    }

    if let Ok(encrypt0) = CoseEncrypt0::from_slice(data) {
        return process_encrypt0(encrypt0, decryptor, verifier);
    }

    Err(Claim169Error::CoseParse(
        "data is not a valid COSE_Sign1 or COSE_Encrypt0 structure".to_string(),
    ))
}

/// Parse COSE data using a KeyResolver to look up verifiers and decryptors
///
/// This function extracts the key_id and algorithm from the COSE headers,
/// then uses the resolver to obtain the appropriate verifier or decryptor.
pub fn parse_with_resolver<R: KeyResolver>(data: &[u8], resolver: &R) -> Result<CoseResult> {
    // First try COSE_Sign1 (most common case)
    if let Ok(sign1) = CoseSign1::from_tagged_slice(data) {
        return process_sign1_with_resolver(sign1, resolver);
    }

    // Try COSE_Encrypt0
    if let Ok(encrypt0) = CoseEncrypt0::from_tagged_slice(data) {
        return process_encrypt0_with_resolver(encrypt0, resolver);
    }

    // Also try without tags
    if let Ok(sign1) = CoseSign1::from_slice(data) {
        return process_sign1_with_resolver(sign1, resolver);
    }

    if let Ok(encrypt0) = CoseEncrypt0::from_slice(data) {
        return process_encrypt0_with_resolver(encrypt0, resolver);
    }

    Err(Claim169Error::CoseParse(
        "data is not a valid COSE_Sign1 or COSE_Encrypt0 structure".to_string(),
    ))
}

/// Process a COSE_Sign1 message using a KeyResolver
fn process_sign1_with_resolver<R: KeyResolver>(
    sign1: CoseSign1,
    resolver: &R,
) -> Result<CoseResult> {
    let algorithm = get_algorithm(&sign1.protected.header);
    let key_id = get_key_id(&sign1.protected.header, &sign1.unprotected);

    let payload = sign1
        .payload
        .clone()
        .ok_or_else(|| Claim169Error::CoseParse("COSE_Sign1 has no payload".to_string()))?;

    // Require explicit algorithm
    let alg = algorithm.ok_or_else(|| {
        Claim169Error::CoseParse(
            "COSE_Sign1 missing required algorithm in protected header".to_string(),
        )
    })?;

    // Resolve verifier
    let verifier = resolver
        .resolve_verifier(key_id.as_deref(), alg)
        .map_err(|e| Claim169Error::Crypto(e.to_string()))?;

    // Build the Sig_structure for verification
    let sig_structure = sign1.tbs_data(&[]);

    let verification_status =
        match verifier.verify(alg, key_id.as_deref(), &sig_structure, &sign1.signature) {
            Ok(()) => VerificationStatus::Verified,
            Err(CryptoError::VerificationFailed) => VerificationStatus::Failed,
            Err(e) => return Err(e.into()),
        };

    Ok(CoseResult {
        payload,
        verification_status,
        algorithm,
        key_id,
    })
}

/// Process a COSE_Encrypt0 message using a KeyResolver
fn process_encrypt0_with_resolver<R: KeyResolver>(
    encrypt0: CoseEncrypt0,
    resolver: &R,
) -> Result<CoseResult> {
    let algorithm = get_algorithm(&encrypt0.protected.header);
    let key_id = get_key_id(&encrypt0.protected.header, &encrypt0.unprotected);

    // Require explicit algorithm
    let alg = algorithm.ok_or_else(|| {
        Claim169Error::CoseParse(
            "COSE_Encrypt0 missing required algorithm in protected header".to_string(),
        )
    })?;

    // Resolve decryptor
    let decryptor = resolver
        .resolve_decryptor(key_id.as_deref(), alg)
        .map_err(|e| Claim169Error::Crypto(e.to_string()))?;

    // Get IV/nonce from header
    let nonce = if !encrypt0.unprotected.iv.is_empty() {
        encrypt0.unprotected.iv.clone()
    } else if !encrypt0.protected.header.iv.is_empty() {
        encrypt0.protected.header.iv.clone()
    } else {
        return Err(Claim169Error::DecryptionFailed(
            "no IV in COSE_Encrypt0".to_string(),
        ));
    };

    // Get ciphertext
    let ciphertext = encrypt0.ciphertext.clone().ok_or_else(|| {
        Claim169Error::DecryptionFailed("COSE_Encrypt0 has no ciphertext".to_string())
    })?;

    // Build AAD
    let aad = build_encrypt0_aad(&encrypt0.protected.original_data.clone().unwrap_or_default());

    // Decrypt
    let plaintext = decryptor
        .decrypt(alg, key_id.as_deref(), &nonce, &aad, &ciphertext)
        .map_err(|e| Claim169Error::DecryptionFailed(e.to_string()))?;

    // Check if decrypted payload is a COSE_Sign1 structure
    let is_cose_sign1 = CoseSign1::from_tagged_slice(&plaintext).is_ok()
        || CoseSign1::from_slice(&plaintext).is_ok();

    if is_cose_sign1 {
        // Recursively process with resolver
        match parse_with_resolver(&plaintext, resolver) {
            Ok(inner_result) => return Ok(inner_result),
            Err(Claim169Error::SignatureInvalid(_)) => {
                return Ok(CoseResult {
                    payload: plaintext,
                    verification_status: VerificationStatus::Failed,
                    algorithm: Some(alg),
                    key_id,
                });
            }
            Err(e) => return Err(e),
        }
    }

    // Return decrypted payload as-is
    Ok(CoseResult {
        payload: plaintext,
        verification_status: VerificationStatus::Skipped,
        algorithm: Some(alg),
        key_id,
    })
}

/// Process a COSE_Sign1 message
fn process_sign1(sign1: CoseSign1, verifier: Option<&dyn SignatureVerifier>) -> Result<CoseResult> {
    let algorithm = get_algorithm(&sign1.protected.header);
    let key_id = get_key_id(&sign1.protected.header, &sign1.unprotected);

    let payload = sign1
        .payload
        .clone()
        .ok_or_else(|| Claim169Error::CoseParse("COSE_Sign1 has no payload".to_string()))?;

    let verification_status = match verifier {
        Some(v) => {
            // Require explicit algorithm when verification is requested - no defaults allowed
            // This prevents algorithm confusion attacks where attacker omits algorithm header
            let alg = algorithm.ok_or_else(|| {
                Claim169Error::CoseParse(
                    "COSE_Sign1 missing required algorithm in protected header".to_string(),
                )
            })?;

            // Build the Sig_structure for verification
            let sig_structure = sign1.tbs_data(&[]);

            match v.verify(alg, key_id.as_deref(), &sig_structure, &sign1.signature) {
                Ok(()) => VerificationStatus::Verified,
                Err(CryptoError::VerificationFailed) => VerificationStatus::Failed,
                Err(e) => return Err(e.into()),
            }
        }
        None => VerificationStatus::Skipped,
    };

    Ok(CoseResult {
        payload,
        verification_status,
        algorithm,
        key_id,
    })
}

/// Process a COSE_Encrypt0 message
fn process_encrypt0(
    encrypt0: CoseEncrypt0,
    decryptor: Option<&dyn Decryptor>,
    verifier: Option<&dyn SignatureVerifier>,
) -> Result<CoseResult> {
    let algorithm = get_algorithm(&encrypt0.protected.header);
    let key_id = get_key_id(&encrypt0.protected.header, &encrypt0.unprotected);

    let decryptor = decryptor
        .ok_or_else(|| Claim169Error::DecryptionFailed("no decryptor provided".to_string()))?;

    // Get IV/nonce from header - check unprotected first, then protected
    let nonce = if !encrypt0.unprotected.iv.is_empty() {
        encrypt0.unprotected.iv.clone()
    } else if !encrypt0.protected.header.iv.is_empty() {
        encrypt0.protected.header.iv.clone()
    } else {
        return Err(Claim169Error::DecryptionFailed(
            "no IV in COSE_Encrypt0".to_string(),
        ));
    };

    // Get ciphertext
    let ciphertext = encrypt0.ciphertext.clone().ok_or_else(|| {
        Claim169Error::DecryptionFailed("COSE_Encrypt0 has no ciphertext".to_string())
    })?;

    // Build AAD (Additional Authenticated Data) - Enc_structure
    // For COSE_Encrypt0, this is ["Encrypt0", protected, external_aad]
    let aad = build_encrypt0_aad(&encrypt0.protected.original_data.clone().unwrap_or_default());

    // Require explicit algorithm when decryption is requested - no defaults allowed
    // This prevents algorithm confusion attacks
    let alg = algorithm.ok_or_else(|| {
        Claim169Error::CoseParse(
            "COSE_Encrypt0 missing required algorithm in protected header".to_string(),
        )
    })?;

    // Decrypt
    let plaintext = decryptor
        .decrypt(alg, key_id.as_deref(), &nonce, &aad, &ciphertext)
        .map_err(|e| Claim169Error::DecryptionFailed(e.to_string()))?;

    // The decrypted payload might be a signed CWT, try to verify
    if let Some(v) = verifier {
        // Check if the decrypted payload is a COSE_Sign1 structure
        let is_cose_sign1 = CoseSign1::from_tagged_slice(&plaintext).is_ok()
            || CoseSign1::from_slice(&plaintext).is_ok();

        if is_cose_sign1 {
            // Inner content is signed - verification result matters
            match parse_and_verify(&plaintext, Some(v), None) {
                Ok(inner_result) => return Ok(inner_result),
                Err(Claim169Error::SignatureInvalid(_)) => {
                    // Inner signature verification explicitly failed
                    // Return Failed status instead of silently returning Skipped
                    return Ok(CoseResult {
                        payload: plaintext,
                        verification_status: VerificationStatus::Failed,
                        algorithm: Some(alg),
                        key_id,
                    });
                }
                Err(e) => return Err(e), // Other errors propagate
            }
        }
    }

    // Return decrypted payload as-is (not a COSE_Sign1 or no verifier provided)
    Ok(CoseResult {
        payload: plaintext,
        verification_status: VerificationStatus::Skipped,
        algorithm: Some(alg),
        key_id,
    })
}

/// Build the Enc_structure AAD for COSE_Encrypt0
/// Structure: ["Encrypt0", protected, external_aad]
fn build_encrypt0_aad(protected_bytes: &[u8]) -> Vec<u8> {
    use ciborium::Value;

    let enc_structure = Value::Array(vec![
        Value::Text("Encrypt0".to_string()),
        Value::Bytes(protected_bytes.to_vec()),
        Value::Bytes(vec![]), // external_aad is empty
    ]);

    let mut aad = Vec::new();
    ciborium::into_writer(&enc_structure, &mut aad).expect("CBOR encoding should not fail");
    aad
}

/// Extract algorithm from COSE header
fn get_algorithm(header: &Header) -> Option<iana::Algorithm> {
    header.alg.as_ref().and_then(|alg| match alg {
        coset::Algorithm::Assigned(a) => Some(*a),
        _ => None,
    })
}

/// Extract key ID from COSE headers
fn get_key_id(protected: &Header, unprotected: &Header) -> Option<Vec<u8>> {
    // Try protected header first, then unprotected
    if !protected.key_id.is_empty() {
        Some(protected.key_id.clone())
    } else if !unprotected.key_id.is_empty() {
        Some(unprotected.key_id.clone())
    } else {
        None
    }
}

/// Create a COSE_Sign1 structure for encoding (used in test vector generation)
pub fn create_sign1(payload: &[u8], algorithm: iana::Algorithm) -> CoseSign1 {
    coset::CoseSign1Builder::new()
        .protected(coset::HeaderBuilder::new().algorithm(algorithm).build())
        .payload(payload.to_vec())
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CryptoResult;

    /// Mock verifier for testing
    struct MockVerifier {
        should_pass: bool,
    }

    impl MockVerifier {
        fn passing() -> Self {
            Self { should_pass: true }
        }

        fn failing() -> Self {
            Self { should_pass: false }
        }
    }

    impl SignatureVerifier for MockVerifier {
        fn verify(
            &self,
            _algorithm: iana::Algorithm,
            _key_id: Option<&[u8]>,
            _data: &[u8],
            _signature: &[u8],
        ) -> CryptoResult<()> {
            if self.should_pass {
                Ok(())
            } else {
                Err(CryptoError::VerificationFailed)
            }
        }
    }

    /// Mock decryptor for testing
    struct MockDecryptor {
        plaintext: Vec<u8>,
    }

    impl MockDecryptor {
        fn returning(plaintext: Vec<u8>) -> Self {
            Self { plaintext }
        }
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

    #[test]
    fn test_create_sign1() {
        let payload = b"test payload";
        let sign1 = create_sign1(payload, iana::Algorithm::EdDSA);

        assert_eq!(sign1.payload.as_deref(), Some(payload.as_slice()));
        assert!(sign1.signature.is_empty()); // Not signed yet
    }

    #[test]
    fn test_invalid_cose_data() {
        let invalid = b"not valid COSE data";
        let result = parse_and_verify(invalid, None, None);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Claim169Error::CoseParse(_)));
    }

    #[test]
    fn test_sign1_missing_algorithm_returns_error_when_verifier_provided() {
        // Create COSE_Sign1 without algorithm in protected header
        let sign1 = coset::CoseSign1Builder::new()
            .protected(coset::HeaderBuilder::new().build()) // No algorithm!
            .payload(b"test payload".to_vec())
            .signature(vec![0u8; 64])
            .build();

        let tagged = sign1.to_tagged_vec().unwrap();
        let verifier = MockVerifier::passing();

        let result = parse_and_verify(&tagged, Some(&verifier), None);

        assert!(result.is_err());
        match result.unwrap_err() {
            Claim169Error::CoseParse(msg) => {
                assert!(
                    msg.contains("algorithm"),
                    "Error should mention missing algorithm: {}",
                    msg
                );
            }
            e => panic!("Expected CoseParse error, got: {:?}", e),
        }
    }

    #[test]
    fn test_sign1_missing_algorithm_ok_when_no_verifier() {
        // Without a verifier, missing algorithm is fine (verification skipped)
        let sign1 = coset::CoseSign1Builder::new()
            .protected(coset::HeaderBuilder::new().build()) // No algorithm
            .payload(b"test payload".to_vec())
            .signature(vec![0u8; 64])
            .build();

        let tagged = sign1.to_tagged_vec().unwrap();

        let result = parse_and_verify(&tagged, None, None);

        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.verification_status, VerificationStatus::Skipped);
        assert_eq!(res.algorithm, None);
    }

    #[test]
    fn test_sign1_with_algorithm_verifies_successfully() {
        let sign1 = coset::CoseSign1Builder::new()
            .protected(
                coset::HeaderBuilder::new()
                    .algorithm(iana::Algorithm::EdDSA)
                    .build(),
            )
            .payload(b"test payload".to_vec())
            .signature(vec![0u8; 64])
            .build();

        let tagged = sign1.to_tagged_vec().unwrap();
        let verifier = MockVerifier::passing();

        let result = parse_and_verify(&tagged, Some(&verifier), None);

        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.verification_status, VerificationStatus::Verified);
        assert_eq!(res.algorithm, Some(iana::Algorithm::EdDSA));
    }

    #[test]
    fn test_sign1_verification_fails_returns_failed_status() {
        let sign1 = coset::CoseSign1Builder::new()
            .protected(
                coset::HeaderBuilder::new()
                    .algorithm(iana::Algorithm::EdDSA)
                    .build(),
            )
            .payload(b"test payload".to_vec())
            .signature(vec![0u8; 64])
            .build();

        let tagged = sign1.to_tagged_vec().unwrap();
        let verifier = MockVerifier::failing();

        let result = parse_and_verify(&tagged, Some(&verifier), None);

        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.verification_status, VerificationStatus::Failed);
    }

    #[test]
    fn test_encrypt0_missing_algorithm_returns_error() {
        // Create COSE_Encrypt0 without algorithm
        let encrypt0 = coset::CoseEncrypt0Builder::new()
            .protected(coset::HeaderBuilder::new().build()) // No algorithm!
            .unprotected(coset::HeaderBuilder::new().iv(vec![0u8; 12]).build())
            .ciphertext(vec![0u8; 32])
            .build();

        let tagged = encrypt0.to_tagged_vec().unwrap();
        let decryptor = MockDecryptor::returning(vec![1, 2, 3]);

        let result = parse_and_verify(&tagged, None, Some(&decryptor));

        assert!(result.is_err());
        match result.unwrap_err() {
            Claim169Error::CoseParse(msg) => {
                assert!(
                    msg.contains("algorithm"),
                    "Error should mention missing algorithm: {}",
                    msg
                );
            }
            e => panic!("Expected CoseParse error, got: {:?}", e),
        }
    }

    #[test]
    fn test_encrypt0_with_algorithm_decrypts_successfully() {
        let encrypt0 = coset::CoseEncrypt0Builder::new()
            .protected(
                coset::HeaderBuilder::new()
                    .algorithm(iana::Algorithm::A256GCM)
                    .build(),
            )
            .unprotected(coset::HeaderBuilder::new().iv(vec![0u8; 12]).build())
            .ciphertext(vec![0u8; 32])
            .build();

        let tagged = encrypt0.to_tagged_vec().unwrap();
        let expected_plaintext = b"decrypted content".to_vec();
        let decryptor = MockDecryptor::returning(expected_plaintext.clone());

        let result = parse_and_verify(&tagged, None, Some(&decryptor));

        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.payload, expected_plaintext);
        assert_eq!(res.algorithm, Some(iana::Algorithm::A256GCM));
    }
}
