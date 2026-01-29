# Custom Crypto Providers

This guide covers integrating external cryptographic providers such as Android Keystore, Hardware Security Modules (HSMs), and cloud Key Management Services (KMS).

## Overview

The claim169 Kotlin SDK uses interface-based crypto hooks, allowing you to:

- **Sign** with keys stored in Android Keystore, HSMs, or cloud KMS
- **Verify** signatures using external providers
- **Encrypt** with externally managed keys
- **Decrypt** using hardware-backed or cloud-hosted keys

This can help meet security requirements that mandate private key material stays within secure hardware (depending on your provider and configuration).

## Crypto Interfaces

### SignatureVerifier

Implement this interface to provide custom signature verification:

```kotlin
import org.acn.claim169.SignatureVerifier

class MyVerifier : SignatureVerifier {
    override fun verify(
        algorithm: String,      // "EdDSA" or "ES256"
        keyId: ByteArray?,      // Optional key identifier from COSE header
        data: ByteArray,        // Signed data
        signature: ByteArray    // Signature to verify
    ) {
        // Throw any exception if verification fails
        // Return normally if verification succeeds
    }
}
```

### Signer

Implement this interface to provide custom signing:

```kotlin
import org.acn.claim169.Signer

class MySigner : Signer {
    override val algorithm: String = "EdDSA"  // or "ES256"
    override val keyId: ByteArray? = null      // Optional key identifier

    override fun sign(data: ByteArray): ByteArray {
        // Return signature bytes
        return doSign(data)
    }
}
```

### Decryptor

Implement this interface to provide custom decryption:

```kotlin
import org.acn.claim169.Decryptor

class MyDecryptor : Decryptor {
    override fun decrypt(
        algorithm: String,      // "A256GCM" or "A128GCM"
        keyId: ByteArray?,      // Optional key identifier from COSE header
        nonce: ByteArray,       // 12-byte nonce
        aad: ByteArray,         // Additional authenticated data
        ciphertext: ByteArray   // Encrypted data with authentication tag
    ): ByteArray {
        // Return decrypted plaintext
        return doDecrypt(nonce, aad, ciphertext)
    }
}
```

### Encryptor

Implement this interface to provide custom encryption:

```kotlin
import org.acn.claim169.Encryptor

class MyEncryptor : Encryptor {
    override val algorithm: String = "A256GCM"  // or "A128GCM"
    override val keyId: ByteArray? = null        // Optional key identifier

    override fun encrypt(
        nonce: ByteArray,       // 12-byte nonce
        aad: ByteArray,         // Additional authenticated data
        plaintext: ByteArray    // Data to encrypt
    ): ByteArray {
        // Return ciphertext with authentication tag
        return doEncrypt(nonce, aad, plaintext)
    }
}
```

## Android Keystore Integration

Android Keystore provides hardware-backed key storage on Android devices.

### Signing with Android Keystore

```kotlin
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import org.acn.claim169.Claim169
import org.acn.claim169.Signer
import java.security.KeyPairGenerator
import java.security.KeyStore
import java.security.Signature

// Generate an ECDSA P-256 key pair in the Keystore
fun generateKeystoreKey(alias: String) {
    val keyPairGenerator = KeyPairGenerator.getInstance(
        KeyProperties.KEY_ALGORITHM_EC,
        "AndroidKeyStore"
    )
    keyPairGenerator.initialize(
        KeyGenParameterSpec.Builder(alias, KeyProperties.PURPOSE_SIGN or KeyProperties.PURPOSE_VERIFY)
            .setDigests(KeyProperties.DIGEST_SHA256)
            .setAlgorithmParameterSpec(java.security.spec.ECGenParameterSpec("secp256r1"))
            .build()
    )
    keyPairGenerator.generateKeyPair()
}

// Signer using Android Keystore
class AndroidKeystoreSigner(private val keyAlias: String) : Signer {
    override val algorithm: String = "ES256"
    override val keyId: ByteArray? = keyAlias.toByteArray()

    private val keyStore = KeyStore.getInstance("AndroidKeyStore").apply { load(null) }

    override fun sign(data: ByteArray): ByteArray {
        val privateKey = keyStore.getKey(keyAlias, null) as java.security.PrivateKey
        val sig = Signature.getInstance("SHA256withECDSA")
        sig.initSign(privateKey)
        sig.update(data)
        val derSignature = sig.sign()

        // Convert DER to raw r||s format (64 bytes)
        return derToRawEcdsa(derSignature)
    }
}

// Usage
generateKeystoreKey("claim169-signing-key")

val signer = AndroidKeystoreSigner("claim169-signing-key")
val qrData = Claim169.encode(data, meta) {
    signWith(signer)
}
```

