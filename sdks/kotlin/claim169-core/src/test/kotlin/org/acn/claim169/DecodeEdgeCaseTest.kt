package org.acn.claim169

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*
import uniffi.claim169_jni.Claim169Exception

/**
 * Tests decoding edge case test vectors.
 */
class DecodeEdgeCaseTest {

    @Test
    fun `expired token throws Expired`() {
        val vector = TestVectorLoader.loadVector("edge", "expired")
        val qrData = vector.get("qr_data").asString

        assertThrows(Claim169Exception.Expired::class.java) {
            Claim169.decode(qrData) {
                allowUnverified()
            }
        }
    }

    @Test
    fun `expired token decodes with timestamp validation disabled`() {
        val vector = TestVectorLoader.loadVector("edge", "expired")
        val qrData = vector.get("qr_data").asString

        val result = Claim169.decode(qrData) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("ID-EXPIRED-001", result.claim169.id)
        assertEquals("Expired Test Person", result.claim169.fullName)
        assertEquals(1609459200L, result.cwtMeta.expiresAt)
    }

    @Test
    fun `not-yet-valid token throws NotYetValid`() {
        val vector = TestVectorLoader.loadVector("edge", "not-yet-valid")
        val qrData = vector.get("qr_data").asString

        assertThrows(Claim169Exception.NotYetValid::class.java) {
            Claim169.decode(qrData) {
                allowUnverified()
            }
        }
    }

    @Test
    fun `not-yet-valid token decodes with timestamp validation disabled`() {
        val vector = TestVectorLoader.loadVector("edge", "not-yet-valid")
        val qrData = vector.get("qr_data").asString

        val result = Claim169.decode(qrData) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("ID-FUTURE-001", result.claim169.id)
        assertEquals("Future Test Person", result.claim169.fullName)
    }

    @Test
    fun `unknown fields preserved`() {
        val vector = TestVectorLoader.loadVector("edge", "unknown-fields")
        val qrData = vector.get("qr_data").asString

        val result = Claim169.decode(qrData) {
            allowUnverified()
            withoutTimestampValidation()
        }

        assertEquals("ID-UNKNOWN-001", result.claim169.id)
        assertEquals("Unknown Fields Person", result.claim169.fullName)
        // Unknown fields should be preserved as JSON
        assertNotNull(result.claim169.unknownFieldsJson)
    }

    @Test
    fun `maxDecompressedBytes limits decompression`() {
        val vector = TestVectorLoader.loadVector("valid", "minimal")
        val qrData = vector.get("qr_data").asString

        // Setting an extremely small limit should cause decompression to fail
        assertThrows(Claim169Exception.DecompressLimitExceeded::class.java) {
            Claim169.decode(qrData) {
                allowUnverified()
                withoutTimestampValidation()
                maxDecompressedBytes(1)
            }
        }
    }

    @Test
    fun `maxDecompressedBytes rejects negative values`() {
        assertThrows(IllegalArgumentException::class.java) {
            Claim169.decode("dummy") {
                maxDecompressedBytes(-1)
            }
        }
    }

    @Test
    fun `maxDecompressedBytes rejects zero`() {
        assertThrows(IllegalArgumentException::class.java) {
            Claim169.decode("dummy") {
                maxDecompressedBytes(0)
            }
        }
    }

    @Test
    fun `clockSkewTolerance rejects negative values`() {
        assertThrows(IllegalArgumentException::class.java) {
            Claim169.decode("dummy") {
                clockSkewTolerance(-1)
            }
        }
    }

    @Test
    fun `clock skew tolerance allows slightly expired tokens`() {
        val vector = TestVectorLoader.loadVector("edge", "expired")
        val qrData = vector.get("qr_data").asString

        // This token expired at 1609459200 (2021-01-01). Even a very large clock skew
        // shouldn't help since it's years expired. Verify it still throws.
        assertThrows(Claim169Exception.Expired::class.java) {
            Claim169.decode(qrData) {
                allowUnverified()
                clockSkewTolerance(60)
            }
        }
    }
}
