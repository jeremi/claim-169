# Référence API

Documentation API complète pour le SDK Python claim169.

## Fonctions du module

### version()

Récupérer la version de la bibliothèque.

```python
def version() -> str
```

**Renvoie :** chaîne de version au format semver (p. ex. "0.1.0-alpha.2")

**Exemple :**
```python
import claim169
print(claim169.version())  # "0.1.0-alpha.2"
```

### generate_nonce()

Générer un nonce aléatoire de 12 octets pour le chiffrement AES-GCM.

```python
def generate_nonce() -> list[int]
```

**Renvoie :** nonce 12 octets sous forme de liste d’entiers (convertir avec `bytes()`)

**Exemple :**
```python
nonce = claim169.generate_nonce()
nonce_bytes = bytes(nonce)  # 12 bytes
```

---

## Fonctions de décodage

### decode_unverified()

Décoder sans vérification de signature. **INSÉCURISÉ — tests uniquement.**

```python
def decode_unverified(
    qr_text: str,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0
) -> DecodeResult
```

**Paramètres :**

| Nom | Type | Par défaut | Description |
|-----|------|------------|-------------|
| `qr_text` | `str` | requis | Contenu QR encodé en Base45 |
| `skip_biometrics` | `bool` | `False` | Ignorer le parsing biométrique |
| `max_decompressed_bytes` | `int` | `65536` | Limite de taille décompressée |
| `validate_timestamps` | `bool` | `True` | Valider les horodatages exp/nbf |
| `clock_skew_tolerance_seconds` | `int` | `0` | Tolérance aux écarts d’horloge |

**Renvoie :** `DecodeResult`

**Lève :** `Base45DecodeError`, `DecompressError`, `CoseParseError`, `CwtParseError`, `Claim169NotFoundError`, `Claim169Exception`

---

### decode_with_ed25519()

Décoder avec vérification de signature Ed25519.

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

**Paramètres :**

| Nom | Type | Par défaut | Description |
|-----|------|------------|-------------|
| `qr_text` | `str` | requis | Contenu QR encodé en Base45 |
| `public_key` | `bytes` | requis | Clé publique Ed25519 (32 octets) |
| `skip_biometrics` | `bool` | `False` | Ignorer le parsing biométrique |
| `max_decompressed_bytes` | `int` | `65536` | Limite de taille décompressée |
| `validate_timestamps` | `bool` | `True` | Valider les horodatages exp/nbf |
| `clock_skew_tolerance_seconds` | `int` | `0` | Tolérance aux écarts d’horloge |

**Renvoie :** `DecodeResult`

**Lève :** `SignatureError`, `Base45DecodeError`, `DecompressError`, `CoseParseError`, `CwtParseError`, `Claim169NotFoundError`

---

### decode_with_ecdsa_p256()

Décoder avec vérification de signature ECDSA P-256.

```python
def decode_with_ecdsa_p256(
    qr_text: str,
    public_key: bytes,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0
) -> DecodeResult
```

**Paramètres :**

| Nom | Type | Par défaut | Description |
|-----|------|------------|-------------|
| `qr_text` | `str` | requis | Contenu QR encodé en Base45 |
| `public_key` | `bytes` | requis | Clé publique P-256 encodée SEC1 (33 ou 65 octets) |
| `skip_biometrics` | `bool` | `False` | Ignorer le parsing biométrique |
| `max_decompressed_bytes` | `int` | `65536` | Limite de taille décompressée |
| `validate_timestamps` | `bool` | `True` | Valider les horodatages exp/nbf |
| `clock_skew_tolerance_seconds` | `int` | `0` | Tolérance aux écarts d’horloge |

**Renvoie :** `DecodeResult`

**Lève :** `SignatureError`, `Base45DecodeError`, `DecompressError`, `CoseParseError`, `CwtParseError`, `Claim169NotFoundError`

---

### decode_with_ed25519_pem()

Décoder avec vérification de signature Ed25519 à partir d'une clé publique PEM.

```python
def decode_with_ed25519_pem(
    qr_text: str,
    pem: str,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0
) -> DecodeResult
```

**Paramètres :**

