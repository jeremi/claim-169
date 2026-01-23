# Python API Reference

## Installation

```bash
pip install claim169
```

## Quick reference

```python
import claim169

from claim169 import (
    # Exceptions
    Claim169Exception,
    Base45DecodeError,
    DecompressError,
    CoseParseError,
    CwtParseError,
    Claim169NotFoundError,
    SignatureError,
    DecryptionError,
    # Data classes
    Claim169Input,
    CwtMetaInput,
    DecodeResult,
    # Decode
    decode_unverified,
    decode_with_ed25519,
    decode_with_ecdsa_p256,
    decode_with_verifier,
    decode_encrypted_aes,
    decode_with_decryptor,
    # Encode
    encode_with_ed25519,
    encode_with_ecdsa_p256,
    encode_signed_encrypted,
    encode_unsigned,
    # Utilities
    generate_nonce,
    version,
)
```

!!! warning "About `decode()`"
    `decode()` requires a verification key by default. To explicitly decode without verification (testing only), pass `allow_unverified=True`.

## Exceptions

All Python errors inherit from `Claim169Exception`. Common cases:

- `Base45DecodeError`
- `DecompressError`
- `CoseParseError`
- `CwtParseError`
- `Claim169NotFoundError`
- `SignatureError`
- `DecryptionError`

## Decoding

### `decode` (recommended entry point)

```python
def decode(
    qr_text: str,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0,
    verify_with_ed25519: bytes | None = None,
    verify_with_ecdsa_p256: bytes | None = None,
    allow_unverified: bool = False,
) -> DecodeResult
```

### `decode_unverified` (testing only)

```python
def decode_unverified(
    qr_text: str,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0,
) -> DecodeResult
```

### `decode_with_ed25519`

```python
def decode_with_ed25519(
    qr_text: str,
    public_key: bytes,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0,
) -> DecodeResult
```

- `public_key` must be 32 bytes.

### `decode_with_ecdsa_p256`

```python
def decode_with_ecdsa_p256(
    qr_text: str,
    public_key: bytes,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0,
) -> DecodeResult
```

- `public_key` must be SEC1-encoded (33-byte compressed or 65-byte uncompressed).

### `decode_with_verifier` (HSM integration)

Use a custom verifier callback:

```python
def decode_with_verifier(qr_text: str, verifier: VerifierCallback) -> DecodeResult
```

### `decode_encrypted_aes`

Decode encrypted credentials with an AES key (16 or 32 bytes). By default, this requires a `verifier` callback for the nested COSE_Sign1 signature. To explicitly skip verification (testing only), pass `allow_unverified=True`.

```python
def decode_encrypted_aes(
    qr_text: str,
    key: bytes,
    verifier: VerifierCallback | None = None,
    allow_unverified: bool = False,
) -> DecodeResult
```

### `decode_with_decryptor` (custom decryptor)

```python
def decode_with_decryptor(
    qr_text: str,
    decryptor: DecryptorCallback,
    verifier: VerifierCallback | None = None,
    allow_unverified: bool = False,
) -> DecodeResult
```

## Encoding

### Inputs

Python encoding uses `Claim169Input` and `CwtMetaInput`. The Python encoder currently exposes a **subset** of Claim 169 fields for encoding (demographics + photo + secondary fields). Biometric encoding and `bestQualityFingers` are not exposed yet.

### `encode_with_ed25519`

```python
def encode_with_ed25519(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    private_key: bytes,
) -> str
```

### `encode_with_ecdsa_p256`

```python
def encode_with_ecdsa_p256(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    private_key: bytes,
) -> str
```

### `encode_signed_encrypted` (Ed25519 + AES-256-GCM)

```python
def encode_signed_encrypted(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    sign_key: bytes,
    encrypt_key: bytes,
) -> str
```

### `encode_unsigned` (testing only)

```python
def encode_unsigned(claim169: Claim169Input, cwt_meta: CwtMetaInput) -> str
```

## Utilities

```python
def generate_nonce() -> bytes  # 12 bytes
def version() -> str
```
