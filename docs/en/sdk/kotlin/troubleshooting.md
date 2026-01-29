# Troubleshooting

Common errors and solutions for the claim169 Kotlin SDK.

## Native Library Errors

### UnsatisfiedLinkError

```
java.lang.UnsatisfiedLinkError: Unable to load library 'claim169_core':
Native library (linux-x86-64/libclaim169_core.so) not found
```

**Causes:**

- Native library not included in the classpath
- Platform not supported by the bundled native libraries
- Custom native library location not configured

**Solutions:**

1. Verify the dependency is correctly declared:
   ```kotlin
   dependencies {
       implementation("fr.acn.claim169:claim169-core:0.1.0-alpha.2")
   }
   ```

2. Check the library search path:
   ```kotlin
   println("java.library.path: ${System.getProperty("java.library.path")}")
   println("jna.library.path: ${System.getProperty("jna.library.path")}")
   ```

3. Set the library path explicitly:
   ```bash
   java -Djava.library.path=/path/to/native/libs -jar your-app.jar
   ```

4. For development builds, point to the Rust build output:
   ```bash
   java -Djava.library.path=../../target/release -jar your-app.jar
   ```

### UnsatisfiedLinkError on Android

```
java.lang.UnsatisfiedLinkError: dlopen failed: library "libclaim169_core.so" not found
```

**Solutions:**

1. Verify the AAR includes native libraries for the target ABI:
   ```
   src/main/jniLibs/
       arm64-v8a/libclaim169_core.so
       armeabi-v7a/libclaim169_core.so
       x86_64/libclaim169_core.so
   ```

2. Check that the ABI is not excluded in your build config:
   ```kotlin
   android {
       defaultConfig {
           ndk {
               abiFilters += listOf("arm64-v8a", "armeabi-v7a", "x86_64")
           }
       }
   }
   ```

3. Verify the published AAR artifact includes the `jni` directory

---

## JNA Errors

### NoClassDefFoundError: JNA

```
java.lang.NoClassDefFoundError: com/sun/jna/Library
```

**Cause:** JNA is not on the classpath.

**Solution:** Add JNA explicitly if it is not pulled in as a transitive dependency:

=== "Gradle Kotlin DSL"

    ```kotlin
    dependencies {
        implementation("net.java.dev.jna:jna:5.14.0")
    }
    ```

=== "Maven"

    ```xml
    <dependency>
        <groupId>net.java.dev.jna</groupId>
        <artifactId>jna</artifactId>
        <version>5.14.0</version>
    </dependency>
    ```

### JNA Architecture Mismatch

```
java.lang.UnsatisfiedLinkError: Native library (darwin-aarch64/libclaim169_core.dylib) not found
```

**Cause:** Running on a platform/architecture combination not included in the JAR.

**Solutions:**

1. Verify your JDK architecture matches expected platform:
   ```kotlin
   println("os.arch: ${System.getProperty("os.arch")}")
   println("os.name: ${System.getProperty("os.name")}")
   ```

2. On Apple Silicon Macs, ensure you are using an ARM64 JDK (not Rosetta x86_64)

3. Build the native library for your platform:
   ```bash
   cargo build --release
   ```

---

## Decoding Errors

### Base45DecodeError

```
fr.acn.claim169.Claim169Exception$Base45DecodeError: Invalid Base45 character at position 15
```

**Causes:**

- QR code was not fully scanned
- QR code content was truncated
- Input is not a Claim 169 QR code

**Solutions:**

1. Verify the QR code was fully scanned
2. Strip whitespace from input:
   ```kotlin
   val result = Claim169.decode(qrData.trim()) {
       verifyWithEd25519(publicKey)
   }
   ```
3. Verify the QR code is a MOSIP Claim 169 credential

### DecompressError: Size Limit Exceeded

```
fr.acn.claim169.Claim169Exception$DecompressError: decompressed size 150000 exceeds limit 65536
```

**Cause:** Credential decompresses to larger than the limit (default 64KB).

**Solution:** Increase the limit if you trust the source:

```kotlin
val result = Claim169.decode(qrData) {
    verifyWithEd25519(publicKey)
    maxDecompressedBytes(200_000)  // 200KB limit
}
```

### SignatureError

```
fr.acn.claim169.Claim169Exception$SignatureError: Signature verification failed
```

**Causes:**

- Wrong public key
- Credential was tampered with
- Key/algorithm mismatch (Ed25519 vs ECDSA P-256)

**Solutions:**

1. Verify you are using the correct public key for the issuer
2. Check the key format and length:
   ```kotlin
   println("Key length: ${publicKey.size} bytes")
   // Ed25519 should be 32
   // ECDSA P-256 compressed should be 33
   // ECDSA P-256 uncompressed should be 65
   ```
3. Ensure you are calling the correct verification method (`verifyWithEd25519` vs `verifyWithEcdsaP256`)

### TimestampValidationError

```
fr.acn.claim169.Claim169Exception$TimestampValidationError: Token expired at 1700000000
```

**Cause:** The credential has expired or its `nbf` time is in the future.

**Solutions:**

1. Check if the credential should be rejected (it is expired)
2. For testing, disable timestamp validation:
   ```kotlin
   val result = Claim169.decode(qrData) {
       verifyWithEd25519(publicKey)
       withoutTimestampValidation()
   }
   ```
3. Add clock skew tolerance:
   ```kotlin
   val result = Claim169.decode(qrData) {
       verifyWithEd25519(publicKey)
       clockSkewTolerance(seconds = 300)  // 5 minutes
   }
   ```

### DecryptionError

```
fr.acn.claim169.Claim169Exception$DecryptionError: Decryption failed
```