| Nom | Type | Par défaut | Description |
|-----|------|------------|-------------|
| `qr_text` | `str` | requis | Contenu QR encodé en Base45 |
| `pem` | `str` | requis | Clé publique Ed25519 au format PEM (SPKI) |
| `skip_biometrics` | `bool` | `False` | Ignorer le parsing biométrique |
| `max_decompressed_bytes` | `int` | `65536` | Limite de taille décompressée |
| `validate_timestamps` | `bool` | `True` | Valider les horodatages exp/nbf |
| `clock_skew_tolerance_seconds` | `int` | `0` | Tolérance aux écarts d'horloge |

**Renvoie :** `DecodeResult`

**Lève :** `SignatureError`, `Base45DecodeError`, `DecompressError`, `CoseParseError`, `CwtParseError`, `Claim169NotFoundError`

---

### decode_with_ecdsa_p256_pem()

Décoder avec vérification de signature ECDSA P-256 à partir d'une clé publique PEM.

```python
def decode_with_ecdsa_p256_pem(
    qr_text: str,
    pem: str,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0
) -> DecodeResult
```

**Paramètres :**

| Nom | Type | Par défaut | Description |
|-----|------|------------|-------------|
| `qr_text` | `str` | requis | Contenu QR encodé en Base45 |
| `pem` | `str` | requis | Clé publique P-256 au format PEM (SPKI) |
| `skip_biometrics` | `bool` | `False` | Ignorer le parsing biométrique |
| `max_decompressed_bytes` | `int` | `65536` | Limite de taille décompressée |
| `validate_timestamps` | `bool` | `True` | Valider les horodatages exp/nbf |
| `clock_skew_tolerance_seconds` | `int` | `0` | Tolérance aux écarts d'horloge |

**Renvoie :** `DecodeResult`

**Lève :** `SignatureError`, `Base45DecodeError`, `DecompressError`, `CoseParseError`, `CwtParseError`, `Claim169NotFoundError`

---

### decode_with_verifier()

Décoder via un callback de vérification personnalisé (intégration HSM/KMS).

```python
def decode_with_verifier(
    qr_text: str,
    verifier: Callable[[str, bytes | None, bytes, bytes], None]
) -> DecodeResult
```

**Paramètres :**

| Nom | Type | Description |
|-----|------|-------------|
| `qr_text` | `str` | Contenu QR encodé en Base45 |
| `verifier` | `Callable` | Callback `(algorithm, key_id, data, signature) -> None` |

Le callback reçoit :
- `algorithm` : nom d’algorithme ("EdDSA" ou "ES256")
- `key_id` : identifiant de clé optionnel depuis l’en-tête COSE
- `data` : données signées
- `signature` : signature à vérifier

Le callback doit lever une exception si la vérification échoue.

**Renvoie :** `DecodeResult`

**Lève :** `SignatureError`, `Claim169Exception`

---

### decode_encrypted_aes()

Décoder un identifiant chiffré AES-GCM (détection automatique de taille de clé).

```python
def decode_encrypted_aes(
    qr_text: str,
    key: bytes,
    verifier: Callable | None = None,
    allow_unverified: bool = False
) -> DecodeResult
```

**Paramètres :**

| Nom | Type | Par défaut | Description |
|-----|------|------------|-------------|
| `qr_text` | `str` | requis | Contenu QR encodé en Base45 |
| `key` | `bytes` | requis | Clé AES (16 ou 32 octets) |
| `verifier` | `Callable` | `None` | Callback de vérification optionnel |
| `allow_unverified` | `bool` | `False` | Ignorer la vérification (INSÉCURISÉ) |

**Renvoie :** `DecodeResult`

**Lève :** `DecryptionError`, `SignatureError`, `ValueError`, `Claim169Exception`

---

### decode_encrypted_aes256()

Décoder un identifiant chiffré AES-256-GCM (valide une clé 32 octets).

```python
def decode_encrypted_aes256(
    qr_text: str,
    key: bytes,
    verifier: Callable | None = None,
    allow_unverified: bool = False
) -> DecodeResult
```

**Paramètres :**

| Nom | Type | Par défaut | Description |
|-----|------|------------|-------------|
| `qr_text` | `str` | requis | Contenu QR encodé en Base45 |
| `key` | `bytes` | requis | Clé AES-256 (32 octets) |
| `verifier` | `Callable` | `None` | Callback de vérification optionnel |
| `allow_unverified` | `bool` | `False` | Ignorer la vérification (INSÉCURISÉ) |

