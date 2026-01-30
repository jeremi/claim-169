package fr.acn.claim169

import uniffi.claim169_jni.Claim169Decoder
import uniffi.claim169_jni.Claim169Encoder
import uniffi.claim169_jni.Claim169Data
import uniffi.claim169_jni.CwtMetaData
import uniffi.claim169_jni.DecodeResultData
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
 *     claim169 {
 *         id = "ID-12345"
 *         fullName = "Jane Doe"
 *     },
 *     cwtMeta {
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
     * @throws uniffi.claim169_jni.Claim169Exception on decode errors
     */
    fun decode(
        qrText: String,
        configure: DecoderBuilder.() -> Unit
    ): DecodeResultData {
        val builder = DecoderBuilder(qrText)
        builder.configure()
        return builder.execute()
    }

    @JvmStatic
    @JvmName("decode")
    fun decodeWith(
        qrText: String,
        configure: DecoderConfigurer
    ): DecodeResultData {
        val builder = DecoderBuilder(qrText)
        configure.configure(builder)
        return builder.execute()
    }

    /**
     * Decode a Claim 169 QR code string and return a closeable wrapper that zeroizes
     * sensitive byte arrays when closed.
     */
    fun decodeCloseable(
        qrText: String,
        configure: DecoderBuilder.() -> Unit
    ): CloseableDecodeResult {
        return CloseableDecodeResult(decode(qrText, configure))
    }

    @JvmStatic
    @JvmName("decodeCloseable")
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
     * @throws uniffi.claim169_jni.Claim169Exception on encode errors
     */
    fun encode(
        claim169: Claim169Data,
        cwtMeta: CwtMetaData,
        configure: EncoderBuilder.() -> Unit
    ): String {
        val builder = EncoderBuilder(claim169, cwtMeta)
        builder.configure()
        return builder.execute()
    }

    @JvmStatic
    @JvmName("encode")
    fun encodeWith(
        claim169: Claim169Data,
        cwtMeta: CwtMetaData,
        configure: EncoderConfigurer
    ): String {
        val builder = EncoderBuilder(claim169, cwtMeta)
        configure.configure(builder)
        return builder.execute()
    }

    /**
     * Get the native library version.
     */
    fun version(): String = nativeVersion()
}
