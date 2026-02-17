package fr.acn.claim169

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*

/**
 * Tests for Claim169.inspect() metadata extraction.
 */
class InspectTest {

    @Test
    fun `inspect returns metadata from ed25519 signed credential`() {
        val vector = TestVectorLoader.loadVector("valid", "ed25519-signed")
        val qrData = vector.get("qr_data").asString
        val expectedMeta = vector.getAsJsonObject("expected_cwt_meta")

        val result = Claim169.inspect(qrData)

        assertEquals(expectedMeta.get("issuer").asString, result.issuer)
        assertEquals("EdDSA", result.algorithm)
        assertEquals("Sign1", result.coseType)
    }

    @Test
    fun `inspect returns expiration timestamp`() {
        val vector = TestVectorLoader.loadVector("valid", "ed25519-signed")
        val qrData = vector.get("qr_data").asString
        val expectedMeta = vector.getAsJsonObject("expected_cwt_meta")

        val result = Claim169.inspect(qrData)

        assertEquals(expectedMeta.get("expiresAt").asLong, result.expiresAt)
    }

    @Test
    fun `inspect ecdsa p256 signed credential`() {
        val vector = TestVectorLoader.loadVector("valid", "ecdsa-p256-signed")
        val qrData = vector.get("qr_data").asString
        val expectedMeta = vector.getAsJsonObject("expected_cwt_meta")

        val result = Claim169.inspect(qrData)

        assertEquals(expectedMeta.get("issuer").asString, result.issuer)
        assertEquals("ES256", result.algorithm)
        assertEquals("Sign1", result.coseType)
    }

    @Test
    fun `inspect unsigned credential`() {
        val vector = TestVectorLoader.loadVector("valid", "minimal")
        val qrData = vector.get("qr_data").asString
        val expectedMeta = vector.getAsJsonObject("expected_cwt_meta")

        val result = Claim169.inspect(qrData)

        assertEquals(expectedMeta.get("issuer").asString, result.issuer)
        assertNull(result.keyId)
        assertEquals("Sign1", result.coseType)
    }

    @Test
    fun `inspect returns subject from demographics-full`() {
        val vector = TestVectorLoader.loadVector("valid", "demographics-full")
        val qrData = vector.get("qr_data").asString
        val expectedMeta = vector.getAsJsonObject("expected_cwt_meta")

        val result = Claim169.inspect(qrData)

        if (expectedMeta.has("subject")) {
            assertEquals(expectedMeta.get("subject").asString, result.subject)
        }
    }

    @Test
    fun `inspect encrypted credential returns Encrypt0 type`() {
        val vector = TestVectorLoader.loadVector("valid", "encrypted-aes256")
        val qrData = vector.get("qr_data").asString

        val result = Claim169.inspect(qrData)

        assertEquals("Encrypt0", result.coseType)
    }

    @Test
    fun `inspect encrypted credential has null CWT fields`() {
        val vector = TestVectorLoader.loadVector("valid", "encrypted-aes256")
        val qrData = vector.get("qr_data").asString

        val result = Claim169.inspect(qrData)

        // CWT fields are not accessible for encrypted payloads
        assertNull(result.issuer)
        assertNull(result.subject)
        assertNull(result.expiresAt)
    }

    @Test
    fun `inspect encrypted credential has algorithm`() {
        val vector = TestVectorLoader.loadVector("valid", "encrypted-aes256")
        val qrData = vector.get("qr_data").asString

        val result = Claim169.inspect(qrData)

        assertNotNull(result.algorithm)
    }

    @Test
    fun `inspect throws on invalid base45`() {
        assertThrows(Claim169Exception.Base45Decode::class.java) {
            Claim169.inspect("NOT_VALID_BASE45!!!")
        }
    }

    @Test
    fun `inspect throws on empty string`() {
        assertThrows(Claim169Exception::class.java) {
            Claim169.inspect("")
        }
    }

    @Test
    fun `inspect roundtrip with ed25519 encoding`() {
        val vector = TestVectorLoader.loadVector("valid", "ed25519-signed")
        val signingKey = vector.getAsJsonObject("signing_key")
        val privateKeyHex = signingKey.get("private_key_hex").asString
        val privateKey = TestVectorLoader.hexToByteArray(privateKeyHex)

        val data = claim169Data {
            id = "INSPECT-RT-001"
            fullName = "Inspect Roundtrip Person"
        }
        val meta = cwtMetaData {
            issuer = "https://inspect.roundtrip.org"
            expiresAt = 1900000000L
        }

        val encoded = Claim169.encode(data, meta) {
            signWithEd25519(privateKey)
        }

        val result = Claim169.inspect(encoded)

        assertEquals("https://inspect.roundtrip.org", result.issuer)
        assertEquals("EdDSA", result.algorithm)
        assertEquals("Sign1", result.coseType)
        assertEquals(1900000000L, result.expiresAt)
    }

    @Test
    fun `inspect roundtrip with unsigned encoding`() {
        val data = claim169Data {
            id = "INSPECT-UNSIGNED-001"
            fullName = "Unsigned Inspect Person"
        }
        val meta = cwtMetaData {
            issuer = "https://unsigned.inspect.org"
            expiresAt = 1900000000L
        }

        val encoded = Claim169.encode(data, meta) {
            allowUnsigned()
        }

        val result = Claim169.inspect(encoded)

        assertEquals("https://unsigned.inspect.org", result.issuer)
        assertNull(result.keyId)
        assertEquals("Sign1", result.coseType)
        assertEquals(1900000000L, result.expiresAt)
    }
}
