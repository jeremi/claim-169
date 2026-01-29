package org.mosip.claim169

/**
 * Interface for custom signature verification.
 *
 * Implement this for HSM, KMS, or other custom crypto backends.
 * Uses [ByteArray] instead of the UniFFI-generated `List<UByte>` for idiomatic Kotlin.
 *
 * ## Usage
 * ```kotlin
 * val result = Claim169.decode(qrText) {
 *     verifyWith(object : SignatureVerifier {
 *         override fun verify(algorithm: String, keyId: ByteArray?, data: ByteArray, signature: ByteArray) {
 *             hsmProvider.verify(keyId, data, signature)
 *         }
 *     })
 * }
 * ```
 */
interface SignatureVerifier {
    /**
     * Verify a digital signature.
     *
     * @param algorithm COSE algorithm name (e.g., "EdDSA", "ES256")
     * @param keyId Optional key identifier bytes
     * @param data The data that was signed (COSE Sig_structure)
     * @param signature The signature bytes to verify
     * @throws Exception if verification fails
     */
    fun verify(algorithm: String, keyId: ByteArray?, data: ByteArray, signature: ByteArray)
}

/**
 * Interface for custom decryption.
 *
 * Implement this for HSM, KMS, or other custom crypto backends.
 */
interface Decryptor {
    /**
     * Decrypt ciphertext using AEAD.
     *
     * @param algorithm COSE algorithm name (e.g., "A256GCM")
     * @param keyId Optional key identifier bytes
     * @param nonce The IV/nonce
     * @param aad Additional authenticated data
     * @param ciphertext The ciphertext to decrypt (includes auth tag for AEAD)
     * @return The decrypted plaintext bytes
     * @throws Exception if decryption fails
     */
    fun decrypt(algorithm: String, keyId: ByteArray?, nonce: ByteArray, aad: ByteArray, ciphertext: ByteArray): ByteArray
}

/**
 * Interface for custom signing.
 *
 * Implement this for HSM, KMS, or other custom crypto backends.
 */
interface Signer {
    /**
     * Sign data and return the signature.
     *
     * @param algorithm COSE algorithm name (e.g., "EdDSA", "ES256")
     * @param keyId Optional key identifier bytes
     * @param data The data to sign (COSE Sig_structure)
     * @return The signature bytes
     * @throws Exception if signing fails
     */
    fun sign(algorithm: String, keyId: ByteArray?, data: ByteArray): ByteArray

    /**
     * Get the key ID for this signer. Returns null if no key ID.
     */
    fun keyId(): ByteArray? = null
}

/**
 * Interface for custom encryption.
 *
 * Implement this for HSM, KMS, or other custom crypto backends.
 */
interface Encryptor {
    /**
     * Encrypt plaintext using AEAD.
     *
     * @param algorithm COSE algorithm name (e.g., "A256GCM")
     * @param keyId Optional key identifier bytes
     * @param nonce The IV/nonce
     * @param aad Additional authenticated data
     * @param plaintext The plaintext to encrypt
     * @return The ciphertext bytes (includes auth tag for AEAD)
     * @throws Exception if encryption fails
     */
    fun encrypt(algorithm: String, keyId: ByteArray?, nonce: ByteArray, aad: ByteArray, plaintext: ByteArray): ByteArray
}
