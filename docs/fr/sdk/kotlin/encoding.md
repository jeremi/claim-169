# Encodage

Ce guide montre comment produire une chaîne Base45 prête à être mise dans un QR code.

## Encodage minimal (Ed25519)

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

val qrData = Claim169.encode(claim, meta) {
  signWithEd25519(privateKey)
}
```

## Rappels importants

- La signature est requise en production (les identifiants non signés ne sont pas vérifiables).
- `issuer` doit être cohérent avec le mécanisme de distribution des clés publiques côté vérificateur.
- Les timestamps CWT (`issuedAt`, `expiresAt`, `notBefore`) pilotent la validité temporelle.

!!! note "Options avancées"
    Pour ECDSA P-256, PEM, `kid`, ou l’encodage de photos/biométrie, basculez sur la version anglaise via le sélecteur de langue (English).
