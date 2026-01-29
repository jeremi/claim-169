package org.acn.claim169

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*
import uniffi.claim169_jni.Claim169Exception

/**
 * Tests encoder error paths: invalid keys, missing signing method, etc.
 */
class EncodeErrorTest {

    private val validData = claim169 {
        id = "ERR-001"
        fullName = "Error Test"
    }
    private val validMeta = cwtMeta {
        issuer = "https://test.example.com"
        expiresAt = 2000000000L
    }

    @Test
    fun `encode without signing method throws error`() {
        assertThrows(Claim169Exception::class.java) {
            Claim169.encode(validData, validMeta) {
                // No signing method and no allowUnsigned()
            }
        }
    }

    @Test
    fun `encode with invalid Ed25519 key length throws error`() {
        val shortKey = ByteArray(16) { 0x01 } // Ed25519 requires exactly 32 bytes

        assertThrows(Claim169Exception::class.java) {
            Claim169.encode(validData, validMeta) {
                signWithEd25519(shortKey)
            }
        }
    }

    @Test
    fun `encode with invalid ECDSA P-256 key length throws error`() {
        val shortKey = ByteArray(16) { 0x01 } // ECDSA P-256 requires 32-byte scalar

        assertThrows(Claim169Exception::class.java) {
            Claim169.encode(validData, validMeta) {
                signWithEcdsaP256(shortKey)
            }
        }
    }

    @Test
    fun `encode with invalid AES-256 key length throws error`() {
        val shortKey = ByteArray(16) { 0x01 } // AES-256 requires 32 bytes

        assertThrows(Claim169Exception::class.java) {
            Claim169.encode(validData, validMeta) {
                allowUnsigned()
                encryptWithAes256(shortKey)
            }
        }
    }

    @Test
    fun `encode with invalid AES-128 key length throws error`() {
        val wrongKey = ByteArray(32) { 0x01 } // AES-128 requires 16 bytes

        assertThrows(Claim169Exception::class.java) {
            Claim169.encode(validData, validMeta) {
                allowUnsigned()
                encryptWithAes128(wrongKey)
            }
        }
    }

    @Test
    fun `encode with signed and encrypted using wrong encryption key length`() {
        val vector = TestVectorLoader.loadVector("valid", "ed25519-signed")
        val signingKey = vector.getAsJsonObject("signing_key")
        val privateKeyHex = signingKey.get("private_key_hex").asString
        val privateKey = TestVectorLoader.hexToByteArray(privateKeyHex)
        val wrongSizeEncKey = ByteArray(10) { 0x01 }

        assertThrows(Claim169Exception::class.java) {
            Claim169.encode(validData, validMeta) {
                signWithEd25519(privateKey)
                encryptWithAes256(wrongSizeEncKey)
            }
        }
    }
}