**Renvoie :** `DecodeResult`

**Lève :** `DecryptionError`, `ValueError` (si la clé n’a pas 32 octets)

---

### decode_encrypted_aes128()

Décoder un identifiant chiffré AES-128-GCM (valide une clé 16 octets).

```python
def decode_encrypted_aes128(
    qr_text: str,
    key: bytes,
    verifier: Callable | None = None,
    allow_unverified: bool = False
) -> DecodeResult
```

**Paramètres :**

| Nom | Type | Par défaut | Description |
|-----|------|------------|-------------|
| `qr_text` | `str` | requis | Contenu QR encodé en Base45 |
| `key` | `bytes` | requis | Clé AES-128 (16 octets) |
| `verifier` | `Callable` | `None` | Callback de vérification optionnel |
| `allow_unverified` | `bool` | `False` | Ignorer la vérification (INSÉCURISÉ) |

**Renvoie :** `DecodeResult`

**Lève :** `DecryptionError`, `ValueError` (si la clé n’a pas 16 octets)

---

### decode_with_decryptor()

Décoder via un callback de déchiffrement personnalisé (intégration HSM/KMS).

```python
def decode_with_decryptor(
    qr_text: str,
    decryptor: Callable[[str, bytes | None, bytes, bytes, bytes], bytes],
    verifier: Callable | None = None,
    allow_unverified: bool = False
) -> DecodeResult
```

**Paramètres :**

| Nom | Type | Par défaut | Description |
|-----|------|------------|-------------|
| `qr_text` | `str` | requis | Contenu QR encodé en Base45 |
| `decryptor` | `Callable` | requis | Callback `(algorithm, key_id, nonce, aad, ciphertext) -> bytes` |
| `verifier` | `Callable` | `None` | Callback de vérification optionnel |
| `allow_unverified` | `bool` | `False` | Ignorer la vérification (INSÉCURISÉ) |

Le callback decryptor reçoit :
- `algorithm` : nom d’algorithme ("A256GCM" ou "A128GCM")
- `key_id` : identifiant de clé optionnel depuis l’en-tête COSE
- `nonce` : nonce 12 octets
- `aad` : additional authenticated data
- `ciphertext` : données chiffrées avec tag d’authentification

Le callback doit retourner les octets du texte clair.

**Renvoie :** `DecodeResult`

**Lève :** `DecryptionError`, `Claim169Exception`

---

## Fonctions d’encodage

### encode_with_ed25519()

Encoder avec une signature Ed25519.

```python
def encode_with_ed25519(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    private_key: bytes,
    skip_biometrics: bool = False
) -> str
```

**Paramètres :**

| Nom | Type | Par défaut | Description |
|-----|------|------------|-------------|
| `claim169` | `Claim169Input` | requis | Données d’identité |
| `cwt_meta` | `CwtMetaInput` | requis | Métadonnées du jeton |
| `private_key` | `bytes` | requis | Clé privée Ed25519 (32 octets) |
| `skip_biometrics` | `bool` | `False` | Exclure la biométrie |

**Renvoie :** chaîne encodée Base45

**Lève :** `ValueError`, `Claim169Exception`

---

### encode_with_ecdsa_p256()

Encoder avec une signature ECDSA P-256.

```python
def encode_with_ecdsa_p256(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    private_key: bytes,
    skip_biometrics: bool = False
) -> str
```

**Paramètres :**

| Nom | Type | Par défaut | Description |
|-----|------|------------|-------------|
| `claim169` | `Claim169Input` | requis | Données d’identité |
| `cwt_meta` | `CwtMetaInput` | requis | Métadonnées du jeton |
| `private_key` | `bytes` | requis | Clé privée ECDSA P-256 (32 octets) |
| `skip_biometrics` | `bool` | `False` | Exclure la biométrie |

**Renvoie :** chaîne encodée Base45

**Lève :** `ValueError`, `Claim169Exception`

---

### encode_signed_encrypted()

Encoder avec signature Ed25519 et chiffrement AES-256-GCM.

```python
def encode_signed_encrypted(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    sign_key: bytes,
    encrypt_key: bytes,
    skip_biometrics: bool = False
) -> str
```

**Paramètres :**

