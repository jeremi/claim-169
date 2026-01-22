//! Test vector generator for MOSIP Claim 169
//!
//! Generates synthetic test vectors for testing the Claim 169 decoder library.

use std::fs;
use std::path::{Path, PathBuf};

use ciborium::Value;
use claim169_core::crypto::software::{AesGcmEncryptor, EcdsaP256Signer, Ed25519Signer};
use claim169_core::crypto::traits::{Encryptor, Signer};
use claim169_core::model::CwtMeta;
use claim169_core::pipeline::{base45, cwt, decompress};
use clap::{Parser, Subcommand};
use coset::{
    iana, CborSerializable, CoseEncrypt0Builder, CoseSign1Builder, HeaderBuilder,
    TaggedCborSerializable,
};
use serde::{Deserialize, Serialize};

/// Test vector generator for MOSIP Claim 169
#[derive(Parser)]
#[command(name = "generate-vectors")]
#[command(about = "Generate test vectors for Claim 169 decoder")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate all test vectors
    All {
        /// Output directory
        #[arg(short, long, default_value = "test-vectors")]
        output: PathBuf,
    },
    /// Generate a single test vector
    Single {
        /// Vector name (e.g., "minimal", "full-demographics")
        name: String,

        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// List available vector types
    List,
}

/// Test vector metadata
#[derive(Serialize, Deserialize)]
struct TestVector {
    name: String,
    description: String,
    category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_error: Option<String>,
    qr_data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    signing_key: Option<KeyMaterial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key: Option<KeyMaterial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_claim169: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_cwt_meta: Option<CwtMeta>,
}

#[derive(Serialize, Deserialize)]
struct KeyMaterial {
    algorithm: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key_hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_key_hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symmetric_key_hex: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::All { output } => generate_all(&output),
        Commands::Single { name, output } => generate_single(&name, output.as_deref()),
        Commands::List => list_vectors(),
    }
}

fn generate_all(output_dir: &Path) {
    // Create directory structure
    let valid_dir = output_dir.join("valid");
    let invalid_dir = output_dir.join("invalid");
    let edge_dir = output_dir.join("edge");

    fs::create_dir_all(&valid_dir).expect("Failed to create valid directory");
    fs::create_dir_all(&invalid_dir).expect("Failed to create invalid directory");
    fs::create_dir_all(&edge_dir).expect("Failed to create edge directory");

    // Generate valid vectors
    let valid_vectors = vec![
        ("minimal", generate_minimal()),
        ("demographics-full", generate_full_demographics()),
        ("with-face", generate_with_face()),
        ("with-fingerprints", generate_with_fingerprints()),
        ("with-all-biometrics", generate_with_all_biometrics()),
        ("claim169-example", generate_claim169_example()),
        ("ed25519-signed", generate_ed25519_signed()),
        ("ecdsa-p256-signed", generate_ecdsa_p256_signed()),
        ("encrypted-aes256", generate_encrypted_aes256()),
        ("encrypted-signed", generate_encrypted_signed()),
    ];

    for (name, vector) in valid_vectors {
        let path = valid_dir.join(format!("{}.json", name));
        let json = serde_json::to_string_pretty(&vector).unwrap();
        fs::write(&path, json).expect("Failed to write vector");
        println!("Generated: {}", path.display());
    }

    // Generate invalid vectors
    let invalid_vectors = vec![
        ("bad-base45", generate_bad_base45()),
        ("bad-zlib", generate_bad_zlib()),
        ("not-cose", generate_not_cose()),
        ("missing-169", generate_missing_169()),
    ];

    for (name, vector) in invalid_vectors {
        let path = invalid_dir.join(format!("{}.json", name));
        let json = serde_json::to_string_pretty(&vector).unwrap();
        fs::write(&path, json).expect("Failed to write vector");
        println!("Generated: {}", path.display());
    }

    // Generate edge case vectors
    let edge_vectors = vec![
        ("expired", generate_expired()),
        ("not-yet-valid", generate_not_yet_valid()),
        ("unknown-fields", generate_unknown_fields()),
    ];

    for (name, vector) in edge_vectors {
        let path = edge_dir.join(format!("{}.json", name));
        let json = serde_json::to_string_pretty(&vector).unwrap();
        fs::write(&path, json).expect("Failed to write vector");
        println!("Generated: {}", path.display());
    }

    println!("\nGenerated {} test vectors", 17);
}

