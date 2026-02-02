# API Reference

Complete API documentation for the claim169 Java SDK. The Java SDK uses the same `claim169-core` artifact as the Kotlin SDK, accessed through Java-compatible entry points.

!!! note "Checked Exceptions"
    All decode and encode methods are annotated with `@Throws(Claim169Exception::class)` in the Kotlin source. Java callers must handle or declare `Claim169Exception` as a checked exception.

## Claim169

```java
public final class Claim169
```

Main entry point for the MOSIP Claim 169 SDK. Provides static methods for decode and encode operations.

### Decoding

```java
DecodeResultData result = Claim169.decode(qrText, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);
});
System.out.println(result.getClaim169().getFullName());
```

### Encoding

```java
String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    builder.signWithEd25519(privateKey);
});
```

### Static Methods

| Name | Signature | Summary |
|---|---|---|
| decode | `DecodeResultData decode(String qrText, DecoderConfigurer configure)` | Decode a Claim 169 QR code string. |
| decodeCloseable | `CloseableDecodeResult decodeCloseable(String qrText, DecoderConfigurer configure)` | Decode and return a closeable wrapper that zeroizes sensitive byte arrays when closed. |
| encode | `String encode(Claim169Data claim169, CwtMetaData cwtMeta, EncoderConfigurer configure)` | Encode Claim 169 data into a QR-ready Base45 string. |
| version | `String version()` | Get the native library version. |
| verificationStatus | `String verificationStatus(DecodeResultData result)` | Get the verification status string from a decode result. |
| claim169 | `Claim169Data claim169(Claim169DataConfigurer configure)` | Create a Claim169Data using a configurer lambda. |
| cwtMeta | `CwtMetaData cwtMeta(CwtMetaDataConfigurer configure)` | Create a CwtMetaData using a configurer lambda. |

### decode

```java
public static DecodeResultData decode(String qrText, DecoderConfigurer configure) throws Claim169Exception
```

Decode a Claim 169 QR code string.

**Parameters:**

| | |
|---|---|
| `qrText` | The Base45-encoded QR code content |
| `configure` | Configurer to set up verification, decryption, and options |

**Returns:** The decoded result containing claim data, CWT metadata, and verification status.

**Throws:** `Claim169Exception` on decode errors.

### decodeCloseable

```java
public static CloseableDecodeResult decodeCloseable(String qrText, DecoderConfigurer configure) throws Claim169Exception
```

Decode a Claim 169 QR code string and return a closeable wrapper that zeroizes sensitive byte arrays when closed.

### encode

```java
public static String encode(Claim169Data claim169, CwtMetaData cwtMeta, EncoderConfigurer configure) throws Claim169Exception
```

Encode Claim 169 data into a QR-ready Base45 string.

**Parameters:**

| | |
|---|---|
| `claim169` | The identity claim data |
| `cwtMeta` | The CWT metadata (issuer, expiration, etc.) |
| `configure` | Configurer to set up signing, encryption, and options |

**Returns:** The Base45-encoded QR string.

**Throws:** `Claim169Exception` on encode errors.

### version

```java
public static String version()
```

Get the native library version.

### verificationStatus

```java
public static String verificationStatus(DecodeResultData result)
```

Get the verification status string from a decode result.

**Returns:** A status string such as `"verified"`, `"skipped"`, etc.

### claim169

```java
public static Claim169Data claim169(Claim169DataConfigurer configure)
```

Create a `Claim169Data` using a configurer lambda.

### cwtMeta

```java
public static CwtMetaData cwtMeta(CwtMetaDataConfigurer configure)
```

Create a `CwtMetaData` using a configurer lambda.

---

## DecoderBuilder

```java
public class DecoderBuilder
```

Builder for decoding Claim 169 QR codes. Passed to the `DecoderConfigurer` lambda.

### Methods

