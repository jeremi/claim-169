# Decoding & Verification

This document explains the conceptual model for decoding and verifying Claim 169 credentials.

## Decoding Pipeline

Credential decoding reverses the encoding pipeline:

```
QR Code → Base45 → zlib → COSE → CWT → Claim 169
```

At each stage, the library validates the data structure and applies security checks.

## Verification Model

### Trust Decisions

When decoding a credential, you must make trust decisions:

| Question | Answer |
|----------|--------|
| **Who issued this?** | Check `issuer` claim, use correct public key |
| **Is it authentic?** | Verify signature with issuer's public key |
| **Is it current?** | Validate `exp` and `nbf` timestamps |
| **Is it encrypted?** | Provide decryption key if needed |

### Verification Status

After decoding, check the verification status:

| Status | Meaning |
|--------|---------|
| `Verified` | Signature valid with provided key |
| `Unverified` | Decoded without verification (testing only) |
| Error | Signature invalid or verification failed |

## Signature Verification

### Why Verification is Required

The library requires verification by default because:

- Unverified credentials could be forged
- Attackers could modify legitimate credentials
- Trust assumptions must be explicit

### Choosing a Verifier

Select the verifier matching the issuer's key:

| If issuer uses... | Use verifier... |
|-------------------|-----------------|
| Ed25519 | `verify_with_ed25519(public_key)` |
| ECDSA P-256 | `verify_with_ecdsa_p256(public_key)` |
| HSM/KMS | `verify_with(custom_verifier)` |

### Key ID Resolution

COSE headers may include a key ID (`kid`). Use this to:

1. Look up the correct public key from a keystore
2. Route verification to the correct HSM key
3. Support key rotation

## Timestamp Validation

### Claim Timestamps

CWT credentials support temporal validity:

| Claim | Validation |
|-------|------------|
| `exp` (expiration) | Must be in the future |
| `nbf` (not before) | Must be in the past |
| `iat` (issued at) | Informational only |

### Clock Skew

Real-world systems have clock differences. Configure tolerance:

```
Default: 0 seconds (strict)
Typical: 300 seconds (5 minutes)
```

### Disabling Validation

For testing or when managing timestamps externally:

- Use `without_timestamp_validation()`
- WASM/TypeScript disables by default (browser clocks unreliable)

## Decryption

### Detecting Encryption

The library automatically detects encrypted credentials (COSE_Encrypt0 wrapper).

### Providing Decryption Key

If the credential is encrypted, provide the symmetric key:

| Algorithm | Key Size | Method |
|-----------|----------|--------|
| AES-256-GCM | 32 bytes | `decrypt_with_aes256(key)` |
| AES-128-GCM | 16 bytes | `decrypt_with_aes128(key)` |

### Decryption Order

For encrypted credentials:

1. Decrypt the outer COSE_Encrypt0
2. Verify the inner COSE_Sign1 signature
3. Parse the CWT and Claim 169 payload

## Decoder Builder Pattern

All SDKs use a builder pattern for decoding:

1. Create decoder with QR string
2. Configure verification (required unless opted out)
3. Configure decryption (if needed)
4. Call `decode()` to get result

### Required Configuration

You must configure verification before decoding:

```
✓ decoder.verify_with_ed25519(key).decode()
✓ decoder.allow_unverified().decode()  // Testing only!
✗ decoder.decode()  // Error: verification not configured
```

## Decode Result

A successful decode returns:

| Field | Contents |
|-------|----------|
| `claim169` | Identity data (id, name, DOB, etc.) |
| `cwt_meta` | Token metadata (issuer, timestamps) |
| `verification_status` | `Verified` or `Unverified` |

## Error Handling

Decoding can fail at various stages:

| Stage | Possible Errors |
|-------|-----------------|
| Base45 | Invalid encoding, wrong characters |
| Decompression | Corrupted data, size limit exceeded |
| COSE | Invalid structure, unknown type |
| Signature | Verification failed, wrong key |
| Decryption | Wrong key, corrupted ciphertext |
| CWT | Invalid claims, missing Claim 169 |
| Timestamps | Expired, not yet valid |

### Error Categories

| Error | User Action |
|-------|-------------|
| `Base45Decode` | QR data corrupted or truncated |
| `Decompress` | Data corrupted |
| `DecompressLimitExceeded` | Increase limit or reject |
| `SignatureInvalid` | Wrong key or tampered data |
| `DecryptionFailed` | Wrong decryption key |
| `Expired` | Credential no longer valid |
| `NotYetValid` | Credential not yet active |

## Security Considerations

### Untrusted Input

Treat all QR data as untrusted:

- Validate before processing
- Handle errors gracefully
- Don't trust claims until verified

### Key Management

- Store public keys securely
- Associate keys with issuers
- Support key rotation via key IDs

### Decompression Limits

Protect against decompression bombs:

- Default limit: 64 KB
- Increase only if needed
- Consider memory constraints