fn generate_single(name: &str, output: Option<&std::path::Path>) {
    let vector = match name {
        "minimal" => generate_minimal(),
        "demographics-full" => generate_full_demographics(),
        "with-face" => generate_with_face(),
        "with-fingerprints" => generate_with_fingerprints(),
        "with-all-biometrics" => generate_with_all_biometrics(),
        "claim169-example" => generate_claim169_example(),
        "ed25519-signed" => generate_ed25519_signed(),
        "ecdsa-p256-signed" => generate_ecdsa_p256_signed(),
        "encrypted-aes256" => generate_encrypted_aes256(),
        "encrypted-signed" => generate_encrypted_signed(),
        "bad-base45" => generate_bad_base45(),
        "bad-zlib" => generate_bad_zlib(),
        "not-cose" => generate_not_cose(),
        "missing-169" => generate_missing_169(),
        "expired" => generate_expired(),
        "not-yet-valid" => generate_not_yet_valid(),
        "unknown-fields" => generate_unknown_fields(),
        _ => {
            eprintln!("Unknown vector: {}", name);
            eprintln!("Use 'generate-vectors list' to see available vectors");
            std::process::exit(1);
        }
    };

    let json = serde_json::to_string_pretty(&vector).unwrap();

    if let Some(path) = output {
        fs::write(path, json).expect("Failed to write vector");
        println!("Generated: {}", path.display());
    } else {
        println!("{}", json);
    }
}

fn list_vectors() {
    println!("Available test vectors:\n");
    println!("Valid:");
    println!("  minimal             - Minimal claim with ID and full name only");
    println!("  demographics-full   - All demographic fields (1-23)");
    println!("  with-face           - Face biometric included");
    println!("  with-fingerprints   - Multiple fingerprint biometrics");
    println!("  with-all-biometrics - All biometric types included");
    println!("  claim169-example    - Example from claim_169.md specification");
    println!("  ed25519-signed      - COSE_Sign1 with Ed25519 signature");
    println!("  ecdsa-p256-signed   - COSE_Sign1 with ECDSA P-256 signature");
    println!("  encrypted-aes256    - COSE_Encrypt0 with AES-256-GCM");
    println!("  encrypted-signed    - COSE_Encrypt0 containing signed COSE_Sign1");
    println!();
    println!("Invalid:");
    println!("  bad-base45          - Invalid Base45 encoding");
    println!("  bad-zlib            - Invalid zlib data");
    println!("  not-cose            - Valid CBOR but not COSE");
    println!("  missing-169         - CWT without claim 169");
    println!();
    println!("Edge cases:");
    println!("  expired             - Token with exp in past");
    println!("  not-yet-valid       - Token with nbf in future");
    println!("  unknown-fields      - Contains unknown CBOR keys");
}

// ============================================================================
// Valid Vectors
// ============================================================================

fn generate_minimal() -> TestVector {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-12345-ABCDE".to_string())),
        (4, Value::Text("John Doe".to_string())),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://mosip.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(1800000000);

    let qr_data = encode_unsigned_qr(&meta, &claim_169);

    TestVector {
        name: "minimal".to_string(),
        description: "Minimal claim with ID and full name only".to_string(),
        category: "valid".to_string(),
        expected_error: None,
        qr_data,
        signing_key: None,
        encryption_key: None,
        expected_claim169: Some(serde_json::json!({
            "id": "ID-12345-ABCDE",
            "fullName": "John Doe"
        })),
        expected_cwt_meta: Some(meta),
    }
}

fn generate_full_demographics() -> TestVector {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-67890-FGHIJ".to_string())),
        (2, Value::Text("1.0".to_string())),
        (3, Value::Text("eng".to_string())),
        (4, Value::Text("Jane Marie Smith".to_string())),
        (5, Value::Text("Jane".to_string())),
        (6, Value::Text("Marie".to_string())),
        (7, Value::Text("Smith".to_string())),
        (8, Value::Text("19900515".to_string())),
        (9, Value::Integer(2.into())), // Female
        (
            10,
            Value::Text("123 Main St\nApt 4\nNew York, NY 10001".to_string()),
        ),
        (11, Value::Text("jane.smith@example.com".to_string())),
        (12, Value::Text("+1 555 123 4567".to_string())),
        (13, Value::Text("USA".to_string())),
        (14, Value::Integer(2.into())), // Married
        (15, Value::Text("Guardian-ID-001".to_string())),
        (17, Value::Integer(1.into())), // JPEG
        (19, Value::Text("Jane Marie Smith (Hindi)".to_string())),
        (20, Value::Text("hin".to_string())),
        (21, Value::Text("US-NY-NYC".to_string())),
        (22, Value::Text("citizen".to_string())),
        (23, Value::Text("USA".to_string())),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://mosip.example.org")
        .with_subject("ID-67890-FGHIJ")
        .with_issued_at(1700000000)
        .with_expires_at(1800000000)
        .with_not_before(1700000000);

    let qr_data = encode_unsigned_qr(&meta, &claim_169);

    TestVector {
        name: "demographics-full".to_string(),
        description: "All demographic fields (1-23)".to_string(),
        category: "valid".to_string(),
        expected_error: None,
        qr_data,
        signing_key: None,
        encryption_key: None,
        expected_claim169: Some(serde_json::json!({
            "id": "ID-67890-FGHIJ",
            "version": "1.0",
            "language": "eng",
            "fullName": "Jane Marie Smith",
            "firstName": "Jane",
            "middleName": "Marie",
            "lastName": "Smith",
            "dateOfBirth": "19900515",
            "gender": 2,
            "address": "123 Main St\nApt 4\nNew York, NY 10001",
            "email": "jane.smith@example.com",
            "phone": "+1 555 123 4567",
            "nationality": "USA",
            "maritalStatus": 2,
            "guardian": "Guardian-ID-001",
            "photoFormat": 1,
            "secondaryFullName": "Jane Marie Smith (Hindi)",
            "secondaryLanguage": "hin",
            "locationCode": "US-NY-NYC",
            "legalStatus": "citizen",
            "countryOfIssuance": "USA"
        })),
        expected_cwt_meta: Some(meta),
    }
}