| Name | Signature | Summary |
|---|---|---|
| verifyWithEd25519 | `void verifyWithEd25519(byte[] publicKey)` | Verify with an Ed25519 public key (32 raw bytes). |
| verifyWithEd25519Pem | `void verifyWithEd25519Pem(String pem)` | Verify with an Ed25519 public key in PEM format. |
| verifyWithEcdsaP256 | `void verifyWithEcdsaP256(byte[] publicKey)` | Verify with an ECDSA P-256 public key (SEC1-encoded, 33 or 65 bytes). |
| verifyWithEcdsaP256Pem | `void verifyWithEcdsaP256Pem(String pem)` | Verify with an ECDSA P-256 public key in PEM format. |
| verifyWith | `void verifyWith(SignatureVerifier verifier)` | Verify with a custom SignatureVerifier implementation. |
| allowUnverified | `void allowUnverified()` | Allow decoding without signature verification. |
| decryptWithAes256 | `void decryptWithAes256(byte[] key)` | Decrypt with AES-256-GCM (32-byte key). |
| decryptWithAes128 | `void decryptWithAes128(byte[] key)` | Decrypt with AES-128-GCM (16-byte key). |
| decryptWith | `void decryptWith(Decryptor decryptor)` | Decrypt with a custom Decryptor implementation. |
| skipBiometrics | `void skipBiometrics()` | Skip biometric data parsing for faster decoding. |
| maxDecompressedBytes | `void maxDecompressedBytes(long maxBytes)` | Set maximum decompressed size in bytes (default: 65536). |
| withoutTimestampValidation | `void withoutTimestampValidation()` | Disable timestamp validation (expiration and not-before checks). |
| clockSkewTolerance | `void clockSkewTolerance(long seconds)` | Set clock skew tolerance for timestamp validation (in seconds). |

---

## EncoderBuilder

```java
public class EncoderBuilder
```

Builder for encoding Claim 169 credentials into QR-ready strings. Passed to the `EncoderConfigurer` lambda.

### Methods

| Name | Signature | Summary |
|---|---|---|
| signWithEd25519 | `void signWithEd25519(byte[] privateKey)` | Sign with an Ed25519 private key (32 raw bytes). |
| signWithEcdsaP256 | `void signWithEcdsaP256(byte[] privateKey)` | Sign with an ECDSA P-256 private key (32-byte scalar). |
| signWith | `void signWith(Signer signer, String algorithm)` | Sign with a custom Signer implementation. |
| signWith | `void signWith(Signer signer, CoseAlgorithm algorithm)` | Sign with a custom Signer using a known COSE algorithm. |
| allowUnsigned | `void allowUnsigned()` | Allow encoding without a signature. |
| encryptWithAes256 | `void encryptWithAes256(byte[] key)` | Encrypt with AES-256-GCM (32-byte key). |
| encryptWithAes128 | `void encryptWithAes128(byte[] key)` | Encrypt with AES-128-GCM (16-byte key). |
| encryptWith | `void encryptWith(Encryptor encryptor, String algorithm)` | Encrypt with a custom Encryptor implementation. |
| encryptWith | `void encryptWith(Encryptor encryptor, CoseAlgorithm algorithm)` | Encrypt with a custom Encryptor using a known COSE algorithm. |
| skipBiometrics | `void skipBiometrics()` | Skip biometric data during encoding. |

---

## Claim169DataBuilder

```java
public class Claim169DataBuilder
```

Builder for creating `Claim169Data` instances. Use with the configurer pattern or construct directly.

### Usage (Configurer Pattern)

```java
Claim169Data data = Claim169.claim169((Claim169DataConfigurer) builder -> {
    builder.setId("ID-12345");
    builder.setFullName("Jane Doe");
    builder.setGenderEnum(Gender.Female);
});
```

### Usage (Direct Builder)

```java
Claim169DataBuilder builder = new Claim169DataBuilder();
builder.setId("ID-12345");
builder.setFullName("Jane Doe");
Claim169Data data = builder.build();
```

### Setter Methods

| Name | Parameter Type | Description |
|---|---|---|
| setId | `String` | Unique identifier |
| setVersion | `String` | Credential version |
| setLanguage | `String` | Primary language code |
| setFullName | `String` | Full name |
| setFirstName | `String` | First/given name |
| setMiddleName | `String` | Middle name |
| setLastName | `String` | Last/family name |
| setDateOfBirth | `String` | Date of birth (YYYY-MM-DD) |
| setGender | `Long` | Gender (1=Male, 2=Female, 3=Other) |
| setGenderEnum | `Gender` | Type-safe gender |
| setAddress | `String` | Full address |
| setEmail | `String` | Email address |
| setPhone | `String` | Phone number |
| setNationality | `String` | Nationality code |
| setMaritalStatus | `Long` | Marital status (1=Unmarried, 2=Married, 3=Divorced) |
| setMaritalStatusEnum | `MaritalStatus` | Type-safe marital status |
| setGuardian | `String` | Guardian name |
| setPhoto | `byte[]` | Photo data |
| setPhotoFormat | `Long` | Photo format (1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP) |
| setPhotoFormatEnum | `PhotoFormat` | Type-safe photo format |
| setSecondaryFullName | `String` | Name in secondary language |
| setSecondaryLanguage | `String` | Secondary language code |
| setLocationCode | `String` | Location code |
| setLegalStatus | `String` | Legal status |
| setCountryOfIssuance | `String` | Issuing country code |
| setUnknownFieldsJson | `String` | JSON map of unknown CBOR fields for forward compatibility |

