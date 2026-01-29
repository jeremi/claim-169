# Java Usage Guide

The Claim 169 Kotlin SDK is fully usable from Java. There is no separate Java artifact — you use the same `claim169-core` Maven coordinates from both Kotlin and Java.

## Installation

Installation is identical to Kotlin. See the [Installation guide](installation.md) for Gradle and Maven setup.

**Requirements:** JDK 17+

## Decoding a QR Code

In Kotlin, the `decode()` function accepts a trailing lambda (DSL). From Java, use the `DecoderConfigurer` functional interface instead:

```java
import org.acn.claim169.Claim169;
import org.acn.claim169.DecodeResultData;

byte[] publicKey = hexToByteArray("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a");

DecodeResultData result = Claim169.decode(qrData, builder -> {
    builder.verifyWithEd25519(publicKey);
});

System.out.println("ID: " + result.getClaim169().getId());
System.out.println("Name: " + result.getClaim169().getFullName());
System.out.println("Verified: " + result.isVerified());
```

### With ECDSA P-256

```java
byte[] publicKey = hexToByteArray("04...");

DecodeResultData result = Claim169.decode(qrData, builder -> {
    builder.verifyWithEcdsaP256(publicKey);
});
```

### With PEM Public Keys

```java
String pemKey = "-----BEGIN PUBLIC KEY-----\n"
    + "MCowBQYDK2VwAyEA11qYAYKxCrfVS/7TyWQHOg7hcvPapjJa8CCWX4cBURo=\n"
    + "-----END PUBLIC KEY-----";

DecodeResultData result = Claim169.decode(qrData, builder -> {
    builder.verifyWithEd25519Pem(pemKey);
});
```

### Without Verification (Testing Only)

```java
// WARNING: INSECURE - skips signature verification
DecodeResultData result = Claim169.decode(qrData, builder -> {
    builder.allowUnverified();
});
```

### Timestamp Options

```java
DecodeResultData result = Claim169.decode(qrData, builder -> {
    builder.verifyWithEd25519(publicKey);
    builder.withoutTimestampValidation();
});

// Or with clock skew tolerance
DecodeResultData result = Claim169.decode(qrData, builder -> {
    builder.verifyWithEd25519(publicKey);
    builder.clockSkewTolerance(60); // 60 seconds
});
```

## Closeable Decode (Memory Zeroization)

Use Java's try-with-resources to ensure biometric byte arrays are zeroized after use:

```java
import org.acn.claim169.Claim169;
import org.acn.claim169.CloseableDecodeResult;

try (CloseableDecodeResult result = Claim169.decodeCloseable(qrData, builder -> {
    builder.verifyWithEd25519(publicKey);
})) {
    String name = result.getData().getClaim169().getFullName();
    byte[] photo = result.getData().getClaim169().getPhoto();
    // ... process credential
}
// All biometric and photo byte arrays are now zeroed
```

## Encoding a Credential

From Java, use `Claim169DataBuilder` and `CwtMetaDataBuilder` directly, then pass an `EncoderConfigurer`:

```java
import org.acn.claim169.Claim169;
import org.acn.claim169.Claim169Data;
import org.acn.claim169.Claim169DataBuilder;
import org.acn.claim169.CwtMetaData;
import org.acn.claim169.CwtMetaDataBuilder;

// Build identity data
Claim169DataBuilder dataBuilder = new Claim169DataBuilder();
dataBuilder.setId("MOSIP-2024-001");
dataBuilder.setFullName("Jane Doe");
dataBuilder.setDateOfBirth("1990-05-15");
dataBuilder.setGender(2L); // 1=Male, 2=Female, 3=Other
dataBuilder.setEmail("jane.doe@example.org");
Claim169Data data = dataBuilder.build();

// Build CWT metadata
CwtMetaDataBuilder metaBuilder = new CwtMetaDataBuilder();
metaBuilder.setIssuer("https://id.example.org");
metaBuilder.setExpiresAt(1900000000L);
metaBuilder.setIssuedAt(1700000000L);
CwtMetaData meta = metaBuilder.build();

// Encode with Ed25519 signature
byte[] privateKey = hexToByteArray("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");

String qrData = Claim169.encode(data, meta, builder -> {
    builder.signWithEd25519(privateKey);
});
```

