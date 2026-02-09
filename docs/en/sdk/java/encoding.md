# Encoding Credentials

This guide covers creating signed identity credentials that can be encoded in QR codes using Java.

## Overview

Encoding follows these steps:

1. Create a `Claim169Data` with `Claim169.claim169()` or `Claim169DataBuilder`
2. Create a `CwtMetaData` with `Claim169.cwtMeta()` or `CwtMetaDataBuilder`
3. Sign with a private key
4. Optionally encrypt with a symmetric key
5. Receive a Base45-encoded string for QR code generation

## Creating Identity Data

### Using the Configurer Pattern

The `Claim169.claim169()` static method accepts a `Claim169DataConfigurer` lambda:

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.Claim169Data;
import fr.acn.claim169.Claim169DataConfigurer;
import fr.acn.claim169.Gender;
import fr.acn.claim169.MaritalStatus;

// Create with all demographics
Claim169Data data = Claim169.claim169((Claim169DataConfigurer) builder -> {
    builder.setId("MOSIP-2024-001");
    builder.setVersion("1.0.0");
    builder.setLanguage("en");
    builder.setFullName("Jane Marie Doe");
    builder.setFirstName("Jane");
    builder.setMiddleName("Marie");
    builder.setLastName("Doe");
    builder.setDateOfBirth("1990-05-15");
    builder.setGenderEnum(Gender.Female);
    builder.setAddress("123 Main Street, Springfield, IL 62701");
    builder.setEmail("jane.doe@example.org");
    builder.setPhone("+1-555-123-4567");
    builder.setNationality("US");
    builder.setMaritalStatusEnum(MaritalStatus.Unmarried);
    builder.setGuardian("John Doe Sr.");
    builder.setSecondaryFullName("Juana Maria Doe");
    builder.setSecondaryLanguage("es");
    builder.setLocationCode("US-IL");
    builder.setLegalStatus("citizen");
    builder.setCountryOfIssuance("US");
});
```

### Using the Explicit Builder Pattern

You can also construct data directly with `Claim169DataBuilder`:

```java
import fr.acn.claim169.Claim169DataBuilder;
import fr.acn.claim169.Claim169Data;
import fr.acn.claim169.Gender;

Claim169DataBuilder dataBuilder = new Claim169DataBuilder();
dataBuilder.setId("MOSIP-2024-001");
dataBuilder.setFullName("Jane Marie Doe");
dataBuilder.setDateOfBirth("1990-05-15");
dataBuilder.setGenderEnum(Gender.Female);
dataBuilder.setEmail("jane.doe@example.org");
Claim169Data data = dataBuilder.build();
```

### Field Reference

| Field | Type | Description |
|-------|------|-------------|
| `id` | `String` | Unique identifier |
| `version` | `String` | Credential version |
| `language` | `String` | Primary language code (ISO 639-1) |
| `fullName` | `String` | Full name |
| `firstName` | `String` | First/given name |
| `middleName` | `String` | Middle name |
| `lastName` | `String` | Last/family name |
| `dateOfBirth` | `String` | Date of birth (YYYY-MM-DD) |
| `gender` | `Long` | 1=Male, 2=Female, 3=Other (or use `setGenderEnum()`) |
| `genderEnum` | `Gender` | Type-safe: `Gender.Male`, `Gender.Female`, `Gender.Other` |
| `address` | `String` | Full address |
| `email` | `String` | Email address |
| `phone` | `String` | Phone number |
| `nationality` | `String` | Nationality code |
| `maritalStatus` | `Long` | 1=Unmarried, 2=Married, 3=Divorced (or use `setMaritalStatusEnum()`) |
| `maritalStatusEnum` | `MaritalStatus` | Type-safe: `MaritalStatus.Unmarried`, `MaritalStatus.Married`, `MaritalStatus.Divorced` |
| `guardian` | `String` | Guardian name |
| `photo` | `byte[]` | Photo data |
| `photoFormat` | `Long` | 1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP (or use `setPhotoFormatEnum()`) |
| `photoFormatEnum` | `PhotoFormat` | Type-safe: `PhotoFormat.Jpeg`, `PhotoFormat.Jpeg2000`, `PhotoFormat.Avif`, `PhotoFormat.Webp` |
| `secondaryFullName` | `String` | Name in secondary language |
| `secondaryLanguage` | `String` | Secondary language code |
| `locationCode` | `String` | Location code |
| `legalStatus` | `String` | Legal status |
| `countryOfIssuance` | `String` | Issuing country code |

### Including a Photo

```java
import java.io.File;
import java.nio.file.Files;
import fr.acn.claim169.PhotoFormat;

