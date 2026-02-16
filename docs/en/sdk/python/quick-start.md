# Quick Start

This guide covers the essential operations: encoding credentials and decoding QR codes.

## Decoding a QR Code

The most common operation is decoding a QR code that was scanned from an identity credential.

!!! warning "Do not trim Base45"
    The Base45 alphabet includes a literal space character (`" "`). Preserve the scanned QR text exactly as-is (no `.strip()`, `.trim()`, or whitespace normalization), or you can corrupt valid credentials.

### With Ed25519 Verification

```python
import claim169

# QR code content (Base45 encoded string from scanner)
qr_data = "NCFOXN..."

# Issuer's Ed25519 public key (32 bytes)
public_key = bytes.fromhex(
    "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
)

# Decode and verify
result = claim169.decode_with_ed25519(qr_data, public_key)

# Access identity data
print(f"ID: {result.claim169.id}")
print(f"Name: {result.claim169.full_name}")
print(f"Date of Birth: {result.claim169.date_of_birth}")

# Check verification status
if result.is_verified():
    print("Signature verified successfully")
else:
    print(f"Verification status: {result.verification_status}")

# Access CWT metadata
print(f"Issuer: {result.cwt_meta.issuer}")
print(f"Expires: {result.cwt_meta.expires_at}")
```

### Checking Token Validity

```python
# Check if the token is currently valid (not expired, not before nbf)
if result.cwt_meta.is_valid_now():
    print("Token is valid")
else:
    print("Token has expired or is not yet valid")

# Check expiration specifically
if result.cwt_meta.is_expired():
    print("Token has expired")
```

## Encoding a Credential

Create a signed credential that can be encoded in a QR code.

### Basic Encoding with Ed25519

```python
import claim169

# Create identity data
claim = claim169.Claim169Input(
    id="MOSIP-2024-001",
    full_name="Jane Doe",
    date_of_birth="1990-05-15",
    gender=claim169.Gender.FEMALE,
    email="jane.doe@example.org",
    nationality="US",
)

# Create CWT metadata
meta = claim169.CwtMetaInput(
    issuer="https://id.example.org",
    expires_at=1900000000,  # Unix timestamp
    issued_at=1700000000,
)

# Ed25519 private key (32 bytes) - keep this secret!
private_key = bytes.fromhex(
    "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
)

# Encode the credential
qr_data = claim169.encode(claim, meta, sign_with_ed25519=private_key)

print(f"QR Code content ({len(qr_data)} chars):")
print(qr_data)
```

### Roundtrip Example

Encode a credential and immediately decode it to verify:

```python
import claim169

# Keys
private_key = bytes.fromhex(
    "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
)
public_key = bytes.fromhex(
    "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
)

# Create and encode
claim = claim169.Claim169Input(id="TEST-001", full_name="Test User")
meta = claim169.CwtMetaInput(
    issuer="https://test.example.org",
    expires_at=1900000000
)

qr_data = claim169.encode(claim, meta, sign_with_ed25519=private_key)

# Decode and verify
result = claim169.decode_with_ed25519(qr_data, public_key)

assert result.claim169.id == "TEST-001"
assert result.claim169.full_name == "Test User"
assert result.is_verified()
print("Roundtrip successful!")
```

## Working with Biometrics

### Checking for Biometric Data

```python
result = claim169.decode_with_ed25519(qr_data, public_key)

if result.claim169.has_biometrics():
    print("Credential contains biometric data")

    # Check specific biometric types
    if result.claim169.face:
        face = result.claim169.face[0]
        print(f"Face photo: {len(face.data)} bytes, format={face.format}")

    if result.claim169.right_thumb:
        thumb = result.claim169.right_thumb[0]
        print(f"Right thumb: {len(thumb.data)} bytes")
```

### Skipping Biometrics (Faster Decoding)

For use cases that don't need biometric data:

```python
# Skip biometric parsing for faster decoding
result = claim169.decode_with_ed25519(
    qr_data,
    public_key,
    skip_biometrics=True
)

# Biometric fields will be None
assert result.claim169.face is None
```

## Error Handling

```python
import claim169

try:
    result = claim169.decode_with_ed25519(qr_data, public_key)
except claim169.Base45DecodeError as e:
    print(f"Invalid QR code format: {e}")
except claim169.DecompressError as e:
    print(f"Decompression failed: {e}")
except claim169.CoseParseError as e:
    print(f"Invalid COSE structure: {e}")
except claim169.SignatureError as e:
    print(f"Signature verification failed: {e}")
except claim169.Claim169NotFoundError as e:
    print(f"Not a Claim 169 credential: {e}")
except claim169.Claim169Exception as e:
    print(f"Decoding failed: {e}")
```

## Converting to Dictionary

```python
result = claim169.decode_with_ed25519(qr_data, public_key)

# Convert claim to a Python dictionary
claim_dict = result.claim169.to_dict()
print(claim_dict)
# {'id': 'TEST-001', 'fullName': 'Test User', ...}
```

## Next Steps

- [Encoding Guide](encoding.md) — Detailed encoding with all demographics
- [Decoding Guide](decoding.md) — Advanced decoding options
- [Encryption](encryption.md) — Add AES-GCM encryption
- [Custom Crypto](custom-crypto.md) — HSM/KMS integration
