# Troubleshooting

Common errors and solutions for the claim169 Java SDK.

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

    === "Gradle Kotlin DSL"

        ```kotlin
        dependencies {
            implementation("fr.acn.claim169:claim169-core:0.1.0-alpha.2")
        }
        ```

    === "Maven"

        ```xml
        <dependency>
            <groupId>fr.acn.claim169</groupId>
            <artifactId>claim169-core</artifactId>
            <version>0.1.0-alpha.2</version>
        </dependency>
        ```

2. Check the library search path:
   ```java
   System.out.println("java.library.path: " + System.getProperty("java.library.path"));
   System.out.println("jna.library.path: " + System.getProperty("jna.library.path"));
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
   ```java
   System.out.println("os.arch: " + System.getProperty("os.arch"));
   System.out.println("os.name: " + System.getProperty("os.name"));
   ```

2. On Apple Silicon Macs, ensure you are using an ARM64 JDK (not Rosetta x86_64)

3. Build the native library for your platform:
   ```bash
   cargo build --release
   ```

---

## Java-Specific Issues

### Checked Exception Handling

All `Claim169.decode()` and `Claim169.encode()` methods throw `Claim169Exception`, which is a checked exception from Java's perspective (annotated with `@Throws` in Kotlin). You must handle or declare it:

```java
// Option 1: Try-catch
try {
    DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
        builder.verifyWithEd25519(publicKey);
    });
} catch (Claim169Exception e) {
    // Handle error
}

// Option 2: Declare in method signature
public DecodeResultData decodeCredential(String qrData) throws Claim169Exception {
    return Claim169.decode(qrData, (DecoderConfigurer) builder -> {
        builder.verifyWithEd25519(publicKey);
    });
}
```

### Lambda Ambiguity with Explicit Casts

When calling `Claim169.decode()` or `Claim169.encode()` from Java, the compiler may see both the Kotlin trailing-lambda overload and the `Configurer` functional interface overload. This causes an "ambiguous method call" compilation error.

**Solution:** Use an explicit cast to the configurer interface:

```java
// Wrong - may cause ambiguous method call
DecodeResultData result = Claim169.decode(qrData, builder -> {
    builder.verifyWithEd25519(publicKey);
});

// Correct - explicit cast resolves the ambiguity
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);
});
```

The same applies to `encode()`, `claim169()`, and `cwtMeta()`:

```java
// Encoding
String qr = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    builder.signWithEd25519(privateKey);
});

// Building data
Claim169Data data = Claim169.claim169((Claim169DataConfigurer) builder -> {
    builder.setId("X");
});

// Building metadata
CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) builder -> {
    builder.setIssuer("https://example.org");
});
```

### Void Lambda vs Unit Return

Kotlin lambdas return `Unit` while Java lambdas return `void`. This is normally transparent, but if you encounter type inference issues in complex generic contexts, the explicit cast resolves it.

---

## Decoding Errors

### Base45DecodeError

```
fr.acn.claim169.Claim169Exception$Base45Decode: Invalid Base45 character at position 15
```

**Causes:**

- QR code was not fully scanned
- QR code content was truncated
- Input is not a Claim 169 QR code

**Solutions:**

1. Verify the QR code was fully scanned
2. Check whitespace handling (do not trim Base45 content):
   ```java
   // Do NOT trim - Base45 may contain meaningful spaces
   DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
       builder.verifyWithEd25519(publicKey);
   });
   ```
3. Verify the QR code is a MOSIP Claim 169 credential

### DecompressError: Size Limit Exceeded

```
fr.acn.claim169.Claim169Exception$Decompress: decompressed size 150000 exceeds limit 65536
```

**Cause:** Credential decompresses to larger than the limit (default 64KB).

**Solution:** Increase the limit if you trust the source:

```java
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);
    builder.maxDecompressedBytes(200_000);  // 200KB limit
});
```

### SignatureError

```
fr.acn.claim169.Claim169Exception$SignatureInvalid: Signature verification failed
```

**Causes:**

- Wrong public key
- Credential was tampered with
- Key/algorithm mismatch (Ed25519 vs ECDSA P-256)

**Solutions:**

1. Verify you are using the correct public key for the issuer
2. Check the key format and length:
   ```java
   System.out.println("Key length: " + publicKey.length + " bytes");
   // Ed25519 should be 32
   // ECDSA P-256 compressed should be 33
   // ECDSA P-256 uncompressed should be 65
   ```
3. Ensure you are calling the correct verification method (`verifyWithEd25519` vs `verifyWithEcdsaP256`)

### TimestampValidationError

```
fr.acn.claim169.Claim169Exception$Expired: Token expired at 1700000000
```

**Cause:** The credential has expired or its `nbf` time is in the future.

**Solutions:**

1. Check if the credential should be rejected (it is expired)
2. For testing, disable timestamp validation:
   ```java
   DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
       builder.verifyWithEd25519(publicKey);
       builder.withoutTimestampValidation();
   });
   ```
3. Add clock skew tolerance:
   ```java
   DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
       builder.verifyWithEd25519(publicKey);
       builder.clockSkewTolerance(300);  // 5 minutes
   });
   ```

### DecryptionError

```
fr.acn.claim169.Claim169Exception$DecryptionFailed: Decryption failed
```

**Causes:**

- Wrong decryption key
- Wrong key size for algorithm
- Corrupted ciphertext

**Solutions:**

1. Verify key size matches algorithm:
   ```java
   System.out.println("Key length: " + encryptKey.length + " bytes");
   // AES-256: 32 bytes
   // AES-128: 16 bytes
   ```
2. Use the correct method for your key size:
   ```java
   // For 32-byte keys
   builder.decryptWithAes256(key32);

   // For 16-byte keys
   builder.decryptWithAes128(key16);
   ```

---

## Encoding Errors

### Invalid Key Length

```
java.lang.IllegalArgumentException: Ed25519 private key must be 32 bytes, got 64
```

**Solution:** Ensure your key is the correct length. If using a JCA-generated key, extract the raw 32-byte seed:

```java
import java.security.KeyPairGenerator;