### Encoding Without Signature (Testing Only)

```java
String qrData = Claim169.encode(data, meta, builder -> {
    builder.allowUnsigned();
});
```

## Enum Handling

Kotlin enums use companion object methods. From Java, access them via the `Companion` field:

### Reading Enums from Decoded Data

```java
import org.acn.claim169.Gender;
import org.acn.claim169.MaritalStatus;
import org.acn.claim169.PhotoFormat;

DecodeResultData result = Claim169.decode(qrData, builder -> {
    builder.allowUnverified();
    builder.withoutTimestampValidation();
});

// Convert raw Long value to type-safe enum
Long genderValue = result.getClaim169().getGender();
if (genderValue != null) {
    Gender gender = Gender.Companion.fromValue(genderValue);
    System.out.println("Gender: " + gender); // Gender.Female
}

Long maritalValue = result.getClaim169().getMaritalStatus();
if (maritalValue != null) {
    MaritalStatus status = MaritalStatus.Companion.fromValue(maritalValue);
    System.out.println("Marital Status: " + status);
}
```

### Setting Enums When Building Data

```java
Claim169DataBuilder dataBuilder = new Claim169DataBuilder();
dataBuilder.setId("ENUM-JAVA-001");
dataBuilder.setFullName("Jane Doe");
dataBuilder.setGenderEnum(Gender.Female);
dataBuilder.setMaritalStatusEnum(MaritalStatus.Married);
dataBuilder.setPhotoFormatEnum(PhotoFormat.Jpeg);
Claim169Data data = dataBuilder.build();
```

### Verification Status Enum

```java
import org.acn.claim169.VerificationStatus;

DecodeResultData result = Claim169.decode(qrData, builder -> {
    builder.verifyWithEd25519(publicKey);
});

VerificationStatus status = result.verificationStatusEnum();
if (status == VerificationStatus.Verified) {
    System.out.println("Signature verified");
}
```

## Signature Verification

### Built-in Verifiers

```java
// Ed25519 (raw key bytes)
Claim169.decode(qrData, builder -> {
    builder.verifyWithEd25519(publicKey);
});

// Ed25519 (PEM format)
Claim169.decode(qrData, builder -> {
    builder.verifyWithEd25519Pem(pemString);
});

// ECDSA P-256 (raw key bytes)
Claim169.decode(qrData, builder -> {
    builder.verifyWithEcdsaP256(publicKey);
});

// ECDSA P-256 (PEM format)
Claim169.decode(qrData, builder -> {
    builder.verifyWithEcdsaP256Pem(pemString);
});
```

### Custom Verifier

Implement the `SignatureVerifier` interface:

```java
import org.acn.claim169.SignatureVerifier;

SignatureVerifier verifier = new SignatureVerifier() {
    @Override
    public void verify(String algorithm, byte[] keyId, byte[] data, byte[] signature) {
        // Your verification logic here
        // Throw an exception if verification fails
    }
};

DecodeResultData result = Claim169.decode(qrData, builder -> {
    builder.verifyWith(verifier);
});
```

## Encryption and Decryption

### Encoding with Encryption

```java
byte[] signKey = hexToByteArray("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");
byte[] encryptKey = hexToByteArray("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");

String qrData = Claim169.encode(data, meta, builder -> {
    builder.signWithEd25519(signKey);
    builder.encryptWithAes256(encryptKey);
});
```

### Decoding with Decryption

```java
byte[] encryptKey = hexToByteArray("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
byte[] publicKey = hexToByteArray("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a");

DecodeResultData result = Claim169.decode(qrData, builder -> {
    builder.decryptWithAes256(encryptKey);
    builder.verifyWithEd25519(publicKey);
});
```

### Custom Decryptor

