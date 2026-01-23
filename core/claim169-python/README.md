# claim169

> **Alpha Software**: This library is under active development. APIs may change without notice. Not recommended for production use without thorough testing.

[![PyPI](https://img.shields.io/pypi/v/claim169.svg)](https://pypi.org/project/claim169/)
[![Python](https://img.shields.io/pypi/pyversions/claim169.svg)](https://pypi.org/project/claim169/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Python library for encoding and decoding MOSIP Claim 169 QR codes. Built on Rust for performance and security.

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

# Decode a QR code (recommended: with signature verification)
qr_text = "6BF5YZB2..."  # Base45-encoded QR content
result = claim169.decode(qr_text, verify_with_ed25519=public_key_bytes)

# Access identity data
print(f"ID: {result.claim169.id}")
print(f"Name: {result.claim169.full_name}")
print(f"DOB: {result.claim169.date_of_birth}")

# Access CWT metadata
print(f"Issuer: {result.cwt_meta.issuer}")
print(f"Expires: {result.cwt_meta.expires_at}")
```

## Encoding

Create Claim 169 QR code data with various signing and encryption options.
In production, keys are typically provisioned and managed externally (HSM/KMS or secure key management). The examples below assume you already have key bytes.

```python
from claim169 import Claim169Input, CwtMetaInput, encode_with_ed25519, encode_signed_encrypted

# Create identity data
claim = Claim169Input(id="123456", full_name="John Doe")
claim.date_of_birth = "1990-01-15"
claim.email = "john@example.com"

# Create CWT metadata
meta = CwtMetaInput(issuer="https://issuer.example.com", expires_at=1800000000)

# Encode with Ed25519 signature (32-byte private key)
qr_data = encode_with_ed25519(claim, meta, private_key_bytes)

# Encode with signature and AES-256 encryption
qr_data = encode_signed_encrypted(claim, meta, sign_key_bytes, encrypt_key_bytes)
```

### Encoding Functions

```python
from claim169 import (
    encode_with_ed25519,           # Ed25519 signed
    encode_with_ecdsa_p256,        # ECDSA P-256 signed
    encode_signed_encrypted,       # Signed + AES-256-GCM encrypted
    encode_signed_encrypted_aes128,# Signed + AES-128-GCM encrypted
    encode_with_signer,            # Custom signer (HSM, cloud KMS, etc.)
    encode_with_signer_and_encryptor,  # Custom signer + encryptor
    encode_unsigned,               # Unsigned (testing only)
    generate_nonce,                # Generate random 12-byte nonce
)

# Ed25519 signed (recommended)
qr_data = encode_with_ed25519(claim, cwt_meta, private_key)  # 32-byte key

# ECDSA P-256 signed
qr_data = encode_with_ecdsa_p256(claim, cwt_meta, private_key)  # 32-byte key

# Signed and encrypted (AES-256-GCM)
qr_data = encode_signed_encrypted(claim, cwt_meta, sign_key, encrypt_key)

# Signed and encrypted (AES-128-GCM)
qr_data = encode_signed_encrypted_aes128(claim, cwt_meta, sign_key, encrypt_key)

# Unsigned (for testing only - not recommended for production)
qr_data = encode_unsigned(claim, cwt_meta)

# Generate a random 12-byte nonce for encryption
nonce = generate_nonce()
```

### Custom Signer (HSM/Cloud KMS)

For integrating with external crypto providers like HSMs, cloud KMS (AWS KMS, Google Cloud KMS, Azure Key Vault), smart cards, TPMs, or remote signing services:

```python
from claim169 import encode_with_signer, encode_with_signer_and_encryptor

def my_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
    """Custom signer callback.

    Args:
        algorithm: COSE algorithm name ("EdDSA" or "ES256")
        key_id: Optional key identifier
        data: The data to sign (COSE Sig_structure)

    Returns:
        Signature bytes (64 bytes for EdDSA, 64 bytes for ES256)
    """
    return my_hsm.sign(key_id, data)

# Sign with custom signer
qr_data = encode_with_signer(claim, meta, my_signer, "EdDSA")

# Sign with custom signer and key_id
qr_data = encode_with_signer(claim, meta, my_signer, "EdDSA", key_id=b"my-key-123")

# Sign with custom signer (ES256)
qr_data = encode_with_signer(claim, meta, my_signer, "ES256")
```

### Custom Encryptor (HSM/Cloud KMS)

```python
def my_encryptor(algorithm: str, key_id: bytes | None, nonce: bytes, aad: bytes, plaintext: bytes) -> bytes:
    """Custom encryptor callback.

    Args:
        algorithm: COSE algorithm name ("A256GCM" or "A128GCM")
        key_id: Optional key identifier
        nonce: 12-byte IV for AES-GCM
        aad: Additional authenticated data
        plaintext: Data to encrypt

    Returns:
        Ciphertext with authentication tag appended
    """
    return my_hsm.encrypt(key_id, nonce, aad, plaintext)

# Sign with custom signer and encrypt with custom encryptor
qr_data = encode_with_signer_and_encryptor(
    claim, meta,
    my_signer, "EdDSA",
    my_encryptor, "A256GCM"
)
```

## Signature Verification

### Ed25519 Verification (Recommended)

```python
# Decode with Ed25519 signature verification
public_key = bytes.fromhex("d75a980182b10ab7...")  # 32 bytes
result = claim169.decode_with_ed25519(qr_text, public_key)

if result.is_verified():
    print("Signature is valid!")
```

### ECDSA P-256 Verification

```python
# Decode with ECDSA P-256 signature verification
public_key = bytes.fromhex("04...")  # SEC1-encoded (33 or 65 bytes)
result = claim169.decode_with_ecdsa_p256(qr_text, public_key)
```

### Custom Verifier (HSM/Cloud KMS)

For integrating with external crypto providers like HSMs, cloud KMS (AWS KMS, Google Cloud KMS, Azure Key Vault), smart cards, or TPMs:

```python
def my_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes):
    """Custom verifier callback.

    Args:
        algorithm: COSE algorithm name ("EdDSA" or "ES256")
        key_id: Optional key identifier from COSE header
        data: The signed data (COSE Sig_structure)
        signature: The signature bytes

    Raises:
        Exception: If verification fails
    """
    # Delegate to your crypto provider
    my_hsm.verify(key_id, data, signature)

result = claim169.decode_with_verifier(qr_text, my_verifier)
```

## Encrypted Payloads

### AES-GCM Decryption

```python
# Decrypt with AES-256-GCM key
aes_key = bytes.fromhex("000102030405...")  # 32 bytes for AES-256
result = claim169.decode_encrypted_aes(qr_text, aes_key, allow_unverified=True)  # testing only
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

### Custom Decryptor (HSM/Cloud KMS)

```python
def my_decryptor(algorithm: str, key_id: bytes | None, nonce: bytes, aad: bytes, ciphertext: bytes) -> bytes:
    """Custom decryptor callback.

    Args:
        algorithm: COSE algorithm name ("A256GCM" or "A128GCM")
        key_id: Optional key identifier from COSE header
        nonce: 12-byte IV from COSE header
        aad: Additional authenticated data (COSE Enc_structure)
        ciphertext: The encrypted data with auth tag

    Returns:
        Decrypted plaintext bytes
    """
    return my_hsm.decrypt(key_id, nonce, aad, ciphertext)

# Provide a verifier for the inner COSE_Sign1 (recommended)
result = claim169.decode_with_decryptor(qr_text, my_decryptor, verifier=my_verifier)
```

## Decode Options

```python
# Skip biometric data for faster parsing
result = claim169.decode(
    qr_text,
    verify_with_ed25519=public_key_bytes,
    skip_biometrics=True,
)

# Limit decompressed size (default: 64KB)
result = claim169.decode(
    qr_text,
    verify_with_ed25519=public_key_bytes,
    max_decompressed_bytes=32768,
)
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

### Claim169Input

Input class for encoding identity data into QR codes.

```python
from claim169 import Claim169Input

claim = Claim169Input(id="123456", full_name="John Doe")

# Set optional fields
claim.first_name = "John"
claim.last_name = "Doe"
claim.date_of_birth = "1990-01-15"
claim.gender = 1  # 1=Male, 2=Female, 3=Other
claim.email = "john@example.com"
claim.phone = "+1234567890"
claim.address = "123 Main St"
claim.nationality = "US"
claim.marital_status = 1
```

### CwtMetaInput

Input class for encoding CWT (CBOR Web Token) metadata.

```python
from claim169 import CwtMetaInput

meta = CwtMetaInput(issuer="https://issuer.example.com", expires_at=1800000000)

# Set optional fields
meta.subject = "user-123"
meta.not_before = 1700000000
meta.issued_at = 1700000000
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
cd core/claim169-python
uv run pytest tests/ -v
```

## License

MIT License - See LICENSE file for details.
