# Encodage

Ce guide montre comment produire une chaîne Base45 prête à être mise dans un QR code depuis Java.

## Encodage minimal (Ed25519)

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.Claim169DataConfigurer;
import fr.acn.claim169.CwtMetaDataConfigurer;
import fr.acn.claim169.EncoderConfigurer;
import fr.acn.claim169.Claim169Data;
import fr.acn.claim169.CwtMetaData;

Claim169Data data = Claim169.claim169((Claim169DataConfigurer) b -> {
    b.setId("ID-12345");
    b.setFullName("Jane Doe");
});

CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) b -> {
    b.setIssuer("https://issuer.example.org");
    b.setIssuedAt(1700000000L);
    b.setExpiresAt(1800000000L);
});

byte[] privateKey = hexToByteArray("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");

String qrData = Claim169.encode(data, meta, (EncoderConfigurer) b -> {
    b.signWithEd25519(privateKey);
});
```

## Avec le builder explicite

```java
import fr.acn.claim169.Claim169DataBuilder;
import fr.acn.claim169.CwtMetaDataBuilder;

Claim169DataBuilder dataBuilder = new Claim169DataBuilder();
dataBuilder.setId("ID-12345");
dataBuilder.setFullName("Jane Doe");
Claim169Data data = dataBuilder.build();

CwtMetaDataBuilder metaBuilder = new CwtMetaDataBuilder();
metaBuilder.setIssuer("https://issuer.example.org");
metaBuilder.setExpiresAt(1800000000L);
CwtMetaData meta = metaBuilder.build();
```

## Rappels importants

- La signature est requise en production (les identifiants non signés ne sont pas vérifiables).
- `issuer` doit être cohérent avec le mécanisme de distribution des clés publiques côté vérificateur.
- Les timestamps CWT (`issuedAt`, `expiresAt`, `notBefore`) pilotent la validité temporelle.

!!! note "Options avancées"
    Pour ECDSA P-256, PEM, `kid`, ou l'encodage de photos/biométrie, basculez sur la version anglaise via le sélecteur de langue (English).