```java
import org.acn.claim169.Decryptor;

Decryptor decryptor = new Decryptor() {
    @Override
    public byte[] decrypt(String algorithm, byte[] keyId, byte[] nonce, byte[] aad, byte[] ciphertext) {
        // Your decryption logic
        return yourHsm.decrypt(algorithm, nonce, aad, ciphertext);
    }
};

DecodeResultData result = Claim169.decode(qrData, builder -> {
    builder.decryptWith(decryptor);
    builder.verifyWithEd25519(publicKey);
});
```

### Custom Encryptor

```java
import org.acn.claim169.Encryptor;
import org.acn.claim169.CoseAlgorithm;

Encryptor encryptor = new Encryptor() {
    @Override
    public byte[] encrypt(String algorithm, byte[] keyId, byte[] nonce, byte[] aad, byte[] plaintext) {
        return yourHsm.encrypt(nonce, aad, plaintext);
    }
};

String qrData = Claim169.encode(data, meta, builder -> {
    builder.signWithEd25519(signKey);
    builder.encryptWith(encryptor, CoseAlgorithm.A256GCM);
});
```

## Custom Crypto Providers

All four crypto interfaces (`SignatureVerifier`, `Signer`, `Decryptor`, `Encryptor`) can be implemented as Java classes or anonymous classes.

### Custom Signer

```java
import org.acn.claim169.Signer;
import org.acn.claim169.CoseAlgorithm;

Signer signer = new Signer() {
    @Override
    public byte[] sign(String algorithm, byte[] keyId, byte[] data) {
        // Your HSM/KMS signing logic
        return yourHsm.sign(data);
    }

    @Override
    public byte[] keyId() {
        return null; // Optional key identifier
    }
};

String qrData = Claim169.encode(data, meta, builder -> {
    builder.signWith(signer, CoseAlgorithm.EdDSA);
});
```

## Error Handling

Java catches the same `Claim169Exception` sealed class hierarchy. Use `instanceof` checks:

```java
import org.acn.claim169.Claim169;
import uniffi.claim169_jni.Claim169Exception;

try {
    DecodeResultData result = Claim169.decode(qrData, builder -> {
        builder.verifyWithEd25519(publicKey);
    });
} catch (Claim169Exception.Base45DecodeError e) {
    System.out.println("QR code format error: " + e.getMessage());
} catch (Claim169Exception.SignatureError e) {
    System.out.println("Invalid signature: " + e.getMessage());
} catch (Claim169Exception.TimestampValidationError e) {
    System.out.println("Token expired or not yet valid: " + e.getMessage());
} catch (Claim169Exception.DecryptionError e) {
    System.out.println("Decryption failed: " + e.getMessage());
} catch (Claim169Exception e) {
    System.out.println("Decode error: " + e.getMessage());
}
```

## Kotlin vs Java Comparison

| Operation | Kotlin | Java |
|-----------|--------|------|
| Decode | `Claim169.decode(qr) { verifyWithEd25519(key) }` | `Claim169.decode(qr, b -> { b.verifyWithEd25519(key); })` |
| Encode | `Claim169.encode(data, meta) { signWithEd25519(key) }` | `Claim169.encode(data, meta, b -> { b.signWithEd25519(key); })` |
| Build data | `claim169 { id = "X"; fullName = "Y" }` | `new Claim169DataBuilder(); b.setId("X"); b.setFullName("Y"); b.build()` |
| Build meta | `cwtMeta { issuer = "X" }` | `new CwtMetaDataBuilder(); b.setIssuer("X"); b.build()` |
| Closeable | `.use { result -> ... }` | `try (var r = ...) { ... }` |
| Enum access | `Gender.fromValue(v)` | `Gender.Companion.fromValue(v)` |
| Enum set | `genderEnum = Gender.Female` | `b.setGenderEnum(Gender.Female)` |
| Error handling | `when (e) { is ...Error -> }` | `catch (Claim169Exception.SignatureError e)` |

## Next Steps

- [Decoding Guide](decoding.md) -- Advanced decoding options (Kotlin examples)
- [Encoding Guide](encoding.md) -- Detailed encoding with all demographics
- [Encryption](encryption.md) -- AES-GCM encryption
- [Custom Crypto](custom-crypto.md) -- HSM/KMS integration
- [API Reference](api.md) -- Complete class documentation
