# Chiffrement

Ce guide couvre le chiffrement des identifiants avec AES-GCM pour une protection supplémentaire de la confidentialité.

## Quand utiliser le chiffrement

Chiffrez des identifiants lorsque :

- Les QR codes peuvent être photographiés par des tiers
- Les identifiants contiennent des données biométriques sensibles
- Des réglementations imposent une protection des données
- Les identifiants sont partagés entre périmètres de confiance

## Vue d’ensemble du chiffrement

La bibliothèque supporte le **sign-then-encrypt** : l’identifiant est d’abord signé, puis la charge utile signée est chiffrée.

```
Identity Data -> Sign -> Encrypt -> Compress -> Base45 -> QR Code
```

Le déchiffrement inverse le processus :

```
QR Code -> Base45 -> Decompress -> Decrypt -> Verify -> Identity Data
```

## Algorithmes supportés

| Algorithme | Taille de clé | Taille de nonce | Cas d’usage |
|-----------|---------------|-----------------|------------|
| AES-256-GCM | 32 octets | 12 octets | Haute sécurité (recommandé) |
| AES-128-GCM | 16 octets | 12 octets | Sécurité standard |

## Encoder avec chiffrement

### Signer + chiffrer avec AES-256-GCM

```python
import claim169

# Données d’identité
claim = claim169.Claim169Input(id="ENC-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(
    issuer="https://id.example.org",
    expires_at=1900000000
)

# Clés
sign_key = bytes.fromhex(
    "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
)  # Clé privée Ed25519 (32 octets)

encrypt_key = bytes.fromhex(
    "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
)  # Clé AES-256 (32 octets)

# Encoder avec signature et chiffrement
qr_data = claim169.encode(
    claim,
    meta,
    sign_with_ed25519=sign_key,
    encrypt_with_aes256=encrypt_key,
)

print(f"Encrypted credential: {len(qr_data)} characters")
```

### Signer + chiffrer avec AES-128-GCM

```python
import claim169

claim = claim169.Claim169Input(id="ENC128-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(
    issuer="https://id.example.org",
    expires_at=1900000000
)

sign_key = bytes(32)  # Clé privée Ed25519
encrypt_key = bytes(16)  # Clé AES-128 (16 octets)

qr_data = claim169.encode(
    claim,
    meta,
    sign_with_ed25519=sign_key,
    encrypt_with_aes128=encrypt_key,
)
```

## Décoder des identifiants chiffrés

### Déchiffrer AES-256-GCM avec vérification

Pour des identifiants chiffrés, il faut généralement :

1. La clé de déchiffrement (clé AES symétrique)
2. La clé de vérification (clé publique de signature) ou un callback de vérification

```python
import claim169
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PublicKey

# Clés
encrypt_key = bytes.fromhex(
    "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
)
public_key_bytes = bytes.fromhex(
    "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
)

# Créer un callback de vérification
public_key = Ed25519PublicKey.from_public_bytes(public_key_bytes)

def verify_callback(algorithm, key_id, data, signature):
    public_key.verify(signature, data)

# Décoder et vérifier
result = claim169.decode_encrypted_aes(
    qr_data,
    encrypt_key,
    verifier=verify_callback
)

print(f"ID: {result.claim169.id}")
print(f"Verified: {result.is_verified()}")
```

### Déchiffrer AES-256 (taille de clé validée)

Utilisez `decode_encrypted_aes256()` si vous voulez garantir une clé de 32 octets :

```python
result = claim169.decode_encrypted_aes256(
    qr_data,
    encrypt_key,  # Doit faire exactement 32 octets
    verifier=verify_callback
)
```

### Déchiffrer AES-128 (taille de clé validée)

Utilisez `decode_encrypted_aes128()` pour des clés de 16 octets :

```python
encrypt_key_128 = bytes(16)  # Clé AES-128

result = claim169.decode_encrypted_aes128(
    qr_data,
    encrypt_key_128,  # Doit faire exactement 16 octets
    verifier=verify_callback
)
```

### Déchiffrer sans vérification de signature

Tests uniquement. Utilisez `allow_unverified=True` :

```python
# WARNING: INSECURE - ignore la vérification de signature
result = claim169.decode_encrypted_aes(
    qr_data,
    encrypt_key,
    allow_unverified=True
)

print(f"Status: {result.verification_status}")  # "skipped"
```

## Déchiffrement personnalisé

Pour un déchiffrement HSM ou KMS, utilisez un callback :

```python
import claim169
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

# Clé de déchiffrement (stockée en HSM/KMS)
aes_key = bytes(32)

def my_decryptor(algorithm, key_id, nonce, aad, ciphertext):
    """Callback de déchiffrement personnalisé.

    Args:
        algorithm: Nom d’algorithme (p. ex. "A256GCM", "A128GCM")
        key_id: Identifiant de clé optionnel depuis l’en-tête COSE
        nonce: Nonce de 12 octets
        aad: Additional authenticated data
        ciphertext: Données chiffrées avec tag d’authentification

    Returns:
        Octets du texte clair
    """
    aesgcm = AESGCM(aes_key)
    return aesgcm.decrypt(nonce, ciphertext, aad)

def my_verifier(algorithm, key_id, data, signature):
    # Votre logique de vérification
    pass

result = claim169.decode_with_decryptor(
    qr_data,
    my_decryptor,
    verifier=my_verifier
)
```