### build

```java
public Claim169Data build()
```

Build the `Claim169Data` instance from the configured properties.

---

## CwtMetaDataBuilder

```java
public class CwtMetaDataBuilder
```

Builder for creating `CwtMetaData` instances.

### Usage (Configurer Pattern)

```java
CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) builder -> {
    builder.setIssuer("https://issuer.example.com");
    builder.setExpiresAt(1800000000L);
});
```

### Usage (Direct Builder)

```java
CwtMetaDataBuilder builder = new CwtMetaDataBuilder();
builder.setIssuer("https://issuer.example.com");
builder.setExpiresAt(1800000000L);
CwtMetaData meta = builder.build();
```

### Setter Methods

| Name | Parameter Type | Description |
|---|---|---|
| setIssuer | `String` | Credential issuer |
| setSubject | `String` | Subject identifier |
| setExpiresAt | `Long` | Expiration timestamp (Unix epoch) |
| setNotBefore | `Long` | Not valid before timestamp |
| setIssuedAt | `Long` | Issuance timestamp |

### build

```java
public CwtMetaData build()
```

Build the `CwtMetaData` instance from the configured properties.

---

## CloseableDecodeResult

```java
public class CloseableDecodeResult implements Closeable
```

A `Closeable` wrapper around `DecodeResultData` that zeroizes sensitive byte arrays (biometric templates, photos, and other binary fields) when `close()` is called.

### Usage

```java
try (CloseableDecodeResult result = Claim169.decodeCloseable(qrText, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);
})) {
    String name = result.getData().getClaim169().getFullName();
    // ... process credential
}
// All biometric and photo byte arrays are now zeroed.
```

### Methods

| Name | Signature | Summary |
|---|---|---|
| getData | `DecodeResultData getData()` | Get the underlying decode result. |
| close | `void close()` | Zeroize all sensitive byte arrays within the decoded credential. |

---

## Configurer Interfaces

These functional interfaces enable the Java lambda pattern for configuration.

### DecoderConfigurer

```java
@FunctionalInterface
public interface DecoderConfigurer {
    void configure(DecoderBuilder builder);
}
```

Used in `Claim169.decode()` and `Claim169.decodeCloseable()`.

### EncoderConfigurer

```java
@FunctionalInterface
public interface EncoderConfigurer {
    void configure(EncoderBuilder builder);
}
```

Used in `Claim169.encode()`.

### Claim169DataConfigurer

```java
@FunctionalInterface
public interface Claim169DataConfigurer {
    void configure(Claim169DataBuilder builder);
}
```

Used in `Claim169.claim169()`.

### CwtMetaDataConfigurer

```java
@FunctionalInterface
public interface CwtMetaDataConfigurer {
    void configure(CwtMetaDataBuilder builder);
}
```

Used in `Claim169.cwtMeta()`.

!!! tip "Lambda Ambiguity"
    When calling `Claim169.decode()` from Java, the compiler may see both the Kotlin lambda overload and the `DecoderConfigurer` overload. Use an explicit cast to resolve the ambiguity: `(DecoderConfigurer) builder -> { ... }`.

---

## Crypto Interfaces

### SignatureVerifier

```java
public interface SignatureVerifier {
    void verify(String algorithm, byte[] keyId, byte[] data, byte[] signature);
}
```

Interface for custom signature verification. Implement this for HSM, KMS, or other custom crypto backends.

**Parameters:**

| | |
|---|---|
| `algorithm` | COSE algorithm name (e.g., `"EdDSA"`, `"ES256"`) |
| `keyId` | Optional key identifier bytes (nullable) |
| `data` | The data that was signed (COSE Sig_structure) |
| `signature` | The signature bytes to verify |

Throw any exception to indicate verification failure. Return normally to indicate success.

### Signer

```java
public interface Signer {
    byte[] sign(String algorithm, byte[] keyId, byte[] data);
    byte[] keyId();
}
```

Interface for custom signing. Implement this for HSM, KMS, or other custom crypto backends.

**sign() Parameters:**

| | |
|---|---|
| `algorithm` | COSE algorithm name (e.g., `"EdDSA"`, `"ES256"`) |
| `keyId` | Optional key identifier bytes (nullable) |
| `data` | The data to sign (COSE Sig_structure) |

**Returns:** The signature bytes.

**keyId():** Returns the key ID for this signer, or `null` if no key ID.

### Decryptor

