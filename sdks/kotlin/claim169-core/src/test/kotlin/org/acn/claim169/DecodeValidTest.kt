package org.acn.claim169

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*
import uniffi.claim169_jni.Claim169Exception
import java.util.Base64

/**
 * Tests decoding valid test vectors.
 */
class DecodeValidTest {

    companion object {
        /** DER prefix for Ed25519 SPKI (RFC 8410): OID 1.3.101.112 */
        private val ED25519_SPKI_PREFIX = byteArrayOf(
            0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65,
            0x70, 0x03, 0x21, 0x00
        )

        /** DER prefix for ECDSA P-256 SPKI: OID 1.2.840.10045.2.1 + 1.2.840.10045.3.1.7 */
        private val P256_SPKI_PREFIX = byteArrayOf(
            0x30, 0x59, 0x30, 0x13, 0x06, 0x07, 0x2a, 0x86.toByte(),
            0x48, 0xce.toByte(), 0x3d, 0x02, 0x01, 0x06, 0x08, 0x2a,
            0x86.toByte(), 0x48, 0xce.toByte(), 0x3d, 0x03, 0x01, 0x07,
            0x03, 0x42, 0x00
        )

        fun ed25519PublicKeyToPem(rawKey: ByteArray): String {
            val der = ED25519_SPKI_PREFIX + rawKey
            val b64 = Base64.getMimeEncoder(64, "\n".toByteArray()).encodeToString(der)
            return "-----BEGIN PUBLIC KEY-----\n$b64\n-----END PUBLIC KEY-----"
        }

        fun p256PublicKeyToPem(rawKey: ByteArray): String {
            val der = P256_SPKI_PREFIX + rawKey
            val b64 = Base64.getMimeEncoder(64, "\n".toByteArray()).encodeToString(der)
            return "-----BEGIN PUBLIC KEY-----\n$b64\n-----END PUBLIC KEY-----"
        }
    }

    @Test
    fun `decode minimal unverified`() {
        val vector = TestVectorLoader.loadVector("valid", "minimal")
        val qrData = vector.get("qr_data").asString
        val expected = vector.getAsJsonObject("expected_claim169")
        val expectedMeta = vector.getAsJsonObject("expected_cwt_meta")

        val result = Claim169.decode(qrData) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("ID-12345-ABCDE", result.claim169.id)
        assertEquals("John Doe", result.claim169.fullName)
        assertEquals(expected.get("id").asString, result.claim169.id)
        assertEquals(expected.get("fullName").asString, result.claim169.fullName)

        assertEquals(expectedMeta.get("issuer").asString, result.cwtMeta.issuer)
        assertEquals(expectedMeta.get("expiresAt").asLong, result.cwtMeta.expiresAt)
        assertEquals(expectedMeta.get("issuedAt").asLong, result.cwtMeta.issuedAt)

        assertEquals("skipped", result.verificationStatus)
    }

    @Test
    fun `decode with ed25519 verification`() {
        val vector = TestVectorLoader.loadVector("valid", "ed25519-signed")
        val qrData = vector.get("qr_data").asString
        val signingKey = vector.getAsJsonObject("signing_key")
        val publicKeyHex = signingKey.get("public_key_hex").asString
        val publicKey = TestVectorLoader.hexToByteArray(publicKeyHex)

        val result = Claim169.decode(qrData) {
            verifyWithEd25519(publicKey)
            withoutTimestampValidation()
        }

        assertEquals("ID-SIGNED-001", result.claim169.id)
        assertEquals("Signed Test Person", result.claim169.fullName)
        assertEquals("verified", result.verificationStatus)
    }

    @Test
    fun `decode with ecdsa p256 verification`() {
        val vector = TestVectorLoader.loadVector("valid", "ecdsa-p256-signed")
        val qrData = vector.get("qr_data").asString
        val signingKey = vector.getAsJsonObject("signing_key")
        val publicKeyHex = signingKey.get("public_key_hex").asString
        val publicKey = TestVectorLoader.hexToByteArray(publicKeyHex)

        val result = Claim169.decode(qrData) {
            verifyWithEcdsaP256(publicKey)
            withoutTimestampValidation()
        }

        assertEquals("ID-ECDSA-001", result.claim169.id)
        assertEquals("ECDSA Test Person", result.claim169.fullName)
        assertEquals("verified", result.verificationStatus)
    }

