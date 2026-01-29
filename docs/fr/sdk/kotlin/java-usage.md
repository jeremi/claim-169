# Utilisation avec Java

Le SDK Kotlin est utilisable depuis Java (meme artefact Maven).

## Decodage (Java)

La DSL Kotlin utilise une lambda "trailing". Depuis Java, utilisez l'interface fonctionnelle `DecoderConfigurer` :

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.DecodeResultData;

DecodeResultData result = Claim169.decode(qrData, builder -> {
    builder.verifyWithEd25519(publicKey);
});
```

## Encodage (Java)

```java
String out = Claim169.encode(claim169, cwtMeta, builder -> {
    builder.signWithEd25519(privateKey);
});
```

!!! note "Details"
    Pour des exemples complets (PEM, ECDSA, chiffrement), basculez sur la version anglaise via le selecteur de langue (English).
