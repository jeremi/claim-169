# Decoding Credentials

This guide covers decoding and verifying identity credentials from QR codes.

## Overview

Decoding follows these steps:

1. Receive Base45-encoded string from QR scanner
2. Choose verification method (Ed25519, ECDSA P-256, or custom)
3. Call the appropriate decode function
4. Access the decoded claim and metadata

## Decoding with Ed25519 Verification

The most common case using Ed25519 signatures:

```python
import claim169

qr_data = "NCFOXN..."  # From QR scanner
public_key = bytes.fromhex(
    "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
)

result = claim169.decode_with_ed25519(qr_data, public_key)

# Access identity data
print(f"ID: {result.claim169.id}")
print(f"Name: {result.claim169.full_name}")
print(f"DOB: {result.claim169.date_of_birth}")
print(f"Gender: {result.claim169.gender}")

# Verification status
print(f"Verified: {result.is_verified()}")
print(f"Status: {result.verification_status}")
```

### Function Signature

```python
def decode_with_ed25519(
    qr_text: str,
    public_key: bytes,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0
) -> DecodeResult
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `qr_text` | `str` | required | Base45-encoded QR content |
| `public_key` | `bytes` | required | 32-byte Ed25519 public key |
| `skip_biometrics` | `bool` | `False` | Skip parsing biometric data |
| `max_decompressed_bytes` | `int` | `65536` | Maximum decompressed size |
| `validate_timestamps` | `bool` | `True` | Validate exp/nbf timestamps |
| `clock_skew_tolerance_seconds` | `int` | `0` | Tolerance for clock differences |

## Decoding with ECDSA P-256 Verification

For credentials signed with ECDSA P-256:

```python
import claim169

qr_data = "NCFOXN..."
# SEC1 encoded P-256 public key (33 bytes compressed, or 65 bytes uncompressed)
public_key = bytes.fromhex("04...")

result = claim169.decode_with_ecdsa_p256(qr_data, public_key)

print(f"ID: {result.claim169.id}")
print(f"Verified: {result.is_verified()}")
```

## Decoding with Custom Verifier

For HSM, KMS, or custom crypto providers:

```python
import claim169
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PublicKey

# Load your public key
public_key_bytes = bytes.fromhex("d75a980182b10ab7...")
public_key = Ed25519PublicKey.from_public_bytes(public_key_bytes)

def my_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes):
    """Custom verification callback.

    Args:
        algorithm: Algorithm name (e.g., "EdDSA", "ES256")
        key_id: Optional key identifier from COSE header
        data: Data that was signed
        signature: Signature to verify

    Raises:
        Any exception if verification fails
    """
    # Verify using your crypto provider
    public_key.verify(bytes(signature), bytes(data))

result = claim169.decode_with_verifier(qr_data, my_verifier)
print(f"Verified: {result.is_verified()}")
```

## Decoding Without Verification

For testing and development only. Never use in production.

```python
import claim169

# WARNING: INSECURE - skips signature verification
result = claim169.decode_unverified(qr_data)

print(f"ID: {result.claim169.id}")
print(f"Status: {result.verification_status}")  # "skipped"
```

### Options

```python
result = claim169.decode_unverified(
    qr_data,
    skip_biometrics=False,
    max_decompressed_bytes=65536,
    validate_timestamps=True,
    clock_skew_tolerance_seconds=0
)
```

## Accessing Decoded Data

### DecodeResult

The decode functions return a `DecodeResult` object:

```python
result = claim169.decode_with_ed25519(qr_data, public_key)

# The decoded identity claim
claim = result.claim169

# CWT metadata (issuer, timestamps)
meta = result.cwt_meta

# Verification status string
status = result.verification_status  # "verified", "skipped", etc.

# Helper method
is_verified = result.is_verified()  # True/False
```

### Claim169 Fields

```python
claim = result.claim169

# Demographics
claim.id                    # str | None
claim.version               # str | None
claim.language              # str | None
claim.full_name             # str | None
claim.first_name            # str | None
claim.middle_name           # str | None
claim.last_name             # str | None
claim.date_of_birth         # str | None
claim.gender                # int | None (1=Male, 2=Female, 3=Other)
claim.address               # str | None
claim.email                 # str | None
claim.phone                 # str | None
claim.nationality           # str | None
claim.marital_status        # int | None (1=Unmarried, 2=Married, 3=Divorced)
claim.guardian              # str | None
claim.photo                 # bytes | None
claim.photo_format          # int | None (1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP)
claim.secondary_full_name   # str | None
claim.secondary_language    # str | None
claim.location_code         # str | None
claim.legal_status          # str | None
claim.country_of_issuance   # str | None