| Nom | Type | Par défaut | Description |
|-----|------|------------|-------------|
| `claim169` | `Claim169Input` | requis | Données d’identité |
| `cwt_meta` | `CwtMetaInput` | requis | Métadonnées du jeton |
| `sign_key` | `bytes` | requis | Clé privée Ed25519 (32 octets) |
| `encrypt_key` | `bytes` | requis | Clé AES-256 (32 octets) |
| `skip_biometrics` | `bool` | `False` | Exclure la biométrie |

**Renvoie :** chaîne encodée Base45

**Lève :** `ValueError`, `Claim169Exception`

---

### encode_signed_encrypted_aes128()

Encoder avec signature Ed25519 et chiffrement AES-128-GCM.

```python
def encode_signed_encrypted_aes128(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    sign_key: bytes,
    encrypt_key: bytes,
    skip_biometrics: bool = False
) -> str
```

**Paramètres :**

| Nom | Type | Par défaut | Description |
|-----|------|------------|-------------|
| `claim169` | `Claim169Input` | requis | Données d’identité |
| `cwt_meta` | `CwtMetaInput` | requis | Métadonnées du jeton |
| `sign_key` | `bytes` | requis | Clé privée Ed25519 (32 octets) |
| `encrypt_key` | `bytes` | requis | Clé AES-128 (16 octets) |
| `skip_biometrics` | `bool` | `False` | Exclure la biométrie |

**Renvoie :** chaîne encodée Base45

**Lève :** `ValueError`, `Claim169Exception`

---

### encode_unsigned()

Encoder sans signature. **INSÉCURISÉ — tests uniquement.**

```python
def encode_unsigned(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    skip_biometrics: bool = False
) -> str
```

**Paramètres :**

| Nom | Type | Par défaut | Description |
|-----|------|------------|-------------|
| `claim169` | `Claim169Input` | requis | Données d’identité |
| `cwt_meta` | `CwtMetaInput` | requis | Métadonnées du jeton |
| `skip_biometrics` | `bool` | `False` | Exclure la biométrie |

**Renvoie :** chaîne encodée Base45

**Lève :** `Claim169Exception`

---

### encode_with_signer()

Encoder via un callback de signature personnalisé (intégration HSM/KMS).

```python
def encode_with_signer(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    signer: Callable[[str, bytes | None, bytes], bytes],
    algorithm: str,
    key_id: bytes | None = None,
    skip_biometrics: bool = False
) -> str
```

**Paramètres :**

| Nom | Type | Par défaut | Description |
|-----|------|------------|-------------|
| `claim169` | `Claim169Input` | requis | Données d’identité |
| `cwt_meta` | `CwtMetaInput` | requis | Métadonnées du jeton |
| `signer` | `Callable` | requis | Callback `(algorithm, key_id, data) -> signature` |
| `algorithm` | `str` | requis | "EdDSA" ou "ES256" |
| `key_id` | `bytes` | `None` | Identifiant de clé optionnel |
| `skip_biometrics` | `bool` | `False` | Exclure la biométrie |

**Renvoie :** chaîne encodée Base45

**Lève :** `ValueError`, `Claim169Exception`

---

### encode_with_signer_and_encryptor()

Encoder via des callbacks personnalisés (signer + encryptor).

```python
def encode_with_signer_and_encryptor(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    signer: Callable[[str, bytes | None, bytes], bytes],
    sign_algorithm: str,
    encryptor: Callable[[str, bytes | None, bytes, bytes, bytes], bytes],
    encrypt_algorithm: str,
    key_id: bytes | None = None,
    skip_biometrics: bool = False
) -> str
```

**Paramètres :**

| Nom | Type | Par défaut | Description |
|-----|------|------------|-------------|
| `claim169` | `Claim169Input` | requis | Données d’identité |
| `cwt_meta` | `CwtMetaInput` | requis | Métadonnées du jeton |
| `signer` | `Callable` | requis | Callback de signature |
| `sign_algorithm` | `str` | requis | "EdDSA" ou "ES256" |
| `encryptor` | `Callable` | requis | Callback de chiffrement |
| `encrypt_algorithm` | `str` | requis | "A256GCM" ou "A128GCM" |
| `key_id` | `bytes` | `None` | Identifiant de clé optionnel |
| `skip_biometrics` | `bool` | `False` | Exclure la biométrie |

**Renvoie :** chaîne encodée Base45

