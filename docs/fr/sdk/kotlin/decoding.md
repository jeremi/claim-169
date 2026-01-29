# Décodage

Ce guide couvre le décodage d’un QR code Claim 169 et la vérification de signature.

!!! warning "Ne pas modifier la chaîne Base45"
    L’alphabet Base45 inclut un caractère espace (`" "`). Conservez le texte scanné tel quel (pas de `.trim()`, ni normalisation des espaces).

## Vérifier une signature Ed25519

```kotlin
import fr.acn.claim169.Claim169

val result = Claim169.decode(qrData) {
  verifyWithEd25519(publicKey)
}

check(result.isVerified)
println(result.claim169.fullName)
```

## Identifiants chiffrés

Si le QR est chiffré (COSE_Encrypt0), fournissez la clé AES avant de décoder :

```kotlin
val result = Claim169.decode(qrData) {
  decryptWithAes256(aesKey) // 32 octets
  verifyWithEd25519(publicKey)
}
```

## Statut vs erreurs

- `verificationStatus == "verified"` : signature valide
- `verificationStatus == "skipped"` : vérification explicitement ignorée (tests uniquement)
- Erreur/exception : structure invalide, signature invalide, chiffrement invalide, expiré, etc.

!!! note "Détails"
    Pour ECDSA P-256, PEM, tolérance d’horloge, limites de décompression, ou `skipBiometrics`, basculez sur la version anglaise via le sélecteur de langue (English).
