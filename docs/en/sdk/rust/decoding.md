# Decoding

The `Decoder` provides a fluent builder API for reading and verifying Claim 169 QR codes.

## Decoder Pipeline

The decoding process reverses the encoding pipeline:

```text
Base45 -> zlib -> COSE_Encrypt0 (if encrypted) -> COSE_Sign1 -> CWT -> Claim169
```

## Basic Usage

```rust
use claim169_core::Decoder;

let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .decode()?;

println!("Name: {:?}", result.claim169.full_name);
println!("Issuer: {:?}", result.cwt_meta.issuer);
```

## DecodeResult Structure

The `decode()` method returns a `DecodeResult`:

```rust
pub struct DecodeResult {
    /// The extracted Claim 169 identity data
    pub claim169: Claim169,

    /// CWT metadata (issuer, expiration, etc.)
    pub cwt_meta: CwtMeta,

    /// Signature verification status
    pub verification_status: VerificationStatus,

    /// Compression format detected during decoding
    pub detected_compression: DetectedCompression,

    /// Non-fatal warnings
    pub warnings: Vec<Warning>,
}
```

### Accessing Identity Data

```rust
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .decode()?;

// Demographics
if let Some(id) = &result.claim169.id {
    println!("ID: {}", id);
}
if let Some(name) = &result.claim169.full_name {
    println!("Name: {}", name);
}
if let Some(dob) = &result.claim169.date_of_birth {
    println!("DOB: {}", dob);
}
if let Some(gender) = &result.claim169.gender {
    println!("Gender: {:?}", gender);
}

// Contact
if let Some(email) = &result.claim169.email {
    println!("Email: {}", email);
}
if let Some(phone) = &result.claim169.phone {
    println!("Phone: {}", phone);
}

// Check for biometrics
if result.claim169.has_biometrics() {
    println!("Contains {} biometric entries", result.claim169.biometric_count());
}
```

### Accessing CWT Metadata

```rust
if let Some(issuer) = &result.cwt_meta.issuer {
    println!("Issued by: {}", issuer);
}
if let Some(subject) = &result.cwt_meta.subject {
    println!("Subject: {}", subject);
}
if let Some(exp) = result.cwt_meta.expires_at {
    println!("Expires at: {}", exp);
}
if let Some(iat) = result.cwt_meta.issued_at {
    println!("Issued at: {}", iat);
}

// Check validity at a specific time
let current_time = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_secs() as i64;

if result.cwt_meta.is_time_valid(current_time) {
    println!("Credential is currently valid");
}
if result.cwt_meta.is_expired(current_time) {
    println!("Credential has expired");
}
```

### Checking Verification Status

```rust
use claim169_core::VerificationStatus;

match result.verification_status {
    VerificationStatus::Verified => {
        println!("Signature verified successfully");
    }
    VerificationStatus::Skipped => {
        println!("Verification was skipped (allow_unverified was called)");
    }
    VerificationStatus::Failed => {
        println!("Signature verification failed");
    }
}
```

### Handling Warnings

```rust
use claim169_core::WarningCode;

for warning in &result.warnings {
    match warning.code {
        WarningCode::ExpiringSoon => {
            println!("Credential expiring soon: {}", warning.message);
        }
        WarningCode::UnknownFields => {
            println!("Unknown fields found: {}", warning.message);
        }
        WarningCode::TimestampValidationSkipped => {
            println!("Timestamps not validated: {}", warning.message);
        }
        WarningCode::BiometricsSkipped => {
            println!("Biometrics skipped: {}", warning.message);
        }
        WarningCode::NonStandardCompression => {
            println!("Non-standard compression: {}", warning.message);
        }
    }
}
```

## Signature Verification

### Ed25519 (Recommended)

```rust
// From raw bytes (32 bytes)
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .decode()?;

// From PEM format
let pem = r#"-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEA...
-----END PUBLIC KEY-----"#;

let result = Decoder::new(qr_content)
    .verify_with_ed25519_pem(pem)?
    .decode()?;
```