### Verifying with Android Keystore

```kotlin
import org.acn.claim169.Claim169
import org.acn.claim169.SignatureVerifier
import java.security.KeyStore
import java.security.Signature

class AndroidKeystoreVerifier(private val keyAlias: String) : SignatureVerifier {
    private val keyStore = KeyStore.getInstance("AndroidKeyStore").apply { load(null) }

    override fun verify(
        algorithm: String,
        keyId: ByteArray?,
        data: ByteArray,
        signature: ByteArray
    ) {
        val publicKey = keyStore.getCertificate(keyAlias).publicKey
        val sig = Signature.getInstance("SHA256withECDSA")
        sig.initVerify(publicKey)
        sig.update(data)

        // Convert raw r||s to DER format
        val derSignature = rawToDerEcdsa(signature)

        if (!sig.verify(derSignature)) {
            throw SecurityException("Signature verification failed")
        }
    }
}

// Usage
val verifier = AndroidKeystoreVerifier("claim169-signing-key")
val result = Claim169.decode(qrData) {
    verifyWith(verifier)
}
```

### DER/Raw ECDSA Conversion Utilities

```kotlin
import java.math.BigInteger

fun derToRawEcdsa(derSignature: ByteArray): ByteArray {
    // Parse DER sequence: 0x30 <len> 0x02 <len> <r> 0x02 <len> <s>
    var offset = 2 // Skip SEQUENCE tag and length
    if (derSignature[0] != 0x30.toByte()) throw IllegalArgumentException("Not a DER sequence")

    // Read r
    if (derSignature[offset] != 0x02.toByte()) throw IllegalArgumentException("Expected INTEGER tag")
    offset++
    val rLen = derSignature[offset].toInt() and 0xFF
    offset++
    val r = BigInteger(1, derSignature.sliceArray(offset until offset + rLen))
    offset += rLen

    // Read s
    if (derSignature[offset] != 0x02.toByte()) throw IllegalArgumentException("Expected INTEGER tag")
    offset++
    val sLen = derSignature[offset].toInt() and 0xFF
    offset++
    val s = BigInteger(1, derSignature.sliceArray(offset until offset + sLen))

    // Convert to fixed-size 32-byte arrays
    val rBytes = r.toByteArray().let { if (it.size > 32) it.sliceArray(it.size - 32 until it.size) else it }
    val sBytes = s.toByteArray().let { if (it.size > 32) it.sliceArray(it.size - 32 until it.size) else it }

    val result = ByteArray(64)
    rBytes.copyInto(result, 32 - rBytes.size)
    sBytes.copyInto(result, 64 - sBytes.size)
    return result
}

fun rawToDerEcdsa(rawSignature: ByteArray): ByteArray {
    require(rawSignature.size == 64) { "Raw ECDSA signature must be 64 bytes" }

    val r = BigInteger(1, rawSignature.sliceArray(0 until 32))
    val s = BigInteger(1, rawSignature.sliceArray(32 until 64))

    val rBytes = r.toByteArray()
    val sBytes = s.toByteArray()

    val totalLen = 2 + rBytes.size + 2 + sBytes.size
    val der = ByteArray(2 + totalLen)
    var offset = 0
    der[offset++] = 0x30  // SEQUENCE
    der[offset++] = totalLen.toByte()
    der[offset++] = 0x02  // INTEGER
    der[offset++] = rBytes.size.toByte()
    rBytes.copyInto(der, offset)
    offset += rBytes.size
    der[offset++] = 0x02  // INTEGER
    der[offset++] = sBytes.size.toByte()
    sBytes.copyInto(der, offset)

    return der
}
```

