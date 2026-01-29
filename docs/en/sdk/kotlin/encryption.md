# Encryption

This guide covers encrypting credentials with AES-GCM for additional privacy protection.

## When to Use Encryption

Encrypt credentials when:

- QR codes may be photographed by third parties
- Credentials contain sensitive biometric data
- Privacy regulations require data protection
- Credentials are shared across trust boundaries

## Encryption Overview

The library supports **sign-then-encrypt**: credentials are first signed, then the signed payload is encrypted.

```
Identity Data -> Sign -> Encrypt -> Compress -> Base45 -> QR Code
```

Decryption reverses the process:

```
QR Code -> Base45 -> Decompress -> Decrypt -> Verify -> Identity Data
```

## Supported Algorithms

| Algorithm | Key Size | Nonce Size | Use Case |
|-----------|----------|------------|----------|
| AES-256-GCM | 32 bytes | 12 bytes | High security (recommended) |
| AES-128-GCM | 16 bytes | 12 bytes | Standard security |

## Encoding with Encryption

### Sign + Encrypt with AES-256-GCM

```kotlin
import org.acn.claim169.Claim169
import org.acn.claim169.claim169
import org.acn.claim169.cwtMeta

// Identity data
val data = claim169 {
    id = "ENC-001"
    fullName = "Jane Doe"
}

val meta = cwtMeta {
    issuer = "https://id.example.org"
    expiresAt = 1900000000L
}

// Keys
val signKey = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
    .hexToByteArray()  // Ed25519 private key (32 bytes)

val encryptKey = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
    .hexToByteArray()  // AES-256 key (32 bytes)

// Encode with signing and encryption
val qrData = Claim169.encode(data, meta) {
    signWithEd25519(signKey)
    encryptWithAes256(encryptKey)
}

println("Encrypted credential: ${qrData.length} characters")
```

### Sign + Encrypt with AES-128-GCM

```kotlin
import org.acn.claim169.Claim169
import org.acn.claim169.claim169
import org.acn.claim169.cwtMeta

val data = claim169 {
    id = "ENC128-001"
    fullName = "Jane Doe"
}

val meta = cwtMeta {
    issuer = "https://id.example.org"
    expiresAt = 1900000000L
}

val signKey = ByteArray(32)  // Ed25519 private key
val encryptKey = ByteArray(16)  // AES-128 key (16 bytes)

val qrData = Claim169.encode(data, meta) {
    signWithEd25519(signKey)
    encryptWithAes128(encryptKey)
}
```

## Decoding Encrypted Credentials

### Decrypt AES-256-GCM with Verification

For encrypted credentials, you typically need both:

1. Decryption key (symmetric AES key)
2. Verification key (signing public key) or verification callback

```kotlin
import org.acn.claim169.Claim169

// Keys
val encryptKey = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
    .hexToByteArray()
val publicKey = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
    .hexToByteArray()

// Decode, decrypt, and verify
val result = Claim169.decode(qrData) {
    decryptWithAes256(encryptKey)
    verifyWithEd25519(publicKey)
}

println("ID: ${result.claim169.id}")
println("Verified: ${result.isVerified}")
```

### Decrypt AES-128-GCM with Verification

```kotlin
val encryptKey128 = ByteArray(16)  // AES-128 key

val result = Claim169.decode(qrData) {
    decryptWithAes128(encryptKey128)
    verifyWithEd25519(publicKey)
}
```

### Decrypt Without Signature Verification

For testing only:

```kotlin
// WARNING: INSECURE - skips signature verification
val result = Claim169.decode(qrData) {
    decryptWithAes256(encryptKey)
    allowUnverified()
}

println("Status: ${result.verificationStatus}")  // "skipped"
```

## Custom Decryption

For HSM or KMS decryption, provide a `Decryptor` implementation:

```kotlin
import org.acn.claim169.Claim169
import org.acn.claim169.Decryptor

val customDecryptor = object : Decryptor {
    override fun decrypt(
        algorithm: String,
        keyId: ByteArray?,
        nonce: ByteArray,
        aad: ByteArray,
        ciphertext: ByteArray
    ): ByteArray {
        // Your HSM/KMS decryption logic
        return yourHsm.decrypt(algorithm, nonce, aad, ciphertext)
    }
}

val result = Claim169.decode(qrData) {
    decryptWith(customDecryptor)
    verifyWithEd25519(publicKey)
}
```

