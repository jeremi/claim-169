# Kotlin SDK

<div class="badges" markdown>
[![Maven Central](https://img.shields.io/maven-central/v/fr.acn.claim169/claim169-core)](https://central.sonatype.com/artifact/fr.acn.claim169/claim169-core)
[![Kotlin](https://img.shields.io/badge/kotlin-1.9+-7F52FF.svg)](https://kotlinlang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jeremi/claim-169/blob/main/LICENSE)
</div>

The Kotlin SDK provides native bindings for encoding and decoding MOSIP Claim 169 QR codes. Built with UniFFI-generated JNA bindings, it delivers the full power of the Rust core library to JVM and Android applications through an idiomatic Kotlin DSL.

## Why Kotlin?

- **DSL API** — Idiomatic Kotlin builders with `claim169Data {}` and `cwtMetaData {}` syntax
- **Android Ready** — Supports Android API 24+ with native library loading
- **JVM Server-Side** — Runs on any JDK 17+ backend (Spring Boot, Ktor, Micronaut)
- **HSM/KMS Ready** — Interface-based crypto hooks for Android Keystore, HSMs, and cloud KMS
- **Type Safe** — Sealed class exception hierarchy and nullable field handling

## Installation

=== "Gradle Kotlin DSL"

    ```kotlin
    dependencies {
        implementation("fr.acn.claim169:claim169-core:0.2.0-alpha")
    }
    ```

=== "Maven"

    ```xml
    <dependency>
        <groupId>fr.acn.claim169</groupId>
        <artifactId>claim169-core</artifactId>
        <version>0.2.0-alpha</version>
    </dependency>
    ```

## Quick Start

```kotlin
import fr.acn.claim169.Claim169

// Decode a QR code with Ed25519 verification
val qrData = "NCFOXN..."  // Base45 string from QR scanner
val publicKey = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
    .hexToByteArray()

val result = Claim169.decode(qrData) {
    verifyWithEd25519(publicKey)
}

println("ID: ${result.claim169.id}")
println("Name: ${result.claim169.fullName}")
println("Status: ${result.verificationStatus}")  // VerificationStatus.Verified
```

## Documentation

<div class="doc-grid" markdown>

<div class="doc-card" markdown>
### [Installation](installation.md)
JDK requirements, Gradle/Maven setup, native library loading.
</div>

<div class="doc-card" markdown>
### [Quick Start](quick-start.md)
Simple encode/decode examples to get you started.
</div>

<div class="doc-card" markdown>
### [Encoding](encoding.md)
Create signed credentials with Ed25519 or ECDSA P-256.
</div>

<div class="doc-card" markdown>
### [Decoding](decoding.md)
Verify and extract identity data from QR codes.
</div>

<div class="doc-card" markdown>
### [Encryption](encryption.md)
AES-256-GCM and AES-128-GCM encryption examples.
</div>

<div class="doc-card" markdown>
### [Custom Crypto](custom-crypto.md)
Android Keystore, HSM, and cloud KMS integration.
</div>

<div class="doc-card" markdown>
### [API Reference](api.md)
Complete API documentation for all classes and interfaces.
</div>

<div class="doc-card" markdown>
### [Troubleshooting](troubleshooting.md)
Common errors and solutions.
</div>

</div>

## Features

### Decoding

| Method | Description |
|--------|-------------|
| `verifyWithEd25519()` | Decode with Ed25519 signature verification |
| `verifyWithEcdsaP256()` | Decode with ECDSA P-256 signature verification |
| `verifyWithEd25519Pem()` | Decode with Ed25519 PEM public key |
| `verifyWithEcdsaP256Pem()` | Decode with ECDSA P-256 PEM public key |
| `verifyWith(SignatureVerifier)` | Decode with custom verifier (HSM/KMS) |
| `decryptWithAes256()` | Decrypt AES-256-GCM and decode |
| `decryptWithAes128()` | Decrypt AES-128-GCM and decode |
| `decryptWith(Decryptor)` | Decrypt with custom decryptor callback |
| `allowUnverified()` | Decode without verification (testing only) |

### Encoding

| Method | Description |
|--------|-------------|
| `signWithEd25519()` | Encode with Ed25519 signature |
| `signWithEcdsaP256()` | Encode with ECDSA P-256 signature |
| `signWith(Signer)` | Encode with custom signer callback (HSM/KMS) |
| `encryptWithAes256()` | Encrypt with AES-256-GCM |
| `encryptWithAes128()` | Encrypt with AES-128-GCM |
| `encryptWith(Encryptor)` | Encrypt with custom encryptor callback |
| `skipBiometrics()` | Exclude biometric data from output |

## Requirements

- **JDK 17+** or **Android API 24+**
- **JNA** (Java Native Access) — transitive dependency, included automatically
- No additional dependencies for basic usage

## License

MIT License. See [LICENSE](https://github.com/jeremi/claim-169/blob/main/LICENSE) for details.