# Biometrics (each is list[Biometric] | None)
claim.right_thumb
claim.right_pointer_finger
claim.right_middle_finger
claim.right_ring_finger
claim.right_little_finger
claim.left_thumb
claim.left_pointer_finger
claim.left_middle_finger
claim.left_ring_finger
claim.left_little_finger
claim.right_iris
claim.left_iris
claim.face
claim.right_palm
claim.left_palm
claim.voice

# Helper methods
claim.has_biometrics()  # True if any biometric data present
claim.to_dict()         # Convert to Python dictionary
```

### CwtMeta Fields

```python
meta = result.cwt_meta

meta.issuer       # str | None - Credential issuer
meta.subject      # str | None - Subject identifier
meta.expires_at   # int | None - Expiration timestamp
meta.not_before   # int | None - Not valid before timestamp
meta.issued_at    # int | None - Issuance timestamp

# Helper methods
meta.is_valid_now()  # True if token is currently valid
meta.is_expired()    # True if token has expired
```

### Biometric Fields

```python
if result.claim169.face:
    face = result.claim169.face[0]

    face.data       # bytes - Raw biometric data
    face.format     # int | None - Format code
    face.sub_format # int | None - Sub-format code
    face.issuer     # str | None - Biometric issuer
```

## Handling Timestamps

### Timestamp Validation

By default, the decoder validates timestamps:

```python
# This will raise an exception if the token is expired or not yet valid
result = claim169.decode_with_ed25519(qr_data, public_key)
```

### Disabling Timestamp Validation

```python
result = claim169.decode_with_ed25519(
    qr_data,
    public_key,
    validate_timestamps=False
)
```

### Clock Skew Tolerance

For distributed systems with clock differences:

```python
result = claim169.decode_with_ed25519(
    qr_data,
    public_key,
    clock_skew_tolerance_seconds=60  # Allow 60 seconds of drift
)
```

## Optimizing Decoding

### Skip Biometrics

For faster decoding when biometrics aren't needed:

```python
result = claim169.decode_with_ed25519(
    qr_data,
    public_key,
    skip_biometrics=True
)

# Biometric fields will be None
assert result.claim169.face is None
```

### Limit Decompressed Size

Protect against decompression bombs:

```python
result = claim169.decode_with_ed25519(
    qr_data,
    public_key,
    max_decompressed_bytes=32768  # 32 KB limit
)
```

## Error Handling

```python
import claim169

try:
    result = claim169.decode_with_ed25519(qr_data, public_key)

except claim169.Base45DecodeError as e:
    # Invalid Base45 encoding
    print(f"QR code format error: {e}")

except claim169.DecompressError as e:
    # Zlib decompression failed or size limit exceeded
    print(f"Decompression error: {e}")

except claim169.CoseParseError as e:
    # Invalid COSE structure
    print(f"COSE parse error: {e}")

except claim169.CwtParseError as e:
    # Invalid CWT structure
    print(f"CWT parse error: {e}")

except claim169.Claim169NotFoundError as e:
    # Claim 169 not present in CWT
    print(f"Not a Claim 169 credential: {e}")

except claim169.SignatureError as e:
    # Signature verification failed
    print(f"Invalid signature: {e}")

except claim169.Claim169Exception as e:
    # Generic error (base class)
    print(f"Decoding failed: {e}")
```

## Complete Example

```python
import claim169

def verify_credential(qr_data: str, public_key: bytes) -> dict | None:
    """Verify and decode a Claim 169 credential.

    Returns:
        Decoded claim as dictionary, or None if verification fails.
    """
    try:
        result = claim169.decode_with_ed25519(
            qr_data,
            public_key,
            clock_skew_tolerance_seconds=60
        )

        if not result.is_verified():
            print(f"Warning: {result.verification_status}")
            return None

        if result.cwt_meta.is_expired():
            print("Credential has expired")
            return None

        return {
            "id": result.claim169.id,
            "full_name": result.claim169.full_name,
            "date_of_birth": result.claim169.date_of_birth,
            "issuer": result.cwt_meta.issuer,
            "expires_at": result.cwt_meta.expires_at,
            "has_photo": result.claim169.photo is not None,
            "has_biometrics": result.claim169.has_biometrics(),
        }

    except claim169.SignatureError:
        print("Invalid signature - credential may be tampered")
        return None

    except claim169.Claim169Exception as e:
        print(f"Failed to decode credential: {e}")
        return None


# Usage
public_key = bytes.fromhex("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
qr_data = "NCFOXN..."

credential = verify_credential(qr_data, public_key)
if credential:
    print(f"Verified: {credential['full_name']}")
```

## Next Steps

- [Encryption](encryption.md) — Decode encrypted credentials
- [Custom Crypto](custom-crypto.md) — HSM/KMS integration
- [API Reference](api.md) — Complete function documentation
