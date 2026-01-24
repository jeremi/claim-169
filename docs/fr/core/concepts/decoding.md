# Décodage & vérification

Ce document explique le modèle conceptuel pour décoder et vérifier des identifiants Claim 169.

## Pipeline de décodage

Le décodage inverse le pipeline d’encodage :

```
QR Code → Base45 → zlib → COSE → CWT → Claim 169
```

À chaque étape, la bibliothèque valide la structure et applique des contrôles de sécurité.

## Modèle de vérification

### Décisions de confiance

Lors du décodage d’un identifiant, vous devez prendre des décisions de confiance :

| Question | Réponse |
|----------|---------|
| **Qui a émis ceci ?** | Vérifier le claim `issuer`, utiliser la bonne clé publique |
| **Est-ce authentique ?** | Vérifier la signature avec la clé publique de l’émetteur |
| **Est-ce à jour ?** | Valider les horodatages `exp` et `nbf` |
| **Est-ce chiffré ?** | Fournir la clé de déchiffrement si nécessaire |

### Statut de vérification

Après décodage, vérifiez le statut :

| Statut | Signification |
|--------|---------------|
| `Verified` | Signature valide avec la clé fournie |
| `Unverified` | Décodé sans vérification (tests uniquement) |
| Erreur | Signature invalide ou échec de vérification |

## Vérification de signature

### Pourquoi la vérification est requise

La bibliothèque exige la vérification par défaut car :

- Des identifiants non vérifiés peuvent être falsifiés
- Un attaquant peut modifier des identifiants légitimes
- Les hypothèses de confiance doivent être explicites

### Choisir un vérificateur

Sélectionnez le vérificateur correspondant à la clé de l’émetteur :

| Si l’émetteur utilise… | Utiliser le vérificateur… |
|------------------------|--------------------------|
| Ed25519 | `verify_with_ed25519(public_key)` |
| ECDSA P-256 | `verify_with_ecdsa_p256(public_key)` |
| HSM/KMS | `verify_with(custom_verifier)` |

### Résolution via Key ID

Les en-têtes COSE peuvent inclure un identifiant de clé (`kid`). Utilisez-le pour :

1. Retrouver la bonne clé publique dans un keystore
2. Router la vérification vers la bonne clé HSM
3. Supporter la rotation de clés

## Validation des horodatages

### Horodatages

Les identifiants CWT supportent une validité temporelle :

| Claim | Validation |
|-------|------------|
| `exp` (expiration) | Doit être dans le futur |
| `nbf` (not before) | Doit être dans le passé |
| `iat` (issued at) | Informatif uniquement |

### Dérive d’horloge

Les systèmes réels ont des écarts d’horloge. Configurez une tolérance :

```
Par défaut : 0 seconde (strict)
Typique : 300 secondes (5 minutes)
```

### Désactiver la validation

Pour les tests ou si les horodatages sont gérés ailleurs :

- Utiliser `without_timestamp_validation()`
- WASM/TypeScript : désactivé par défaut (horloges navigateur peu fiables)

## Déchiffrement

### Détecter le chiffrement

La bibliothèque détecte automatiquement les identifiants chiffrés (enveloppe COSE_Encrypt0).

### Fournir la clé de déchiffrement

Si l’identifiant est chiffré, fournissez la clé symétrique :

| Algorithme | Taille de clé | Méthode |
|-----------|---------------|---------|
| AES-256-GCM | 32 octets | `decrypt_with_aes256(key)` |
| AES-128-GCM | 16 octets | `decrypt_with_aes128(key)` |

### Ordre de déchiffrement

Pour un identifiant chiffré :

1. Déchiffrer le COSE_Encrypt0 externe
2. Vérifier la signature du COSE_Sign1 interne
3. Parser le CWT et la charge utile Claim 169

## Pattern « builder » du décodeur

Tous les SDKs suivent un pattern de type builder pour le décodage :

1. Créer le décodeur avec la chaîne QR
2. Configurer la vérification (requis sauf opt-out)
3. Configurer le déchiffrement (si nécessaire)
4. Appeler `decode()` pour obtenir le résultat

### Configuration requise

Vous devez configurer la vérification avant le décodage :

```
✓ decoder.verify_with_ed25519(key).decode()
✓ decoder.allow_unverified().decode()  // Tests uniquement !
✗ decoder.decode()  // Erreur : vérification non configurée
```

## Résultat de décodage

Un décodage réussi retourne :

| Champ | Contenu |
|-------|---------|
| `claim169` | Données d’identité (id, nom, date de naissance, etc.) |
| `cwt_meta` | Métadonnées du jeton (issuer, horodatages) |
| `verification_status` | `Verified` ou `Unverified` |

## Gestion des erreurs

Le décodage peut échouer à différents stades :

| Étape | Erreurs possibles |
|------|-------------------|
| Base45 | Encodage invalide, mauvais caractères |
| Décompression | Données corrompues, dépassement de limite |
| COSE | Structure invalide, type inconnu |
| Signature | Échec de vérification, mauvaise clé |
| Déchiffrement | Mauvaise clé, ciphertext corrompu |
| CWT | Claims invalides, Claim 169 absent |
| Horodatages | Expiré, pas encore valide |

### Catégories d’erreurs

| Erreur | Action utilisateur |
|--------|-------------------|
| `Base45Decode` | Données QR corrompues ou tronquées |
| `Decompress` | Données corrompues |
| `DecompressLimitExceeded` | Augmenter la limite ou rejeter |
| `SignatureInvalid` | Mauvaise clé ou données altérées |
| `DecryptionFailed` | Mauvaise clé de déchiffrement |
| `Expired` | Identifiant expiré |
| `NotYetValid` | Identifiant pas encore actif |

## Considérations de sécurité

### Entrées non fiables

Traitez toutes les données QR comme non fiables :

- Valider avant traitement
- Gérer les erreurs proprement
- Ne pas faire confiance aux claims avant vérification

### Gestion des clés

- Stocker les clés publiques de manière sûre
- Associer les clés aux émetteurs
- Supporter la rotation via `kid`

### Limites de décompression

Se protéger des bombes de décompression :

- Limite par défaut : 64 KB
- N’augmenter que si nécessaire
- Tenir compte des contraintes mémoire