## AWS KMS Integration

### Signing with AWS KMS

```kotlin
import org.acn.claim169.Claim169
import org.acn.claim169.Signer
import software.amazon.awssdk.services.kms.KmsClient
import software.amazon.awssdk.services.kms.model.MessageType
import software.amazon.awssdk.services.kms.model.SigningAlgorithmSpec

class AwsKmsSigner(
    private val kmsClient: KmsClient,
    private val keyArn: String
) : Signer {
    override val algorithm: String = "ES256"
    override val keyId: ByteArray? = keyArn.toByteArray()

    override fun sign(data: ByteArray): ByteArray {
        val response = kmsClient.sign { builder ->
            builder.keyId(keyArn)
            builder.message(software.amazon.awssdk.core.SdkBytes.fromByteArray(data))
            builder.messageType(MessageType.RAW)
            builder.signingAlgorithm(SigningAlgorithmSpec.ECDSA_SHA_256)
        }

        // AWS KMS returns DER-encoded signature, convert to raw r||s
        return derToRawEcdsa(response.signature().asByteArray())
    }
}

// Usage
val kmsClient = KmsClient.builder().build()
val signer = AwsKmsSigner(kmsClient, "arn:aws:kms:us-east-1:123456789012:key/...")

val qrData = Claim169.encode(data, meta) {
    signWith(signer)
}
```

### Verifying with AWS KMS

```kotlin
import org.acn.claim169.SignatureVerifier
import software.amazon.awssdk.services.kms.KmsClient
import software.amazon.awssdk.services.kms.model.SigningAlgorithmSpec

class AwsKmsVerifier(
    private val kmsClient: KmsClient,
    private val keyArn: String
) : SignatureVerifier {
    override fun verify(
        algorithm: String,
        keyId: ByteArray?,
        data: ByteArray,
        signature: ByteArray
    ) {
        // Convert raw r||s back to DER for AWS KMS
        val derSignature = rawToDerEcdsa(signature)

        val response = kmsClient.verify { builder ->
            builder.keyId(keyArn)
            builder.message(software.amazon.awssdk.core.SdkBytes.fromByteArray(data))
            builder.messageType(software.amazon.awssdk.services.kms.model.MessageType.RAW)
            builder.signature(software.amazon.awssdk.core.SdkBytes.fromByteArray(derSignature))
            builder.signingAlgorithm(SigningAlgorithmSpec.ECDSA_SHA_256)
        }

        if (!response.signatureValid()) {
            throw SecurityException("AWS KMS signature verification failed")
        }
    }
}
```

## Azure Key Vault Integration

### Signing with Azure Key Vault

```kotlin
import com.azure.identity.DefaultAzureCredentialBuilder
import com.azure.security.keyvault.keys.cryptography.CryptographyClient
import com.azure.security.keyvault.keys.cryptography.CryptographyClientBuilder
import com.azure.security.keyvault.keys.cryptography.models.SignatureAlgorithm
import org.acn.claim169.Signer
import java.security.MessageDigest

class AzureKeyVaultSigner(
    private val cryptoClient: CryptographyClient
) : Signer {
    override val algorithm: String = "ES256"
    override val keyId: ByteArray? = null

    override fun sign(data: ByteArray): ByteArray {
        // Azure requires pre-hashed data for ECDSA
        val digest = MessageDigest.getInstance("SHA-256").digest(data)
        val result = cryptoClient.sign(SignatureAlgorithm.ES256, digest)
        return result.signature
    }
}

// Usage
val credential = DefaultAzureCredentialBuilder().build()
val cryptoClient = CryptographyClientBuilder()
    .credential(credential)
    .keyIdentifier("https://my-vault.vault.azure.net/keys/my-key/version")
    .buildClient()

val signer = AzureKeyVaultSigner(cryptoClient)
val qrData = Claim169.encode(data, meta) {
    signWith(signer)
}
```

## Google Cloud KMS Integration

### Signing with Google Cloud KMS

