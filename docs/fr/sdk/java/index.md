# SDK Java

Le SDK Java permet d'encoder et décoder des QR codes Claim 169 sur JVM (et Android) via des bindings natifs. Il utilise le même artefact Maven que le SDK Kotlin (`claim169-core`), avec des points d'entrée adaptés à Java.

## Pré-requis

- JDK 17+
- La bibliothèque charge un binaire natif via JNA (configuration détaillée dans la page Installation)

## Installation (Gradle)

```kotlin
dependencies {
  implementation("fr.acn.claim169:claim169-core:<version>")
}
```

## Exemple rapide

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.DecoderConfigurer;
import uniffi.claim169_jni.DecodeResultData;

DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) b -> {
    b.verifyWithEd25519(publicKey);
});

System.out.println("ID: " + result.getClaim169().getId());
System.out.println("Nom: " + result.getClaim169().getFullName());
```

## Guides

- Démarrage rapide : `quick-start.md`
- Installation : `installation.md`
- Encodage : `encoding.md`
- Décodage : `decoding.md`
- Chiffrement : `encryption.md`
- Crypto personnalisée (HSM/KMS) : `custom-crypto.md`
- Référence API : `api.md` (référence complète en anglais)

!!! note "Utilisateurs Kotlin"
    Si vous utilisez Kotlin, consultez la [documentation Kotlin](../kotlin/index.md) pour la syntaxe DSL idiomatique.

!!! note "Documentation complète"
    Si vous avez besoin de détails supplémentaires, basculez sur la version anglaise via le sélecteur de langue (English).