**Lève :** `ValueError`, `Claim169Exception`

---

### encode_with_encryptor()

Encoder avec signature logicielle et callback de chiffrement personnalisé.

```python
def encode_with_encryptor(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    sign_key: bytes,
    encryptor: Callable[[str, bytes | None, bytes, bytes, bytes], bytes],
    encrypt_algorithm: str,
    skip_biometrics: bool = False
) -> str
```

**Paramètres :**

| Nom | Type | Par défaut | Description |
|-----|------|------------|-------------|
| `claim169` | `Claim169Input` | requis | Données d’identité |
| `cwt_meta` | `CwtMetaInput` | requis | Métadonnées du jeton |
| `sign_key` | `bytes` | requis | Clé privée Ed25519 (32 octets) |
| `encryptor` | `Callable` | requis | Callback de chiffrement |
| `encrypt_algorithm` | `str` | requis | "A256GCM" ou "A128GCM" |
| `skip_biometrics` | `bool` | `False` | Exclure la biométrie |

**Renvoie :** chaîne encodée Base45

**Lève :** `ValueError`, `Claim169Exception`

---

## Classes

### Claim169Input

Classe d’entrée pour encoder des données d’identité.

```python
class Claim169Input:
    def __init__(
        self,
        id: str | None = None,
        full_name: str | None = None
    ) -> None
```

**Attributs :**

| Nom | Type | Description |
|-----|------|-------------|
| `id` | `str \| None` | Identifiant unique |
| `version` | `str \| None` | Version de l’identifiant |
| `language` | `str \| None` | Code langue principal |
| `full_name` | `str \| None` | Nom complet |
| `first_name` | `str \| None` | Prénom |
| `middle_name` | `str \| None` | Deuxième prénom |
| `last_name` | `str \| None` | Nom de famille |
| `date_of_birth` | `str \| None` | Date de naissance (YYYY-MM-DD) |
| `gender` | `int \| None` | 1=Male, 2=Female, 3=Other |
| `address` | `str \| None` | Adresse complète |
| `email` | `str \| None` | Adresse email |
| `phone` | `str \| None` | Numéro de téléphone |
| `nationality` | `str \| None` | Code nationalité |
| `marital_status` | `int \| None` | 1=Unmarried, 2=Married, 3=Divorced |
| `guardian` | `str \| None` | Nom du tuteur/responsable |
| `photo` | `bytes \| None` | Données photo |
| `photo_format` | `int \| None` | 1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP |
| `secondary_full_name` | `str \| None` | Nom en langue secondaire |
| `secondary_language` | `str \| None` | Code langue secondaire |
| `location_code` | `str \| None` | Code de localisation |
| `legal_status` | `str \| None` | Statut légal |
| `country_of_issuance` | `str \| None` | Code pays d’émission |

---

### CwtMetaInput

Classe d’entrée pour les métadonnées CWT.

```python
class CwtMetaInput:
    def __init__(
        self,
        issuer: str | None = None,
        expires_at: int | None = None
    ) -> None
```

**Attributs :**

| Nom | Type | Description |
|-----|------|-------------|
| `issuer` | `str \| None` | Émetteur |
| `subject` | `str \| None` | Identifiant du sujet |
| `expires_at` | `int \| None` | Expiration (Unix epoch) |
| `not_before` | `int \| None` | Pas valide avant (Unix epoch) |
| `issued_at` | `int \| None` | Date d’émission (Unix epoch) |

---

### DecodeResult

Résultat du décodage d’un identifiant.

```python
class DecodeResult:
    claim169: Claim169
    cwt_meta: CwtMeta
    verification_status: str

    def is_verified(self) -> bool
```

**Attributs :**

| Nom | Type | Description |
|-----|------|-------------|
| `claim169` | `Claim169` | Données d’identité décodées |
| `cwt_meta` | `CwtMeta` | Métadonnées CWT |
| `verification_status` | `str` | "verified", "skipped", etc. |

**Méthodes :**

- `is_verified() -> bool` : renvoie `True` si la signature a été vérifiée

---

### Claim169

Claim d’identité décodé.

