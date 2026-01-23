# Rust API Reference

Full API documentation is available at [docs.rs/claim169-core](https://docs.rs/claim169-core).

## Features

The crate ships with software crypto enabled by default:

- `software-crypto` (default): Ed25519, ECDSA P-256, AES-GCM helpers

Disable default features if you want to plug in your own crypto (HSM/KMS) via the `Signer` / `SignatureVerifier` / `Encryptor` / `Decryptor` traits.

## Core types

### Decoder

Builder for decoding QR data.

```rust
use claim169_core::Decoder;

let decoder = Decoder::new(qr_data);
```

#### Methods

| Method | Description |
|--------|-------------|
| `new(qr_text)` | Create decoder from Base45 text |
| `verify_with(verifier)` | Use a custom verifier (HSM/KMS integration) |
| `verify_with_ed25519(public_key)` | Verify Ed25519 (requires `software-crypto`) |
| `verify_with_ed25519_pem(pem)` | Verify Ed25519 from PEM/SPKI (requires `software-crypto`) |
| `verify_with_ecdsa_p256(public_key)` | Verify ECDSA P-256 from SEC1 bytes (requires `software-crypto`) |
| `verify_with_ecdsa_p256_pem(pem)` | Verify ECDSA P-256 from PEM/SPKI (requires `software-crypto`) |
| `decrypt_with(decryptor)` | Use a custom decryptor (HSM/KMS integration) |
| `decrypt_with_aes256(key)` | Decrypt AES-256-GCM (requires `software-crypto`) |
| `decrypt_with_aes128(key)` | Decrypt AES-128-GCM (requires `software-crypto`) |
| `allow_unverified()` | Skip signature verification (testing only) |
| `skip_biometrics()` | Skip biometric parsing for speed |
| `without_timestamp_validation()` | Disable `exp`/`nbf` checks |
| `clock_skew_tolerance(seconds)` | Allow clock skew for timestamp checks |
| `max_decompressed_bytes(bytes)` | Set the decompression size limit |
| `decode()` | Execute decoding pipeline |

### Encoder

Builder for encoding credentials.

```rust
use claim169_core::{Encoder, Claim169, CwtMeta};

let encoder = Encoder::new(claim, meta);
```

#### Methods

| Method | Description |
|--------|-------------|
| `new(claim169, cwt_meta)` | Create encoder |
| `sign_with(signer, algorithm)` | Use a custom signer (HSM/KMS integration) |
| `sign_with_ed25519(private_key)` | Sign with Ed25519 (requires `software-crypto`) |
| `sign_with_ecdsa_p256(private_key)` | Sign with ECDSA P-256 (requires `software-crypto`) |
| `allow_unsigned()` | Skip signing (testing only) |
| `encrypt_with(encryptor, algorithm)` | Use a custom encryptor |
| `encrypt_with_aes256(key)` | Encrypt with AES-256-GCM (requires `software-crypto`) |
| `encrypt_with_aes128(key)` | Encrypt with AES-128-GCM (requires `software-crypto`) |
| `encrypt_with_aes256_nonce(key, nonce)` | AES-256-GCM with explicit nonce (testing only) |
| `encrypt_with_aes128_nonce(key, nonce)` | AES-128-GCM with explicit nonce (testing only) |
| `skip_biometrics()` | Skip biometric fields during encoding |
| `encode()` | Execute encoding pipeline |

### DecodeResult

Result of successful decoding.

```rust
pub struct DecodeResult {
    pub claim169: Claim169,
    pub cwt_meta: CwtMeta,
    pub verification_status: VerificationStatus,
    pub warnings: Vec<Warning>,
}
```

## Errors

High-level operations return `claim169_core::Result<T>` which is a `Result<T, Claim169Error>`.

Common `Claim169Error` cases:

- `DecodingConfig(...)` (no verifier and no `allow_unverified()`)
- `EncodingConfig(...)` (no signer and no `allow_unsigned()`)
- `SignatureInvalid(...)`
- `DecryptionFailed(...)`
- `Expired(ts)` / `NotYetValid(ts)`
- `DecompressLimitExceeded { max_bytes }`

## Example

```rust
use claim169_core::{
    Decoder, Encoder,
    Claim169, CwtMeta,
    Gender,
};

fn main() -> Result<(), claim169_core::Claim169Error> {
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
