# Rust API Reference

Full API documentation is available at [docs.rs/claim169-core](https://docs.rs/claim169-core).

## Core Types

### Decoder

Builder for decoding QR data.

```rust
use claim169_core::Decoder;

let decoder = Decoder::new(qr_data);
```

#### Methods

| Method | Description |
|--------|-------------|
| `new(qr_data: &str)` | Create decoder from Base45 string |
| `verify_with_ed25519(&public_key)` | Enable Ed25519 verification |
| `verify_with_ecdsa_p256(&public_key)` | Enable ECDSA P-256 verification |
| `allow_unverified()` | Skip signature verification (testing only) |
| `decrypt_with_aes256(&key)` | Enable AES-256-GCM decryption |
| `decrypt_with_aes128(&key)` | Enable AES-128-GCM decryption |
| `decode()` | Execute decoding pipeline |

### Encoder

Builder for encoding credentials.

```rust
use claim169_core::{Encoder, Claim169Input, CwtMetaInput};

let encoder = Encoder::new(claim, meta);
```

#### Methods

| Method | Description |
|--------|-------------|
| `new(claim: Claim169Input, meta: CwtMetaInput)` | Create encoder |
| `sign_with_ed25519(&private_key)` | Sign with Ed25519 |
| `sign_with_ecdsa_p256(&private_key)` | Sign with ECDSA P-256 |
| `allow_unsigned()` | Skip signing (testing only) |
| `encrypt_with_aes256(&key)` | Encrypt with AES-256-GCM |
| `encrypt_with_aes128(&key)` | Encrypt with AES-128-GCM |
| `encode()` | Execute encoding pipeline |

### DecodeResult

Result of successful decoding.

```rust
pub struct DecodeResult {
    pub claim169: Claim169,
    pub cwt_meta: CwtMeta,
}
```

### Claim169

Identity data structure with builder methods for ergonomic construction.

```rust
// Using builder pattern (recommended)
let claim = Claim169::new()
    .with_id("USER-001")
    .with_full_name("Alice Smith")
    .with_gender(Gender::Female)
    .with_email("alice@example.com");

// Or using minimal constructor
let claim = Claim169::minimal("USER-001", "Alice Smith");
```

#### Builder Methods

| Method | Description |
|--------|-------------|
| `new()` | Create empty claim |
| `minimal(id, full_name)` | Create claim with ID and name |
| `with_id(id)` | Set unique identifier |
| `with_full_name(name)` | Set full name |
| `with_first_name(name)` | Set first name |
| `with_last_name(name)` | Set last name |
| `with_date_of_birth(dob)` | Set DOB (YYYYMMDD) |
| `with_gender(gender)` | Set gender |
| `with_email(email)` | Set email |
| `with_phone(phone)` | Set phone |
| `with_address(addr)` | Set address |
| `with_nationality(nat)` | Set nationality |
| `with_marital_status(status)` | Set marital status |
| ... | (all fields have `with_*` methods) |

#### Fields

```rust
pub struct Claim169 {
    pub id: Option<String>,
    pub version: Option<String>,
    pub language: Option<String>,
    pub full_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<String>,
    pub gender: Option<Gender>,
    pub address: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub nationality: Option<String>,
    pub marital_status: Option<MaritalStatus>,
    pub guardian: Option<String>,
    pub photo: Option<Vec<u8>>,
    pub photo_format: Option<PhotoFormat>,
    pub legal_status: Option<String>,
    pub country_of_issuance: Option<String>,
    pub location_code: Option<String>,
    pub secondary_language: Option<String>,
    pub secondary_full_name: Option<String>,
    pub best_quality_fingers: Option<Vec<u8>>,
    pub biometrics: Option<Vec<Biometric>>,
    pub unknown_fields: HashMap<i64, ciborium::Value>,
}
```

### CwtMeta

CWT metadata from decoded credential.

```rust
pub struct CwtMeta {
    pub issuer: Option<String>,
    pub subject: Option<String>,
    pub expires_at: Option<i64>,
    pub not_before: Option<i64>,
    pub issued_at: Option<i64>,
}
```

### CwtMetaInput

Input for CWT metadata when encoding.

```rust
pub struct CwtMetaInput {
    pub issuer: Option<String>,
    pub subject: Option<String>,
    pub expires_at: Option<i64>,
    pub not_before: Option<i64>,
    pub issued_at: Option<i64>,
}
```

## Enums

### Gender

```rust
pub enum Gender {
    Male = 1,
    Female = 2,
    Other = 3,
}
```

### MaritalStatus

```rust
pub enum MaritalStatus {
    Unmarried = 1,
    Married = 2,
    Divorced = 3,
}
```

### PhotoFormat

```rust
pub enum PhotoFormat {
    Jpeg = 1,
    Jpeg2000 = 2,
    Avif = 3,
}
```

## Error Types

### DecodeError

```rust
pub enum DecodeError {
    Base45(Base45Error),
    Decompression(DecompressionError),
    Cose(CoseError),
    Cwt(CwtError),
    Claim169(Claim169Error),
    Signature(SignatureError),
    Decryption(DecryptionError),
    Configuration(String),
}
```

## Example

```rust
use claim169_core::{
    Decoder, Encoder,
    Claim169, CwtMeta,
    Gender, DecodeError
};

fn main() -> Result<(), DecodeError> {
    // Create a credential using builder pattern
    let claim = Claim169::new()
        .with_id("USER-001")
        .with_full_name("Alice Smith")
        .with_gender(Gender::Female);

    let meta = CwtMeta::new()
        .with_issuer("https://example.com");

    // Encode (unsigned for demo)
    let qr_data = Encoder::new(claim, meta)
        .allow_unsigned()
        .encode()?;

    // Decode
    let result = Decoder::new(&qr_data)
        .allow_unverified()
        .decode()?;

    println!("Name: {:?}", result.claim169.full_name);
    Ok(())
}
```
