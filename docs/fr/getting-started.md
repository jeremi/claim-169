# Bien démarrer

Les QR codes Claim 169 sont des chaînes **Base45** contenant des données d’identité signées (et éventuellement chiffrées).

!!! warning "Ne pas modifier la chaîne Base45"
    L’alphabet Base45 inclut un caractère espace (`" "`). Conservez le texte scanné tel quel (pas de `.trim()`, ni normalisation des espaces), sinon vous risquez de corrompre des identifiants valides.

## Choisir votre parcours

- **Vérifier (lire)** : décoder un QR code scanné et le vérifier avec la clé publique de l’émetteur.
- **Émettre (écrire)** : construire un payload Claim 169, le signer avec la clé privée de l’émetteur, puis l’encoder pour un QR code.

## Vérifier (Lire)

Vous avez besoin de :

- Le texte scanné (Base45)
- La clé publique de l’émetteur (et le bon algorithme : Ed25519 ou ECDSA P-256)

Commencez ici :

- Python : `sdk/python/quick-start.md`
- Rust : `sdk/rust/quick-start.md`
- TypeScript : `sdk/typescript/quick-start.md`
- Kotlin : `sdk/kotlin/quick-start.md`

## Émettre (Écrire)

Vous avez besoin de :

- Une **clé privée** d’émetteur (Ed25519 recommandé)
- Des métadonnées CWT (au minimum `issuer`, et souvent `issuedAt`/`expiresAt`)
- Un payload Claim 169 minimal (souvent `id` + `fullName`)

Commencez ici :

- Python : `sdk/python/encoding.md`
- Rust : `sdk/rust/encoding.md`
- TypeScript : `sdk/typescript/encoding.md`
- Kotlin : `sdk/kotlin/encoding.md`

## Entrées de référence (vecteurs de test)

Pour décoder un QR déjà prêt (ou valider une autre implémentation), utilisez les vecteurs de test du dépôt :

- `test-vectors/valid/ed25519-signed.json` (signé)
- `test-vectors/valid/ecdsa-p256-signed.json` (signé)
- `test-vectors/valid/encrypted-signed.json` (chiffré + signé)

La référence de spécification est dans `core/specification.md`.