```kotlin
import com.google.cloud.kms.v1.AsymmetricSignRequest
import com.google.cloud.kms.v1.CryptoKeyVersionName
import com.google.cloud.kms.v1.Digest
import com.google.cloud.kms.v1.KeyManagementServiceClient
import com.google.protobuf.ByteString
import org.acn.claim169.Signer
import java.security.MessageDigest

class GcpKmsSigner(
    private val kmsClient: KeyManagementServiceClient,
    private val keyVersionName: CryptoKeyVersionName
) : Signer {
    override val algorithm: String = "ES256"
    override val keyId: ByteArray? = null

    override fun sign(data: ByteArray): ByteArray {
        val sha256Digest = MessageDigest.getInstance("SHA-256").digest(data)
        val digest = Digest.newBuilder()
            .setSha256(ByteString.copyFrom(sha256Digest))
            .build()

        val response = kmsClient.asymmetricSign(
            AsymmetricSignRequest.newBuilder()
                .setName(keyVersionName.toString())
                .setDigest(digest)
                .build()
        )

        // GCP returns DER, convert to raw r||s
        return derToRawEcdsa(response.signature.toByteArray())
    }
}

// Usage
val kmsClient = KeyManagementServiceClient.create()
val keyVersionName = CryptoKeyVersionName.of(
    "my-project", "us-east1", "my-keyring", "my-key", "1"
)

val signer = GcpKmsSigner(kmsClient, keyVersionName)
val qrData = Claim169.encode(data, meta) {
    signWith(signer)
}
```

## HSM/PKCS#11 Integration Pattern

For PKCS#11 HSMs using a JCA provider:

```kotlin
import org.acn.claim169.Signer
import java.security.KeyStore
import java.security.Signature

class Pkcs11Signer(
    private val keyAlias: String,
    private val pin: CharArray
) : Signer {
    override val algorithm: String = "ES256"
    override val keyId: ByteArray? = null

    private val keyStore: KeyStore = KeyStore.getInstance("PKCS11").apply {
        load(null, pin)
    }

    override fun sign(data: ByteArray): ByteArray {
        val privateKey = keyStore.getKey(keyAlias, pin) as java.security.PrivateKey
        val sig = Signature.getInstance("SHA256withECDSA")
        sig.initSign(privateKey)
        sig.update(data)
        val derSignature = sig.sign()

        return derToRawEcdsa(derSignature)
    }
}

// Usage
val signer = Pkcs11Signer("my-signing-key", "1234".toCharArray())
val qrData = Claim169.encode(data, meta) {
    signWith(signer)
}
```

## Error Handling

Wrap provider-specific exceptions so they surface as meaningful messages:

```kotlin
import org.acn.claim169.Signer

class SafeSigner(private val delegate: Signer) : Signer {
    override val algorithm: String = delegate.algorithm
    override val keyId: ByteArray? = delegate.keyId

    override fun sign(data: ByteArray): ByteArray {
        return try {
            delegate.sign(data)
        } catch (e: java.net.ConnectException) {
            throw RuntimeException("Crypto provider unavailable: ${e.message}", e)
        } catch (e: SecurityException) {
            throw RuntimeException("Access denied to signing key: ${e.message}", e)
        }
    }
}

// Usage
val safeSigner = SafeSigner(AwsKmsSigner(kmsClient, keyArn))

try {
    val qrData = Claim169.encode(data, meta) {
        signWith(safeSigner)
    }
} catch (e: Claim169Exception) {
    println("Encoding failed: ${e.message}")
}
```

## Best Practices

### Key Rotation

- Implement key versioning in your KMS
- Use `keyId` property to track key versions
- Plan for graceful key rotation

### Error Handling

- Catch and wrap provider-specific exceptions
- Implement retry logic for transient failures
- Log crypto operations for audit trails

### Performance

- Cache KMS clients and connections
- Consider connection pooling for HSMs
- Use coroutines with suspending wrappers for async KMS calls

### Security

- Use IAM roles/managed identities, not static credentials
- Enable audit logging on your KMS
- Implement proper key access controls
- On Android, use hardware-backed key attestation

## Next Steps

- [API Reference](api.md) -- Complete interface documentation
- [Troubleshooting](troubleshooting.md) -- Common errors and solutions
