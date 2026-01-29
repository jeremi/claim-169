package fr.acn.claim169

import uniffi.claim169_jni.Claim169Data
import uniffi.claim169_jni.Claim169Encoder
import uniffi.claim169_jni.CwtMetaData
import uniffi.claim169_jni.SignerCallback
import uniffi.claim169_jni.EncryptorCallback
import uniffi.claim169_jni.CryptoException

/**
 * DSL builder for encoding Claim 169 credentials into QR-ready strings.
 *
 * Wraps the UniFFI-generated [Claim169Encoder] with an idiomatic Kotlin API.
 *
 * ## Usage
 * ```kotlin
 * val qrData = Claim169.encode(claim, meta) {
 *     signWithEd25519(privateKey)
 * }
 * ```
 */
class EncoderBuilder(claim169: Claim169Data, cwtMeta: CwtMetaData) {
    private val encoder = Claim169Encoder(claim169, cwtMeta)

    /**
     * Sign with an Ed25519 private key (32 raw bytes).
     *
     * **Security note**: The [privateKey] bytes are passed into native code but the JVM copy
     * remains in the caller's heap. Callers should zeroize the array after encoding completes:
     * ```kotlin
     * privateKey.fill(0)
     * ```
     */
    fun signWithEd25519(privateKey: ByteArray) {
        encoder.signWithEd25519(privateKey)
    }

    /**
     * Sign with an ECDSA P-256 private key (32-byte scalar).
     *
     * **Security note**: The [privateKey] bytes are passed into native code but the JVM copy
     * remains in the caller's heap. Callers should zeroize the array after encoding completes:
     * ```kotlin
     * privateKey.fill(0)
     * ```
     */
    fun signWithEcdsaP256(privateKey: ByteArray) {
        encoder.signWithEcdsaP256(privateKey)
    }

    /**
     * Sign with a custom [Signer] implementation (for HSM/KMS).
     *
     * **Security note**: If the [Signer] holds in-memory key material, implementors should
     * zeroize it when it is no longer needed to minimize exposure on the JVM heap.
     *
     * @param signer The signer implementation
     * @param algorithm COSE algorithm name (e.g., "EdDSA", "ES256")
     */
    fun signWith(signer: Signer, algorithm: String) {
        encoder.signWithCallback(object : SignerCallback {
            override fun sign(
                algorithm: String,
                keyId: ByteArray?,
                data: ByteArray
            ): ByteArray {
                return try {
                    signer.sign(algorithm, keyId, data)
                } catch (e: CryptoException) {
                    throw RuntimeException(e.message ?: "signing failed", e)
                } catch (e: RuntimeException) {
                    throw e
                } catch (e: Exception) {
                    throw RuntimeException(e.message ?: "signing failed", e)
                }
            }

            override fun keyId(): ByteArray? {
                return try {
                    signer.keyId()
                } catch (e: CryptoException) {
                    throw RuntimeException(e.message ?: "key id lookup failed", e)
                } catch (e: RuntimeException) {
                    throw e
                } catch (e: Exception) {
                    throw RuntimeException(e.message ?: "key id lookup failed", e)
                }
            }
        }, algorithm)
    }

    /**
     * Sign with a custom [Signer] implementation using a known COSE algorithm.
     */
    fun signWith(signer: Signer, algorithm: CoseAlgorithm) {
        signWith(signer, algorithm.coseName)
    }

    /**
     * Allow encoding without a signature.
     *
     * **Security warning**: Unsigned credentials cannot be verified.
     */
    fun allowUnsigned() {
        encoder.allowUnsigned()
    }

    /**
     * Encrypt with AES-256-GCM (32-byte key). Nonce is generated randomly.
     */
    fun encryptWithAes256(key: ByteArray) {
        encoder.encryptWithAes256(key)
    }

    /**
     * Encrypt with AES-128-GCM (16-byte key). Nonce is generated randomly.
     */
    fun encryptWithAes128(key: ByteArray) {
        encoder.encryptWithAes128(key)
    }

    /**
     * Encrypt with a custom [Encryptor] implementation (for HSM/KMS).
     *
     * @param encryptor The encryptor implementation
     * @param algorithm COSE algorithm name (e.g., "A256GCM")
     */
    fun encryptWith(encryptor: Encryptor, algorithm: String) {
        encoder.encryptWithCallback(object : EncryptorCallback {
            override fun encrypt(
                algorithm: String,
                keyId: ByteArray?,
                nonce: ByteArray,
                aad: ByteArray,
                plaintext: ByteArray
            ): ByteArray {
                return try {
                    encryptor.encrypt(algorithm, keyId, nonce, aad, plaintext)
                } catch (e: CryptoException) {
                    throw RuntimeException(e.message ?: "encryption failed", e)
                } catch (e: RuntimeException) {
                    throw e
                } catch (e: Exception) {
                    throw RuntimeException(e.message ?: "encryption failed", e)
                }
            }
        }, algorithm)
    }

    /**
     * Encrypt with a custom [Encryptor] implementation using a known COSE algorithm.
     */
    fun encryptWith(encryptor: Encryptor, algorithm: CoseAlgorithm) {
        encryptWith(encryptor, algorithm.coseName)
    }

    /**
     * Skip biometric data during encoding.
     */
    fun skipBiometrics() {
        encoder.skipBiometrics()
    }

    /**
     * Execute the encode and return the Base45-encoded QR string.
     * Called automatically by [Claim169.encode].
     */
    internal fun execute(): String {
        return encoder.execute()
    }
}