**Causes:**

- Wrong decryption key
- Wrong key size for algorithm
- Corrupted ciphertext

**Solutions:**

1. Verify key size matches algorithm:
   ```kotlin
   println("Key length: ${encryptKey.size} bytes")
   // AES-256: 32 bytes
   // AES-128: 16 bytes
   ```
2. Use the correct method for your key size:
   ```kotlin
   // For 32-byte keys
   decryptWithAes256(key32)

   // For 16-byte keys
   decryptWithAes128(key16)
   ```

---

## Encoding Errors

### Invalid Key Length

```
java.lang.IllegalArgumentException: Ed25519 private key must be 32 bytes, got 64
```

**Solution:** Ensure your key is the correct length. If using a JCA-generated key, extract the raw 32-byte seed:

```kotlin
import java.security.KeyPairGenerator

val keyPair = KeyPairGenerator.getInstance("Ed25519").generateKeyPair()
val privateKeyEncoded = keyPair.private.encoded
// PKCS#8 encoded key is longer; extract the raw 32-byte seed
val rawSeed = privateKeyEncoded.sliceArray(privateKeyEncoded.size - 32 until privateKeyEncoded.size)
println("Raw key length: ${rawSeed.size}")  // 32
```

---

## Android ProGuard Rules

When using R8/ProGuard with the SDK, add these rules to your `proguard-rules.pro`:

```proguard
# claim169 SDK - keep JNA and UniFFI bindings
-keep class fr.acn.claim169.** { *; }
-keep class com.sun.jna.** { *; }
-keepclassmembers class * extends com.sun.jna.Structure {
    public *;
}

# Keep native method names
-keepclasseswithmembernames class * {
    native <methods>;
}

# Keep UniFFI callback interfaces
-keep interface fr.acn.claim169.SignatureVerifier { *; }
-keep interface fr.acn.claim169.Signer { *; }
-keep interface fr.acn.claim169.Decryptor { *; }
-keep interface fr.acn.claim169.Encryptor { *; }
```

---

## Memory and Threading

### Memory Usage

Large credentials (especially those with biometric data) consume memory during decoding. Control memory usage by:

1. Setting decompression limits:
   ```kotlin
   val result = Claim169.decode(qrData) {
       verifyWithEd25519(publicKey)
       maxDecompressedBytes(32_768)  // 32KB limit
   }
   ```

2. Skipping biometrics when not needed:
   ```kotlin
   val result = Claim169.decode(qrData) {
       verifyWithEd25519(publicKey)
       skipBiometrics()
   }
   ```

### Thread Safety

The `Claim169` singleton methods are thread-safe. You can call `decode()` and `encode()` concurrently from multiple threads or coroutines without synchronization.

Custom crypto provider implementations (`SignatureVerifier`, `Signer`, `Decryptor`, `Encryptor`) must be thread-safe if shared across threads.

```kotlin
import kotlinx.coroutines.*

// Safe: concurrent decoding
coroutineScope {
    val jobs = qrCodes.map { qr ->
        async(Dispatchers.Default) {
            Claim169.decode(qr) {
                verifyWithEd25519(publicKey)
            }
        }
    }
    val results = jobs.awaitAll()
}
```

### Native Memory

The SDK allocates native memory through JNA for Rust core operations. This memory is freed automatically when the JVM garbage collects the associated Java objects. On Android, be mindful of native memory limits in memory-constrained environments.

---

## Common Mistakes

### Forgetting to Convert Key from Hex

```kotlin
// Wrong - passing hex string as UTF-8 bytes
val publicKey = "d75a98...".toByteArray()
val result = Claim169.decode(qrData) {
    verifyWithEd25519(publicKey)  // Will fail!
}

// Correct - convert hex to bytes
val publicKey = "d75a98...".hexToByteArray()
val result = Claim169.decode(qrData) {
    verifyWithEd25519(publicKey)  // Works!
}
```

### Using Private Key for Verification

```kotlin
// Wrong - using private key for verification
val result = Claim169.decode(qrData) {
    verifyWithEd25519(privateKey)  // May fail!
}

// Correct - use public key for verification
val result = Claim169.decode(qrData) {
    verifyWithEd25519(publicKey)
}
```

### Missing Verification for Encrypted Credentials

```kotlin
// Wrong - no verifier configured for encrypted credential
val result = Claim169.decode(qrData) {
    decryptWithAes256(key)
    // No verifier -> will fail unless allowUnverified() is set
}

// Correct - provide verifier
val result = Claim169.decode(qrData) {
    decryptWithAes256(key)
    verifyWithEd25519(publicKey)
}

// Or for testing:
val result = Claim169.decode(qrData) {
    decryptWithAes256(key)
    allowUnverified()
}
```

### hexToByteArray Not Available

If `String.hexToByteArray()` is not available (requires Kotlin 1.9+), use a utility function:

```kotlin
fun String.decodeHex(): ByteArray {
    check(length % 2 == 0) { "Hex string must have even length" }
    return chunked(2)
        .map { it.toInt(16).toByte() }
        .toByteArray()
}

val publicKey = "d75a98...".decodeHex()
```

---

## Getting Help

If you encounter an issue not covered here:

1. **Check the API reference** -- [api.md](api.md)
2. **Review examples** -- Check test files in the repository
3. **Open an issue** -- [GitHub Issues](https://github.com/jeremi/claim-169/issues)

When reporting issues, include:

- JDK version: `java -version`
- Kotlin version
- claim169 version: `Claim169.version()`
- Operating system and architecture
- Android API level (if applicable)
- Minimal code to reproduce the issue
- Full stack trace
