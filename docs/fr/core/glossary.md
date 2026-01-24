# Glossaire

Termes et concepts utilisés dans la spécification et la documentation Claim 169.

## Standards & protocoles

### CBOR
**Concise Binary Object Representation** — Format de données binaire conçu pour minimiser la taille du code et des messages. Similaire à JSON mais plus compact. Défini dans la [RFC 8949](https://www.rfc-editor.org/rfc/rfc8949).

### COSE
**CBOR Object Signing and Encryption** — Cadre pour signer et chiffrer des données CBOR. Fournit `COSE_Sign1` pour les signatures et `COSE_Encrypt0` pour le chiffrement. Défini dans la [RFC 9052](https://www.rfc-editor.org/rfc/rfc9052).

### CWT
**CBOR Web Token** — Format de jeton compact pour des claims, similaire à JWT mais basé sur CBOR. Transporte des claims d’identité et des métadonnées. Défini dans la [RFC 8392](https://www.rfc-editor.org/rfc/rfc8392).

### Base45
Schéma d’encodage qui convertit des données binaires en caractères alphanumériques optimisés pour les QR codes. Défini dans la [RFC 9285](https://www.rfc-editor.org/rfc/rfc9285).

### zlib
Bibliothèque de compression utilisant l’algorithme DEFLATE. Utilisée pour réduire la taille de l’identifiant avant l’encodage Base45.

## Termes cryptographiques

### Ed25519
Algorithme de signature numérique basé sur les courbes elliptiques. Rapide, sûr, signatures de 64 octets avec clés de 32 octets.

### ECDSA P-256
**Elliptic Curve Digital Signature Algorithm** utilisant la courbe NIST P-256. Largement supporté, signatures de 64 octets avec clés privées de 32 octets.

### AES-GCM
**Advanced Encryption Standard - Galois/Counter Mode** — Algorithme de chiffrement authentifié fournissant confidentialité et intégrité.

### AEAD
**Authenticated Encryption with Associated Data** — Chiffrement qui fournit à la fois confidentialité et authenticité. AES-GCM est un algorithme AEAD.

### Nonce
**Number used once** — Valeur aléatoire qui ne doit jamais être réutilisée avec la même clé. Pour AES-GCM, les nonces font 12 octets.

### Key ID (kid)
Identifiant inclus dans les en-têtes COSE pour aider le vérificateur à sélectionner la bonne clé.

## Types de messages COSE

### COSE_Sign1
Message COSE contenant une seule signature. Utilisé pour des identifiants signés.

### COSE_Encrypt0
Message COSE contenant un contenu chiffré avec un seul destinataire. Utilisé pour des identifiants chiffrés.

### En-tête protégé
Paramètres d’en-tête COSE inclus dans le calcul de signature. Ne peuvent pas être modifiés sans invalider la signature.

### En-tête non protégé
Paramètres d’en-tête COSE non inclus dans le calcul de signature. Peuvent être modifiés sans affecter la vérification.

## Claims CWT

### iss (Issuer)
**Clé de claim : 1** — Identifie l’émetteur de l’identifiant. Typiquement une URL ou un identifiant d’organisation.

### sub (Subject)
**Clé de claim : 2** — Identifie la personne concernée. Peut être un identifiant utilisateur ou un nom.

### exp (Expiration)
**Clé de claim : 4** — Horodatage Unix après lequel l’identifiant est invalide.

### nbf (Not Before)
**Clé de claim : 5** — Horodatage Unix avant lequel l’identifiant n’est pas valide.

### iat (Issued At)
**Clé de claim : 6** — Horodatage Unix indiquant quand l’identifiant a été émis.

### Claim 169
**Clé de claim : 169** — Claim enregistré à l’IANA contenant la charge utile d’identité.

## Champs d’identité

### Données démographiques
Informations d’identité principales : nom, date de naissance, genre, adresse, nationalité, etc. Clés CBOR 1–23.

### Biométrie
Marqueurs d’identité biologique : empreintes, iris, visage, voix. Clés CBOR 50–65.

### Format photo
Format d’image pour les photos embarquées : JPEG, JPEG 2000, AVIF, ou WEBP.

### Best Quality Fingers
Tableau indiquant quels doigts ont les captures biométriques de meilleure qualité, ordonnées par qualité.

## Termes de sécurité

### Vérification de signature
Processus consistant à vérifier qu’une signature est valide via la clé publique de l’émetteur.

### Validation des horodatages
Vérifier qu’un identifiant est dans sa fenêtre de validité (après `nbf`, avant `exp`).

### Bombe de décompression
Charge utile compressée malveillante qui se décompresse à une taille énorme, pouvant épuiser la mémoire.

### Confusion d’algorithme
Attaque où un attaquant tente de faire utiliser un algorithme différent de celui prévu.

### Compatibilité ascendante
Capacité à décoder des identifiants contenant des champs inconnus, en les conservant pour une utilisation future.

## Termes de la bibliothèque

### Decoder
Composant qui convertit une chaîne QR Base45 en identifiant décodé.

### Encoder
Composant qui convertit des données d’identité en chaîne Base45 signée et compressée.

### Statut de vérification
Résultat de la vérification : `Verified`, `Unverified`, ou une erreur.

### Fournisseur crypto personnalisé
Implémentation d’opérations cryptographiques via des systèmes externes (HSM, KMS, etc.).

