# API Reference

Complete API documentation for the claim169 Kotlin SDK.

## Claim169 Object

The `Claim169` singleton is the main entry point for encoding and decoding.

### version()

Get the library version.

```kotlin
fun version(): String
```

**Returns:** Version string in semver format (e.g., "0.1.0-alpha.2")

**Example:**
```kotlin
import org.acn.claim169.Claim169

println(Claim169.version())  // "0.1.0-alpha.2"
```

---

### decode()

Decode a Base45-encoded QR code string using a builder DSL.

```kotlin
fun decode(qrText: String, builder: DecoderBuilder.() -> Unit): DecodeResultData
```

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `qrText` | `String` | Base45-encoded QR content |
| `builder` | `DecoderBuilder.() -> Unit` | DSL block to configure decoding options |

**Returns:** `DecodeResultData`

**Throws:** `Claim169Exception` (sealed class with specific subtypes)

**Example:**
```kotlin
val result = Claim169.decode("NCFOXN...") {
    verifyWithEd25519(publicKey)
}
```

---

### encode()

Encode identity data into a Base45-encoded string using a builder DSL.

```kotlin
fun encode(data: Claim169Data, meta: CwtMetaData, builder: EncoderBuilder.() -> Unit): String
```

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `data` | `Claim169Data` | Identity data (created with `claim169 {}` builder) |
| `meta` | `CwtMetaData` | CWT metadata (created with `cwtMeta {}` builder) |
| `builder` | `EncoderBuilder.() -> Unit` | DSL block to configure encoding options |

**Returns:** Base45-encoded string

**Throws:** `Claim169Exception`, `IllegalArgumentException`

**Example:**
```kotlin
val qrData = Claim169.encode(data, meta) {
    signWithEd25519(privateKey)
}
```

---

## DecoderBuilder

Builder class for configuring decode options. Used within `Claim169.decode() {}`.

### verifyWithEd25519()

Configure Ed25519 signature verification with a raw 32-byte public key.

```kotlin
fun verifyWithEd25519(publicKey: ByteArray)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `publicKey` | `ByteArray` | 32-byte Ed25519 public key |

---

### verifyWithEcdsaP256()

Configure ECDSA P-256 signature verification with a raw public key.

```kotlin
fun verifyWithEcdsaP256(publicKey: ByteArray)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `publicKey` | `ByteArray` | SEC1 encoded P-256 public key (33 or 65 bytes) |

---

### verifyWithEd25519Pem()

Configure Ed25519 signature verification with a PEM-encoded public key.

```kotlin
fun verifyWithEd25519Pem(pem: String)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `pem` | `String` | PEM-encoded Ed25519 public key |

---

### verifyWithEcdsaP256Pem()

Configure ECDSA P-256 signature verification with a PEM-encoded public key.

```kotlin
fun verifyWithEcdsaP256Pem(pem: String)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `pem` | `String` | PEM-encoded ECDSA P-256 public key |

---

### verifyWith()

Configure custom signature verification using a `SignatureVerifier` implementation.

```kotlin
fun verifyWith(verifier: SignatureVerifier)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `verifier` | `SignatureVerifier` | Custom verifier implementation |

---

### allowUnverified()

Skip signature verification. **INSECURE -- testing only.**

```kotlin
fun allowUnverified()
```

---

### decryptWithAes256()

Configure AES-256-GCM decryption.

```kotlin
fun decryptWithAes256(key: ByteArray)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `key` | `ByteArray` | 32-byte AES-256 key |

---

### decryptWithAes128()

Configure AES-128-GCM decryption.

```kotlin
fun decryptWithAes128(key: ByteArray)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `key` | `ByteArray` | 16-byte AES-128 key |

---

### decryptWith()

Configure custom decryption using a `Decryptor` implementation.

```kotlin
fun decryptWith(decryptor: Decryptor)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `decryptor` | `Decryptor` | Custom decryptor implementation |

---

### skipBiometrics()

Skip parsing biometric data for faster decoding.