```java
public interface Decryptor {
    byte[] decrypt(String algorithm, byte[] keyId, byte[] nonce, byte[] aad, byte[] ciphertext);
}
```

Interface for custom decryption.

**Parameters:**

| | |
|---|---|
| `algorithm` | COSE algorithm name (e.g., `"A256GCM"`) |
| `keyId` | Optional key identifier bytes (nullable) |
| `nonce` | The IV/nonce |
| `aad` | Additional authenticated data |
| `ciphertext` | The ciphertext to decrypt (includes auth tag for AEAD) |

**Returns:** The decrypted plaintext bytes.

### Encryptor

```java
public interface Encryptor {
    byte[] encrypt(String algorithm, byte[] keyId, byte[] nonce, byte[] aad, byte[] plaintext);
}
```

Interface for custom encryption.

**Parameters:**

| | |
|---|---|
| `algorithm` | COSE algorithm name (e.g., `"A256GCM"`) |
| `keyId` | Optional key identifier bytes (nullable) |
| `nonce` | The IV/nonce |
| `aad` | Additional authenticated data |
| `plaintext` | The plaintext to encrypt |

**Returns:** The ciphertext bytes (includes auth tag for AEAD).

---

## Enums

All enums provide a `fromValue()` static method for converting numeric values. From Java, access via the `Companion` field or directly if annotated with `@JvmStatic`.

### Gender

```java
public enum Gender {
    Male,    // value = 1
    Female,  // value = 2
    Other    // value = 3
}
```

| Method | Signature | Summary |
|---|---|---|
| fromValue | `Gender fromValue(long value)` | Convert numeric value to Gender (returns null if unknown). |
| getValue | `long getValue()` | Get the numeric value. |

```java
Gender gender = Gender.fromValue(2L);  // Gender.Female
long value = Gender.Female.getValue(); // 2
```

### MaritalStatus

```java
public enum MaritalStatus {
    Unmarried,  // value = 1
    Married,    // value = 2
    Divorced    // value = 3
}
```

| Method | Signature | Summary |
|---|---|---|
| fromValue | `MaritalStatus fromValue(long value)` | Convert numeric value to MaritalStatus. |
| getValue | `long getValue()` | Get the numeric value. |

### PhotoFormat

```java
public enum PhotoFormat {
    Jpeg,      // value = 1
    Jpeg2000,  // value = 2
    Avif,      // value = 3
    Webp       // value = 4
}
```

| Method | Signature | Summary |
|---|---|---|
| fromValue | `PhotoFormat fromValue(long value)` | Convert numeric value to PhotoFormat. |
| getValue | `long getValue()` | Get the numeric value. |

### CoseAlgorithm

```java
public enum CoseAlgorithm {
    EdDSA,    // coseName = "EdDSA"
    ES256,    // coseName = "ES256"
    A256GCM,  // coseName = "A256GCM"
    A128GCM   // coseName = "A128GCM"
}
```

| Method | Signature | Summary |
|---|---|---|
| getCoseName | `String getCoseName()` | Get the COSE algorithm name string. |

### VerificationStatus

```java
public enum VerificationStatus {
    Verified,
    Skipped,
    NotVerified
}
```

| Method | Signature | Summary |
|---|---|---|
| fromValue | `VerificationStatus fromValue(String value)` | Convert status string to enum. |
| getValue | `String getValue()` | Get the status string value. |

---

## Claim169Exception

```java
public sealed class Claim169Exception extends Exception
```

Sealed exception hierarchy for all SDK errors. Each subclass represents a specific stage of the decode/encode pipeline.

### Subclasses

| Exception | Description |
|---|---|
| `Claim169Exception.Base45Decode` | Invalid Base45 encoding |
| `Claim169Exception.Decompress` | Decompression failed (corrupted data or size limit exceeded) |
| `Claim169Exception.CoseParse` | Invalid COSE_Sign1 or COSE_Encrypt0 structure |
| `Claim169Exception.CwtParse` | Invalid CWT claims |
| `Claim169Exception.Claim169NotFound` | Claim tag 169 not present in CWT payload |
| `Claim169Exception.SignatureInvalid` | Signature verification failed |
| `Claim169Exception.DecryptionFailed` | Decryption failed |
| `Claim169Exception.Expired` | Token has expired |
| `Claim169Exception.NotYetValid` | Token not yet valid (before `nbf` time) |

### Usage

```java
try {
    DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
        builder.verifyWithEd25519(publicKey);
    });
} catch (Claim169Exception.SignatureInvalid e) {
    System.out.println("Signature invalid: " + e.getMessage());
} catch (Claim169Exception e) {
    System.out.println("Decode failed: " + e.getMessage());
}
```