byte[] photoData = Files.readAllBytes(new File("photo.jpg").toPath());

Claim169Data data = Claim169.claim169((Claim169DataConfigurer) builder -> {
    builder.setId("PHOTO-001");
    builder.setFullName("Jane Doe");
    builder.setPhoto(photoData);
    builder.setPhotoFormatEnum(PhotoFormat.Jpeg);
});
```

## Creating Token Metadata

### Using the Configurer Pattern

The `Claim169.cwtMeta()` static method accepts a `CwtMetaDataConfigurer` lambda:

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.CwtMetaData;
import fr.acn.claim169.CwtMetaDataConfigurer;

long now = System.currentTimeMillis() / 1000;

CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) builder -> {
    builder.setIssuer("https://id.example.org");
    builder.setSubject("user-12345");
    builder.setExpiresAt(now + (365 * 24 * 60 * 60));  // 1 year from now
    builder.setIssuedAt(now);
    builder.setNotBefore(now);  // Valid immediately
});
```

### Using the Explicit Builder Pattern

```java
import fr.acn.claim169.CwtMetaDataBuilder;
import fr.acn.claim169.CwtMetaData;

CwtMetaDataBuilder metaBuilder = new CwtMetaDataBuilder();
metaBuilder.setIssuer("https://id.example.org");
metaBuilder.setExpiresAt(1900000000L);
metaBuilder.setIssuedAt(1700000000L);
CwtMetaData meta = metaBuilder.build();
```

### Metadata Fields

| Field | Type | Description |
|-------|------|-------------|
| `issuer` | `String` | Credential issuer (URL or identifier) |
| `subject` | `String` | Subject identifier |
| `expiresAt` | `Long` | Expiration timestamp (Unix epoch) |
| `notBefore` | `Long` | Not valid before timestamp |
| `issuedAt` | `Long` | Issuance timestamp |

## Signing with Ed25519

Ed25519 is recommended for its small signatures and fast verification.

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.Claim169Data;
import fr.acn.claim169.Claim169DataConfigurer;
import fr.acn.claim169.CwtMetaData;
import fr.acn.claim169.CwtMetaDataConfigurer;
import fr.acn.claim169.EncoderConfigurer;

// Identity data
Claim169Data data = Claim169.claim169((Claim169DataConfigurer) builder -> {
    builder.setId("ED25519-001");
    builder.setFullName("Jane Doe");
    builder.setDateOfBirth("1990-05-15");
});

// Token metadata
CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) builder -> {
    builder.setIssuer("https://id.example.org");
    builder.setExpiresAt(1900000000L);
    builder.setIssuedAt(1700000000L);
});

// Ed25519 private key (32 bytes)
byte[] privateKey = hexToByteArray("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");

// Encode
String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    builder.signWithEd25519(privateKey);
});

System.out.println("Encoded: " + qrData.length() + " characters");
```

## Signing with ECDSA P-256

ECDSA P-256 is widely supported in enterprise environments.

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.EncoderConfigurer;

Claim169Data data = Claim169.claim169((Claim169DataConfigurer) builder -> {
    builder.setId("ECDSA-001");
    builder.setFullName("Jane Doe");
});

CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) builder -> {
    builder.setIssuer("https://id.example.org");
    builder.setExpiresAt(1900000000L);
});

// ECDSA P-256 private key (32 bytes)
byte[] privateKey = new byte[32];  // Replace with actual key

String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    builder.signWithEcdsaP256(privateKey);
});
```

