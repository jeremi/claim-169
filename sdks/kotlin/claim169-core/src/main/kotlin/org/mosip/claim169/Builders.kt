package org.mosip.claim169

import uniffi.claim169_jni.BiometricData
import uniffi.claim169_jni.Claim169Data
import uniffi.claim169_jni.CwtMetaData

/**
 * DSL builder for creating [Claim169Data] instances.
 *
 * ## Usage
 * ```kotlin
 * val data = claim169 {
 *     id = "ID-12345"
 *     fullName = "Jane Doe"
 *     dateOfBirth = "19900115"
 *     gender = 2L  // Female
 *     email = "jane@example.com"
 * }
 * ```
 */
class Claim169DataBuilder {
    var id: String? = null
    var version: String? = null
    var language: String? = null
    var fullName: String? = null
    var firstName: String? = null
    var middleName: String? = null
    var lastName: String? = null
    var dateOfBirth: String? = null
    var gender: Long? = null
    var address: String? = null
    var email: String? = null
    var phone: String? = null
    var nationality: String? = null
    var maritalStatus: Long? = null
    var guardian: String? = null
    var photo: ByteArray? = null
    var photoFormat: Long? = null
    var bestQualityFingers: ByteArray? = null
    var secondaryFullName: String? = null
    var secondaryLanguage: String? = null
    var locationCode: String? = null
    var legalStatus: String? = null
    var countryOfIssuance: String? = null

    // Biometrics
    var rightThumb: List<BiometricData>? = null
    var rightPointerFinger: List<BiometricData>? = null
    var rightMiddleFinger: List<BiometricData>? = null
    var rightRingFinger: List<BiometricData>? = null
    var rightLittleFinger: List<BiometricData>? = null
    var leftThumb: List<BiometricData>? = null
    var leftPointerFinger: List<BiometricData>? = null
    var leftMiddleFinger: List<BiometricData>? = null
    var leftRingFinger: List<BiometricData>? = null
    var leftLittleFinger: List<BiometricData>? = null
    var rightIris: List<BiometricData>? = null
    var leftIris: List<BiometricData>? = null
    var face: List<BiometricData>? = null
    var rightPalm: List<BiometricData>? = null
    var leftPalm: List<BiometricData>? = null
    var voice: List<BiometricData>? = null

    var unknownFieldsJson: String? = null

    internal fun build(): Claim169Data = Claim169Data(
        id = id,
        version = version,
        language = language,
        fullName = fullName,
        firstName = firstName,
        middleName = middleName,
        lastName = lastName,
        dateOfBirth = dateOfBirth,
        gender = gender,
        address = address,
        email = email,
        phone = phone,
        nationality = nationality,
        maritalStatus = maritalStatus,
        guardian = guardian,
        photo = photo,
        photoFormat = photoFormat,
        bestQualityFingers = bestQualityFingers,
        secondaryFullName = secondaryFullName,
        secondaryLanguage = secondaryLanguage,
        locationCode = locationCode,
        legalStatus = legalStatus,
        countryOfIssuance = countryOfIssuance,
        rightThumb = rightThumb,
        rightPointerFinger = rightPointerFinger,
        rightMiddleFinger = rightMiddleFinger,
        rightRingFinger = rightRingFinger,
        rightLittleFinger = rightLittleFinger,
        leftThumb = leftThumb,
        leftPointerFinger = leftPointerFinger,
        leftMiddleFinger = leftMiddleFinger,
        leftRingFinger = leftRingFinger,
        leftLittleFinger = leftLittleFinger,
        rightIris = rightIris,
        leftIris = leftIris,
        face = face,
        rightPalm = rightPalm,
        leftPalm = leftPalm,
        voice = voice,
        unknownFieldsJson = unknownFieldsJson,
    )
}

/**
 * DSL builder for creating [CwtMetaData] instances.
 *
 * ## Usage
 * ```kotlin
 * val meta = cwtMeta {
 *     issuer = "https://issuer.example.com"
 *     expiresAt = 1800000000L
 * }
 * ```
 */
class CwtMetaDataBuilder {
    var issuer: String? = null
    var subject: String? = null
    var expiresAt: Long? = null
    var notBefore: Long? = null
    var issuedAt: Long? = null

    internal fun build(): CwtMetaData = CwtMetaData(
        issuer = issuer,
        subject = subject,
        expiresAt = expiresAt,
        notBefore = notBefore,
        issuedAt = issuedAt,
    )
}

/**
 * Create a [Claim169Data] using DSL syntax.
 */
fun claim169(configure: Claim169DataBuilder.() -> Unit): Claim169Data {
    val builder = Claim169DataBuilder()
    builder.configure()
    return builder.build()
}

/**
 * Create a [CwtMetaData] using DSL syntax.
 */
fun cwtMeta(configure: CwtMetaDataBuilder.() -> Unit): CwtMetaData {
    val builder = CwtMetaDataBuilder()
    builder.configure()
    return builder.build()
}
