# Gestion des clés

Ce document couvre les formats de clés, la génération, le stockage et la rotation pour des identifiants Claim 169.

## Types de clés

### Clés de signature (asymétriques)

Utilisées pour signer et vérifier des identifiants :

| Algorithme | Clé privée | Clé publique | Usage |
|-----------|------------|--------------|------|
| Ed25519 | 32 octets | 32 octets | Signature |
| ECDSA P-256 | 32 octets | 33 ou 65 octets | Signature |

### Clés de chiffrement (symétriques)

Utilisées pour chiffrer des identifiants :

| Algorithme | Taille de clé | Usage |
|-----------|---------------|------|
| AES-256-GCM | 32 octets | Chiffrement |
| AES-128-GCM | 16 octets | Chiffrement |

## Formats de clés

### Ed25519

| Type | Format | Taille |
|------|--------|--------|
| Privée | Octets bruts | 32 octets |
| Publique | Octets bruts | 32 octets |
| Publique | PEM (SPKI) | ~110 caractères |

Les clés Ed25519 peuvent être fournies sous forme de :

- **Octets bruts** — 32 octets en chaîne hexadécimale (64 caractères hex)
- **Format PEM** — Encodées en SPKI, avec l'en-tête `-----BEGIN PUBLIC KEY-----`

Exemple de format PEM :
```
-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEA11qYAYKxCrfVS/7TyWQHOg7hcvPapjJa8CCWX4cBURo=
-----END PUBLIC KEY-----
```

### ECDSA P-256

| Type | Format | Taille |
|------|--------|--------|
| Privée | Scalaire brut | 32 octets |
| Publique (compressée) | SEC1 compressé | 33 octets |
| Publique (non compressée) | SEC1 non compressé | 65 octets |
| Publique | PEM (SPKI) | ~170 caractères |

Les clés publiques peuvent être fournies sous forme de :

- **SEC1 hex** — Commence par `02`/`03` (compressé, 33 octets) ou `04` (non compressé, 65 octets)
- **Format PEM** — Encodées en SPKI, avec l'en-tête `-----BEGIN PUBLIC KEY-----`

Exemple de format PEM :
```
-----BEGIN PUBLIC KEY-----
MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE...
-----END PUBLIC KEY-----
```

### Clés AES

| Algorithme | Format | Taille |
|-----------|--------|--------|
| AES-256-GCM | Octets bruts ou Base64 | 32 octets |
| AES-128-GCM | Octets bruts ou Base64 | 16 octets |

Les clés AES peuvent être fournies sous forme de :

- **Chaîne hex** — 64 caractères hex pour AES-256, 32 pour AES-128
- **Chaîne Base64** — 44 caractères pour AES-256, 24 pour AES-128

Exemples de formats :
```
# Hex (AES-256)
0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef

# Base64 (AES-256)
ASNFZ4mrze8BI0VniavN7wEjRWeJq83vASNFZ4mrze8=
```

## Génération de clés

### Générer des clés de signature

Utilisez une génération aléatoire cryptographiquement sûre :

=== "Ligne de commande"

    ```bash
    # Ed25519
    openssl genpkey -algorithm ED25519 -out private.pem
    openssl pkey -in private.pem -pubout -out public.pem

    # ECDSA P-256
    openssl ecparam -name prime256v1 -genkey -out private.pem
    openssl ec -in private.pem -pubout -out public.pem
    ```

=== "Python"

    ```python
    from cryptography.hazmat.primitives.asymmetric import ed25519, ec
    from cryptography.hazmat.primitives import serialization

    # Ed25519
    private_key = ed25519.Ed25519PrivateKey.generate()
    public_key = private_key.public_key()
    private_bytes = private_key.private_bytes_raw()  # 32 octets
    public_bytes = public_key.public_bytes_raw()     # 32 octets

    # ECDSA P-256
    private_key = ec.generate_private_key(ec.SECP256R1())
    public_key = private_key.public_key()
    ```

=== "Node.js"

    ```javascript
    const crypto = require('crypto');

    // Ed25519
    const { publicKey, privateKey } = crypto.generateKeyPairSync('ed25519');

    // ECDSA P-256
    const { publicKey, privateKey } = crypto.generateKeyPairSync('ec', {
      namedCurve: 'prime256v1'
    });
    ```

### Générer des clés de chiffrement

