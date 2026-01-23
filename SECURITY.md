# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

Please report security vulnerabilities through [GitHub's private vulnerability reporting](https://github.com/jeremi/claim-169/security/advisories/new).

This allows us to discuss and address the issue privately before any public disclosure. You will receive updates on the progress of your report through GitHub.

Do not report security vulnerabilities through public GitHub issues.

## Security Considerations

### Cryptographic Primitives

| Algorithm | Purpose | Crate |
|-----------|---------|-------|
| Ed25519 | Signature verification | `ed25519-dalek` |
| ECDSA P-256 | Signature verification | `p256` |
| AES-GCM | Authenticated encryption | `aes-gcm` |

All cryptographic operations use constant-time implementations where applicable.

### Secure Defaults

The `Decoder` requires explicit configuration:

```rust
// Verification is required by default
Decoder::new(qr_text)
    .verify_with_ed25519(&public_key)?
    .decode()?;

// To skip verification, you must explicitly opt out
Decoder::new(qr_text)
    .allow_unverified()
    .decode()?;
```

Default settings:
- Signature verification: **required** (must call `allow_unverified()` to skip)
- Timestamp validation: **enabled**
- Clock skew tolerance: **0 seconds**
- Max decompressed size: **64KB**

### Input Validation

- **Decompression limits**: Configurable `max_decompressed_bytes` (default: 64KB) protects against zip bomb attacks
- **CBOR depth limits**: Maximum nesting depth of 128 prevents stack overflow attacks
- **Timestamp validation**: Credentials are validated against `exp` and `nbf` claims by default
- **Algorithm enforcement**: COSE messages must contain explicit algorithm headers

### Memory Security

- **Key zeroization**: Signing and encryption keys are automatically zeroized on drop via the `zeroize` crate
- **Weak key rejection**: All-zeros keys and small-order Ed25519 points are rejected

### Nonce Management (AES-GCM)

`AesGcmEncryptor::generate_nonce()` is provided for convenience, but **you are responsible for ensuring nonces are never reused** with the same key. Nonce reuse with AES-GCM allows key recovery.

### Key Management

This library does not handle key distribution or trust establishment. You are responsible for:
- Secure key distribution and storage
- Certificate/key pinning and validation
- Key rotation and revocation

## Threat Model

This library is designed to decode and verify Claim 169 QR codes where:
- The QR code content may be adversarially crafted
- The verifier has access to trusted issuer public keys

The library does NOT protect against:
- Key compromise at the issuer
- Replay attacks (implement application-level protection)
- Side-channel attacks on the host system

## Fuzzing

```bash
cargo +nightly fuzz run fuzz_decode
cargo +nightly fuzz run fuzz_base45
cargo +nightly fuzz run fuzz_decompress
```

## Release Signing

- **Git tags**: GPG-signed (`git verify-tag v0.1.0`)
- **npm**: Published with [npm provenance](https://docs.npmjs.com/generating-provenance-statements)
- **PyPI**: Published using [Trusted Publishing](https://docs.pypi.org/trusted-publishers/)
- **crates.io**: Published from verified GitHub Actions workflows
