package fr.acn.claim169

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*
import uniffi.claim169_jni.BiometricData

/**
 * Tests edge-case data handling: unicode, special characters, large values, empty fields.
 */
class EdgeCaseDataTest {

    @Test
    fun `roundtrip with unicode characters in fields`() {
        val data = claim169 {
            id = "UNICODE-001"
            fullName = "\u0416\u0430\u043d\u0430\u0440\u0434\u0430\u043d \u0411\u0421" // Cyrillic
            firstName = "\u5f20" // Chinese
            lastName = "\u30e4\u30de\u30c0" // Japanese Katakana
            address = "\u0645\u0646\u0632\u0644 \u062c\u062f\u064a\u062f" // Arabic
            secondaryFullName = "\u099c\u09a8\u09be\u09b0\u09cd\u09a6\u09a8" // Bengali
            secondaryLanguage = "bn"
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        val encoded = Claim169.encode(data, meta) {
            allowUnsigned()
        }

        val result = Claim169.decode(encoded) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("UNICODE-001", result.claim169.id)
        assertEquals("\u0416\u0430\u043d\u0430\u0440\u0434\u0430\u043d \u0411\u0421", result.claim169.fullName)
        assertEquals("\u5f20", result.claim169.firstName)
        assertEquals("\u30e4\u30de\u30c0", result.claim169.lastName)
        assertEquals("\u0645\u0646\u0632\u0644 \u062c\u062f\u064a\u062f", result.claim169.address)
        assertEquals("\u099c\u09a8\u09be\u09b0\u09cd\u09a6\u09a8", result.claim169.secondaryFullName)
    }

    @Test
    fun `roundtrip with special characters in fields`() {
        val data = claim169 {
            id = "SPECIAL-001"
            fullName = "O'Brien-Smith & Co. <test>"
            address = "123 Main St.\nApt #4\n\"Penthouse\""
            email = "test+special@example.co.uk"
            phone = "+1 (555) 123-4567"
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        val encoded = Claim169.encode(data, meta) {
            allowUnsigned()
        }

        val result = Claim169.decode(encoded) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("SPECIAL-001", result.claim169.id)
        assertEquals("O'Brien-Smith & Co. <test>", result.claim169.fullName)
        assertEquals("123 Main St.\nApt #4\n\"Penthouse\"", result.claim169.address)
        assertEquals("test+special@example.co.uk", result.claim169.email)
        assertEquals("+1 (555) 123-4567", result.claim169.phone)
    }

    @Test
    fun `roundtrip with all optional fields null`() {
        val data = claim169 {
            // Only set required fields
            id = "NULL-001"
            fullName = "Null Fields Test"
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        val encoded = Claim169.encode(data, meta) {
            allowUnsigned()
        }

        val result = Claim169.decode(encoded) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("NULL-001", result.claim169.id)
        assertEquals("Null Fields Test", result.claim169.fullName)
        assertNull(result.claim169.version)
        assertNull(result.claim169.language)
        assertNull(result.claim169.firstName)
        assertNull(result.claim169.middleName)
        assertNull(result.claim169.lastName)
        assertNull(result.claim169.dateOfBirth)
        assertNull(result.claim169.gender)
        assertNull(result.claim169.address)
        assertNull(result.claim169.email)
        assertNull(result.claim169.phone)
        assertNull(result.claim169.nationality)
        assertNull(result.claim169.maritalStatus)
        assertNull(result.claim169.guardian)
        assertNull(result.claim169.photo)
        assertNull(result.claim169.photoFormat)
        assertNull(result.claim169.bestQualityFingers)
        assertNull(result.claim169.face)
        assertNull(result.claim169.rightThumb)
        assertNull(result.claim169.leftIris)
        assertNull(result.claim169.voice)
    }

    @Test
    fun `roundtrip with photo and photoFormat`() {
        val photoData = byteArrayOf(0xFF.toByte(), 0xD8.toByte(), 0xFF.toByte(), 0xE0.toByte()) // JPEG magic
        val data = claim169 {
            id = "PHOTO-001"
            fullName = "Photo Test"
            photo = photoData
            photoFormatEnum = PhotoFormat.Jpeg
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        val encoded = Claim169.encode(data, meta) {
            allowUnsigned()
        }

        val result = Claim169.decode(encoded) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("PHOTO-001", result.claim169.id)
        assertNotNull(result.claim169.photo)
        assertArrayEquals(photoData, result.claim169.photo)
        assertEquals(PhotoFormat.Jpeg.value, result.claim169.photoFormat)
    }

    @Test
    fun `roundtrip with bestQualityFingers`() {
        val data = claim169 {
            id = "BQF-001"
            fullName = "Best Quality Fingers Test"
            bestQualityFingers = byteArrayOf(1, 2, 6)
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        val encoded = Claim169.encode(data, meta) {
            allowUnsigned()
        }

        val result = Claim169.decode(encoded) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("BQF-001", result.claim169.id)
        assertNotNull(result.claim169.bestQualityFingers)
        val fingers = result.claim169.bestQualityFingers!!
        assertEquals(3, fingers.size)
        assertEquals(1.toByte(), fingers[0])
        assertEquals(2.toByte(), fingers[1])
        assertEquals(6.toByte(), fingers[2])
    }

    @Test
    fun `roundtrip preserves CWT meta subject and notBefore`() {
        val data = claim169 {
            id = "META-001"
            fullName = "Meta Fields Test"
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            subject = "subject-id-123"
            expiresAt = 2000000000L
            notBefore = 1700000000L
            issuedAt = 1700000000L
        }

        val encoded = Claim169.encode(data, meta) {
            allowUnsigned()
        }

        val result = Claim169.decode(encoded) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("https://test.example.com", result.cwtMeta.issuer)
        assertEquals("subject-id-123", result.cwtMeta.subject)
        assertEquals(2000000000L, result.cwtMeta.expiresAt)
        assertEquals(1700000000L, result.cwtMeta.notBefore)
        assertEquals(1700000000L, result.cwtMeta.issuedAt)
    }

    @Test
    fun `warnings field is populated for unknown fields`() {
        val vector = TestVectorLoader.loadVector("edge", "unknown-fields")
        val qrData = vector.get("qr_data").asString

        val result = Claim169.decode(qrData) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("ID-UNKNOWN-001", result.claim169.id)
        // unknownFieldsJson should be populated
        assertNotNull(result.claim169.unknownFieldsJson)
        assertTrue(result.claim169.unknownFieldsJson!!.isNotEmpty())
    }
}
