# Installation

## Adding to Your Project

Add `claim169-core` to your `Cargo.toml`:

```toml
[dependencies]
claim169-core = "0.2.0-alpha"
```

Or use cargo add:

```bash
cargo add claim169-core
```

## Feature Flags

The crate has one feature flag:

| Feature | Default | Description |
|---------|---------|-------------|
| `software-crypto` | Yes | Software implementations of Ed25519, ECDSA P-256, and AES-GCM |

### Default Configuration

By default, the `software-crypto` feature is enabled, providing ready-to-use cryptographic implementations:

```toml
[dependencies]
claim169-core = "0.2.0-alpha"
```

This includes:
- `Ed25519Signer` / `Ed25519Verifier` - Ed25519 signing and verification
- `EcdsaP256Signer` / `EcdsaP256Verifier` - ECDSA P-256 signing and verification
- `AesGcmEncryptor` / `AesGcmDecryptor` - AES-128/256-GCM encryption and decryption

### HSM/KMS Integration (No Default Features)

For hardware security modules or cloud KMS integration, disable default features:

```toml
[dependencies]
claim169-core = { version = "0.2.0-alpha", default-features = false }
```

This removes the software crypto dependencies and requires you to implement the cryptographic traits:

- [`Signer`](./custom-crypto.md#signer-trait) - For signing credentials
- [`SignatureVerifier`](./custom-crypto.md#signatureverifier-trait) - For verifying signatures
- [`Encryptor`](./custom-crypto.md#encryptor-trait) - For encrypting credentials
- [`Decryptor`](./custom-crypto.md#decryptor-trait) - For decrypting credentials

See [Custom Crypto](./custom-crypto.md) for implementation details.

## Minimum Supported Rust Version

The crate supports Rust 1.70 and later.

## Dependencies

When `software-crypto` is enabled, the crate depends on:

- `ed25519-dalek` - Ed25519 signatures
- `p256` - ECDSA P-256 signatures
- `aes-gcm` - AES-GCM encryption
- `rand` - Random number generation
- `zeroize` - Secure memory clearing

Core dependencies (always included):

- `coset` - COSE parsing
- `ciborium` - CBOR encoding/decoding
- `base45` - Base45 encoding
- `flate2` - zlib compression
- `serde` / `serde_json` - Serialization
- `thiserror` - Error handling

## Verifying Installation

Create a simple test to verify the installation:

```rust
use claim169_core::{Claim169, CwtMeta, Encoder, Decoder};

fn main() -> claim169_core::Result<()> {
    // Create minimal identity
    let claim169 = Claim169::minimal("test-id", "Test User");
    let cwt_meta = CwtMeta::new().with_issuer("test");

    // Encode (unsigned for testing)
    let qr_data = Encoder::new(claim169.clone(), cwt_meta)
        .allow_unsigned()
        .encode()?;

    // Decode
    let result = Decoder::new(&qr_data)
        .allow_unverified()
        .decode()?;

    assert_eq!(result.claim169.id, claim169.id);
    println!("Installation verified successfully!");

    Ok(())
}
```

Run with:

```bash
cargo run
```
