//! Security edge case tests for claim169-core
//!
//! These tests verify that security vulnerabilities and edge cases are properly handled.

use ciborium::Value;
use claim169_core::{
    Claim169Error, CwtMeta, Decoder, EcdsaP256Signer, EcdsaP256Verifier, Ed25519Signer,
    Ed25519Verifier, Signer,
};
use coset::{iana, CoseSign1Builder, HeaderBuilder, TaggedCborSerializable};

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

// ============================================================================
// Signature Security Tests
// ============================================================================

#[test]
fn test_empty_signature_rejected() {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-EMPTY-SIG".to_string())),
        (4, Value::Text("Empty Sig Person".to_string())),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(2000000000);

    let cwt_bytes = encode_cwt(&meta, &claim_169);

    // Create COSE_Sign1 with empty signature
    let sign1 = CoseSign1Builder::new()
        .protected(
            HeaderBuilder::new()
                .algorithm(iana::Algorithm::EdDSA)
                .build(),
        )
        .payload(cwt_bytes)
        .signature(vec![]) // Empty signature!
        .build();

    let cose_bytes = sign1.to_tagged_vec().unwrap();
    let compressed = claim169_core::pipeline::decompress::compress(&cose_bytes);
    let qr_data = claim169_core::pipeline::base45::encode(&compressed);

    // Try to verify - should fail
    let signer = Ed25519Signer::generate();
    let public_key = signer.public_key_bytes();

    let result = Decoder::new(&qr_data)
        .verify_with_ed25519(&public_key)
        .unwrap()
        .decode();

    assert!(result.is_err());
    match result.unwrap_err() {
        Claim169Error::SignatureInvalid(_) => {}
        e => panic!("Expected SignatureInvalid error, got: {:?}", e),
    }
}

#[test]
fn test_truncated_signature_rejected() {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-TRUNCATED-SIG".to_string())),
        (4, Value::Text("Truncated Sig Person".to_string())),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(2000000000);

    let cwt_bytes = encode_cwt(&meta, &claim_169);

    // Create COSE_Sign1 with truncated signature (only 32 bytes instead of 64)
    let sign1 = CoseSign1Builder::new()
        .protected(
            HeaderBuilder::new()
                .algorithm(iana::Algorithm::EdDSA)
                .build(),
        )
        .payload(cwt_bytes)
        .signature(vec![0u8; 32]) // Truncated - should be 64 bytes for Ed25519
        .build();

    let cose_bytes = sign1.to_tagged_vec().unwrap();
    let compressed = claim169_core::pipeline::decompress::compress(&cose_bytes);
    let qr_data = claim169_core::pipeline::base45::encode(&compressed);

    let signer = Ed25519Signer::generate();
    let public_key = signer.public_key_bytes();

    let result = Decoder::new(&qr_data)
        .verify_with_ed25519(&public_key)
        .unwrap()
        .decode();

    assert!(result.is_err());
    match result.unwrap_err() {
        Claim169Error::SignatureInvalid(_) => {}
        e => panic!("Expected SignatureInvalid error, got: {:?}", e),
    }
}

#[test]
fn test_corrupted_signature_rejected() {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-CORRUPT-SIG".to_string())),
        (4, Value::Text("Corrupt Sig Person".to_string())),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(2000000000);

    let cwt_bytes = encode_cwt(&meta, &claim_169);

    // Generate a real key pair
    let signer = Ed25519Signer::generate();
    let public_key = signer.public_key_bytes();

    // Build properly signed COSE_Sign1
    let protected = HeaderBuilder::new()
        .algorithm(iana::Algorithm::EdDSA)
        .build();

    let mut sign1 = CoseSign1Builder::new()
        .protected(protected)
        .payload(cwt_bytes)
        .build();

    let tbs_data = sign1.tbs_data(&[]);
    let mut signature = signer
        .sign(iana::Algorithm::EdDSA, None, &tbs_data)
        .expect("signing failed");

    // Corrupt the signature by flipping bits
    signature[0] ^= 0xFF;
    signature[31] ^= 0xFF;
    sign1.signature = signature;

    let cose_bytes = sign1.to_tagged_vec().unwrap();
    let compressed = claim169_core::pipeline::decompress::compress(&cose_bytes);
    let qr_data = claim169_core::pipeline::base45::encode(&compressed);

    let result = Decoder::new(&qr_data)
        .verify_with_ed25519(&public_key)
        .unwrap()
        .decode();

    assert!(result.is_err());
    match result.unwrap_err() {
        Claim169Error::SignatureInvalid(_) => {}
        e => panic!("Expected SignatureInvalid error, got: {:?}", e),
    }
}

