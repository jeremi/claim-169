# Key Material & Formats

This page explains what key material the library expects (raw bytes vs PEM), and how it maps to MOSIP Claim 169 operations.

## What keys are used?

- **Signing (authenticity)**: Ed25519 (COSE `EdDSA`) or ECDSA P-256 (COSE `ES256`)
- **Encryption (privacy, optional)**: AES-GCM (COSE `A256GCM` or `A128GCM`)

!!! warning "Production key management"
    Treat signing keys and encryption keys as high-value secrets. For production use, keep them in an HSM/KMS and use the library’s “custom crypto” hooks (where available) rather than loading raw private key bytes into application memory.

## Key formats by algorithm

### Ed25519

- **Public key**: 32 bytes
- **Private key**: 32 bytes (seed)

In the Rust crate (default `software-crypto` feature), the decoder also supports verifying keys in **PEM/SPKI** form:

```rust
use claim169_core::Decoder;

let result = Decoder::new(qr_text)
    .verify_with_ed25519_pem(ed25519_public_key_pem)?
    .decode()?;
```

### ECDSA P-256 (ES256)

- **Public key**: SEC1-encoded point, either:
  - **33 bytes** (compressed, starts with `0x02` or `0x03`), or
  - **65 bytes** (uncompressed, starts with `0x04`)
- **Private key**: 32-byte scalar

Rust also supports verifying keys in **PEM/SPKI** form:

```rust
use claim169_core::Decoder;

let result = Decoder::new(qr_text)
    .verify_with_ecdsa_p256_pem(p256_public_key_pem)?
    .decode()?;
```

### AES-GCM (A256GCM / A128GCM)

- **AES-256-GCM key**: 32 bytes
- **AES-128-GCM key**: 16 bytes
- **Nonce/IV**: 12 bytes (random per encryption)

In normal usage you do **not** need to supply a nonce: the encoder generates a random nonce automatically.

!!! danger "Nonce reuse breaks security"
    Never reuse an AES-GCM nonce with the same key. Only use explicit-nonce APIs for testing.

## Generating development keys (Rust)

If you are using the default `software-crypto` feature, you can generate temporary keys for local testing:

```rust
use claim169_core::{Ed25519Signer, EcdsaP256Signer};

let ed_signer = Ed25519Signer::generate();
let ed_public_key: [u8; 32] = ed_signer.public_key_bytes();

let p256_signer = EcdsaP256Signer::generate();
let p256_public_key_uncompressed: Vec<u8> = p256_signer.public_key_uncompressed(); // 65 bytes
```

## Generating AES keys (Python / TypeScript)

=== "Python"

    ```python
    import secrets

    aes256_key = secrets.token_bytes(32)
    aes128_key = secrets.token_bytes(16)
    ```

=== "TypeScript"

    ```ts
    // Browser
    const aes256Key = crypto.getRandomValues(new Uint8Array(32));

    // Node.js
    import { randomBytes } from "crypto";
    const aes256KeyNode = randomBytes(32);
    ```

## Test vectors

For known-good example keys (only for testing), see `test-vectors/valid/*.json`. These vectors include `public_key_hex` and (for some vectors) `private_key_hex`.

