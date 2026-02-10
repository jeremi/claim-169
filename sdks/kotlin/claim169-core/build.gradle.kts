plugins {
    kotlin("jvm")
    `maven-publish`
    signing
    id("org.jetbrains.dokka")
}

tasks.withType<org.jetbrains.dokka.gradle.DokkaTask>().configureEach {
    moduleName.set("claim169-core")
    dokkaSourceSets {
        configureEach {
            // Suppress auto-generated UniFFI JNI bindings from documentation
            perPackageOption {
                matchingRegex.set("uniffi\\..*")
                suppress.set(true)
            }
        }
    }
}

dependencies {
    // JNA for UniFFI native library loading
    implementation("net.java.dev.jna:jna:5.14.0")

    // Testing
    testImplementation(kotlin("test"))
    testImplementation("org.junit.jupiter:junit-jupiter:5.10.2")
    testImplementation("com.google.code.gson:gson:2.11.0")
}

tasks.test {
    useJUnitPlatform()

    // Pass native library path for tests
    val nativeLibDir = rootProject.projectDir.resolve("../../target/debug")
    systemProperty("java.library.path", nativeLibDir.absolutePath)
    systemProperty("jna.library.path", nativeLibDir.absolutePath)

    // Pass test vectors path
    val testVectorsDir = rootProject.projectDir.resolve("../../test-vectors")
    systemProperty("test.vectors.dir", testVectorsDir.absolutePath)

    // Forward conformance test properties from Gradle command line
    listOf("conformance.vectors.path", "conformance.output.path").forEach { prop ->
        System.getProperty(prop)?.let { systemProperty(prop, it) }
    }
}

kotlin {
    jvmToolchain(17)
}

java {
    withSourcesJar()
    withJavadocJar()
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            from(components["java"])

            pom {
                name.set("Claim 169 SDK")
                description.set("Kotlin/Java SDK for encoding and decoding Claim 169 QR codes")
                url.set("https://github.com/jeremi/claim-169")

                licenses {
                    license {
                        name.set("MIT")
                        url.set("https://opensource.org/licenses/MIT")
                    }
                }

                developers {
                    developer {
                        id.set("jeremi")
                        name.set("Jeremi Joslin")
                        url.set("https://github.com/jeremi")
                    }
                }

                scm {
                    connection.set("scm:git:git://github.com/jeremi/claim-169.git")
                    developerConnection.set("scm:git:ssh://github.com:jeremi/claim-169.git")
                    url.set("https://github.com/jeremi/claim-169")
                }
            }
        }
    }

    // Repository is configured by the gradle-nexus/publish-plugin in the root project.
}

signing {
    // Prefer the new env var name `GPG_PRIVATE_KEY` (Central Portal docs),
    // but keep `GPG_SIGNING_KEY` for backwards compatibility.
    val signingKey = System.getenv("GPG_PRIVATE_KEY") ?: System.getenv("GPG_SIGNING_KEY")
    // Password is optional: unencrypted keys can be used with an empty passphrase.
    val signingPassword = System.getenv("GPG_SIGNING_PASSWORD") ?: ""

    if (!signingKey.isNullOrBlank()) {
        useInMemoryPgpKeys(signingKey, signingPassword)
        sign(publishing.publications["maven"])
    }
}
