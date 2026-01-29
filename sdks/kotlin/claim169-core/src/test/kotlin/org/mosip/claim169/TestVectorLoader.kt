package org.mosip.claim169

import com.google.gson.Gson
import com.google.gson.JsonObject
import java.io.File

/**
 * Loads test vectors from the shared test-vectors directory.
 */
object TestVectorLoader {
    private val gson = Gson()
    private val vectorsDir: File by lazy {
        val path = System.getProperty("test.vectors.dir")
            ?: throw IllegalStateException("test.vectors.dir system property not set")
        File(path).also {
            require(it.exists()) { "Test vectors directory does not exist: $path" }
        }
    }

    fun loadVector(category: String, name: String): JsonObject {
        val file = File(vectorsDir, "$category/$name.json")
        require(file.exists()) { "Test vector not found: ${file.absolutePath}" }
        return gson.fromJson(file.readText(), JsonObject::class.java)
    }

    fun loadAllInCategory(category: String): List<JsonObject> {
        val dir = File(vectorsDir, category)
        if (!dir.exists()) return emptyList()
        return dir.listFiles { f -> f.extension == "json" }
            ?.map { gson.fromJson(it.readText(), JsonObject::class.java) }
            ?: emptyList()
    }

    fun hexToByteArray(hex: String): ByteArray {
        require(hex.length % 2 == 0) { "Hex string must have even length" }
        return ByteArray(hex.length / 2) { i ->
            hex.substring(i * 2, i * 2 + 2).toInt(16).toByte()
        }
    }
}
