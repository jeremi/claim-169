package fr.acn.claim169

import uniffi.claim169_jni.Claim169Exception as NativeClaim169Exception

/**
 * High-level errors from the Claim 169 decoding/encoding pipeline.
 *
 * This mirrors the native error variants while keeping the public Java/Kotlin API
 * in the `fr.acn.claim169` package.
 */
sealed class Claim169Exception(message: String, cause: Throwable? = null) : Exception(message, cause) {
    /** The input string is not valid Base45 encoding. */
    class Base45Decode(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** Zlib/Brotli decompression of the decoded payload failed. */
    class Decompress(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** Decompressed data exceeds the configured size limit (decompression bomb protection). */
    class DecompressLimitExceeded(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** The decompressed bytes are not a valid COSE_Sign1 or COSE_Encrypt0 structure. */
    class CoseParse(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** The COSE structure uses an unsupported type (neither Sign1 nor Encrypt0). */
    class UnsupportedCoseType(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** The COSE_Sign1 signature does not match the provided public key. */
    class SignatureInvalid(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** COSE_Encrypt0 decryption failed (wrong key, corrupted ciphertext, or AAD mismatch). */
    class DecryptionFailed(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** The COSE payload is not valid CBOR. */
    class CborParse(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** The CBOR payload is not a valid CWT (CBOR Web Token). */
    class CwtParse(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** The CWT payload does not contain a claim 169 entry. */
    class Claim169NotFound(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** The claim 169 CBOR map contains structurally invalid data. */
    class Claim169Invalid(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** The COSE algorithm header specifies an unsupported algorithm. */
    class UnsupportedAlgorithm(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** No key was found matching the COSE key ID header. */
    class KeyNotFound(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** The credential's `exp` (expiration) timestamp has passed. */
    class Expired(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** The credential's `nbf` (not-before) timestamp is in the future. */
    class NotYetValid(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** A low-level cryptographic operation failed. */
    class Crypto(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** An I/O error occurred during encoding or decoding. */
    class Io(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** CBOR encoding of claim data failed. */
    class CborEncode(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** Signing the COSE_Sign1 payload failed. */
    class SignatureFailed(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** Encrypting the COSE_Encrypt0 payload failed. */
    class EncryptionFailed(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** Invalid encoder configuration (e.g., no signing method specified without [EncoderBuilder.allowUnsigned]). */
    class EncodingConfig(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    /** Invalid decoder configuration (e.g., no verification method specified without [DecoderBuilder.allowUnverified]). */
    class DecodingConfig(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
}

internal fun NativeClaim169Exception.toSdkException(): Claim169Exception {
    val msg = message ?: this::class.simpleName.orEmpty()
    return when (this) {
        is NativeClaim169Exception.Base45Decode -> Claim169Exception.Base45Decode(msg, this)
        is NativeClaim169Exception.Decompress -> Claim169Exception.Decompress(msg, this)
        is NativeClaim169Exception.DecompressLimitExceeded -> Claim169Exception.DecompressLimitExceeded(msg, this)
        is NativeClaim169Exception.CoseParse -> Claim169Exception.CoseParse(msg, this)
        is NativeClaim169Exception.UnsupportedCoseType -> Claim169Exception.UnsupportedCoseType(msg, this)
        is NativeClaim169Exception.SignatureInvalid -> Claim169Exception.SignatureInvalid(msg, this)
        is NativeClaim169Exception.DecryptionFailed -> Claim169Exception.DecryptionFailed(msg, this)
        is NativeClaim169Exception.CborParse -> Claim169Exception.CborParse(msg, this)
        is NativeClaim169Exception.CwtParse -> Claim169Exception.CwtParse(msg, this)
        is NativeClaim169Exception.Claim169NotFound -> Claim169Exception.Claim169NotFound(msg, this)
        is NativeClaim169Exception.Claim169Invalid -> Claim169Exception.Claim169Invalid(msg, this)
        is NativeClaim169Exception.UnsupportedAlgorithm -> Claim169Exception.UnsupportedAlgorithm(msg, this)
        is NativeClaim169Exception.KeyNotFound -> Claim169Exception.KeyNotFound(msg, this)
        is NativeClaim169Exception.Expired -> Claim169Exception.Expired(msg, this)
        is NativeClaim169Exception.NotYetValid -> Claim169Exception.NotYetValid(msg, this)
        is NativeClaim169Exception.Crypto -> Claim169Exception.Crypto(msg, this)
        is NativeClaim169Exception.Io -> Claim169Exception.Io(msg, this)
        is NativeClaim169Exception.CborEncode -> Claim169Exception.CborEncode(msg, this)
        is NativeClaim169Exception.SignatureFailed -> Claim169Exception.SignatureFailed(msg, this)
        is NativeClaim169Exception.EncryptionFailed -> Claim169Exception.EncryptionFailed(msg, this)
        is NativeClaim169Exception.EncodingConfig -> Claim169Exception.EncodingConfig(msg, this)
        is NativeClaim169Exception.DecodingConfig -> Claim169Exception.DecodingConfig(msg, this)
    }
}