// ============================================================================
// Algorithm Security Tests
// ============================================================================

#[test]
fn test_missing_algorithm_rejected_with_verifier() {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-NO-ALG".to_string())),
        (4, Value::Text("No Algorithm Person".to_string())),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(2000000000);

    let cwt_bytes = encode_cwt(&meta, &claim_169);

    // Create COSE_Sign1 WITHOUT algorithm header
    let sign1 = CoseSign1Builder::new()
        .protected(HeaderBuilder::new().build()) // No algorithm!
        .payload(cwt_bytes)
        .signature(vec![0u8; 64])
        .build();

    let cose_bytes = sign1.to_tagged_vec().unwrap();
    let compressed = claim169_core::pipeline::decompress::compress(&cose_bytes);
    let qr_data = claim169_core::pipeline::base45::encode(&compressed);

    let signer = Ed25519Signer::generate();
    let public_key = signer.public_key_bytes();

    let result = Decoder::new(&qr_data)
        .verify_with_ed25519(&public_key)
        .unwrap()
        .decode();

    assert!(result.is_err());
    match result.unwrap_err() {
        Claim169Error::CoseParse(msg) => {
            assert!(
                msg.contains("algorithm"),
                "Error should mention algorithm: {}",
                msg
            );
        }
        e => panic!("Expected CoseParse error about algorithm, got: {:?}", e),
    }
}

// ============================================================================
// Algorithm Confusion Attack Tests
// ============================================================================

#[test]
fn test_eddsa_signed_ecdsa_verifier_rejected() {
    // Sign with EdDSA but try to verify with ECDSA - should be rejected
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-ALGO-CONFUSION-1".to_string())),
        (4, Value::Text("Algorithm Confusion Person".to_string())),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(2000000000);

    let cwt_bytes = encode_cwt(&meta, &claim_169);

    // Sign with EdDSA
    let ed25519_signer = Ed25519Signer::generate();
    let protected = HeaderBuilder::new()
        .algorithm(iana::Algorithm::EdDSA)
        .build();

    let mut sign1 = CoseSign1Builder::new()
        .protected(protected)
        .payload(cwt_bytes)
        .build();

    let tbs_data = sign1.tbs_data(&[]);
    let signature = ed25519_signer
        .sign(iana::Algorithm::EdDSA, None, &tbs_data)
        .expect("signing failed");
    sign1.signature = signature;

    let cose_bytes = sign1.to_tagged_vec().unwrap();
    let compressed = claim169_core::pipeline::decompress::compress(&cose_bytes);
    let qr_data = claim169_core::pipeline::base45::encode(&compressed);

    // Try to verify with ECDSA verifier - should fail due to algorithm mismatch
    let ecdsa_signer = EcdsaP256Signer::generate();
    let public_key = ecdsa_signer.public_key_uncompressed();

    let result = Decoder::new(&qr_data)
        .verify_with_ecdsa_p256(&public_key)
        .unwrap()
        .decode();

    assert!(result.is_err());
    match result.unwrap_err() {
        Claim169Error::UnsupportedAlgorithm(msg) => {
            // Best case: algorithm mismatch detected early
            assert!(
                msg.contains("EdDSA") || msg.contains("algorithm"),
                "Error should indicate algorithm: {}",
                msg
            );
        }
        Claim169Error::SignatureInvalid(msg) => {
            // The error should indicate algorithm mismatch or verification failure
            assert!(
                msg.contains("verification failed")
                    || msg.contains("algorithm")
                    || msg.contains("Unsupported"),
                "Error should indicate algorithm issue: {}",
                msg
            );
        }
        Claim169Error::Crypto(msg) => {
            assert!(
                msg.contains("Unsupported") || msg.contains("algorithm"),
                "Crypto error should indicate unsupported algorithm: {}",
                msg
            );
        }
        e => panic!(
            "Expected UnsupportedAlgorithm, SignatureInvalid or Crypto error, got: {:?}",
            e
        ),
    }
}

