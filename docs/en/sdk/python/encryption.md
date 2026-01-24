# Encryption

This guide covers encrypting credentials with AES-GCM for additional privacy protection.

## When to Use Encryption

Encrypt credentials when:

- QR codes may be photographed by third parties
- Credentials contain sensitive biometric data
- Privacy regulations require data protection
- Credentials are shared across trust boundaries

## Encryption Overview

The library supports **sign-then-encrypt**: credentials are first signed, then the signed payload is encrypted.

```
Identity Data -> Sign -> Encrypt -> Compress -> Base45 -> QR Code
```

Decryption reverses the process:

```
QR Code -> Base45 -> Decompress -> Decrypt -> Verify -> Identity Data
```

## Supported Algorithms

| Algorithm | Key Size | Nonce Size | Use Case |
|-----------|----------|------------|----------|
| AES-256-GCM | 32 bytes | 12 bytes | High security (recommended) |
| AES-128-GCM | 16 bytes | 12 bytes | Standard security |

## Encoding with Encryption

### Sign + Encrypt with AES-256-GCM

```python
import claim169

# Identity data
claim = claim169.Claim169Input(id="ENC-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(
    issuer="https://id.example.org",
    expires_at=1900000000
)

# Keys
sign_key = bytes.fromhex(
    "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
)  # Ed25519 private key (32 bytes)

encrypt_key = bytes.fromhex(
    "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
)  # AES-256 key (32 bytes)

# Encode with signing and encryption
qr_data = claim169.encode_signed_encrypted(
    claim,
    meta,
    sign_key,
    encrypt_key
)

print(f"Encrypted credential: {len(qr_data)} characters")
```

### Sign + Encrypt with AES-128-GCM

```python
import claim169

claim = claim169.Claim169Input(id="ENC128-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(
    issuer="https://id.example.org",
    expires_at=1900000000
)

sign_key = bytes(32)  # Ed25519 private key
encrypt_key = bytes(16)  # AES-128 key (16 bytes)

qr_data = claim169.encode_signed_encrypted_aes128(
    claim,
    meta,
    sign_key,
    encrypt_key
)
```

## Decoding Encrypted Credentials

### Decrypt AES-256-GCM with Verification

For encrypted credentials, you typically need both:

1. Decryption key (symmetric AES key)
2. Verification key (signing public key) or verification callback

```python
import claim169
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PublicKey

# Keys
encrypt_key = bytes.fromhex(
    "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
)
public_key_bytes = bytes.fromhex(
    "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
)

# Create verifier callback
public_key = Ed25519PublicKey.from_public_bytes(public_key_bytes)

def verify_callback(algorithm, key_id, data, signature):
    public_key.verify(bytes(signature), bytes(data))

# Decode and verify
result = claim169.decode_encrypted_aes(
    qr_data,
    encrypt_key,
    verifier=verify_callback
)

print(f"ID: {result.claim169.id}")
print(f"Verified: {result.is_verified()}")
```

### Decrypt AES-256 (Key Size Validated)

Use `decode_encrypted_aes256()` when you want to ensure a 32-byte key:

```python
result = claim169.decode_encrypted_aes256(
    qr_data,
    encrypt_key,  # Must be exactly 32 bytes
    verifier=verify_callback
)
```

### Decrypt AES-128 (Key Size Validated)

Use `decode_encrypted_aes128()` for 16-byte keys:

```python
encrypt_key_128 = bytes(16)  # AES-128 key

result = claim169.decode_encrypted_aes128(
    qr_data,
    encrypt_key_128,  # Must be exactly 16 bytes
    verifier=verify_callback
)
```

### Decrypt Without Signature Verification

For testing only. Use `allow_unverified=True`:

```python
# WARNING: INSECURE - skips signature verification
result = claim169.decode_encrypted_aes(
    qr_data,
    encrypt_key,
    allow_unverified=True
)

print(f"Status: {result.verification_status}")  # "skipped"
```

## Custom Decryption

For HSM or KMS decryption, use a callback:

```python
import claim169
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

# Your decryption key (stored in HSM/KMS)
aes_key = bytes(32)

def my_decryptor(algorithm, key_id, nonce, aad, ciphertext):
    """Custom decryption callback.

    Args:
        algorithm: Algorithm name (e.g., "A256GCM", "A128GCM")
        key_id: Optional key identifier from COSE header
        nonce: 12-byte nonce
        aad: Additional authenticated data
        ciphertext: Encrypted data with authentication tag

    Returns:
        Decrypted plaintext bytes
    """
    aesgcm = AESGCM(aes_key)
    return aesgcm.decrypt(bytes(nonce), bytes(ciphertext), bytes(aad))

def my_verifier(algorithm, key_id, data, signature):
    # Your verification logic
    pass

result = claim169.decode_with_decryptor(
    qr_data,
    my_decryptor,
    verifier=my_verifier
)
```

