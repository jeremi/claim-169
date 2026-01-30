package fr.acn.claim169

import uniffi.claim169_jni.DecodeResultData

enum class Gender(val value: Long) {
    Male(1),
    Female(2),
    Other(3);

    companion object {
        fun fromValue(value: Long): Gender? = entries.firstOrNull { it.value == value }
    }
}

enum class MaritalStatus(val value: Long) {
    Unmarried(1),
    Married(2),
    Divorced(3);

    companion object {
        fun fromValue(value: Long): MaritalStatus? = entries.firstOrNull { it.value == value }
    }
}

enum class PhotoFormat(val value: Long) {
    Jpeg(1),
    Jpeg2000(2),
    Avif(3),
    Webp(4);

    companion object {
        fun fromValue(value: Long): PhotoFormat? = entries.firstOrNull { it.value == value }
    }
}

enum class VerificationStatus(val value: String) {
    Verified("verified"),
    Failed("failed"),
    Skipped("skipped"),
    Unknown("unknown");

    companion object {
        fun fromValue(value: String): VerificationStatus =
            entries.firstOrNull { it.value == value } ?: Unknown
    }
}

enum class CoseAlgorithm(val coseName: String) {
    EdDSA("EdDSA"),
    ES256("ES256"),
    ES384("ES384"),
    ES512("ES512"),
    A128GCM("A128GCM"),
    A192GCM("A192GCM"),
    A256GCM("A256GCM"),
}

fun DecodeResultData.verificationStatusEnum(): VerificationStatus =
    VerificationStatus.fromValue(verificationStatus)