## Encoding Without Signature

For testing and development only. Never use in production.

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.EncoderConfigurer;

Claim169Data data = Claim169.claim169((Claim169DataConfigurer) builder -> {
    builder.setId("TEST-001");
    builder.setFullName("Test User");
});

CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) builder -> {
    builder.setExpiresAt(1900000000L);
});

// Encode without signature (INSECURE - testing only)
String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    // No signing method configured
});
```

## Skipping Biometrics

To reduce QR code size, skip encoding biometric data:

```java
String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    builder.signWithEd25519(privateKey);
    builder.skipBiometrics();
});
```

## Custom Signer Callback

For HSM or KMS signing, provide a `Signer` implementation using an anonymous class:

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.Signer;
import fr.acn.claim169.CoseAlgorithm;
import fr.acn.claim169.EncoderConfigurer;

Signer customSigner = new Signer() {
    @Override
    public byte[] sign(String algorithm, byte[] keyId, byte[] data) {
        // Your HSM/KMS signing logic here
        return yourHsm.sign(data);
    }

    @Override
    public byte[] keyId() {
        return null;
    }
};

String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    builder.signWith(customSigner, CoseAlgorithm.EdDSA);
});
```

The `CoseAlgorithm` enum provides type-safe algorithm selection. You can also pass a raw string:

```java
builder.signWith(customSigner, "EdDSA");
```

## Full Example

Complete example with all demographics:

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.Claim169Data;
import fr.acn.claim169.Claim169DataConfigurer;
import fr.acn.claim169.CwtMetaData;
import fr.acn.claim169.CwtMetaDataConfigurer;
import fr.acn.claim169.EncoderConfigurer;
import fr.acn.claim169.Gender;
import fr.acn.claim169.MaritalStatus;

// Create comprehensive identity data
long now = System.currentTimeMillis() / 1000;

Claim169Data data = Claim169.claim169((Claim169DataConfigurer) builder -> {
    builder.setId("FULL-DEMO-2024-001");
    builder.setVersion("1.0.0");
    builder.setLanguage("en");
    builder.setFullName("Jane Marie Doe");
    builder.setFirstName("Jane");
    builder.setMiddleName("Marie");
    builder.setLastName("Doe");
    builder.setDateOfBirth("1990-05-15");
    builder.setGenderEnum(Gender.Female);
    builder.setAddress("123 Main Street, Springfield, IL 62701, USA");
    builder.setEmail("jane.doe@example.org");
    builder.setPhone("+1-555-123-4567");
    builder.setNationality("US");
    builder.setMaritalStatusEnum(MaritalStatus.Married);
    builder.setSecondaryFullName("Juana Maria Doe");
    builder.setSecondaryLanguage("es");
    builder.setLocationCode("US-IL-SPR");
    builder.setLegalStatus("citizen");
    builder.setCountryOfIssuance("US");
});

// Create token metadata
CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) builder -> {
    builder.setIssuer("https://id.state.il.us");
    builder.setSubject("IL-DL-2024-001");
    builder.setExpiresAt(now + (5L * 365 * 24 * 60 * 60));  // 5 years
    builder.setIssuedAt(now);
    builder.setNotBefore(now);
});

// Sign with Ed25519
byte[] privateKey = hexToByteArray("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");

String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    builder.signWithEd25519(privateKey);
});

System.out.println("QR Code content (" + qrData.length() + " characters)");
System.out.println("Ready for QR code generation");
```

## Error Handling

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.EncoderConfigurer;
import fr.acn.claim169.Claim169Exception;

try {
    String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
        builder.signWithEd25519(privateKey);
    });
} catch (IllegalArgumentException e) {
    System.out.println("Invalid key format: " + e.getMessage());
} catch (Claim169Exception e) {
    System.out.println("Encoding failed: " + e.getMessage());
}
```

## Next Steps

- [Encryption](encryption.md) -- Add AES-GCM encryption
- [Custom Crypto](custom-crypto.md) -- Use HSM/KMS for signing
- [API Reference](api.md) -- Complete class documentation
