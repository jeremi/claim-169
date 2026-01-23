# Encryption

This document explains when and how to use encryption with Claim 169 credentials.

## When to Encrypt

Encryption adds a layer of confidentiality. Consider encryption when:

| Scenario | Recommendation |
|----------|----------------|
| QR displayed publicly | Encrypt |
| QR may be photographed | Encrypt |
| Contains biometrics | Encrypt |
| Privacy regulations apply | Encrypt |
| Controlled environment only | Signing sufficient |
| Public information only | Signing sufficient |

## Encryption vs Signing

| Property | Signing | Encryption |
|----------|---------|------------|
| **Authenticity** | ✓ Proves who issued it | ✗ Does not prove origin |
| **Integrity** | ✓ Detects tampering | ✓ Detects tampering |
| **Confidentiality** | ✗ Anyone can read | ✓ Only key holders can read |
| **Required** | Always | Optional |

!!! important "Always Sign"
    Encryption does not replace signing. Always sign credentials, optionally encrypt.

## Encryption Order

Credentials are signed first, then encrypted:

```
Identity Data → Sign → Encrypt → Compress → Base45
```

This order ensures:

1. Signature covers original data
2. Encryption protects signed data
3. Verifier sees authentic content after decryption

## Supported Algorithms

| Algorithm | Key Size | Security Level |
|-----------|----------|----------------|
| AES-256-GCM | 32 bytes | 256-bit |
| AES-128-GCM | 16 bytes | 128-bit |

Both algorithms are AEAD (Authenticated Encryption with Associated Data), providing:

- Confidentiality (data is encrypted)
- Integrity (tampering is detected)

## Nonce Requirements

AES-GCM requires a 12-byte nonce (initialization vector):

!!! danger "Never Reuse Nonces"
    Reusing a nonce with the same key completely breaks AES-GCM security. An attacker can:

    - Recover the keystream
    - Decrypt other messages
    - Forge new messages

### Generating Nonces

Use the library's secure random generator:

```
generate_random_nonce()  // Returns 12 random bytes
```

Or use your platform's cryptographic random:

- Python: `secrets.token_bytes(12)`
- Node.js: `crypto.randomBytes(12)`
- Rust: `rand::thread_rng().gen::<[u8; 12]>()`

### Nonce Uniqueness

Ensure uniqueness through:

1. **Random generation** — 2^96 possible values, collision unlikely
2. **Counter-based** — Increment counter for each encryption
3. **Timestamp + random** — Combine time with random bits

## Key Management

### Symmetric Key Distribution

Unlike signing (public/private keys), encryption uses symmetric keys:

- Same key encrypts and decrypts
- Key must be shared securely with verifiers
- Compromise affects all credentials encrypted with that key

### Key Distribution Strategies

| Strategy | Use Case |
|----------|----------|
| Pre-shared keys | Closed systems, known verifiers |
| Key derivation | Derive from shared secret or PIN |
| Key encapsulation | Wrap symmetric key with public key |
| HSM/KMS | Enterprise key management |

### Key Rotation

Plan for key rotation:

1. Include key identifier in credential
2. Verifiers maintain key history
3. Retire old keys on schedule
4. New credentials use current key

## Threat Model

### What Encryption Protects Against

| Threat | Protection |
|--------|------------|
| Casual observation | ✓ |
| Photography of QR | ✓ |
| Network interception | ✓ |
| Storage of QR images | ✓ |

### What Encryption Does NOT Protect Against

| Threat | Why Not |
|--------|---------|
| Key compromise | All credentials decryptable |
| Authorized verifier misuse | They have the key |
| Credential holder sharing | They can show decrypted data |
| Metadata analysis | Encryption present is visible |

## Custom Encryption Providers

For HSM or KMS integration, implement the `Encryptor` and `Decryptor` traits:

### Encryptor Interface

```
encrypt(algorithm, key_id, nonce, aad, plaintext) → ciphertext
```

Parameters:
- `algorithm`: COSE algorithm (A256GCM or A128GCM)
- `key_id`: Optional key identifier
- `nonce`: 12-byte initialization vector
- `aad`: Additional authenticated data
- `plaintext`: Data to encrypt

### Decryptor Interface

```
decrypt(algorithm, key_id, nonce, aad, ciphertext) → plaintext
```

Parameters:
- `algorithm`: COSE algorithm
- `key_id`: Optional key identifier (from COSE header)
- `nonce`: 12-byte IV (from COSE header)
- `aad`: Additional authenticated data
- `ciphertext`: Encrypted data with auth tag

## Performance Considerations

| Operation | Relative Cost |
|-----------|---------------|
| Signing | Low |
| Encryption | Low |
| Signing + Encryption | Low |
| Key derivation | Variable |
| HSM operations | Higher latency |

AES-GCM is hardware-accelerated on most platforms, making encryption overhead minimal.

## Size Impact

Encryption adds overhead:

| Component | Size |
|-----------|------|
| COSE_Encrypt0 header | ~20 bytes |
| Nonce | 12 bytes |
| Auth tag | 16 bytes |
| **Total overhead** | ~48 bytes |

This is typically insignificant compared to credential content.
