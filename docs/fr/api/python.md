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
    decode_with_ed25519,
    decode_with_ecdsa_p256,
    decode_with_verifier,
    decode_encrypted_aes,
    decode_encrypted_aes128,
    decode_encrypted_aes256,
    decode_with_decryptor,
    # Encodage
    encode_with_ed25519,
    encode_with_ecdsa_p256,
    encode_signed_encrypted,
    encode_signed_encrypted_aes128,
    encode_with_signer,
    encode_with_signer_and_encryptor,
    encode_with_encryptor,
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

## Fournisseurs Cryptographiques Personnalisés

Les fournisseurs cryptographiques personnalisés permettent l'intégration avec des systèmes cryptographiques externes tels que les Modules de Sécurité Matérielle (HSM), les Services de Gestion de Clés cloud (AWS KMS, Google Cloud KMS, Azure Key Vault), les cartes à puce, les Modules de Plateforme Sécurisée (TPM) et les services de signature à distance.

### Signatures des Callbacks

```python
# Callback de signature : signe les données et retourne la signature
# Lève une exception en cas d'échec
SignerCallback = Callable[[str, bytes | None, bytes], bytes]
# Paramètres : (algorithm, key_id, data) -> signature

# Callback de vérification : vérifie une signature, lève une exception si invalide
VerifierCallback = Callable[[str, bytes | None, bytes, bytes], None]
# Paramètres : (algorithm, key_id, data, signature) -> None

# Callback de chiffrement : chiffre le texte clair et retourne le texte chiffré avec le tag d'authentification
EncryptorCallback = Callable[[str, bytes | None, bytes, bytes, bytes], bytes]
# Paramètres : (algorithm, key_id, nonce, aad, plaintext) -> ciphertext

# Callback de déchiffrement : déchiffre le texte chiffré et retourne le texte clair
DecryptorCallback = Callable[[str, bytes | None, bytes, bytes, bytes], bytes]
# Paramètres : (algorithm, key_id, nonce, aad, ciphertext) -> plaintext
```

### Identifiants d'Algorithme

- `"EdDSA"` - Signatures Ed25519
- `"ES256"` - Signatures ECDSA P-256
- `"A128GCM"` - Chiffrement AES-128-GCM
- `"A256GCM"` - Chiffrement AES-256-GCM

### Encodage avec Signature Personnalisée

#### `encode_with_signer`

Signer des identifiants en utilisant un callback de signature personnalisé :

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

Exemple avec AWS KMS :

```python
import boto3

kms = boto3.client("kms")

def aws_kms_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
    # ARN de la clé AWS KMS stockée dans key_id ou configurée en externe
    response = kms.sign(
        KeyId="arn:aws:kms:us-east-1:123456789:key/example-key-id",
        Message=data,
        MessageType="RAW",
        SigningAlgorithm="ECDSA_SHA_256",  # Mapper depuis le paramètre algorithm
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

Signer avec un callback personnalisé et chiffrer avec un callback personnalisé :

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

Exemple avec HSM pour les deux opérations :

```python
def hsm_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
    # Utiliser le HSM pour signer
    return hsm.sign(key_id, data, algorithm)

def hsm_encryptor(
    algorithm: str,
    key_id: bytes | None,
    nonce: bytes,
    aad: bytes,
    plaintext: bytes,
) -> bytes:
    # Utiliser le HSM pour chiffrer (retourne texte chiffré + tag d'authentification)
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

Signer avec Ed25519 logiciel, chiffrer avec un callback personnalisé :

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

### Encodage AES-128-GCM

#### `encode_signed_encrypted_aes128`

Signer avec Ed25519 et chiffrer avec AES-128-GCM :

```python
def encode_signed_encrypted_aes128(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    sign_key: bytes,
    encrypt_key: bytes,
) -> str
```

- `sign_key` : clé privée Ed25519 de 32 octets
- `encrypt_key` : clé AES-128 de 16 octets

### Déchiffrement AES Explicite

#### `decode_encrypted_aes128`

Décoder des identifiants chiffrés avec AES-128-GCM :

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

- `key` : clé AES-128 de 16 octets

#### `decode_encrypted_aes256`

Décoder des identifiants chiffrés avec AES-256-GCM (version explicite) :

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

- `key` : clé AES-256 de 32 octets

### Exemple de Déchiffrement Personnalisé

Utilisation de Google Cloud KMS pour le déchiffrement :

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
    # Déchiffrement AEAD GCP KMS
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
    allow_unverified=True,  # Ou fournir un verifier
)
```

### Exemple de Vérification Personnalisée

Utilisation d'Azure Key Vault pour la vérification de signature :

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
    # Mapper l'algorithme vers Azure SignatureAlgorithm
    az_alg = SignatureAlgorithm.es256 if algorithm == "ES256" else SignatureAlgorithm.eddsa
    result = client.verify(az_alg, data, signature)
    if not result.is_valid:
        raise ValueError("Échec de la vérification de signature")

result = decode_with_verifier(qr_text, verifier=azure_verifier)
```

## Utilitaires

```python
def generate_nonce() -> bytes  # 12 octets
def version() -> str
```
