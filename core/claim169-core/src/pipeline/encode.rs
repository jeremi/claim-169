//! Encoding pipeline for Claim 169 QR codes.
//!
//! This module provides the internal encoding pipeline that transforms
//! identity data into a QR-ready Base45 string.
//!
//! # Pipeline
//!
//! ```text
//! Claim169 → CBOR → CWT → COSE_Sign1 → [COSE_Encrypt0] → zlib → Base45
//! ```

use coset::{
    iana, CborSerializable, CoseSign1Builder, Header, HeaderBuilder, TaggedCborSerializable,
};

use crate::crypto::traits::{Encryptor, Signer};
use crate::error::{Claim169Error, Result};
use crate::model::{Claim169, CwtMeta};

use super::{base45_encode, claim169_to_cbor, compress, cwt_encode};

/// Configuration for the encoding pipeline
#[derive(Debug, Clone, Default)]
pub struct EncodeConfig {
    /// Skip biometric fields during encoding
    pub skip_biometrics: bool,
}

/// Encode a Claim169 to a Base45-encoded QR string with optional signing.
///
/// # Arguments
///
/// * `claim169` - The identity claim data
/// * `cwt_meta` - CWT metadata (issuer, expiration, etc.)
/// * `signer` - Optional signer for creating COSE_Sign1 signatures
/// * `algorithm` - The signing algorithm (required if signer is provided)
/// * `config` - Encoding configuration options
///
/// # Returns
///
/// Base45-encoded string suitable for QR code generation
pub fn encode_signed(
    claim169: &Claim169,
    cwt_meta: &CwtMeta,
    signer: Option<&dyn Signer>,
    algorithm: Option<iana::Algorithm>,
    config: &EncodeConfig,
) -> Result<String> {
    // Step 1: Convert Claim169 to CBOR Value
    let claim169_cbor = if config.skip_biometrics {
        claim169_to_cbor(&claim169.without_biometrics())
    } else {
        claim169_to_cbor(claim169)
    };

    // Step 2: Encode CWT payload
    let cwt_payload = cwt_encode(cwt_meta, &claim169_cbor);

    // Step 3: Create and sign COSE_Sign1
    let cose_bytes = create_signed_cose(&cwt_payload, signer, algorithm)?;

    // Step 4: Compress with zlib
    let compressed = compress(&cose_bytes);

    // Step 5: Encode as Base45
    Ok(base45_encode(&compressed))
}

/// Encode a Claim169 with signing and encryption.
///
/// # Arguments
///
/// * `claim169` - The identity claim data
/// * `cwt_meta` - CWT metadata (issuer, expiration, etc.)
/// * `signer` - Optional signer for creating COSE_Sign1 signatures
/// * `sign_algorithm` - The signing algorithm (required if signer is provided)
/// * `encryptor` - Encryptor for COSE_Encrypt0 encryption
/// * `encrypt_algorithm` - The encryption algorithm
/// * `nonce` - 12-byte nonce for AES-GCM encryption
/// * `config` - Encoding configuration options
///
/// # Returns
///
/// Base45-encoded string suitable for QR code generation
#[allow(clippy::too_many_arguments)]
pub fn encode_signed_and_encrypted(
    claim169: &Claim169,
    cwt_meta: &CwtMeta,
    signer: Option<&dyn Signer>,
    sign_algorithm: Option<iana::Algorithm>,
    encryptor: &dyn Encryptor,
    encrypt_algorithm: iana::Algorithm,
    nonce: &[u8; 12],
    config: &EncodeConfig,
) -> Result<String> {
    // Step 1: Convert Claim169 to CBOR Value
    let claim169_cbor = if config.skip_biometrics {
        claim169_to_cbor(&claim169.without_biometrics())
    } else {
        claim169_to_cbor(claim169)
    };

    // Step 2: Encode CWT payload
    let cwt_payload = cwt_encode(cwt_meta, &claim169_cbor);

    // Step 3: Create and sign COSE_Sign1
    let signed_bytes = create_signed_cose(&cwt_payload, signer, sign_algorithm)?;

    // Step 4: Encrypt with COSE_Encrypt0
    let encrypted_bytes =
        create_encrypted_cose(&signed_bytes, encryptor, encrypt_algorithm, nonce)?;

    // Step 5: Compress with zlib
    let compressed = compress(&encrypted_bytes);

    // Step 6: Encode as Base45
    Ok(base45_encode(&compressed))
}

