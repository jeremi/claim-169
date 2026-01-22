# Python API Reference

## Installation

```bash
pip install claim169
```

## Quick Reference

### Decoding Functions

```python
from claim169 import (
    decode_unverified,
    decode_with_ed25519,
    decode_with_ecdsa_p256,
    Decoder,
)
```

### Encoding Functions

```python
from claim169 import (
    encode_with_ed25519,
    encode_with_ecdsa_p256,
    Encoder,
    Claim169Input,
    CwtMetaInput,
)
```

## Decoding

### decode_unverified

Decode without signature verification (testing only).

```python
def decode_unverified(qr_data: str) -> DecodeResult
```

**Parameters:**

- `qr_data` - Base45-encoded QR string

**Returns:** `DecodeResult`

**Example:**

```python
result = decode_unverified("6BF590B20F...")
print(result.claim169.full_name)
```

### decode_with_ed25519

Decode and verify Ed25519 signature.

```python
def decode_with_ed25519(qr_data: str, public_key: bytes) -> DecodeResult
```

**Parameters:**

- `qr_data` - Base45-encoded QR string
- `public_key` - Ed25519 public key (32 bytes)

**Returns:** `DecodeResult`

**Raises:** `Claim169Error` if verification fails

**Example:**

```python
public_key = bytes.fromhex("d75a980182b10ab7...")
result = decode_with_ed25519("6BF590B20F...", public_key)
```

### decode_with_ecdsa_p256

Decode and verify ECDSA P-256 signature.

```python
def decode_with_ecdsa_p256(qr_data: str, public_key: bytes) -> DecodeResult
```

**Parameters:**

- `qr_data` - Base45-encoded QR string
- `public_key` - ECDSA P-256 public key (33 bytes compressed or 65 bytes uncompressed)

**Returns:** `DecodeResult`

### Decoder (Builder)

For advanced use cases with encryption.

```python
class Decoder:
    def __init__(self, qr_data: str) -> None
    def decrypt_with_aes256(self, key: bytes) -> Decoder
    def decrypt_with_aes128(self, key: bytes) -> Decoder
    def verify_with_ed25519(self, public_key: bytes) -> Decoder
    def verify_with_ecdsa_p256(self, public_key: bytes) -> Decoder
    def allow_unverified(self) -> Decoder
    def decode(self) -> DecodeResult
```

**Example:**

```python
result = (Decoder("6BFCA0410D...")
    .decrypt_with_aes256(encryption_key)
    .verify_with_ed25519(public_key)
    .decode())
```

## Encoding

### encode_with_ed25519

Encode and sign with Ed25519.

```python
def encode_with_ed25519(
    claim: Claim169Input,
    meta: CwtMetaInput,
    private_key: bytes
) -> str
```

**Parameters:**

- `claim` - Identity data
- `meta` - CWT metadata
- `private_key` - Ed25519 private key (32 bytes)

**Returns:** Base45-encoded string

**Example:**

```python
claim = Claim169Input(id="USER-001", full_name="Alice Smith")
meta = CwtMetaInput(issuer="https://example.com")
private_key = bytes.fromhex("9d61b19deffd5a60...")

qr_data = encode_with_ed25519(claim, meta, private_key)
```

### encode_with_ecdsa_p256

Encode and sign with ECDSA P-256.

```python
def encode_with_ecdsa_p256(
    claim: Claim169Input,
    meta: CwtMetaInput,
    private_key: bytes
) -> str
```

### Encoder (Builder)

For advanced use cases with encryption.

```python
class Encoder:
    def __init__(self, claim: Claim169Input, meta: CwtMetaInput) -> None
    def sign_with_ed25519(self, private_key: bytes) -> Encoder
    def sign_with_ecdsa_p256(self, private_key: bytes) -> Encoder
    def allow_unsigned(self) -> Encoder
    def encrypt_with_aes256(self, key: bytes) -> Encoder
    def encrypt_with_aes128(self, key: bytes) -> Encoder
    def encode(self) -> str
```

**Example:**

```python
qr_data = (Encoder(claim, meta)
    .sign_with_ed25519(signing_key)
    .encrypt_with_aes256(encryption_key)
    .encode())
```

