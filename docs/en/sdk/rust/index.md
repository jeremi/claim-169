# Rust SDK

[![Crates.io](https://img.shields.io/crates/v/claim169-core.svg)](https://crates.io/crates/claim169-core)
[![Documentation](https://docs.rs/claim169-core/badge.svg)](https://docs.rs/claim169-core)
[![License](https://img.shields.io/crates/l/claim169-core.svg)](https://github.com/jeremi/claim-169)

A Rust library for encoding and decoding MOSIP Claim 169 QR codes, designed for offline verification of digital identity credentials.

## Overview

The `claim169-core` crate provides a complete implementation of the [MOSIP Claim 169](https://github.com/mosip/id-claim-169) specification. It handles the full encoding and decoding pipeline:

```text
Identity Data -> CBOR -> CWT -> COSE_Sign1 -> [COSE_Encrypt0] -> zlib -> Base45 -> QR Code
```

## Features

- **Encoding and Decoding** - Full support for creating and reading Claim 169 QR codes
- **Digital Signatures** - Ed25519 and ECDSA P-256 signature support
- **Encryption** - Optional AES-128-GCM and AES-256-GCM encryption layer
- **Builder Pattern** - Fluent API for configuration
- **HSM Integration** - Trait-based crypto for hardware security modules and cloud KMS
- **Security First** - Signature verification required by default, decompression bomb protection
- **Forward Compatibility** - Unknown fields preserved for future spec extensions

## Supported Algorithms

| Type | Algorithms |
|------|------------|
| Signature | Ed25519 (EdDSA), ECDSA P-256 (ES256) |
| Encryption | AES-128-GCM (A128GCM), AES-256-GCM (A256GCM) |

## Documentation

- [Installation](./installation.md) - Adding the crate to your project
- [Quick Start](./quick-start.md) - Basic encode/decode examples
- [Encoding](./encoding.md) - Creating QR codes with the Encoder
- [Decoding](./decoding.md) - Reading QR codes with the Decoder
- [Encryption](./encryption.md) - Using AES-GCM encryption
- [Custom Crypto](./custom-crypto.md) - HSM and cloud KMS integration
- [API Reference](./api.md) - Complete API documentation
- [Troubleshooting](./troubleshooting.md) - Common errors and solutions

## Quick Example

```rust
use claim169_core::{Encoder, Decoder, Claim169, CwtMeta};

// Create identity data
let claim169 = Claim169::new()
    .with_id("ID-12345")
    .with_full_name("Jane Doe")
    .with_date_of_birth("19900115");

let cwt_meta = CwtMeta::new()
    .with_issuer("https://issuer.example.com")
    .with_expires_at(1800000000);

// Encode to QR string (signed with Ed25519)
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&private_key)?
    .encode()?;

// Decode and verify
let result = Decoder::new(&qr_data)
    .verify_with_ed25519(&public_key)?
    .decode()?;

println!("Name: {:?}", result.claim169.full_name);
println!("Issuer: {:?}", result.cwt_meta.issuer);
```

## Security Model

The library enforces secure defaults:

- **Signatures required** - Encoding without a signature requires explicit `allow_unsigned()`
- **Verification required** - Decoding without verification requires explicit `allow_unverified()`
- **Decompression limits** - Protection against zip bomb attacks (default: 64KB)
- **Timestamp validation** - Expired/not-yet-valid credentials are rejected by default
- **Weak key rejection** - All-zero keys and small-order points are rejected

## License

This project is licensed under the MIT License.