#[test]
fn test_ecdsa_signed_eddsa_verifier_rejected() {
    // Sign with ECDSA but try to verify with EdDSA - should be rejected
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-ALGO-CONFUSION-2".to_string())),
        (4, Value::Text("Algorithm Confusion Person 2".to_string())),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(2000000000);

    let cwt_bytes = encode_cwt(&meta, &claim_169);

    // Sign with ECDSA (ES256)
    let ecdsa_signer = EcdsaP256Signer::generate();
    let protected = HeaderBuilder::new()
        .algorithm(iana::Algorithm::ES256)
        .build();

    let mut sign1 = CoseSign1Builder::new()
        .protected(protected)
        .payload(cwt_bytes)
        .build();

    let tbs_data = sign1.tbs_data(&[]);
    let signature = ecdsa_signer
        .sign(iana::Algorithm::ES256, None, &tbs_data)
        .expect("signing failed");
    sign1.signature = signature;

    let cose_bytes = sign1.to_tagged_vec().unwrap();
    let compressed = claim169_core::pipeline::decompress::compress(&cose_bytes);
    let qr_data = claim169_core::pipeline::base45::encode(&compressed);

    // Try to verify with EdDSA verifier - should fail due to algorithm mismatch
    let ed25519_signer = Ed25519Signer::generate();
    let public_key = ed25519_signer.public_key_bytes();

    let result = Decoder::new(&qr_data)
        .verify_with_ed25519(&public_key)
        .unwrap()
        .decode();

    assert!(result.is_err());
    match result.unwrap_err() {
        Claim169Error::UnsupportedAlgorithm(msg) => {
            // Best case: algorithm mismatch detected early
            assert!(
                msg.contains("ES256") || msg.contains("algorithm"),
                "Error should indicate algorithm: {}",
                msg
            );
        }
        Claim169Error::SignatureInvalid(msg) => {
            assert!(
                msg.contains("verification failed")
                    || msg.contains("algorithm")
                    || msg.contains("Unsupported"),
                "Error should indicate algorithm issue: {}",
                msg
            );
        }
        Claim169Error::Crypto(msg) => {
            assert!(
                msg.contains("Unsupported") || msg.contains("algorithm"),
                "Crypto error should indicate unsupported algorithm: {}",
                msg
            );
        }
        e => panic!(
            "Expected UnsupportedAlgorithm, SignatureInvalid or Crypto error, got: {:?}",
            e
        ),
    }
}

// ============================================================================
// ECDSA P-256 Full Pipeline Tests
// ============================================================================

#[test]
fn test_ecdsa_p256_full_pipeline_sign_verify() {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-ECDSA-P256".to_string())),
        (4, Value::Text("ECDSA P-256 Person".to_string())),
        (8, Value::Text("19850615".to_string())),
        (9, Value::Integer(2.into())), // Female
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://ecdsa.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(2000000000);

    let cwt_bytes = encode_cwt(&meta, &claim_169);

    // Generate ECDSA P-256 key pair
    let signer = EcdsaP256Signer::generate();
    let public_key = signer.public_key_uncompressed();

    // Build and sign COSE_Sign1
    let protected = HeaderBuilder::new()
        .algorithm(iana::Algorithm::ES256)
        .build();

    let mut sign1 = CoseSign1Builder::new()
        .protected(protected)
        .payload(cwt_bytes)
        .build();

    let tbs_data = sign1.tbs_data(&[]);
    let signature = signer
        .sign(iana::Algorithm::ES256, None, &tbs_data)
        .expect("ECDSA signing failed");
    sign1.signature = signature;

    let cose_bytes = sign1.to_tagged_vec().unwrap();
    let compressed = claim169_core::pipeline::decompress::compress(&cose_bytes);
    let qr_data = claim169_core::pipeline::base45::encode(&compressed);

    // Decode with verification
    let result = Decoder::new(&qr_data)
        .verify_with_ecdsa_p256(&public_key)
        .unwrap()
        .decode()
        .expect("ECDSA verification should succeed");

    assert_eq!(result.claim169.id, Some("ID-ECDSA-P256".to_string()));
    assert_eq!(
        result.claim169.full_name,
        Some("ECDSA P-256 Person".to_string())
    );
    assert_eq!(
        result.verification_status,
        claim169_core::VerificationStatus::Verified
    );
}

