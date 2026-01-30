# SDK Kotlin

Le SDK Kotlin/Java permet d’encoder et décoder des QR codes Claim 169 sur JVM (et Android) via des bindings natifs.

## Pré-requis

- JDK 17+
- La bibliothèque charge un binaire natif via JNA (configuration détaillée dans la page Installation)

## Installation (Gradle)

```kotlin
dependencies {
  implementation("fr.acn.claim169:claim169-core:<version>")
}
```

## Guides

- Démarrage rapide : `quick-start.md`
- Installation : `installation.md`
- Encodage : `encoding.md`
- Décodage : `decoding.md`
- Chiffrement : `encryption.md`
- Crypto personnalisée (HSM/KMS) : `custom-crypto.md`
- Utilisation Java : `java-usage.md`
- Référence API : `api.md` (référence complète en anglais)

!!! note "Documentation complète"
    Si vous avez besoin de détails supplémentaires, basculez sur la version anglaise via le sélecteur de langue (English).
