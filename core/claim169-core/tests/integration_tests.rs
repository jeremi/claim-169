//! Integration tests for claim169-core
//!
//! These tests verify the full decode pipeline from Base45 QR data to Claim169 structs.

use ciborium::Value;
use claim169_core::{
    AesGcmDecryptor, AesGcmEncryptor, Claim169Error, CwtMeta, Decoder, Ed25519Signer, Encryptor,
    Signer, VerificationStatus,
};
use coset::{
    iana, CborSerializable, CoseEncrypt0Builder, CoseSign1Builder, HeaderBuilder,
    TaggedCborSerializable,
};

/// Helper to create claim 169 CBOR map
fn create_claim169_map(fields: Vec<(i64, Value)>) -> Value {
    Value::Map(
        fields
            .into_iter()
            .map(|(k, v)| (Value::Integer(k.into()), v))
            .collect(),
    )
}

/// Helper to encode CWT
fn encode_cwt(meta: &CwtMeta, claim_169: &Value) -> Vec<u8> {
    claim169_core::pipeline::cwt::encode(meta, claim_169)
}

/// Helper to encode unsigned QR data (with algorithm header for security)
fn encode_unsigned_qr(meta: &CwtMeta, claim_169: &Value) -> String {
    let cwt_bytes = encode_cwt(meta, claim_169);

    let sign1 = CoseSign1Builder::new()
        .protected(
            HeaderBuilder::new()
                .algorithm(iana::Algorithm::EdDSA)
                .build(),
        )
        .payload(cwt_bytes)
        .build();

    let cose_bytes = sign1.to_tagged_vec().unwrap();
    let compressed = claim169_core::pipeline::decompress::compress(&cose_bytes);
    claim169_core::pipeline::base45::encode(&compressed)
}

/// Helper to encode signed QR data
fn encode_signed_qr(meta: &CwtMeta, claim_169: &Value, signer: &Ed25519Signer) -> String {
    let cwt_bytes = encode_cwt(meta, claim_169);

    let protected = HeaderBuilder::new()
        .algorithm(iana::Algorithm::EdDSA)
        .build();

    let mut sign1 = CoseSign1Builder::new()
        .protected(protected)
        .payload(cwt_bytes)
        .build();

    let tbs_data = sign1.tbs_data(&[]);
    let signature = signer
        .sign(iana::Algorithm::EdDSA, None, &tbs_data)
        .expect("signing failed");
    sign1.signature = signature;

    let cose_bytes = sign1.to_tagged_vec().unwrap();
    let compressed = claim169_core::pipeline::decompress::compress(&cose_bytes);
    claim169_core::pipeline::base45::encode(&compressed)
}

/// Helper to build Enc_structure AAD for COSE_Encrypt0
fn build_encrypt0_aad(protected_bytes: &[u8]) -> Vec<u8> {
    let enc_structure = Value::Array(vec![
        Value::Text("Encrypt0".to_string()),
        Value::Bytes(protected_bytes.to_vec()),
        Value::Bytes(vec![]),
    ]);

    let mut aad = Vec::new();
    ciborium::into_writer(&enc_structure, &mut aad).expect("CBOR encoding should not fail");
    aad
}

/// Helper to encode encrypted QR data
fn encode_encrypted_qr(
    meta: &CwtMeta,
    claim_169: &Value,
    encryptor: &AesGcmEncryptor,
    nonce: &[u8],
) -> String {
    let cwt_bytes = encode_cwt(meta, claim_169);

    let protected = HeaderBuilder::new()
        .algorithm(iana::Algorithm::A256GCM)
        .build();

    let protected_bytes = protected.clone().to_vec().unwrap();
    let aad = build_encrypt0_aad(&protected_bytes);

    let ciphertext = encryptor
        .encrypt(iana::Algorithm::A256GCM, None, nonce, &aad, &cwt_bytes)
        .expect("encryption failed");

    let encrypt0 = CoseEncrypt0Builder::new()
        .protected(protected)
        .unprotected(HeaderBuilder::new().iv(nonce.to_vec()).build())
        .ciphertext(ciphertext)
        .build();

    let cose_bytes = encrypt0.to_tagged_vec().unwrap();
    let compressed = claim169_core::pipeline::decompress::compress(&cose_bytes);
    claim169_core::pipeline::base45::encode(&compressed)
}

// ============================================================================
// Full Pipeline Tests
// ============================================================================

