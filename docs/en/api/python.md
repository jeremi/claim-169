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
    decode_encrypted_aes128,
    decode_encrypted_aes256,
    decode_with_decryptor,
    # Encode
    encode_with_ed25519,
    encode_with_ecdsa_p256,
    encode_signed_encrypted,
    encode_signed_encrypted_aes128,
    encode_with_signer,
    encode_with_signer_and_encryptor,
    encode_with_encryptor,
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

## Custom Crypto Providers

Custom crypto providers enable integration with external cryptographic systems such as Hardware Security Modules (HSMs), cloud Key Management Services (AWS KMS, Google Cloud KMS, Azure Key Vault), smart cards, Trusted Platform Modules (TPMs), and remote signing services.

### Callback Signatures

```python
# Signer callback: signs data and returns the signature
# Raises an exception on failure
SignerCallback = Callable[[str, bytes | None, bytes], bytes]
# Parameters: (algorithm, key_id, data) -> signature

# Verifier callback: verifies a signature, raises exception if invalid
VerifierCallback = Callable[[str, bytes | None, bytes, bytes], None]
# Parameters: (algorithm, key_id, data, signature) -> None

# Encryptor callback: encrypts plaintext and returns ciphertext with auth tag
EncryptorCallback = Callable[[str, bytes | None, bytes, bytes, bytes], bytes]
# Parameters: (algorithm, key_id, nonce, aad, plaintext) -> ciphertext

# Decryptor callback: decrypts ciphertext and returns plaintext
DecryptorCallback = Callable[[str, bytes | None, bytes, bytes, bytes], bytes]
# Parameters: (algorithm, key_id, nonce, aad, ciphertext) -> plaintext
```

### Algorithm Identifiers

- `"EdDSA"` - Ed25519 signatures
- `"ES256"` - ECDSA P-256 signatures
- `"A128GCM"` - AES-128-GCM encryption
- `"A256GCM"` - AES-256-GCM encryption

### Custom Signer Encoding

#### `encode_with_signer`

Sign credentials using a custom signer callback:

```python
def encode_with_signer(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    signer: SignerCallback,
    algorithm: str,
    key_id: bytes | None = None,
    skip_biometrics: bool = False,
) -> str
```

Example with AWS KMS:

```python
import boto3

kms = boto3.client("kms")

def aws_kms_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
    # AWS KMS key ARN stored in key_id or configured externally
    response = kms.sign(
        KeyId="arn:aws:kms:us-east-1:123456789:key/example-key-id",
        Message=data,
        MessageType="RAW",
        SigningAlgorithm="ECDSA_SHA_256",  # Map from algorithm parameter
    )
    return response["Signature"]

qr_text = encode_with_signer(
    claim169=claim_data,
    cwt_meta=meta,
    signer=aws_kms_signer,
    algorithm="ES256",
    key_id=b"my-key-id",
)
```

#### `encode_with_signer_and_encryptor`

Sign with a custom signer and encrypt with a custom encryptor:

```python
def encode_with_signer_and_encryptor(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    signer: SignerCallback,
    sign_algorithm: str,
    encryptor: EncryptorCallback,
    encrypt_algorithm: str,
    sign_key_id: bytes | None = None,
    encrypt_key_id: bytes | None = None,
    skip_biometrics: bool = False,
) -> str
```

Example with HSM for both operations:

```python
def hsm_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
    # Use HSM to sign
    return hsm.sign(key_id, data, algorithm)

def hsm_encryptor(
    algorithm: str,
    key_id: bytes | None,
    nonce: bytes,
    aad: bytes,
    plaintext: bytes,
) -> bytes:
    # Use HSM to encrypt (returns ciphertext + auth tag)
    return hsm.encrypt_aead(key_id, nonce, aad, plaintext, algorithm)

qr_text = encode_with_signer_and_encryptor(
    claim169=claim_data,
    cwt_meta=meta,
    signer=hsm_signer,
    sign_algorithm="EdDSA",
    encryptor=hsm_encryptor,
    encrypt_algorithm="A256GCM",
)
```

#### `encode_with_encryptor`

Sign with software Ed25519, encrypt with a custom encryptor:

```python
def encode_with_encryptor(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    sign_key: bytes,
    encryptor: EncryptorCallback,
    encrypt_algorithm: str,
    encrypt_key_id: bytes | None = None,
    skip_biometrics: bool = False,
) -> str
```

### AES-128-GCM Encoding

#### `encode_signed_encrypted_aes128`

Sign with Ed25519 and encrypt with AES-128-GCM:

```python
def encode_signed_encrypted_aes128(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    sign_key: bytes,
    encrypt_key: bytes,
) -> str
```

- `sign_key`: 32-byte Ed25519 private key
- `encrypt_key`: 16-byte AES-128 key

### Explicit AES Decryption

#### `decode_encrypted_aes128`

Decode credentials encrypted with AES-128-GCM:

```python
def decode_encrypted_aes128(
    qr_text: str,
    key: bytes,
    verifier: VerifierCallback | None = None,
    allow_unverified: bool = False,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0,
) -> DecodeResult
```

- `key`: 16-byte AES-128 key

#### `decode_encrypted_aes256`

Decode credentials encrypted with AES-256-GCM (explicit version):

```python
def decode_encrypted_aes256(
    qr_text: str,
    key: bytes,
    verifier: VerifierCallback | None = None,
    allow_unverified: bool = False,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0,
) -> DecodeResult
```

- `key`: 32-byte AES-256 key

### Custom Decryption Example

Using Google Cloud KMS for decryption:

```python
from google.cloud import kms

def gcp_kms_decryptor(
    algorithm: str,
    key_id: bytes | None,
    nonce: bytes,
    aad: bytes,
    ciphertext: bytes,
) -> bytes:
    client = kms.KeyManagementServiceClient()
    key_name = client.crypto_key_path(
        "my-project", "global", "my-keyring", "my-key"
    )
    # GCP KMS AEAD decryption
    response = client.decrypt(
        request={
            "name": key_name,
            "ciphertext": ciphertext,
            "additional_authenticated_data": aad,
        }
    )
    return response.plaintext

result = decode_with_decryptor(
    qr_text=encrypted_qr,
    decryptor=gcp_kms_decryptor,
    allow_unverified=True,  # Or provide a verifier
)
```

### Custom Verification Example

Using Azure Key Vault for signature verification:

```python
from azure.identity import DefaultAzureCredential
from azure.keyvault.keys.crypto import CryptographyClient, SignatureAlgorithm

def azure_verifier(
    algorithm: str,
    key_id: bytes | None,
    data: bytes,
    signature: bytes,
) -> None:
    credential = DefaultAzureCredential()
    client = CryptographyClient(
        "https://my-vault.vault.azure.net/keys/my-key/version",
        credential,
    )
    # Map algorithm to Azure SignatureAlgorithm
    az_alg = SignatureAlgorithm.es256 if algorithm == "ES256" else SignatureAlgorithm.eddsa
    result = client.verify(az_alg, data, signature)
    if not result.is_valid:
        raise ValueError("Signature verification failed")

result = decode_with_verifier(qr_text, verifier=azure_verifier)
```

## Utilities

```python
def generate_nonce() -> bytes  # 12 bytes
def version() -> str
```
