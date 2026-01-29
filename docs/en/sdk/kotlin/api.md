# API Reference

Complete API documentation for the claim169 Kotlin SDK, auto-generated from source code using [Dokka](https://kotl.in/dokka).

## Claim169

```kotlin
object Claim169
```

Main entry point for the MOSIP Claim 169 SDK.

Provides DSL-style decode and encode operations for Claim 169 QR codes.

### Decoding

```kotlin
val result = Claim169.decode(qrText) {
    verifyWithEd25519(publicKey)
}
println(result.claim169.fullName)
```

### Encoding

```kotlin
val qrData = Claim169.encode(
    claim169 {
        id = "ID-12345"
        fullName = "Jane Doe"
    },
    cwtMeta {
        issuer = "https://issuer.example.com"
    }
) {
    signWithEd25519(privateKey)
}
```

### Functions

| Name | Summary |
|---|---|
| decode | fun decode(qrText: `String`, configure: DecoderBuilder.() -> `Unit`): DecodeResultData<br>Decode a Claim 169 QR code string. |
| decodeCloseable | fun decodeCloseable(qrText: `String`, configure: DecoderBuilder.() -> `Unit`): CloseableDecodeResult<br>Decode a Claim 169 QR code string and return a closeable wrapper that zeroizes sensitive byte arrays when closed. |
| decodeWith | fun decodeWith(qrText: `String`, configure: DecoderConfigurer): DecodeResultData |
| encode | fun encode(claim169: Claim169Data, cwtMeta: CwtMetaData, configure: EncoderBuilder.() -> `Unit`): `String`<br>Encode Claim 169 data into a QR-ready Base45 string. |
| version | fun version(): `String`<br>Get the native library version. |

### decodeCloseableWith

```kotlin
fun decodeCloseableWith(qrText: `String`, configure: DecoderConfigurer): CloseableDecodeResult
```

### decodeCloseable

```kotlin
fun decodeCloseable(qrText: `String`, configure: DecoderBuilder.() -> `Unit`): CloseableDecodeResult
```

Decode a Claim 169 QR code string and return a closeable wrapper that zeroizes sensitive byte arrays when closed.

### decodeWith

```kotlin
fun decodeWith(qrText: `String`, configure: DecoderConfigurer): DecodeResultData
```

### decode

```kotlin
fun decode(qrText: `String`, configure: DecoderBuilder.() -> `Unit`): DecodeResultData
```

Decode a Claim 169 QR code string.

###### Return

The decoded result containing claim data, CWT metadata, and verification status

###### Parameters

| | |
|---|---|
| qrText | The Base45-encoded QR code content |
| configure | DSL block to configure verification, decryption, and options |

###### Throws

| | |
|---|---|
| Claim169Exception | on decode errors |

### encodeWith

```kotlin
fun encodeWith(claim169: Claim169Data, cwtMeta: CwtMetaData, configure: EncoderConfigurer): `String`
```

### encode

```kotlin
fun encode(claim169: Claim169Data, cwtMeta: CwtMetaData, configure: EncoderBuilder.() -> `Unit`): `String`
```

Encode Claim 169 data into a QR-ready Base45 string.

###### Return

The Base45-encoded QR string

###### Parameters

| | |
|---|---|
| claim169 | The identity claim data |
| cwtMeta | The CWT metadata (issuer, expiration, etc.) |
| configure | DSL block to configure signing, encryption, and options |

###### Throws

| | |
|---|---|
| Claim169Exception | on encode errors |

### version

```kotlin
fun version(): `String`
```

Get the native library version.

---

## DecoderBuilder

```kotlin
class DecoderBuilder(qrText: `String`)
```

DSL builder for decoding Claim 169 QR codes.

Wraps the UniFFI-generated Claim169Decoder with an idiomatic Kotlin API.

### Usage

```kotlin
val result = Claim169.decode(qrText) {
    verifyWithEd25519(publicKey)
    skipBiometrics()
}
```

### Functions

| Name | Summary |
|---|---|
| allowUnverified | fun allowUnverified()<br>Allow decoding without signature verification. |
| clockSkewTolerance | fun clockSkewTolerance(seconds: `Long`)<br>Set clock skew tolerance for timestamp validation (in seconds). |
| decryptWith | fun decryptWith(decryptor: Decryptor)<br>Decrypt with a custom Decryptor implementation (for HSM/KMS). |
| decryptWithAes128 | fun decryptWithAes128(key: `ByteArray`)<br>Decrypt with AES-128-GCM (16-byte key). |
| decryptWithAes256 | fun decryptWithAes256(key: `ByteArray`)<br>Decrypt with AES-256-GCM (32-byte key). |
| maxDecompressedBytes | fun maxDecompressedBytes(maxBytes: `Long`)<br>Set maximum decompressed size in bytes (default: 65536). |
| skipBiometrics | fun skipBiometrics()<br>Skip biometric data parsing for faster decoding. |
| verifyWith | fun verifyWith(verifier: SignatureVerifier)<br>Verify with a custom SignatureVerifier implementation (for HSM/KMS). |
| verifyWithEcdsaP256 | fun verifyWithEcdsaP256(publicKey: `ByteArray`)<br>Verify with an ECDSA P-256 public key (SEC1-encoded, 33 or 65 bytes). |
| verifyWithEcdsaP256Pem | fun verifyWithEcdsaP256Pem(pem: `String`)<br>Verify with an ECDSA P-256 public key in PEM format. |
| verifyWithEd25519 | fun verifyWithEd25519(publicKey: `ByteArray`)<br>Verify with an Ed25519 public key (32 raw bytes). |
| verifyWithEd25519Pem | fun verifyWithEd25519Pem(pem: `String`)<br>Verify with an Ed25519 public key in PEM format. |
| withoutTimestampValidation | fun withoutTimestampValidation()<br>Disable timestamp validation (expiration and not-before checks). |

### DecoderBuilder

```kotlin
constructor(qrText: `String`)
```

### allowUnverified

```kotlin
fun allowUnverified()
```

Allow decoding without signature verification.

**Security warning**: Credentials decoded with verification skipped (status `Skipped`) cannot be trusted.

### clockSkewTolerance

```kotlin
fun clockSkewTolerance(seconds: `Long`)
```

Set clock skew tolerance for timestamp validation (in seconds).

### decryptWithAes128

```kotlin
fun decryptWithAes128(key: `ByteArray`)
```

Decrypt with AES-128-GCM (16-byte key).

### decryptWithAes256

```kotlin
fun decryptWithAes256(key: `ByteArray`)
```

Decrypt with AES-256-GCM (32-byte key).

### decryptWith

```kotlin
fun decryptWith(decryptor: Decryptor)
```

Decrypt with a custom Decryptor implementation (for HSM/KMS).

### maxDecompressedBytes

```kotlin
fun maxDecompressedBytes(maxBytes: `Long`)
```

Set maximum decompressed size in bytes (default: 65536).

### skipBiometrics

```kotlin
fun skipBiometrics()
```

Skip biometric data parsing for faster decoding.

### verifyWithEcdsaP256Pem

```kotlin
fun verifyWithEcdsaP256Pem(pem: `String`)
```

Verify with an ECDSA P-256 public key in PEM format.

### verifyWithEcdsaP256

```kotlin
fun verifyWithEcdsaP256(publicKey: `ByteArray`)
```

Verify with an ECDSA P-256 public key (SEC1-encoded, 33 or 65 bytes).

### verifyWithEd25519Pem

```kotlin
fun verifyWithEd25519Pem(pem: `String`)
```

Verify with an Ed25519 public key in PEM format.

### verifyWithEd25519

```kotlin
fun verifyWithEd25519(publicKey: `ByteArray`)
```

Verify with an Ed25519 public key (32 raw bytes).

### verifyWith

```kotlin
fun verifyWith(verifier: SignatureVerifier)
```

Verify with a custom SignatureVerifier implementation (for HSM/KMS).

### withoutTimestampValidation

```kotlin
fun withoutTimestampValidation()
```

Disable timestamp validation (expiration and not-before checks).

---

## EncoderBuilder

```kotlin
class EncoderBuilder(claim169: Claim169Data, cwtMeta: CwtMetaData)
```

DSL builder for encoding Claim 169 credentials into QR-ready strings.

Wraps the UniFFI-generated Claim169Encoder with an idiomatic Kotlin API.

### Usage

```kotlin
val qrData = Claim169.encode(claim, meta) {
    signWithEd25519(privateKey)
}
```

### Functions

| Name | Summary |
|---|---|
| allowUnsigned | fun allowUnsigned()<br>Allow encoding without a signature. |
| encryptWith | fun encryptWith(encryptor: Encryptor, algorithm: `String`)<br>Encrypt with a custom Encryptor implementation (for HSM/KMS).<br>fun encryptWith(encryptor: Encryptor, algorithm: CoseAlgorithm)<br>Encrypt with a custom Encryptor implementation using a known COSE algorithm. |
| encryptWithAes128 | fun encryptWithAes128(key: `ByteArray`)<br>Encrypt with AES-128-GCM (16-byte key). Nonce is generated randomly. |
| encryptWithAes256 | fun encryptWithAes256(key: `ByteArray`)<br>Encrypt with AES-256-GCM (32-byte key). Nonce is generated randomly. |
| signWith | fun signWith(signer: Signer, algorithm: `String`)<br>Sign with a custom Signer implementation (for HSM/KMS).<br>fun signWith(signer: Signer, algorithm: CoseAlgorithm)<br>Sign with a custom Signer implementation using a known COSE algorithm. |
| signWithEcdsaP256 | fun signWithEcdsaP256(privateKey: `ByteArray`)<br>Sign with an ECDSA P-256 private key (32-byte scalar). |
| signWithEd25519 | fun signWithEd25519(privateKey: `ByteArray`)<br>Sign with an Ed25519 private key (32 raw bytes). |
| skipBiometrics | fun skipBiometrics()<br>Skip biometric data during encoding. |

### EncoderBuilder

```kotlin
constructor(claim169: Claim169Data, cwtMeta: CwtMetaData)
```

### allowUnsigned

```kotlin
fun allowUnsigned()
```

Allow encoding without a signature.

**Security warning**: Unsigned credentials cannot be verified.

### encryptWithAes128

```kotlin
fun encryptWithAes128(key: `ByteArray`)
```

Encrypt with AES-128-GCM (16-byte key). Nonce is generated randomly.

### encryptWithAes256

```kotlin
fun encryptWithAes256(key: `ByteArray`)
```

Encrypt with AES-256-GCM (32-byte key). Nonce is generated randomly.

### encryptWith

```kotlin
fun encryptWith(encryptor: Encryptor, algorithm: `String`)
```

Encrypt with a custom Encryptor implementation (for HSM/KMS).

###### Parameters

| | |
|---|---|
| encryptor | The encryptor implementation |
| algorithm | COSE algorithm name (e.g., "A256GCM") |

```kotlin
fun encryptWith(encryptor: Encryptor, algorithm: CoseAlgorithm)
```

Encrypt with a custom Encryptor implementation using a known COSE algorithm.

### signWithEcdsaP256

```kotlin
fun signWithEcdsaP256(privateKey: `ByteArray`)
```

Sign with an ECDSA P-256 private key (32-byte scalar).

**Security note**: The privateKey bytes are passed into native code but the JVM copy remains in the caller's heap. Callers should zeroize the array after encoding completes:

```kotlin
privateKey.fill(0)
```

### signWithEd25519

```kotlin
fun signWithEd25519(privateKey: `ByteArray`)
```

Sign with an Ed25519 private key (32 raw bytes).

**Security note**: The privateKey bytes are passed into native code but the JVM copy remains in the caller's heap. Callers should zeroize the array after encoding completes:

```kotlin
privateKey.fill(0)
```

### signWith

```kotlin
fun signWith(signer: Signer, algorithm: `String`)
```

Sign with a custom Signer implementation (for HSM/KMS).

**Security note**: If the Signer holds in-memory key material, implementors should zeroize it when it is no longer needed to minimize exposure on the JVM heap.

###### Parameters

| | |
|---|---|
| signer | The signer implementation |
| algorithm | COSE algorithm name (e.g., "EdDSA", "ES256") |

```kotlin
fun signWith(signer: Signer, algorithm: CoseAlgorithm)
```

Sign with a custom Signer implementation using a known COSE algorithm.

### skipBiometrics

```kotlin
fun skipBiometrics()
```

Skip biometric data during encoding.

---

## Claim169DataBuilder

```kotlin
class Claim169DataBuilder
```

DSL builder for creating Claim169Data instances.

### Usage

```kotlin
val data = claim169 {
    id = "ID-12345"
    fullName = "Jane Doe"
    dateOfBirth = "19900115"
    genderEnum = Gender.Female
    email = "jane@example.com"
}
```

### Properties

| Name | Summary |
|---|---|
| address | var address: `String`? |
| bestQualityFingers | var bestQualityFingers: `ByteArray`? |
| countryOfIssuance | var countryOfIssuance: `String`? |
| dateOfBirth | var dateOfBirth: `String`? |
| email | var email: `String`? |
| face | var face: `List`<BiometricData>? |
| firstName | var firstName: `String`? |
| fullName | var fullName: `String`? |
| gender | var gender: `Long`? |
| genderEnum | var genderEnum: Gender? |
| guardian | var guardian: `String`? |
| id | var id: `String`? |
| language | var language: `String`? |
| lastName | var lastName: `String`? |
| leftIris | var leftIris: `List`<BiometricData>? |
| leftLittleFinger | var leftLittleFinger: `List`<BiometricData>? |
| leftMiddleFinger | var leftMiddleFinger: `List`<BiometricData>? |
| leftPalm | var leftPalm: `List`<BiometricData>? |
| leftPointerFinger | var leftPointerFinger: `List`<BiometricData>? |
| leftRingFinger | var leftRingFinger: `List`<BiometricData>? |
| leftThumb | var leftThumb: `List`<BiometricData>? |
| legalStatus | var legalStatus: `String`? |
| locationCode | var locationCode: `String`? |
| maritalStatus | var maritalStatus: `Long`? |
| maritalStatusEnum | var maritalStatusEnum: MaritalStatus? |
| middleName | var middleName: `String`? |
| nationality | var nationality: `String`? |
| phone | var phone: `String`? |
| photo | var photo: `ByteArray`? |
| photoFormat | var photoFormat: `Long`? |
| photoFormatEnum | var photoFormatEnum: PhotoFormat? |
| rightIris | var rightIris: `List`<BiometricData>? |
| rightLittleFinger | var rightLittleFinger: `List`<BiometricData>? |
| rightMiddleFinger | var rightMiddleFinger: `List`<BiometricData>? |
| rightPalm | var rightPalm: `List`<BiometricData>? |
| rightPointerFinger | var rightPointerFinger: `List`<BiometricData>? |
| rightRingFinger | var rightRingFinger: `List`<BiometricData>? |
| rightThumb | var rightThumb: `List`<BiometricData>? |
| secondaryFullName | var secondaryFullName: `String`? |
| secondaryLanguage | var secondaryLanguage: `String`? |
| unknownFieldsJson | var unknownFieldsJson: `String`?<br>JSON-encoded map of unknown CBOR fields for forward compatibility. Must be valid JSON (e.g., `{"100":"value"}`). Malformed JSON will cause uniffi.claim169_jni.Claim169Exception.Claim169Invalid when encoding. |
| version | var version: `String`? |
| voice | var voice: `List`<BiometricData>? |

### Claim169DataBuilder

```kotlin
constructor()
```

### address

```kotlin
var address: `String`?
```

### bestQualityFingers

```kotlin
var bestQualityFingers: `ByteArray`?
```

### countryOfIssuance

```kotlin
var countryOfIssuance: `String`?
```

### dateOfBirth

```kotlin
var dateOfBirth: `String`?
```

### email

```kotlin
var email: `String`?
```

### face

```kotlin
var face: `List`<BiometricData>?
```

### firstName

```kotlin
var firstName: `String`?
```

### fullName

```kotlin
var fullName: `String`?
```

### genderEnum

```kotlin
var genderEnum: Gender?
```

### gender

```kotlin
var gender: `Long`?
```

### guardian

```kotlin
var guardian: `String`?
```

### id

```kotlin
var id: `String`?
```

### language

```kotlin
var language: `String`?
```

### lastName

```kotlin
var lastName: `String`?
```

### leftIris

```kotlin
var leftIris: `List`<BiometricData>?
```

### leftLittleFinger

```kotlin
var leftLittleFinger: `List`<BiometricData>?
```

### leftMiddleFinger

```kotlin
var leftMiddleFinger: `List`<BiometricData>?
```

### leftPalm

```kotlin
var leftPalm: `List`<BiometricData>?
```

### leftPointerFinger

```kotlin
var leftPointerFinger: `List`<BiometricData>?
```

### leftRingFinger

```kotlin
var leftRingFinger: `List`<BiometricData>?
```

### leftThumb

```kotlin
var leftThumb: `List`<BiometricData>?
```

### legalStatus

```kotlin
var legalStatus: `String`?
```

### locationCode

```kotlin
var locationCode: `String`?
```

### maritalStatusEnum

```kotlin
var maritalStatusEnum: MaritalStatus?
```

### maritalStatus

```kotlin
var maritalStatus: `Long`?
```

### middleName

```kotlin
var middleName: `String`?
```

### nationality

```kotlin
var nationality: `String`?
```

### phone

```kotlin
var phone: `String`?
```

### photoFormatEnum

```kotlin
var photoFormatEnum: PhotoFormat?
```

### photoFormat

```kotlin
var photoFormat: `Long`?
```

### photo

```kotlin
var photo: `ByteArray`?
```

### rightIris

```kotlin
var rightIris: `List`<BiometricData>?
```

### rightLittleFinger

```kotlin
var rightLittleFinger: `List`<BiometricData>?
```

### rightMiddleFinger

```kotlin
var rightMiddleFinger: `List`<BiometricData>?
```

### rightPalm

```kotlin
var rightPalm: `List`<BiometricData>?
```

### rightPointerFinger

```kotlin
var rightPointerFinger: `List`<BiometricData>?
```

### rightRingFinger

```kotlin
var rightRingFinger: `List`<BiometricData>?
```

### rightThumb

```kotlin
var rightThumb: `List`<BiometricData>?
```

### secondaryFullName

```kotlin
var secondaryFullName: `String`?
```

### secondaryLanguage

```kotlin
var secondaryLanguage: `String`?
```

### unknownFieldsJson

```kotlin
var unknownFieldsJson: `String`?
```

JSON-encoded map of unknown CBOR fields for forward compatibility. Must be valid JSON (e.g., `{"100":"value"}`). Malformed JSON will cause uniffi.claim169_jni.Claim169Exception.Claim169Invalid when encoding.

### version

```kotlin
var version: `String`?
```

### voice

```kotlin
var voice: `List`<BiometricData>?
```

---

## CwtMetaDataBuilder

```kotlin
class CwtMetaDataBuilder
```

DSL builder for creating CwtMetaData instances.

### Usage

```kotlin
val meta = cwtMeta {
    issuer = "https://issuer.example.com"
    expiresAt = 1800000000L
}
```

### Properties

| Name | Summary |
|---|---|
| expiresAt | var expiresAt: `Long`? |
| issuedAt | var issuedAt: `Long`? |
| issuer | var issuer: `String`? |
| notBefore | var notBefore: `Long`? |
| subject | var subject: `String`? |

### CwtMetaDataBuilder

```kotlin
constructor()
```

### expiresAt

```kotlin
var expiresAt: `Long`?
```

### issuedAt

```kotlin
var issuedAt: `Long`?
```

### issuer

```kotlin
var issuer: `String`?
```

### notBefore

```kotlin
var notBefore: `Long`?
```

### subject

```kotlin
var subject: `String`?
```

---

## CloseableDecodeResult

```kotlin
class CloseableDecodeResult(val data: DecodeResultData) : [Closeable](https://docs.oracle.com/javase/8/docs/api/java/io/Closeable.html)
```

A [Closeable](https://docs.oracle.com/javase/8/docs/api/java/io/Closeable.html) wrapper around DecodeResultData that zeroizes sensitive byte arrays (biometric templates, photos, and other binary fields) when close is called.

The Rust core library uses the `zeroize` crate to scrub secrets from memory. On the JVM side, decoded credential data containing biometric templates and photos persists in the heap until garbage collected. This wrapper provides deterministic zeroization so callers can limit the window of exposure.

### Usage

```kotlin
CloseableDecodeResult(
    Claim169.decode(qrText) { verifyWithEd25519(publicKey) }
).use { result ->
    val name = result.data.claim169.fullName
    // ... process credential
}
// All biometric and photo byte arrays are now zeroed.
```

### Properties

| Name | Summary |
|---|---|
| data | val data: DecodeResultData<br>The underlying decode result. |

### Functions

| Name | Summary |
|---|---|
| close | open override fun close()<br>Zeroizes all sensitive byte arrays within the decoded credential. |

### CloseableDecodeResult

```kotlin
constructor(data: DecodeResultData)
```

### close

```kotlin
open override fun close()
```

Zeroizes all sensitive byte arrays within the decoded credential.

This fills photo, bestQualityFingers, and all biometric data byte arrays with zeros. After calling this method the byte arrays still exist but contain only zero bytes. Callers should not read the data after closing.

### data

```kotlin
val data: DecodeResultData
```

The underlying decode result.

---

## DecoderConfigurer

```kotlin
fun interface DecoderConfigurer
```

### configure

```kotlin
abstract fun configure(builder: DecoderBuilder)
```

---

## EncoderConfigurer

```kotlin
fun interface EncoderConfigurer
```

### configure

```kotlin
abstract fun configure(builder: EncoderBuilder)
```

---

## SignatureVerifier

```kotlin
interface SignatureVerifier
```

Interface for custom signature verification.

Implement this for HSM, KMS, or other custom crypto backends. Uses `ByteArray` instead of the UniFFI-generated `List<UByte>` for idiomatic Kotlin.

### Usage

```kotlin
val result = Claim169.decode(qrText) {
    verifyWith(object : SignatureVerifier {
        override fun verify(algorithm: String, keyId: ByteArray?, data: ByteArray, signature: ByteArray): VerificationResult {
            return if (hsmProvider.verify(keyId, data, signature))
                VerificationResult.Valid
            else
                VerificationResult.Invalid("HSM rejected signature")
        }
    })
}
```

### Functions

| Name | Summary |
|---|---|
| verify | abstract fun verify(algorithm: `String`, keyId: `ByteArray`?, data: `ByteArray`, signature: `ByteArray`): VerificationResult<br>Verify a digital signature. |

### verify

```kotlin
abstract fun verify(algorithm: `String`, keyId: `ByteArray`?, data: `ByteArray`, signature: `ByteArray`): VerificationResult
```

Verify a digital signature.

Implementations MUST return VerificationResult.Valid only after performing actual cryptographic verification. Returning VerificationResult.Valid without checking the signature defeats the security purpose of this library.

###### Return

VerificationResult.Valid if the signature is valid,     VerificationResult.Invalid if verification fails

###### Parameters

| | |
|---|---|
| algorithm | COSE algorithm name (e.g., "EdDSA", "ES256") |
| keyId | Optional key identifier bytes |
| data | The data that was signed (COSE Sig_structure) |
| signature | The signature bytes to verify |

---

## Signer

```kotlin
interface Signer
```

Interface for custom signing.

Implement this for HSM, KMS, or other custom crypto backends.

### Functions

| Name | Summary |
|---|---|
| keyId | open fun keyId(): `ByteArray`?<br>Get the key ID for this signer. Returns null if no key ID. |
| sign | abstract fun sign(algorithm: `String`, keyId: `ByteArray`?, data: `ByteArray`): `ByteArray`<br>Sign data and return the signature. |

### keyId

```kotlin
open fun keyId(): `ByteArray`?
```

Get the key ID for this signer. Returns null if no key ID.

### sign

```kotlin
abstract fun sign(algorithm: `String`, keyId: `ByteArray`?, data: `ByteArray`): `ByteArray`
```

Sign data and return the signature.

###### Return

The signature bytes

###### Parameters

| | |
|---|---|
| algorithm | COSE algorithm name (e.g., "EdDSA", "ES256") |
| keyId | Optional key identifier bytes |
| data | The data to sign (COSE Sig_structure) |

###### Throws

| | |
|---|---|
| `Exception` | if signing fails |

---

## Decryptor

```kotlin
interface Decryptor
```

Interface for custom decryption.

Implement this for HSM, KMS, or other custom crypto backends.

### Functions

| Name | Summary |
|---|---|
| decrypt | abstract fun decrypt(algorithm: `String`, keyId: `ByteArray`?, nonce: `ByteArray`, aad: `ByteArray`, ciphertext: `ByteArray`): `ByteArray`<br>Decrypt ciphertext using AEAD. |

### decrypt

```kotlin
abstract fun decrypt(algorithm: `String`, keyId: `ByteArray`?, nonce: `ByteArray`, aad: `ByteArray`, ciphertext: `ByteArray`): `ByteArray`
```

Decrypt ciphertext using AEAD.

###### Return

The decrypted plaintext bytes

###### Parameters

| | |
|---|---|
| algorithm | COSE algorithm name (e.g., "A256GCM") |
| keyId | Optional key identifier bytes |
| nonce | The IV/nonce |
| aad | Additional authenticated data |
| ciphertext | The ciphertext to decrypt (includes auth tag for AEAD) |

###### Throws

| | |
|---|---|
| `Exception` | if decryption fails |

---

## Encryptor

```kotlin
interface Encryptor
```

Interface for custom encryption.

Implement this for HSM, KMS, or other custom crypto backends.

### Functions

| Name | Summary |
|---|---|
| encrypt | abstract fun encrypt(algorithm: `String`, keyId: `ByteArray`?, nonce: `ByteArray`, aad: `ByteArray`, plaintext: `ByteArray`): `ByteArray`<br>Encrypt plaintext using AEAD. |

### encrypt

```kotlin
abstract fun encrypt(algorithm: `String`, keyId: `ByteArray`?, nonce: `ByteArray`, aad: `ByteArray`, plaintext: `ByteArray`): `ByteArray`
```

Encrypt plaintext using AEAD.

###### Return

The ciphertext bytes (includes auth tag for AEAD)

###### Parameters

| | |
|---|---|
| algorithm | COSE algorithm name (e.g., "A256GCM") |
| keyId | Optional key identifier bytes |
| nonce | The IV/nonce |
| aad | Additional authenticated data |
| plaintext | The plaintext to encrypt |

###### Throws

| | |
|---|---|
| `Exception` | if encryption fails |

---

## Gender

```kotlin
enum Gender : `Enum`<Gender>
```

### Properties

| Name | Summary |
|---|---|
| entries | val entries: `EnumEntries`<Gender><br>Returns a representation of an immutable list of all enum entries, in the order they're declared. |

### Functions

| Name | Summary |
|---|---|
| values | fun values(): `Array`<Gender><br>Returns an array containing the constants of this enum type, in the order they're declared. |

### entries

```kotlin
val entries: `EnumEntries`<Gender>
```

Returns a representation of an immutable list of all enum entries, in the order they're declared.

This method may be used to iterate over the enum entries.

### valueOf

```kotlin
fun valueOf(value: `String`): Gender
```

Returns the enum constant of this type with the specified name. The string must match exactly an identifier used to declare an enum constant in this type. (Extraneous whitespace characters are not permitted.)

###### Throws

| | |
|---|---|
| `IllegalArgumentException` | if this enum type has no constant with the specified name |

### value

```kotlin
val value: `Long`
```

### values

```kotlin
fun values(): `Array`<Gender>
```

Returns an array containing the constants of this enum type, in the order they're declared.

This method may be used to iterate over the constants.

### fromValue

```kotlin
fun fromValue(value: `Long`): Gender?
```

---

## MaritalStatus

```kotlin
enum MaritalStatus : `Enum`<MaritalStatus>
```

### Properties

| Name | Summary |
|---|---|
| entries | val entries: `EnumEntries`<MaritalStatus><br>Returns a representation of an immutable list of all enum entries, in the order they're declared. |

### Functions

| Name | Summary |
|---|---|
| values | fun values(): `Array`<MaritalStatus><br>Returns an array containing the constants of this enum type, in the order they're declared. |

### entries

```kotlin
val entries: `EnumEntries`<MaritalStatus>
```

Returns a representation of an immutable list of all enum entries, in the order they're declared.

This method may be used to iterate over the enum entries.

### valueOf

```kotlin
fun valueOf(value: `String`): MaritalStatus
```

Returns the enum constant of this type with the specified name. The string must match exactly an identifier used to declare an enum constant in this type. (Extraneous whitespace characters are not permitted.)

###### Throws

| | |
|---|---|
| `IllegalArgumentException` | if this enum type has no constant with the specified name |

### value

```kotlin
val value: `Long`
```

### values

```kotlin
fun values(): `Array`<MaritalStatus>
```

Returns an array containing the constants of this enum type, in the order they're declared.

This method may be used to iterate over the constants.

### fromValue

```kotlin
fun fromValue(value: `Long`): MaritalStatus?
```

---

## PhotoFormat

```kotlin
enum PhotoFormat : `Enum`<PhotoFormat>
```

### Properties

| Name | Summary |
|---|---|
| entries | val entries: `EnumEntries`<PhotoFormat><br>Returns a representation of an immutable list of all enum entries, in the order they're declared. |

### Functions

| Name | Summary |
|---|---|
| values | fun values(): `Array`<PhotoFormat><br>Returns an array containing the constants of this enum type, in the order they're declared. |

### entries

```kotlin
val entries: `EnumEntries`<PhotoFormat>
```

Returns a representation of an immutable list of all enum entries, in the order they're declared.

This method may be used to iterate over the enum entries.

### valueOf

```kotlin
fun valueOf(value: `String`): PhotoFormat
```

Returns the enum constant of this type with the specified name. The string must match exactly an identifier used to declare an enum constant in this type. (Extraneous whitespace characters are not permitted.)

###### Throws

| | |
|---|---|
| `IllegalArgumentException` | if this enum type has no constant with the specified name |

### value

```kotlin
val value: `Long`
```

### values

```kotlin
fun values(): `Array`<PhotoFormat>
```

Returns an array containing the constants of this enum type, in the order they're declared.

This method may be used to iterate over the constants.

### fromValue

```kotlin
fun fromValue(value: `Long`): PhotoFormat?
```

---

## CoseAlgorithm

```kotlin
enum CoseAlgorithm : `Enum`<CoseAlgorithm>
```

### Properties

| Name | Summary |
|---|---|
| entries | val entries: `EnumEntries`<CoseAlgorithm><br>Returns a representation of an immutable list of all enum entries, in the order they're declared. |

### Functions

| Name | Summary |
|---|---|
| values | fun values(): `Array`<CoseAlgorithm><br>Returns an array containing the constants of this enum type, in the order they're declared. |

### coseName

```kotlin
val coseName: `String`
```

### entries

```kotlin
val entries: `EnumEntries`<CoseAlgorithm>
```

Returns a representation of an immutable list of all enum entries, in the order they're declared.

This method may be used to iterate over the enum entries.

### valueOf

```kotlin
fun valueOf(value: `String`): CoseAlgorithm
```

Returns the enum constant of this type with the specified name. The string must match exactly an identifier used to declare an enum constant in this type. (Extraneous whitespace characters are not permitted.)

###### Throws

| | |
|---|---|
| `IllegalArgumentException` | if this enum type has no constant with the specified name |

### values

```kotlin
fun values(): `Array`<CoseAlgorithm>
```

Returns an array containing the constants of this enum type, in the order they're declared.

This method may be used to iterate over the constants.

---

## VerificationStatus

```kotlin
enum VerificationStatus : `Enum`<VerificationStatus>
```

### Properties

| Name | Summary |
|---|---|
| entries | val entries: `EnumEntries`<VerificationStatus><br>Returns a representation of an immutable list of all enum entries, in the order they're declared. |

### Functions

| Name | Summary |
|---|---|
| values | fun values(): `Array`<VerificationStatus><br>Returns an array containing the constants of this enum type, in the order they're declared. |

### entries

```kotlin
val entries: `EnumEntries`<VerificationStatus>
```

Returns a representation of an immutable list of all enum entries, in the order they're declared.

This method may be used to iterate over the enum entries.

### valueOf

```kotlin
fun valueOf(value: `String`): VerificationStatus
```

Returns the enum constant of this type with the specified name. The string must match exactly an identifier used to declare an enum constant in this type. (Extraneous whitespace characters are not permitted.)

###### Throws

| | |
|---|---|
| `IllegalArgumentException` | if this enum type has no constant with the specified name |

### value

```kotlin
val value: `String`
```

### values

```kotlin
fun values(): `Array`<VerificationStatus>
```

Returns an array containing the constants of this enum type, in the order they're declared.

This method may be used to iterate over the constants.

### fromValue

```kotlin
fun fromValue(value: `String`): VerificationStatus
```

---

## VerificationResult

```kotlin
sealed interface VerificationResult
```

Result of a signature verification operation.

Forces implementors to make an explicit accept/reject decision, preventing accidental acceptance from empty method bodies.

### Types

| Name | Summary |
|---|---|
| Invalid | data class Invalid(val reason: `String`) : VerificationResult<br>The signature is invalid or verification failed. |
| Valid | data object Valid : VerificationResult<br>The signature is valid. |

### Invalid

```kotlin
constructor(reason: `String`)
```

### reason

```kotlin
val reason: `String`
```

---

## Top-Level Functions

### claim169

```kotlin
fun claim169(configure: Claim169DataBuilder.() -> `Unit`): Claim169Data
```

Create a Claim169Data using DSL syntax.

### cwtMeta

```kotlin
fun cwtMeta(configure: CwtMetaDataBuilder.() -> `Unit`): CwtMetaData
```

Create a CwtMetaData using DSL syntax.

### verificationStatusEnum

```kotlin
fun DecodeResultData.verificationStatusEnum(): VerificationStatus
```

### zeroizeClaim169Data

```kotlin
fun zeroizeClaim169Data(claim: Claim169Data)
```

Zeroizes all sensitive byte arrays within a Claim169Data instance.

Fills photo, bestQualityFingers, and every biometric data byte array with zeros.
