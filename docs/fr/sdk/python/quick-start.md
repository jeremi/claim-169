# Démarrage rapide

Ce guide couvre les opérations essentielles : encoder des identifiants et décoder des QR codes.

## Décoder un QR code

L’opération la plus courante est de décoder un QR code scanné depuis un identifiant.

!!! warning "Ne pas modifier la chaîne Base45"
    L’alphabet Base45 inclut un caractère espace (`" "`). Conservez le texte scanné tel quel (pas de `.strip()`, `.trim()`, ni normalisation des espaces), sinon vous risquez de corrompre des identifiants valides.

### Avec vérification Ed25519

```python
import claim169

# Contenu du QR code (chaîne Base45 depuis le scanner)
qr_data = "NCFOXN..."

# Clé publique Ed25519 de l’émetteur (32 octets)
public_key = bytes.fromhex(
    "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
)

# Décoder et vérifier
result = claim169.decode_with_ed25519(qr_data, public_key)

# Accéder aux données d’identité
print(f"ID: {result.claim169.id}")
print(f"Name: {result.claim169.full_name}")
print(f"Date of Birth: {result.claim169.date_of_birth}")

# Vérifier le statut
if result.is_verified():
    print("Signature verified successfully")
else:
    print(f"Verification status: {result.verification_status}")

# Accéder aux métadonnées CWT
print(f"Issuer: {result.cwt_meta.issuer}")
print(f"Expires: {result.cwt_meta.expires_at}")
```

### Vérifier la validité du jeton

```python
# Vérifier si le jeton est actuellement valide (pas expiré, pas avant nbf)
if result.cwt_meta.is_valid_now():
    print("Token is valid")
else:
    print("Token has expired or is not yet valid")

# Vérifier l’expiration spécifiquement
if result.cwt_meta.is_expired():
    print("Token has expired")
```

## Encoder un identifiant

Créer un identifiant signé qui peut être encodé dans un QR code.

### Encodage basique avec Ed25519

```python
import claim169

# Créer les données d’identité
claim = claim169.Claim169Input(
    id="MOSIP-2024-001",
    full_name="Jane Doe"
)
claim.date_of_birth = "1990-05-15"
claim.gender = 2  # Female
claim.email = "jane.doe@example.org"
claim.nationality = "US"

# Créer les métadonnées CWT
meta = claim169.CwtMetaInput(
    issuer="https://id.example.org",
    expires_at=1900000000  # Timestamp Unix
)
meta.issued_at = 1700000000

# Clé privée Ed25519 (32 octets) - gardez-la secrète !
private_key = bytes.fromhex(
    "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
)

# Encoder l’identifiant
qr_data = claim169.encode_with_ed25519(claim, meta, private_key)

print(f"QR Code content ({len(qr_data)} chars):")
print(qr_data)
```

### Exemple « roundtrip »

Encoder un identifiant puis le décoder immédiatement pour vérifier :

```python
import claim169

# Clés
private_key = bytes.fromhex(
    "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
)
public_key = bytes.fromhex(
    "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
)

# Créer et encoder
claim = claim169.Claim169Input(id="TEST-001", full_name="Test User")
meta = claim169.CwtMetaInput(
    issuer="https://test.example.org",
    expires_at=1900000000
)

qr_data = claim169.encode_with_ed25519(claim, meta, private_key)

# Décoder et vérifier
result = claim169.decode_with_ed25519(qr_data, public_key)

assert result.claim169.id == "TEST-001"
assert result.claim169.full_name == "Test User"
assert result.is_verified()
print("Roundtrip successful!")
```

## Travailler avec la biométrie

### Vérifier la présence de biométrie

```python
result = claim169.decode_with_ed25519(qr_data, public_key)

if result.claim169.has_biometrics():
    print("Credential contains biometric data")

    # Vérifier des types spécifiques
    if result.claim169.face:
        face = result.claim169.face[0]
        print(f"Face photo: {len(face.data)} bytes, format={face.format}")

    if result.claim169.right_thumb:
        thumb = result.claim169.right_thumb[0]
        print(f"Right thumb: {len(thumb.data)} bytes")
```

### Ignorer la biométrie (décodage plus rapide)

Pour des usages qui n’ont pas besoin de biométrie :

```python
# Ignorer le parsing biométrique pour accélérer
result = claim169.decode_with_ed25519(
    qr_data,
    public_key,
    skip_biometrics=True
)

# Les champs biométriques seront None
assert result.claim169.face is None
```

## Gestion des erreurs

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

## Conversion en dictionnaire

```python
result = claim169.decode_with_ed25519(qr_data, public_key)

# Convertir le claim en dict Python
claim_dict = result.claim169.to_dict()
print(claim_dict)
# {'id': 'TEST-001', 'fullName': 'Test User', ...}
```

## Étapes suivantes

- [Guide d’encodage](encoding.md) — encodage détaillé avec toutes les données démographiques
- [Guide de décodage](decoding.md) — options avancées de décodage
- [Chiffrement](encryption.md) — ajouter le chiffrement AES-GCM
- [Crypto personnalisée](custom-crypto.md) — intégration HSM/KMS