fn generate_with_face() -> TestVector {
    // Create a fake face biometric (JPEG image placeholder)
    let face_biometric = create_biometric_array(
        0,                                     // Image format
        1,                                     // JPEG sub-format
        &[0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10], // Fake JPEG header
    );

    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-FACE-001".to_string())),
        (4, Value::Text("Face Test Person".to_string())),
        (62, face_biometric), // Face biometric key
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://mosip.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(1800000000);

    let qr_data = encode_unsigned_qr(&meta, &claim_169);

    TestVector {
        name: "with-face".to_string(),
        description: "Face biometric included".to_string(),
        category: "valid".to_string(),
        expected_error: None,
        qr_data,
        signing_key: None,
        encryption_key: None,
        expected_claim169: Some(serde_json::json!({
            "id": "ID-FACE-001",
            "fullName": "Face Test Person",
            "face": [{
                "data": "//3+4AAQ",
                "format": 0,
                "subFormat": 1
            }]
        })),
        expected_cwt_meta: Some(meta),
    }
}

fn generate_claim169_example() -> TestVector {
    // This matches the example from claim_169.md section 3.2.1
    // Face biometric data from the example (WEBP format)
    // The hex string from the example needs to be decoded
    let face_data_hex = "52494646dc0100005745425056503820d0010000b00d009d012a400040003e913c9b4925a322a12a1ccae8b01209690013e295b2585d5ee72395f7fe4a35103d1894a549b58a4febe751ae9a3d00cb96f016fc35075f892786b3bcce1deffb2b3e55e3598b7d4913c80a237f1d9e51be7f271cc971d63fda0c2c3c34b27a574ec1bbd7752969c56c8c0000fefeffce44d1e6b7ad2535538b4cc7a3cf016f5b7d160c4e7202269bc041f0609efdf8e687702cdd6bd64e90b2931c9210f095f3c3bef00a954bfef4e70c76948b9eedf20e5be9e885edbcceada8f6fbdb9037490fa2eecaeaa62de8123028505f9f2eb2f781fdfc9b55ff127f12cb657cdc5927866e650426e3032500af838514711241395bfb130fda3c29d836527eeb82d92121b5a6f3b951d4ecc51ae1566c58266227b0f02ced0050fe35e0e42a33026a2c44c581fc65ddd135b6a7e5bc888ef852f6c477ccd817b850b90fa3565e11b61e7fe46f965abe210d097ef03eaaf028c4ff9dff5f55ad472464b4920a5958b8c98ef0e0029160f20a8f4d1a02ad3b5ad0c43c0b03dc549576cafb6c3d6c36f1014c57d94f6985f8a328dc7aef8df3507041dc440e99fe9acd90cd3ede4381d5b3d64064bce4bb8d05113fd901b158698312bdf8a21049288d6006a2c944dae7bc3e24000000";
    let face_data = hex::decode(face_data_hex.replace(" ", "").replace("\n", ""))
        .expect("Failed to decode face data hex");

    let face_biometric = create_biometric_array(
        0, // Image format
        4, // WEBP sub-format
        &face_data,
    );

    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("3918592438".to_string())),
        (2, Value::Text("1.0".to_string())),
        (3, Value::Text("eng".to_string())),
        (4, Value::Text("Janardhan BS".to_string())),
        (8, Value::Text("19840118".to_string())), // Converted from 1984-04-18 to YYYYMMDD
        (9, Value::Integer(1.into())),            // Male
        (
            10,
            Value::Text("New House, Near Metro Line, Bengaluru, KA".to_string()),
        ),
        (11, Value::Text("janardhan@example.com".to_string())),
        (12, Value::Text("+919876543210".to_string())),
        (13, Value::Text("IN".to_string())),
        (14, Value::Integer(2.into())), // Married
        (19, Value::Text("جاناردان بنغالور سرينيفاس".to_string())), // Secondary language full name
        (20, Value::Text("AR".to_string())), // Secondary language code (Arabic)
        (21, Value::Text("849VCWC8+R9".to_string())), // Location code
        (22, Value::Text("Refugee".to_string())), // Legal status
        (23, Value::Text("IN".to_string())), // Country of issuance
        (62, face_biometric),           // Face biometric
    ]);

    // Use the issuer and timestamps from the example in claim_169.md
    let meta = CwtMeta::new()
        .with_issuer("www.mosip.io")
        .with_issued_at(1756376445) // iat from example
        .with_expires_at(1787912445) // exp from example
        .with_not_before(1756376445); // nbf from example

    let qr_data = encode_unsigned_qr(&meta, &claim_169);

    TestVector {
        name: "claim169-example".to_string(),
        description: "Example from claim_169.md specification section 3.2.1".to_string(),
        category: "valid".to_string(),
        expected_error: None,
        qr_data,
        signing_key: None,
        encryption_key: None,
        expected_claim169: Some(serde_json::json!({
            "id": "3918592438",
            "version": "1.0",
            "language": "eng",
            "fullName": "Janardhan BS",
            "dateOfBirth": "19840118",
            "gender": 1,
            "address": "New House, Near Metro Line, Bengaluru, KA",
            "email": "janardhan@example.com",
            "phone": "+919876543210",
            "nationality": "IN",
            "maritalStatus": 2,
            "secondaryFullName": "جاناردان بنغالور سرينيفاس",
            "secondaryLanguage": "AR",
            "locationCode": "849VCWC8+R9",
            "legalStatus": "Refugee",
            "countryOfIssuance": "IN",
            "face": [{
                "format": 0,
                "subFormat": 4
            }]
        })),
        expected_cwt_meta: Some(meta),
    }
}

