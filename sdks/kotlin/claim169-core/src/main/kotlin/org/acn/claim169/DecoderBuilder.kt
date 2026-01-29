package org.acn.claim169

import uniffi.claim169_jni.Claim169Decoder
import uniffi.claim169_jni.DecodeResultData
import uniffi.claim169_jni.SignatureVerifierCallback
import uniffi.claim169_jni.DecryptorCallback
import uniffi.claim169_jni.CryptoException

/**
 * DSL builder for decoding Claim 169 QR codes.
 *
 * Wraps the UniFFI-generated [Claim169Decoder] with an idiomatic Kotlin API.
 *
 * ## Usage
 * ```kotlin
 * val result = Claim169.decode(qrText) {
 *     verifyWithEd25519(publicKey)
 *     skipBiometrics()
 * }
 * ```
 */
class DecoderBuilder(qrText: String) {
    private val decoder = Claim169Decoder(qrText)

    /**
     * Verify with an Ed25519 public key (32 raw bytes).
     */
    fun verifyWithEd25519(publicKey: ByteArray) {
        decoder.verifyWithEd25519(publicKey)
    }

    /**
     * Verify with an Ed25519 public key in PEM format.
     */
    fun verifyWithEd25519Pem(pem: String) {
        decoder.verifyWithEd25519Pem(pem)
    }

    /**
     * Verify with an ECDSA P-256 public key (SEC1-encoded, 33 or 65 bytes).
     */
    fun verifyWithEcdsaP256(publicKey: ByteArray) {
        decoder.verifyWithEcdsaP256(publicKey)
    }

    /**
     * Verify with an ECDSA P-256 public key in PEM format.
     */
    fun verifyWithEcdsaP256Pem(pem: String) {
        decoder.verifyWithEcdsaP256Pem(pem)
    }

    /**
     * Verify with a custom [SignatureVerifier] implementation (for HSM/KMS).
     */
    fun verifyWith(verifier: SignatureVerifier) {
        decoder.verifyWithCallback(object : SignatureVerifierCallback {
            override fun verify(
                algorithm: String,
                keyId: ByteArray?,
                data: ByteArray,
                signature: ByteArray
            ) {
                try {
                    when (val result = verifier.verify(algorithm, keyId, data, signature)) {
                        is VerificationResult.Valid -> { /* signature accepted */ }
                        is VerificationResult.Invalid -> {
                            throw RuntimeException("signature verification failed: ${result.reason}")
                        }
                    }
                } catch (e: CryptoException) {
                    throw RuntimeException(e.message ?: "signature verification failed", e)
                } catch (e: RuntimeException) {
                    throw e
                } catch (e: Exception) {
                    throw RuntimeException(e.message ?: "signature verification failed", e)
                }
            }
        })
    }

    /**
     * Allow decoding without signature verification.
     *
     * **Security warning**: Credentials decoded with verification skipped (status `Skipped`)
     * cannot be trusted.
     */
    fun allowUnverified() {
        decoder.allowUnverified()
    }

    /**
     * Decrypt with AES-256-GCM (32-byte key).
     */
    fun decryptWithAes256(key: ByteArray) {
        decoder.decryptWithAes256(key)
    }

    /**
     * Decrypt with AES-128-GCM (16-byte key).
     */
    fun decryptWithAes128(key: ByteArray) {
        decoder.decryptWithAes128(key)
    }

    /**
     * Decrypt with a custom [Decryptor] implementation (for HSM/KMS).
     */
    fun decryptWith(decryptor: Decryptor) {
        decoder.decryptWithCallback(object : DecryptorCallback {
            override fun decrypt(
                algorithm: String,
                keyId: ByteArray?,
                nonce: ByteArray,
                aad: ByteArray,
                ciphertext: ByteArray
            ): ByteArray {
                return try {
                    decryptor.decrypt(algorithm, keyId, nonce, aad, ciphertext)
                } catch (e: CryptoException) {
                    throw RuntimeException(e.message ?: "decryption failed", e)
                } catch (e: RuntimeException) {
                    throw e
                } catch (e: Exception) {
                    throw RuntimeException(e.message ?: "decryption failed", e)
                }
            }
        })
    }

    /**
     * Skip biometric data parsing for faster decoding.
     */
    fun skipBiometrics() {
        decoder.skipBiometrics()
    }

    /**
     * Disable timestamp validation (expiration and not-before checks).
     */
    fun withoutTimestampValidation() {
        decoder.withoutTimestampValidation()
    }

    /**
     * Set clock skew tolerance for timestamp validation (in seconds).
     */
    fun clockSkewTolerance(seconds: Long) {
        require(seconds >= 0) { "Clock skew tolerance must be non-negative, got $seconds" }
        decoder.clockSkewTolerance(seconds)
    }

    /**
     * Set maximum decompressed size in bytes (default: 65536).
     */
    fun maxDecompressedBytes(maxBytes: Long) {
        require(maxBytes > 0) { "maxDecompressedBytes must be positive, got $maxBytes" }
        if (isLikely32BitJvm() && maxBytes > UInt.MAX_VALUE.toLong()) {
            throw IllegalArgumentException(
                "maxDecompressedBytes exceeds 32-bit platform limit, got $maxBytes"
            )
        }
        decoder.maxDecompressedBytes(maxBytes.toULong())
    }

    /**
     * Execute the decode and return the result. Called automatically by [Claim169.decode].
     */
    internal fun execute(): DecodeResultData {
        return decoder.execute()
    }
}

private fun isLikely32BitJvm(): Boolean {
    val dataModel = System.getProperty("sun.arch.data.model")
    if (dataModel == "32") return true
    if (dataModel == "64") return false
    val arch = (System.getProperty("os.arch") ?: "").lowercase()
    return !(arch.contains("64") || arch.contains("x86_64") || arch.contains("amd64") || arch.contains("aarch64"))
}
