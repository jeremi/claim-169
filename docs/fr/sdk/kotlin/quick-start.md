# Démarrage rapide

Ce guide couvre les opérations essentielles : décoder un QR code (lecture) et encoder un identifiant (écriture).

!!! warning "Ne pas modifier la chaîne Base45"
    L’alphabet Base45 inclut un caractère espace (`" "`). Conservez le texte scanné tel quel (pas de `.trim()`, ni normalisation des espaces), sinon vous risquez de corrompre des identifiants valides.

## Décoder (avec vérification Ed25519)

```kotlin
import fr.acn.claim169.Claim169

val qrData = "..." // Base45 depuis le scanner
val publicKey = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
  .hexToByteArray()

val result = Claim169.decode(qrData) {
  verifyWithEd25519(publicKey)
}

println("ID: ${result.claim169.id}")
println("Nom: ${result.claim169.fullName}")
println("Statut: ${result.verificationStatus}") // "verified" / "skipped"
```

## Décoder (sans vérification - tests uniquement)

```kotlin
val result = Claim169.decode(qrData) {
  allowUnverified()
}

check(result.verificationStatus == "skipped")
```

## Encoder (signer Ed25519)

```kotlin
import fr.acn.claim169.Claim169
import fr.acn.claim169.claim169Data
import fr.acn.claim169.cwtMetaData

val claim = claim169Data {
  id = "ID-12345"
  fullName = "Jane Doe"
}

val meta = cwtMetaData {
  issuer = "https://issuer.example.org"
  issuedAt = 1700000000L
  expiresAt = 1800000000L
}

val privateKey = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
  .hexToByteArray()

val out = Claim169.encode(claim, meta) {
  signWithEd25519(privateKey)
}

println(out) // Base45 à mettre dans un QR code
```

!!! note "Aller plus loin"
    Pour des exemples plus complets (ECDSA, PEM, chiffrement, HSM/KMS), basculez sur la version anglaise via le sélecteur de langue (English).
