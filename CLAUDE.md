# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a multi-language implementation of the **MOSIP Claim 169 QR Code Specification** (IANA registered tag 169). It decodes identity credentials from QR codes using a compact binary format optimized for offline verification.

**Encoding Pipeline:**
```
Identity Data → CBOR (numeric keys) → CWT → COSE_Sign1 → zlib → Base45 → QR Code
```

## Build & Test Commands

### Rust Core Library
```bash
# Build
cargo build --release

# Test all (includes unit, integration, security tests)
cargo test --all-features

# Run a single test
cargo test test_name --all-features

# Run tests in a specific file
cargo test --test integration_tests --all-features

# Lint
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

### Python SDK
```bash
# Build native extension (from repo root)
cd core/claim169-python && maturin develop

# Run tests
cd core/claim169-python && uv run pytest tests/ -v

# Run single test
cd core/claim169-python && uv run pytest tests/test_decode.py::test_name -v
```

### TypeScript/WASM SDK
```bash
cd sdks/typescript

# Build WASM module
npm run build:wasm

# Build TypeScript
npm run build:ts

# Run tests
npm test
```

### Test Vectors
```bash
# Generate test vectors (required before running SDK tests)
cargo run -p generate-vectors
```

### Fuzz Testing
```bash
cargo +nightly fuzz run fuzz_decode
cargo +nightly fuzz run fuzz_base45
cargo +nightly fuzz run fuzz_decompress
```

## Architecture

### Workspace Structure
```
core/
  claim169-core/     # Rust core library (all decoding logic)
  claim169-python/   # PyO3 bindings
  claim169-wasm/     # wasm-bindgen bindings
sdks/
  python/            # Python SDK with type hints
  typescript/        # TypeScript SDK with full types
tools/
  generate-vectors/  # Test vector generator
test-vectors/        # JSON test fixtures (valid/, invalid/, edge/)
schema/              # JSON Schema for output validation
```

### Core Library Modules (`core/claim169-core/src/`)

- **`pipeline/`** - Decoding stages executed in order:
  - `base45.rs` - Base45 decode
  - `decompress.rs` - zlib decompress (with bomb protection)
  - `cose.rs` - COSE_Sign1/Encrypt0 parsing and verification
  - `cwt.rs` - CWT claim extraction (iss, exp, nbf, iat + claim 169)
  - `claim169.rs` - CBOR map transformation to typed struct

- **`crypto/`** - Cryptographic implementations:
  - `traits.rs` - `SignatureVerifier`, `Signer`, `Decryptor` traits
  - `software/` - Software implementations (Ed25519, ECDSA P-256, AES-GCM)

- **`model/`** - Data structures:
  - `claim169.rs` - Main `Claim169` struct with all demographic/biometric fields
  - `enums.rs` - Gender, MaritalStatus, PhotoFormat, BiometricFormat, etc.
  - `biometrics.rs` - `Biometric` struct (data, format, sub_format, issuer)
  - `cwt_meta.rs` - CWT metadata (issuer, subject, timestamps)

- **`error.rs`** - Error types for each pipeline stage
- **`decode.rs`** - Public decoding API: `Decoder` builder
- **`encode.rs`** - Public encoding API: `Encoder` builder
- **`lib.rs`** - Crate exports and top-level docs

### CBOR Key Mapping (from Claim 169 spec)

| Range | Purpose |
|-------|---------|
| 1-23 | Demographics (id, name, DOB, gender, address, etc.) |
| 24-49 | Reserved for future demographics |
| 50-65 | Biometrics (fingers, iris, face, palm, voice) |
| 66-99 | Reserved for future use |

Biometric structure uses keys 0-3: data (bstr), format (int), sub_format (int), issuer (tstr)

### Security Features

- Decompression bomb protection via `max_decompressed_bytes`
- CBOR nesting depth limit (128) to prevent stack overflow
- No algorithm defaults in COSE verification (prevents alg confusion attacks)
- Secure-by-default: verification required unless `allow_unverified()` explicitly called
- Weak key rejection for Ed25519 (small order points) and ECDSA (identity point)

## Commit Message Format

This project uses **Conventional Commits** for automatic changelog generation. All commits must follow this format:

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Commit Types

| Type | Description | Changelog Section |
|------|-------------|-------------------|
| `feat` | New feature | Features |
| `fix` | Bug fix | Bug Fixes |
| `docs` | Documentation only | Documentation |
| `perf` | Performance improvement | Performance |
| `refactor` | Code refactoring (no feature/fix) | Refactoring |
| `test` | Adding/updating tests | Testing |
| `chore` | Maintenance tasks | Miscellaneous |
| `ci` | CI/CD changes | CI/CD |
| `build` | Build system changes | Build |

### Scopes (Optional)

Use scopes to indicate which part of the codebase is affected:

- `core` - Rust core library
- `python` - Python SDK
- `typescript` - TypeScript SDK
- `wasm` - WASM bindings
- `deps` - Dependencies (e.g., `chore(deps): update coset to 0.4`)
- `release` - Release automation (auto-generated, skipped in changelog)

### Examples

```bash
# Feature with scope
feat(core): add support for palm biometrics

# Bug fix
fix(python): handle empty QR code input

# Breaking change (add ! after type)
feat(core)!: change decode() return type to Result

# Breaking change with footer
feat(core): remove deprecated unverified_decode()

BREAKING CHANGE: Use decode() with allow_unverified() option instead.

# Documentation
docs: update installation instructions

# Chore (dependencies)
chore(deps): update ed25519-dalek to 2.1
```

### Breaking Changes

Mark breaking changes with `!` after the type/scope OR include a `BREAKING CHANGE:` footer. Breaking changes will be highlighted in the changelog.

## Release Process

Releases are automated via GitHub Actions:

1. **Prepare release**: Run the "Prepare Release" workflow with version (e.g., `0.2.0`)
2. **Review PR**: A PR is created with version bumps and changelog
3. **Merge PR**: After CI passes, merge the release PR
4. **Tag release**: Create and push a GPG-signed tag:
   ```bash
   git checkout main && git pull
   git tag -s v0.2.0 -m "Release v0.2.0"
   git push origin v0.2.0
   ```
5. **Auto-publish**: Publishing to crates.io, PyPI, and npm happens automatically

### Pre-release Versions

Use pre-release suffixes for alpha/beta releases:
- `0.2.0-alpha` - Alpha release (npm tag: `alpha`)
- `0.2.0-beta` - Beta release (npm tag: `beta`)
- `0.2.0-rc.1` - Release candidate (npm tag: `rc`)

## Specification Reference

The implementation follows `claim_169.md` (MOSIP Claim 169 v1.2.0). Key compliance points:
- All enums use 1-indexed values matching spec (Gender: 1=Male, 2=Female, 3=Other)
- JSON output serializes enums as integers (not strings) to match schema
- `best_quality_fingers` validated to range 0-10
- Unknown CBOR keys preserved in `unknown_fields` for forward compatibility
