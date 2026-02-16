@file:JvmName("Claim169Builders")

package fr.acn.claim169

import uniffi.claim169_jni.Claim169Data as NativeClaim169Data

/**
 * DSL marker for Claim 169 builder scopes.
 *
 * Prevents accidental access to outer builder receivers from nested DSL blocks.
 */
@DslMarker
@Target(AnnotationTarget.CLASS)
annotation class Claim169Dsl

/**
 * Java-friendly functional interface for configuring a [Claim169DataBuilder].
 *
 * From Java: `Claim169.claim169(b -> { b.setId("X"); b.setFullName("Y"); })`
 */
fun interface Claim169DataConfigurer {
    fun configure(builder: Claim169DataBuilder)
}

/**
 * Java-friendly functional interface for configuring a [CwtMetaDataBuilder].
 *
 * From Java: `Claim169.cwtMeta(b -> { b.setIssuer("https://..."); })`
 */
fun interface CwtMetaDataConfigurer {
    fun configure(builder: CwtMetaDataBuilder)
}

/**
 * DSL builder for creating [Claim169Data] instances.
 *
 * ## Usage
 * ```kotlin
 * val data = claim169Data {
 *     id = "ID-12345"
 *     fullName = "Jane Doe"
 *     dateOfBirth = "19900115"
 *     genderEnum = Gender.Female
 *     email = "jane@example.com"
 * }
 * ```
 */
@Claim169Dsl
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
    var genderEnum: Gender?
        get() = gender?.let(Gender::fromValue)
        set(value) {
            gender = value?.value
        }
    var address: String? = null
    var email: String? = null
    var phone: String? = null
    var nationality: String? = null
    var maritalStatus: Long? = null
    var maritalStatusEnum: MaritalStatus?
        get() = maritalStatus?.let(MaritalStatus::fromValue)
        set(value) {
            maritalStatus = value?.value
        }
    var guardian: String? = null
    var photo: ByteArray? = null
    var photoFormat: Long? = null
    var photoFormatEnum: PhotoFormat?
        get() = photoFormat?.let(PhotoFormat::fromValue)
        set(value) {
            photoFormat = value?.value
        }
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

    /**
     * JSON-encoded map of unknown CBOR fields for forward compatibility.
     * Must be valid JSON (e.g., `{"100":"value"}`). Malformed JSON will cause
     * [Claim169Exception.Claim169Invalid] when encoding.
     */
    var unknownFieldsJson: String? = null

    fun build(): Claim169Data = Claim169Data.fromNative(
        NativeClaim169Data(
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
            rightThumb = rightThumb.toNativeBiometrics(),
            rightPointerFinger = rightPointerFinger.toNativeBiometrics(),
            rightMiddleFinger = rightMiddleFinger.toNativeBiometrics(),
            rightRingFinger = rightRingFinger.toNativeBiometrics(),
            rightLittleFinger = rightLittleFinger.toNativeBiometrics(),
            leftThumb = leftThumb.toNativeBiometrics(),
            leftPointerFinger = leftPointerFinger.toNativeBiometrics(),
            leftMiddleFinger = leftMiddleFinger.toNativeBiometrics(),
            leftRingFinger = leftRingFinger.toNativeBiometrics(),
            leftLittleFinger = leftLittleFinger.toNativeBiometrics(),
            rightIris = rightIris.toNativeBiometrics(),
            leftIris = leftIris.toNativeBiometrics(),
            face = face.toNativeBiometrics(),
            rightPalm = rightPalm.toNativeBiometrics(),
            leftPalm = leftPalm.toNativeBiometrics(),
            voice = voice.toNativeBiometrics(),
            unknownFieldsJson = unknownFieldsJson,
        )
    )
}

/**
 * DSL builder for creating [CwtMetaData] instances.
 *
 * ## Usage
 * ```kotlin
 * val meta = cwtMetaData {
 *     issuer = "https://issuer.example.com"
 *     expiresAt = 1800000000L
 * }
 * ```
 */
@Claim169Dsl
class CwtMetaDataBuilder {
    var issuer: String? = null
    var subject: String? = null
    var expiresAt: Long? = null
    var notBefore: Long? = null
    var issuedAt: Long? = null

    fun build(): CwtMetaData = CwtMetaData(
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
fun claim169Data(configure: Claim169DataBuilder.() -> Unit): Claim169Data {
    val builder = Claim169DataBuilder()
    builder.configure()
    return builder.build()
}

/**
 * Create a [CwtMetaData] using DSL syntax.
 */
fun cwtMetaData(configure: CwtMetaDataBuilder.() -> Unit): CwtMetaData {
    val builder = CwtMetaDataBuilder()
    builder.configure()
    return builder.build()
}