### ECDSA P-256

```rust
// From SEC1 bytes (33 or 65 bytes)
let result = Decoder::new(qr_content)
    .verify_with_ecdsa_p256(&public_key)?
    .decode()?;

// From PEM format
let result = Decoder::new(qr_content)
    .verify_with_ecdsa_p256_pem(pem)?
    .decode()?;
```

### Custom Verifier

For HSM or KMS integration:

```rust
use claim169_core::{Decoder, SignatureVerifier};

struct MyHsmVerifier { /* ... */ }
impl SignatureVerifier for MyHsmVerifier { /* ... */ }

let result = Decoder::new(qr_content)
    .verify_with(my_hsm_verifier)
    .decode()?;
```

See [Custom Crypto](./custom-crypto.md) for details.

## Decryption

For encrypted credentials, provide the decryption key:

```rust
// AES-256-GCM
let result = Decoder::new(qr_content)
    .decrypt_with_aes256(&aes_key)?  // 32-byte key
    .verify_with_ed25519(&public_key)?
    .decode()?;

// AES-128-GCM
let result = Decoder::new(qr_content)
    .decrypt_with_aes128(&aes_key)?  // 16-byte key
    .verify_with_ed25519(&public_key)?
    .decode()?;
```

### Custom Decryptor

```rust
use claim169_core::{Decoder, Decryptor};

struct MyHsmDecryptor { /* ... */ }
impl Decryptor for MyHsmDecryptor { /* ... */ }

let result = Decoder::new(qr_content)
    .decrypt_with(my_hsm_decryptor)
    .verify_with_ed25519(&public_key)?
    .decode()?;
```

## Decoder Options

### Skip Verification (Testing Only)

```rust
let result = Decoder::new(qr_content)
    .allow_unverified()  // Required to decode without verification
    .decode()?;
```

**Warning**: Never skip verification in production.

### Skip Biometrics

For faster decoding when biometrics are not needed:

```rust
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .skip_biometrics()  // Don't parse biometric data
    .decode()?;
```

### Disable Timestamp Validation

For offline scenarios or testing:

```rust
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .without_timestamp_validation()  // Don't check exp/nbf
    .decode()?;
```

### Clock Skew Tolerance

Allow for clock differences between systems:

```rust
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .clock_skew_tolerance(300)  // 5 minutes tolerance
    .decode()?;
```

### Decompression Limit

Adjust the maximum decompressed size (default: 64KB):

```rust
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .max_decompressed_bytes(131072)  // 128KB
    .decode()?;
```

This protects against zip bomb attacks.

### Strict Compression

By default, the decoder auto-detects and accepts any compression format (zlib, brotli, or none). To enforce spec compliance and reject non-zlib data:

```rust
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .strict_compression()  // Reject non-zlib compression
    .decode()?;
```

This is useful for validators that must enforce spec conformance. Without `strict_compression()`, non-zlib credentials decode normally but produce a `NonStandardCompression` warning.

### Checking Detected Compression

After decoding, you can inspect which compression format was used:

```rust
use claim169_core::DetectedCompression;

let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .decode()?;

match result.detected_compression {
    DetectedCompression::Zlib => println!("Standard zlib compression"),
    DetectedCompression::None => println!("No compression"),
    #[cfg(feature = "compression-brotli")]
    DetectedCompression::Brotli => println!("Brotli compression"),
}
```

## Operation Order

The decoder always processes in this order, regardless of method call order:

1. Base45 decode
2. Decompress (auto-detects zlib, brotli, or raw)
3. Decrypt with COSE_Encrypt0 (if decryptor provided)
4. Verify signature with COSE_Sign1 (if verifier provided)
5. Parse CWT
6. Validate timestamps (unless disabled)
7. Transform Claim 169

## Error Handling

