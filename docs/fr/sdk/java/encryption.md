# Chiffrement

Le chiffrement est optionnel. Il enveloppe un payload signé dans un COSE_Encrypt0 (AES-GCM).

## Encoder + chiffrer (AES-256-GCM)

```java
String qrData = Claim169.encode(data, meta, (EncoderConfigurer) b -> {
    b.signWithEd25519(privateKey);
    b.encryptWithAes256(aesKey); // 32 octets
});
```

## Décoder + déchiffrer

```java
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) b -> {
    b.decryptWithAes256(aesKey);
    b.verifyWithEd25519(publicKey);
});
```

## Bonnes pratiques

- Ne réutilisez jamais une nonce (le SDK génère une nonce aléatoire côté encodeur).
- La gestion de la clé AES (distribution/rotation) dépend de votre système.

!!! note "Détails"
    Pour AES-128, chiffrement via provider custom (HSM/KMS) et considérations de sécurité, basculez sur la version anglaise via le sélecteur de langue (English).
