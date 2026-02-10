plugins {
    kotlin("jvm") version "1.9.24" apply false
    kotlin("android") version "1.9.24" apply false
    id("com.android.library") version "8.3.2" apply false
    id("org.jetbrains.dokka") version "2.0.0" apply false
}

allprojects {
    group = "fr.acn.claim169"
    version = "0.2.0-alpha"
}
