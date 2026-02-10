# Installation

## Pré-requis

- JDK 17+
- Le SDK utilise JNA pour charger la bibliothèque native (générée via UniFFI).

## Gradle (Kotlin DSL)

```kotlin
dependencies {
  implementation("fr.acn.claim169:claim169-core:0.2.0-alpha")
}
```

## Gradle (Groovy)

```groovy
dependencies {
  implementation "fr.acn.claim169:claim169-core:0.2.0-alpha"
}
```

## Où placer la lib native

Dans la plupart des cas (usage standard), vous n’avez rien à faire : la lib native est embarquée et chargée par JNA.

Si vous avez un besoin spécifique (tests, packaging, Android, chemins custom), vous devrez peut-être configurer :

- `java.library.path`
- `jna.library.path`

!!! note "Détails"
    Pour l’ordre exact de recherche et les exemples de configuration (Linux/macOS/Windows, Android), basculez sur la version anglaise via le sélecteur de langue (English).
