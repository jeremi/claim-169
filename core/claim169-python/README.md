# claim169

A Python library for decoding MOSIP Claim 169 QR codes. Built on Rust for performance and security.

## Installation

```bash
pip install claim169
```

## Overview

MOSIP Claim 169 defines a standard for encoding identity data in QR codes using:
- CBOR encoding with numeric keys for compactness
- CWT (CBOR Web Token) for standard claims
- COSE_Sign1 for digital signatures
- COSE_Encrypt0 for optional encryption
- zlib compression + Base45 encoding for QR-friendly output

## Quick Start

```python
import claim169

# Decode a QR code (without signature verification)
qr_text = "6BF5YZB2..."  # Base45-encoded QR content
result = claim169.decode(qr_text)

# Access identity data
print(f"ID: {result.claim169.id}")
print(f"Name: {result.claim169.full_name}")
print(f"DOB: {result.claim169.date_of_birth}")

# Access CWT metadata
print(f"Issuer: {result.cwt_meta.issuer}")
print(f"Expires: {result.cwt_meta.expires_at}")
```

## Signature Verification

### Ed25519 Verification

```python
# Decode with Ed25519 signature verification
public_key = bytes.fromhex("d75a980182b10ab7...")  # 32 bytes
result = claim169.decode_with_ed25519(qr_text, public_key)

if result.verification_status == "verified":
    print("Signature is valid!")
```

### ECDSA P-256 Verification

```python
# Decode with ECDSA P-256 signature verification
public_key = bytes.fromhex("04...")  # SEC1-encoded (33 or 65 bytes)
result = claim169.decode_with_ecdsa_p256(qr_text, public_key)
```

### Custom Verifier (HSM Support)

For hardware security module (HSM) integration:

```python
def my_hsm_verify(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes):
    """Custom verifier callback for HSM integration.

    Args:
        algorithm: COSE algorithm name (e.g., "EdDSA", "ES256")
        key_id: Optional key identifier from COSE header
        data: The signed data (Sig_structure)
        signature: The signature bytes

    Raises:
        Exception: If verification fails
    """
    # Delegate to your HSM
    hsm.verify(key_id, data, signature)

result = claim169.decode_with_verifier(qr_text, my_hsm_verify)
```

## Encrypted Payloads

### AES-GCM Decryption

```python
# Decrypt with AES-256-GCM key
aes_key = bytes.fromhex("000102030405...")  # 32 bytes for AES-256
result = claim169.decode_encrypted_aes(qr_text, aes_key)
```

### With Nested Signature Verification

```python
def verify_callback(algorithm, key_id, data, signature):
    public_key.verify(signature, data)

result = claim169.decode_encrypted_aes(
    qr_text,
    aes_key,
    verifier=verify_callback
)
```

### Custom Decryptor (HSM Support)

```python
def my_hsm_decrypt(algorithm: str, key_id: bytes | None, nonce: bytes, aad: bytes, ciphertext: bytes) -> bytes:
    """Custom decryptor callback for HSM integration.

    Args:
        algorithm: COSE algorithm name (e.g., "A256GCM")
        key_id: Optional key identifier from COSE header
        nonce: IV/nonce from COSE header
        aad: Additional authenticated data (Enc_structure)
        ciphertext: The encrypted data

    Returns:
        Decrypted plaintext bytes
    """
    return hsm.decrypt(key_id, nonce, aad, ciphertext)

result = claim169.decode_with_decryptor(qr_text, my_hsm_decrypt)
```

## Decode Options

```python
# Skip biometric data for faster parsing
result = claim169.decode(qr_text, skip_biometrics=True)

# Limit decompressed size (default: 64KB)
result = claim169.decode(qr_text, max_decompressed_bytes=32768)
```

## Data Model

### DecodeResult

```python
result.claim169           # Claim169 - Identity data
result.cwt_meta           # CwtMeta - Token metadata
result.verification_status  # "verified", "skipped", or "failed"

# Helper methods
result.is_verified()      # True if signature was verified
```

### Claim169

```python
claim = result.claim169

# Demographics
claim.id                  # Unique identifier
claim.full_name           # Full name
claim.first_name          # First name
claim.middle_name         # Middle name
claim.last_name           # Last name
claim.date_of_birth       # ISO 8601 format
claim.gender              # 1=Male, 2=Female, 3=Other
claim.address             # Address
claim.email               # Email address
claim.phone               # Phone number
claim.nationality         # Nationality code
claim.marital_status      # Marital status code

# Biometrics (when present)
claim.face                # List of face biometrics
claim.right_thumb         # Right thumb fingerprint
# ... (all finger/iris/palm biometrics)

# Helper methods
claim.has_biometrics()    # True if any biometric data present
claim.to_dict()           # Convert to dictionary
```

### CwtMeta

```python
meta = result.cwt_meta

meta.issuer               # Token issuer
meta.subject              # Token subject
meta.expires_at           # Expiration timestamp (Unix seconds)
meta.not_before           # Not-before timestamp
meta.issued_at            # Issued-at timestamp

# Helper methods
meta.is_valid_now()       # True if token is currently valid
meta.is_expired()         # True if token has expired
```

### Biometric

```python
bio = claim.face[0]

bio.data                  # Raw biometric data bytes
bio.format                # Biometric format code
bio.sub_format            # Sub-format code (optional)
bio.issuer                # Issuer identifier (optional)
```

## Exception Types

```python
from claim169 import (
    Claim169Exception,       # Base exception
    Base45DecodeError,       # Invalid Base45 encoding
    DecompressError,         # zlib decompression failed
    CoseParseError,          # Invalid COSE structure
    CwtParseError,           # Invalid CWT structure
    Claim169NotFoundError,   # Missing claim 169
    SignatureError,          # Signature verification failed
    DecryptionError,         # Decryption failed
)
```

## Development

### Building from Source

```bash
# Install maturin
pip install maturin

# Build and install in development mode
cd core/claim169-python
maturin develop
```

### Running Tests

```bash
cd sdks/python
uv run pytest tests/ -v
```

## License

MIT License - See LICENSE file for details.