#[test]
fn test_ecdsa_p256_wrong_verifier_fails() {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-ECDSA-WRONG-KEY".to_string())),
        (4, Value::Text("ECDSA Wrong Key Person".to_string())),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(2000000000);

    let cwt_bytes = encode_cwt(&meta, &claim_169);

    // Sign with one ECDSA key
    let signer = EcdsaP256Signer::generate();

    let protected = HeaderBuilder::new()
        .algorithm(iana::Algorithm::ES256)
        .build();

    let mut sign1 = CoseSign1Builder::new()
        .protected(protected)
        .payload(cwt_bytes)
        .build();

    let tbs_data = sign1.tbs_data(&[]);
    let signature = signer
        .sign(iana::Algorithm::ES256, None, &tbs_data)
        .expect("signing failed");
    sign1.signature = signature;

    let cose_bytes = sign1.to_tagged_vec().unwrap();
    let compressed = claim169_core::pipeline::decompress::compress(&cose_bytes);
    let qr_data = claim169_core::pipeline::base45::encode(&compressed);

    // Try to verify with different ECDSA key
    let wrong_signer = EcdsaP256Signer::generate();
    let wrong_public_key = wrong_signer.public_key_uncompressed();

    let result = Decoder::new(&qr_data)
        .verify_with_ecdsa_p256(&wrong_public_key)
        .unwrap()
        .decode();

    assert!(result.is_err());
    match result.unwrap_err() {
        Claim169Error::SignatureInvalid(_) => {}
        e => panic!("Expected SignatureInvalid error, got: {:?}", e),
    }
}

// ============================================================================
// Decompression Security Tests
// ============================================================================

#[test]
fn test_zip_bomb_protection() {
    // Create a large string that compresses very well (simulated zip bomb)
    // The real payload when decompressed would be much larger than allowed
    let large_payload = "A".repeat(100_000); // 100KB of repeated 'A'

    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-ZIPBOMB".to_string())),
        (4, Value::Text(large_payload)),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(2000000000);

    let cwt_bytes = encode_cwt(&meta, &claim_169);

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

    // Use builder with small decompression limit
    let result = Decoder::new(&qr_data)
        .max_decompressed_bytes(1024) // Only 1KB allowed
        .allow_unverified()
        .decode();

    assert!(result.is_err());
    match result.unwrap_err() {
        Claim169Error::DecompressLimitExceeded { max_bytes } => {
            assert_eq!(max_bytes, 1024);
        }
        Claim169Error::Decompress(msg) => {
            assert!(
                msg.contains("limit") || msg.contains("exceeded"),
                "Error should mention limit exceeded: {}",
                msg
            );
        }
        e => panic!(
            "Expected Decompress or DecompressLimitExceeded error, got: {:?}",
            e
        ),
    }
}

// ============================================================================
// CBOR Security Tests
// ============================================================================