```bash
# 32 octets pour AES-256
openssl rand 32 > aes256.key

# 16 octets pour AES-128
openssl rand 16 > aes128.key
```

## Stockage des clés

### Principes

1. **Ne jamais hardcoder des clés** — utiliser des variables d’environnement ou un stockage sécurisé
2. **Chiffrer au repos** — protéger les clés stockées
3. **Limiter les accès** — principe du moindre privilège
4. **Auditer les accès** — journaliser l’usage des clés

### Options de stockage

| Option | Sécurité | Cas d’usage |
|--------|----------|-------------|
| Variables d’environnement | Moyenne | Développement, conteneurs |
| Fichiers chiffrés | Moyenne | Déploiements simples |
| Gestionnaires de secrets | Élevée | Déploiements cloud |
| HSM/KMS | Très élevée | Entreprise, régulé |

### Exemple : variables d’environnement

```bash
export CLAIM169_PRIVATE_KEY="$(cat private.key | xxd -p | tr -d '\n')"
export CLAIM169_PUBLIC_KEY="$(cat public.key | xxd -p | tr -d '\n')"
```

## IDs de clé

COSE supporte des identifiants de clé (`kid`) dans les en-têtes :

### Objectif

- **Sélection de clé** — le vérificateur choisit la bonne clé
- **Rotation de clés** — supporter plusieurs clés actives
- **Multi-émetteur** — différentes clés pour différents émetteurs

### Format

Les IDs de clé sont des chaînes d’octets arbitraires. Formats courants :

| Format | Exemple |
|--------|---------|
| UUID | `550e8400-e29b-41d4-a716-446655440000` |
| Hash de la clé publique | 8 premiers octets de SHA-256 |
| Séquentiel | `key-001`, `key-002` |
| Basé sur une date | `2024-01-15-primary` |

### Renseigner un Key ID

À l’encodage, le signataire peut spécifier un key ID inclus dans l’en-tête COSE. Les vérificateurs l’utilisent pour retrouver la bonne clé publique.

## Rotation de clés

### Pourquoi faire tourner les clés

- Limiter l’exposition en cas de compromission
- Respecter des politiques de sécurité
- Retirer des algorithmes obsolètes

### Stratégie de rotation

1. **Générer une nouvelle paire de clés**
2. **Distribuer la nouvelle clé publique** aux vérificateurs
3. **Commencer à signer** avec la nouvelle clé
4. **Conserver l’ancienne clé publique** pour vérifier les identifiants existants
5. **Retirer l’ancienne clé** une fois les identifiants expirés

### Chevauchement de validité

Pendant la rotation, les deux clés peuvent être valides :

```
Ancienne clé : ████████████░░░░░░░░
Nouvelle clé : ░░░░░░░████████████████
              ^        ^
              Début    Fin
              rotation rotation
```

## Intégration HSM/KMS

Pour les déploiements à haute sécurité, utilisez une gestion de clés matérielle ou cloud.

### Avantages

- Les clés privées peuvent rester dans du matériel sécurisé (selon votre fournisseur et configuration)
- Contrôles d’accès imposés matériellement
- Journalisation/audit
- Possibilité de conformité (p. ex. FIPS/Common Criteria) selon configuration et audits

### Points d’intégration

La bibliothèque supporte des fournisseurs crypto personnalisés :

| Trait | Rôle |
|-------|-----|
| `SignatureVerifier` | Vérification de signature personnalisée |
| `Signer` | Signature personnalisée |
| `Decryptor` | Déchiffrement personnalisé |
| `Encryptor` | Chiffrement personnalisé |

### Exemples de KMS cloud

| Fournisseur | Service |
|------------|---------|
| AWS | AWS KMS, CloudHSM |
| Google Cloud | Cloud KMS, Cloud HSM |
| Azure | Key Vault, Managed HSM |

Voir les guides « custom crypto » des SDKs pour des détails d’implémentation.

## Checklist de sécurité

- [ ] Clés générées via aléa cryptographiquement sûr
- [ ] Clés privées stockées de façon sûre
- [ ] Clés publiques distribuées aux vérificateurs
- [ ] Plan de rotation en place
- [ ] IDs de clé utilisés pour scénarios multi-clés
- [ ] Accès aux clés audités
- [ ] Procédures de sauvegarde et restauration
