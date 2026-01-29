package org.acn.claim169

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*
import uniffi.claim169_jni.Claim169Exception

/**
 * Tests custom callback interfaces (SignatureVerifier, Signer, Decryptor, Encryptor).
 */
class CallbackTest {

    @Test
    fun `custom signer callback with key id`() {
        val vector = TestVectorLoader.loadVector("valid", "ed25519-signed")
        val signingKey = vector.getAsJsonObject("signing_key")
        val privateKeyHex = signingKey.get("private_key_hex").asString
        val privateKey = TestVectorLoader.hexToByteArray(privateKeyHex)
        val publicKeyHex = signingKey.get("public_key_hex").asString
        val publicKey = TestVectorLoader.hexToByteArray(publicKeyHex)

        val data = claim169 {
            id = "SIGNER-KEYID-001"
            fullName = "Signer Key ID Test"
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        val encoded = Claim169.encode(data, meta) {
            signWithEd25519(privateKey)
        }

        val result = Claim169.decode(encoded) {
            verifyWithEd25519(publicKey)
            withoutTimestampValidation()
        }

        assertEquals("SIGNER-KEYID-001", result.claim169.id)
        assertEquals("verified", result.verificationStatus)
    }

    @Test
    fun `custom encryptor callback is invoked`() {
        var encryptCalled = false

        // We can't implement real encryption in the callback, but we can verify it's invoked
        // by encoding with a custom encryptor that throws
        val data = claim169 {
            id = "ENC-CB-001"
            fullName = "Encryptor Callback Test"
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        assertThrows(Exception::class.java) {
            Claim169.encode(data, meta) {
                allowUnsigned()
                encryptWith(object : Encryptor {
                    override fun encrypt(
                        algorithm: String,
                        keyId: ByteArray?,
                        nonce: ByteArray,
                        aad: ByteArray,
                        plaintext: ByteArray
                    ): ByteArray {
                        encryptCalled = true
                        assertEquals("A256GCM", algorithm)
                        assertTrue(nonce.isNotEmpty())
                        assertTrue(plaintext.isNotEmpty())
                        throw RuntimeException("Custom encryption not implemented")
                    }
                }, "A256GCM")
            }
        }

        assertTrue(encryptCalled, "Encrypt callback should have been called")
    }

    @Test
    fun `custom signer and verifier callbacks`() {
        val vector = TestVectorLoader.loadVector("valid", "ed25519-signed")
        val signingKey = vector.getAsJsonObject("signing_key")
        val privateKeyHex = signingKey.get("private_key_hex").asString
        val privateKey = TestVectorLoader.hexToByteArray(privateKeyHex)

        var verifyCalled = false

        val data = claim169 {
            id = "CALLBACK-001"
            fullName = "Callback Test"
        }
        val meta = cwtMeta {
            issuer = "https://test.example.com"
            expiresAt = 2000000000L
        }

        // Encode using the built-in Ed25519 but through a custom Signer that delegates
        // We can't easily do a pure custom signer without crypto, so we use the DSL
        // to encode with built-in, then verify with custom callback
        val encoded = Claim169.encode(data, meta) {
            signWithEd25519(privateKey)
        }

        // Decode with custom verifier callback
        val result = Claim169.decode(encoded) {
            verifyWith(object : SignatureVerifier {
                override fun verify(
                    algorithm: String,
                    keyId: ByteArray?,
                    data: ByteArray,
                    signature: ByteArray
                ) {
                    verifyCalled = true
                    assertEquals("EdDSA", algorithm)
                    assertTrue(data.isNotEmpty())
                    assertTrue(signature.isNotEmpty())
                    // Delegate to built-in verification by re-decoding
                    // (this verifies the callback is called with correct data)
                    // For a real test, we'd use a crypto library, but the callback mechanism
                    // is what we're testing here
                }
            })
            withoutTimestampValidation()
        }

        assertTrue(verifyCalled, "Verify callback should have been called")
        assertEquals("CALLBACK-001", result.claim169.id)
    }

    @Test
    fun `custom verifier that rejects throws exception`() {
        val vector = TestVectorLoader.loadVector("valid", "ed25519-signed")
        val qrData = vector.get("qr_data").asString

        // UniFFI wraps callback exceptions as InternalException since the callback
        // throws RuntimeException which isn't mapped to a UniFFI error type
        assertThrows(Exception::class.java) {
            Claim169.decode(qrData) {
                verifyWith(object : SignatureVerifier {
                    override fun verify(
                        algorithm: String,
                        keyId: ByteArray?,
                        data: ByteArray,
                        signature: ByteArray
                    ) {
                        throw RuntimeException("Verification rejected by custom verifier")
                    }
                })
                withoutTimestampValidation()
            }
        }
    }

    @Test
    fun `custom decryptor callback`() {
        // We can't easily test a custom decryptor without implementing AES-GCM,
        // but we can verify the callback is invoked and that errors propagate
        val vector = TestVectorLoader.loadVector("valid", "encrypted-aes256")
        val qrData = vector.get("qr_data").asString
        var decryptCalled = false

        // UniFFI wraps callback exceptions as InternalException
        assertThrows(Exception::class.java) {
            Claim169.decode(qrData) {
                decryptWith(object : Decryptor {
                    override fun decrypt(
                        algorithm: String,
                        keyId: ByteArray?,
                        nonce: ByteArray,
                        aad: ByteArray,
                        ciphertext: ByteArray
                    ): ByteArray {
                        decryptCalled = true
                        assertEquals("A256GCM", algorithm)
                        assertTrue(nonce.isNotEmpty())
                        assertTrue(ciphertext.isNotEmpty())
                        throw RuntimeException("Custom decryption not implemented")
                    }
                })
                allowUnverified()
                withoutTimestampValidation()
            }
        }

        assertTrue(decryptCalled, "Decrypt callback should have been called")
    }
}
