@file:JvmName("Claim169Enums")

package fr.acn.claim169

enum class Gender(val value: Long) {
    Male(1),
    Female(2),
    Other(3);

    companion object {
        @JvmStatic
        fun fromValue(value: Long): Gender? = entries.firstOrNull { it.value == value }
    }
}

enum class MaritalStatus(val value: Long) {
    Unmarried(1),
    Married(2),
    Divorced(3);

    companion object {
        @JvmStatic
        fun fromValue(value: Long): MaritalStatus? = entries.firstOrNull { it.value == value }
    }
}

enum class PhotoFormat(val value: Long) {
    Jpeg(1),
    Jpeg2000(2),
    Avif(3),
    Webp(4);

    companion object {
        @JvmStatic
        fun fromValue(value: Long): PhotoFormat? = entries.firstOrNull { it.value == value }
    }
}

enum class VerificationStatus(val value: String) {
    Verified("verified"),
    Failed("failed"),
    Skipped("skipped"),
    Unknown("unknown");

    companion object {
        @JvmStatic
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

/**
 * Extension property to get the [Gender] enum from a [Claim169Data]'s numeric gender field.
 */
val Claim169Data.genderEnum: Gender?
    get() = gender?.let(Gender::fromValue)

/**
 * Extension property to get the [MaritalStatus] enum from a [Claim169Data]'s numeric marital status field.
 */
val Claim169Data.maritalStatusEnum: MaritalStatus?
    get() = maritalStatus?.let(MaritalStatus::fromValue)

/**
 * Extension property to get the [PhotoFormat] enum from a [Claim169Data]'s numeric photo format field.
 */
val Claim169Data.photoFormatEnum: PhotoFormat?
    get() = photoFormat?.let(PhotoFormat::fromValue)