```python
class Claim169:
    # All fields are read-only
    id: str | None
    version: str | None
    language: str | None
    full_name: str | None
    first_name: str | None
    middle_name: str | None
    last_name: str | None
    date_of_birth: str | None
    gender: int | None
    address: str | None
    email: str | None
    phone: str | None
    nationality: str | None
    marital_status: int | None
    guardian: str | None
    photo: bytes | None
    photo_format: int | None
    best_quality_fingers: bytes | None
    secondary_full_name: str | None
    secondary_language: str | None
    location_code: str | None
    legal_status: str | None
    country_of_issuance: str | None

    # Biometrics
    right_thumb: list[Biometric] | None
    right_pointer_finger: list[Biometric] | None
    right_middle_finger: list[Biometric] | None
    right_ring_finger: list[Biometric] | None
    right_little_finger: list[Biometric] | None
    left_thumb: list[Biometric] | None
    left_pointer_finger: list[Biometric] | None
    left_middle_finger: list[Biometric] | None
    left_ring_finger: list[Biometric] | None
    left_little_finger: list[Biometric] | None
    right_iris: list[Biometric] | None
    left_iris: list[Biometric] | None
    face: list[Biometric] | None
    right_palm: list[Biometric] | None
    left_palm: list[Biometric] | None
    voice: list[Biometric] | None

    def has_biometrics(self) -> bool
    def to_dict(self) -> dict
```

**Méthodes :**

- `has_biometrics() -> bool` : renvoie `True` si une biométrie est présente
- `to_dict() -> dict` : convertir en dictionnaire Python

---

### CwtMeta

Métadonnées CWT issues de l’identifiant décodé.

```python
class CwtMeta:
    issuer: str | None
    subject: str | None
    expires_at: int | None
    not_before: int | None
    issued_at: int | None

    def is_valid_now(self) -> bool
    def is_expired(self) -> bool
```

**Méthodes :**

- `is_valid_now() -> bool` : renvoie `True` si le jeton est actuellement valide
- `is_expired() -> bool` : renvoie `True` si le jeton est expiré

---

### Biometric

Conteneur de données biométriques.

```python
class Biometric:
    data: bytes
    format: int | None
    sub_format: int | None
    issuer: str | None
```

**Attributs :**

| Nom | Type | Description |
|-----|------|-------------|
| `data` | `bytes` | Données biométriques brutes |
| `format` | `int \| None` | Code format |
| `sub_format` | `int \| None` | Code sous-format |
| `issuer` | `str \| None` | Émetteur biométrique |

---

## Exceptions

Toutes les exceptions héritent de `Claim169Exception`.

### Claim169Exception

Exception de base pour toutes les erreurs claim169.

```python
class Claim169Exception(Exception):
    pass
```

### Base45DecodeError

Levé lorsque le décodage Base45 échoue.

```python
class Base45DecodeError(Claim169Exception):
    pass
```

### DecompressError

Levé lorsque la décompression zlib échoue ou que la limite est dépassée.

```python
class DecompressError(Claim169Exception):
    pass
```

### CoseParseError

Levé lorsque le parsing de la structure COSE échoue.

```python
class CoseParseError(Claim169Exception):
    pass
```

### CwtParseError

Levé lorsque le parsing CWT échoue.

```python
class CwtParseError(Claim169Exception):
    pass
```

### Claim169NotFoundError

Levé lorsque Claim 169 n’est pas présent dans le CWT.

```python
class Claim169NotFoundError(Claim169Exception):
    pass
```

### SignatureError

Levé lorsque la vérification de signature échoue.

```python
class SignatureError(Claim169Exception):
    pass
```

### DecryptionError

Levé lorsque le déchiffrement échoue.

```python
class DecryptionError(Claim169Exception):
    pass
```

---

## Constantes

### Valeurs de genre

| Valeur | Signification |
|-------|---------------|
| 1 | Male |
| 2 | Female |
| 3 | Other |

### Valeurs d’état civil

| Valeur | Signification |
|-------|---------------|
| 1 | Unmarried |
| 2 | Married |
| 3 | Divorced |

### Valeurs de format photo

| Valeur | Signification |
|-------|---------------|
| 1 | JPEG |
| 2 | JPEG2000 |
| 3 | AVIF |
| 4 | WebP |

### Noms d’algorithmes

**Signature :**
- `"EdDSA"` — Ed25519
- `"ES256"` — ECDSA P-256

**Chiffrement :**
- `"A256GCM"` — AES-256-GCM
- `"A128GCM"` — AES-128-GCM
