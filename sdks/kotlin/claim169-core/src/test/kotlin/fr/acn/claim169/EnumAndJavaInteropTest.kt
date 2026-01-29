package fr.acn.claim169

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*
import uniffi.claim169_jni.Claim169Exception

/**
 * Tests for enum classes, Java interop APIs, and closeable decode.
 */
class EnumAndJavaInteropTest {

    // -- Enum fromValue round-trips --

    @Test
    fun `Gender fromValue round-trips all entries`() {
        for (gender in Gender.entries) {
            assertEquals(gender, Gender.fromValue(gender.value))
        }
        assertNull(Gender.fromValue(0L))
        assertNull(Gender.fromValue(99L))
    }

    @Test
    fun `MaritalStatus fromValue round-trips all entries`() {
        for (status in MaritalStatus.entries) {
            assertEquals(status, MaritalStatus.fromValue(status.value))
        }
        assertNull(MaritalStatus.fromValue(0L))
        assertNull(MaritalStatus.fromValue(99L))
    }

    @Test
    fun `PhotoFormat fromValue round-trips all entries`() {
        for (format in PhotoFormat.entries) {
            assertEquals(format, PhotoFormat.fromValue(format.value))
        }
        assertNull(PhotoFormat.fromValue(0L))
        assertNull(PhotoFormat.fromValue(99L))
    }

    @Test
    fun `VerificationStatus fromValue returns correct entries`() {
        assertEquals(VerificationStatus.Verified, VerificationStatus.fromValue("verified"))
        assertEquals(VerificationStatus.Failed, VerificationStatus.fromValue("failed"))
        assertEquals(VerificationStatus.Skipped, VerificationStatus.fromValue("skipped"))
        assertEquals(VerificationStatus.Unknown, VerificationStatus.fromValue("unknown"))
        assertEquals(VerificationStatus.Unknown, VerificationStatus.fromValue("something_else"))
    }

    @Test
    fun `CoseAlgorithm entries have correct coseName values`() {
        assertEquals("EdDSA", CoseAlgorithm.EdDSA.coseName)
        assertEquals("ES256", CoseAlgorithm.ES256.coseName)
        assertEquals("ES384", CoseAlgorithm.ES384.coseName)
        assertEquals("ES512", CoseAlgorithm.ES512.coseName)
        assertEquals("A128GCM", CoseAlgorithm.A128GCM.coseName)
        assertEquals("A192GCM", CoseAlgorithm.A192GCM.coseName)
        assertEquals("A256GCM", CoseAlgorithm.A256GCM.coseName)
    }

    // -- Enum builder properties --

    @Test
    fun `genderEnum builder property sets and gets correctly`() {
        val data = claim169 {
            id = "ENUM-GENDER-001"
            fullName = "Gender Enum Test"
            genderEnum = Gender.Female
        }
        assertEquals(Gender.Female.value, data.gender)

        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        val encoded = Claim169.encode(data, meta) { allowUnsigned() }
        val result = Claim169.decode(encoded) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals(Gender.Female.value, result.claim169.gender)
    }

    @Test
    fun `maritalStatusEnum builder property sets and gets correctly`() {
        val data = claim169 {
            id = "ENUM-MARITAL-001"
            fullName = "Marital Enum Test"
            maritalStatusEnum = MaritalStatus.Divorced
        }
        assertEquals(MaritalStatus.Divorced.value, data.maritalStatus)

        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        val encoded = Claim169.encode(data, meta) { allowUnsigned() }
        val result = Claim169.decode(encoded) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals(MaritalStatus.Divorced.value, result.claim169.maritalStatus)
    }

    @Test
    fun `photoFormatEnum builder property sets and gets correctly`() {
        val photoData = byteArrayOf(0xFF.toByte(), 0xD8.toByte())
        val data = claim169 {
            id = "ENUM-PHOTO-001"
            fullName = "Photo Enum Test"
            photo = photoData
            photoFormatEnum = PhotoFormat.Webp
        }
        assertEquals(PhotoFormat.Webp.value, data.photoFormat)

        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        val encoded = Claim169.encode(data, meta) { allowUnsigned() }
        val result = Claim169.decode(encoded) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals(PhotoFormat.Webp.value, result.claim169.photoFormat)
    }

    // -- verificationStatusEnum() --