/// Create a COSE_Sign1 structure, optionally signed.
fn create_signed_cose(
    payload: &[u8],
    signer: Option<&dyn Signer>,
    algorithm: Option<iana::Algorithm>,
) -> Result<Vec<u8>> {
    match (signer, algorithm) {
        (Some(signer), Some(alg)) => {
            // Build protected header with algorithm
            let protected = HeaderBuilder::new().algorithm(alg).build();

            // Build COSE_Sign1 with payload
            let mut sign1 = CoseSign1Builder::new()
                .protected(protected)
                .payload(payload.to_vec())
                .build();

            // Create the Sig_structure for signing
            let tbs_data = sign1.tbs_data(&[]);

            // Sign the data
            let signature = signer
                .sign(alg, signer.key_id(), &tbs_data)
                .map_err(|e| Claim169Error::SignatureFailed(e.to_string()))?;

            sign1.signature = signature;

            // Serialize with tag
            sign1
                .to_tagged_vec()
                .map_err(|e| Claim169Error::CborEncode(e.to_string()))
        }
        (None, None) => {
            // Unsigned: still create COSE_Sign1 but with empty signature
            let sign1 = CoseSign1Builder::new()
                .protected(Header::default())
                .payload(payload.to_vec())
                .build();

            sign1
                .to_tagged_vec()
                .map_err(|e| Claim169Error::CborEncode(e.to_string()))
        }
        (Some(_), None) => Err(Claim169Error::EncodingConfig(
            "signer provided but no algorithm specified".to_string(),
        )),
        (None, Some(_)) => Err(Claim169Error::EncodingConfig(
            "algorithm specified but no signer provided".to_string(),
        )),
    }
}

/// Create a COSE_Encrypt0 structure.
fn create_encrypted_cose(
    plaintext: &[u8],
    encryptor: &dyn Encryptor,
    algorithm: iana::Algorithm,
    nonce: &[u8; 12],
) -> Result<Vec<u8>> {
    use coset::CoseEncrypt0Builder;

    // Build protected header with algorithm
    let protected = HeaderBuilder::new().algorithm(algorithm).build();

    // Build unprotected header with IV/nonce
    let unprotected = HeaderBuilder::new().iv(nonce.to_vec()).build();

    // Serialize protected header for AAD computation (clone before consuming)
    let protected_bytes = protected
        .clone()
        .to_vec()
        .map_err(|e| Claim169Error::CborEncode(e.to_string()))?;

    // Build AAD (Enc_structure)
    let aad = build_encrypt0_aad(&protected_bytes);

    // Encrypt the plaintext
    let ciphertext = encryptor
        .encrypt(algorithm, None, nonce, &aad, plaintext)
        .map_err(|e| Claim169Error::EncryptionFailed(e.to_string()))?;

    // Build COSE_Encrypt0
    let encrypt0 = CoseEncrypt0Builder::new()
        .protected(protected)
        .unprotected(unprotected)
        .ciphertext(ciphertext)
        .build();

    // Serialize with tag
    encrypt0
        .to_tagged_vec()
        .map_err(|e| Claim169Error::CborEncode(e.to_string()))
}

