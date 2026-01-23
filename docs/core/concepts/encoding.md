# Encoding Credentials

This document explains the conceptual model for encoding identity credentials into QR codes.

## Encoding Pipeline

Credential encoding follows a multi-stage pipeline:

```
Identity Data → CBOR → CWT → COSE_Sign1 → [COSE_Encrypt0] → zlib → Base45 → QR Code
```

Each stage serves a specific purpose in producing a compact, secure, verifiable credential.

## 1. Identity Data

Start with the data you want to encode. At minimum, you need:

- **Claim 169 payload** — Identity fields (name, DOB, photo, etc.)
- **CWT metadata** — Issuer, subject, timestamps

### Required vs Optional Fields

All Claim 169 fields are optional. Encode only what's needed:

| Minimal | Typical | Full |
|---------|---------|------|
| id | id, fullName, DOB | All demographics |
| | | + photo |
| | | + biometrics |

### CWT Metadata

| Field | Purpose | Recommendation |
|-------|---------|----------------|
| `issuer` | Who issued the credential | Always set |
| `subject` | Who the credential is about | Optional |
| `issuedAt` | When issued | Recommended |
| `expiresAt` | When it expires | Recommended |
| `notBefore` | When it becomes valid | Optional |

## 2. Signing

All credentials must be signed to enable verification. Choose an algorithm:

### Ed25519 (Recommended)

- Fast signing and verification
- Small signatures (64 bytes)
- Small keys (32 bytes)
- COSE algorithm: EdDSA (-8)

### ECDSA P-256

- Widely supported
- 64-byte signatures
- 32-byte private key
- COSE algorithm: ES256 (-7)

### Key Material

You need a **private key** for signing. The corresponding **public key** is distributed to verifiers.

| Algorithm | Private Key | Public Key |
|-----------|-------------|------------|
| Ed25519 | 32 bytes | 32 bytes |
| ECDSA P-256 | 32 bytes | 33 bytes (compressed) or 65 bytes (uncompressed) |

## 3. Encryption (Optional)

Encrypt credentials when privacy is required:

### When to Encrypt

- QR code may be photographed
- Contains sensitive biometrics
- Privacy regulations apply
- Credential shared across trust boundaries

### Encryption Algorithms

| Algorithm | Key Size | Nonce Size |
|-----------|----------|------------|
| AES-256-GCM | 32 bytes | 12 bytes |
| AES-128-GCM | 16 bytes | 12 bytes |

### Encryption Order

Encryption wraps the signed credential:

```
Sign → Encrypt
```

The verifier must:
1. Decrypt with the symmetric key
2. Verify the signature with the public key

### Nonce Requirements

!!! warning "Never Reuse Nonces"
    Each encryption must use a unique nonce. Reusing nonces with the same key breaks security.

Use `generate_random_nonce()` or your platform's secure random generator.

## 4. Compression

The library automatically compresses the COSE structure using zlib:

- Reduces QR code size significantly
- Especially effective for photo/biometric data
- Transparent to the application

## 5. Base45 Encoding

The final step encodes compressed bytes as alphanumeric text:

- Optimized for QR alphanumeric mode
- More efficient than Base64 for QR codes
- Produces uppercase letters and digits

## Size Considerations

QR code capacity limits what you can encode:

| QR Version | Alphanumeric Capacity |
|------------|----------------------|
| 10 | 395 chars |
| 20 | 1,249 chars |
| 30 | 2,520 chars |
| 40 | 4,296 chars |

### Size Optimization Tips

1. **Include only needed fields** — Omit unused optional fields
2. **Compress photos** — Use JPEG or AVIF, reduce resolution
3. **Limit biometrics** — Include only essential biometric data
4. **Skip biometrics** — Use `skip_biometrics()` for smaller codes

## Encoder Builder Pattern

All SDKs use a builder pattern for encoding:

1. Create encoder with claim data and CWT metadata
2. Configure signing (required)
3. Configure encryption (optional)
4. Call `encode()` to produce Base45 string

See the SDK-specific encoding guides for implementation examples.

## Error Handling

Encoding can fail for several reasons:

| Error | Cause |
|-------|-------|
| Invalid key format | Key bytes wrong length or format |
| Signing failed | Crypto operation failed |
| Encryption failed | Crypto operation failed |
| CBOR encoding failed | Invalid data structure |

Handle errors appropriately in your application.
