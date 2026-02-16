package fr.acn.claim169

import uniffi.claim169_jni.BiometricData as NativeBiometricData
import uniffi.claim169_jni.CertificateHashData as NativeCertificateHashData
import uniffi.claim169_jni.Claim169Data as NativeClaim169Data
import uniffi.claim169_jni.CwtMetaData as NativeCwtMetaData
import uniffi.claim169_jni.DecodeResultData as NativeDecodeResultData
import uniffi.claim169_jni.WarningData as NativeWarningData
import uniffi.claim169_jni.X509HeadersData as NativeX509HeadersData

/**
 * A single biometric data entry (fingerprint, iris, face, palm, or voice).
 *
 * @property data Raw biometric data bytes (template or image).
 * @property format Biometric format: 0=Image, 1=Template, 2=Sound, 3=BioHash. `null` if not specified.
 * @property subFormat Biometric sub-format (interpretation depends on [format]). `null` if not specified.
 * @property issuer Issuer URI of the biometric data. `null` if not specified.
 */
class BiometricData private constructor(internal val raw: NativeBiometricData) {
    @JvmOverloads
    constructor(
        data: ByteArray,
        format: Long? = null,
        subFormat: Long? = null,
        issuer: String? = null
    ) : this(NativeBiometricData(data, format, subFormat, issuer))

    var data: ByteArray
        get() = raw.data
        set(value) {
            raw.data = value
        }

    var format: Long?
        get() = raw.format
        set(value) {
            raw.format = value
        }

    var subFormat: Long?
        get() = raw.subFormat
        set(value) {
            raw.subFormat = value
        }

    var issuer: String?
        get() = raw.issuer
        set(value) {
            raw.issuer = value
        }

    override fun equals(other: Any?): Boolean = other is BiometricData && raw == other.raw
    override fun hashCode(): Int = raw.hashCode()
    override fun toString(): String = raw.toString()

    internal companion object {
        fun fromNative(raw: NativeBiometricData): BiometricData = BiometricData(raw)
    }
}

/**
 * X.509 certificate hash used in COSE `x5t` headers.
 *
 * @property algorithmNumeric Numeric COSE hash algorithm ID (e.g., -16 for SHA-256). `null` if named.
 * @property algorithmName Hash algorithm name (for non-numeric algorithms). `null` if numeric.
 * @property hashValue The certificate hash value bytes.
 */
class CertificateHashData private constructor(internal val raw: NativeCertificateHashData) {
    @JvmOverloads
    constructor(
        algorithmNumeric: Long? = null,
        algorithmName: String? = null,
        hashValue: ByteArray
    ) : this(NativeCertificateHashData(algorithmNumeric, algorithmName, hashValue))

    var algorithmNumeric: Long?
        get() = raw.algorithmNumeric
        set(value) {
            raw.algorithmNumeric = value
        }

    var algorithmName: String?
        get() = raw.algorithmName
        set(value) {
            raw.algorithmName = value
        }

    var hashValue: ByteArray
        get() = raw.hashValue
        set(value) {
            raw.hashValue = value
        }

    override fun equals(other: Any?): Boolean = other is CertificateHashData && raw == other.raw
    override fun hashCode(): Int = raw.hashCode()
    override fun toString(): String = raw.toString()

    internal companion object {
        fun fromNative(raw: NativeCertificateHashData): CertificateHashData = CertificateHashData(raw)
    }
}

/**
 * Warning generated during decoding.
 *
 * @property code Warning code: `"expiring_soon"`, `"unknown_fields"`,
 *   `"timestamp_validation_skipped"`, `"biometrics_skipped"`, or `"non_standard_compression"`.
 * @property message Human-readable warning description.
 */
class WarningData private constructor(internal val raw: NativeWarningData) {
    constructor(code: String, message: String) : this(NativeWarningData(code, message))

    var code: String
        get() = raw.code
        set(value) {
            raw.code = value
        }

    var message: String
        get() = raw.message
        set(value) {
            raw.message = value
        }

    override fun equals(other: Any?): Boolean = other is WarningData && raw == other.raw
    override fun hashCode(): Int = raw.hashCode()
    override fun toString(): String = raw.toString()

    internal companion object {
        fun fromNative(raw: NativeWarningData): WarningData = WarningData(raw)
    }
}

/**
 * X.509 certificate headers extracted from the COSE structure.
 *
 * @property x5bag Unordered bag of DER-encoded X.509 certificates. `null` if not present.
 * @property x5chain Ordered chain of DER-encoded X.509 certificates. `null` if not present.
 * @property x5t Certificate thumbprint hash. `null` if not present.
 * @property x5u URI pointing to an X.509 certificate. `null` if not present.
 */