KeyPairGenerator keyGen = KeyPairGenerator.getInstance("Ed25519");
var keyPair = keyGen.generateKeyPair();
byte[] privateKeyEncoded = keyPair.getPrivate().getEncoded();
// PKCS#8 encoded key is longer; extract the raw 32-byte seed
byte[] rawSeed = java.util.Arrays.copyOfRange(
    privateKeyEncoded, privateKeyEncoded.length - 32, privateKeyEncoded.length);
System.out.println("Raw key length: " + rawSeed.length);  // 32
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
   ```java
   DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
       builder.verifyWithEd25519(publicKey);
       builder.maxDecompressedBytes(32_768);  // 32KB limit
   });
   ```

2. Skipping biometrics when not needed:
   ```java
   DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
       builder.verifyWithEd25519(publicKey);
       builder.skipBiometrics();
   });
   ```

### Thread Safety

The `Claim169` static methods are thread-safe. You can call `decode()` and `encode()` concurrently from multiple threads without synchronization.

Custom crypto provider implementations (`SignatureVerifier`, `Signer`, `Decryptor`, `Encryptor`) must be thread-safe if shared across threads.

```java
import java.util.List;
import java.util.concurrent.*;

// Safe: concurrent decoding with virtual threads (JDK 21+)
try (var executor = Executors.newVirtualThreadPerTaskExecutor()) {
    List<Future<DecodeResultData>> futures = qrCodes.stream()
        .map(qr -> executor.submit(() ->
            Claim169.decode(qr, (DecoderConfigurer) builder -> {
                builder.verifyWithEd25519(publicKey);
            })
        ))
        .toList();

    for (Future<DecodeResultData> future : futures) {
        DecodeResultData result = future.get();
        System.out.println("Decoded: " + result.getClaim169().getFullName());
    }
}
```

### Native Memory

The SDK allocates native memory through JNA for Rust core operations. This memory is freed automatically when the JVM garbage collects the associated Java objects. On Android, be mindful of native memory limits in memory-constrained environments.

---

## Common Java Mistakes

### Forgetting to Convert Key from Hex

```java
// Wrong - passing hex string as UTF-8 bytes
byte[] publicKey = "d75a98...".getBytes();
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);  // Will fail!
});

// Correct - convert hex to bytes
byte[] publicKey = hexToByteArray("d75a98...");
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);  // Works!
});
```

### Forgetting the Configurer Cast

```java
// Wrong - ambiguous method call
DecodeResultData result = Claim169.decode(qrData, builder -> {
    builder.verifyWithEd25519(publicKey);
});

// Correct - explicit cast
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);
});
```

### Using Private Key for Verification

```java
// Wrong - using private key for verification
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(privateKey);  // May fail!
});

// Correct - use public key for verification
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);
});
```

### Missing Verification for Encrypted Credentials

```java
// Wrong - no verifier configured for encrypted credential
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.decryptWithAes256(key);
    // No verifier -> will fail unless allowUnverified() is set
});

// Correct - provide verifier
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.decryptWithAes256(key);
    builder.verifyWithEd25519(publicKey);
});

// Or for testing:
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.decryptWithAes256(key);
    builder.allowUnverified();
});
```

### hexToByteArray Utility

Java does not have a built-in hex decode method (prior to JDK 17's `HexFormat`). Use one of these approaches:

```java
// JDK 17+ (HexFormat)
import java.util.HexFormat;

byte[] key = HexFormat.of().parseHex("d75a980182b10ab7...");

// Manual utility
public static byte[] hexToByteArray(String hex) {
    int len = hex.length();
    if (len % 2 != 0) {
        throw new IllegalArgumentException("Hex string must have even length");
    }
    byte[] data = new byte[len / 2];
    for (int i = 0; i < len; i += 2) {
        data[i / 2] = (byte) ((Character.digit(hex.charAt(i), 16) << 4)
                             + Character.digit(hex.charAt(i + 1), 16));
    }
    return data;
}
```

---

## Getting Help

If you encounter an issue not covered here:

1. **Check the API reference** -- [api.md](api.md)
2. **Review examples** -- Check test files in the repository
3. **Open an issue** -- [GitHub Issues](https://github.com/jeremi/claim-169/issues)

When reporting issues, include:

- JDK version: `java -version`
- claim169 version: `Claim169.version()`
- Operating system and architecture
- Android API level (if applicable)
- Minimal code to reproduce the issue
- Full stack trace
