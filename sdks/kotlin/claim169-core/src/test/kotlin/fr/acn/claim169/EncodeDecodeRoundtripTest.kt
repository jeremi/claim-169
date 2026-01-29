package fr.acn.claim169

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*
import uniffi.claim169_jni.Claim169Exception

/**
 * Tests encode → decode roundtrip.
 */
class EncodeDecodeRoundtripTest {

    @Test
    fun `roundtrip with ed25519`() {
        val vector = TestVectorLoader.loadVector("valid", "ed25519-signed")
        val signingKey = vector.getAsJsonObject("signing_key")
        val privateKeyHex = signingKey.get("private_key_hex").asString
        val publicKeyHex = signingKey.get("public_key_hex").asString
        val privateKey = TestVectorLoader.hexToByteArray(privateKeyHex)
        val publicKey = TestVectorLoader.hexToByteArray(publicKeyHex)

        val data = claim169 {
            id = "ROUNDTRIP-001"
            fullName = "Roundtrip Test"
            dateOfBirth = "20000101"
            genderEnum = Gender.Male
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
            issuedAt = 1700000000L
        }

        val encoded = Claim169.encode(data, meta) {
            signWithEd25519(privateKey)
        }

        assertNotNull(encoded)
        assertTrue(encoded.isNotEmpty())

        val result = Claim169.decode(encoded) {
            verifyWithEd25519(publicKey)
            withoutTimestampValidation()
        }

        assertEquals("ROUNDTRIP-001", result.claim169.id)
        assertEquals("Roundtrip Test", result.claim169.fullName)
        assertEquals("20000101", result.claim169.dateOfBirth)
        assertEquals(Gender.Male.value, result.claim169.gender)
        assertEquals("https://test.example.com", result.cwtMeta.issuer)
        assertEquals(2000000000L, result.cwtMeta.expiresAt)
        assertEquals("verified", result.verificationStatus)
    }

    @Test
    fun `roundtrip unsigned`() {
        val data = claim169 {
            id = "UNSIGNED-001"
            fullName = "Unsigned Test"
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

        assertEquals("UNSIGNED-001", result.claim169.id)
        assertEquals("Unsigned Test", result.claim169.fullName)
        assertEquals("skipped", result.verificationStatus)
    }

    @Test
    fun `roundtrip with encryption`() {
        val vector = TestVectorLoader.loadVector("valid", "encrypted-aes256")
        val encKey = vector.getAsJsonObject("encryption_key")
        val keyHex = encKey.get("symmetric_key_hex").asString
        val key = TestVectorLoader.hexToByteArray(keyHex)

        val data = claim169 {
            id = "ENC-ROUNDTRIP-001"
            fullName = "Encrypted Roundtrip"
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        val encoded = Claim169.encode(data, meta) {
            allowUnsigned()
            encryptWithAes256(key)
        }

        val result = Claim169.decode(encoded) {
            decryptWithAes256(key)
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("ENC-ROUNDTRIP-001", result.claim169.id)
        assertEquals("Encrypted Roundtrip", result.claim169.fullName)
    }

    @Test
    fun `roundtrip with aes128 encryption`() {
        // AES-128-GCM uses a 16-byte key
        val key = ByteArray(16) { (it + 1).toByte() }

        val data = claim169 {
            id = "AES128-001"
            fullName = "AES-128 Test"
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        val encoded = Claim169.encode(data, meta) {
            allowUnsigned()
            encryptWithAes128(key)
        }

        val result = Claim169.decode(encoded) {
            decryptWithAes128(key)
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("AES128-001", result.claim169.id)
        assertEquals("AES-128 Test", result.claim169.fullName)
    }

    @Test
    fun `roundtrip with skipBiometrics on encode`() {
        val data = claim169 {
            id = "SKIP-BIO-001"
            fullName = "Skip Bio Test"
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        val encoded = Claim169.encode(data, meta) {
            allowUnsigned()
            skipBiometrics()
        }

        val result = Claim169.decode(encoded) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("SKIP-BIO-001", result.claim169.id)
        assertEquals("Skip Bio Test", result.claim169.fullName)
    }

    @Test
    fun `roundtrip with skipBiometrics on decode`() {
        val data = claim169 {
            id = "SKIP-BIO-DEC-001"
            fullName = "Skip Bio Decode Test"
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
            skipBiometrics()
        }

        assertEquals("SKIP-BIO-DEC-001", result.claim169.id)
        assertEquals("Skip Bio Decode Test", result.claim169.fullName)
    }

    @Test
    fun `roundtrip with all demographic fields`() {
        val data = claim169 {
            id = "FULL-001"
            version = "1.0"
            language = "eng"
            fullName = "Full Demographics Test"
            firstName = "Test"
            middleName = "Middle"
            lastName = "Person"
            dateOfBirth = "19850310"
            genderEnum = Gender.Male
            address = "456 Test St"
            email = "test@example.com"
            phone = "+1234567890"
            nationality = "GBR"
            maritalStatusEnum = MaritalStatus.Unmarried
            guardian = "GUARDIAN-001"
            secondaryFullName = "Secondary Name"
            secondaryLanguage = "fra"
            locationCode = "GB-LDN"
            legalStatus = "resident"
            countryOfIssuance = "GBR"
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            subject = "FULL-001"
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

        val claim = result.claim169
        assertEquals("FULL-001", claim.id)
        assertEquals("1.0", claim.version)
        assertEquals("eng", claim.language)
        assertEquals("Full Demographics Test", claim.fullName)
        assertEquals("Test", claim.firstName)
        assertEquals("Middle", claim.middleName)
        assertEquals("Person", claim.lastName)
        assertEquals("19850310", claim.dateOfBirth)
        assertEquals(Gender.Male.value, claim.gender)
        assertEquals("456 Test St", claim.address)
        assertEquals("test@example.com", claim.email)
        assertEquals("+1234567890", claim.phone)
        assertEquals("GBR", claim.nationality)
        assertEquals(MaritalStatus.Unmarried.value, claim.maritalStatus)
        assertEquals("GUARDIAN-001", claim.guardian)
        assertEquals("Secondary Name", claim.secondaryFullName)
        assertEquals("fra", claim.secondaryLanguage)
        assertEquals("GB-LDN", claim.locationCode)
        assertEquals("resident", claim.legalStatus)
        assertEquals("GBR", claim.countryOfIssuance)
    }
}
