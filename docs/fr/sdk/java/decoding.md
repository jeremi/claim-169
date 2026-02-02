# Décodage

Ce guide couvre le décodage d'un QR code Claim 169 et la vérification de signature depuis Java.

!!! warning "Ne pas modifier la chaîne Base45"
    L'alphabet Base45 inclut un caractère espace (`" "`). Conservez le texte scanné tel quel (pas de `.trim()`, ni normalisation des espaces).

## Vérifier une signature Ed25519

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.DecoderConfigurer;

DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) b -> {
    b.verifyWithEd25519(publicKey);
});

System.out.println(result.getClaim169().getFullName());
System.out.println(result.getVerificationStatus()); // "verified"
```

## Identifiants chiffrés

Si le QR est chiffré (COSE_Encrypt0), fournissez la clé AES avant de décoder :

```java
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) b -> {
    b.decryptWithAes256(aesKey); // 32 octets
    b.verifyWithEd25519(publicKey);
});
```

## Décodage sécurisé (zéroisation)

Utilisez try-with-resources pour zéroiser les données biométriques :

```java
try (CloseableDecodeResult result = Claim169.decodeCloseable(qrData, (DecoderConfigurer) b -> {
    b.verifyWithEd25519(publicKey);
})) {
    String name = result.getData().getClaim169().getFullName();
    // ... traiter le credential
}
// Les tableaux de bytes biométriques/photo sont maintenant remplis de zéros
```

## Statut de vérification

```java
import fr.acn.claim169.VerificationStatus;

VerificationStatus status = Claim169.verificationStatus(result);
if (status == VerificationStatus.Verified) {
    System.out.println("Signature vérifiée");
}
```

## Statut vs erreurs

- `verificationStatus == "verified"` : signature valide
- `verificationStatus == "skipped"` : vérification explicitement ignorée (tests uniquement)
- Erreur/exception : structure invalide, signature invalide, chiffrement invalide, expiré, etc.

!!! note "Détails"
    Pour ECDSA P-256, PEM, tolérance d'horloge, limites de décompression, ou `skipBiometrics`, basculez sur la version anglaise via le sélecteur de langue (English).