/// Build the Enc_structure AAD for COSE_Encrypt0.
///
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_unsigned() {
        let claim169 = Claim169 {
            id: Some("12345".to_string()),
            full_name: Some("Test User".to_string()),
            ..Default::default()
        };

        let cwt_meta = CwtMeta::new()
            .with_issuer("https://test.mosip.io")
            .with_issued_at(1700000000);

        let config = EncodeConfig::default();
        let result = encode_signed(&claim169, &cwt_meta, None, None, &config);

        assert!(result.is_ok());
        let qr_data = result.unwrap();

        // Base45 encoded data should be non-empty and contain valid chars
        assert!(!qr_data.is_empty());
        assert!(qr_data
            .chars()
            .all(|c| { c.is_ascii_uppercase() || c.is_ascii_digit() || " $%*+-./:".contains(c) }));
    }

    #[test]
    fn test_encode_config_mismatch() {
        let claim169 = Claim169::default();
        let cwt_meta = CwtMeta::default();
        let config = EncodeConfig::default();

        // Signer without algorithm should fail
        struct MockSigner;
        impl Signer for MockSigner {
            fn sign(
                &self,
                _: iana::Algorithm,
                _: Option<&[u8]>,
                _: &[u8],
            ) -> crate::error::CryptoResult<Vec<u8>> {
                Ok(vec![0u8; 64])
            }
        }

        let result = encode_signed(&claim169, &cwt_meta, Some(&MockSigner), None, &config);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Claim169Error::EncodingConfig(_)
        ));

        // Algorithm without signer should also fail
        let result = encode_signed(
            &claim169,
            &cwt_meta,
            None,
            Some(iana::Algorithm::EdDSA),
            &config,
        );
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Claim169Error::EncodingConfig(_)
        ));
    }

    #[cfg(feature = "software-crypto")]
    #[test]
    fn test_encode_signed_ed25519() {
        use crate::crypto::software::Ed25519Signer;

        let claim169 = Claim169 {
            id: Some("12345".to_string()),
            full_name: Some("Signed User".to_string()),
            ..Default::default()
        };

        let cwt_meta = CwtMeta::new()
            .with_issuer("https://test.mosip.io")
            .with_expires_at(1800000000);

        let signer = Ed25519Signer::generate();
        let config = EncodeConfig::default();

        let result = encode_signed(
            &claim169,
            &cwt_meta,
            Some(&signer),
            Some(iana::Algorithm::EdDSA),
            &config,
        );

        assert!(result.is_ok());
        let qr_data = result.unwrap();
        assert!(!qr_data.is_empty());
    }

    #[cfg(feature = "software-crypto")]
    #[test]
    fn test_encode_signed_and_encrypted() {
        use crate::crypto::software::{AesGcmEncryptor, Ed25519Signer};

        let claim169 = Claim169 {
            id: Some("encrypted-12345".to_string()),
            full_name: Some("Encrypted User".to_string()),
            ..Default::default()
        };

        let cwt_meta = CwtMeta::new()
            .with_issuer("https://test.mosip.io")
            .with_expires_at(1800000000);

        let signer = Ed25519Signer::generate();
        let encryption_key = [0u8; 32];
        let encryptor = AesGcmEncryptor::aes256(&encryption_key).unwrap();
        let nonce = [1u8; 12];
        let config = EncodeConfig::default();

        let result = encode_signed_and_encrypted(
            &claim169,
            &cwt_meta,
            Some(&signer),
            Some(iana::Algorithm::EdDSA),
            &encryptor,
            iana::Algorithm::A256GCM,
            &nonce,
            &config,
        );

        assert!(result.is_ok());
        let qr_data = result.unwrap();
        assert!(!qr_data.is_empty());
    }

    #[cfg(feature = "software-crypto")]
    #[test]
    fn test_roundtrip_signed() {
        use crate::crypto::software::Ed25519Signer;
        use crate::pipeline::claim169::transform;
        use crate::pipeline::cose::parse_and_verify;
        use crate::pipeline::cwt::parse as cwt_parse;
        use crate::pipeline::{base45_decode, decompress};

        let original_claim = Claim169 {
            id: Some("roundtrip-test".to_string()),
            full_name: Some("Roundtrip User".to_string()),
            email: Some("test@example.com".to_string()),
            ..Default::default()
        };

        let cwt_meta = CwtMeta::new()
            .with_issuer("https://roundtrip.test")
            .with_issued_at(1700000000)
            .with_expires_at(1800000000);

        let signer = Ed25519Signer::generate();
        let verifier = signer.verifying_key();
        let config = EncodeConfig::default();

        // Encode
        let qr_data = encode_signed(
            &original_claim,
            &cwt_meta,
            Some(&signer),
            Some(iana::Algorithm::EdDSA),
            &config,
        )
        .unwrap();

        // Decode: Base45 → decompress → COSE → CWT → Claim169
        let compressed = base45_decode(&qr_data).unwrap();
        let cose_bytes = decompress(&compressed, 65536).unwrap();
        let cose_result = parse_and_verify(&cose_bytes, Some(&verifier), None).unwrap();

        assert_eq!(
            cose_result.verification_status,
            crate::model::VerificationStatus::Verified
        );

        let cwt_result = cwt_parse(&cose_result.payload).unwrap();
        let decoded_claim = transform(cwt_result.claim_169, false).unwrap();

        // Verify roundtrip
        assert_eq!(decoded_claim.id, original_claim.id);
        assert_eq!(decoded_claim.full_name, original_claim.full_name);
        assert_eq!(decoded_claim.email, original_claim.email);

        assert_eq!(cwt_result.meta.issuer, cwt_meta.issuer);
        assert_eq!(cwt_result.meta.issued_at, cwt_meta.issued_at);
        assert_eq!(cwt_result.meta.expires_at, cwt_meta.expires_at);
    }

    #[cfg(feature = "software-crypto")]
    #[test]
    fn test_roundtrip_signed_and_encrypted() {
        use crate::crypto::software::{AesGcmDecryptor, AesGcmEncryptor, Ed25519Signer};
        use crate::pipeline::claim169::transform;
        use crate::pipeline::cose::parse_and_verify;
        use crate::pipeline::cwt::parse as cwt_parse;
        use crate::pipeline::{base45_decode, decompress};

        let original_claim = Claim169 {
            id: Some("encrypted-roundtrip".to_string()),
            full_name: Some("Encrypted Roundtrip User".to_string()),
            ..Default::default()
        };

        let cwt_meta = CwtMeta::new()
            .with_issuer("https://encrypted.test")
            .with_expires_at(1800000000);

        let signer = Ed25519Signer::generate();
        let verifier = signer.verifying_key();

        let encryption_key = [42u8; 32];
        let encryptor = AesGcmEncryptor::aes256(&encryption_key).unwrap();
        let decryptor = AesGcmDecryptor::aes256(&encryption_key).unwrap();
        let nonce = [7u8; 12];
        let config = EncodeConfig::default();

        // Encode
        let qr_data = encode_signed_and_encrypted(
            &original_claim,
            &cwt_meta,
            Some(&signer),
            Some(iana::Algorithm::EdDSA),
            &encryptor,
            iana::Algorithm::A256GCM,
            &nonce,
            &config,
        )
        .unwrap();

        // Decode: Base45 → decompress → COSE (decrypt + verify) → CWT → Claim169
        let compressed = base45_decode(&qr_data).unwrap();
        let cose_bytes = decompress(&compressed, 65536).unwrap();
        let cose_result = parse_and_verify(&cose_bytes, Some(&verifier), Some(&decryptor)).unwrap();

        assert_eq!(
            cose_result.verification_status,
            crate::model::VerificationStatus::Verified
        );

        let cwt_result = cwt_parse(&cose_result.payload).unwrap();
        let decoded_claim = transform(cwt_result.claim_169, false).unwrap();

        // Verify roundtrip
        assert_eq!(decoded_claim.id, original_claim.id);
        assert_eq!(decoded_claim.full_name, original_claim.full_name);
    }
}