fn generate_with_fingerprints() -> TestVector {
    // Create fake fingerprint templates
    let right_thumb = create_biometric_array(
        1,                         // Template format
        1,                         // ISO 19794-2 sub-format
        &[0x46, 0x49, 0x52, 0x00], // Fake FIR header
    );

    let right_pointer = create_biometric_array(
        1, // Template format
        1, // ISO 19794-2 sub-format
        &[0x46, 0x49, 0x52, 0x01],
    );

    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-FINGER-001".to_string())),
        (4, Value::Text("Fingerprint Test Person".to_string())),
        (50, right_thumb),   // Right thumb
        (51, right_pointer), // Right pointer finger
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://mosip.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(1800000000);

    let qr_data = encode_unsigned_qr(&meta, &claim_169);

    TestVector {
        name: "with-fingerprints".to_string(),
        description: "Multiple fingerprint biometrics".to_string(),
        category: "valid".to_string(),
        expected_error: None,
        qr_data,
        signing_key: None,
        encryption_key: None,
        expected_claim169: Some(serde_json::json!({
            "id": "ID-FINGER-001",
            "fullName": "Fingerprint Test Person",
            "rightThumb": [{
                "data": "RklSAA==",
                "format": 1,
                "subFormat": 1
            }],
            "rightPointerFinger": [{
                "data": "RklSAQ==",
                "format": 1,
                "subFormat": 1
            }]
        })),
        expected_cwt_meta: Some(meta),
    }
}

fn generate_ed25519_signed() -> TestVector {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-SIGNED-001".to_string())),
        (4, Value::Text("Signed Test Person".to_string())),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://mosip.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(1800000000);

    // Generate deterministic key for reproducibility
    let seed: [u8; 32] = [
        0x9d, 0x61, 0xb1, 0x9d, 0xef, 0xfd, 0x5a, 0x60, 0xba, 0x84, 0x4a, 0xf4, 0x92, 0xec, 0x2c,
        0xc4, 0x44, 0x49, 0xc5, 0x69, 0x7b, 0x32, 0x69, 0x19, 0x70, 0x3b, 0xac, 0x03, 0x1c, 0xae,
        0x7f, 0x60,
    ];

    let signer = Ed25519Signer::from_bytes(&seed).unwrap();
    let public_key = signer.public_key_bytes();

    let qr_data = encode_signed_qr(&meta, &claim_169, &signer);

    TestVector {
        name: "ed25519-signed".to_string(),
        description: "COSE_Sign1 with Ed25519 signature".to_string(),
        category: "valid".to_string(),
        expected_error: None,
        qr_data,
        signing_key: Some(KeyMaterial {
            algorithm: "EdDSA".to_string(),
            public_key_hex: Some(hex::encode(public_key)),
            private_key_hex: Some(hex::encode(seed)),
            symmetric_key_hex: None,
        }),
        encryption_key: None,
        expected_claim169: Some(serde_json::json!({
            "id": "ID-SIGNED-001",
            "fullName": "Signed Test Person"
        })),
        expected_cwt_meta: Some(meta),
    }
}

