# Claim 169

> **Alpha Software**: This library is under active development. APIs may change. Test thoroughly before production use.

[![CI](https://github.com/jeremi/claim-169/actions/workflows/ci.yml/badge.svg)](https://github.com/jeremi/claim-169/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/jeremi/claim-169/graph/badge.svg)](https://codecov.io/gh/jeremi/claim-169)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![crates.io](https://img.shields.io/crates/v/claim169-core.svg?label=crates.io)](https://crates.io/crates/claim169-core)
[![PyPI](https://img.shields.io/pypi/v/claim169.svg?label=pypi)](https://pypi.org/project/claim169/)
[![npm](https://img.shields.io/npm/v/claim169.svg?label=npm)](https://www.npmjs.com/package/claim169)
[![Maven Central](https://img.shields.io/maven-central/v/fr.acn.claim169/claim169-core.svg?label=maven)](https://central.sonatype.com/artifact/fr.acn.claim169/claim169-core)

Multi-language SDKs for encoding and decoding [MOSIP Claim 169](https://github.com/mosip/id-claim-169/tree/main) QR codes — a compact, secure format for digital identity credentials.

> **Try it now**: [Interactive Playground](https://jeremi.github.io/claim-169/) — encode and decode QR codes in your browser

## Why Claim 169?

Claim 169 QR codes enable **offline verification** of identity credentials — no internet connection required at the point of verification. Common use cases include:

- **Border control & immigration** — verify travel documents without network access
- **Healthcare credentials** — vaccination records, insurance cards, patient IDs
- **Government benefits** — social protection programs, subsidy distribution
- **Digital driver's licenses** — offline-verifiable mobile driving permits
- **Event access** — tamper-evident tickets with identity binding

The format is designed for constrained environments: a typical credential fits in a single QR code scannable by any smartphone camera.

## Overview

[MOSIP Claim 169](https://github.com/mosip/id-claim-169/tree/main) is an IANA-registered specification for encoding identity data in QR codes. This project provides SDKs for multiple languages:

| SDK | Package | Use Case |
|-----|---------|----------|
| **Rust** | [`claim169-core`](https://crates.io/crates/claim169-core) | High-performance core library, embedded systems |
| **Python** | [`claim169`](https://pypi.org/project/claim169/) | Server-side integration, HSM support |
| **TypeScript/JavaScript** | [`claim169`](https://www.npmjs.com/package/claim169) | Browser apps, Node.js services |
| **Kotlin/Java** | [`claim169-core`](https://central.sonatype.com/artifact/fr.acn.claim169/claim169-core) | Android apps, JVM server-side |

All SDKs share the same Rust core via native bindings (Python, Kotlin/Java) or WebAssembly (TypeScript), ensuring consistent behavior across platforms.

### Key Features

- **Encode and decode** identity credentials to/from QR codes
- **Sign and verify** with Ed25519 or ECDSA P-256
- **Encrypt and decrypt** with AES-GCM (128 or 256 bit)
- **Pluggable crypto backends** for HSM, cloud KMS, smart cards, and TPM integration
- **Comprehensive security** including weak key rejection, decompression limits, and timestamp validation

### Encoding Pipeline

```
Identity Data → CBOR → CWT → COSE_Sign1 → [COSE_Encrypt0] → zlib → Base45 → QR Code
```

### Supported Algorithms

| Operation | Algorithms |
|-----------|------------|
| Signing | Ed25519, ECDSA P-256 (ES256) |
| Encryption | AES-128-GCM, AES-256-GCM |
| Compression | zlib (DEFLATE) |
| Encoding | Base45 |

## Project Structure

```
claim-169/
├── core/
│   ├── claim169-core/     # Rust core library
│   ├── claim169-jni/      # Kotlin/Java bindings (UniFFI)
│   ├── claim169-wasm/     # WebAssembly bindings
│   └── claim169-python/   # Python bindings (PyO3)
├── sdks/
│   ├── kotlin/            # Kotlin/Java SDK
│   └── typescript/        # TypeScript/JavaScript SDK
├── playground/            # Interactive web playground
├── examples/              # Runnable examples (Python, TypeScript)
├── fuzz/                  # Fuzz testing targets
├── test-vectors/          # Test vectors for compliance
└── tools/                 # CLI and utilities
```

## Quick Start

### Rust

```toml
[dependencies]
claim169-core = "0.1.0-alpha.3"
```

```rust
use claim169_core::{Decoder, Encoder, Claim169, CwtMeta};

// Decode and verify a QR code
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .decode()?;

println!("Name: {:?}", result.claim169.full_name);
```

<details>
<summary>Encoding example</summary>

```rust
// Create and sign a new credential
let claim = Claim169::default()
    .with_id("123456789")
    .with_full_name("Jane Doe");

let meta = CwtMeta::new()
    .with_issuer("https://issuer.example.com")
    .with_expires_at(1800000000);

let qr_data = Encoder::new(claim, meta)
    .sign_with_ed25519(&private_key)?
    .encode()?;
```

</details>

### Python

```bash
pip install claim169
```

```python
from claim169 import decode_with_ed25519

# Decode and verify a QR code
result = decode_with_ed25519(qr_text, public_key_bytes)
print(f"Name: {result.claim169.full_name}")
print(f"Expired: {result.cwt_meta.is_expired()}")
```

<details>
<summary>Encoding example</summary>

```python
from claim169 import Claim169Input, CwtMetaInput, encode_with_ed25519

claim = Claim169Input(id="123", full_name="Jane Doe")
meta = CwtMetaInput(issuer="https://issuer.example.com")
qr_data = encode_with_ed25519(claim, meta, private_key_bytes)
```

</details>

### TypeScript/JavaScript

```bash
npm install claim169
```

```typescript
import { Decoder } from 'claim169';

// Decode and verify a QR code
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

console.log(`Name: ${result.claim169.fullName}`);
```

<details>
<summary>Encoding example</summary>

```typescript
import { Encoder, Claim169Input, CwtMetaInput } from 'claim169';

const claim: Claim169Input = { id: "123", fullName: "Jane Doe" };
const meta: CwtMetaInput = { issuer: "https://issuer.example.com" };
const qrData = new Encoder(claim, meta)
  .signWithEd25519(privateKey)
  .encode();
```

</details>

### Kotlin/Java

```kotlin
// Gradle (Kotlin DSL)
implementation("fr.acn.claim169:claim169-core:0.1.0-alpha.3")
```

```kotlin
import fr.acn.claim169.*

// Decode and verify a QR code
val result = Claim169.decode(qrText) {
    verifyWithEd25519(publicKey)
}

println("Name: ${result.claim169.fullName}")
```

<details>
<summary>Encoding example</summary>

```kotlin
val qrData = Claim169.encode(
    claim169 {
        id = "123456789"
        fullName = "Jane Doe"
    },
    cwtMeta {
        issuer = "https://issuer.example.com"
        expiresAt = 1800000000L
    }
) {
    signWithEd25519(privateKey)
}
```

</details>

## Building from Source

### Prerequisites

- Rust 1.75+ with cargo
- Python 3.8+ with maturin (for Python bindings)
- Node.js 18+ with npm (for TypeScript SDK)
- JDK 17+ with Gradle (for Kotlin SDK)
- wasm-pack (for WebAssembly bindings)

### Compatibility

| Platform | Minimum Version | Notes |
|----------|-----------------|-------|
| Rust | 1.75+ | MSRV tested in CI |
| Python | 3.8+ | Wheels for Linux, macOS, Windows |
| Node.js | 18+ | ESM and CommonJS |
| Browsers | Chrome 89+, Firefox 89+, Safari 15+ | Via WebAssembly |
| JVM | JDK 17+ | Kotlin/Java via JNA |
| Android | API 24+ (7.0) | Native .so via JNA |

### Build All

```bash
# Clone the repository
git clone https://github.com/jeremi/claim-169.git
cd claim-169

# Build Rust libraries
cargo build --release

# Run tests
cargo test --all-features

# Build Python bindings
cd core/claim169-python
maturin develop --release

# Build WASM and TypeScript SDK
cd ../../sdks/typescript
npm install
npm run build
```

### Build Individual Components

```bash
# Core library only
cargo build -p claim169-core --release

# WASM bindings
cd core/claim169-wasm
wasm-pack build --target bundler --release

# Python bindings
cd core/claim169-python
maturin build --release

# Kotlin/Java SDK (requires native library)
cargo build -p claim169-jni
cd sdks/kotlin
./gradlew :claim169-core:test
```

## Security

This library implements several security measures:

- **Signature verification** with Ed25519 and ECDSA P-256
- **Weak key rejection** (all-zeros keys, small-order Ed25519 points)
- **Decompression limits** to prevent zip bomb attacks (default: 64KB)
- **CBOR depth limits** to prevent stack overflow (max: 128)
- **Timestamp validation** with configurable clock skew tolerance
- **Memory zeroization** for sensitive key material

See [SECURITY.md](SECURITY.md) for detailed security information and threat model.

### Reporting Security Issues

Please report security vulnerabilities through [GitHub's private vulnerability reporting](https://github.com/jeremi/claim-169/security/advisories/new). Do not use public GitHub issues for security reports.

## Documentation

- [Rust API Documentation](https://docs.rs/claim169-core)
- [MOSIP Claim 169 Specification](https://github.com/mosip/id-claim-169/tree/main)
- [Examples](examples/) - Runnable Python and TypeScript examples
- [Contributing Guide](CONTRIBUTING.md)
- [Security Policy](SECURITY.md)
- [Changelog](CHANGELOG.md)

## Testing

```bash
# Run all tests
cargo test --all-features

# Run with verbose output
cargo test --all-features -- --nocapture

# Run fuzz tests (requires nightly)
cd fuzz
cargo +nightly fuzz run fuzz_decode
```

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

Quick overview:
1. Fork the repository and create a feature branch
2. Make your changes following the project's code style
3. Ensure all tests pass (`cargo test --all-features`)
4. Submit a Pull Request

Please ensure code is formatted (`cargo fmt`) and has no clippy warnings (`cargo clippy --all-features`).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [MOSIP](https://mosip.io/) for the Claim 169 specification
- [RustCrypto](https://github.com/RustCrypto) for cryptographic primitives
- [coset](https://github.com/AltNyx/coset) for COSE implementation
