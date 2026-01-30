package fr.acn.claim169

import uniffi.claim169_jni.BiometricData
import uniffi.claim169_jni.Claim169Data
import uniffi.claim169_jni.DecodeResultData
import java.io.Closeable

/**
 * A [Closeable] wrapper around [DecodeResultData] that zeroizes sensitive byte arrays
 * (biometric templates, photos, and other binary fields) when [close] is called.
 *
 * The Rust core library uses the `zeroize` crate to scrub secrets from memory. On the JVM
 * side, decoded credential data containing biometric templates and photos persists in the
 * heap until garbage collected. This wrapper provides deterministic zeroization so callers
 * can limit the window of exposure.
 *
 * ## Usage
 * ```kotlin
 * CloseableDecodeResult(
 *     Claim169.decode(qrText) { verifyWithEd25519(publicKey) }
 * ).use { result ->
 *     val name = result.data.claim169.fullName
 *     // ... process credential
 * }
 * // All biometric and photo byte arrays are now zeroed.
 * ```
 */
class CloseableDecodeResult(
    /** The underlying decode result. */
    val data: DecodeResultData
) : Closeable {

    /**
     * Zeroizes all sensitive byte arrays within the decoded credential.
     *
     * This fills photo, bestQualityFingers, and all biometric data byte arrays with
     * zeros. After calling this method the byte arrays still exist but contain only
     * zero bytes. Callers should not read the data after closing.
     */
    override fun close() {
        zeroizeClaim169Data(data.claim169)
    }
}

/**
 * Zeroizes all sensitive byte arrays within a [Claim169Data] instance.
 *
 * Fills photo, bestQualityFingers, and every biometric data byte array with zeros.
 */
fun zeroizeClaim169Data(claim: Claim169Data) {
    claim.photo?.fill(0)
    claim.bestQualityFingers?.fill(0)

    claim.rightThumb?.zeroizeBiometrics()
    claim.rightPointerFinger?.zeroizeBiometrics()
    claim.rightMiddleFinger?.zeroizeBiometrics()
    claim.rightRingFinger?.zeroizeBiometrics()
    claim.rightLittleFinger?.zeroizeBiometrics()
    claim.leftThumb?.zeroizeBiometrics()
    claim.leftPointerFinger?.zeroizeBiometrics()
    claim.leftMiddleFinger?.zeroizeBiometrics()
    claim.leftRingFinger?.zeroizeBiometrics()
    claim.leftLittleFinger?.zeroizeBiometrics()
    claim.rightIris?.zeroizeBiometrics()
    claim.leftIris?.zeroizeBiometrics()
    claim.face?.zeroizeBiometrics()
    claim.rightPalm?.zeroizeBiometrics()
    claim.leftPalm?.zeroizeBiometrics()
    claim.voice?.zeroizeBiometrics()
}

/**
 * Zeroizes the data byte array of each [BiometricData] in this list.
 */
private fun List<BiometricData>.zeroizeBiometrics() {
    for (biometric in this) {
        biometric.data.fill(0)
    }
}
