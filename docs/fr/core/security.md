# Sécurité

Ce document décrit le modèle de sécurité, les menaces atténuées et les valeurs sûres implémentées dans la bibliothèque Claim 169.

## Sécurisé par défaut

La bibliothèque applique des bonnes pratiques de sécurité par défaut :

| Protection | Par défaut | Contournement |
|------------|------------|---------------|
| Vérification de signature | Requise | `allow_unverified()` |
| Validation des horodatages | Activée (Rust/Python + TypeScript côté hôte) | Rust/Python : `without_timestamp_validation()` ; TypeScript : `withoutTimestampValidation()` / `validateTimestamps: false` |
| Limite de décompression | 64 KB | `max_decompressed_bytes()` |
| Profondeur d’imbrication CBOR | 128 niveaux | Non configurable |
| Confusion d’algorithme | Empêchée | Algorithme lu uniquement depuis l’en-tête COSE |

## Vérification de signature

### Pourquoi la vérification est requise

Tous les identifiants doivent être vérifiés cryptographiquement pour éviter :

- **Falsification** — création de faux identifiants
- **Altération** — modification d’identifiants légitimes
- **Confiance implicite** — considérer des entrées non fiables comme authentiques

### Sélection de l’algorithme

La bibliothèque impose la sélection d’algorithme uniquement depuis l’en-tête COSE protégé :

- **Aucun algorithme par défaut** — il faut choisir le type de vérificateur
- **Aucune négociation d’algorithme** — l’algorithme vient de l’en-tête
- **Aucun repli sur un algorithme faible** — seulement Ed25519 et ECDSA P-256

### Rejet des clés faibles

La bibliothèque rejette des clés connues comme faibles :

- **Ed25519** : points d’ordre faible (identité, sous-groupes d’ordre faible)
- **ECDSA P-256** : point identité (point à l’infini)

## Sécurité de la décompression

### Bombes de décompression

Des QR codes malveillants peuvent contenir des données qui se décompressent en tailles énormes. La bibliothèque s’en protège :

```
Limite par défaut : 64 KB (65 536 octets)
Configurable : max_decompressed_bytes()
```

Si les données décompressées dépassent la limite, le décodage échoue avec `DecompressLimitExceeded`.

### Pourquoi 64 KB ?

- Suffisant pour la plupart des identifiants avec photos
- Assez faible pour éviter l’épuisement mémoire
- N’augmentez que si vous devez inclure de très grosses biométries

## Validation des horodatages

### Horodatages de claim

Les identifiants CWT supportent trois horodatages :

| Claim | Clé | Validation |
|-------|-----|------------|
| `exp` | 4 | Doit être dans le futur |
| `nbf` | 5 | Doit être dans le passé |
| `iat` | 6 | Informatif uniquement |

!!! note "Ce que les horodatages protègent (et ne protègent pas)"
    La validation des horodatages empêche d’accepter des identifiants **expirés** et des identifiants **pas encore valides**.
    Elle n’empêche **pas** un attaquant de rejouer un identifiant encore dans sa fenêtre de validité.

### Tolérance à la dérive d’horloge

Les systèmes réels ont de la dérive. Configurez une tolérance :

```
Par défaut : 0 seconde
Typique : 300 secondes (5 minutes)
```

### Désactiver la validation

Pour les tests, ou lorsque les horodatages sont gérés ailleurs :

- Rust : `without_timestamp_validation()`
- Python : `without_timestamp_validation()`
- TypeScript : désactivé par défaut (horloges navigateur peu fiables)

## Rejeu & révocation

Les QR codes Claim 169 sont souvent utilisés en contexte **hors ligne**. Dans ce cas, certaines protections dépendent nécessairement de votre application :

- **Rejeu d’identifiants encore valides** : cette bibliothèque ne maintient pas de cache « identifiant déjà vu ». Si vous avez besoin d’anti-rejeu, implémentez-le côté application (p. ex. identifiants à courte durée de vie, liaison transaction/challenge, listes locales, ou contrôles en ligne si disponibles).
- **Révocation / statut d’identifiant** : cette bibliothèque ne définit ni n’impose de mécanisme de révocation. Si vous en avez besoin, intégrez les contrôles de statut de votre écosystème ou des listes allow/block.

## Sécurité CBOR

### Limite de profondeur d’imbrication

Des structures CBOR profondément imbriquées peuvent provoquer un dépassement de pile. La bibliothèque limite la profondeur à 128 niveaux.

### Champs inconnus

Les clés CBOR inconnues sont conservées dans `unknown_fields` pour la compatibilité ascendante. Cela permet :

- De nouvelles versions de la spécification avec des champs additionnels
- Des extensions spécifiques à un fournisseur
- Une dégradation maîtrisée

## Considérations sur le chiffrement

### Quand chiffrer

Chiffrez lorsque :

- Le QR code peut être photographié ou partagé
- L’identifiant contient des biométries sensibles
- Des exigences réglementaires imposent la protection des données

### Gestion des clés

- **Ne jamais hardcoder des clés** — utilisez un stockage sécurisé
- **Rotation régulière** — définissez des politiques de rotation
- **Nonces uniques** — ne réutilisez jamais un nonce avec la même clé

### Exigences de nonce

AES-GCM exige des nonces uniques de 12 octets :

- Utilisez `generate_random_nonce()` pour de nouveaux chiffrages
- Ne réutilisez jamais de nonces avec la même clé
- Réutiliser un nonce casse la confidentialité

## Fournisseurs crypto personnalisés

Lors de l’intégration avec un HSM ou un KMS :

### Isolation des clés

- Gardez les clés privées dans le HSM/KMS
- N’exportez que les clés publiques pour la vérification
- Utilisez des IDs de clé dans les en-têtes COSE pour la résolution de clé

### Gestion des erreurs

Les fournisseurs personnalisés doivent retourner des erreurs appropriées :

- `CryptoError::KeyNotFound` — ID de clé absent du keystore
- `CryptoError::UnsupportedAlgorithm` — algorithme non supporté
- `CryptoError::VerificationFailed` — signature invalide

## Modèle de menaces

### Dans le périmètre

| Menace | Atténuation |
|--------|------------|
| Falsification d’identifiant | Vérification de signature |
| Altération d’identifiant | Vérification de signature |
| Application exp/nbf | Validation des horodatages (`exp`/`nbf`) |
| Épuisement mémoire | Limites de décompression |
| Confusion d’algorithme | Algorithme uniquement depuis l’en-tête |
| Clés faibles | Validation des clés |
| Fuite de confidentialité | Chiffrement optionnel |

### Hors périmètre

| Menace | Raison |
|--------|--------|
| Compromission de clé | Responsabilité applicative |
| Rejeu d’identifiants encore valides | Responsabilité applicative |
| Révocation / statut d’identifiant | Responsabilité applicative |
| Attaques par canaux auxiliaires | Dépend de l’implémentation crypto |
| Sécurité physique du QR code | Responsabilité applicative |
| Distribution de clés | Responsabilité applicative |
