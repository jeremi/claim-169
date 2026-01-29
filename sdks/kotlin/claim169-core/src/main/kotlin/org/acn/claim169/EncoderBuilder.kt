package org.acn.claim169

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
     */
    fun signWithEd25519(privateKey: ByteArray) {
        encoder.signWithEd25519(privateKey)
    }

    /**
     * Sign with an ECDSA P-256 private key (32-byte scalar).
     */
    fun signWithEcdsaP256(privateKey: ByteArray) {
        encoder.signWithEcdsaP256(privateKey)
    }

    /**
     * Sign with a custom [Signer] implementation (for HSM/KMS).
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
                return signer.sign(algorithm, keyId, data)
            }

            override fun keyId(): ByteArray? {
                return signer.keyId()
            }
        }, algorithm)
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
                return encryptor.encrypt(algorithm, keyId, nonce, aad, plaintext)
            }
        }, algorithm)
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
