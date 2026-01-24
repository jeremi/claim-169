# Décoder des identifiants

Ce guide couvre le décodage et la vérification d’identifiants d’identité depuis des QR codes.

## Vue d’ensemble

Le décodage suit ces étapes :

1. Récupérer une chaîne Base45 depuis un scanner QR
2. Choisir une méthode de vérification (Ed25519, ECDSA P-256, ou personnalisée)
3. Appeler la fonction de décodage appropriée
4. Accéder au claim décodé et aux métadonnées

## Décoder avec vérification Ed25519

Le cas le plus courant : signatures Ed25519.

```python
import claim169

qr_data = "NCFOXN..."  # Depuis le scanner QR
public_key = bytes.fromhex(
    "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
)

result = claim169.decode_with_ed25519(qr_data, public_key)

# Accéder aux données d’identité
print(f"ID: {result.claim169.id}")
print(f"Name: {result.claim169.full_name}")
print(f"DOB: {result.claim169.date_of_birth}")
print(f"Gender: {result.claim169.gender}")

# Statut de vérification
print(f"Verified: {result.is_verified()}")
print(f"Status: {result.verification_status}")
```

### Signature de la fonction

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

| Paramètre | Type | Par défaut | Description |
|-----------|------|------------|-------------|
| `qr_text` | `str` | requis | Contenu QR encodé en Base45 |
| `public_key` | `bytes` | requis | Clé publique Ed25519 de 32 octets |
| `skip_biometrics` | `bool` | `False` | Ignorer le parsing des données biométriques |
| `max_decompressed_bytes` | `int` | `65536` | Taille max après décompression |
| `validate_timestamps` | `bool` | `True` | Valider les timestamps exp/nbf |
| `clock_skew_tolerance_seconds` | `int` | `0` | Tolérance aux écarts d’horloge |

## Décoder avec vérification ECDSA P-256

Pour des identifiants signés en ECDSA P-256 :

```python
import claim169

qr_data = "NCFOXN..."
# Clé publique P-256 encodée SEC1 (33 octets compressée, ou 65 octets non compressée)
public_key = bytes.fromhex("04...")

result = claim169.decode_with_ecdsa_p256(qr_data, public_key)

print(f"ID: {result.claim169.id}")
print(f"Verified: {result.is_verified()}")
```

## Décoder avec un vérificateur personnalisé

Pour HSM, KMS, ou fournisseurs crypto personnalisés :

```python
import claim169
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PublicKey

# Charger votre clé publique
public_key_bytes = bytes.fromhex("d75a980182b10ab7...")
public_key = Ed25519PublicKey.from_public_bytes(public_key_bytes)

def my_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes):
    """Callback de vérification personnalisé.

    Args:
        algorithm: Nom d’algorithme (p. ex. "EdDSA", "ES256")
        key_id: Identifiant de clé optionnel depuis l’en-tête COSE
        data: Données signées
        signature: Signature à vérifier

    Raises:
        Une exception si la vérification échoue
    """
    # Vérifier via votre fournisseur crypto
    public_key.verify(bytes(signature), bytes(data))

result = claim169.decode_with_verifier(qr_data, my_verifier)
print(f"Verified: {result.is_verified()}")
```

## Décoder sans vérification

Uniquement pour les tests et le développement. Ne jamais utiliser en production.

```python
import claim169

# WARNING: INSECURE - ignore la vérification de signature
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

## Accéder aux données décodées

### DecodeResult

Les fonctions de décodage renvoient un objet `DecodeResult` :

```python
result = claim169.decode_with_ed25519(qr_data, public_key)

# Le claim d’identité décodé
claim = result.claim169

# Métadonnées CWT (issuer, timestamps)
meta = result.cwt_meta

# Statut de vérification sous forme de chaîne
status = result.verification_status  # "verified", "skipped", etc.

# Méthode helper
is_verified = result.is_verified()  # True/False
```

### Champs Claim169

```python
claim = result.claim169

# Démographie
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

# Biométrie (chacun est list[Biometric] | None)
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

# Méthodes helpers
claim.has_biometrics()  # True si une biométrie est présente
claim.to_dict()         # Convertir en dict Python
```

### Champs CwtMeta

```python
meta = result.cwt_meta

meta.issuer       # str | None - Émetteur de l’identifiant
meta.subject      # str | None - Identifiant du sujet
meta.expires_at   # int | None - Timestamp d’expiration
meta.not_before   # int | None - Timestamp « not before »
meta.issued_at    # int | None - Timestamp d’émission

# Méthodes helpers
meta.is_valid_now()  # True si le jeton est actuellement valide
meta.is_expired()    # True si le jeton est expiré
```

### Champs biométriques

```python
if result.claim169.face:
    face = result.claim169.face[0]

    face.data       # bytes - Données biométriques brutes
    face.format     # int | None - Code format
    face.sub_format # int | None - Code sous-format
    face.issuer     # str | None - Émetteur biométrique
```

## Gérer les horodatages

### Validation des horodatages

Par défaut, le décodeur valide les horodatages :

```python
# Lève une exception si le jeton est expiré ou pas encore valide
result = claim169.decode_with_ed25519(qr_data, public_key)
```

### Désactiver la validation des horodatages

```python
result = claim169.decode_with_ed25519(
    qr_data,
    public_key,
    validate_timestamps=False
)
```

### Tolérance à la dérive d’horloge

Pour des systèmes distribués avec des écarts d’horloge :

```python
result = claim169.decode_with_ed25519(
    qr_data,
    public_key,
    clock_skew_tolerance_seconds=60  # Autoriser 60 secondes de dérive
)
```

## Optimiser le décodage

### Ignorer la biométrie

Pour accélérer lorsque la biométrie n’est pas nécessaire :

```python
result = claim169.decode_with_ed25519(
    qr_data,
    public_key,
    skip_biometrics=True
)

# Les champs biométriques seront None
assert result.claim169.face is None
```

### Limiter la taille décompressée

Se protéger contre les bombes de décompression :

```python
result = claim169.decode_with_ed25519(
    qr_data,
    public_key,
    max_decompressed_bytes=32768  # Limite 32 KB
)
```

## Gestion des erreurs

```python
import claim169

try:
    result = claim169.decode_with_ed25519(qr_data, public_key)

except claim169.Base45DecodeError as e:
    # Encodage Base45 invalide
    print(f"QR code format error: {e}")

except claim169.DecompressError as e:
    # Échec de décompression zlib ou dépassement de taille
    print(f"Decompression error: {e}")

except claim169.CoseParseError as e:
    # Structure COSE invalide
    print(f"COSE parse error: {e}")

except claim169.CwtParseError as e:
    # Structure CWT invalide
    print(f"CWT parse error: {e}")

except claim169.Claim169NotFoundError as e:
    # Claim 169 absent du CWT
    print(f"Not a Claim 169 credential: {e}")

except claim169.SignatureError as e:
    # Échec de vérification de signature
    print(f"Invalid signature: {e}")

except claim169.Claim169Exception as e:
    # Erreur générique (classe de base)
    print(f"Decoding failed: {e}")
```

## Exemple complet

```python
import claim169

def verify_credential(qr_data: str, public_key: bytes) -> dict | None:
    """Vérifier et décoder un identifiant Claim 169.

    Returns:
        Claim décodé sous forme de dictionnaire, ou None si la vérification échoue.
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

## Étapes suivantes

- [Chiffrement](encryption.md) — décoder des identifiants chiffrés
- [Crypto personnalisée](custom-crypto.md) — intégration HSM/KMS
- [Référence API](api.md) — documentation complète des fonctions
