# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

Please report security vulnerabilities by emailing security@openspp.org.

Do not report security vulnerabilities through public GitHub issues.

## Security Considerations

### Cryptographic Properties

This library uses the following cryptographic primitives:

- **Ed25519**: Signature verification via `ed25519-dalek` (RustCrypto)
- **ECDSA P-256**: Signature verification via `p256` (RustCrypto)
- **AES-GCM**: Authenticated encryption via `aes-gcm` (RustCrypto)

All cryptographic operations are performed using constant-time implementations where applicable.

### Memory Security

- **Key Zeroization**: All signing keys and encryption keys are automatically zeroized on drop using the `zeroize` crate. This prevents sensitive key material from lingering in memory after use.

- **Weak Key Rejection**: The library rejects known weak keys including:
  - All-zeros Ed25519 public keys
  - Small-order Ed25519 points (potential subgroup attacks)
  - P-256 keys with all-zero coordinates

### Input Validation

- **Decompression Limits**: A configurable `max_decompressed_bytes` limit (default: 64KB) protects against zip bomb attacks.

- **CBOR Depth Limits**: A maximum nesting depth of 128 prevents stack overflow attacks via deeply nested structures.

- **Timestamp Validation**: Credentials are validated against `exp` (expiration) and `nbf` (not-before) claims by default.

- **Algorithm Enforcement**: COSE messages must contain explicit algorithm headers - no default algorithms are assumed.

### Secure Defaults

The library uses secure defaults:

```rust
DecodeOptions::default()
    .allow_unverified: false    // Signature verification required
    .validate_timestamps: true  // Timestamp validation enabled
    .clock_skew_tolerance_seconds: 0  // No clock skew tolerance
```

### Nonce Management (AES-GCM)

The `AesGcmEncryptor` provides `generate_nonce()` for convenience, but **you are responsible for ensuring nonces are never reused** with the same key. Nonce reuse with AES-GCM is catastrophic and allows key recovery.

For high-security contexts:
- Use a hardware random number generator (HSM)
- Consider AES-GCM-SIV for nonce-misuse resistance
- Implement nonce tracking at your application layer

### Key ID Handling

The library extracts `key_id` from COSE headers but does not enforce binding between key IDs and keys. If your security model requires key ID validation, implement this check before calling `decode_with_verifier()`.

### Key Management

The library does not handle key distribution or trust establishment. You are responsible for:

1. **Secure Key Distribution**: Obtaining issuer public keys through secure channels
2. **Certificate/Key Pinning**: Validating that keys come from trusted issuers
3. **Key Rotation**: Handling key updates and revocation
4. **JWKS Fetching**: If using `.well-known` JWKS endpoints, use TLS with certificate validation

### Audit Logging

Security-relevant events to consider logging in your application:

- Signature verification failures (potential attack indicator)
- Expired or not-yet-valid credentials
- Unknown algorithm requests
- Decompression limit exceeded
- CBOR depth limit exceeded
- Weak key rejection

### Clock Synchronization

Timestamp validation (`exp`, `nbf`) requires synchronized clocks between issuer and verifier. Use the `clock_skew_tolerance_seconds` option for deployments with potential clock drift:

```rust
DecodeOptions::new()
    .with_clock_skew_tolerance(60)  // 60 seconds tolerance
```

### Rate Limiting

The library does not implement rate limiting. For production deployments, implement rate limiting at your application or infrastructure layer to prevent:

- Brute-force signature guessing attempts
- Denial of service via expensive operations

## Threat Model

This library is designed to decode and verify Claim 169 QR codes in scenarios where:

1. The QR code content may be adversarially crafted
2. The verifier has access to trusted issuer public keys
3. Biometric data (if present) is handled appropriately

The library does NOT protect against:

1. Key compromise at the issuer
2. Physical QR code tampering (use tamper-evident materials)
3. Replay attacks (implement application-level replay protection)
4. Side-channel attacks on the host system

## Fuzzing

The library includes fuzz testing targets. To run fuzzing:

```bash
cd fuzz
cargo +nightly fuzz run fuzz_decode
cargo +nightly fuzz run fuzz_base45
cargo +nightly fuzz run fuzz_decompress
```

## Dependencies

All cryptographic dependencies are from the RustCrypto project or well-established maintainers:

| Crate | Purpose |
|-------|---------|
| `ed25519-dalek` | Ed25519 signatures |
| `p256` | ECDSA P-256 signatures |
| `aes-gcm` | AES-GCM encryption |
| `coset` | COSE structure parsing |
| `ciborium` | CBOR parsing |
| `zeroize` | Secure memory clearing |

## Release Signing

All releases are cryptographically signed and verifiable:

### Git Tags

Release tags are GPG-signed. To verify a release:

```bash
git verify-tag v0.1.0
```

### Package Provenance

- **npm**: Published with [npm provenance](https://docs.npmjs.com/generating-provenance-statements), providing cryptographic proof that packages were built by our GitHub Actions workflows. Look for the provenance badge on the npm package page.

- **PyPI**: Published using [Trusted Publishing](https://docs.pypi.org/trusted-publishers/) (OpenID Connect), which provides cryptographic attestation that packages came from our GitHub repository.

- **crates.io**: Published from verified GitHub Actions workflows with authentication tokens.

### Verifying Releases

1. **Verify git tag signature**:
   ```bash
   git fetch --tags
   git verify-tag v0.1.0
   ```

2. **Check npm provenance**:
   - Visit https://www.npmjs.com/package/claim169
   - Look for the "Provenance" badge linking to the build

3. **Verify checksums**: Each GitHub Release includes SHA256 checksums for all artifacts