fn generate_encrypted_aes256() -> TestVector {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-ENCRYPTED-001".to_string())),
        (4, Value::Text("Encrypted Test Person".to_string())),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://mosip.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(1800000000);

    // Deterministic key and nonce
    let key: [u8; 32] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
        0x1e, 0x1f,
    ];
    let nonce: [u8; 12] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
    ];

    let encryptor = AesGcmEncryptor::aes256(&key).unwrap();
    let qr_data = encode_encrypted_qr(&meta, &claim_169, &encryptor, &nonce);

    TestVector {
        name: "encrypted-aes256".to_string(),
        description: "COSE_Encrypt0 with AES-256-GCM".to_string(),
        category: "valid".to_string(),
        expected_error: None,
        qr_data,
        signing_key: None,
        encryption_key: Some(KeyMaterial {
            algorithm: "A256GCM".to_string(),
            public_key_hex: None,
            private_key_hex: None,
            symmetric_key_hex: Some(hex::encode(key)),
        }),
        expected_claim169: Some(serde_json::json!({
            "id": "ID-ENCRYPTED-001",
            "fullName": "Encrypted Test Person"
        })),
        expected_cwt_meta: Some(meta),
    }
}

fn generate_with_all_biometrics() -> TestVector {
    // Create biometrics for all types
    let right_thumb = create_biometric_array(1, 1, &[0x46, 0x49, 0x52, 0x00]);
    let right_pointer = create_biometric_array(1, 1, &[0x46, 0x49, 0x52, 0x01]);
    let right_middle = create_biometric_array(1, 1, &[0x46, 0x49, 0x52, 0x02]);
    let right_ring = create_biometric_array(1, 1, &[0x46, 0x49, 0x52, 0x03]);
    let right_little = create_biometric_array(1, 1, &[0x46, 0x49, 0x52, 0x04]);
    let left_thumb = create_biometric_array(1, 1, &[0x46, 0x49, 0x52, 0x05]);
    let left_pointer = create_biometric_array(1, 1, &[0x46, 0x49, 0x52, 0x06]);
    let left_middle = create_biometric_array(1, 1, &[0x46, 0x49, 0x52, 0x07]);
    let left_ring = create_biometric_array(1, 1, &[0x46, 0x49, 0x52, 0x08]);
    let left_little = create_biometric_array(1, 1, &[0x46, 0x49, 0x52, 0x09]);
    let right_iris = create_biometric_array(2, 0, &[0x49, 0x52, 0x49, 0x53, 0x00]);
    let left_iris = create_biometric_array(2, 0, &[0x49, 0x52, 0x49, 0x53, 0x01]);
    let face = create_biometric_array(0, 1, &[0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10]);

    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-ALL-BIO-001".to_string())),
        (4, Value::Text("All Biometrics Test Person".to_string())),
        (50, right_thumb),
        (51, right_pointer),
        (52, right_middle),
        (53, right_ring),
        (54, right_little),
        (55, left_thumb),
        (56, left_pointer),
        (57, left_middle),
        (58, left_ring),
        (59, left_little),
        (60, right_iris),
        (61, left_iris),
        (62, face),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://mosip.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(1800000000);

    let qr_data = encode_unsigned_qr(&meta, &claim_169);

    TestVector {
        name: "with-all-biometrics".to_string(),
        description: "All biometric types included".to_string(),
        category: "valid".to_string(),
        expected_error: None,
        qr_data,
        signing_key: None,
        encryption_key: None,
        expected_claim169: Some(serde_json::json!({
            "id": "ID-ALL-BIO-001",
            "fullName": "All Biometrics Test Person"
        })),
        expected_cwt_meta: Some(meta),
    }
}

fn generate_ecdsa_p256_signed() -> TestVector {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-ECDSA-001".to_string())),
        (4, Value::Text("ECDSA Test Person".to_string())),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://mosip.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(1800000000);

    // Generate deterministic P-256 key for reproducibility
    let signer = EcdsaP256Signer::generate();
    let public_key = signer.public_key_uncompressed();

    let qr_data = encode_ecdsa_signed_qr(&meta, &claim_169, &signer);

    TestVector {
        name: "ecdsa-p256-signed".to_string(),
        description: "COSE_Sign1 with ECDSA P-256 signature".to_string(),
        category: "valid".to_string(),
        expected_error: None,
        qr_data,
        signing_key: Some(KeyMaterial {
            algorithm: "ES256".to_string(),
            public_key_hex: Some(hex::encode(public_key)),
            private_key_hex: None, // Don't expose private key
            symmetric_key_hex: None,
        }),
        encryption_key: None,
        expected_claim169: Some(serde_json::json!({
            "id": "ID-ECDSA-001",
            "fullName": "ECDSA Test Person"
        })),
        expected_cwt_meta: Some(meta),
    }
}

