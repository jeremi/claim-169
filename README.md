# Claim 169

> **Alpha Software**: This library is under active development. APIs may change without notice. Not recommended for production use without thorough testing.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)

A Rust implementation of the [MOSIP Claim 169](https://github.com/mosip/id-claim-169/tree/main) QR code specification for encoding and verifying digital identity credentials.

## Overview

MOSIP Claim 169 defines a compact, secure format for encoding identity data in QR codes. This implementation provides:

- **Rust core library** with full encoding, decoding, signature verification, and encryption support
- **Python bindings** for server-side integration with HSM support
- **TypeScript/JavaScript SDK** via WebAssembly for browser and Node.js
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
│   ├── claim169-wasm/     # WebAssembly bindings
│   └── claim169-python/   # Python bindings (PyO3)
├── sdks/
│   └── typescript/        # TypeScript/JavaScript SDK
├── examples/              # Runnable examples (Python, TypeScript)
├── fuzz/                  # Fuzz testing targets
├── test-vectors/          # Test vectors for compliance
└── tools/                 # CLI and utilities
```

## Quick Start

### Rust

```toml
# Cargo.toml
[dependencies]
claim169-core = "0.1"
```

```rust
use claim169_core::{Decoder, Encoder, Claim169, CwtMeta};

// Decoding (with Ed25519 verification)
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .decode()?;

println!("Name: {:?}", result.claim169.full_name);
println!("Issuer: {:?}", result.cwt_meta.issuer);

// Encoding (Ed25519 signed)
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&private_key)?
    .encode()?;
```

### Python

```bash
pip install claim169
```

```python
from claim169 import Claim169Input, CwtMetaInput, encode_with_ed25519, decode_with_ed25519

# Encoding (Ed25519 signed)
claim = Claim169Input(id="123", full_name="John Doe")
meta = CwtMetaInput(issuer="https://example.com")
qr_data = encode_with_ed25519(claim, meta, private_key_bytes)

# Decoding (with Ed25519 verification)
result = decode_with_ed25519(qr_text, public_key_bytes)

print(f"Name: {result.claim169.full_name}")
print(f"Verified: {result.is_verified()}")

# Check expiration
if result.cwt_meta.is_expired():
    print("Credential has expired!")
```

### TypeScript/JavaScript

```bash
npm install claim169
```

```typescript
import { Encoder, Decoder, Claim169Input, CwtMetaInput } from 'claim169';

// Encoding (Ed25519 signed)
const claim169: Claim169Input = { id: "123", fullName: "John Doe" };
const cwtMeta: CwtMetaInput = { issuer: "https://example.com" };
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();

// Decoding (with Ed25519 verification)
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

console.log(`Name: ${result.claim169.fullName}`);
console.log(`Issuer: ${result.cwtMeta.issuer}`);
```

## Building from Source

### Prerequisites

- Rust 1.70+ with cargo
- Python 3.8+ with maturin (for Python bindings)
- Node.js 18+ with npm (for TypeScript SDK)
- wasm-pack (for WebAssembly bindings)

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

Please report security vulnerabilities to security@openspp.org. Do not use public GitHub issues for security reports.

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