```kotlin
fun skipBiometrics()
```

---

### maxDecompressedBytes()

Set the maximum decompressed size limit (default: 65536).

```kotlin
fun maxDecompressedBytes(limit: Int)
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `limit` | `Int` | `65536` | Maximum decompressed size in bytes |

---

### withoutTimestampValidation()

Disable exp/nbf timestamp validation.

```kotlin
fun withoutTimestampValidation()
```

---

### clockSkewTolerance()

Set clock skew tolerance for timestamp validation.

```kotlin
fun clockSkewTolerance(seconds: Int)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `seconds` | `Int` | Number of seconds of clock drift to tolerate |

---

## EncoderBuilder

Builder class for configuring encode options. Used within `Claim169.encode() {}`.

### signWithEd25519()

Configure Ed25519 signing.

```kotlin
fun signWithEd25519(privateKey: ByteArray)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `privateKey` | `ByteArray` | 32-byte Ed25519 private key |

---

### signWithEcdsaP256()

Configure ECDSA P-256 signing.

```kotlin
fun signWithEcdsaP256(privateKey: ByteArray)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `privateKey` | `ByteArray` | 32-byte ECDSA P-256 private key |

---

### signWith()

Configure custom signing using a `Signer` implementation.

```kotlin
fun signWith(signer: Signer)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `signer` | `Signer` | Custom signer implementation |

---

### encryptWithAes256()

Configure AES-256-GCM encryption.

```kotlin
fun encryptWithAes256(key: ByteArray)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `key` | `ByteArray` | 32-byte AES-256 key |

---

### encryptWithAes128()

Configure AES-128-GCM encryption.

```kotlin
fun encryptWithAes128(key: ByteArray)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `key` | `ByteArray` | 16-byte AES-128 key |

---

### encryptWith()

Configure custom encryption using an `Encryptor` implementation.

```kotlin
fun encryptWith(encryptor: Encryptor)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `encryptor` | `Encryptor` | Custom encryptor implementation |

---

### skipBiometrics()

Exclude biometric data from the encoded output.

```kotlin
fun skipBiometrics()
```

---

## Claim169DataBuilder

Builder for creating identity data. Used via the `claim169 {}` DSL function.

### DSL Function

```kotlin
fun claim169(block: Claim169DataBuilder.() -> Unit): Claim169Data
```

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `id` | `String?` | Unique identifier |
| `version` | `String?` | Credential version |
| `language` | `String?` | Primary language code (ISO 639-1) |
| `fullName` | `String?` | Full name |
| `firstName` | `String?` | First/given name |
| `middleName` | `String?` | Middle name |
| `lastName` | `String?` | Last/family name |
| `dateOfBirth` | `String?` | Date of birth (YYYY-MM-DD) |
| `gender` | `Int?` | Gender (see Gender values table) |
| `address` | `String?` | Full address |
| `email` | `String?` | Email address |
| `phone` | `String?` | Phone number |
| `nationality` | `String?` | Nationality code |
| `maritalStatus` | `Int?` | Marital status (see Marital Status values table) |
| `guardian` | `String?` | Guardian name |
| `photo` | `ByteArray?` | Photo data |
| `photoFormat` | `Int?` | Photo format (see Photo Format values table) |
| `bestQualityFingers` | `ByteArray?` | Best quality finger indices |
| `secondaryFullName` | `String?` | Name in secondary language |
| `secondaryLanguage` | `String?` | Secondary language code |
| `locationCode` | `String?` | Location code |
| `legalStatus` | `String?` | Legal status |
| `countryOfIssuance` | `String?` | Issuing country code |

---

## CwtMetaDataBuilder

Builder for creating CWT metadata. Used via the `cwtMeta {}` DSL function.

### DSL Function