## Chiffrement personnalisé

Pour un chiffrement HSM ou KMS lors de l’encodage :

```python
import claim169
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

aes_key = bytes(32)

def my_encryptor(algorithm, key_id, nonce, aad, plaintext):
    """Callback de chiffrement personnalisé.

    Args:
        algorithm: Nom d’algorithme (p. ex. "A256GCM", "A128GCM")
        key_id: Identifiant de clé optionnel
        nonce: Nonce de 12 octets
        aad: Additional authenticated data
        plaintext: Données à chiffrer

    Returns:
        Ciphertext avec tag d’authentification
    """
    aesgcm = AESGCM(aes_key)
    return aesgcm.encrypt(nonce, plaintext, aad)

# Signature logicielle avec chiffrement personnalisé
sign_key = bytes(32)  # Clé privée Ed25519

qr_data = claim169.encode_with_encryptor(
    claim,
    meta,
    sign_key,
    my_encryptor,
    "A256GCM"  # ou "A128GCM"
)
```

## Crypto personnalisée complète

Utiliser des callbacks personnalisés pour la signature et le chiffrement :

```python
import claim169
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

# Générer des clés
sign_private = Ed25519PrivateKey.generate()
sign_public_bytes = sign_private.public_key().public_bytes_raw()
aes_key = bytes(32)

def my_signer(algorithm, key_id, data):
    return sign_private.sign(data)

def my_encryptor(algorithm, key_id, nonce, aad, plaintext):
    return AESGCM(aes_key).encrypt(nonce, plaintext, aad)

# Encoder avec les deux callbacks
claim = claim169.Claim169Input(id="CUSTOM-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(issuer="https://id.example.org")

qr_data = claim169.encode_with_signer_and_encryptor(
    claim,
    meta,
    my_signer,
    "EdDSA",      # Sign algorithm
    my_encryptor,
    "A256GCM"     # Encrypt algorithm
)

# Décoder avec les callbacks
def my_verifier(algorithm, key_id, data, signature):
    sign_private.public_key().verify(signature, data)

def my_decryptor(algorithm, key_id, nonce, aad, ciphertext):
    return AESGCM(aes_key).decrypt(nonce, ciphertext, aad)

result = claim169.decode_with_decryptor(
    qr_data,
    my_decryptor,
    verifier=my_verifier
)

print(f"ID: {result.claim169.id}")
print(f"Verified: {result.is_verified()}")
```

## Générer des clés

### Clé AES-256

```python
import secrets

aes_256_key = secrets.token_bytes(32)
print(f"AES-256 key: {aes_256_key.hex()}")
```

### Clé AES-128

```python
import secrets

aes_128_key = secrets.token_bytes(16)
print(f"AES-128 key: {aes_128_key.hex()}")
```

## Formats de clé supportés

Les clés AES peuvent être fournies sous différents formats :

### Format hexadécimal

```python
# AES-256 (64 caractères hex)
key_hex = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
key = bytes.fromhex(key_hex)

# AES-128 (32 caractères hex)
key_hex_128 = "0123456789abcdef0123456789abcdef"
key_128 = bytes.fromhex(key_hex_128)
```

### Format Base64

```python
import base64

# AES-256 (44 caractères Base64)
key_b64 = "ASNFZ4mrze8BI0VniavN7wEjRWeJq83vASNFZ4mrze8="
key = base64.b64decode(key_b64)

# AES-128 (24 caractères Base64)
key_b64_128 = "ASNFZ4mrze8BI0VniavN7w=="
key_128 = base64.b64decode(key_b64_128)
```

Le playground détecte automatiquement le format (hex ou Base64) lors de la saisie de clés de chiffrement.

### Nonce aléatoire

La bibliothèque génère des nonces automatiquement, mais vous pouvez aussi en générer :

```python
import claim169

nonce = claim169.generate_nonce()
print(f"Nonce: {nonce.hex()}")  # 12 bytes
```

## Gestion des erreurs

```python
import claim169

try:
    result = claim169.decode_encrypted_aes256(qr_data, encrypt_key, allow_unverified=True)

except claim169.DecryptionError as e:
    # Échec de déchiffrement (mauvaise clé, données corrompues, etc.)
    print(f"Decryption failed: {e}")

except ValueError as e:
    # Taille de clé invalide
    print(f"Invalid key: {e}")

except claim169.Claim169Exception as e:
    # Autres erreurs
    print(f"Error: {e}")
```

## Bonnes pratiques de sécurité

### Gestion des clés

- **Ne jamais hardcoder des clés** dans le code
- **Utiliser un stockage sécurisé** (HSM, KMS, secret manager)
- **Faire tourner les clés** périodiquement
- **Limiter l’accès aux clés** aux seuls systèmes autorisés

### Exigences de nonce

- **Ne jamais réutiliser des nonces** avec la même clé
- La bibliothèque génère des nonces aléatoires automatiquement
- Pour un chiffrement personnalisé, utilisez toujours des nonces aléatoires cryptographiquement sûrs

### Distribution de clés

- Distribuer les clés de chiffrement via des canaux sûrs
- Envisager des KDF pour des secrets partagés
- Mettre en place des protocoles d’échange de clés pour des systèmes distribués

## Étapes suivantes

- [Crypto personnalisée](custom-crypto.md) — exemples d’intégration HSM/KMS
- [Référence API](api.md) — documentation complète des fonctions
