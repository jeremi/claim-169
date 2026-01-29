plugins {
    kotlin("jvm")
    `maven-publish`
    signing
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
                url.set("https://github.com/niclas-AptusGlobal/claim-169")

                licenses {
                    license {
                        name.set("MIT")
                        url.set("https://opensource.org/licenses/MIT")
                    }
                }

                scm {
                    connection.set("scm:git:git://github.com/niclas-AptusGlobal/claim-169.git")
                    developerConnection.set("scm:git:ssh://github.com:niclas-AptusGlobal/claim-169.git")
                    url.set("https://github.com/niclas-AptusGlobal/claim-169")
                }
            }
        }
    }

    repositories {
        maven {
            name = "OSSRH"
            val releasesUrl = uri("https://s01.oss.sonatype.org/service/local/staging/deploy/maven2/")
            val snapshotsUrl = uri("https://s01.oss.sonatype.org/content/repositories/snapshots/")
            url = if (version.toString().endsWith("SNAPSHOT")) snapshotsUrl else releasesUrl

            credentials {
                username = System.getenv("MAVEN_USERNAME") ?: ""
                password = System.getenv("MAVEN_PASSWORD") ?: ""
            }
        }
    }
}

signing {
    val signingKey = System.getenv("GPG_SIGNING_KEY")
    val signingPassword = System.getenv("GPG_SIGNING_PASSWORD")
    if (signingKey != null && signingPassword != null) {
        useInMemoryPgpKeys(signingKey, signingPassword)
        sign(publishing.publications["maven"])
    }
}
