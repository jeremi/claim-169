package fr.acn.claim169

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*
import fr.acn.claim169.Claim169Exception

/**
 * Security regression tests: verifies secure-by-default behavior.
 */
class SecurityTest {

    @Test
    fun `decode without verification or allowUnverified throws error`() {
        val vector = TestVectorLoader.loadVector("valid", "minimal")
        val qrData = vector.get("qr_data").asString

        // Secure by default: must either provide a key or explicitly allow unverified
        assertThrows(Claim169Exception::class.java) {
            Claim169.decode(qrData) {
                withoutTimestampValidation()
            }
        }
    }

    @Test
    fun `timestamp validation enabled by default rejects expired credential`() {
        // Encode with an already-expired timestamp
        val vector = TestVectorLoader.loadVector("valid", "ed25519-signed")
        val signingKey = vector.getAsJsonObject("signing_key")
        val privateKeyHex = signingKey.get("private_key_hex").asString
        val publicKeyHex = signingKey.get("public_key_hex").asString
        val privateKey = TestVectorLoader.hexToByteArray(privateKeyHex)
        val publicKey = TestVectorLoader.hexToByteArray(publicKeyHex)

        val data = claim169Data {
            id = "SEC-EXPIRED-001"
            fullName = "Expired Security Test"
        }
        val meta = cwtMetaData {
            issuer = "https://test.example.com"
            expiresAt = 1L // epoch + 1 second, definitely expired
        }

        val encoded = Claim169.encode(data, meta) {
            signWithEd25519(privateKey)
        }

        // Should throw Expired because timestamp validation is enabled by default
        assertThrows(Claim169Exception.Expired::class.java) {
            Claim169.decode(encoded) {
                verifyWithEd25519(publicKey)
            }
        }

        // Should succeed with timestamp validation disabled
        val result = Claim169.decode(encoded) {
            verifyWithEd25519(publicKey)
            withoutTimestampValidation()
        }
        assertEquals("SEC-EXPIRED-001", result.claim169.id)
    }

    @Test
    fun `wrong ECDSA P-256 key fails verification`() {
        val vector = TestVectorLoader.loadVector("valid", "ecdsa-p256-signed")
        val qrData = vector.get("qr_data").asString
        // Wrong key: 65 bytes of 0xFF (uncompressed point prefix 0x04 + 64 bytes)
        val wrongKey = ByteArray(65) { 0xFF.toByte() }
        wrongKey[0] = 0x04 // uncompressed point prefix

        assertThrows(Claim169Exception::class.java) {
            Claim169.decode(qrData) {
                verifyWithEcdsaP256(wrongKey)
                withoutTimestampValidation()
            }
        }
    }

    @Test
    fun `wrong AES-256 key fails decryption`() {
        val vector = TestVectorLoader.loadVector("valid", "encrypted-aes256")
        val qrData = vector.get("qr_data").asString
        val wrongKey = ByteArray(32) { 0xAA.toByte() }

        assertThrows(Claim169Exception::class.java) {
            Claim169.decode(qrData) {
                decryptWithAes256(wrongKey)
                allowUnverified()
                withoutTimestampValidation()
            }
        }
    }
}