    @Test
    fun `verificationStatusEnum returns Skipped for unverified decode`() {
        val data = claim169 {
            id = "STATUS-001"
            fullName = "Status Test"
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

        assertEquals(VerificationStatus.Skipped, result.verificationStatusEnum())
    }

    @Test
    fun `verificationStatusEnum returns Verified for signed decode`() {
        val vector = TestVectorLoader.loadVector("valid", "ed25519-signed")
        val qrData = vector.get("qr_data").asString
        val signingKey = vector.getAsJsonObject("signing_key")
        val publicKeyHex = signingKey.get("public_key_hex").asString
        val publicKey = TestVectorLoader.hexToByteArray(publicKeyHex)

        val result = Claim169.decode(qrData) {
            verifyWithEd25519(publicKey)
            withoutTimestampValidation()
        }

        assertEquals(VerificationStatus.Verified, result.verificationStatusEnum())
    }

    // -- decodeCloseable --

    @Test
    fun `decodeCloseable returns CloseableDecodeResult that works with use block`() {
        val data = claim169 {
            id = "CLOSEABLE-001"
            fullName = "Closeable Test"
            photo = byteArrayOf(0x01, 0x02, 0x03)
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        val encoded = Claim169.encode(data, meta) { allowUnsigned() }

        var capturedPhoto: ByteArray? = null

        Claim169.decodeCloseable(encoded) {
            allowUnverified()
            withoutTimestampValidation()
        }.use { result ->
            assertEquals("CLOSEABLE-001", result.data.claim169.id)
            assertEquals("Closeable Test", result.data.claim169.fullName)
            assertNotNull(result.data.claim169.photo)
            capturedPhoto = result.data.claim169.photo
        }

        // After close, the photo byte array should be zeroed
        assertNotNull(capturedPhoto)
        assertTrue(capturedPhoto!!.all { it == 0.toByte() },
            "Photo bytes should be zeroed after close")
    }

    // -- DecoderConfigurer (Java interop) --

    @Test
    fun `decodeWith using DecoderConfigurer functional interface`() {
        val data = claim169 {
            id = "CONFIGURER-001"
            fullName = "Configurer Test"
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        val encoded = Claim169.encode(data, meta) { allowUnsigned() }

        val configurer = DecoderConfigurer { builder ->
            builder.allowUnverified()
            builder.withoutTimestampValidation()
        }

        val result = Claim169.decodeWith(encoded, configurer)

        assertEquals("CONFIGURER-001", result.claim169.id)
        assertEquals("Configurer Test", result.claim169.fullName)
        assertEquals("skipped", result.verificationStatus)
    }

    // -- EncoderConfigurer (Java interop) --

    @Test
    fun `encodeWith using EncoderConfigurer functional interface`() {
        val data = claim169 {
            id = "ENC-CONFIGURER-001"
            fullName = "Encoder Configurer Test"
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        val configurer = EncoderConfigurer { builder ->
            builder.allowUnsigned()
        }

        val encoded = Claim169.encodeWith(data, meta, configurer)
        assertNotNull(encoded)
        assertTrue(encoded.isNotEmpty())

        val result = Claim169.decode(encoded) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("ENC-CONFIGURER-001", result.claim169.id)
    }

    // -- signWith CoseAlgorithm overload --

    @Test
    fun `signWith CoseAlgorithm EdDSA overload passes correct algorithm`() {
        var receivedAlgorithm: String? = null

        val signer = object : Signer {
            override fun sign(algorithm: String, keyId: ByteArray?, data: ByteArray): ByteArray {
                receivedAlgorithm = algorithm
                throw UnsupportedOperationException("sign called with algorithm=$algorithm")
            }

            override fun keyId(): ByteArray? = null
        }

        val data = claim169 {
            id = "ALGO-ENUM-001"
            fullName = "CoseAlgorithm Test"
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        // The signer callback throws, which the native code wraps in Claim169Exception
        assertThrows(Exception::class.java) {
            Claim169.encode(data, meta) {
                signWith(signer, CoseAlgorithm.EdDSA)
            }
        }
        assertEquals("EdDSA", receivedAlgorithm,
            "CoseAlgorithm.EdDSA should pass 'EdDSA' to the signer callback")
    }

    // -- encryptWith CoseAlgorithm overload --

    @Test
    fun `encryptWith CoseAlgorithm A256GCM overload passes correct algorithm`() {
        var receivedAlgorithm: String? = null

        val encryptor = object : Encryptor {
            override fun encrypt(
                algorithm: String,
                keyId: ByteArray?,
                nonce: ByteArray,
                aad: ByteArray,
                plaintext: ByteArray
            ): ByteArray {
                receivedAlgorithm = algorithm
                throw UnsupportedOperationException("encrypt called with algorithm=$algorithm")
            }
        }

        val data = claim169 {
            id = "ENC-ALGO-001"
            fullName = "Encrypt CoseAlgorithm Test"
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        assertThrows(Exception::class.java) {
            Claim169.encode(data, meta) {
                allowUnsigned()
                encryptWith(encryptor, CoseAlgorithm.A256GCM)
            }
        }
        assertEquals("A256GCM", receivedAlgorithm,
            "CoseAlgorithm.A256GCM should pass 'A256GCM' to the encryptor callback")
    }
}
