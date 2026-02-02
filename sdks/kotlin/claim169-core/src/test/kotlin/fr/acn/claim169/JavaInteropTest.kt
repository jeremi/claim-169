package fr.acn.claim169

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*
import uniffi.claim169_jni.DecodeResultData

/**
 * Tests for Java interop features: fun interfaces for data builders,
 * @JvmStatic annotations, and static helper methods.
 */
class JavaInteropTest {

    // -- Claim169DataConfigurer fun interface --

    @Test
    fun `claim169With using Claim169DataConfigurer functional interface`() {
        val configurer = Claim169DataConfigurer { builder ->
            builder.id = "CONFIGURER-DATA-001"
            builder.fullName = "Configurer Data Test"
        }

        val data = Claim169.claim169With(configurer)

        assertEquals("CONFIGURER-DATA-001", data.id)
        assertEquals("Configurer Data Test", data.fullName)
    }

    @Test
    fun `claim169With with lambda shorthand`() {
        val data = Claim169.claim169With { builder ->
            builder.id = "LAMBDA-DATA-001"
            builder.fullName = "Lambda Data Test"
            builder.genderEnum = Gender.Female
        }

        assertEquals("LAMBDA-DATA-001", data.id)
        assertEquals("Lambda Data Test", data.fullName)
        assertEquals(Gender.Female.value, data.gender)
    }

    // -- CwtMetaDataConfigurer fun interface --

    @Test
    fun `cwtMetaWith using CwtMetaDataConfigurer functional interface`() {
        val configurer = CwtMetaDataConfigurer { builder ->
            builder.issuer = "https://test.example.com"
            builder.expiresAt = 2000000000L
        }

        val meta = Claim169.cwtMetaWith(configurer)

        assertEquals("https://test.example.com", meta.issuer)
        assertEquals(2000000000L, meta.expiresAt)
    }

    @Test
    fun `cwtMetaWith with lambda shorthand`() {
        val meta = Claim169.cwtMetaWith { builder ->
            builder.issuer = "https://lambda.example.com"
            builder.issuedAt = 1700000000L
        }

        assertEquals("https://lambda.example.com", meta.issuer)
        assertEquals(1700000000L, meta.issuedAt)
    }

    // -- build() is public --

    @Test
    fun `Claim169DataBuilder build is callable`() {
        val builder = Claim169DataBuilder()
        builder.id = "BUILD-PUBLIC-001"
        builder.fullName = "Build Public Test"

        val data = builder.build()

        assertEquals("BUILD-PUBLIC-001", data.id)
        assertEquals("Build Public Test", data.fullName)
    }

    @Test
    fun `CwtMetaDataBuilder build is callable`() {
        val builder = CwtMetaDataBuilder()
        builder.issuer = "https://build.example.com"
        builder.expiresAt = 2000000000L

        val meta = builder.build()

        assertEquals("https://build.example.com", meta.issuer)
        assertEquals(2000000000L, meta.expiresAt)
    }

    // -- @JvmStatic on version() --

    @Test
    fun `version is accessible as static method`() {
        val version = Claim169.version()
        assertNotNull(version)
        assertTrue(version.isNotEmpty())
    }

    // -- verificationStatus() static helper --

    @Test
    fun `verificationStatus static helper returns correct enum`() {
        val data = claim169 {
            id = "STATIC-STATUS-001"
            fullName = "Static Status Test"
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        val encoded = Claim169.encode(data, meta) { allowUnsigned() }
        val result = Claim169.decode(encoded) {
            allowUnverified()
            withoutTimestampValidation()
        }

        val status = Claim169.verificationStatus(result)
        assertEquals(VerificationStatus.Skipped, status)
    }

    @Test
    fun `verificationStatus static helper matches extension function`() {
        val data = claim169 {
            id = "MATCH-STATUS-001"
            fullName = "Match Status Test"
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        val encoded = Claim169.encode(data, meta) { allowUnsigned() }
        val result = Claim169.decode(encoded) {
            allowUnverified()
            withoutTimestampValidation()
        }

        // Both approaches should return the same value
        assertEquals(
            result.verificationStatusEnum(),
            Claim169.verificationStatus(result)
        )
    }

    // -- @JvmStatic on enum fromValue() --

    @Test
    fun `Gender fromValue is accessible without Companion`() {
        // This test verifies @JvmStatic is present.
        // In Kotlin, both Gender.fromValue() and Gender.Companion.fromValue() work,
        // but from Java only Gender.fromValue() works with @JvmStatic.
        assertEquals(Gender.Male, Gender.fromValue(1L))
        assertEquals(Gender.Female, Gender.fromValue(2L))
        assertEquals(Gender.Other, Gender.fromValue(3L))
        assertNull(Gender.fromValue(99L))
    }

    @Test
    fun `MaritalStatus fromValue is accessible without Companion`() {
        assertEquals(MaritalStatus.Unmarried, MaritalStatus.fromValue(1L))
        assertEquals(MaritalStatus.Married, MaritalStatus.fromValue(2L))
        assertEquals(MaritalStatus.Divorced, MaritalStatus.fromValue(3L))
        assertNull(MaritalStatus.fromValue(99L))
    }

    @Test
    fun `PhotoFormat fromValue is accessible without Companion`() {
        assertEquals(PhotoFormat.Jpeg, PhotoFormat.fromValue(1L))
        assertEquals(PhotoFormat.Jpeg2000, PhotoFormat.fromValue(2L))
        assertEquals(PhotoFormat.Avif, PhotoFormat.fromValue(3L))
        assertEquals(PhotoFormat.Webp, PhotoFormat.fromValue(4L))
        assertNull(PhotoFormat.fromValue(99L))
    }

    @Test
    fun `VerificationStatus fromValue is accessible without Companion`() {
        assertEquals(VerificationStatus.Verified, VerificationStatus.fromValue("verified"))
        assertEquals(VerificationStatus.Failed, VerificationStatus.fromValue("failed"))
        assertEquals(VerificationStatus.Skipped, VerificationStatus.fromValue("skipped"))
        assertEquals(VerificationStatus.Unknown, VerificationStatus.fromValue("unknown"))
        assertEquals(VerificationStatus.Unknown, VerificationStatus.fromValue("something_else"))
    }

    // -- Roundtrip with configurers --

    @Test
    fun `roundtrip using configurer-based builders`() {
        val data = Claim169.claim169With { builder ->
            builder.id = "ROUNDTRIP-CONFIGURER-001"
            builder.fullName = "Roundtrip Configurer Test"
            builder.genderEnum = Gender.Male
        }
        val meta = Claim169.cwtMetaWith { builder ->
            builder.issuer = "https://test.example.com"
            builder.expiresAt = 2000000000L
        }

        val encoded = Claim169.encodeWith(data, meta, EncoderConfigurer { builder ->
            builder.allowUnsigned()
        })

        val result = Claim169.decodeWith(encoded, DecoderConfigurer { builder ->
            builder.allowUnverified()
            builder.withoutTimestampValidation()
        })

        assertEquals("ROUNDTRIP-CONFIGURER-001", result.claim169.id)
        assertEquals("Roundtrip Configurer Test", result.claim169.fullName)
        assertEquals(Gender.Male.value, result.claim169.gender)
    }
}