## Custom Encryption

For HSM or KMS encryption during encoding:

```kotlin
import org.acn.claim169.Claim169
import org.acn.claim169.Encryptor
import org.acn.claim169.CoseAlgorithm

val customEncryptor = object : Encryptor {
    override fun encrypt(
        algorithm: String,
        keyId: ByteArray?,
        nonce: ByteArray,
        aad: ByteArray,
        plaintext: ByteArray
    ): ByteArray {
        // Your HSM/KMS encryption logic
        return yourHsm.encrypt(nonce, aad, plaintext)
    }
}

val qrData = Claim169.encode(data, meta) {
    signWithEd25519(signKey)
    encryptWith(customEncryptor, CoseAlgorithm.A256GCM)
}
```

The `CoseAlgorithm` enum provides type-safe algorithm selection. You can also pass a raw string:

```kotlin
encryptWith(customEncryptor, "A256GCM")
```

## Full Custom Crypto

Use custom callbacks for both signing and encryption:

```kotlin
import org.acn.claim169.Claim169
import org.acn.claim169.Signer
import org.acn.claim169.Encryptor
import org.acn.claim169.SignatureVerifier
import org.acn.claim169.Decryptor
import org.acn.claim169.claim169
import org.acn.claim169.cwtMeta

// Custom signer
val signer = object : Signer {
    override val algorithm: String = "EdDSA"
    override val keyId: ByteArray? = null

    override fun sign(data: ByteArray): ByteArray {
        return yourSigningProvider.sign(data)
    }
}

// Custom encryptor
val encryptor = object : Encryptor {
    override val algorithm: String = "A256GCM"
    override val keyId: ByteArray? = null

    override fun encrypt(nonce: ByteArray, aad: ByteArray, plaintext: ByteArray): ByteArray {
        return yourEncryptionProvider.encrypt(nonce, aad, plaintext)
    }
}

// Encode with both custom callbacks
val data = claim169 {
    id = "CUSTOM-001"
    fullName = "Jane Doe"
}

val meta = cwtMeta {
    issuer = "https://id.example.org"
}

val qrData = Claim169.encode(data, meta) {
    signWith(signer)
    encryptWith(encryptor)
}

// Custom verifier and decryptor
val verifier = object : SignatureVerifier {
    override fun verify(algorithm: String, keyId: ByteArray?, data: ByteArray, signature: ByteArray) {
        yourSigningProvider.verify(data, signature)
    }
}

val decryptor = object : Decryptor {
    override fun decrypt(algorithm: String, keyId: ByteArray?, nonce: ByteArray, aad: ByteArray, ciphertext: ByteArray): ByteArray {
        return yourEncryptionProvider.decrypt(nonce, aad, ciphertext)
    }
}

// Decode with custom callbacks
val result = Claim169.decode(qrData) {
    decryptWith(decryptor)
    verifyWith(verifier)
}

println("ID: ${result.claim169.id}")
println("Verified: ${result.isVerified}")
```

## Error Handling

```kotlin
import org.acn.claim169.Claim169
import org.acn.claim169.Claim169Exception

try {
    val result = Claim169.decode(qrData) {
        decryptWithAes256(encryptKey)
        allowUnverified()
    }
} catch (e: Claim169Exception.DecryptionError) {
    // Decryption failed (wrong key, corrupted data, etc.)
    println("Decryption failed: ${e.message}")
} catch (e: IllegalArgumentException) {
    // Invalid key size
    println("Invalid key: ${e.message}")
} catch (e: Claim169Exception) {
    // Other errors
    println("Error: ${e.message}")
}
```

## Security Best Practices

### Key Management

- **Never hardcode keys** in source code
- **Use secure storage** (Android Keystore, HSM, KMS, secret manager)
- **Rotate keys** periodically
- **Limit key access** to authorized systems only

### Nonce Requirements

- **Never reuse nonces** with the same key
- The library generates random nonces automatically
- For custom encryption, always use cryptographically secure random nonces

### Key Distribution

- Distribute encryption keys through secure channels
- Consider using key derivation functions for shared secrets
- Implement proper key exchange protocols for distributed systems

## Next Steps

- [Custom Crypto](custom-crypto.md) -- HSM/KMS integration examples
- [API Reference](api.md) -- Complete class documentation