#[test]
fn test_full_pipeline_base45_to_claim169() {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-PIPELINE-001".to_string())),
        (4, Value::Text("Pipeline Test Person".to_string())),
        (8, Value::Text("19900101".to_string())),
        (9, Value::Integer(1.into())), // Male
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(2000000000); // Far future

    let qr_data = encode_unsigned_qr(&meta, &claim_169);

    // Decode with permissive options (no verification required)
    let result = Decoder::new(&qr_data)
        .allow_unverified()
        .without_timestamp_validation()
        .decode()
        .expect("decode should succeed");

    assert_eq!(result.claim169.id, Some("ID-PIPELINE-001".to_string()));
    assert_eq!(
        result.claim169.full_name,
        Some("Pipeline Test Person".to_string())
    );
    assert_eq!(result.claim169.date_of_birth, Some("19900101".to_string()));
    assert_eq!(
        result.cwt_meta.issuer,
        Some("https://test.example.org".to_string())
    );
    assert_eq!(result.verification_status, VerificationStatus::Skipped);
}

#[test]
fn test_full_pipeline_with_signature_verification() {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-SIGNED-PIPE".to_string())),
        (4, Value::Text("Signed Pipeline Person".to_string())),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://signed.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(2000000000);

    // Generate key pair
    let signer = Ed25519Signer::generate();

    let qr_data = encode_signed_qr(&meta, &claim_169, &signer);

    // Decode with verification
    let result = Decoder::new(&qr_data)
        .verify_with_ed25519(&signer.public_key_bytes())
        .unwrap()
        .decode()
        .expect("decode with verification should succeed");

    assert_eq!(result.claim169.id, Some("ID-SIGNED-PIPE".to_string()));
    assert_eq!(result.verification_status, VerificationStatus::Verified);
}

#[test]
fn test_encrypted_credential_full_pipeline() {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-ENCRYPTED-PIPE".to_string())),
        (4, Value::Text("Encrypted Pipeline Person".to_string())),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://encrypted.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(2000000000);

    // Encryption key and nonce
    let key: [u8; 32] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
        0x1e, 0x1f,
    ];
    let nonce: [u8; 12] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
    ];

    let encryptor = AesGcmEncryptor::aes256(&key).unwrap();
    let decryptor = AesGcmDecryptor::aes256(&key).unwrap();

    let qr_data = encode_encrypted_qr(&meta, &claim_169, &encryptor, &nonce);

    // Decode encrypted credential (permissive since inner content is not signed)
    let result = Decoder::new(&qr_data)
        .decrypt_with(decryptor)
        .allow_unverified()
        .without_timestamp_validation()
        .decode()
        .expect("encrypted decode should succeed");

    assert_eq!(result.claim169.id, Some("ID-ENCRYPTED-PIPE".to_string()));
    assert_eq!(
        result.claim169.full_name,
        Some("Encrypted Pipeline Person".to_string())
    );
}

// ============================================================================
// Timestamp Validation Tests
// ============================================================================

#[test]
fn test_expired_credential_rejected() {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-EXPIRED".to_string())),
        (4, Value::Text("Expired Person".to_string())),
    ]);

    // Expired in 2021
    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(1577836800) // 2020-01-01
        .with_expires_at(1609459200); // 2021-01-01

    let qr_data = encode_unsigned_qr(&meta, &claim_169);

    // Use default options (validates timestamps) but allow unverified
    let result = Decoder::new(&qr_data).allow_unverified().decode();

    assert!(result.is_err());
    match result.unwrap_err() {
        Claim169Error::Expired(exp) => {
            assert_eq!(exp, 1609459200);
        }
        e => panic!("Expected Expired error, got: {:?}", e),
    }
}

#[test]
fn test_not_yet_valid_credential_rejected() {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-FUTURE".to_string())),
        (4, Value::Text("Future Person".to_string())),
    ]);

    // Not valid until 2050
    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(1700000000)
        .with_not_before(2524608000) // 2050-01-01
        .with_expires_at(2556144000); // 2051-01-01

    let qr_data = encode_unsigned_qr(&meta, &claim_169);

    // Use default options (validates timestamps) but allow unverified
    let result = Decoder::new(&qr_data).allow_unverified().decode();

    assert!(result.is_err());
    match result.unwrap_err() {
        Claim169Error::NotYetValid(nbf) => {
            assert_eq!(nbf, 2524608000);
        }
        e => panic!("Expected NotYetValid error, got: {:?}", e),
    }
}

