# Encoder des identifiants

Ce document explique le modèle conceptuel pour encoder des identifiants d’identité dans des QR codes.

## Pipeline d’encodage

L’encodage suit un pipeline multi-étapes :

```
Identity Data → CBOR → CWT → COSE_Sign1 → [COSE_Encrypt0] → zlib → Base45 → QR Code
```

Chaque étape a un rôle précis pour produire un identifiant compact, sûr et vérifiable.

!!! warning "Ne pas modifier la chaîne Base45"
    L’alphabet Base45 inclut un caractère espace (`" "`). Conservez la chaîne encodée exactement telle qu’elle est produite (ou scannée), sans `.trim()`, ni normalisation des espaces.

## 1. Données d’identité

Commencez par les données que vous voulez encoder. Au minimum, il faut :

- **Charge utile Claim 169** — champs d’identité (nom, date de naissance, photo, etc.)
- **Métadonnées CWT** — émetteur, sujet, horodatages

### Champs requis vs optionnels

Tous les champs Claim 169 sont optionnels. N’encodez que ce dont vous avez besoin :

| Minimal | Typique | Complet |
|---------|---------|---------|
| id | id, fullName, DOB | Toutes les données démographiques |
| | | + photo |
| | | + biométrie |

### Métadonnées CWT

| Champ | Rôle | Recommandation |
|-------|------|----------------|
| `issuer` | Qui a émis l’identifiant | Toujours renseigner |
| `subject` | À propos de qui | Optionnel |
| `issuedAt` | Date d’émission | Recommandé |
| `expiresAt` | Date d’expiration | Recommandé |
| `notBefore` | Début de validité | Optionnel |

## 2. Signature

Tous les identifiants doivent être signés pour permettre la vérification. Choisissez un algorithme :

### Ed25519 (recommandé)

- Signature et vérification rapides
- Signatures courtes (64 octets)
- Clés courtes (32 octets)
- Algorithme COSE : EdDSA (-8)

### ECDSA P-256

- Largement supporté
- Signatures de 64 octets
- Clé privée de 32 octets
- Algorithme COSE : ES256 (-7)

### Matériel de clé

Vous avez besoin d’une **clé privée** pour signer. La **clé publique** correspondante est distribuée aux vérificateurs.

| Algorithme | Clé privée | Clé publique |
|-----------|------------|-------------|
| Ed25519 | 32 octets | 32 octets |
| ECDSA P-256 | 32 octets | 33 octets (compressée) ou 65 octets (non compressée) |

## 3. Chiffrement (optionnel)

Chiffrez des identifiants lorsque la confidentialité est requise :

### Quand chiffrer

- Le QR code peut être photographié
- Contient des biométries sensibles
- Réglementation confidentialité applicable
- Identifiant partagé entre périmètres de confiance

### Algorithmes de chiffrement

| Algorithme | Taille de clé | Taille de nonce |
|-----------|---------------|-----------------|
| AES-256-GCM | 32 octets | 12 octets |
| AES-128-GCM | 16 octets | 12 octets |

### Ordre du chiffrement

Le chiffrement enveloppe l’identifiant signé :

```
Sign → Encrypt
```

Le vérificateur doit :
1. Déchiffrer avec la clé symétrique
2. Vérifier la signature avec la clé publique

### Exigences de nonce

!!! warning "Ne jamais réutiliser des nonces"
    Chaque chiffrement doit utiliser un nonce unique. Réutiliser des nonces avec la même clé casse la sécurité.

Utilisez `generate_random_nonce()` ou le générateur aléatoire sûr de votre plateforme.

## 4. Compression

La bibliothèque compresse la structure COSE avant l'encodage Base45. Par défaut, zlib (DEFLATE) est utilisé, conformément à la spécification Claim 169.

### Modes de compression

| Mode | Conforme spec | Description |
|------|:-:|-------------|
| `Zlib` | Oui | Par défaut. Compression standard zlib/DEFLATE |
| `None` | Non | Pas de compression. Utile pour les petites charges utiles où zlib ajoute du poids |
| `Adaptive` | Non | Choisit zlib si cela réduit la taille, sinon stocke brut |
| `Brotli(quality)` | Non | Compression Brotli avec qualité 0–11. Nécessite la feature `compression-brotli` |
| `AdaptiveBrotli(quality)` | Non | Choisit Brotli si cela réduit la taille, sinon stocke brut |

Les modes non standard (tout sauf Zlib) génèrent un avertissement `NonStandardCompression` dans le `EncodeResult`.

### Auto-détection au décodage

Le décodeur détecte automatiquement le format de compression utilisé, de sorte que les identifiants créés avec n'importe quel mode peuvent être décodés de manière transparente. Le format détecté est rapporté dans `DecodeResult.detected_compression`.

!!! warning "Interopérabilité"
    Les identifiants compressés avec un format non standard ne peuvent être décodés que par cette bibliothèque. Les autres décodeurs Claim 169 qui ne supportent que zlib les rejetteront. N'utilisez la compression non standard que dans des écosystèmes fermés où vous contrôlez à la fois l'encodeur et le décodeur.

## 5. Encodage Base45

Dernière étape : encoder les octets compressés en texte alphanumérique :

- Optimisé pour le mode alphanumérique des QR codes
- Plus efficace que Base64 pour les QR codes
- Produit des lettres majuscules et des chiffres

## Considérations de taille

La capacité d’un QR code limite ce que vous pouvez encoder :

| Version QR | Capacité alphanumérique |
|------------|--------------------------|
| 10 | 395 caractères |
| 20 | 1 249 caractères |
| 30 | 2 520 caractères |
| 40 | 4 296 caractères |

### Conseils d’optimisation

1. **Inclure uniquement les champs nécessaires** — omettez les champs inutilisés
2. **Compresser les photos** — utilisez JPEG ou AVIF, réduisez la résolution
3. **Limiter la biométrie** — incluez uniquement l’essentiel
4. **Ignorer la biométrie** — utilisez `skip_biometrics()` pour des codes plus petits

## Pattern « builder » de l'encodeur

Tous les SDKs suivent un pattern de type builder pour l'encodage :

1. Créer l'encodeur avec la donnée Claim et les métadonnées CWT
2. Configurer la signature (requis)
3. Configurer le chiffrement (optionnel)
4. Configurer la compression (optionnel, zlib par défaut)
5. Appeler `encode()` pour produire le résultat

### Résultat d'encodage

`encode()` renvoie un `EncodeResult` (ou équivalent dans chaque SDK) contenant :

| Champ | Description |
|-------|-------------|
| `qr_data` | La chaîne encodée en Base45 pour la génération du QR code |
| `compression_used` | Quelle compression a été appliquée (`Zlib`, `Brotli`, ou `None`) |
| `warnings` | Avertissements non fatals (p. ex. `NonStandardCompression`) |

Voir les guides d'encodage spécifiques à chaque SDK pour des exemples d'implémentation.

## Gestion des erreurs

L’encodage peut échouer pour plusieurs raisons :

| Erreur | Cause |
|-------|-------|
| Format de clé invalide | Longueur/format des octets de clé incorrect |
| Échec de signature | Échec d’opération crypto |
| Échec de chiffrement | Échec d’opération crypto |
| Échec d’encodage CBOR | Structure de données invalide |

Gérez ces erreurs correctement dans votre application.
