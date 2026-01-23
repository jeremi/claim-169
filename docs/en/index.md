# Claim 169

A multi-language implementation of the [MOSIP Claim 169](https://github.com/mosip/id-claim-169) QR code specification for encoding and verifying digital identity credentials.

## Overview

MOSIP Claim 169 defines a compact, secure format for encoding identity data in QR codes, optimized for offline verification. This library provides:

- **Rust core library** with full encoding, decoding, signature verification, and encryption support
- **Python SDK** for server-side integration
- **TypeScript/JavaScript SDK** via WebAssembly for browser and Node.js
- **Interactive playground** for experimenting with QR codes

## Encoding Pipeline

```
Identity Data → CBOR → CWT → COSE_Sign1 → [COSE_Encrypt0] → zlib → Base45 → QR Code
```

## Supported Algorithms

| Operation | Algorithms |
|-----------|------------|
| Signing | Ed25519, ECDSA P-256 (ES256) |
| Encryption | AES-128-GCM, AES-256-GCM |
| Compression | zlib (DEFLATE) |
| Encoding | Base45 |

## Quick Example

=== "Rust"

    ```rust
    use claim169_core::Decoder;

    // Decode a QR code
    let result = Decoder::new(qr_content)
        .verify_with_ed25519(&public_key)?
        .decode()?;

    println!("Name: {:?}", result.claim169.full_name);
    ```

=== "Python"

    ```python
    from claim169 import decode_with_ed25519

    result = decode_with_ed25519(qr_text, public_key_bytes)
    print(f"Name: {result.claim169.full_name}")
    ```

=== "TypeScript"

    ```typescript
    import { Decoder } from 'claim169';

    const result = new Decoder(qrText)
      .verifyWithEd25519(publicKey)
      .decode();

    console.log(`Name: ${result.claim169.fullName}`);
    ```

## Next Steps

- [Installation](getting-started/installation.md) - Install the SDK for your language
- [Quick Start](getting-started/quick-start.md) - Encode and decode your first credential
- [Key Material & Formats](guides/keys.md) - Understand key formats and PEM support
- [Security & Validation](guides/security.md) - Learn safe defaults and policy knobs
- [Playground](playground.md) - Try it in your browser
