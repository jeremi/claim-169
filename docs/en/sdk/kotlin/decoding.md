# Decoding Credentials

This guide covers decoding and verifying identity credentials from QR codes.

## Overview

Decoding follows these steps:

1. Receive Base45-encoded string from QR scanner
2. Choose verification method (Ed25519, ECDSA P-256, or custom)
3. Call `Claim169.decode()` with a builder DSL
4. Access the decoded claim and metadata

## Decoding with Ed25519 Verification

The most common case using Ed25519 signatures:

```kotlin
import fr.acn.claim169.Claim169

val qrData = "NCFOXN..."  // From QR scanner
val publicKey = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
    .hexToByteArray()

val result = Claim169.decode(qrData) {
    verifyWithEd25519(publicKey)
}

// Access identity data
println("ID: ${result.claim169.id}")
println("Name: ${result.claim169.fullName}")
println("DOB: ${result.claim169.dateOfBirth}")
println("Gender: ${result.claim169.gender}")

// Verification status
println("Verified: ${result.isVerified}")
println("Status: ${result.verificationStatus}")
```

## Decoding with ECDSA P-256 Verification

For credentials signed with ECDSA P-256:

```kotlin
import fr.acn.claim169.Claim169

val qrData = "NCFOXN..."
// SEC1 encoded P-256 public key (33 bytes compressed, or 65 bytes uncompressed)
val publicKey = "04...".hexToByteArray()

val result = Claim169.decode(qrData) {
    verifyWithEcdsaP256(publicKey)
}

println("ID: ${result.claim169.id}")
println("Verified: ${result.isVerified}")
```

## Decoding with PEM Public Keys

If you have public keys in PEM format (e.g., from OpenSSL or X.509 certificates), use the PEM decode methods:

### Ed25519 with PEM

```kotlin
import fr.acn.claim169.Claim169

val qrData = "NCFOXN..."
val pemKey = """
-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEA11qYAYKxCrfVS/7TyWQHOg7hcvPapjJa8CCWX4cBURo=
-----END PUBLIC KEY-----
""".trimIndent()

val result = Claim169.decode(qrData) {
    verifyWithEd25519Pem(pemKey)
}

println("Verified: ${result.isVerified}")
```

### ECDSA P-256 with PEM

```kotlin
import fr.acn.claim169.Claim169

val qrData = "NCFOXN..."
val pemKey = """
-----BEGIN PUBLIC KEY-----
MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE...
-----END PUBLIC KEY-----
""".trimIndent()

val result = Claim169.decode(qrData) {
    verifyWithEcdsaP256Pem(pemKey)
}

println("Verified: ${result.isVerified}")
```

## Decoding with Custom Verifier

For HSM, KMS, or custom crypto providers:

```kotlin
import fr.acn.claim169.Claim169
import fr.acn.claim169.SignatureVerifier

val customVerifier = object : SignatureVerifier {
    override fun verify(
        algorithm: String,
        keyId: ByteArray?,
        data: ByteArray,
        signature: ByteArray
    ) {
        // Your crypto provider logic here
        // Throw an exception if verification fails
        yourCryptoProvider.verify(data, signature)
    }
}

val result = Claim169.decode(qrData) {
    verifyWith(customVerifier)
}

println("Verified: ${result.isVerified}")
```

## Decoding Without Verification

For testing and development only. Never use in production.

```kotlin
import fr.acn.claim169.Claim169

// WARNING: INSECURE - skips signature verification
val result = Claim169.decode(qrData) {
    allowUnverified()
}

println("ID: ${result.claim169.id}")
println("Status: ${result.verificationStatus}")  // "skipped"
```

## Handling Timestamps

### Timestamp Validation

By default, the decoder validates timestamps (exp, nbf):

```kotlin
// This will throw Claim169Exception.TimestampValidationError
// if the token is expired or not yet valid
val result = Claim169.decode(qrData) {
    verifyWithEd25519(publicKey)
}
```

### Disabling Timestamp Validation

```kotlin
val result = Claim169.decode(qrData) {
    verifyWithEd25519(publicKey)
    withoutTimestampValidation()
}
```

### Clock Skew Tolerance

For distributed systems with clock differences:

```kotlin
val result = Claim169.decode(qrData) {
    verifyWithEd25519(publicKey)
    clockSkewTolerance(seconds = 60)  // Allow 60 seconds of drift
}
```

## Decompression Limits

Protect against decompression bombs by controlling the maximum decompressed size:

```kotlin
val result = Claim169.decode(qrData) {
    verifyWithEd25519(publicKey)
    maxDecompressedBytes(32_768)  // 32 KB limit (default is 65536)
}
```

## Skipping Biometrics

For faster decoding when biometrics are not needed:

```kotlin
val result = Claim169.decode(qrData) {
    verifyWithEd25519(publicKey)
    skipBiometrics()
}

// Biometric fields will be null
check(result.claim169.face == null)
```

## Closeable Decode (Memory Zeroization)

For applications handling sensitive biometric data, use `decodeCloseable()` to ensure
byte arrays are zeroized when you're done:

```kotlin
import fr.acn.claim169.Claim169

Claim169.decodeCloseable(qrData) {
    verifyWithEd25519(publicKey)
}.use { result ->
    val name = result.data.claim169.fullName
    val face = result.data.claim169.face
    // ... process credential
}
// All biometric and photo byte arrays are now zeroed
```

The `CloseableDecodeResult` implements `Closeable`, so Kotlin's `.use {}` block
automatically calls `close()` which zeroizes photo, bestQualityFingers, and all
biometric data byte arrays.

