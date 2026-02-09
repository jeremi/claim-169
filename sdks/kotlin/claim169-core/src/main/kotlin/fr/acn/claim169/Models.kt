package fr.acn.claim169

import uniffi.claim169_jni.BiometricData as NativeBiometricData
import uniffi.claim169_jni.CertificateHashData as NativeCertificateHashData
import uniffi.claim169_jni.Claim169Data as NativeClaim169Data
import uniffi.claim169_jni.CwtMetaData as NativeCwtMetaData
import uniffi.claim169_jni.DecodeResultData as NativeDecodeResultData
import uniffi.claim169_jni.WarningData as NativeWarningData
import uniffi.claim169_jni.X509HeadersData as NativeX509HeadersData

/**
 * Wrapper for biometric data that keeps the public API in `fr.acn.claim169`.
 */
class BiometricData private constructor(internal val raw: NativeBiometricData) {
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
 * Wrapper for X.509 certificate hash data.
 */
class CertificateHashData private constructor(internal val raw: NativeCertificateHashData) {
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
 * Wrapper for decode warnings.
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
 * Wrapper for COSE X.509 header data.
 */
class X509HeadersData private constructor(internal val raw: NativeX509HeadersData) {
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
 * Claim 169 identity data wrapper.
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
 * CWT (CBOR Web Token) metadata wrapper.
 */
class CwtMetaData private constructor(internal val raw: NativeCwtMetaData) {
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
 */
class DecodeResultData private constructor(internal val raw: NativeDecodeResultData) {
    var claim169: Claim169Data
        get() = Claim169Data.fromNative(raw.claim169)
        set(value) {
            raw.claim169 = value.raw
        }

    var cwtMeta: CwtMetaData
        get() = CwtMetaData.fromNative(raw.cwtMeta)
        set(value) {
            raw.cwtMeta = value.raw
        }

    var verificationStatus: String
        get() = raw.verificationStatus
        set(value) {
            raw.verificationStatus = value
        }

    var x509Headers: X509HeadersData
        get() = X509HeadersData.fromNative(raw.x509Headers)
        set(value) {
            raw.x509Headers = value.raw
        }

    var warnings: List<WarningData>
        get() = raw.warnings.toSdkWarnings()
        set(value) {
            raw.warnings = value.toNativeWarnings()
        }

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
