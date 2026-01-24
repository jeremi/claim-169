# Chiffrement

Ce document explique quand et comment utiliser le chiffrement avec des identifiants Claim 169.

## Quand chiffrer

Le chiffrement ajoute une couche de confidentialité. Envisagez le chiffrement lorsque :

| Scénario | Recommandation |
|----------|----------------|
| QR affiché publiquement | Chiffrer |
| QR susceptible d’être photographié | Chiffrer |
| Contient de la biométrie | Chiffrer |
| Réglementations confidentialité applicables | Chiffrer |
| Environnement contrôlé uniquement | La signature suffit |
| Information publique uniquement | La signature suffit |

## Chiffrement vs signature

| Propriété | Signature | Chiffrement |
|----------|----------|-------------|
| **Authenticité** | ✓ Prouve qui l’a émis | ✗ Ne prouve pas l’origine |
| **Intégrité** | ✓ Détecte les altérations | ✓ Détecte les altérations |
| **Confidentialité** | ✗ Lisible par tous | ✓ Lisible seulement avec la clé |
| **Requis** | Toujours | Optionnel |

!!! important "Toujours signer"
    Le chiffrement ne remplace pas la signature. Signez toujours les identifiants, et chiffrez éventuellement.

## Ordre du chiffrement

Les identifiants sont d’abord signés, puis chiffrés :

```
Identity Data → Sign → Encrypt → Compress → Base45
```

Cet ordre garantit :

1. La signature couvre les données originales
2. Le chiffrement protège les données signées
3. Le vérificateur retrouve un contenu authentique après déchiffrement

## Algorithmes supportés

| Algorithme | Taille de clé | Niveau de sécurité |
|-----------|---------------|--------------------|
| AES-256-GCM | 32 octets | 256 bits |
| AES-128-GCM | 16 octets | 128 bits |

Ces algorithmes sont des AEAD (Authenticated Encryption with Associated Data), fournissant :

- Confidentialité (données chiffrées)
- Intégrité (altération détectée)

## Exigences de nonce

AES-GCM exige un nonce de 12 octets (vecteur d’initialisation) :

!!! danger "Ne jamais réutiliser des nonces"
    Réutiliser un nonce avec la même clé casse totalement la sécurité d’AES-GCM. Un attaquant peut :

    - Retrouver le flot de chiffrement (keystream)
    - Déchiffrer d’autres messages
    - Forger de nouveaux messages

### Générer des nonces

Utilisez le générateur aléatoire sûr de la bibliothèque :

```
generate_random_nonce()  // Renvoie 12 octets aléatoires
```

Ou le générateur aléatoire cryptographique de votre plateforme :

- Python : `secrets.token_bytes(12)`
- Node.js : `crypto.randomBytes(12)`
- Rust : `rand::thread_rng().gen::<[u8; 12]>()`

### Unicité des nonces

Assurez l’unicité via :

1. **Génération aléatoire** — 2^96 valeurs possibles, collision très improbable
2. **Compteur** — incrémenter un compteur à chaque chiffrement
3. **Horodatage + aléatoire** — combiner le temps avec des bits aléatoires

## Gestion des clés

### Distribution de clés symétriques

Contrairement à la signature (clé publique/privée), le chiffrement utilise des clés symétriques :

- La même clé chiffre et déchiffre
- La clé doit être partagée de façon sûre avec les vérificateurs
- Une compromission affecte tous les identifiants chiffrés avec cette clé

### Stratégies de distribution

| Stratégie | Cas d’usage |
|----------|-------------|
| Clés pré-partagées | Systèmes fermés, vérificateurs connus |
| Dérivation de clé | À partir d’un secret partagé ou d’un PIN |
| Enveloppement de clé | Encapsuler la clé symétrique via une clé publique |
| HSM/KMS | Gestion de clés en entreprise |

### Rotation de clés

Planifiez la rotation :

1. Inclure un identifiant de clé dans l’identifiant
2. Les vérificateurs maintiennent l’historique des clés
3. Retirer les anciennes clés selon calendrier
4. Les nouveaux identifiants utilisent la clé courante

## Modèle de menaces

### Ce que le chiffrement protège

| Menace | Protection |
|--------|------------|
| Observation occasionnelle | ✓ |
| Photographie du QR | ✓ |
| Interception réseau | ✓ |
| Stockage d’images de QR | ✓ |

### Ce que le chiffrement ne protège pas

| Menace | Pourquoi |
|--------|----------|
| Compromission de clé | Tous les identifiants sont déchiffrables |
| Abus par vérificateur autorisé | Il possède la clé |
| Partage par le porteur | Il peut montrer les données |
| Analyse de métadonnées | Le fait d’être chiffré reste visible |

## Fournisseurs de chiffrement personnalisés

Pour une intégration HSM ou KMS, implémentez les traits `Encryptor` et `Decryptor` :

### Interface Encryptor

```
encrypt(algorithm, key_id, nonce, aad, plaintext) → ciphertext
```

Paramètres :
- `algorithm` : algorithme COSE (A256GCM ou A128GCM)
- `key_id` : identifiant de clé optionnel
- `nonce` : IV de 12 octets
- `aad` : additional authenticated data
- `plaintext` : données à chiffrer

### Interface Decryptor

```
decrypt(algorithm, key_id, nonce, aad, ciphertext) → plaintext
```

Paramètres :
- `algorithm` : algorithme COSE
- `key_id` : identifiant de clé optionnel (depuis l’en-tête COSE)
- `nonce` : IV de 12 octets (depuis l’en-tête COSE)
- `aad` : additional authenticated data
- `ciphertext` : données chiffrées avec tag d’authentification

## Considérations de performance

| Opération | Coût relatif |
|-----------|--------------|
| Signature | Faible |
| Chiffrement | Faible |
| Signature + chiffrement | Faible |
| Dérivation de clé | Variable |
| Opérations HSM | Latence plus élevée |

AES-GCM est accéléré matériellement sur la plupart des plateformes : le surcoût est généralement minimal.

## Impact sur la taille

Le chiffrement ajoute un surcoût :

| Composant | Taille |
|-----------|--------|
| En-tête COSE_Encrypt0 | ~20 octets |
| Nonce | 12 octets |
| Tag d’authentification | 16 octets |
| **Surcoût total** | ~48 octets |

En général, c’est négligeable par rapport au contenu de l’identifiant.
