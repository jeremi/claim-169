# Quick Start

This guide covers the essential operations: encoding credentials and decoding QR codes.

## Decoding a QR Code

The most common operation is decoding a QR code that was scanned from an identity credential.

!!! warning "Do not trim Base45"
    The Base45 alphabet includes a literal space character (`" "`). Preserve the scanned QR text exactly as-is (no `.trim()`, whitespace normalization, etc.), or you can corrupt valid credentials.

### With Ed25519 Verification

```kotlin
import fr.acn.claim169.Claim169
import fr.acn.claim169.VerificationStatus

// QR code content (Base45 encoded string from scanner)
val qrData = "NCFOXN..."

// Issuer's Ed25519 public key (32 bytes)
val publicKey = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
    .hexToByteArray()

// Decode and verify
val result = Claim169.decode(qrData) {
    verifyWithEd25519(publicKey)
}

// Access identity data
println("ID: ${result.claim169.id}")
println("Name: ${result.claim169.fullName}")
println("Date of Birth: ${result.claim169.dateOfBirth}")

// Check verification status
when (result.verificationStatus) {
    VerificationStatus.Verified -> println("Signature verified successfully")
    else -> println("Verification status: ${result.verificationStatus}")
}

// Access CWT metadata
println("Issuer: ${result.cwtMeta.issuer}")
println("Expires: ${result.cwtMeta.expiresAt}")
```

### Decode Without Verification

For testing and development only. Never use in production.

```kotlin
import fr.acn.claim169.Claim169

// WARNING: INSECURE - skips signature verification
val result = Claim169.decode(qrData) {
    allowUnverified()
}

println("ID: ${result.claim169.id}")
println("Status: ${result.verificationStatus}")  // VerificationStatus.Skipped
```

## Encoding a Credential

Create a signed credential that can be encoded in a QR code.

### Basic Encoding with Ed25519

```kotlin
import fr.acn.claim169.Claim169
import fr.acn.claim169.claim169Data
import fr.acn.claim169.cwtMetaData
import fr.acn.claim169.Gender

// Create identity data using DSL builder
val data = claim169Data {
    id = "MOSIP-2024-001"
    fullName = "Jane Doe"
    dateOfBirth = "1990-05-15"
    genderEnum = Gender.Female
    email = "jane.doe@example.org"
    nationality = "US"
}

// Create CWT metadata using DSL builder
val meta = cwtMetaData {
    issuer = "https://id.example.org"
    expiresAt = 1900000000L
    issuedAt = 1700000000L
}

// Ed25519 private key (32 bytes) - keep this secret!
val privateKey = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
    .hexToByteArray()

// Encode the credential
val qrData = Claim169.encode(data, meta) {
    signWithEd25519(privateKey)
}

println("QR Code content (${qrData.length} chars):")
println(qrData)
```

### Roundtrip Example

Encode a credential and immediately decode it to verify:

```kotlin
import fr.acn.claim169.Claim169
import fr.acn.claim169.claim169Data
import fr.acn.claim169.cwtMetaData

// Keys
val privateKey = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
    .hexToByteArray()
val publicKey = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
    .hexToByteArray()

// Create and encode
val data = claim169Data {
    id = "TEST-001"
    fullName = "Test User"
}

val meta = cwtMetaData {
    issuer = "https://test.example.org"
    expiresAt = 1900000000L
}

val qrData = Claim169.encode(data, meta) {
    signWithEd25519(privateKey)
}

// Decode and verify
val result = Claim169.decode(qrData) {
    verifyWithEd25519(publicKey)
}

check(result.claim169.id == "TEST-001")
check(result.claim169.fullName == "Test User")
check(result.verificationStatus == VerificationStatus.Verified)
println("Roundtrip successful!")
```

## Error Handling

The SDK uses a sealed class hierarchy for exceptions, enabling exhaustive `when` expressions:

```kotlin
import fr.acn.claim169.Claim169
import fr.acn.claim169.Claim169Exception

try {
    val result = Claim169.decode(qrData) {
        verifyWithEd25519(publicKey)
    }
} catch (e: Claim169Exception) {
    when (e) {
        is Claim169Exception.Base45Decode ->
            println("Invalid QR code format: ${e.message}")
        is Claim169Exception.Decompress ->
            println("Decompression failed: ${e.message}")
        is Claim169Exception.CoseParse ->
            println("Invalid COSE structure: ${e.message}")
        is Claim169Exception.CwtParse ->
            println("Invalid CWT structure: ${e.message}")
        is Claim169Exception.Claim169NotFound ->
            println("Not a Claim 169 credential: ${e.message}")
        is Claim169Exception.SignatureInvalid ->
            println("Signature verification failed: ${e.message}")
        is Claim169Exception.DecryptionFailed ->
            println("Decryption failed: ${e.message}")
        is Claim169Exception.Expired ->
            println("Token expired: ${e.message}")
        is Claim169Exception.NotYetValid ->
            println("Token not yet valid: ${e.message}")
    }
}
```

## Working with Biometrics

### Checking for Biometric Data

```kotlin
val result = Claim169.decode(qrData) {
    verifyWithEd25519(publicKey)
}

if (result.claim169.hasBiometrics()) {
    println("Credential contains biometric data")

    // Check specific biometric types
    result.claim169.face?.firstOrNull()?.let { face ->
        println("Face photo: ${face.data.size} bytes, format=${face.format}")
    }

    result.claim169.rightThumb?.firstOrNull()?.let { thumb ->
        println("Right thumb: ${thumb.data.size} bytes")
    }
}
```

### Skipping Biometrics (Faster Decoding)

For use cases that don't need biometric data:

```kotlin
val result = Claim169.decode(qrData) {
    verifyWithEd25519(publicKey)
    skipBiometrics()
}

// Biometric fields will be null
check(result.claim169.face == null)
```

## Next Steps

- [Encoding Guide](encoding.md) -- Detailed encoding with all demographics
- [Decoding Guide](decoding.md) -- Advanced decoding options
- [Encryption](encryption.md) -- Add AES-GCM encryption
- [Custom Crypto](custom-crypto.md) -- HSM/KMS integration
