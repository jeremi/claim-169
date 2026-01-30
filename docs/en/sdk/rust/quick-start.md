# Quick Start

This guide shows you how to encode and decode Claim 169 QR codes in just a few minutes.

## Basic Encoding

Create a signed QR code from identity data:

```rust
use claim169_core::{Encoder, Claim169, CwtMeta, Gender};

fn create_credential(private_key: &[u8]) -> claim169_core::Result<String> {
    // Build identity data
    let claim169 = Claim169::new()
        .with_id("ID-12345-ABCDE")
        .with_full_name("Jane Marie Smith")
        .with_date_of_birth("19900515")
        .with_gender(Gender::Female)
        .with_email("jane.smith@example.com");

    // Set CWT metadata
    let cwt_meta = CwtMeta::new()
        .with_issuer("https://issuer.example.com")
        .with_subject("jane.smith")
        .with_expires_at(1800000000)  // Unix timestamp
        .with_issued_at(1700000000);

    // Encode with Ed25519 signature
    let qr_data = Encoder::new(claim169, cwt_meta)
        .sign_with_ed25519(private_key)?
        .encode()?;

    Ok(qr_data)
}
```

## Basic Decoding

Read and verify a QR code:

!!! warning "Do not trim Base45"
    The Base45 alphabet includes a literal space character (`" "`). Preserve the scanned QR text exactly as-is (no `.trim()` or whitespace normalization), or you can corrupt valid credentials.

```rust
use claim169_core::{Decoder, VerificationStatus};

fn verify_credential(qr_content: &str, public_key: &[u8]) -> claim169_core::Result<()> {
    let result = Decoder::new(qr_content)
        .verify_with_ed25519(public_key)?
        .decode()?;

    // Check verification status
    match result.verification_status {
        VerificationStatus::Verified => println!("Signature verified"),
        VerificationStatus::Skipped => println!("Verification skipped"),
        VerificationStatus::Failed => println!("Verification failed"),
    }

    // Access identity data
    println!("ID: {:?}", result.claim169.id);
    println!("Name: {:?}", result.claim169.full_name);
    println!("DOB: {:?}", result.claim169.date_of_birth);
    println!("Email: {:?}", result.claim169.email);

    // Access CWT metadata
    println!("Issuer: {:?}", result.cwt_meta.issuer);
    println!("Expires: {:?}", result.cwt_meta.expires_at);

    // Check for warnings
    for warning in &result.warnings {
        println!("Warning: {}", warning.message);
    }

    Ok(())
}
```

## Complete Roundtrip Example

A full example with key generation:

```rust
use claim169_core::{
    Encoder, Decoder, Claim169, CwtMeta, Gender, MaritalStatus,
    Ed25519Signer, VerificationStatus,
};

fn main() -> claim169_core::Result<()> {
    // Generate Ed25519 key pair
    let signer = Ed25519Signer::generate();
    let public_key = signer.public_key_bytes();

    // Create identity data
    let claim169 = Claim169::new()
        .with_id("CITIZEN-2024-001")
        .with_full_name("Maria Garcia Lopez")
        .with_first_name("Maria")
        .with_last_name("Garcia Lopez")
        .with_date_of_birth("19850320")
        .with_gender(Gender::Female)
        .with_nationality("ES")
        .with_marital_status(MaritalStatus::Married)
        .with_address("Calle Principal 123\nMadrid, 28001\nSpain")
        .with_email("maria.garcia@example.com")
        .with_phone("+34 612 345 678");

    // Create CWT metadata
    let cwt_meta = CwtMeta::new()
        .with_issuer("https://id.government.es")
        .with_subject("maria.garcia")
        .with_expires_at(1893456000)  // 2030-01-01
        .with_issued_at(1704067200);  // 2024-01-01

    // Encode to QR string
    let qr_data = Encoder::new(claim169.clone(), cwt_meta.clone())
        .sign_with(signer, coset::iana::Algorithm::EdDSA)
        .encode()?;

    println!("QR Data ({} chars): {}...", qr_data.len(), &qr_data[..50]);

    // Decode and verify
    let result = Decoder::new(&qr_data)
        .verify_with_ed25519(&public_key)?
        .decode()?;

    // Verify the roundtrip
    assert_eq!(result.verification_status, VerificationStatus::Verified);
    assert_eq!(result.claim169.id, claim169.id);
    assert_eq!(result.claim169.full_name, claim169.full_name);
    assert_eq!(result.cwt_meta.issuer, cwt_meta.issuer);

    println!("Roundtrip successful!");
    println!("Verified: {:?}", result.verification_status);
    println!("Name: {:?}", result.claim169.full_name);
    println!("Issuer: {:?}", result.cwt_meta.issuer);

    Ok(())
}
```

## Testing Without Signatures

For development and testing, you can skip signatures:

```rust
use claim169_core::{Encoder, Decoder, Claim169, CwtMeta};

fn test_without_signatures() -> claim169_core::Result<()> {
    let claim169 = Claim169::minimal("test-123", "Test User");
    let cwt_meta = CwtMeta::new().with_issuer("test");

    // Encode without signature (testing only)
    let qr_data = Encoder::new(claim169, cwt_meta)
        .allow_unsigned()
        .encode()?;

    // Decode without verification (testing only)
    let result = Decoder::new(&qr_data)
        .allow_unverified()
        .decode()?;

    println!("Decoded: {:?}", result.claim169.full_name);
    Ok(())
}
```

**Warning**: Never use `allow_unsigned()` or `allow_unverified()` in production. Credentials decoded with verification skipped (`verification_status = Skipped`) cannot be trusted.

## Next Steps

- [Encoding](./encoding.md) - Learn all encoding options
- [Decoding](./decoding.md) - Learn all decoding options
- [Encryption](./encryption.md) - Add AES-GCM encryption
- [Custom Crypto](./custom-crypto.md) - Integrate with HSM/KMS
