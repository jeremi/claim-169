# Dépannage

Cette page liste les erreurs fréquentes et la manière la plus rapide de les corriger.

## « decoding configuration error »

Si vous obtenez une erreur du type « either provide a verifier or explicitly allow unverified decoding », cela signifie que vous avez tenté de décoder sans :

- configurer la vérification de signature, ou
- vous désengager explicitement de la vérification (tests uniquement).

Correctifs :

- **Rust** : en production, appeler `.verify_with_ed25519(...)` / `.verify_with_ecdsa_p256(...)` ; en test, appeler `.allow_unverified()`
- **Python** : en production, utiliser `decode_with_ed25519()` / `decode_with_ecdsa_p256()` ; en test, utiliser `decode_unverified()` (ou `decode()` si l’alias est activé)
- **TypeScript** : en production, appeler `.verifyWithEd25519(...)` / `.verifyWithEcdsaP256(...)` ; en test, appeler `.allowUnverified()`

## Échecs de vérification de signature

Causes courantes :

- mauvais algorithme (Ed25519 vs ES256),
- mauvaise clé publique (mauvais émetteur / mauvais environnement),
- QR tronqué ou corrompu.

Correctifs :

- vérifier que le couple vecteur/clé correspond,
- vérifier le format attendu pour la clé (voir le guide des clés).

## « credential expired » / « not valid until … »

La validation des timestamps rejette l’identifiant via `exp`/`nbf`.

Options :

- utiliser des identifiants non expirés,
- ajuster la tolérance d’horloge,
- désactiver la validation seulement si votre modèle de menace l’autorise.

## « decompression limit exceeded »

La librairie impose une limite de taille après décompression (64KB par défaut).

Correctifs :

- si vous contrôlez l’émetteur et attendez des payloads plus gros, augmenter la limite (`max_decompressed_bytes(...)` / `maxDecompressedBytes(...)`),
- sinon considérer l’entrée comme potentiellement malveillante et rejeter.

## Échecs de déchiffrement

Causes courantes :

- mauvaise clé AES (ou mauvaise longueur),
- ordre incorrect (il faut déchiffrer avant de vérifier),
- ciphertext corrompu.

Correctifs :

- déchiffrer **avant** de vérifier,
- vérifier la taille : AES-256 = 32 octets, AES-128 = 16 octets.

