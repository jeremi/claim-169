# Security

This document describes the security model, threat mitigations, and safe defaults implemented in the Claim 169 library.

## Secure by Default

The library enforces security best practices out of the box:

| Protection | Default | Override |
|------------|---------|----------|
| Signature verification | Required | `allow_unverified()` |
| Timestamp validation | Enabled (Rust/Python + TypeScript host-side) | Rust/Python: `without_timestamp_validation()`; TypeScript: `withoutTimestampValidation()` / `validateTimestamps: false` |
| Decompression limit | 64 KB | `max_decompressed_bytes()` |
| CBOR nesting depth | 128 levels | Not configurable |
| Algorithm confusion | Prevented | Algorithm from COSE header only |

## Signature Verification

### Why Verification is Required

All credentials must be cryptographically verified to prevent:

- **Forgery** — Attackers creating fake credentials
- **Tampering** — Modifying legitimate credentials
- **Implicit trust** — Treating untrusted input as authentic data

### Algorithm Selection

The library enforces algorithm selection from the COSE protected header only:

- **No algorithm defaults** — Must specify verifier type
- **No algorithm negotiation** — Uses header algorithm
- **No weak algorithm fallback** — Only Ed25519 and ECDSA P-256

### Weak Key Rejection

The library rejects known-weak keys:

- **Ed25519**: Small-order points (identity, low-order subgroups)
- **ECDSA P-256**: Identity point (point at infinity)

## Decompression Safety

### Decompression Bombs

Malicious QR codes could contain data that decompresses to enormous sizes. The library prevents this:

```
Default limit: 64 KB (65,536 bytes)
Configurable: max_decompressed_bytes()
```

If decompressed data exceeds the limit, decoding fails with `DecompressLimitExceeded`.

### Why 64 KB?

- Sufficient for most identity credentials with photos
- Small enough to prevent memory exhaustion
- Increase only if you need large biometric data

## Timestamp Validation

### Claim Timestamps

CWT credentials support three timestamps:

| Claim | Key | Validation |
|-------|-----|------------|
| `exp` | 4 | Must be in the future |
| `nbf` | 5 | Must be in the past |
| `iat` | 6 | Informational only |

!!! note "What timestamps do (and don’t) protect against"
    Timestamp validation prevents accepting **expired** credentials and credentials that are **not yet valid**.
    It does **not** stop an attacker from replaying a credential that is still within its valid time window.

### Clock Skew Tolerance

Real-world systems have clock drift. Configure tolerance:

```
Default: 0 seconds
Typical: 300 seconds (5 minutes)
```

### Disabling Validation

For testing or when timestamps are managed externally:

- Rust: `without_timestamp_validation()`
- Python: `without_timestamp_validation()`
- TypeScript: Disabled by default (browser clocks unreliable)

## Replay & Revocation

Claim 169 QR codes are often used in **offline** scenarios. In that environment, some classes of protection are necessarily application-dependent:

- **Replay of still-valid credentials**: This library does not maintain a “seen credential” cache. If your workflow needs anti-replay, implement it at the application level (e.g., short-lived credentials, transaction/challenge binding, local allow-lists, or online checks when available).
- **Revocation / credential status**: This library does not define or enforce revocation mechanisms. If you need revocation, integrate with your ecosystem’s status checks or trusted allow/block lists.

## CBOR Safety

### Nesting Depth Limit

Deeply nested CBOR structures can cause stack overflow. The library limits nesting to 128 levels.

### Unknown Fields

Unknown CBOR keys are preserved in `unknown_fields` for forward compatibility. This allows:

- New specification versions with additional fields
- Custom vendor extensions
- Graceful degradation

## Encryption Considerations

### When to Encrypt

Use encryption when:

- QR code may be photographed or shared
- Credential contains sensitive biometrics
- Privacy regulations require data protection

### Key Management

- **Never hardcode keys** — Use secure key storage
- **Rotate keys regularly** — Establish key rotation policies
- **Use unique nonces** — Never reuse nonces with the same key

### Nonce Requirements

AES-GCM requires unique 12-byte nonces:

- Use `generate_random_nonce()` for new encryptions
- Never reuse nonces with the same key
- Nonce reuse breaks confidentiality

## Custom Crypto Providers

When integrating HSM or KMS:

### Key Isolation

- Keep private keys in the HSM/KMS
- Only export public keys for verification
- Use key IDs in COSE headers for key resolution

### Error Handling

Custom providers should return appropriate errors:

- `CryptoError::KeyNotFound` — Key ID not in keystore
- `CryptoError::UnsupportedAlgorithm` — Algorithm not supported
- `CryptoError::VerificationFailed` — Signature invalid

## Threat Model

### In Scope

| Threat | Mitigation |
|--------|------------|
| Credential forgery | Signature verification |
| Credential tampering | Signature verification |
| Use-after-expiry / not-before enforcement | Timestamp validation (`exp`/`nbf`) |
| Memory exhaustion | Decompression limits |
| Algorithm confusion | Header-only algorithm |
| Weak keys | Key validation |
| Privacy leakage | Optional encryption |

### Out of Scope

| Threat | Reason |
|--------|--------|
| Key compromise | Application responsibility |
| Replay of still-valid credentials | Application responsibility |
| Revocation / credential status | Application responsibility |
| Side-channel attacks | Depends on crypto implementation |
| QR code physical security | Application responsibility |
| Key distribution | Application responsibility |