## Custom Encryption

For HSM or KMS encryption during encoding:

```python
import claim169
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

aes_key = bytes(32)

def my_encryptor(algorithm, key_id, nonce, aad, plaintext):
    """Custom encryption callback.

    Args:
        algorithm: Algorithm name (e.g., "A256GCM", "A128GCM")
        key_id: Optional key identifier
        nonce: 12-byte nonce
        aad: Additional authenticated data
        plaintext: Data to encrypt

    Returns:
        Ciphertext with authentication tag
    """
    aesgcm = AESGCM(aes_key)
    return aesgcm.encrypt(bytes(nonce), bytes(plaintext), bytes(aad))

# Software signing with custom encryption
sign_key = bytes(32)  # Ed25519 private key

qr_data = claim169.encode_with_encryptor(
    claim,
    meta,
    sign_key,
    my_encryptor,
    "A256GCM"  # or "A128GCM"
)
```

## Full Custom Crypto

Use custom callbacks for both signing and encryption:

```python
import claim169
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

# Generate keys
sign_private = Ed25519PrivateKey.generate()
sign_public_bytes = sign_private.public_key().public_bytes_raw()
aes_key = bytes(32)

def my_signer(algorithm, key_id, data):
    return sign_private.sign(bytes(data))

def my_encryptor(algorithm, key_id, nonce, aad, plaintext):
    return AESGCM(aes_key).encrypt(bytes(nonce), bytes(plaintext), bytes(aad))

# Encode with both custom callbacks
claim = claim169.Claim169Input(id="CUSTOM-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(issuer="https://id.example.org")

qr_data = claim169.encode_with_signer_and_encryptor(
    claim,
    meta,
    my_signer,
    "EdDSA",      # Sign algorithm
    my_encryptor,
    "A256GCM"     # Encrypt algorithm
)

# Decode with custom callbacks
def my_verifier(algorithm, key_id, data, signature):
    sign_private.public_key().verify(bytes(signature), bytes(data))

def my_decryptor(algorithm, key_id, nonce, aad, ciphertext):
    return AESGCM(aes_key).decrypt(bytes(nonce), bytes(ciphertext), bytes(aad))

result = claim169.decode_with_decryptor(
    qr_data,
    my_decryptor,
    verifier=my_verifier
)

print(f"ID: {result.claim169.id}")
print(f"Verified: {result.is_verified()}")
```

## Generating Keys

### AES-256 Key

```python
import secrets

aes_256_key = secrets.token_bytes(32)
print(f"AES-256 key: {aes_256_key.hex()}")
```

### AES-128 Key

```python
import secrets

aes_128_key = secrets.token_bytes(16)
print(f"AES-128 key: {aes_128_key.hex()}")
```

### Random Nonce

The library generates nonces automatically, but you can also generate them:

```python
import claim169

nonce = claim169.generate_nonce()
print(f"Nonce: {bytes(nonce).hex()}")  # 12 bytes
```

## Error Handling

```python
import claim169

try:
    result = claim169.decode_encrypted_aes256(qr_data, encrypt_key, allow_unverified=True)

except claim169.DecryptionError as e:
    # Decryption failed (wrong key, corrupted data, etc.)
    print(f"Decryption failed: {e}")

except ValueError as e:
    # Invalid key size
    print(f"Invalid key: {e}")

except claim169.Claim169Exception as e:
    # Other errors
    print(f"Error: {e}")
```

## Security Best Practices

### Key Management

- **Never hardcode keys** in source code
- **Use secure storage** (HSM, KMS, secret manager)
- **Rotate keys** periodically
- **Limit key access** to authorized systems only

### Nonce Requirements

- **Never reuse nonces** with the same key
- The library generates random nonces automatically
- For custom encryption, always use cryptographically secure random nonces

### Key Distribution

- Distribute encryption keys through secure channels
- Consider using key derivation functions for shared secrets
- Implement proper key exchange protocols for distributed systems

## Next Steps

- [Custom Crypto](custom-crypto.md) — HSM/KMS integration examples
- [API Reference](api.md) — Complete function documentation
