# Key Management

This document covers key formats, generation, storage, and rotation for Claim 169 credentials.

## Key Types

### Signing Keys (Asymmetric)

Used for credential signing and verification:

| Algorithm | Private Key | Public Key | Use |
|-----------|-------------|------------|-----|
| Ed25519 | 32 bytes | 32 bytes | Signing |
| ECDSA P-256 | 32 bytes | 33 or 65 bytes | Signing |

### Encryption Keys (Symmetric)

Used for credential encryption:

| Algorithm | Key Size | Use |
|-----------|----------|-----|
| AES-256-GCM | 32 bytes | Encryption |
| AES-128-GCM | 16 bytes | Encryption |

## Key Formats

### Ed25519

| Type | Format | Size |
|------|--------|------|
| Private | Raw bytes | 32 bytes |
| Public | Raw bytes | 32 bytes |

Ed25519 keys are simply raw byte arrays. No encoding wrapper is used.

### ECDSA P-256

| Type | Format | Size |
|------|--------|------|
| Private | Raw scalar | 32 bytes |
| Public (compressed) | SEC1 compressed | 33 bytes |
| Public (uncompressed) | SEC1 uncompressed | 65 bytes |

Public keys start with:
- `0x02` or `0x03` for compressed (33 bytes)
- `0x04` for uncompressed (65 bytes)

### AES Keys

| Algorithm | Format | Size |
|-----------|--------|------|
| AES-256-GCM | Raw bytes | 32 bytes |
| AES-128-GCM | Raw bytes | 16 bytes |

## Key Generation

### Generate Signing Keys

Use cryptographically secure random generation:

=== "Command Line"

    ```bash
    # Ed25519
    openssl genpkey -algorithm ED25519 -out private.pem
    openssl pkey -in private.pem -pubout -out public.pem

    # ECDSA P-256
    openssl ecparam -name prime256v1 -genkey -out private.pem
    openssl ec -in private.pem -pubout -out public.pem
    ```

=== "Python"

    ```python
    from cryptography.hazmat.primitives.asymmetric import ed25519, ec
    from cryptography.hazmat.primitives import serialization

    # Ed25519
    private_key = ed25519.Ed25519PrivateKey.generate()
    public_key = private_key.public_key()
    private_bytes = private_key.private_bytes_raw()  # 32 bytes
    public_bytes = public_key.public_bytes_raw()     # 32 bytes

    # ECDSA P-256
    private_key = ec.generate_private_key(ec.SECP256R1())
    public_key = private_key.public_key()
    ```

=== "Node.js"

    ```javascript
    const crypto = require('crypto');

    // Ed25519
    const { publicKey, privateKey } = crypto.generateKeyPairSync('ed25519');

    // ECDSA P-256
    const { publicKey, privateKey } = crypto.generateKeyPairSync('ec', {
      namedCurve: 'prime256v1'
    });
    ```

### Generate Encryption Keys

```bash
# 32 bytes for AES-256
openssl rand 32 > aes256.key

# 16 bytes for AES-128
openssl rand 16 > aes128.key
```

## Key Storage

### Principles

1. **Never hardcode keys** — Use environment variables or secure storage
2. **Encrypt at rest** — Protect stored keys
3. **Limit access** — Principle of least privilege
4. **Audit access** — Log key usage

### Storage Options

| Option | Security | Use Case |
|--------|----------|----------|
| Environment variables | Medium | Development, containers |
| Encrypted files | Medium | Simple deployments |
| Secret managers | High | Cloud deployments |
| HSM/KMS | Highest | Enterprise, regulated |

### Example: Environment Variables

```bash
export CLAIM169_PRIVATE_KEY="$(cat private.key | xxd -p | tr -d '\n')"
export CLAIM169_PUBLIC_KEY="$(cat public.key | xxd -p | tr -d '\n')"
```

## Key IDs

COSE supports key identifiers (`kid`) in headers:

### Purpose

- **Key selection** — Verifier selects correct key
- **Key rotation** — Support multiple active keys
- **Multi-issuer** — Different keys for different issuers

### Format

Key IDs are arbitrary byte strings. Common formats:

| Format | Example |
|--------|---------|
| UUID | `550e8400-e29b-41d4-a716-446655440000` |
| Hash of public key | First 8 bytes of SHA-256 |
| Sequential | `key-001`, `key-002` |
| Timestamp-based | `2024-01-15-primary` |

### Setting Key ID

When encoding, the signer can specify a key ID that will be included in the COSE header. Verifiers use this to look up the correct public key.

## Key Rotation

### Why Rotate Keys

- Limit exposure from potential compromise
- Comply with security policies
- Retire deprecated algorithms

### Rotation Strategy

1. **Generate new key pair**
2. **Distribute new public key** to verifiers
3. **Start signing** with new key
4. **Keep old public key** for verification of existing credentials
5. **Retire old key** after all old credentials expire

### Overlapping Validity

During rotation, both keys are valid:

```
Old key: ████████████░░░░░░░░
New key: ░░░░░░░████████████████
         ^        ^
         Start    Complete
         new      rotation
```

## HSM/KMS Integration

For high-security deployments, use hardware or cloud key management.

### Benefits

- Private keys can stay in secure hardware (depending on your provider and configuration)
- Hardware-enforced access controls
- Audit logging
- Can support compliance programs (for example, FIPS/Common Criteria) depending on your HSM/KMS configuration and audits

### Integration Points

The library supports custom crypto providers:

| Trait | Purpose |
|-------|---------|
| `SignatureVerifier` | Custom signature verification |
| `Signer` | Custom signing |
| `Decryptor` | Custom decryption |
| `Encryptor` | Custom encryption |

### Cloud KMS Examples

| Provider | Service |
|----------|---------|
| AWS | AWS KMS, CloudHSM |
| Google Cloud | Cloud KMS, Cloud HSM |
| Azure | Key Vault, Managed HSM |

See SDK-specific custom crypto guides for implementation details.

## Security Checklist

- [ ] Keys generated with secure random
- [ ] Private keys stored securely
- [ ] Public keys distributed to verifiers
- [ ] Key rotation plan in place
- [ ] Key IDs used for multi-key scenarios
- [ ] Access to keys audited
- [ ] Backup and recovery procedures
