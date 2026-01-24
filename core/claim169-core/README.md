# claim169-core

> **Alpha Software**: This library is under active development. APIs may change without notice. Not recommended for production use without thorough testing.

[![Crates.io](https://img.shields.io/crates/v/claim169-core.svg)](https://crates.io/crates/claim169-core)
[![Documentation](https://docs.rs/claim169-core/badge.svg)](https://docs.rs/claim169-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust library for encoding and decoding MOSIP Claim 169 QR codes.

## Features

- **Encode** Claim 169 identity credentials to QR codes (Ed25519, ECDSA P-256)
- **Decode** Claim 169 identity credentials from QR codes
- **Sign** with Ed25519 or ECDSA P-256
- **Verify** signatures (Ed25519, ECDSA P-256)
- **Encrypt/Decrypt** with AES-GCM (128 or 256 bit)
- **Pluggable crypto backends** for HSM integration
- **Comprehensive security**: weak key rejection, decompression limits, timestamp validation

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
claim169-core = "0.1.0-alpha.3"
```

### Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `software-crypto` | Yes | Software implementations of Ed25519, ECDSA P-256, AES-GCM |

Disable default features to use only custom crypto backends:

```toml
[dependencies]
claim169-core = { version = "0.1.0-alpha.3", default-features = false }
```

## Encoding (Creating QR Codes)

### Ed25519 Signed (Recommended)

```rust
use claim169_core::{Encoder, Claim169, CwtMeta};

// Assume you already have a 32-byte Ed25519 private key
let private_key: [u8; 32] = [/* your private key bytes */];

let claim = Claim169::default()
    .with_id("123456789")
    .with_full_name("John Doe")
    .with_date_of_birth("1990-01-15");

let meta = CwtMeta::new()
    .with_issuer("https://issuer.example.com")
    .with_expires_at(1800000000);

let qr_data = Encoder::new(claim, meta)
    .sign_with_ed25519(&private_key)?
    .encode()?;

// qr_data is a Base45 string ready for QR code generation
```

### ECDSA P-256 Signed

```rust
use claim169_core::Encoder;

// Assume you already have a 32-byte ECDSA P-256 private key
let private_key: [u8; 32] = [/* your private key bytes */];

let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ecdsa_p256(&private_key)?
    .encode()?;
```

### Signed and Encrypted

```rust
use claim169_core::Encoder;

// Assume you already have keys
let private_key: [u8; 32] = [/* your Ed25519 private key */];
let aes_key: [u8; 32] = [/* your AES-256 key */];

// Sign first, then encrypt (order enforced by library)
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&private_key)?
    .encrypt_with_aes256(&aes_key)?
    .encode()?;
```

### Unsigned (Testing Only)

```rust
// Requires explicit opt-in - INSECURE
let qr_data = Encoder::new(claim169, cwt_meta)
    .allow_unsigned()
    .encode()?;
```

### Custom Signer (HSM Integration)

```rust
use claim169_core::{Encoder, Signer, CryptoResult};
use coset::iana;

struct HsmSigner {
    hsm_client: HsmClient, // Your HSM client
}

impl Signer for HsmSigner {
    fn sign(
        &self,
        algorithm: iana::Algorithm,
        _key_id: Option<&[u8]>,
        data: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        // Delegate to your HSM
        self.hsm_client.sign(data)
    }

    // Optional: override to provide a key ID
    fn key_id(&self) -> Option<&[u8]> {
        Some(b"my-key-id")
    }
}

let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with(hsm_signer, iana::Algorithm::EdDSA)
    .encode()?;
```

## Decoding (Reading QR Codes)

### With Ed25519 Verification (Recommended)

```rust
use claim169_core::Decoder;

let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .decode()?;

println!("ID: {:?}", result.claim169.id);
println!("Name: {:?}", result.claim169.full_name);
println!("Issuer: {:?}", result.cwt_meta.issuer);
```

### With ECDSA P-256 Verification

```rust
let result = Decoder::new(qr_content)
    .verify_with_ecdsa_p256(&public_key)?
    .decode()?;
```

### Decrypting Encrypted Credentials

```rust
let result = Decoder::new(qr_content)
    .decrypt_with_aes256(&aes_key)?
    .verify_with_ed25519(&public_key)?
    .decode()?;
```

### Without Verification (Testing Only)

```rust
// Requires explicit opt-in - INSECURE
let result = Decoder::new(qr_content)
    .allow_unverified()
    .decode()?;
```

### With Options

```rust
let result = Decoder::new(qr_content)
    .skip_biometrics()                    // Don't parse biometric data
    .max_decompressed_bytes(32768)        // 32KB limit (default: 64KB)
    .clock_skew_tolerance(60)             // 60 seconds tolerance
    .without_timestamp_validation()       // Disable exp/nbf validation
    .verify_with_ed25519(&public_key)?
    .decode()?;
```

### Custom Verifier (HSM Integration)

```rust
use claim169_core::{Decoder, SignatureVerifier, CryptoResult, CryptoError};
use coset::iana;

struct HsmVerifier {
    hsm_client: HsmClient, // Your HSM client
}

impl SignatureVerifier for HsmVerifier {
    fn verify(
        &self,
        algorithm: iana::Algorithm,
        _key_id: Option<&[u8]>,
        data: &[u8],
        signature: &[u8],
    ) -> CryptoResult<()> {
        // Delegate to your HSM
        self.hsm_client
            .verify(data, signature)
            .map_err(|_| CryptoError::VerificationFailed)
    }
}

let result = Decoder::new(qr_content)
    .verify_with(hsm_verifier)
    .decode()?;
```

## Data Model

### Claim169

The main identity data structure containing:

- **Demographics**: id, name, date of birth, gender, address, etc.
- **Biometrics**: fingerprints, iris scans, face images, voice samples
- **Unknown fields**: Forward-compatible storage for new spec fields

```rust
let claim = result.claim169;

// Demographics
println!("ID: {:?}", claim.id);
println!("Name: {:?}", claim.full_name);
println!("DOB: {:?}", claim.date_of_birth);

// Biometrics
if claim.has_biometrics() {
    println!("Has {} biometric entries", claim.biometric_count());
}
```

### CwtMeta

CWT (CBOR Web Token) metadata:

```rust
let meta = result.cwt_meta;

println!("Issuer: {:?}", meta.issuer);
println!("Expires: {:?}", meta.expires_at);

// Check validity
if meta.is_expired(current_time) {
    println!("Credential expired!");
}
```

## Error Handling

```rust
use claim169_core::{Decoder, Claim169Error};

match Decoder::new(qr_content).allow_unverified().decode() {
    Ok(result) => println!("Decoded: {:?}", result.claim169.full_name),
    Err(Claim169Error::Base45Decode(msg)) => println!("Invalid QR encoding: {}", msg),
    Err(Claim169Error::Expired(ts)) => println!("Expired at {}", ts),
    Err(Claim169Error::SignatureInvalid(msg)) => println!("Bad signature: {}", msg),
    Err(Claim169Error::DecodingConfig(msg)) => println!("Config error: {}", msg),
    Err(e) => println!("Error: {}", e),
}
```

## Security

- **Always verify signatures** in production
- **Always sign credentials** when encoding
- **Weak keys are rejected** (all-zeros, small-order points)
- **Decompression is limited** to prevent zip bomb attacks
- **Timestamps are validated** by default
- **Signing keys are zeroized** on drop (where supported)

See [SECURITY.md](../../SECURITY.md) for detailed security information.

## Key Generation (Optional Convenience)

You will need cryptographic keys, but in production those are typically provisioned and managed externally (HSM/KMS or secure key management). The examples below are provided for local development and testing convenience; for production, prefer integrating your existing key management and using the `Signer`/`SignatureVerifier` traits.

### Ed25519 Keys

```rust
use claim169_core::Ed25519Signer;

// Generate a new random Ed25519 signing key
let signer = Ed25519Signer::generate();

// Get the public key for verification (32 bytes)
let public_key = signer.public_key_bytes();

// Or get a verifier directly
let verifier = signer.verifying_key();

// You can also create a signer from existing private key bytes (32 bytes)
let private_key_bytes: [u8; 32] = [/* your 32-byte private key */];
let signer = Ed25519Signer::from_bytes(&private_key_bytes)?;
```

### ECDSA P-256 Keys

```rust
use claim169_core::EcdsaP256Signer;

// Generate a new random ECDSA P-256 signing key
let signer = EcdsaP256Signer::generate();

// Get the public key in uncompressed SEC1 format (65 bytes)
let public_key = signer.public_key_uncompressed();

// Or get a verifier directly
let verifier = signer.verifying_key();

// You can also create a signer from existing private key bytes (32 bytes)
let private_key_bytes: [u8; 32] = [/* your 32-byte private key */];
let signer = EcdsaP256Signer::from_bytes(&private_key_bytes)?;
```

### AES-GCM Keys

AES keys must be generated using a secure random number generator. The library doesn't provide a key generation method, so use the `rand` crate:

```rust
use rand::RngCore;

// Generate a 32-byte key for AES-256-GCM
let mut aes256_key = [0u8; 32];
rand::thread_rng().fill_bytes(&mut aes256_key);

// Generate a 16-byte key for AES-128-GCM
let mut aes128_key = [0u8; 16];
rand::thread_rng().fill_bytes(&mut aes128_key);
```

**Note**: In production, use a cryptographically secure random number generator. The `rand` crate with `OsRng` is recommended:

```rust
use rand::{RngCore, rngs::OsRng};

let mut aes_key = [0u8; 32];
OsRng.fill_bytes(&mut aes_key);
```

## Building

```bash
# Build
cargo build --release

# Run tests
cargo test --all-features

# Generate documentation
cargo doc --all-features --open

# Run clippy
cargo clippy --all-features
```

## License

MIT License - See [LICENSE](../../LICENSE) for details.