!!! tip "Java Users"
    For Java-specific examples and patterns, see the [Java Usage Guide](java-usage.md).

## Accessing Decoded Data

### DecodeResultData

The decode function returns a `DecodeResultData` object:

```kotlin
val result = Claim169.decode(qrData) {
    verifyWithEd25519(publicKey)
}

// The decoded identity claim
val claim = result.claim169

// CWT metadata (issuer, timestamps)
val meta = result.cwtMeta

// Verification status string
val status = result.verificationStatus  // "verified", "skipped", etc.

// Helper property
val isVerified = result.isVerified  // true/false

// Type-safe verification status enum
val statusEnum = result.verificationStatusEnum()  // VerificationStatus.Verified, etc.
```

### Claim169Data Fields

```kotlin
val claim = result.claim169

// Demographics
claim.id                    // String?
claim.version               // String?
claim.language              // String?
claim.fullName              // String?
claim.firstName             // String?
claim.middleName            // String?
claim.lastName              // String?
claim.dateOfBirth           // String?
claim.gender                // Long? (1=Male, 2=Female, 3=Other)
// Use Gender.fromValue(claim.gender!!) for type-safe enum
claim.address               // String?
claim.email                 // String?
claim.phone                 // String?
claim.nationality           // String?
claim.maritalStatus         // Long? (1=Unmarried, 2=Married, 3=Divorced)
// Use MaritalStatus.fromValue(claim.maritalStatus!!) for type-safe enum
claim.guardian              // String?
claim.photo                 // ByteArray?
claim.photoFormat           // Long? (1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP)
// Use PhotoFormat.fromValue(claim.photoFormat!!) for type-safe enum
claim.secondaryFullName     // String?
claim.secondaryLanguage     // String?
claim.locationCode          // String?
claim.legalStatus           // String?
claim.countryOfIssuance     // String?

// Biometrics (each is List<BiometricData>?)
claim.rightThumb
claim.rightPointerFinger
claim.rightMiddleFinger
claim.rightRingFinger
claim.rightLittleFinger
claim.leftThumb
claim.leftPointerFinger
claim.leftMiddleFinger
claim.leftRingFinger
claim.leftLittleFinger
claim.rightIris
claim.leftIris
claim.face
claim.rightPalm
claim.leftPalm
claim.voice

// Helper method
claim.hasBiometrics()  // true if any biometric data present
```

### CwtMetaData Fields

```kotlin
val meta = result.cwtMeta

meta.issuer       // String? - Credential issuer
meta.subject      // String? - Subject identifier
meta.expiresAt    // Long? - Expiration timestamp (Unix epoch)
meta.notBefore    // Long? - Not valid before timestamp
meta.issuedAt     // Long? - Issuance timestamp
```

### BiometricData Fields

```kotlin
result.claim169.face?.firstOrNull()?.let { face ->
    face.data       // ByteArray - Raw biometric data
    face.format     // Int? - Format code
    face.subFormat  // Int? - Sub-format code
    face.issuer     // String? - Biometric issuer
}
```

## Error Handling

```kotlin
import fr.acn.claim169.Claim169
import fr.acn.claim169.Claim169Exception

try {
    val result = Claim169.decode(qrData) {
        verifyWithEd25519(publicKey)
    }
} catch (e: Claim169Exception) {
    when (e) {
        is Claim169Exception.Base45DecodeError ->
            println("QR code format error: ${e.message}")
        is Claim169Exception.DecompressError ->
            println("Decompression error: ${e.message}")
        is Claim169Exception.CoseParseError ->
            println("COSE parse error: ${e.message}")
        is Claim169Exception.CwtParseError ->
            println("CWT parse error: ${e.message}")
        is Claim169Exception.Claim169NotFoundError ->
            println("Not a Claim 169 credential: ${e.message}")
        is Claim169Exception.SignatureError ->
            println("Invalid signature: ${e.message}")
        is Claim169Exception.DecryptionError ->
            println("Decryption failed: ${e.message}")
        is Claim169Exception.TimestampValidationError ->
            println("Token timing error: ${e.message}")
    }
}
```

## Complete Example

```kotlin
import fr.acn.claim169.Claim169
import fr.acn.claim169.Claim169Exception

fun verifyCredential(qrData: String, publicKey: ByteArray): Map<String, Any?>? {
    return try {
        val result = Claim169.decode(qrData) {
            verifyWithEd25519(publicKey)
            clockSkewTolerance(seconds = 60)
        }

        if (!result.isVerified) {
            println("Warning: ${result.verificationStatus}")
            return null
        }

        mapOf(
            "id" to result.claim169.id,
            "fullName" to result.claim169.fullName,
            "dateOfBirth" to result.claim169.dateOfBirth,
            "issuer" to result.cwtMeta.issuer,
            "expiresAt" to result.cwtMeta.expiresAt,
            "hasPhoto" to (result.claim169.photo != null),
            "hasBiometrics" to result.claim169.hasBiometrics(),
        )
    } catch (e: Claim169Exception.SignatureError) {
        println("Invalid signature - credential may be tampered")
        null
    } catch (e: Claim169Exception) {
        println("Failed to decode credential: ${e.message}")
        null
    }
}

// Usage
val publicKey = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
    .hexToByteArray()
val qrData = "NCFOXN..."

val credential = verifyCredential(qrData, publicKey)
if (credential != null) {
    println("Verified: ${credential["fullName"]}")
}
```

## Next Steps

- [Encryption](encryption.md) -- Decode encrypted credentials
- [Custom Crypto](custom-crypto.md) -- HSM/KMS integration
- [API Reference](api.md) -- Complete class documentation
