package fr.acn.claim169

import uniffi.claim169_jni.Claim169Exception as NativeClaim169Exception

/**
 * High-level errors from the Claim 169 decoding/encoding pipeline.
 *
 * This mirrors the native error variants while keeping the public Java/Kotlin API
 * in the `fr.acn.claim169` package.
 */
sealed class Claim169Exception(message: String, cause: Throwable? = null) : Exception(message, cause) {
    class Base45Decode(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class Decompress(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class DecompressLimitExceeded(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class CoseParse(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class UnsupportedCoseType(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class SignatureInvalid(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class DecryptionFailed(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class CborParse(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class CwtParse(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class Claim169NotFound(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class Claim169Invalid(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class UnsupportedAlgorithm(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class KeyNotFound(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class Expired(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class NotYetValid(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class Crypto(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class Io(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class CborEncode(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class SignatureFailed(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class EncryptionFailed(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
    class EncodingConfig(message: String, cause: Throwable? = null) : Claim169Exception(message, cause)
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
