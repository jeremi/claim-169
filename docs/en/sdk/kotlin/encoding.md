# Encoding Credentials

This guide covers creating signed identity credentials that can be encoded in QR codes.

## Overview

Encoding follows these steps:

1. Create a `Claim169Data` with the `claim169 {}` DSL builder
2. Create a `CwtMetaData` with the `cwtMeta {}` DSL builder
3. Sign with a private key
4. Optionally encrypt with a symmetric key
5. Receive a Base45-encoded string for QR code generation

## Creating Identity Data

### claim169 {} Builder

The `claim169 {}` builder provides a type-safe DSL for creating identity data:

```kotlin
import org.acn.claim169.claim169

// Create with all demographics
val data = claim169 {
    id = "MOSIP-2024-001"
    version = "1.0.0"
    language = "en"
    fullName = "Jane Marie Doe"
    firstName = "Jane"
    middleName = "Marie"
    lastName = "Doe"
    dateOfBirth = "1990-05-15"
    gender = 2  // 1=Male, 2=Female, 3=Other
    address = "123 Main Street, Springfield, IL 62701"
    email = "jane.doe@example.org"
    phone = "+1-555-123-4567"
    nationality = "US"
    maritalStatus = 1  // 1=Unmarried, 2=Married, 3=Divorced
    guardian = "John Doe Sr."
    secondaryFullName = "Juana Maria Doe"
    secondaryLanguage = "es"
    locationCode = "US-IL"
    legalStatus = "citizen"
    countryOfIssuance = "US"
}
```

### Field Reference

| Field | Type | Description |
|-------|------|-------------|
| `id` | `String?` | Unique identifier |
| `version` | `String?` | Credential version |
| `language` | `String?` | Primary language code (ISO 639-1) |
| `fullName` | `String?` | Full name |
| `firstName` | `String?` | First/given name |
| `middleName` | `String?` | Middle name |
| `lastName` | `String?` | Last/family name |
| `dateOfBirth` | `String?` | Date of birth (YYYY-MM-DD) |
| `gender` | `Int?` | 1=Male, 2=Female, 3=Other |
| `address` | `String?` | Full address |
| `email` | `String?` | Email address |
| `phone` | `String?` | Phone number |
| `nationality` | `String?` | Nationality code |
| `maritalStatus` | `Int?` | 1=Unmarried, 2=Married, 3=Divorced |
| `guardian` | `String?` | Guardian name |
| `photo` | `ByteArray?` | Photo data |
| `photoFormat` | `Int?` | 1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP |
| `secondaryFullName` | `String?` | Name in secondary language |
| `secondaryLanguage` | `String?` | Secondary language code |
| `locationCode` | `String?` | Location code |
| `legalStatus` | `String?` | Legal status |
| `countryOfIssuance` | `String?` | Issuing country code |

### Including a Photo

```kotlin
import java.io.File

val photoData = File("photo.jpg").readBytes()

val data = claim169 {
    id = "PHOTO-001"
    fullName = "Jane Doe"
    photo = photoData
    photoFormat = 1  // JPEG
}
```

## Creating Token Metadata

### cwtMeta {} Builder

The `cwtMeta {}` builder creates CWT (CBOR Web Token) metadata:

```kotlin
import org.acn.claim169.cwtMeta

val meta = cwtMeta {
    issuer = "https://id.example.org"
    subject = "user-12345"
    expiresAt = System.currentTimeMillis() / 1000 + (365 * 24 * 60 * 60)  // 1 year from now
    issuedAt = System.currentTimeMillis() / 1000
    notBefore = System.currentTimeMillis() / 1000  // Valid immediately
}
```

### Metadata Fields

| Field | Type | Description |
|-------|------|-------------|
| `issuer` | `String?` | Credential issuer (URL or identifier) |
| `subject` | `String?` | Subject identifier |
| `expiresAt` | `Long?` | Expiration timestamp (Unix epoch) |
| `notBefore` | `Long?` | Not valid before timestamp |
| `issuedAt` | `Long?` | Issuance timestamp |

## Signing with Ed25519

Ed25519 is recommended for its small signatures and fast verification.

