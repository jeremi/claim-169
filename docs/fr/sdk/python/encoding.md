# Encoder des identifiants

Ce guide couvre la création d’identifiants d’identité signés, encodables dans des QR codes.

## Vue d’ensemble

L’encodage suit ces étapes :

1. Créer un `Claim169Input` avec les données d’identité
2. Créer un `CwtMetaInput` avec les métadonnées du jeton
3. Signer avec une clé privée
4. Optionnellement chiffrer avec une clé symétrique
5. Recevoir une chaîne Base45 à utiliser pour générer un QR code

## Créer les données d’identité

### Claim169Input

La classe `Claim169Input` contient tous les champs d’identité :

```python
import claim169

# Créer avec tous les champs en arguments nommés
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

### Référence des champs

| Champ | Type | Description |
|-------|------|-------------|
| `id` | `str` | Identifiant unique |
| `version` | `str` | Version de l’identifiant |
| `language` | `str` | Code langue principal (ISO 639-1) |
| `full_name` | `str` | Nom complet |
| `first_name` | `str` | Prénom |
| `middle_name` | `str` | Deuxième prénom |
| `last_name` | `str` | Nom de famille |
| `date_of_birth` | `str` | Date de naissance (YYYY-MM-DD) |
| `gender` | `int` | `Gender.MALE`, `Gender.FEMALE`, `Gender.OTHER` |
| `address` | `str` | Adresse complète |
| `email` | `str` | Adresse email |
| `phone` | `str` | Numéro de téléphone |
| `nationality` | `str` | Code nationalité |
| `marital_status` | `int` | `MaritalStatus.UNMARRIED`, `MaritalStatus.MARRIED`, `MaritalStatus.DIVORCED` |
| `guardian` | `str` | Nom du tuteur/responsable |
| `photo` | `bytes` | Données photo |
| `photo_format` | `int` | `PhotoFormat.JPEG`, `PhotoFormat.JPEG2000`, `PhotoFormat.AVIF`, `PhotoFormat.WEBP` |
| `secondary_full_name` | `str` | Nom en langue secondaire |
| `secondary_language` | `str` | Code langue secondaire |
| `location_code` | `str` | Code de localisation |
| `legal_status` | `str` | Statut légal |
| `country_of_issuance` | `str` | Code pays d’émission |

### Inclure une photo

```python
# Lire un fichier photo
with open("photo.jpg", "rb") as f:
    photo_data = f.read()

claim = claim169.Claim169Input(
    id="PHOTO-001",
    full_name="Jane Doe",
    photo=photo_data,
    photo_format=claim169.PhotoFormat.JPEG,
)
```

## Créer les métadonnées du jeton

### CwtMetaInput

La classe `CwtMetaInput` contient les métadonnées CWT (CBOR Web Token) :

```python
import time

meta = claim169.CwtMetaInput(
    issuer="https://id.example.org",
    expires_at=int(time.time()) + (365 * 24 * 60 * 60)  # 1 an à partir de maintenant
)
meta.subject = "user-12345"
meta.issued_at = int(time.time())
meta.not_before = int(time.time())  # Valide immédiatement
```

### Champs de métadonnées

| Champ | Type | Description |
|-------|------|-------------|
| `issuer` | `str` | Émetteur (URL ou identifiant) |
| `subject` | `str` | Identifiant du sujet |
| `expires_at` | `int` | Expiration (Unix epoch) |
| `not_before` | `int` | Pas valide avant (Unix epoch) |
| `issued_at` | `int` | Date d’émission (Unix epoch) |

## Signer avec Ed25519

Ed25519 est recommandé pour ses signatures courtes et sa vérification rapide.

```python
import claim169

# Données d'identité
claim = claim169.Claim169Input(
    id="ED25519-001",
    full_name="Jane Doe",
    date_of_birth="1990-05-15",
)

# Métadonnées du jeton
meta = claim169.CwtMetaInput(
    issuer="https://id.example.org",
    expires_at=1900000000,
    issued_at=1700000000,
)

# Clé privée Ed25519 (32 octets)
private_key = bytes.fromhex(
    "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
)

# Encoder avec la fonction unifiée encode()
qr_data = claim169.encode(claim, meta, sign_with_ed25519=private_key)
print(f"Encoded: {len(qr_data)} characters")
```

### Générer des clés Ed25519

```python
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey

# Générer une nouvelle paire de clés
private_key_obj = Ed25519PrivateKey.generate()

# Récupérer les octets bruts
private_key = private_key_obj.private_bytes_raw()  # 32 octets
public_key = private_key_obj.public_key().public_bytes_raw()  # 32 octets

print(f"Private key: {private_key.hex()}")
print(f"Public key: {public_key.hex()}")
```

## Signer avec ECDSA P-256

ECDSA P-256 est largement supporté dans des environnements entreprise.

```python
import claim169

claim = claim169.Claim169Input(id="ECDSA-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(
    issuer="https://id.example.org",
    expires_at=1900000000
)