```kotlin
fun cwtMeta(block: CwtMetaDataBuilder.() -> Unit): CwtMetaData
```

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `issuer` | `String?` | Credential issuer (URL or identifier) |
| `subject` | `String?` | Subject identifier |
| `expiresAt` | `Long?` | Expiration timestamp (Unix epoch seconds) |
| `notBefore` | `Long?` | Not valid before timestamp (Unix epoch seconds) |
| `issuedAt` | `Long?` | Issuance timestamp (Unix epoch seconds) |

---

## Crypto Interfaces

### SignatureVerifier

Interface for custom signature verification.

```kotlin
interface SignatureVerifier {
    fun verify(
        algorithm: String,
        keyId: ByteArray?,
        data: ByteArray,
        signature: ByteArray
    )
}
```

The `verify` method should throw an exception if verification fails. Returning normally indicates success.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `algorithm` | `String` | Algorithm name ("EdDSA" or "ES256") |
| `keyId` | `ByteArray?` | Optional key identifier from COSE header |
| `data` | `ByteArray` | Data that was signed |
| `signature` | `ByteArray` | Signature to verify |

---

### Signer

Interface for custom signing.

```kotlin
interface Signer {
    val algorithm: String
    val keyId: ByteArray?

    fun sign(data: ByteArray): ByteArray
}
```

**Properties:**

| Name | Type | Description |
|------|------|-------------|
| `algorithm` | `String` | Algorithm name ("EdDSA" or "ES256") |
| `keyId` | `ByteArray?` | Optional key identifier for COSE header |

**Methods:**

- `sign(data: ByteArray): ByteArray` -- Sign the data and return the signature bytes

---

### Decryptor

Interface for custom decryption.

```kotlin
interface Decryptor {
    fun decrypt(
        algorithm: String,
        keyId: ByteArray?,
        nonce: ByteArray,
        aad: ByteArray,
        ciphertext: ByteArray
    ): ByteArray
}
```

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `algorithm` | `String` | Algorithm name ("A256GCM" or "A128GCM") |
| `keyId` | `ByteArray?` | Optional key identifier from COSE header |
| `nonce` | `ByteArray` | 12-byte nonce |
| `aad` | `ByteArray` | Additional authenticated data |
| `ciphertext` | `ByteArray` | Encrypted data with authentication tag |

**Returns:** Decrypted plaintext bytes

---

### Encryptor

Interface for custom encryption.

```kotlin
interface Encryptor {
    val algorithm: String
    val keyId: ByteArray?

    fun encrypt(
        nonce: ByteArray,
        aad: ByteArray,
        plaintext: ByteArray
    ): ByteArray
}
```

**Properties:**

| Name | Type | Description |
|------|------|-------------|
| `algorithm` | `String` | Algorithm name ("A256GCM" or "A128GCM") |
| `keyId` | `ByteArray?` | Optional key identifier for COSE header |

**Methods:**

- `encrypt(nonce, aad, plaintext): ByteArray` -- Encrypt the data and return ciphertext with authentication tag

---

## Data Classes

### DecodeResultData

Result of decoding a credential.

```kotlin
data class DecodeResultData(
    val claim169: Claim169Data,
    val cwtMeta: CwtMetaData,
    val verificationStatus: String
) {
    val isVerified: Boolean
}
```

**Properties:**

| Name | Type | Description |
|------|------|-------------|
| `claim169` | `Claim169Data` | Decoded identity data |
| `cwtMeta` | `CwtMetaData` | CWT metadata |
| `verificationStatus` | `String` | "verified", "skipped", etc. |
| `isVerified` | `Boolean` | `true` if signature was verified |

---

### Claim169Data

Decoded identity claim data.