class X509HeadersData private constructor(internal val raw: NativeX509HeadersData) {
    @JvmOverloads
    constructor(
        x5bag: List<ByteArray>? = null,
        x5chain: List<ByteArray>? = null,
        x5t: CertificateHashData? = null,
        x5u: String? = null
    ) : this(NativeX509HeadersData(x5bag, x5chain, x5t?.raw, x5u))

    var x5bag: List<ByteArray>?
        get() = raw.x5bag
        set(value) {
            raw.x5bag = value
        }

    var x5chain: List<ByteArray>?
        get() = raw.x5chain
        set(value) {
            raw.x5chain = value
        }

    var x5t: CertificateHashData?
        get() = raw.x5t?.let { CertificateHashData.fromNative(it) }
        set(value) {
            raw.x5t = value?.raw
        }

    var x5u: String?
        get() = raw.x5u
        set(value) {
            raw.x5u = value
        }

    override fun equals(other: Any?): Boolean = other is X509HeadersData && raw == other.raw
    override fun hashCode(): Int = raw.hashCode()
    override fun toString(): String = raw.toString()

    internal companion object {
        fun fromNative(raw: NativeX509HeadersData): X509HeadersData = X509HeadersData(raw)
    }
}

/**
 * Identity data from a MOSIP Claim 169 QR code (CBOR keys 1–23 for demographics, 50–65 for biometrics).
 *
 * Demographic fields use CBOR key ranges 1–23. Enum-valued fields ([gender], [maritalStatus],
 * [photoFormat]) store raw integer values matching the spec; use the extension properties
 * [genderEnum], [maritalStatusEnum], and [photoFormatEnum] for typed access.
 *
 * @property id Unique credential identifier (CBOR key 1).
 * @property version Specification version string (CBOR key 2).
 * @property language Primary language code, e.g. `"en"` (CBOR key 3).
 * @property fullName Full name of the credential holder (CBOR key 4).
 * @property firstName First/given name (CBOR key 5).
 * @property middleName Middle name (CBOR key 6).
 * @property lastName Last/family name (CBOR key 7).
 * @property dateOfBirth Date of birth as `"YYYYMMDD"` or `"YYYY-MM-DD"` (CBOR key 8).
 * @property gender Gender as a 1-indexed integer: 1=Male, 2=Female, 3=Other (CBOR key 9).
 *   Use [genderEnum] for typed access.
 * @property address Full postal address (CBOR key 10).
 * @property email Email address (CBOR key 11).
 * @property phone Phone number (CBOR key 12).
 * @property nationality ISO 3166-1 alpha-2 country code (CBOR key 13).
 * @property maritalStatus Marital status as a 1-indexed integer: 1=Unmarried, 2=Married, 3=Divorced (CBOR key 14).
 *   Use [maritalStatusEnum] for typed access.
 * @property guardian Guardian name (CBOR key 15).
 * @property photo Photo bytes (CBOR key 16). Format indicated by [photoFormat].
 * @property photoFormat Photo format as a 1-indexed integer: 1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP (CBOR key 17).
 *   Use [photoFormatEnum] for typed access.
 * @property bestQualityFingers Ordered list of best-quality finger positions, values 0–10 (CBOR key 18).
 * @property secondaryFullName Secondary full name in alternate language (CBOR key 19).
 * @property secondaryLanguage Secondary language code (CBOR key 20).
 * @property locationCode Location code (CBOR key 21).
 * @property legalStatus Legal status string (CBOR key 22).
 * @property countryOfIssuance ISO 3166-1 alpha-2 country of issuance (CBOR key 23).
 * @property unknownFieldsJson JSON-encoded map of unrecognized CBOR keys, preserved for forward compatibility.
 */
class Claim169Data private constructor(internal val raw: NativeClaim169Data) {
    var id: String?
        get() = raw.id
        set(value) {
            raw.id = value
        }

    var version: String?
        get() = raw.version
        set(value) {
            raw.version = value
        }

    var language: String?
        get() = raw.language
        set(value) {
            raw.language = value
        }

    var fullName: String?
        get() = raw.fullName
        set(value) {
            raw.fullName = value
        }

    var firstName: String?
        get() = raw.firstName
        set(value) {
            raw.firstName = value
        }

    var middleName: String?
        get() = raw.middleName
        set(value) {
            raw.middleName = value
        }

    var lastName: String?
        get() = raw.lastName
        set(value) {
            raw.lastName = value
        }

    var dateOfBirth: String?
        get() = raw.dateOfBirth
        set(value) {
            raw.dateOfBirth = value
        }

