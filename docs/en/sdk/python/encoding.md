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

# Create with all fields as keyword arguments
claim = claim169.Claim169Input(
    id="MOSIP-2024-001",
    full_name="Jane Doe",
    version="1.0.0",
    language="en",
    first_name="Jane",
    middle_name="Marie",
    last_name="Doe",
    date_of_birth="1990-05-15",
    gender=claim169.Gender.FEMALE,
    address="123 Main Street, Springfield, IL 62701",
    email="jane.doe@example.org",
    phone="+1-555-123-4567",
    nationality="US",
    marital_status=claim169.MaritalStatus.UNMARRIED,
    guardian="John Doe Sr.",
    secondary_full_name="Jane Marie Doe",
    secondary_language="es",
    location_code="US-IL",
    legal_status="citizen",
    country_of_issuance="US",
)
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
| `gender` | `int` | `Gender.MALE`, `Gender.FEMALE`, `Gender.OTHER` |
| `address` | `str` | Full address |
| `email` | `str` | Email address |
| `phone` | `str` | Phone number |
| `nationality` | `str` | Nationality code |
| `marital_status` | `int` | `MaritalStatus.UNMARRIED`, `MaritalStatus.MARRIED`, `MaritalStatus.DIVORCED` |
| `guardian` | `str` | Guardian name |
| `photo` | `bytes` | Photo data |
| `photo_format` | `int` | `PhotoFormat.JPEG`, `PhotoFormat.JPEG2000`, `PhotoFormat.AVIF`, `PhotoFormat.WEBP` |
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

claim = claim169.Claim169Input(
    id="PHOTO-001",
    full_name="Jane Doe",
    photo=photo_data,
    photo_format=claim169.PhotoFormat.JPEG,
)
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
claim = claim169.Claim169Input(
    id="ED25519-001",
    full_name="Jane Doe",
    date_of_birth="1990-05-15",
)

# Token metadata
meta = claim169.CwtMetaInput(
    issuer="https://id.example.org",
    expires_at=1900000000,
    issued_at=1700000000,
)

# Ed25519 private key (32 bytes)
private_key = bytes.fromhex(
    "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
)

# Encode using the unified encode() function
qr_data = claim169.encode(claim, meta, sign_with_ed25519=private_key)
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

qr_data = claim169.encode(claim, meta, sign_with_ecdsa_p256=private_key)
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
qr_data = claim169.encode(claim, meta, allow_unsigned=True)
```

## Including Biometrics

All 16 biometric fields can be set on `Claim169Input`. Each field accepts a list of `Biometric` objects:

```python
import claim169

# Create a biometric entry
face_biometric = claim169.Biometric(
    data=face_image_bytes,
    format=1,   # Image format
    sub_format=2,
    issuer="https://biometrics.example.org",
)

# Include biometrics in the credential
claim = claim169.Claim169Input(
    id="BIO-001",
    full_name="Jane Doe",
    face=[face_biometric],
    right_thumb=[claim169.Biometric(data=thumb_bytes, format=1)],
    left_iris=[claim169.Biometric(data=iris_bytes, format=1)],
)

qr_data = claim169.encode(claim, meta, sign_with_ed25519=private_key)
```

### Biometric Fields

| Field | Description |
|-------|-------------|
| `right_thumb` | Right thumb fingerprint |
| `right_pointer_finger` | Right pointer/index finger |
| `right_middle_finger` | Right middle finger |
| `right_ring_finger` | Right ring finger |
| `right_little_finger` | Right little/pinky finger |
| `left_thumb` | Left thumb fingerprint |
| `left_pointer_finger` | Left pointer/index finger |
| `left_middle_finger` | Left middle finger |
| `left_ring_finger` | Left ring finger |
| `left_little_finger` | Left little/pinky finger |
| `right_iris` | Right iris scan |
| `left_iris` | Left iris scan |
| `face` | Face image |
| `right_palm` | Right palm print |
| `left_palm` | Left palm print |
| `voice` | Voice sample |

### Biometric Object

Each `Biometric` has the following fields:

| Field | Type | Description |
|-------|------|-------------|
| `data` | `bytes` | Raw biometric data (required) |
| `format` | `int` | Biometric data format |
| `sub_format` | `int` | Biometric sub-format |
| `issuer` | `str` | Issuing authority |

## Skipping Biometrics

To reduce QR code size, skip encoding biometric data:

```python
qr_data = claim169.encode(
    claim,
    meta,
    sign_with_ed25519=private_key,
    skip_biometrics=True,
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
    full_name="Jane Marie Doe",
    version="1.0.0",
    language="en",
    first_name="Jane",
    middle_name="Marie",
    last_name="Doe",
    date_of_birth="1990-05-15",
    gender=claim169.Gender.FEMALE,
    address="123 Main Street, Springfield, IL 62701, USA",
    email="jane.doe@example.org",
    phone="+1-555-123-4567",
    nationality="US",
    marital_status=claim169.MaritalStatus.MARRIED,
    secondary_full_name="Juana Maria Doe",
    secondary_language="es",
    location_code="US-IL-SPR",
    legal_status="citizen",
    country_of_issuance="US",
)

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

qr_data = claim169.encode(claim, meta, sign_with_ed25519=private_key)

print(f"QR Code content ({len(qr_data)} characters)")
print(f"Ready for QR code generation")
```

## Error Handling

```python
import claim169

try:
    qr_data = claim169.encode(claim, meta, sign_with_ed25519=private_key)
except ValueError as e:
    print(f"Invalid key format: {e}")
except claim169.Claim169Exception as e:
    print(f"Encoding failed: {e}")
```

## Next Steps

- [Encryption](encryption.md) — Add AES-GCM encryption
- [Custom Crypto](custom-crypto.md) — Use HSM/KMS for signing
- [API Reference](api.md) — Complete function documentation
