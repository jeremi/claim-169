package org.acn.claim169

import com.google.gson.Gson
import com.google.gson.GsonBuilder
import com.google.gson.JsonObject
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*
import java.io.File

/**
 * Conformance test that produces JSON output compatible with the
 * cross-language conformance test script (scripts/conformance-test.sh).
 *
 * When invoked via the conformance script, the output is compared
 * against Python and TypeScript SDKs to ensure consistency.
 */
class ConformanceTest {

    private val gson: Gson = GsonBuilder().serializeNulls().create()

    @Test
    fun `process all vectors for conformance`() {
        val vectorsPath = System.getProperty("conformance.vectors.path")
        val outputPath = System.getProperty("conformance.output.path")

        // When run outside of the conformance script, use test vectors directory
        val vectorsDir = System.getProperty("test.vectors.dir")
            ?: return // Skip if not configured

        if (vectorsPath != null && outputPath != null) {
            // Running via conformance script: process vectors JSON file
            processVectorsFile(vectorsPath, outputPath)
        } else {
            // Running standalone: just verify all vectors decode without crashing
            processTestVectorsDirectory(vectorsDir)
        }
    }

    private fun processVectorsFile(vectorsPath: String, outputPath: String) {
        val vectorsJson = File(vectorsPath).readText()
        val vectors = gson.fromJson(vectorsJson, Array<ConformanceVector>::class.java)
        val results = vectors.map { processVector(it) }
        File(outputPath).writeText(gson.toJson(results))
    }

    private fun processTestVectorsDirectory(vectorsDir: String) {
        var total = 0
        var decoded = 0
        var errors = 0

        for (category in listOf("valid", "invalid", "edge")) {
            val dir = File(vectorsDir, category)
            if (!dir.exists()) continue

            for (file in dir.listFiles { f -> f.extension == "json" } ?: emptyArray()) {
                total++
                val json = gson.fromJson(file.readText(), JsonObject::class.java)
                val qrData = json.get("qr_data").asString
                val name = json.get("name").asString

                val vector = ConformanceVector(name, category, qrData)
                val result = processVector(vector)

                if (result.success) decoded++ else errors++
            }
        }

        assertTrue(total > 0, "Should have processed at least one vector")
        // Valid vectors should decode, invalid should fail
        assertTrue(decoded > 0, "At least some vectors should decode successfully")
        assertTrue(errors > 0, "At least some vectors should fail (invalid vectors)")
    }

    private fun processVector(vector: ConformanceVector): ConformanceResult {
        return try {
            val result = Claim169.decode(vector.qrData) {
                allowUnverified()
                withoutTimestampValidation()
            }

            ConformanceResult(
                name = vector.name,
                category = vector.category,
                success = true,
                id = result.claim169.id,
                fullName = result.claim169.fullName,
                dateOfBirth = result.claim169.dateOfBirth,
                gender = result.claim169.gender,
                issuer = result.cwtMeta.issuer,
                expiresAt = result.cwtMeta.expiresAt,
                verificationStatus = result.verificationStatus,
                error = null,
            )
        } catch (e: Exception) {
            ConformanceResult(
                name = vector.name,
                category = vector.category,
                success = false,
                error = e.message,
            )
        }
    }

    data class ConformanceVector(
        val name: String,
        val category: String,
        val qrData: String,
    )

    data class ConformanceResult(
        val name: String,
        val category: String,
        val success: Boolean,
        val id: String? = null,
        val fullName: String? = null,
        val dateOfBirth: String? = null,
        val gender: Long? = null,
        val issuer: String? = null,
        val expiresAt: Long? = null,
        val verificationStatus: String? = null,
        val error: String? = null,
    )
}
