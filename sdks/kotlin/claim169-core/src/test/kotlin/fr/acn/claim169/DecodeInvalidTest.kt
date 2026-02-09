package fr.acn.claim169

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*
import fr.acn.claim169.Claim169Exception

/**
 * Tests decoding invalid test vectors.
 */
class DecodeInvalidTest {

    @Test
    fun `bad base45 throws Base45Decode`() {
        val vector = TestVectorLoader.loadVector("invalid", "bad-base45")
        val qrData = vector.get("qr_data").asString

        assertThrows(Claim169Exception.Base45Decode::class.java) {
            Claim169.decode(qrData) {
                allowUnverified()
                withoutTimestampValidation()
            }
        }
    }

    @Test
    fun `bad zlib throws Decompress`() {
        val vector = TestVectorLoader.loadVector("invalid", "bad-zlib")
        val qrData = vector.get("qr_data").asString

        assertThrows(Claim169Exception.Decompress::class.java) {
            Claim169.decode(qrData) {
                allowUnverified()
                withoutTimestampValidation()
            }
        }
    }

    @Test
    fun `not cose throws CoseParse`() {
        val vector = TestVectorLoader.loadVector("invalid", "not-cose")
        val qrData = vector.get("qr_data").asString

        assertThrows(Claim169Exception.CoseParse::class.java) {
            Claim169.decode(qrData) {
                allowUnverified()
                withoutTimestampValidation()
            }
        }
    }

    @Test
    fun `missing 169 throws Claim169NotFound`() {
        val vector = TestVectorLoader.loadVector("invalid", "missing-169")
        val qrData = vector.get("qr_data").asString

        assertThrows(Claim169Exception.Claim169NotFound::class.java) {
            Claim169.decode(qrData) {
                allowUnverified()
                withoutTimestampValidation()
            }
        }
    }

    @Test
    fun `empty string throws error`() {
        assertThrows(Claim169Exception::class.java) {
            Claim169.decode("") {
                allowUnverified()
                withoutTimestampValidation()
            }
        }
    }
}