```rust
use claim169_core::{Decoder, Claim169Error};

match Decoder::new(qr_content).verify_with_ed25519(&key)?.decode() {
    Ok(result) => {
        println!("Decoded: {:?}", result.claim169.full_name);
    }

    // Encoding/format errors
    Err(Claim169Error::Base45Decode(msg)) => {
        eprintln!("Invalid QR format: {}", msg);
    }
    Err(Claim169Error::Decompress(msg)) => {
        eprintln!("Decompression failed: {}", msg);
    }
    Err(Claim169Error::DecompressLimitExceeded { max_bytes }) => {
        eprintln!("Data too large (max {} bytes)", max_bytes);
    }

    // Structure errors
    Err(Claim169Error::CoseParse(msg)) => {
        eprintln!("Invalid COSE structure: {}", msg);
    }
    Err(Claim169Error::CwtParse(msg)) => {
        eprintln!("Invalid CWT: {}", msg);
    }
    Err(Claim169Error::Claim169NotFound) => {
        eprintln!("Claim 169 not found in payload");
    }

    // Security errors
    Err(Claim169Error::SignatureInvalid(msg)) => {
        eprintln!("Signature verification failed: {}", msg);
    }
    Err(Claim169Error::DecryptionFailed(msg)) => {
        eprintln!("Decryption failed: {}", msg);
    }

    // Timestamp errors
    Err(Claim169Error::Expired(ts)) => {
        eprintln!("Credential expired at {}", ts);
    }
    Err(Claim169Error::NotYetValid(ts)) => {
        eprintln!("Credential not valid until {}", ts);
    }

    // Configuration errors
    Err(Claim169Error::DecodingConfig(msg)) => {
        eprintln!("Configuration error: {}", msg);
    }

    // Other errors
    Err(e) => eprintln!("Error: {}", e),
}
```

## Accessing Unknown Fields

For forward compatibility, unknown CBOR keys are preserved:

```rust
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .decode()?;

// Check for unknown fields
if !result.claim169.unknown_fields.is_empty() {
    println!("Unknown fields found:");
    for (key, value) in &result.claim169.unknown_fields {
        println!("  Key {}: {:?}", key, value);
    }
}
```

## Complete Example

```rust
use claim169_core::{Decoder, VerificationStatus, WarningCode, Claim169Error};

fn verify_credential(
    qr_content: &str,
    public_key: &[u8],
    aes_key: Option<&[u8]>,
) -> claim169_core::Result<()> {
    // Build decoder
    let mut decoder = Decoder::new(qr_content);

    // Add decryption if key provided
    if let Some(key) = aes_key {
        decoder = decoder.decrypt_with_aes256(key)?;
    }

    // Decode with verification
    let result = decoder
        .verify_with_ed25519(public_key)?
        .clock_skew_tolerance(60)  // 1 minute tolerance
        .decode()?;

    // Check verification
    if result.verification_status != VerificationStatus::Verified {
        return Err(Claim169Error::SignatureInvalid(
            "Signature not verified".to_string()
        ));
    }

    // Report warnings
    for warning in &result.warnings {
        eprintln!("Warning [{}]: {}",
            match warning.code {
                WarningCode::ExpiringSoon => "EXPIRING",
                WarningCode::UnknownFields => "UNKNOWN_FIELDS",
                WarningCode::TimestampValidationSkipped => "NO_TIMESTAMP_CHECK",
                WarningCode::BiometricsSkipped => "NO_BIOMETRICS",
                WarningCode::NonStandardCompression => "NON_STANDARD_COMPRESSION",
            },
            warning.message
        );
    }

    // Display credential
    println!("=== Verified Credential ===");
    println!("ID: {:?}", result.claim169.id);
    println!("Name: {:?}", result.claim169.full_name);
    println!("DOB: {:?}", result.claim169.date_of_birth);
    println!("Issuer: {:?}", result.cwt_meta.issuer);
    println!("Expires: {:?}", result.cwt_meta.expires_at);

    Ok(())
}
```
