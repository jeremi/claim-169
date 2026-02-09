package fr.acn.claim169

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*
import java.util.Base64

/**
 * Tests decoding test vectors containing biometric data.
 */
class DecodeBiometricTest {

    @Test
    fun `decode with-face vector and assert face biometric fields`() {
        val vector = TestVectorLoader.loadVector("valid", "with-face")
        val qrData = vector.get("qr_data").asString

        val result = Claim169.decode(qrData) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("ID-FACE-001", result.claim169.id)
        assertEquals("Face Test Person", result.claim169.fullName)

        val faceList = result.claim169.face
        assertNotNull(faceList)
        assertTrue(faceList!!.isNotEmpty(), "Expected at least one face biometric entry")
        val face = faceList[0]
        assertNotNull(face.data)
        assertTrue(face.data.isNotEmpty(), "Face biometric data should not be empty")
        // From test vector: format=0 (Image), subFormat=1
        assertEquals(0L, face.format)
        assertEquals(1L, face.subFormat)
    }

    @Test
    fun `decode with-fingerprints vector and assert fingerprint fields`() {
        val vector = TestVectorLoader.loadVector("valid", "with-fingerprints")
        val qrData = vector.get("qr_data").asString

        val result = Claim169.decode(qrData) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("ID-FINGER-001", result.claim169.id)
        assertEquals("Fingerprint Test Person", result.claim169.fullName)

        // rightThumb
        val rightThumb = result.claim169.rightThumb
        assertNotNull(rightThumb)
        assertTrue(rightThumb!!.isNotEmpty(), "Expected at least one right thumb biometric")
        assertEquals(1L, rightThumb[0].format)  // Template
        assertEquals(1L, rightThumb[0].subFormat)

        // rightPointerFinger
        val rightPointer = result.claim169.rightPointerFinger
        assertNotNull(rightPointer)
        assertTrue(rightPointer!!.isNotEmpty(), "Expected at least one right pointer finger biometric")
        assertEquals(1L, rightPointer[0].format)
        assertEquals(1L, rightPointer[0].subFormat)
    }

    @Test
    fun `decode with-all-biometrics vector`() {
        val vector = TestVectorLoader.loadVector("valid", "with-all-biometrics")
        val qrData = vector.get("qr_data").asString

        val result = Claim169.decode(qrData) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("ID-ALL-BIO-001", result.claim169.id)
        assertEquals("All Biometrics Test Person", result.claim169.fullName)
    }

    @Test
    fun `decode claim169-example vector with full field assertions`() {
        val vector = TestVectorLoader.loadVector("valid", "claim169-example")
        val qrData = vector.get("qr_data").asString

        val result = Claim169.decode(qrData) {
            allowUnverified()
            withoutTimestampValidation()
        }

        val claim = result.claim169
        assertEquals("3918592438", claim.id)
        assertEquals("1.0", claim.version)
        assertEquals("eng", claim.language)
        assertEquals("Janardhan BS", claim.fullName)
        assertEquals("19840118", claim.dateOfBirth)
        assertEquals(Gender.Male.value, claim.gender)
        assertEquals("New House, Near Metro Line, Bengaluru, KA", claim.address)
        assertEquals("janardhan@example.com", claim.email)
        assertEquals("+919876543210", claim.phone)
        assertEquals("IN", claim.nationality)
        assertEquals(MaritalStatus.Married.value, claim.maritalStatus)
        assertEquals("AR", claim.secondaryLanguage)
        assertEquals("849VCWC8+R9", claim.locationCode)
        assertEquals("Refugee", claim.legalStatus)
        assertEquals("IN", claim.countryOfIssuance)

        // Face biometric
        val faceList = claim.face
        assertNotNull(faceList)
        assertTrue(faceList!!.isNotEmpty())
        assertEquals(0L, faceList[0].format)
        assertEquals(4L, faceList[0].subFormat)

        // CWT meta
        val meta = result.cwtMeta
        assertEquals("www.mosip.io", meta.issuer)
        assertEquals(1787912445L, meta.expiresAt)
        assertEquals(1756376445L, meta.notBefore)
        assertEquals(1756376445L, meta.issuedAt)
    }

    @Test
    fun `skipBiometrics nullifies face on decode`() {
        val vector = TestVectorLoader.loadVector("valid", "with-face")
        val qrData = vector.get("qr_data").asString

        val result = Claim169.decode(qrData) {
            allowUnverified()
            withoutTimestampValidation()
            skipBiometrics()
        }

        assertEquals("ID-FACE-001", result.claim169.id)
        assertEquals("Face Test Person", result.claim169.fullName)
        // Biometric fields should be null when skipBiometrics is used
        assertNull(result.claim169.face)
    }

    @Test
    fun `skipBiometrics nullifies fingerprints on decode`() {
        val vector = TestVectorLoader.loadVector("valid", "with-fingerprints")
        val qrData = vector.get("qr_data").asString

        val result = Claim169.decode(qrData) {
            allowUnverified()
            withoutTimestampValidation()
            skipBiometrics()
        }

        assertEquals("ID-FINGER-001", result.claim169.id)
        assertEquals("Fingerprint Test Person", result.claim169.fullName)
        assertNull(result.claim169.rightThumb)
        assertNull(result.claim169.rightPointerFinger)
    }

    @Test
    fun `biometric roundtrip with face data`() {
        val faceData = byteArrayOf(0x89.toByte(), 0x50, 0x4E, 0x47) // PNG magic bytes
        val data = claim169 {
            id = "BIO-FACE-RT-001"
            fullName = "Bio Face Roundtrip"
            face = listOf(
                BiometricData(
                    data = faceData,
                    format = 0L,
                    subFormat = 1L,
                    issuer = "test-issuer"
                )
            )
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

        assertEquals("BIO-FACE-RT-001", result.claim169.id)
        val decodedFace = result.claim169.face
        assertNotNull(decodedFace)
        assertEquals(1, decodedFace!!.size)
        assertArrayEquals(faceData, decodedFace[0].data)
        assertEquals(0L, decodedFace[0].format)
        assertEquals(1L, decodedFace[0].subFormat)
        assertEquals("test-issuer", decodedFace[0].issuer)
    }

    @Test
    fun `biometric roundtrip with fingerprint data`() {
        val fingerData = byteArrayOf(0x46, 0x49, 0x52, 0x00) // "FIR\0"
        val data = claim169 {
            id = "BIO-FINGER-RT-001"
            fullName = "Bio Finger Roundtrip"
            rightThumb = listOf(
                BiometricData(
                    data = fingerData,
                    format = 1L,
                    subFormat = 1L,
                    issuer = null
                )
            )
            rightPointerFinger = listOf(
                BiometricData(
                    data = fingerData,
                    format = 1L,
                    subFormat = 2L,
                    issuer = null
                )
            )
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

        assertEquals("BIO-FINGER-RT-001", result.claim169.id)
        val thumb = result.claim169.rightThumb
        assertNotNull(thumb)
        assertEquals(1, thumb!!.size)
        assertArrayEquals(fingerData, thumb[0].data)
        assertEquals(1L, thumb[0].format)

        val pointer = result.claim169.rightPointerFinger
        assertNotNull(pointer)
        assertEquals(1, pointer!!.size)
        assertEquals(2L, pointer[0].subFormat)
    }
}
