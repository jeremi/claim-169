package fr.acn.claim169

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*
import fr.acn.claim169.Claim169Exception
import java.util.Base64

/**
 * Negative tests for PEM-format public key error paths.
 *
 * Covers malformed PEM inputs, wrong key types, corrupted content,
 * and other invalid PEM scenarios for both Ed25519 and ECDSA P-256.
 */
class PemErrorTest {

    companion object {
        /** Valid Ed25519 PEM (from test vectors) for cross-type tests */
        private fun validEd25519Pem(): String {
            val vector = TestVectorLoader.loadVector("valid", "ed25519-signed")
            val signingKey = vector.getAsJsonObject("signing_key")
            val publicKeyHex = signingKey.get("public_key_hex").asString
            val publicKey = TestVectorLoader.hexToByteArray(publicKeyHex)
            return DecodeValidTest.ed25519PublicKeyToPem(publicKey)
        }

        /** Valid ECDSA P-256 PEM (from test vectors) for cross-type tests */
        private fun validP256Pem(): String {
            val vector = TestVectorLoader.loadVector("valid", "ecdsa-p256-signed")
            val signingKey = vector.getAsJsonObject("signing_key")
            val publicKeyHex = signingKey.get("public_key_hex").asString
            val publicKey = TestVectorLoader.hexToByteArray(publicKeyHex)
            return DecodeValidTest.p256PublicKeyToPem(publicKey)
        }

        /** QR data signed with Ed25519 */
        private fun ed25519QrData(): String {
            val vector = TestVectorLoader.loadVector("valid", "ed25519-signed")
            return vector.get("qr_data").asString
        }

        /** QR data signed with ECDSA P-256 */
        private fun ecdsaP256QrData(): String {
            val vector = TestVectorLoader.loadVector("valid", "ecdsa-p256-signed")
            return vector.get("qr_data").asString
        }
    }

    // --- Empty PEM string ---

    @Test
    fun `ed25519 pem with empty string throws Crypto`() {
        val qrData = ed25519QrData()

        assertThrows(Claim169Exception.Crypto::class.java) {
            Claim169.decode(qrData) {
                verifyWithEd25519Pem("")
                withoutTimestampValidation()
            }
        }
    }

    @Test
    fun `ecdsa p256 pem with empty string throws Crypto`() {
        val qrData = ecdsaP256QrData()

        assertThrows(Claim169Exception.Crypto::class.java) {
            Claim169.decode(qrData) {
                verifyWithEcdsaP256Pem("")
                withoutTimestampValidation()
            }
        }
    }

    // --- Malformed PEM headers ---

    @Test
    fun `ed25519 pem with wrong header throws Crypto`() {
        val qrData = ed25519QrData()
        val badPem = "-----BEGIN CERTIFICATE-----\nMCowBQYDK2VwAyEA\n-----END CERTIFICATE-----"

        assertThrows(Claim169Exception.Crypto::class.java) {
            Claim169.decode(qrData) {
                verifyWithEd25519Pem(badPem)
                withoutTimestampValidation()
            }
        }
    }

    @Test
    fun `ecdsa p256 pem with wrong header throws Crypto`() {
        val qrData = ecdsaP256QrData()
        val badPem = "-----BEGIN CERTIFICATE-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQc=\n-----END CERTIFICATE-----"

        assertThrows(Claim169Exception.Crypto::class.java) {
            Claim169.decode(qrData) {
                verifyWithEcdsaP256Pem(badPem)
                withoutTimestampValidation()
            }
        }
    }

    // --- Corrupted base64 content ---

    @Test
    fun `ed25519 pem with corrupted base64 throws Crypto`() {
        val qrData = ed25519QrData()
        val badPem = "-----BEGIN PUBLIC KEY-----\n!!!not-valid-base64!!!\n-----END PUBLIC KEY-----"

        assertThrows(Claim169Exception.Crypto::class.java) {
            Claim169.decode(qrData) {
                verifyWithEd25519Pem(badPem)
                withoutTimestampValidation()
            }
        }
    }

    @Test
    fun `ecdsa p256 pem with corrupted base64 throws Crypto`() {
        val qrData = ecdsaP256QrData()
        val badPem = "-----BEGIN PUBLIC KEY-----\n@@@corrupted-base64@@@\n-----END PUBLIC KEY-----"

        assertThrows(Claim169Exception.Crypto::class.java) {
            Claim169.decode(qrData) {
                verifyWithEcdsaP256Pem(badPem)
                withoutTimestampValidation()
            }
        }
    }

    // --- Wrong key type (cross-algorithm) ---

    @Test
    fun `ed25519 pem method with ecdsa p256 key throws Crypto`() {
        val qrData = ed25519QrData()
        val p256Pem = validP256Pem()

        // Passing a P-256 PEM to an Ed25519 method should fail
        assertThrows(Claim169Exception.Crypto::class.java) {
            Claim169.decode(qrData) {
                verifyWithEd25519Pem(p256Pem)
                withoutTimestampValidation()
            }
        }
    }

