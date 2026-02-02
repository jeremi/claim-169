# Dépannage

## Erreurs courantes

### Signature invalide

- Vérifiez que vous utilisez la bonne clé publique (et le bon algorithme : Ed25519 vs ECDSA P-256).
- Si `verificationStatus == "skipped"`, la vérification a été ignorée (tests uniquement).

### Expiré / NotYetValid

L'implémentation côté JVM valide les timestamps (`exp` / `nbf`) et peut lever une erreur si :

- le jeton est expiré
- le jeton n'est pas encore valide

### Base45 corrompu

Ne tronquez pas et ne normalisez pas la chaîne Base45. L'alphabet Base45 inclut un caractère espace (`" "`).

## Chargement de la lib native (JNA)

Si la lib native ne se charge pas, vérifiez :

- `java.library.path`
- `jna.library.path`

## Spécificités Java

### Exceptions vérifiées

Les méthodes `decode`, `encode`, et `decodeCloseable` déclarent `throws Claim169Exception`. Vous devez les gérer avec `try-catch` ou propager l'exception.

### Ambiguïté de lambda

Depuis Java, les lambdas peuvent être ambiguës entre la surcharge Kotlin DSL et la surcharge `Configurer`. Utilisez un cast explicite :

```java
// Correct
Claim169.decode(qrData, (DecoderConfigurer) b -> { b.allowUnverified(); });

// Incorrect (ambiguïté)
Claim169.decode(qrData, b -> { b.allowUnverified(); });
```

### ProGuard / R8 (Android)

Ajoutez les règles suivantes à votre `proguard-rules.pro` :

```proguard
-keep class fr.acn.claim169.** { *; }
-keep class com.sun.jna.** { *; }
-keep interface fr.acn.claim169.SignatureVerifier { *; }
-keep interface fr.acn.claim169.Signer { *; }
-keep interface fr.acn.claim169.Decryptor { *; }
-keep interface fr.acn.claim169.Encryptor { *; }
```

!!! note "Détails"
    Pour des messages d'erreurs typiques et les configurations par OS/Android, basculez sur la version anglaise via le sélecteur de langue (English).