fn generate_encrypted_signed() -> TestVector {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-ENC-SIGN-001".to_string())),
        (4, Value::Text("Encrypted Signed Test Person".to_string())),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://mosip.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(1800000000);

    // Generate deterministic keys
    let sign_seed: [u8; 32] = [
        0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xba, 0xdc, 0xfe, 0x10, 0x32, 0x54, 0x76,
        0x98, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee,
        0xff, 0x00,
    ];
    let enc_key: [u8; 32] = [
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d,
        0x2e, 0x2f,
    ];
    let nonce: [u8; 12] = [
        0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab,
    ];

    let signer = Ed25519Signer::from_bytes(&sign_seed).unwrap();
    let public_key = signer.public_key_bytes();
    let encryptor = AesGcmEncryptor::aes256(&enc_key).unwrap();

    let qr_data = encode_encrypted_signed_qr(&meta, &claim_169, &signer, &encryptor, &nonce);

    TestVector {
        name: "encrypted-signed".to_string(),
        description: "COSE_Encrypt0 containing signed COSE_Sign1".to_string(),
        category: "valid".to_string(),
        expected_error: None,
        qr_data,
        signing_key: Some(KeyMaterial {
            algorithm: "EdDSA".to_string(),
            public_key_hex: Some(hex::encode(public_key)),
            private_key_hex: Some(hex::encode(sign_seed)),
            symmetric_key_hex: None,
        }),
        encryption_key: Some(KeyMaterial {
            algorithm: "A256GCM".to_string(),
            public_key_hex: None,
            private_key_hex: None,
            symmetric_key_hex: Some(hex::encode(enc_key)),
        }),
        expected_claim169: Some(serde_json::json!({
            "id": "ID-ENC-SIGN-001",
            "fullName": "Encrypted Signed Test Person"
        })),
        expected_cwt_meta: Some(meta),
    }
}

// ============================================================================
// Invalid Vectors
// ============================================================================

fn generate_bad_base45() -> TestVector {
    TestVector {
        name: "bad-base45".to_string(),
        description: "Invalid Base45 encoding".to_string(),
        category: "invalid".to_string(),
        expected_error: Some("Base45Decode".to_string()),
        qr_data: "THIS!IS@NOT#VALID$BASE45%DATA".to_string(),
        signing_key: None,
        encryption_key: None,
        expected_claim169: None,
        expected_cwt_meta: None,
    }
}

fn generate_bad_zlib() -> TestVector {
    // Valid Base45 but decodes to garbage that isn't zlib
    let garbage = vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];
    let qr_data = base45::encode(&garbage);

    TestVector {
        name: "bad-zlib".to_string(),
        description: "Invalid zlib data after Base45 decode".to_string(),
        category: "invalid".to_string(),
        expected_error: Some("Decompress".to_string()),
        qr_data,
        signing_key: None,
        encryption_key: None,
        expected_claim169: None,
        expected_cwt_meta: None,
    }
}

fn generate_not_cose() -> TestVector {
    // Valid CBOR but not a COSE structure
    let cbor_array = Value::Array(vec![
        Value::Text("hello".to_string()),
        Value::Integer(42.into()),
    ]);

    let mut cbor_bytes = Vec::new();
    ciborium::into_writer(&cbor_array, &mut cbor_bytes).unwrap();

    let compressed = decompress::compress(&cbor_bytes);
    let qr_data = base45::encode(&compressed);

    TestVector {
        name: "not-cose".to_string(),
        description: "Valid CBOR but not a COSE structure".to_string(),
        category: "invalid".to_string(),
        expected_error: Some("CoseParse".to_string()),
        qr_data,
        signing_key: None,
        encryption_key: None,
        expected_claim169: None,
        expected_cwt_meta: None,
    }
}

fn generate_missing_169() -> TestVector {
    // Valid CWT but without claim 169
    let cwt_map = Value::Map(vec![
        (
            Value::Integer(1.into()),
            Value::Text("https://example.org".to_string()),
        ), // iss
        (Value::Integer(4.into()), Value::Integer(1800000000.into())), // exp
        (Value::Integer(6.into()), Value::Integer(1700000000.into())), // iat
                                                                       // No claim 169!
    ]);

    let mut cwt_bytes = Vec::new();
    ciborium::into_writer(&cwt_map, &mut cwt_bytes).unwrap();

    // Wrap in unsigned COSE_Sign1
    let sign1 = CoseSign1Builder::new()
        .protected(
            HeaderBuilder::new()
                .algorithm(iana::Algorithm::EdDSA)
                .build(),
        )
        .payload(cwt_bytes)
        .build();

    let cose_bytes = sign1.to_tagged_vec().unwrap();
    let compressed = decompress::compress(&cose_bytes);
    let qr_data = base45::encode(&compressed);

    TestVector {
        name: "missing-169".to_string(),
        description: "Valid CWT but without claim 169".to_string(),
        category: "invalid".to_string(),
        expected_error: Some("Claim169NotFound".to_string()),
        qr_data,
        signing_key: None,
        encryption_key: None,
        expected_claim169: None,
        expected_cwt_meta: None,
    }
}