    var gender: Long?
        get() = raw.gender
        set(value) {
            raw.gender = value
        }

    var address: String?
        get() = raw.address
        set(value) {
            raw.address = value
        }

    var email: String?
        get() = raw.email
        set(value) {
            raw.email = value
        }

    var phone: String?
        get() = raw.phone
        set(value) {
            raw.phone = value
        }

    var nationality: String?
        get() = raw.nationality
        set(value) {
            raw.nationality = value
        }

    var maritalStatus: Long?
        get() = raw.maritalStatus
        set(value) {
            raw.maritalStatus = value
        }

    var guardian: String?
        get() = raw.guardian
        set(value) {
            raw.guardian = value
        }

    var photo: ByteArray?
        get() = raw.photo
        set(value) {
            raw.photo = value
        }

    var photoFormat: Long?
        get() = raw.photoFormat
        set(value) {
            raw.photoFormat = value
        }

    var bestQualityFingers: ByteArray?
        get() = raw.bestQualityFingers
        set(value) {
            raw.bestQualityFingers = value
        }

    var secondaryFullName: String?
        get() = raw.secondaryFullName
        set(value) {
            raw.secondaryFullName = value
        }

    var secondaryLanguage: String?
        get() = raw.secondaryLanguage
        set(value) {
            raw.secondaryLanguage = value
        }

    var locationCode: String?
        get() = raw.locationCode
        set(value) {
            raw.locationCode = value
        }

    var legalStatus: String?
        get() = raw.legalStatus
        set(value) {
            raw.legalStatus = value
        }

    var countryOfIssuance: String?
        get() = raw.countryOfIssuance
        set(value) {
            raw.countryOfIssuance = value
        }

    var rightThumb: List<BiometricData>?
        get() = raw.rightThumb.toSdkBiometrics()
        set(value) {
            raw.rightThumb = value.toNativeBiometrics()
        }

    var rightPointerFinger: List<BiometricData>?
        get() = raw.rightPointerFinger.toSdkBiometrics()
        set(value) {
            raw.rightPointerFinger = value.toNativeBiometrics()
        }

    var rightMiddleFinger: List<BiometricData>?
        get() = raw.rightMiddleFinger.toSdkBiometrics()
        set(value) {
            raw.rightMiddleFinger = value.toNativeBiometrics()
        }

    var rightRingFinger: List<BiometricData>?
        get() = raw.rightRingFinger.toSdkBiometrics()
        set(value) {
            raw.rightRingFinger = value.toNativeBiometrics()
        }

    var rightLittleFinger: List<BiometricData>?
        get() = raw.rightLittleFinger.toSdkBiometrics()
        set(value) {
            raw.rightLittleFinger = value.toNativeBiometrics()
        }

    var leftThumb: List<BiometricData>?
        get() = raw.leftThumb.toSdkBiometrics()
        set(value) {
            raw.leftThumb = value.toNativeBiometrics()
        }

    var leftPointerFinger: List<BiometricData>?
        get() = raw.leftPointerFinger.toSdkBiometrics()
        set(value) {
            raw.leftPointerFinger = value.toNativeBiometrics()
        }

    var leftMiddleFinger: List<BiometricData>?
        get() = raw.leftMiddleFinger.toSdkBiometrics()
        set(value) {
            raw.leftMiddleFinger = value.toNativeBiometrics()
        }

    var leftRingFinger: List<BiometricData>?
        get() = raw.leftRingFinger.toSdkBiometrics()
        set(value) {
            raw.leftRingFinger = value.toNativeBiometrics()
        }

    var leftLittleFinger: List<BiometricData>?
        get() = raw.leftLittleFinger.toSdkBiometrics()
        set(value) {
            raw.leftLittleFinger = value.toNativeBiometrics()
        }

    var rightIris: List<BiometricData>?
        get() = raw.rightIris.toSdkBiometrics()
        set(value) {
            raw.rightIris = value.toNativeBiometrics()
        }

    var leftIris: List<BiometricData>?
        get() = raw.leftIris.toSdkBiometrics()
        set(value) {
            raw.leftIris = value.toNativeBiometrics()
        }

    var face: List<BiometricData>?
        get() = raw.face.toSdkBiometrics()
        set(value) {
            raw.face = value.toNativeBiometrics()
        }

    var rightPalm: List<BiometricData>?
        get() = raw.rightPalm.toSdkBiometrics()
        set(value) {
            raw.rightPalm = value.toNativeBiometrics()
        }

    var leftPalm: List<BiometricData>?
        get() = raw.leftPalm.toSdkBiometrics()
        set(value) {
            raw.leftPalm = value.toNativeBiometrics()
        }

