# Encoding

The `Encoder` provides a fluent builder API for creating Claim 169 QR codes from identity data.

## Encoder Pipeline

The encoding process transforms data through this pipeline:

```text
Claim169 -> CBOR -> CWT -> COSE_Sign1 -> [COSE_Encrypt0] -> zlib -> Base45
```

## Basic Usage

```rust
use claim169_core::{Encoder, Claim169, CwtMeta};

let claim169 = Claim169::minimal("ID-001", "Jane Doe");
let cwt_meta = CwtMeta::new().with_issuer("https://issuer.example.com");

let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&private_key)?
    .encode()?;
```

## Building Identity Data

The `Claim169` struct uses a builder pattern for all fields:

### Demographics

```rust
use claim169_core::{Claim169, Gender, MaritalStatus, PhotoFormat};

let claim = Claim169::new()
    // Basic identity
    .with_id("ID-12345-ABCDE")
    .with_version("1.0")
    .with_language("eng")

    // Name
    .with_full_name("Jane Marie Smith")
    .with_first_name("Jane")
    .with_middle_name("Marie")
    .with_last_name("Smith")

    // Personal details
    .with_date_of_birth("19900515")  // YYYYMMDD format
    .with_gender(Gender::Female)
    .with_marital_status(MaritalStatus::Married)
    .with_nationality("USA")

    // Contact
    .with_address("123 Main St\nNew York, NY 10001")
    .with_email("jane.smith@example.com")
    .with_phone("+1 555 123 4567")  // E.123 format

    // Additional
    .with_guardian("John Smith Sr.")
    .with_location_code("US-NY-NYC")
    .with_legal_status("citizen")
    .with_country_of_issuance("USA");
```

### Photo

```rust
use claim169_core::{Claim169, PhotoFormat};

// Read photo file
let photo_bytes = std::fs::read("photo.jpg")?;

let claim = Claim169::new()
    .with_id("ID-001")
    .with_full_name("Jane Doe")
    .with_photo(photo_bytes)
    .with_photo_format(PhotoFormat::Jpeg);
```

Supported photo formats:
- `PhotoFormat::Jpeg` (1)
- `PhotoFormat::Jpeg2000` (2)
- `PhotoFormat::Avif` (3)
- `PhotoFormat::Webp` (4)

### Secondary Language

```rust
let claim = Claim169::new()
    .with_full_name("Maria Garcia")
    .with_language("spa")
    .with_secondary_full_name("Maria Garcia")  // In secondary language
    .with_secondary_language("eng");
```

### Best Quality Fingers

```rust
// Specify which fingers have the best quality biometrics
// Values 0-10 correspond to finger positions
let claim = Claim169::new()
    .with_best_quality_fingers(vec![1, 6, 2, 7]);  // Right index, left index, etc.
```

### Minimal Constructor

For quick testing or simple credentials:

```rust
let claim = Claim169::minimal("ID-001", "Jane Doe");
// Equivalent to:
// Claim169::new().with_id("ID-001").with_full_name("Jane Doe")
```

## CWT Metadata

```rust
use claim169_core::CwtMeta;

let cwt_meta = CwtMeta::new()
    .with_issuer("https://issuer.example.com")      // iss - credential issuer
    .with_subject("user-id-12345")                   // sub - credential subject
    .with_expires_at(1893456000)                     // exp - expiration timestamp
    .with_not_before(1704067200)                     // nbf - not valid before
    .with_issued_at(1704067200);                     // iat - issued at timestamp
```

## Signing

### Ed25519 (Recommended)

```rust
// From raw bytes
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&private_key)?  // 32-byte key
    .encode()?;
```

### ECDSA P-256

```rust
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ecdsa_p256(&private_key)?  // 32-byte scalar
    .encode()?;
```

### Custom Signer

For HSM or KMS integration:

```rust
use claim169_core::{Encoder, Signer};
use coset::iana;

struct MyHsmSigner { /* ... */ }
impl Signer for MyHsmSigner { /* ... */ }

let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with(my_hsm_signer, iana::Algorithm::EdDSA)
    .encode()?;
```

See [Custom Crypto](./custom-crypto.md) for details on implementing the `Signer` trait.

## Encryption

Add an encryption layer on top of the signature:

```rust
// AES-256-GCM (recommended)
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&sign_key)?
    .encrypt_with_aes256(&aes_key)?  // 32-byte key
    .encode()?;

// AES-128-GCM
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&sign_key)?
    .encrypt_with_aes128(&aes_key)?  // 16-byte key
    .encode()?;
```

Nonces are generated randomly by default. For deterministic output (testing only):

```rust
let nonce: [u8; 12] = [/* 12 random bytes */];

let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&sign_key)?
    .encrypt_with_aes256_nonce(&aes_key, &nonce)?
    .encode()?;
```

**Warning**: Reusing nonces with the same key is a critical security vulnerability.

See [Encryption](./encryption.md) for more details.