// ============================================================================
// Edge Case Vectors
// ============================================================================

fn generate_expired() -> TestVector {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-EXPIRED-001".to_string())),
        (4, Value::Text("Expired Test Person".to_string())),
    ]);

    // Expired in 2020
    let meta = CwtMeta::new()
        .with_issuer("https://mosip.example.org")
        .with_issued_at(1577836800) // 2020-01-01
        .with_expires_at(1609459200); // 2021-01-01

    let qr_data = encode_unsigned_qr(&meta, &claim_169);

    TestVector {
        name: "expired".to_string(),
        description: "Token with exp in past".to_string(),
        category: "edge".to_string(),
        expected_error: None, // Should still decode, just flag as expired
        qr_data,
        signing_key: None,
        encryption_key: None,
        expected_claim169: Some(serde_json::json!({
            "id": "ID-EXPIRED-001",
            "fullName": "Expired Test Person"
        })),
        expected_cwt_meta: Some(meta),
    }
}

fn generate_not_yet_valid() -> TestVector {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-FUTURE-001".to_string())),
        (4, Value::Text("Future Test Person".to_string())),
    ]);

    // Not valid until 2050
    let meta = CwtMeta::new()
        .with_issuer("https://mosip.example.org")
        .with_issued_at(1700000000)
        .with_not_before(2524608000) // 2050-01-01
        .with_expires_at(2556144000); // 2051-01-01

    let qr_data = encode_unsigned_qr(&meta, &claim_169);

    TestVector {
        name: "not-yet-valid".to_string(),
        description: "Token with nbf in future".to_string(),
        category: "edge".to_string(),
        expected_error: None, // Should still decode, just flag as not yet valid
        qr_data,
        signing_key: None,
        encryption_key: None,
        expected_claim169: Some(serde_json::json!({
            "id": "ID-FUTURE-001",
            "fullName": "Future Test Person"
        })),
        expected_cwt_meta: Some(meta),
    }
}

