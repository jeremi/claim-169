# Chiffrement

Le chiffrement est optionnel. Il enveloppe un payload signé dans un COSE_Encrypt0 (AES-GCM).

## Encoder + chiffrer (AES-256-GCM)

```kotlin
import fr.acn.claim169.Claim169

val qrData = Claim169.encode(claim, meta) {
  signWithEd25519(privateKey)
  encryptWithAes256(aesKey) // 32 octets
}
```

## Décoder + déchiffrer

```kotlin
val result = Claim169.decode(qrData) {
  decryptWithAes256(aesKey)
  verifyWithEd25519(publicKey)
}
```

## Bonnes pratiques

- Ne réutilisez jamais une nonce (le SDK génère une nonce aléatoire côté encodeur).
- La gestion de la clé AES (distribution/rotation) dépend de votre système.

!!! note "Détails"
    Pour AES-128, chiffrement via provider custom (HSM/KMS) et considérations de sécurité, basculez sur la version anglaise via le sélecteur de langue (English).
