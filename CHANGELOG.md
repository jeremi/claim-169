# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Security
- TypeScript `decode()` now requires a verification key unless `allowUnverified: true` is set (prevents accidental acceptance of forged credentials).
- Python `decode_encrypted_aes()` / `decode_with_decryptor()` now require a verifier unless `allow_unverified=True` is set (prevents silently skipping nested signature verification).

### Added
- CHANGELOG.md for tracking version history
- CONTRIBUTING.md with development guidelines

### Changed
- Python `decode()` now requires a verification key by default; unverified decoding requires `allow_unverified=True`.
- Python `decode_with_ed25519()` / `decode_with_ecdsa_p256()` now accept decode options (biometrics skip, decompression limit, timestamp validation).
- Removed `unsafe impl Send/Sync` from Python callback hook wrappers.

## [0.1.0-alpha] - 2024-01-22

Initial alpha release of the MOSIP Claim 169 QR code library.

### Added

#### Core Library (Rust)
- Full encoding and decoding of MOSIP Claim 169 credentials
- Ed25519 and ECDSA P-256 signature support
- AES-128-GCM and AES-256-GCM encryption support
- Secure-by-default design (verification required unless explicitly disabled)
- Decompression bomb protection with configurable limits
- CBOR nesting depth limits to prevent stack overflow
- Weak key rejection for Ed25519 and ECDSA
- Builder pattern API for both Encoder and Decoder

#### Python SDK
- Native extension via PyO3 for high performance
- `decode_unverified()` for testing scenarios
- `decode_with_ed25519()` and `decode_with_ecdsa_p256()` for verified decoding
- `decode_with_verifier()` for custom/HSM verification
- `decode_encrypted_aes()` for encrypted credentials
- `encode_with_ed25519()` and `encode_with_ecdsa_p256()` for signing
- `encode_signed_encrypted()` for signed and encrypted credentials
- Full type hints and docstrings

#### TypeScript/JavaScript SDK
- WebAssembly-based implementation for browser and Node.js
- `Decoder` class with fluent builder API
- `verifyWithEd25519()` and `verifyWithEcdsaP256()` for signature verification
- `decryptWithAes256()` and `decryptWithAes128()` for decryption
- `Encoder` class for creating signed credentials
- Full TypeScript type definitions
- ESM module support with bundler configuration examples

#### Test Infrastructure
- 17 test vectors covering valid, invalid, and edge cases
- Fuzz testing targets for security testing
- Conformance test suite
- Integration tests across all SDKs

#### Documentation
- Comprehensive README for each SDK
- SECURITY.md with threat model and security considerations
- MOSIP Claim 169 specification reference (claim_169.md)
- JSON Schema for output validation

### Security
- Signature verification is required by default (secure-by-default)
- Weak key detection and rejection
- Memory zeroization for sensitive key material
- Configurable decompression limits (default: 64KB)
- CBOR depth limits (max: 128)

## Types of Changes

- `Added` for new features
- `Changed` for changes in existing functionality
- `Deprecated` for soon-to-be removed features
- `Removed` for now removed features
- `Fixed` for any bug fixes
- `Security` for vulnerability fixes
