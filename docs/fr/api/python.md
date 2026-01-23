# Référence API Python

## Installation

```bash
pip install claim169
```

## Référence rapide

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
    # Classes de données
    Claim169Input,
    CwtMetaInput,
    DecodeResult,
    # Décodage
    decode_unverified,
    decode,
    decode_with_ed25519,
    decode_with_ecdsa_p256,
    decode_with_verifier,
    decode_encrypted_aes,
    decode_with_decryptor,
    # Encodage
    encode_with_ed25519,
    encode_with_ecdsa_p256,
    encode_signed_encrypted,
    encode_unsigned,
    # Utilitaires
    generate_nonce,
    version,
)
```

!!! warning "À propos de `decode()`"
    `decode()` exige une clé de vérification par défaut. Pour décoder sans vérification explicitement (tests uniquement), utilisez `allow_unverified=True`.

## Exceptions

Toutes les erreurs héritent de `Claim169Exception`. Cas courants :

- `Base45DecodeError`
- `DecompressError`
- `CoseParseError`
- `CwtParseError`
- `Claim169NotFoundError`
- `SignatureError`
- `DecryptionError`

## Décodage

### `decode` (point d’entrée recommandé)

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

### `decode_unverified` (tests uniquement)

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

- `public_key` doit faire 32 octets.

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

- `public_key` doit être encodée en SEC1 (33 octets compressés ou 65 octets non compressés).

### `decode_with_verifier` (intégration HSM)

Utilisez un callback de vérification :

```python
def decode_with_verifier(qr_text: str, verifier: VerifierCallback) -> DecodeResult
```

### `decode_encrypted_aes`

Décoder des identifiants chiffrés avec une clé AES (16 ou 32 octets). Par défaut, cela nécessite un callback `verifier` pour la signature COSE_Sign1 interne. Pour ignorer explicitement la vérification (tests uniquement), utilisez `allow_unverified=True`.

```python
def decode_encrypted_aes(
    qr_text: str,
    key: bytes,
    verifier: VerifierCallback | None = None,
    allow_unverified: bool = False,
) -> DecodeResult
```

### `decode_with_decryptor` (déchiffrement personnalisé)

```python
def decode_with_decryptor(
    qr_text: str,
    decryptor: DecryptorCallback,
    verifier: VerifierCallback | None = None,
    allow_unverified: bool = False,
) -> DecodeResult
```

## Encodage

### Entrées

L’encodage Python utilise `Claim169Input` et `CwtMetaInput`. Les bindings Python exposent actuellement un **sous-ensemble** des champs Claim 169 à l’encodage (démographie + photo + champs secondaires).

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

### `encode_unsigned` (tests uniquement)

```python
def encode_unsigned(claim169: Claim169Input, cwt_meta: CwtMetaInput) -> str
```

## Utilitaires

```python
def generate_nonce() -> bytes  # 12 octets
def version() -> str
```
