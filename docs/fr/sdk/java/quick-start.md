# Démarrage rapide

Ce guide couvre les opérations essentielles : décoder un QR code (lecture) et encoder un identifiant (écriture) depuis Java.

!!! warning "Ne pas modifier la chaîne Base45"
    L'alphabet Base45 inclut un caractère espace (`" "`). Conservez le texte scanné tel quel (pas de `.trim()`, ni normalisation des espaces), sinon vous risquez de corrompre des identifiants valides.

## Décoder (avec vérification Ed25519)

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.DecoderConfigurer;
import fr.acn.claim169.DecodeResultData;

byte[] publicKey = hexToByteArray("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a");

DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) b -> {
    b.verifyWithEd25519(publicKey);
});

System.out.println("ID: " + result.getClaim169().getId());
System.out.println("Nom: " + result.getClaim169().getFullName());
System.out.println("Statut: " + result.getVerificationStatus());
```

## Décoder (sans vérification - tests uniquement)

```java
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) b -> {
    b.allowUnverified();
});
// result.getVerificationStatus() == "skipped"
```

## Encoder (signer Ed25519)

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

System.out.println(qrData); // Base45 à mettre dans un QR code
```

## Gestion des erreurs

```java
import fr.acn.claim169.Claim169Exception;

try {
    DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) b -> {
        b.verifyWithEd25519(publicKey);
    });
} catch (Claim169Exception.Base45Decode e) {
    System.out.println("Format QR invalide : " + e.getMessage());
} catch (Claim169Exception.SignatureInvalid e) {
    System.out.println("Signature invalide : " + e.getMessage());
} catch (Claim169Exception e) {
    System.out.println("Erreur de décodage : " + e.getMessage());
}
```

!!! note "Aller plus loin"
    Pour des exemples plus complets (ECDSA, PEM, chiffrement, HSM/KMS), basculez sur la version anglaise via le sélecteur de langue (English).