#[test]
fn test_deeply_nested_cbor_rejected() {
    // Create a deeply nested CBOR structure
    fn create_nested(depth: usize) -> Value {
        if depth == 0 {
            Value::Text("leaf".to_string())
        } else {
            Value::Array(vec![create_nested(depth - 1)])
        }
    }

    // Claim 169 with deeply nested content (150 levels - exceeds 128 limit)
    let claim_169 = Value::Map(vec![
        (Value::Integer(1.into()), Value::Text("ID-DEEP".to_string())),
        (Value::Integer(4.into()), create_nested(150)),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(2000000000);

    let cwt_bytes = encode_cwt(&meta, &claim_169);

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
    match result.unwrap_err() {
        Claim169Error::CborParse(msg) => {
            assert!(msg.contains("depth"), "Error should mention depth: {}", msg);
        }
        e => panic!("Expected CborParse error about depth, got: {:?}", e),
    }
}

// ============================================================================
// Key Parsing Security Tests
// ============================================================================

#[test]
fn test_invalid_public_key_rejected() {
    // Try to create verifier with invalid key bytes
    let invalid_key = vec![0u8; 16]; // Wrong length for Ed25519

    let result = Ed25519Verifier::from_bytes(&invalid_key);

    assert!(result.is_err());
}

#[test]
fn test_all_zeros_key_rejected() {
    // All-zero keys should be rejected by the crypto library
    let zero_key = [0u8; 32];

    // Ed25519 should reject this as it's not on the curve
    let result = Ed25519Verifier::from_bytes(&zero_key);

    // The ed25519-dalek library accepts the all-zeros key technically,
    // but verification will fail. This test documents the behavior.
    // In production, key distribution should validate keys.
    if result.is_ok() {
        // If the library accepts it, verify that verification fails

        let claim_169 = create_claim169_map(vec![
            (1, Value::Text("ID-ZERO-KEY".to_string())),
            (4, Value::Text("Zero Key Person".to_string())),
        ]);

        let meta = CwtMeta::new()
            .with_issuer("https://test.example.org")
            .with_issued_at(1700000000)
            .with_expires_at(2000000000);

        let cwt_bytes = encode_cwt(&meta, &claim_169);

        let sign1 = CoseSign1Builder::new()
            .protected(
                HeaderBuilder::new()
                    .algorithm(iana::Algorithm::EdDSA)
                    .build(),
            )
            .payload(cwt_bytes)
            .signature(vec![0u8; 64])
            .build();

        let cose_bytes = sign1.to_tagged_vec().unwrap();
        let compressed = claim169_core::pipeline::decompress::compress(&cose_bytes);
        let qr_data = claim169_core::pipeline::base45::encode(&compressed);

        let result = Decoder::new(&qr_data)
            .verify_with_ed25519(&zero_key)
            .unwrap()
            .decode();

        // Verification should fail with zero key
        assert!(result.is_err());
    }
}

#[test]
fn test_invalid_pem_rejected() {
    // Invalid PEM should be rejected
    let invalid_pem = "-----BEGIN PUBLIC KEY-----\nnotvalidbase64!!!\n-----END PUBLIC KEY-----";

    let result = Ed25519Verifier::from_pem(invalid_pem);

    assert!(result.is_err());
}

#[test]
fn test_ecdsa_invalid_public_key_rejected() {
    // Try to create ECDSA verifier with invalid key bytes
    let invalid_key = vec![0u8; 16]; // Wrong length for P-256

    let result = EcdsaP256Verifier::from_sec1_bytes(&invalid_key);

    assert!(result.is_err());
}

// ============================================================================
// Tampered Content Tests
// ============================================================================

#[test]
fn test_tampered_payload_detected() {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-TAMPER".to_string())),
        (4, Value::Text("Original Name".to_string())),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://test.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(2000000000);

    let cwt_bytes = encode_cwt(&meta, &claim_169);

    // Generate a real signature
    let signer = Ed25519Signer::generate();
    let public_key = signer.public_key_bytes();

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

    // Now tamper with the payload
    let tampered_claim = create_claim169_map(vec![
        (1, Value::Text("ID-TAMPER".to_string())),
        (4, Value::Text("TAMPERED Name".to_string())), // Changed!
    ]);

    let tampered_cwt = encode_cwt(&meta, &tampered_claim);
    sign1.payload = Some(tampered_cwt);

    let cose_bytes = sign1.to_tagged_vec().unwrap();
    let compressed = claim169_core::pipeline::decompress::compress(&cose_bytes);
    let qr_data = claim169_core::pipeline::base45::encode(&compressed);

    // Verification should fail because signature doesn't match tampered content
    let result = Decoder::new(&qr_data)
        .verify_with_ed25519(&public_key)
        .unwrap()
        .decode();

    assert!(result.is_err());
    match result.unwrap_err() {
        Claim169Error::SignatureInvalid(_) => {}
        e => panic!("Expected SignatureInvalid error, got: {:?}", e),
    }
}