#[test]
fn test_expired_credential_accepted_when_validation_disabled() {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-EXPIRED-OK".to_string())),
        (4, Value::Text("Expired But OK Person".to_string())),
    ]);

    // Expired in 2021
    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(1577836800)
        .with_expires_at(1609459200);

    let qr_data = encode_unsigned_qr(&meta, &claim_169);

    // Permissive options (validation disabled)
    let result = Decoder::new(&qr_data)
        .allow_unverified()
        .without_timestamp_validation()
        .decode()
        .expect("should decode when validation disabled");

    assert_eq!(result.claim169.id, Some("ID-EXPIRED-OK".to_string()));
}

// ============================================================================
// Verification Requirement Tests
// ============================================================================

#[test]
fn test_strict_options_require_verifier() {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-NO-VERIFY".to_string())),
        (4, Value::Text("No Verify Person".to_string())),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(2000000000);

    let qr_data = encode_unsigned_qr(&meta, &claim_169);

    // Strict options require verification (no allow_unverified, no verifier)
    let result = Decoder::new(&qr_data).decode();

    assert!(result.is_err());
    match result.unwrap_err() {
        Claim169Error::DecodingConfig(msg) => {
            assert!(msg.contains("verification required"));
        }
        e => panic!("Expected DecodingConfig error, got: {:?}", e),
    }
}

#[test]
fn test_wrong_verifier_fails() {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-WRONG-KEY".to_string())),
        (4, Value::Text("Wrong Key Person".to_string())),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(2000000000);

    // Sign with one key
    let signer = Ed25519Signer::generate();
    let qr_data = encode_signed_qr(&meta, &claim_169, &signer);

    // Verify with different key
    let wrong_signer = Ed25519Signer::generate();

    let result = Decoder::new(&qr_data)
        .verify_with_ed25519(&wrong_signer.public_key_bytes())
        .unwrap()
        .decode();

    assert!(result.is_err());
    match result.unwrap_err() {
        Claim169Error::SignatureInvalid(msg) => {
            assert!(msg.contains("verification failed"));
        }
        e => panic!("Expected SignatureInvalid error, got: {:?}", e),
    }
}

// ============================================================================
// Error Case Tests
// ============================================================================

#[test]
fn test_invalid_base45_rejected() {
    let result = Decoder::new("THIS!IS@NOT#VALID$BASE45")
        .allow_unverified()
        .without_timestamp_validation()
        .decode();

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        Claim169Error::Base45Decode(_)
    ));
}

#[test]
fn test_invalid_zlib_rejected() {
    // Valid Base45 but garbage data
    let garbage = claim169_core::pipeline::base45::encode(&[0xDE, 0xAD, 0xBE, 0xEF]);

    let result = Decoder::new(&garbage)
        .allow_unverified()
        .without_timestamp_validation()
        .decode();

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Claim169Error::Decompress(_)));
}

#[test]
fn test_not_cose_rejected() {
    // Valid CBOR but not COSE
    let cbor_array = Value::Array(vec![
        Value::Text("hello".to_string()),
        Value::Integer(42.into()),
    ]);

    let mut cbor_bytes = Vec::new();
    ciborium::into_writer(&cbor_array, &mut cbor_bytes).unwrap();

    let compressed = claim169_core::pipeline::decompress::compress(&cbor_bytes);
    let qr_data = claim169_core::pipeline::base45::encode(&compressed);

    let result = Decoder::new(&qr_data)
        .allow_unverified()
        .without_timestamp_validation()
        .decode();

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Claim169Error::CoseParse(_)));
}

#[test]
fn test_missing_claim_169_rejected() {
    // CWT without claim 169
    let cwt_map = Value::Map(vec![
        (
            Value::Integer(1.into()),
            Value::Text("https://example.org".to_string()),
        ),
        (Value::Integer(4.into()), Value::Integer(2000000000.into())),
        (Value::Integer(6.into()), Value::Integer(1700000000.into())),
    ]);

    let mut cwt_bytes = Vec::new();
    ciborium::into_writer(&cwt_map, &mut cwt_bytes).unwrap();

    let sign1 = CoseSign1Builder::new()
        .protected(
            HeaderBuilder::new()
                .algorithm(iana::Algorithm::EdDSA)
                .build(),
        )
        .payload(cwt_bytes)
        .build();

    let cose_bytes = sign1.to_tagged_vec().unwrap();
    let compressed = claim169_core::pipeline::decompress::compress(&cose_bytes);
    let qr_data = claim169_core::pipeline::base45::encode(&compressed);

    let result = Decoder::new(&qr_data)
        .allow_unverified()
        .without_timestamp_validation()
        .decode();

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        Claim169Error::Claim169NotFound
    ));
}