```kotlin
data class Claim169Data(
    val id: String?,
    val version: String?,
    val language: String?,
    val fullName: String?,
    val firstName: String?,
    val middleName: String?,
    val lastName: String?,
    val dateOfBirth: String?,
    val gender: Int?,
    val address: String?,
    val email: String?,
    val phone: String?,
    val nationality: String?,
    val maritalStatus: Int?,
    val guardian: String?,
    val photo: ByteArray?,
    val photoFormat: Int?,
    val bestQualityFingers: ByteArray?,
    val secondaryFullName: String?,
    val secondaryLanguage: String?,
    val locationCode: String?,
    val legalStatus: String?,
    val countryOfIssuance: String?,
    val rightThumb: List<BiometricData>?,
    val rightPointerFinger: List<BiometricData>?,
    val rightMiddleFinger: List<BiometricData>?,
    val rightRingFinger: List<BiometricData>?,
    val rightLittleFinger: List<BiometricData>?,
    val leftThumb: List<BiometricData>?,
    val leftPointerFinger: List<BiometricData>?,
    val leftMiddleFinger: List<BiometricData>?,
    val leftRingFinger: List<BiometricData>?,
    val leftLittleFinger: List<BiometricData>?,
    val rightIris: List<BiometricData>?,
    val leftIris: List<BiometricData>?,
    val face: List<BiometricData>?,
    val rightPalm: List<BiometricData>?,
    val leftPalm: List<BiometricData>?,
    val voice: List<BiometricData>?
)
```

**Methods:**

- `hasBiometrics(): Boolean` -- Returns `true` if any biometric data is present

---

### CwtMetaData

CWT metadata from decoded credential.

```kotlin
data class CwtMetaData(
    val issuer: String?,
    val subject: String?,
    val expiresAt: Long?,
    val notBefore: Long?,
    val issuedAt: Long?
)
```

---

### BiometricData

Biometric data container.

```kotlin
data class BiometricData(
    val data: ByteArray,
    val format: Int?,
    val subFormat: Int?,
    val issuer: String?
)
```

**Properties:**

| Name | Type | Description |
|------|------|-------------|
| `data` | `ByteArray` | Raw biometric data |
| `format` | `Int?` | Format code |
| `subFormat` | `Int?` | Sub-format code |
| `issuer` | `String?` | Biometric issuer |

---

## Exception Hierarchy

All exceptions are subclasses of `Claim169Exception`, which is a sealed class enabling exhaustive `when` expressions.

### Claim169Exception

```kotlin
sealed class Claim169Exception(message: String) : Exception(message)
```

### Base45DecodeError

Raised when Base45 decoding fails.

```kotlin
class Base45DecodeError(message: String) : Claim169Exception(message)
```

### DecompressError

Raised when zlib decompression fails or size limit is exceeded.

```kotlin
class DecompressError(message: String) : Claim169Exception(message)
```

### CoseParseError

Raised when COSE structure parsing fails.

```kotlin
class CoseParseError(message: String) : Claim169Exception(message)
```

### CwtParseError

Raised when CWT parsing fails.

```kotlin
class CwtParseError(message: String) : Claim169Exception(message)
```

### Claim169NotFoundError

Raised when Claim 169 is not present in the CWT.

```kotlin
class Claim169NotFoundError(message: String) : Claim169Exception(message)
```

### SignatureError

Raised when signature verification fails.

```kotlin
class SignatureError(message: String) : Claim169Exception(message)
```

### DecryptionError

Raised when decryption fails.

```kotlin
class DecryptionError(message: String) : Claim169Exception(message)
```

### TimestampValidationError

Raised when token timestamp validation fails (expired or not yet valid).

```kotlin
class TimestampValidationError(message: String) : Claim169Exception(message)
```

---

## Enum Values

### Gender

| Value | Meaning |
|-------|---------|
| 1 | Male |
| 2 | Female |
| 3 | Other |

### Marital Status

| Value | Meaning |
|-------|---------|
| 1 | Unmarried |
| 2 | Married |
| 3 | Divorced |

### Photo Format

| Value | Meaning |
|-------|---------|
| 0 | Image (unspecified) |
| 1 | JPEG |
| 2 | JPEG2000 |
| 3 | AVIF |
| 4 | WebP |

### Algorithm Names

**Signing:**

- `"EdDSA"` -- Ed25519
- `"ES256"` -- ECDSA P-256

**Encryption:**

- `"A256GCM"` -- AES-256-GCM
- `"A128GCM"` -- AES-128-GCM
