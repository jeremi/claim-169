//! Security regression tests for claim169-core
//!
//! These tests focus on ambiguous / underspecified encodings that can lead to
//! cross-implementation inconsistencies or surprising validation outcomes.
//!
//! Note: Some of these cases require the issuer to have produced a malformed or
//! ambiguous token (because the payload is signed). They are still useful as
//! robustness tests to ensure the library fails closed on malformed inputs.

use ciborium::{value::Integer, Value};
use claim169_core::{Claim169Error, Decoder, Ed25519Signer, Signer};
use coset::{iana, CoseSign1Builder, HeaderBuilder, TaggedCborSerializable};

fn encode_cbor(value: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    ciborium::into_writer(value, &mut out).expect("CBOR encoding should not fail");
    out
}

fn create_claim169_map(entries: Vec<(i64, Value)>) -> Value {
    Value::Map(
        entries
            .into_iter()
            .map(|(k, v)| (Value::Integer(k.into()), v))
            .collect(),
    )
}

fn create_signed_qr(cwt_bytes: Vec<u8>) -> (String, [u8; 32]) {
    let signer = Ed25519Signer::generate();
    let public_key: [u8; 32] = signer.public_key_bytes();

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
    let qr_data = claim169_core::pipeline::base45::encode(&compressed);

    (qr_data, public_key)
}

#[test]
fn security_duplicate_cwt_claim_keys_rejected() {
    // Two `exp` claims: first is already expired, second is far future.
    // Duplicate keys are ambiguous across implementations; reject (fail closed).
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-DUP-CWT".to_string())),
        (4, Value::Text("Duplicate CWT Claims".to_string())),
    ]);

    let cwt = Value::Map(vec![
        (Value::Integer(4.into()), Value::Integer(1.into())), // exp (expired)
        (
            Value::Integer(4.into()),
            Value::Integer(4_102_444_800i64.into()), // exp (2100-01-01)
        ),
        (Value::Integer(169.into()), claim_169),
    ]);

    let (qr, public_key) = create_signed_qr(encode_cbor(&cwt));

    let err = Decoder::new(qr)
        .verify_with_ed25519(&public_key)
        .unwrap()
        .decode()
        .unwrap_err();

    assert!(matches!(err, Claim169Error::CwtParse(_)));
}

#[test]
fn security_duplicate_claim169_keys_rejected() {
    // Duplicate Claim169 field keys are ambiguous; reject (fail closed).
    let claim_169 = Value::Map(vec![
        (
            Value::Integer(1.into()),
            Value::Text("ID-DUP-169".to_string()),
        ),
        (
            Value::Integer(4.into()),
            Value::Text("First Name Value".to_string()),
        ),
        (
            Value::Integer(4.into()),
            Value::Text("Second Name Value".to_string()),
        ),
    ]);

    let cwt = Value::Map(vec![(Value::Integer(169.into()), claim_169)]);
    let (qr, public_key) = create_signed_qr(encode_cbor(&cwt));

    let err = Decoder::new(qr)
        .verify_with_ed25519(&public_key)
        .unwrap()
        .decode()
        .unwrap_err();

    assert!(matches!(err, Claim169Error::Claim169Invalid(_)));
}

#[test]
fn security_out_of_range_nbf_rejected() {
    // `nbf` is a CWT NumericDate and can exceed i64 range when encoded as CBOR integer.
    // Out-of-range timestamps are rejected (fail closed).

    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-NBF-RANGE".to_string())),
        (4, Value::Text("Out of range nbf".to_string())),
    ]);

    // Control: a future nbf within i64 range should reject as NotYetValid.
    let cwt_future_nbf = Value::Map(vec![
        (
            Value::Integer(5.into()),
            Value::Integer(4_102_444_800i64.into()), // nbf (2100-01-01)
        ),
        (Value::Integer(169.into()), claim_169.clone()),
    ]);
    let (qr_future, public_key_future) = create_signed_qr(encode_cbor(&cwt_future_nbf));
    let err = Decoder::new(qr_future)
        .verify_with_ed25519(&public_key_future)
        .unwrap()
        .decode()
        .unwrap_err();
    assert!(matches!(err, Claim169Error::NotYetValid(_)));

    // Out-of-range nbf should be rejected during CWT parsing.
    let nbf_out_of_range: i128 = i64::MAX as i128 + 1;
    let cwt_oob_nbf = Value::Map(vec![
        (
            Value::Integer(5.into()),
            Value::Integer(Integer::try_from(nbf_out_of_range).unwrap()),
        ), // nbf (dropped)
        (Value::Integer(169.into()), claim_169),
    ]);
    let (qr_oob, public_key_oob) = create_signed_qr(encode_cbor(&cwt_oob_nbf));

    let err = Decoder::new(qr_oob)
        .verify_with_ed25519(&public_key_oob)
        .unwrap()
        .decode()
        .unwrap_err();

    assert!(matches!(err, Claim169Error::CwtParse(_)));
}