// ============================================================================
// Decoder Options Tests
// ============================================================================

#[test]
fn test_skip_biometrics_option() {
    // Create claim with face biometrics
    let bio_map = Value::Map(vec![
        (
            Value::Integer(0.into()),
            Value::Bytes(vec![0xDE, 0xAD, 0xBE, 0xEF]),
        ),
        (Value::Integer(1.into()), Value::Integer(0.into())), // Image
        (Value::Integer(2.into()), Value::Integer(1.into())), // JPEG
    ]);

    let claim_169 = Value::Map(vec![
        (
            Value::Integer(1.into()),
            Value::Text("ID-SKIP-BIO".to_string()),
        ),
        (
            Value::Integer(4.into()),
            Value::Text("Skip Bio Person".to_string()),
        ),
        (Value::Integer(62.into()), Value::Array(vec![bio_map])), // Face biometrics
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(2000000000);

    let qr_data = encode_unsigned_qr(&meta, &claim_169);

    // First decode without skip_biometrics - should have biometrics
    let result_with_bio = Decoder::new(&qr_data)
        .allow_unverified()
        .without_timestamp_validation()
        .decode()
        .expect("decode should succeed");
    assert!(result_with_bio.claim169.face.is_some());
    assert!(result_with_bio.claim169.has_biometrics());

    // Now decode with skip_biometrics - should NOT have biometrics
    let result_without_bio = Decoder::new(&qr_data)
        .allow_unverified()
        .without_timestamp_validation()
        .skip_biometrics()
        .decode()
        .expect("decode should succeed");

    assert!(result_without_bio.claim169.face.is_none());
    assert!(!result_without_bio.claim169.has_biometrics());
    assert_eq!(
        result_without_bio.claim169.id,
        Some("ID-SKIP-BIO".to_string())
    );
    assert_eq!(
        result_without_bio.claim169.full_name,
        Some("Skip Bio Person".to_string())
    );

    // Should have a warning about biometrics being skipped
    assert!(result_without_bio
        .warnings
        .iter()
        .any(|w| { matches!(w.code, claim169_core::WarningCode::BiometricsSkipped) }));
}

#[test]
fn test_max_decompressed_bytes_enforced() {
    // Create a moderately sized claim
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-DECOMPRESS-TEST".to_string())),
        (4, Value::Text("A".repeat(5000))), // 5KB of 'A's in name
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(2000000000);

    let qr_data = encode_unsigned_qr(&meta, &claim_169);

    // With default large limit, should work
    let result = Decoder::new(&qr_data)
        .allow_unverified()
        .without_timestamp_validation()
        .max_decompressed_bytes(100_000) // 100KB limit
        .decode();
    assert!(result.is_ok());

    // With very small limit, should fail
    let result = Decoder::new(&qr_data)
        .allow_unverified()
        .without_timestamp_validation()
        .max_decompressed_bytes(100) // Only 100 bytes allowed
        .decode();

    assert!(result.is_err());
    match result.unwrap_err() {
        Claim169Error::DecompressLimitExceeded { max_bytes } => {
            assert_eq!(max_bytes, 100);
        }
        Claim169Error::Decompress(msg) => {
            assert!(
                msg.contains("limit") || msg.contains("exceeded"),
                "Error should mention limit: {}",
                msg
            );
        }
        e => panic!("Expected Decompress error, got: {:?}", e),
    }
}

#[test]
fn test_clock_skew_tolerance() {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-SKEW".to_string())),
        (4, Value::Text("Clock Skew Person".to_string())),
    ]);

    // Create a credential that expired 2 minutes ago
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(now - 3600) // Issued 1 hour ago
        .with_expires_at(now - 120); // Expired 2 minutes ago

    let qr_data = encode_unsigned_qr(&meta, &claim_169);

    // Without tolerance, should fail
    let result = Decoder::new(&qr_data).allow_unverified().decode();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Claim169Error::Expired(_)));

    // With 5 minute tolerance, should succeed
    let result = Decoder::new(&qr_data)
        .allow_unverified()
        .clock_skew_tolerance(300) // 5 minutes
        .decode();
    assert!(result.is_ok());
}
