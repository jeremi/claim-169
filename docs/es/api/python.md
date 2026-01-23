# Referencia API Python

## Instalación

```bash
pip install claim169
```

## Referencia rápida

```python
import claim169

from claim169 import (
    # Excepciones
    Claim169Exception,
    Base45DecodeError,
    DecompressError,
    CoseParseError,
    CwtParseError,
    Claim169NotFoundError,
    SignatureError,
    DecryptionError,
    # Clases de datos
    Claim169Input,
    CwtMetaInput,
    DecodeResult,
    # Decodificación
    decode_unverified,
    decode,
    decode_with_ed25519,
    decode_with_ecdsa_p256,
    decode_with_verifier,
    decode_encrypted_aes,
    decode_with_decryptor,
    # Codificación
    encode_with_ed25519,
    encode_with_ecdsa_p256,
    encode_signed_encrypted,
    encode_unsigned,
    # Utilidades
    generate_nonce,
    version,
)
```

!!! warning "Sobre `decode()`"
    `decode()` es un alias de `decode_unverified()` (sin verificación de firma). En producción, usa `decode_with_ed25519()` / `decode_with_ecdsa_p256()`.

## Excepciones

Todos los errores heredan de `Claim169Exception`. Casos comunes:

- `Base45DecodeError`
- `DecompressError`
- `CoseParseError`
- `CwtParseError`
- `Claim169NotFoundError`
- `SignatureError`
- `DecryptionError`

## Decodificación

### `decode_unverified` (solo pruebas)

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
def decode_with_ed25519(qr_text: str, public_key: bytes) -> DecodeResult
```

- `public_key` debe tener 32 bytes.

### `decode_with_ecdsa_p256`

```python
def decode_with_ecdsa_p256(qr_text: str, public_key: bytes) -> DecodeResult
```

- `public_key` debe estar codificada como SEC1 (33 bytes comprimida o 65 bytes sin comprimir).

### `decode_with_verifier` (integración HSM)

Usa un callback de verificación:

```python
def decode_with_verifier(qr_text: str, verifier: VerifierCallback) -> DecodeResult
```

### `decode_encrypted_aes`

Decodifica credenciales cifradas con una clave AES (16 o 32 bytes). Si pasas `verifier`, la firma interna COSE_Sign1 se verifica vía callback; si no, la verificación se omite.

```python
def decode_encrypted_aes(
    qr_text: str,
    key: bytes,
    verifier: VerifierCallback | None = None,
) -> DecodeResult
```

### `decode_with_decryptor` (descifrado personalizado)

```python
def decode_with_decryptor(
    qr_text: str,
    decryptor: DecryptorCallback,
    verifier: VerifierCallback | None = None,
) -> DecodeResult
```

## Codificación

### Entradas

La codificación Python usa `Claim169Input` y `CwtMetaInput`. Actualmente los bindings de Python exponen un **subconjunto** de campos de Claim 169 para codificar (demografía + foto + campos secundarios).

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

### `encode_unsigned` (solo pruebas)

```python
def encode_unsigned(claim169: Claim169Input, cwt_meta: CwtMetaInput) -> str
```

## Utilidades

```python
def generate_nonce() -> bytes  # 12 bytes
def version() -> str
```