# Clé privée ECDSA P-256 (32 octets)
private_key = bytes(32)  # Remplacer par la clé réelle

qr_data = claim169.encode(claim, meta, sign_with_ecdsa_p256=private_key)
```

### Générer des clés ECDSA P-256

```python
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.backends import default_backend

# Générer une paire de clés
private_key_obj = ec.generate_private_key(ec.SECP256R1(), default_backend())

# Récupérer la clé privée brute
private_key = private_key_obj.private_numbers().private_value.to_bytes(32, 'big')

# Récupérer la clé publique au format SEC1
public_numbers = private_key_obj.public_key().public_numbers()
public_key = (
    b'\x04' +
    public_numbers.x.to_bytes(32, 'big') +
    public_numbers.y.to_bytes(32, 'big')
)

print(f"Private key: {private_key.hex()}")
print(f"Public key: {public_key.hex()}")
```

## Encoder sans signature

Uniquement pour les tests et le développement. Ne jamais utiliser en production.

```python
import claim169

claim = claim169.Claim169Input(id="TEST-001", full_name="Test User")
meta = claim169.CwtMetaInput(expires_at=1900000000)

# Encoder sans signature (INSÉCURISÉ - tests uniquement)
qr_data = claim169.encode(claim, meta, allow_unsigned=True)
```

## Inclure la biométrie

Les 16 champs biométriques peuvent être définis sur `Claim169Input`. Chaque champ accepte une liste d'objets `Biometric` :

```python
import claim169

# Créer une entrée biométrique
face_biometric = claim169.Biometric(
    data=face_image_bytes,
    format=1,   # Format image
    sub_format=2,
    issuer="https://biometrics.example.org",
)

# Inclure la biométrie dans le credential
claim = claim169.Claim169Input(
    id="BIO-001",
    full_name="Jane Doe",
    face=[face_biometric],
    right_thumb=[claim169.Biometric(data=thumb_bytes, format=1)],
    left_iris=[claim169.Biometric(data=iris_bytes, format=1)],
)

qr_data = claim169.encode(claim, meta, sign_with_ed25519=private_key)
```

### Champs biométriques

| Champ | Description |
|-------|-------------|
| `right_thumb` | Empreinte du pouce droit |
| `right_pointer_finger` | Index droit |
| `right_middle_finger` | Majeur droit |
| `right_ring_finger` | Annulaire droit |
| `right_little_finger` | Auriculaire droit |
| `left_thumb` | Empreinte du pouce gauche |
| `left_pointer_finger` | Index gauche |
| `left_middle_finger` | Majeur gauche |
| `left_ring_finger` | Annulaire gauche |
| `left_little_finger` | Auriculaire gauche |
| `right_iris` | Scan de l'iris droit |
| `left_iris` | Scan de l'iris gauche |
| `face` | Image du visage |
| `right_palm` | Empreinte palmaire droite |
| `left_palm` | Empreinte palmaire gauche |
| `voice` | Échantillon vocal |

### Objet Biometric

Chaque `Biometric` possède les champs suivants :

| Champ | Type | Description |
|-------|------|-------------|
| `data` | `bytes` | Données biométriques brutes (obligatoire) |
| `format` | `int` | Format des données biométriques |
| `sub_format` | `int` | Sous-format biométrique |
| `issuer` | `str` | Autorité émettrice |

## Ignorer la biométrie

Pour réduire la taille du QR code, ignorez l'encodage des données biométriques :

```python
qr_data = claim169.encode(
    claim,
    meta,
    sign_with_ed25519=private_key,
    skip_biometrics=True,
)
```

## Exemple complet

Exemple complet avec toutes les données démographiques :

```python
import claim169
import time

# Créer des données d'identité complètes
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

# Créer les métadonnées du jeton
now = int(time.time())
meta = claim169.CwtMetaInput(
    issuer="https://id.state.il.us",
    expires_at=now + (5 * 365 * 24 * 60 * 60)  # 5 ans
)
meta.subject = "IL-DL-2024-001"
meta.issued_at = now
meta.not_before = now

# Signer avec Ed25519
private_key = bytes.fromhex(
    "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
)

qr_data = claim169.encode(claim, meta, sign_with_ed25519=private_key)

print(f"QR Code content ({len(qr_data)} characters)")
print(f"Ready for QR code generation")
```

## Gestion des erreurs

```python
import claim169

try:
    qr_data = claim169.encode(claim, meta, sign_with_ed25519=private_key)
except ValueError as e:
    print(f"Invalid key format: {e}")
except claim169.Claim169Exception as e:
    print(f"Encoding failed: {e}")
```

## Étapes suivantes

- [Chiffrement](encryption.md) — ajouter le chiffrement AES-GCM
- [Crypto personnalisée](custom-crypto.md) — utiliser un HSM/KMS pour signer
- [Référence API](api.md) — documentation complète des fonctions
