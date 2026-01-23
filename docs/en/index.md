# Claim 169

Encode and verify digital identity credentials with the [MOSIP Claim 169](https://github.com/mosip/id-claim-169) QR code specification.

[Get started](getting-started/installation.md){ .md-button .md-button--primary }
[Try the playground](playground.md){ .md-button }

## What is Claim 169?

MOSIP Claim 169 defines a compact, secure format for encoding identity data in QR codes, optimized for offline verification. This repository provides:

- **Rust core library** (encoding, decoding, verification, encryption)
- **Python SDK** (server-side integration)
- **TypeScript/JavaScript SDK** (WASM for browser and Node.js)
- **Interactive playground** (try vectors and build QR payloads)

<div class="grid cards" markdown>

-   ### Rust Core
    High-performance encoding/decoding with signature verification and optional encryption.

    [Rust API](api/rust.md){ .md-button }

-   ### Python SDK
    Simple functions for verification, decryption, and decoding pipelines in Python services.

    [Python API](api/python.md){ .md-button }

-   ### TypeScript / JavaScript
    WebAssembly-powered SDK for browser and Node.js.

    [TypeScript API](api/typescript.md){ .md-button }

-   ### Playground
    Encode, decode, decrypt, and verify without installing anything.

    [Open playground](playground.md){ .md-button }

</div>

## Encoding Pipeline

```text
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
    import { Decoder } from "claim169";

    const result = new Decoder(qrText)
      .verifyWithEd25519(publicKey)
      .decode();

    console.log(`Name: ${result.claim169.fullName}`);
    ```

## Quick Links

<div class="grid cards" markdown>

-   ### Installation
    Install the SDK for your language.

    [Install](getting-started/installation.md){ .md-button }

-   ### Quick Start
    Encode and decode your first credential.

    [Quick start](getting-started/quick-start.md){ .md-button }

-   ### Keys
    Key formats and how to provide them.

    [Keys](guides/keys.md){ .md-button }

-   ### Security & Validation
    Safe defaults and policy knobs.

    [Security](guides/security.md){ .md-button }

-   ### Specification
    The wire format and structures.

    [Specification](specification.md){ .md-button }

-   ### Troubleshooting
    Common errors and fixes.

    [Troubleshooting](guides/troubleshooting.md){ .md-button }

</div>

## Next Steps

- [Quick Start](getting-started/quick-start.md) — encode and decode your first credential
- [Key Material & Formats](guides/keys.md) — key formats and PEM support
- [Security & Validation](guides/security.md) — safe defaults and policy knobs
- [Glossary](guides/glossary.md) — CBOR, COSE, CWT, etc.
- [Versioning](guides/versioning.md) — how docs relate to releases
- [Troubleshooting](guides/troubleshooting.md) — common errors and fixes

**Need help?** Start with [Troubleshooting](guides/troubleshooting.md) or see [Contributing](guides/contributing.md) for how to improve the docs.