## Data Classes

### Claim169Input

Input for encoding credentials.

```python
@dataclass
class Claim169Input:
    id: Optional[str] = None
    version: Optional[str] = None
    language: Optional[str] = None
    full_name: Optional[str] = None
    first_name: Optional[str] = None
    middle_name: Optional[str] = None
    last_name: Optional[str] = None
    date_of_birth: Optional[str] = None
    gender: Optional[int] = None  # 1=Male, 2=Female, 3=Other
    address: Optional[str] = None
    email: Optional[str] = None
    phone: Optional[str] = None
    nationality: Optional[str] = None
    marital_status: Optional[int] = None  # 1=Unmarried, 2=Married, 3=Divorced
    guardian: Optional[str] = None
    photo: Optional[bytes] = None
    photo_format: Optional[int] = None  # 1=JPEG, 2=JPEG2000, 3=AVIF
    legal_status: Optional[str] = None
    country_of_issuance: Optional[str] = None
    location_code: Optional[str] = None
    secondary_language: Optional[str] = None
    secondary_full_name: Optional[str] = None
    best_quality_fingers: Optional[List[int]] = None
```

### CwtMetaInput

CWT metadata for encoding.

```python
@dataclass
class CwtMetaInput:
    issuer: Optional[str] = None
    subject: Optional[str] = None
    expires_at: Optional[int] = None  # Unix timestamp
    not_before: Optional[int] = None  # Unix timestamp
    issued_at: Optional[int] = None   # Unix timestamp
```

### DecodeResult

Result of successful decoding.

```python
class DecodeResult:
    claim169: Claim169
    cwt_meta: CwtMeta
```

### Claim169

Decoded identity data (read-only).

```python
class Claim169:
    id: Optional[str]
    version: Optional[str]
    language: Optional[str]
    full_name: Optional[str]
    first_name: Optional[str]
    middle_name: Optional[str]
    last_name: Optional[str]
    date_of_birth: Optional[str]
    gender: Optional[int]
    address: Optional[str]
    email: Optional[str]
    phone: Optional[str]
    nationality: Optional[str]
    marital_status: Optional[int]
    guardian: Optional[str]
    photo: Optional[bytes]
    photo_format: Optional[int]
    legal_status: Optional[str]
    country_of_issuance: Optional[str]
    location_code: Optional[str]
    secondary_language: Optional[str]
    secondary_full_name: Optional[str]
    best_quality_fingers: Optional[List[int]]
```

### CwtMeta

Decoded CWT metadata (read-only).

```python
class CwtMeta:
    issuer: Optional[str]
    subject: Optional[str]
    expires_at: Optional[int]
    not_before: Optional[int]
    issued_at: Optional[int]
```

## Exceptions

### Claim169Error

Base exception for all library errors.

```python
from claim169 import Claim169Error

try:
    result = decode_with_ed25519(qr_data, public_key)
except Claim169Error as e:
    print(f"Error: {e}")
```

## Complete Example

```python
from claim169 import (
    Claim169Input, CwtMetaInput,
    encode_with_ed25519, decode_with_ed25519
)
import time

# Generate or load keys
private_key = bytes.fromhex(
    "9d61b19deffd5a60ba844af492ec2cc4"
    "4449c5697b326919703bac031cae7f60"
)
public_key = bytes.fromhex(
    "d75a980182b10ab7d54bfed3c964073a"
    "0ee172f3daa62325af021a68f707511a"
)

# Create credential
claim = Claim169Input(
    id="USER-12345",
    full_name="Alice Smith",
    date_of_birth="19900101",
    gender=2,
    email="alice@example.com"
)

meta = CwtMetaInput(
    issuer="https://identity.example.org",
    issued_at=int(time.time()),
    expires_at=int(time.time()) + 365 * 24 * 60 * 60
)

# Encode
qr_data = encode_with_ed25519(claim, meta, private_key)
print(f"Encoded: {qr_data[:50]}...")

# Decode and verify
result = decode_with_ed25519(qr_data, public_key)
print(f"Name: {result.claim169.full_name}")
print(f"Issuer: {result.cwt_meta.issuer}")
```
