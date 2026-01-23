# Encoding Credentials

This guide covers creating signed identity credentials that can be encoded in QR codes.

## Overview

Encoding follows these steps:

1. Create a `Claim169Input` with identity data
2. Create a `CwtMetaInput` with token metadata
3. Sign with a private key
4. Optionally encrypt with a symmetric key
5. Receive a Base45-encoded string for QR code generation

## Creating Identity Data

### Claim169Input

The `Claim169Input` class holds all identity fields:

```python
import claim169

# Create with required fields
claim = claim169.Claim169Input(
    id="MOSIP-2024-001",
    full_name="Jane Doe"
)

# Set additional demographics
claim.version = "1.0.0"
claim.language = "en"
claim.first_name = "Jane"
claim.middle_name = "Marie"
claim.last_name = "Doe"
claim.date_of_birth = "1990-05-15"
claim.gender = 2  # 1=Male, 2=Female, 3=Other
claim.address = "123 Main Street, Springfield, IL 62701"
claim.email = "jane.doe@example.org"
claim.phone = "+1-555-123-4567"
claim.nationality = "US"
claim.marital_status = 1  # 1=Unmarried, 2=Married, 3=Divorced

# Optional fields
claim.guardian = "John Doe Sr."
claim.secondary_full_name = "Jane Marie Doe"
claim.secondary_language = "es"
claim.location_code = "US-IL"
claim.legal_status = "citizen"
claim.country_of_issuance = "US"
```

### Field Reference

| Field | Type | Description |
|-------|------|-------------|
| `id` | `str` | Unique identifier |
| `version` | `str` | Credential version |
| `language` | `str` | Primary language code (ISO 639-1) |
| `full_name` | `str` | Full name |
| `first_name` | `str` | First/given name |
| `middle_name` | `str` | Middle name |
| `last_name` | `str` | Last/family name |
| `date_of_birth` | `str` | Date of birth (YYYY-MM-DD) |
| `gender` | `int` | 1=Male, 2=Female, 3=Other |
| `address` | `str` | Full address |
| `email` | `str` | Email address |
| `phone` | `str` | Phone number |
| `nationality` | `str` | Nationality code |
| `marital_status` | `int` | 1=Unmarried, 2=Married, 3=Divorced |
| `guardian` | `str` | Guardian name |
| `photo` | `bytes` | Photo data |
| `photo_format` | `int` | 1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP |
| `secondary_full_name` | `str` | Name in secondary language |
| `secondary_language` | `str` | Secondary language code |
| `location_code` | `str` | Location code |
| `legal_status` | `str` | Legal status |
| `country_of_issuance` | `str` | Issuing country code |

### Including a Photo

```python
# Read photo file
with open("photo.jpg", "rb") as f:
    photo_data = f.read()

claim = claim169.Claim169Input(id="PHOTO-001", full_name="Jane Doe")
claim.photo = photo_data
claim.photo_format = 1  # JPEG
```

## Creating Token Metadata

### CwtMetaInput

The `CwtMetaInput` class holds CWT (CBOR Web Token) metadata:

```python
import time

meta = claim169.CwtMetaInput(
    issuer="https://id.example.org",
    expires_at=int(time.time()) + (365 * 24 * 60 * 60)  # 1 year from now
)
meta.subject = "user-12345"
meta.issued_at = int(time.time())
meta.not_before = int(time.time())  # Valid immediately
```

### Metadata Fields

| Field | Type | Description |
|-------|------|-------------|
| `issuer` | `str` | Credential issuer (URL or identifier) |
| `subject` | `str` | Subject identifier |
| `expires_at` | `int` | Expiration timestamp (Unix epoch) |
| `not_before` | `int` | Not valid before timestamp |
| `issued_at` | `int` | Issuance timestamp |

## Signing with Ed25519

Ed25519 is recommended for its small signatures and fast verification.

```python
import claim169

# Identity data
claim = claim169.Claim169Input(id="ED25519-001", full_name="Jane Doe")
claim.date_of_birth = "1990-05-15"

# Token metadata
meta = claim169.CwtMetaInput(
    issuer="https://id.example.org",
    expires_at=1900000000
)
meta.issued_at = 1700000000

# Ed25519 private key (32 bytes)
private_key = bytes.fromhex(
    "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
)

# Encode
qr_data = claim169.encode_with_ed25519(claim, meta, private_key)
print(f"Encoded: {len(qr_data)} characters")
```

