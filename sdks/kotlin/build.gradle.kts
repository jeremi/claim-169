plugins {
    kotlin("jvm") version "1.9.24" apply false
    kotlin("android") version "1.9.24" apply false
    id("com.android.library") version "8.3.2" apply false
    id("org.jetbrains.dokka") version "2.0.0" apply false
    id("io.github.gradle-nexus.publish-plugin") version "2.0.0"
}

allprojects {
    group = "fr.acn.claim169"
    version = "0.2.0-alpha"
}

nexusPublishing {
    repositories {
        sonatype {
            nexusUrl.set(uri("https://ossrh-staging-api.central.sonatype.com/service/local/"))
            snapshotRepositoryUrl.set(uri("https://central.sonatype.com/repository/maven-snapshots/"))
            username.set(System.getenv("MAVEN_USERNAME") ?: "")
            password.set(System.getenv("MAVEN_PASSWORD") ?: "")
        }
    }
}