fn generate_unknown_fields() -> TestVector {
    let claim_169 = create_claim169_map(vec![
        (1, Value::Text("ID-UNKNOWN-001".to_string())),
        (4, Value::Text("Unknown Fields Person".to_string())),
        // Unknown keys in reserved range
        (30, Value::Text("reserved field 30".to_string())),
        (45, Value::Integer(12345.into())),
        // Unknown keys in biometric range (not assigned)
        (70, Value::Bytes(vec![0x01, 0x02, 0x03])),
        // Future extension key
        (99, Value::Bool(true)),
    ]);

    let meta = CwtMeta::new()
        .with_issuer("https://mosip.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(1800000000);

    let qr_data = encode_unsigned_qr(&meta, &claim_169);

    TestVector {
        name: "unknown-fields".to_string(),
        description: "Contains unknown CBOR keys (preserved for forward compatibility)".to_string(),
        category: "edge".to_string(),
        expected_error: None,
        qr_data,
        signing_key: None,
        encryption_key: None,
        expected_claim169: Some(serde_json::json!({
            "id": "ID-UNKNOWN-001",
            "fullName": "Unknown Fields Person"
        })),
        expected_cwt_meta: Some(meta),
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

fn create_claim169_map(fields: Vec<(i64, Value)>) -> Value {
    Value::Map(
        fields
            .into_iter()
            .map(|(k, v)| (Value::Integer(k.into()), v))
            .collect(),
    )
}

fn create_biometric_array(format: i64, sub_format: i64, data: &[u8]) -> Value {
    // Biometric is an array of [format, sub_format, data] or a map
    Value::Array(vec![Value::Map(vec![
        (Value::Integer(0.into()), Value::Bytes(data.to_vec())), // data
        (Value::Integer(1.into()), Value::Integer(format.into())), // format
        (Value::Integer(2.into()), Value::Integer(sub_format.into())), // sub_format
    ])])
}

fn encode_unsigned_qr(meta: &CwtMeta, claim_169: &Value) -> String {
    // Encode CWT
    let cwt_bytes = cwt::encode(meta, claim_169);

    // Wrap in unsigned COSE_Sign1 (for testing without signature)
    let sign1 = CoseSign1Builder::new()
        .protected(
            HeaderBuilder::new()
                .algorithm(iana::Algorithm::EdDSA)
                .build(),
        )
        .payload(cwt_bytes)
        .build();

    let cose_bytes = sign1.to_tagged_vec().unwrap();
    let compressed = decompress::compress(&cose_bytes);
    base45::encode(&compressed)
}

fn encode_signed_qr<S: Signer>(meta: &CwtMeta, claim_169: &Value, signer: &S) -> String {
    // Encode CWT
    let cwt_bytes = cwt::encode(meta, claim_169);

    // Build COSE_Sign1 structure
    let protected = HeaderBuilder::new()
        .algorithm(iana::Algorithm::EdDSA)
        .build();

    let mut sign1 = CoseSign1Builder::new()
        .protected(protected.clone())
        .payload(cwt_bytes.clone())
        .build();

    // Sign
    let tbs_data = sign1.tbs_data(&[]);
    let signature = signer
        .sign(iana::Algorithm::EdDSA, None, &tbs_data)
        .expect("signing failed");
    sign1.signature = signature;

    let cose_bytes = sign1.to_tagged_vec().unwrap();
    let compressed = decompress::compress(&cose_bytes);
    base45::encode(&compressed)
}

fn encode_encrypted_qr<E: Encryptor>(
    meta: &CwtMeta,
    claim_169: &Value,
    encryptor: &E,
    nonce: &[u8],
) -> String {
    // Encode CWT
    let cwt_bytes = cwt::encode(meta, claim_169);

    // Build protected header
    let protected = HeaderBuilder::new()
        .algorithm(iana::Algorithm::A256GCM)
        .build();

    // Build AAD (Enc_structure)
    let protected_bytes = protected.clone().to_vec().unwrap();
    let aad = build_encrypt0_aad(&protected_bytes);

    // Encrypt
    let ciphertext = encryptor
        .encrypt(iana::Algorithm::A256GCM, None, nonce, &aad, &cwt_bytes)
        .expect("encryption failed");

    // Build COSE_Encrypt0
    let encrypt0 = CoseEncrypt0Builder::new()
        .protected(protected)
        .unprotected(HeaderBuilder::new().iv(nonce.to_vec()).build())
        .ciphertext(ciphertext)
        .build();

    let cose_bytes = encrypt0.to_tagged_vec().unwrap();
    let compressed = decompress::compress(&cose_bytes);
    base45::encode(&compressed)
}

fn build_encrypt0_aad(protected_bytes: &[u8]) -> Vec<u8> {
    let enc_structure = Value::Array(vec![
        Value::Text("Encrypt0".to_string()),
        Value::Bytes(protected_bytes.to_vec()),
        Value::Bytes(vec![]), // external_aad is empty
    ]);

    let mut aad = Vec::new();
    ciborium::into_writer(&enc_structure, &mut aad).expect("CBOR encoding should not fail");
    aad
}

fn encode_ecdsa_signed_qr(meta: &CwtMeta, claim_169: &Value, signer: &EcdsaP256Signer) -> String {
    // Encode CWT
    let cwt_bytes = cwt::encode(meta, claim_169);

    // Build COSE_Sign1 structure with ES256
    let protected = HeaderBuilder::new()
        .algorithm(iana::Algorithm::ES256)
        .build();

    let mut sign1 = CoseSign1Builder::new()
        .protected(protected.clone())
        .payload(cwt_bytes.clone())
        .build();

    // Sign
    let tbs_data = sign1.tbs_data(&[]);
    let signature = signer
        .sign(iana::Algorithm::ES256, None, &tbs_data)
        .expect("signing failed");
    sign1.signature = signature;

    let cose_bytes = sign1.to_tagged_vec().unwrap();
    let compressed = decompress::compress(&cose_bytes);
    base45::encode(&compressed)
}

fn encode_encrypted_signed_qr<E: Encryptor>(
    meta: &CwtMeta,
    claim_169: &Value,
    signer: &Ed25519Signer,
    encryptor: &E,
    nonce: &[u8],
) -> String {
    // First create signed COSE_Sign1
    let cwt_bytes = cwt::encode(meta, claim_169);

    let sign_protected = HeaderBuilder::new()
        .algorithm(iana::Algorithm::EdDSA)
        .build();

    let mut sign1 = CoseSign1Builder::new()
        .protected(sign_protected.clone())
        .payload(cwt_bytes.clone())
        .build();

    let tbs_data = sign1.tbs_data(&[]);
    let signature = signer
        .sign(iana::Algorithm::EdDSA, None, &tbs_data)
        .expect("signing failed");
    sign1.signature = signature;

    let signed_bytes = sign1.to_tagged_vec().unwrap();

    // Now encrypt the signed payload
    let enc_protected = HeaderBuilder::new()
        .algorithm(iana::Algorithm::A256GCM)
        .build();

    let protected_bytes = enc_protected.clone().to_vec().unwrap();
    let aad = build_encrypt0_aad(&protected_bytes);

    let ciphertext = encryptor
        .encrypt(iana::Algorithm::A256GCM, None, nonce, &aad, &signed_bytes)
        .expect("encryption failed");

    let encrypt0 = CoseEncrypt0Builder::new()
        .protected(enc_protected)
        .unprotected(HeaderBuilder::new().iv(nonce.to_vec()).build())
        .ciphertext(ciphertext)
        .build();

    let cose_bytes = encrypt0.to_tagged_vec().unwrap();
    let compressed = decompress::compress(&cose_bytes);
    base45::encode(&compressed)
}