### Generating Ed25519 Keys

```python
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey

# Generate a new key pair
private_key_obj = Ed25519PrivateKey.generate()

# Get raw bytes
private_key = private_key_obj.private_bytes_raw()  # 32 bytes
public_key = private_key_obj.public_key().public_bytes_raw()  # 32 bytes

print(f"Private key: {private_key.hex()}")
print(f"Public key: {public_key.hex()}")
```

## Signing with ECDSA P-256

ECDSA P-256 is widely supported in enterprise environments.

```python
import claim169

claim = claim169.Claim169Input(id="ECDSA-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(
    issuer="https://id.example.org",
    expires_at=1900000000
)

# ECDSA P-256 private key (32 bytes)
private_key = bytes(32)  # Replace with actual key

qr_data = claim169.encode_with_ecdsa_p256(claim, meta, private_key)
```

### Generating ECDSA P-256 Keys

```python
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.backends import default_backend

# Generate key pair
private_key_obj = ec.generate_private_key(ec.SECP256R1(), default_backend())

# Get raw private key bytes
private_key = private_key_obj.private_numbers().private_value.to_bytes(32, 'big')

# Get public key in SEC1 format
public_numbers = private_key_obj.public_key().public_numbers()
public_key = (
    b'\x04' +
    public_numbers.x.to_bytes(32, 'big') +
    public_numbers.y.to_bytes(32, 'big')
)

print(f"Private key: {private_key.hex()}")
print(f"Public key: {public_key.hex()}")
```

## Encoding Without Signature

For testing and development only. Never use in production.

```python
import claim169

claim = claim169.Claim169Input(id="TEST-001", full_name="Test User")
meta = claim169.CwtMetaInput(expires_at=1900000000)

# Encode without signature (INSECURE - testing only)
qr_data = claim169.encode_unsigned(claim, meta)
```

## Skipping Biometrics

To reduce QR code size, skip encoding biometric data:

```python
qr_data = claim169.encode_with_ed25519(
    claim,
    meta,
    private_key,
    skip_biometrics=True
)
```

## Full Example

Complete example with all demographics:

```python
import claim169
import time

# Create comprehensive identity data
claim = claim169.Claim169Input(
    id="FULL-DEMO-2024-001",
    full_name="Jane Marie Doe"
)
claim.version = "1.0.0"
claim.language = "en"
claim.first_name = "Jane"
claim.middle_name = "Marie"
claim.last_name = "Doe"
claim.date_of_birth = "1990-05-15"
claim.gender = 2
claim.address = "123 Main Street, Springfield, IL 62701, USA"
claim.email = "jane.doe@example.org"
claim.phone = "+1-555-123-4567"
claim.nationality = "US"
claim.marital_status = 2
claim.secondary_full_name = "Juana Maria Doe"
claim.secondary_language = "es"
claim.location_code = "US-IL-SPR"
claim.legal_status = "citizen"
claim.country_of_issuance = "US"

# Create token metadata
now = int(time.time())
meta = claim169.CwtMetaInput(
    issuer="https://id.state.il.us",
    expires_at=now + (5 * 365 * 24 * 60 * 60)  # 5 years
)
meta.subject = "IL-DL-2024-001"
meta.issued_at = now
meta.not_before = now

# Sign with Ed25519
private_key = bytes.fromhex(
    "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
)

qr_data = claim169.encode_with_ed25519(claim, meta, private_key)

print(f"QR Code content ({len(qr_data)} characters)")
print(f"Ready for QR code generation")
```

## Error Handling

```python
import claim169

try:
    qr_data = claim169.encode_with_ed25519(claim, meta, private_key)
except ValueError as e:
    print(f"Invalid key format: {e}")
except claim169.Claim169Exception as e:
    print(f"Encoding failed: {e}")
```

## Next Steps

- [Encryption](encryption.md) — Add AES-GCM encryption
- [Custom Crypto](custom-crypto.md) — Use HSM/KMS for signing
- [API Reference](api.md) — Complete function documentation