## Compression

By default, the encoder uses zlib (spec-compliant). You can choose a different compression mode:

```rust
use claim169_core::{Encoder, Compression};

// No compression (for tiny payloads where zlib adds overhead)
let result = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&private_key)?
    .compression(Compression::None)
    .encode()?;

// Adaptive: uses zlib if it reduces size, otherwise stores raw
let result = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&private_key)?
    .compression(Compression::Adaptive)
    .encode()?;
```

With the `compression-brotli` feature enabled:

```rust
// Brotli at quality 6 (non-standard, ~15% smaller than zlib)
let result = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&private_key)?
    .compression(Compression::Brotli(6))
    .encode()?;
```

!!! warning "Interoperability"
    Non-standard compression modes produce credentials that only this library can decode. Use them only in closed ecosystems.

## Excluding Biometrics

To reduce QR code size, exclude biometric data:

```rust
let qr_data = Encoder::new(claim169, cwt_meta)
    .skip_biometrics()  // Exclude fingerprints, iris, face, etc.
    .sign_with_ed25519(&private_key)?
    .encode()?;
```

Or create a copy without biometrics:

```rust
let claim_without_bio = claim169.without_biometrics();
```

## Unsigned Encoding (Testing Only)

```rust
let qr_data = Encoder::new(claim169, cwt_meta)
    .allow_unsigned()  // Required to encode without signature
    .encode()?;
```

**Warning**: Never use unsigned credentials in production.

## Operation Order

The encoder always performs operations in this order, regardless of method call order:

1. CBOR encode Claim169
2. Wrap in CWT with metadata
3. Sign with COSE_Sign1 (if signer provided)
4. Encrypt with COSE_Encrypt0 (if encryptor provided)
5. Compress with zlib
6. Encode with Base45

This means `sign_with()` and `encrypt_with()` can be called in any order:

```rust
// These produce identical output:
Encoder::new(claim, meta)
    .sign_with_ed25519(&key)?
    .encrypt_with_aes256(&aes_key)?
    .encode()?;

Encoder::new(claim, meta)
    .encrypt_with_aes256(&aes_key)?
    .sign_with_ed25519(&key)?
    .encode()?;
```

## Encode Result

The `encode()` method returns `Result<EncodeResult, Claim169Error>`:

```rust
use claim169_core::EncodeResult;

let result: EncodeResult = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&private_key)?
    .encode()?;

println!("QR data: {}", result.qr_data);
println!("Compression used: {:?}", result.compression_used);

for warning in &result.warnings {
    println!("Warning: {}", warning.message);
}
```

## Error Handling

```rust
use claim169_core::Claim169Error;

match Encoder::new(claim169, cwt_meta).sign_with_ed25519(&key)?.encode() {
    Ok(result) => println!("Encoded: {}", result.qr_data),
    Err(Claim169Error::EncodingConfig(msg)) => {
        eprintln!("Configuration error: {}", msg);
    }
    Err(Claim169Error::SignatureFailed(msg)) => {
        eprintln!("Signing failed: {}", msg);
    }
    Err(Claim169Error::Crypto(msg)) => {
        eprintln!("Crypto error: {}", msg);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Complete Example

```rust
use claim169_core::{
    Encoder, Claim169, CwtMeta, Gender, MaritalStatus,
    PhotoFormat, Ed25519Signer,
};
use std::time::{SystemTime, UNIX_EPOCH};

fn create_full_credential() -> claim169_core::Result<String> {
    // Generate signing key
    let signer = Ed25519Signer::generate();

    // Current time
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Build comprehensive identity
    let claim169 = Claim169::new()
        .with_id("NATIONAL-ID-2024-000123")
        .with_version("2.0")
        .with_language("eng")
        .with_full_name("Alexandra Maria Johnson")
        .with_first_name("Alexandra")
        .with_middle_name("Maria")
        .with_last_name("Johnson")
        .with_date_of_birth("19880312")
        .with_gender(Gender::Female)
        .with_nationality("USA")
        .with_marital_status(MaritalStatus::Married)
        .with_address("456 Oak Avenue\nApt 7B\nSan Francisco, CA 94102")
        .with_email("alex.johnson@example.com")
        .with_phone("+1 415 555 0123")
        .with_country_of_issuance("USA")
        .with_location_code("US-CA-SF")
        .with_legal_status("citizen");

    // CWT metadata with 1-year validity
    let cwt_meta = CwtMeta::new()
        .with_issuer("https://id.dmv.ca.gov")
        .with_subject("johnson.alexandra.1988")
        .with_issued_at(now)
        .with_not_before(now)
        .with_expires_at(now + 365 * 24 * 60 * 60);  // 1 year

    // Encode with signature
    let qr_data = Encoder::new(claim169, cwt_meta)
        .sign_with(signer, coset::iana::Algorithm::EdDSA)
        .encode()?;

    Ok(qr_data)
}
```