```kotlin
import org.acn.claim169.Claim169
import org.acn.claim169.claim169
import org.acn.claim169.cwtMeta

// Identity data
val data = claim169 {
    id = "ED25519-001"
    fullName = "Jane Doe"
    dateOfBirth = "1990-05-15"
}

// Token metadata
val meta = cwtMeta {
    issuer = "https://id.example.org"
    expiresAt = 1900000000L
    issuedAt = 1700000000L
}

// Ed25519 private key (32 bytes)
val privateKey = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
    .hexToByteArray()

// Encode
val qrData = Claim169.encode(data, meta) {
    signWithEd25519(privateKey)
}

println("Encoded: ${qrData.length} characters")
```

## Signing with ECDSA P-256

ECDSA P-256 is widely supported in enterprise environments.

```kotlin
import org.acn.claim169.Claim169
import org.acn.claim169.claim169
import org.acn.claim169.cwtMeta

val data = claim169 {
    id = "ECDSA-001"
    fullName = "Jane Doe"
}

val meta = cwtMeta {
    issuer = "https://id.example.org"
    expiresAt = 1900000000L
}

// ECDSA P-256 private key (32 bytes)
val privateKey = ByteArray(32)  // Replace with actual key

val qrData = Claim169.encode(data, meta) {
    signWithEcdsaP256(privateKey)
}
```

## Encoding Without Signature

For testing and development only. Never use in production.

```kotlin
import org.acn.claim169.Claim169
import org.acn.claim169.claim169
import org.acn.claim169.cwtMeta

val data = claim169 {
    id = "TEST-001"
    fullName = "Test User"
}

val meta = cwtMeta {
    expiresAt = 1900000000L
}

// Encode without signature (INSECURE - testing only)
val qrData = Claim169.encode(data, meta) {
    // No signing method configured
}
```

## Skipping Biometrics

To reduce QR code size, skip encoding biometric data:

```kotlin
val qrData = Claim169.encode(data, meta) {
    signWithEd25519(privateKey)
    skipBiometrics()
}
```

## Custom Signer Callback

For HSM or KMS signing, provide a `Signer` implementation:

```kotlin
import org.acn.claim169.Claim169
import org.acn.claim169.Signer

val customSigner = object : Signer {
    override val algorithm: String = "EdDSA"
    override val keyId: ByteArray? = null

    override fun sign(data: ByteArray): ByteArray {
        // Your HSM/KMS signing logic here
        return yourHsm.sign(data)
    }
}

val qrData = Claim169.encode(data, meta) {
    signWith(customSigner)
}
```

## Full Example

Complete example with all demographics:

```kotlin
import org.acn.claim169.Claim169
import org.acn.claim169.claim169
import org.acn.claim169.cwtMeta

// Create comprehensive identity data
val data = claim169 {
    id = "FULL-DEMO-2024-001"
    version = "1.0.0"
    language = "en"
    fullName = "Jane Marie Doe"
    firstName = "Jane"
    middleName = "Marie"
    lastName = "Doe"
    dateOfBirth = "1990-05-15"
    gender = 2
    address = "123 Main Street, Springfield, IL 62701, USA"
    email = "jane.doe@example.org"
    phone = "+1-555-123-4567"
    nationality = "US"
    maritalStatus = 2
    secondaryFullName = "Juana Maria Doe"
    secondaryLanguage = "es"
    locationCode = "US-IL-SPR"
    legalStatus = "citizen"
    countryOfIssuance = "US"
}

// Create token metadata
val now = System.currentTimeMillis() / 1000
val meta = cwtMeta {
    issuer = "https://id.state.il.us"
    subject = "IL-DL-2024-001"
    expiresAt = now + (5 * 365 * 24 * 60 * 60)  // 5 years
    issuedAt = now
    notBefore = now
}

// Sign with Ed25519
val privateKey = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
    .hexToByteArray()

val qrData = Claim169.encode(data, meta) {
    signWithEd25519(privateKey)
}

println("QR Code content (${qrData.length} characters)")
println("Ready for QR code generation")
```

## Error Handling

```kotlin
import org.acn.claim169.Claim169
import org.acn.claim169.Claim169Exception

try {
    val qrData = Claim169.encode(data, meta) {
        signWithEd25519(privateKey)
    }
} catch (e: IllegalArgumentException) {
    println("Invalid key format: ${e.message}")
} catch (e: Claim169Exception) {
    println("Encoding failed: ${e.message}")
}
```

## Next Steps

- [Encryption](encryption.md) -- Add AES-GCM encryption
- [Custom Crypto](custom-crypto.md) -- Use HSM/KMS for signing
- [API Reference](api.md) -- Complete class documentation