    @Test
    fun `decode encrypted aes256`() {
        val vector = TestVectorLoader.loadVector("valid", "encrypted-aes256")
        val qrData = vector.get("qr_data").asString
        val encKey = vector.getAsJsonObject("encryption_key")
        val keyHex = encKey.get("symmetric_key_hex").asString
        val key = TestVectorLoader.hexToByteArray(keyHex)

        val result = Claim169.decode(qrData) {
            decryptWithAes256(key)
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("ID-ENCRYPTED-001", result.claim169.id)
        assertEquals("Encrypted Test Person", result.claim169.fullName)
    }

    @Test
    fun `decode encrypted and signed`() {
        val vector = TestVectorLoader.loadVector("valid", "encrypted-signed")
        val qrData = vector.get("qr_data").asString
        val signingKey = vector.getAsJsonObject("signing_key")
        val publicKeyHex = signingKey.get("public_key_hex").asString
        val publicKey = TestVectorLoader.hexToByteArray(publicKeyHex)
        val encKey = vector.getAsJsonObject("encryption_key")
        val keyHex = encKey.get("symmetric_key_hex").asString
        val key = TestVectorLoader.hexToByteArray(keyHex)

        val result = Claim169.decode(qrData) {
            decryptWithAes256(key)
            verifyWithEd25519(publicKey)
            withoutTimestampValidation()
        }

        assertEquals("ID-ENC-SIGN-001", result.claim169.id)
        assertEquals("Encrypted Signed Test Person", result.claim169.fullName)
        assertEquals("verified", result.verificationStatus)
    }

    @Test
    fun `decode demographics full`() {
        val vector = TestVectorLoader.loadVector("valid", "demographics-full")
        val qrData = vector.get("qr_data").asString

        val result = Claim169.decode(qrData) {
            allowUnverified()
            withoutTimestampValidation()
        }

        val claim = result.claim169
        assertEquals("ID-67890-FGHIJ", claim.id)
        assertEquals("1.0", claim.version)
        assertEquals("eng", claim.language)
        assertEquals("Jane Marie Smith", claim.fullName)
        assertEquals("Jane", claim.firstName)
        assertEquals("Marie", claim.middleName)
        assertEquals("Smith", claim.lastName)
        assertEquals("19900515", claim.dateOfBirth)
        assertEquals(Gender.Female.value, claim.gender)
        assertEquals("123 Main St\nApt 4\nNew York, NY 10001", claim.address)
        assertEquals("jane.smith@example.com", claim.email)
        assertEquals("+1 555 123 4567", claim.phone)
        assertEquals("USA", claim.nationality)
        assertEquals(MaritalStatus.Married.value, claim.maritalStatus)
        assertEquals("Guardian-ID-001", claim.guardian)
        assertEquals(PhotoFormat.Jpeg.value, claim.photoFormat)
        assertEquals("Jane Marie Smith (Hindi)", claim.secondaryFullName)
        assertEquals("hin", claim.secondaryLanguage)
        assertEquals("US-NY-NYC", claim.locationCode)
        assertEquals("citizen", claim.legalStatus)
        assertEquals("USA", claim.countryOfIssuance)

        // bestQualityFingers should be [1, 2, 6]
        assertNotNull(claim.bestQualityFingers)
        val fingers = claim.bestQualityFingers!!
        assertEquals(3, fingers.size)
        assertEquals(1.toByte(), fingers[0])
        assertEquals(2.toByte(), fingers[1])
        assertEquals(6.toByte(), fingers[2])

        // CWT meta
        val meta = result.cwtMeta
        assertEquals("https://mosip.example.org", meta.issuer)
        assertEquals("ID-67890-FGHIJ", meta.subject)
        assertEquals(1800000000L, meta.expiresAt)
        assertEquals(1700000000L, meta.notBefore)
        assertEquals(1700000000L, meta.issuedAt)
    }

    @Test
    fun `decode with ed25519 pem verification`() {
        val vector = TestVectorLoader.loadVector("valid", "ed25519-signed")
        val qrData = vector.get("qr_data").asString
        val signingKey = vector.getAsJsonObject("signing_key")
        val publicKeyHex = signingKey.get("public_key_hex").asString
        val publicKey = TestVectorLoader.hexToByteArray(publicKeyHex)
        val pem = ed25519PublicKeyToPem(publicKey)

        val result = Claim169.decode(qrData) {
            verifyWithEd25519Pem(pem)
            withoutTimestampValidation()
        }

        assertEquals("ID-SIGNED-001", result.claim169.id)
        assertEquals("Signed Test Person", result.claim169.fullName)
        assertEquals("verified", result.verificationStatus)
    }

    @Test
    fun `decode with ecdsa p256 pem verification`() {
        val vector = TestVectorLoader.loadVector("valid", "ecdsa-p256-signed")
        val qrData = vector.get("qr_data").asString
        val signingKey = vector.getAsJsonObject("signing_key")
        val publicKeyHex = signingKey.get("public_key_hex").asString
        val publicKey = TestVectorLoader.hexToByteArray(publicKeyHex)
        val pem = p256PublicKeyToPem(publicKey)

        val result = Claim169.decode(qrData) {
            verifyWithEcdsaP256Pem(pem)
            withoutTimestampValidation()
        }

        assertEquals("ID-ECDSA-001", result.claim169.id)
        assertEquals("ECDSA Test Person", result.claim169.fullName)
        assertEquals("verified", result.verificationStatus)
    }

    @Test
    fun `decode with wrong key fails verification`() {
        val vector = TestVectorLoader.loadVector("valid", "ed25519-signed")
        val qrData = vector.get("qr_data").asString
        // Use a different (wrong) key
        val wrongKey = ByteArray(32) { 0xFF.toByte() }

        val exception = assertThrows(Claim169Exception.SignatureInvalid::class.java) {
            Claim169.decode(qrData) {
                verifyWithEd25519(wrongKey)
                withoutTimestampValidation()
            }
        }
        assertNotNull(exception.message)
    }
}