    @Test
    fun `ecdsa p256 pem method with ed25519 key throws Crypto`() {
        val qrData = ecdsaP256QrData()
        val ed25519Pem = validEd25519Pem()

        // Passing an Ed25519 PEM to a P-256 method should fail
        assertThrows(Claim169Exception.Crypto::class.java) {
            Claim169.decode(qrData) {
                verifyWithEcdsaP256Pem(ed25519Pem)
                withoutTimestampValidation()
            }
        }
    }

    // --- PEM with trailing garbage after END marker ---

    @Test
    fun `ed25519 pem with trailing garbage after end marker throws Crypto`() {
        val qrData = ed25519QrData()
        val validPem = validEd25519Pem()
        val pemWithGarbage = validPem + "\nGARBAGE DATA AFTER END MARKER"

        // Trailing garbage should cause a parse error
        assertThrows(Claim169Exception.Crypto::class.java) {
            Claim169.decode(qrData) {
                verifyWithEd25519Pem(pemWithGarbage)
                withoutTimestampValidation()
            }
        }
    }

    @Test
    fun `ecdsa p256 pem with trailing garbage after end marker throws Crypto`() {
        val qrData = ecdsaP256QrData()
        val validPem = validP256Pem()
        val pemWithGarbage = validPem + "\nGARBAGE DATA AFTER END MARKER"

        // Trailing garbage should cause a parse error
        assertThrows(Claim169Exception.Crypto::class.java) {
            Claim169.decode(qrData) {
                verifyWithEcdsaP256Pem(pemWithGarbage)
                withoutTimestampValidation()
            }
        }
    }

    // --- PEM with valid format but invalid key data ---

    @Test
    fun `ed25519 pem with valid format but wrong key length throws Crypto`() {
        val qrData = ed25519QrData()
        // Encode 16 random bytes (too short for Ed25519's 32-byte key) with SPKI prefix
        val shortKey = ByteArray(16) { 0x42 }
        val b64 = Base64.getMimeEncoder(64, "\n".toByteArray()).encodeToString(shortKey)
        val badPem = "-----BEGIN PUBLIC KEY-----\n$b64\n-----END PUBLIC KEY-----"

        assertThrows(Claim169Exception.Crypto::class.java) {
            Claim169.decode(qrData) {
                verifyWithEd25519Pem(badPem)
                withoutTimestampValidation()
            }
        }
    }

    @Test
    fun `ecdsa p256 pem with valid format but wrong key length throws Crypto`() {
        val qrData = ecdsaP256QrData()
        // Encode 16 random bytes (too short for P-256) with SPKI prefix
        val shortKey = ByteArray(16) { 0x42 }
        val b64 = Base64.getMimeEncoder(64, "\n".toByteArray()).encodeToString(shortKey)
        val badPem = "-----BEGIN PUBLIC KEY-----\n$b64\n-----END PUBLIC KEY-----"

        assertThrows(Claim169Exception.Crypto::class.java) {
            Claim169.decode(qrData) {
                verifyWithEcdsaP256Pem(badPem)
                withoutTimestampValidation()
            }
        }
    }

    // --- Plain text (not PEM at all) ---

    @Test
    fun `ed25519 pem with plain text throws Crypto`() {
        val qrData = ed25519QrData()

        assertThrows(Claim169Exception.Crypto::class.java) {
            Claim169.decode(qrData) {
                verifyWithEd25519Pem("this is not a PEM string at all")
                withoutTimestampValidation()
            }
        }
    }

    @Test
    fun `ecdsa p256 pem with plain text throws Crypto`() {
        val qrData = ecdsaP256QrData()

        assertThrows(Claim169Exception.Crypto::class.java) {
            Claim169.decode(qrData) {
                verifyWithEcdsaP256Pem("this is not a PEM string at all")
                withoutTimestampValidation()
            }
        }
    }

    // --- PEM with missing END marker ---

    @Test
    fun `ed25519 pem with missing end marker throws Crypto`() {
        val qrData = ed25519QrData()
        val validPem = validEd25519Pem()
        // Strip the END marker
        val truncatedPem = validPem.substringBefore("-----END PUBLIC KEY-----")

        assertThrows(Claim169Exception.Crypto::class.java) {
            Claim169.decode(qrData) {
                verifyWithEd25519Pem(truncatedPem)
                withoutTimestampValidation()
            }
        }
    }

    @Test
    fun `ecdsa p256 pem with missing end marker throws Crypto`() {
        val qrData = ecdsaP256QrData()
        val validPem = validP256Pem()
        // Strip the END marker
        val truncatedPem = validPem.substringBefore("-----END PUBLIC KEY-----")

        assertThrows(Claim169Exception.Crypto::class.java) {
            Claim169.decode(qrData) {
                verifyWithEcdsaP256Pem(truncatedPem)
                withoutTimestampValidation()
            }
        }
    }
}
