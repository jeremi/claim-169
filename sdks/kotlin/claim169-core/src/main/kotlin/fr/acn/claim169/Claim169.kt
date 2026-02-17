package fr.acn.claim169

import java.security.SecureRandom
import uniffi.claim169_jni.Claim169Exception as NativeClaim169Exception
import uniffi.claim169_jni.inspect as nativeInspect
import uniffi.claim169_jni.version as nativeVersion

fun interface DecoderConfigurer {
    fun configure(builder: DecoderBuilder)
}

fun interface EncoderConfigurer {
    fun configure(builder: EncoderBuilder)
}

/**
 * Main entry point for the MOSIP Claim 169 SDK.
 *
 * Provides DSL-style decode and encode operations for Claim 169 QR codes.
 *
 * ## Decoding
 * ```kotlin
 * val result = Claim169.decode(qrText) {
 *     verifyWithEd25519(publicKey)
 * }
 * println(result.claim169.fullName)
 * ```
 *
 * ## Encoding
 * ```kotlin
 * val qrData = Claim169.encode(
 *     claim169Data {
 *         id = "ID-12345"
 *         fullName = "Jane Doe"
 *     },
 *     cwtMetaData {
 *         issuer = "https://issuer.example.com"
 *     }
 * ) {
 *     signWithEd25519(privateKey)
 * }
 * ```
 */
object Claim169 {

    /**
     * Decode a Claim 169 QR code string.
     *
     * @param qrText The Base45-encoded QR code content
     * @param configure DSL block to configure verification, decryption, and options
     * @return The decoded result containing claim data, CWT metadata, and verification status
     * @throws Claim169Exception on decode errors
     */
    @Throws(Claim169Exception::class)
    fun decode(
        qrText: String,
        configure: DecoderBuilder.() -> Unit
    ): DecodeResultData {
        try {
            val builder = DecoderBuilder(qrText)
            builder.configure()
            return builder.execute()
        } catch (e: NativeClaim169Exception) {
            throw e.toSdkException()
        }
    }

    @JvmStatic
    @JvmName("decode")
    @Throws(Claim169Exception::class)
    fun decodeWith(
        qrText: String,
        configure: DecoderConfigurer
    ): DecodeResultData {
        try {
            val builder = DecoderBuilder(qrText)
            configure.configure(builder)
            return builder.execute()
        } catch (e: NativeClaim169Exception) {
            throw e.toSdkException()
        }
    }

    /**
     * Decode a Claim 169 QR code string and return a closeable wrapper that zeroizes
     * sensitive byte arrays when closed.
     */
    @Throws(Claim169Exception::class)
    fun decodeCloseable(
        qrText: String,
        configure: DecoderBuilder.() -> Unit
    ): CloseableDecodeResult {
        return CloseableDecodeResult(decode(qrText, configure))
    }

    @JvmStatic
    @JvmName("decodeCloseable")
    @Throws(Claim169Exception::class)
    fun decodeCloseableWith(
        qrText: String,
        configure: DecoderConfigurer
    ): CloseableDecodeResult {
        return CloseableDecodeResult(decodeWith(qrText, configure))
    }

    /**
     * Encode Claim 169 data into a QR-ready Base45 string.
     *
     * @param claim169 The identity claim data
     * @param cwtMeta The CWT metadata (issuer, expiration, etc.)
     * @param configure DSL block to configure signing, encryption, and options
     * @return The Base45-encoded QR string
     * @throws Claim169Exception on encode errors
     */
    @Throws(Claim169Exception::class)
    fun encode(
        claim169: Claim169Data,
        cwtMeta: CwtMetaData,
        configure: EncoderBuilder.() -> Unit
    ): String {
        try {
            val builder = EncoderBuilder(claim169, cwtMeta)
            builder.configure()
            return builder.execute()
        } catch (e: NativeClaim169Exception) {
            throw e.toSdkException()
        }
    }

    @JvmStatic
    @JvmName("encode")
    @Throws(Claim169Exception::class)
    fun encodeWith(
        claim169: Claim169Data,
        cwtMeta: CwtMetaData,
        configure: EncoderConfigurer
    ): String {
        try {
            val builder = EncoderBuilder(claim169, cwtMeta)
            configure.configure(builder)
            return builder.execute()
        } catch (e: NativeClaim169Exception) {
            throw e.toSdkException()
        }
    }

    /**
     * Inspect credential metadata without full decoding or verification.
     *
     * Extracts metadata (issuer, key ID, algorithm, expiration) from a QR code
     * without verifying the signature. Useful for multi-issuer key lookup.
     *
     * For encrypted credentials (COSE_Encrypt0), only COSE-level headers are
     * available; CWT-level fields (issuer, subject, expiresAt) will be `null`.
     *
     * @param qrText The Base45-encoded QR code content
     * @return Metadata extracted from the credential
     * @throws Claim169Exception on parse errors
     */
    @JvmStatic
    @Throws(Claim169Exception::class)
    fun inspect(qrText: String): InspectResultData {
        try {
            val native = nativeInspect(qrText)
            return InspectResultData.fromNative(native)
        } catch (e: NativeClaim169Exception) {
            throw e.toSdkException()
        }
    }

    /**
     * Get the native library version.
     */
    @JvmStatic
    fun version(): String = nativeVersion()

    /**
     * Get the [VerificationStatus] enum from a decode result.
     *
     * Java-friendly alternative to `result.getVerificationStatus()`.
     *
     * From Java: `Claim169.verificationStatus(result)`
     */
    @JvmStatic
    fun verificationStatus(result: DecodeResultData): VerificationStatus =
        result.verificationStatus

    /**
     * Generate a cryptographically secure random 12-byte nonce suitable for AES-GCM encryption.
     *
     * @return 12 random bytes from [SecureRandom].
     */
    @JvmStatic
    fun generateNonce(): ByteArray {
        val nonce = ByteArray(12)
        SecureRandom().nextBytes(nonce)
        return nonce
    }

    /**
     * Decode a Claim 169 QR code string, wrapping the result in [kotlin.Result].
     *
     * ```kotlin
     * val result = Claim169.decodeCatching(qrText) { allowUnverified() }
     * result.onSuccess { data -> println(data.claim169.fullName) }
     * result.onFailure { error -> println("Decode failed: $error") }
     * ```
     */
    fun decodeCatching(
        qrText: String,
        configure: DecoderBuilder.() -> Unit
    ): Result<DecodeResultData> = runCatching { decode(qrText, configure) }

    /**
     * Create a [Claim169Data] using a [Claim169DataConfigurer].
     *
     * Java-friendly alternative to the `claim169Data {}` DSL function.
     *
     * From Java: `Claim169.claim169(b -> { b.setId("X"); })`
     */
    @JvmStatic
    @JvmName("claim169")
    fun claim169With(configure: Claim169DataConfigurer): Claim169Data {
        val builder = Claim169DataBuilder()
        configure.configure(builder)
        return builder.build()
    }

    /**
     * Create a [CwtMetaData] using a [CwtMetaDataConfigurer].
     *
     * Java-friendly alternative to the `cwtMetaData {}` DSL function.
     *
     * From Java: `Claim169.cwtMeta(b -> { b.setIssuer("https://..."); })`
     */
    @JvmStatic
    @JvmName("cwtMeta")
    fun cwtMetaWith(configure: CwtMetaDataConfigurer): CwtMetaData {
        val builder = CwtMetaDataBuilder()
        configure.configure(builder)
        return builder.build()
    }
}