    var voice: List<BiometricData>?
        get() = raw.voice.toSdkBiometrics()
        set(value) {
            raw.voice = value.toNativeBiometrics()
        }

    var unknownFieldsJson: String?
        get() = raw.unknownFieldsJson
        set(value) {
            raw.unknownFieldsJson = value
        }

    override fun equals(other: Any?): Boolean = other is Claim169Data && raw == other.raw
    override fun hashCode(): Int = raw.hashCode()
    override fun toString(): String = raw.toString()

    internal companion object {
        fun fromNative(raw: NativeClaim169Data): Claim169Data = Claim169Data(raw)
    }
}

/**
 * CWT (CBOR Web Token) metadata extracted from the COSE payload.
 *
 * Timestamps are Unix epoch seconds (not milliseconds).
 *
 * @property issuer Token issuer URI (CWT claim 1). `null` if not present.
 * @property subject Token subject identifier (CWT claim 2). `null` if not present.
 * @property expiresAt Expiration time as Unix timestamp in seconds (CWT claim 4). `null` if not present.
 * @property notBefore Not-before time as Unix timestamp in seconds (CWT claim 5). `null` if not present.
 * @property issuedAt Issued-at time as Unix timestamp in seconds (CWT claim 6). `null` if not present.
 */
class CwtMetaData private constructor(internal val raw: NativeCwtMetaData) {
    @JvmOverloads
    constructor(
        issuer: String? = null,
        subject: String? = null,
        expiresAt: Long? = null,
        notBefore: Long? = null,
        issuedAt: Long? = null
    ) : this(NativeCwtMetaData(issuer, subject, expiresAt, notBefore, issuedAt))

    var issuer: String?
        get() = raw.issuer
        set(value) {
            raw.issuer = value
        }

    var subject: String?
        get() = raw.subject
        set(value) {
            raw.subject = value
        }

    var expiresAt: Long?
        get() = raw.expiresAt
        set(value) {
            raw.expiresAt = value
        }

    var notBefore: Long?
        get() = raw.notBefore
        set(value) {
            raw.notBefore = value
        }

    var issuedAt: Long?
        get() = raw.issuedAt
        set(value) {
            raw.issuedAt = value
        }

    override fun equals(other: Any?): Boolean = other is CwtMetaData && raw == other.raw
    override fun hashCode(): Int = raw.hashCode()
    override fun toString(): String = raw.toString()

    internal companion object {
        fun fromNative(raw: NativeCwtMetaData): CwtMetaData = CwtMetaData(raw)
    }
}

/**
 * Result of decoding a Claim 169 QR payload.
 *
 * All properties are read-only to prevent accidental mutation of security-sensitive
 * fields such as [verificationStatus].
 *
 * @property claim169 The extracted Claim 169 identity data.
 * @property cwtMeta CWT metadata (issuer, expiration timestamps, etc.).
 * @property verificationStatus Signature verification outcome.
 * @property x509Headers X.509 certificate headers from the COSE structure, if present.
 * @property detectedCompression Detected compression format: `"zlib"`, `"brotli"`, or `"none"`.
 * @property warnings Warnings generated during decoding (e.g., expiring soon, unknown fields).
 */
class DecodeResultData private constructor(internal val raw: NativeDecodeResultData) {
    val claim169: Claim169Data
        get() = Claim169Data.fromNative(raw.claim169)

    val cwtMeta: CwtMetaData
        get() = CwtMetaData.fromNative(raw.cwtMeta)

    val verificationStatus: VerificationStatus
        get() = VerificationStatus.fromValue(raw.verificationStatus)

    val x509Headers: X509HeadersData
        get() = X509HeadersData.fromNative(raw.x509Headers)

    val detectedCompression: String
        get() = raw.detectedCompression

    val warnings: List<WarningData>
        get() = raw.warnings.toSdkWarnings()

    override fun equals(other: Any?): Boolean = other is DecodeResultData && raw == other.raw
    override fun hashCode(): Int = raw.hashCode()
    override fun toString(): String = raw.toString()

    internal companion object {
        fun fromNative(raw: NativeDecodeResultData): DecodeResultData = DecodeResultData(raw)
    }
}

internal fun List<BiometricData>?.toNativeBiometrics(): List<NativeBiometricData>? =
    this?.map { it.raw }

internal fun List<NativeBiometricData>?.toSdkBiometrics(): List<BiometricData>? =
    this?.map { BiometricData.fromNative(it) }

internal fun List<WarningData>.toNativeWarnings(): List<NativeWarningData> =
    map { it.raw }

internal fun List<NativeWarningData>.toSdkWarnings(): List<WarningData> =
    map { WarningData.fromNative(it) }
