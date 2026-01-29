package org.mosip.claim169

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Assertions.*

/**
 * Tests the version API.
 */
class VersionTest {

    @Test
    fun `version returns non-empty string`() {
        val version = Claim169.version()
        assertNotNull(version)
        assertTrue(version.isNotEmpty())
    }

    @Test
    fun `version matches semver pattern`() {
        val version = Claim169.version()
        // Should match semver (possibly with pre-release suffix)
        assertTrue(
            version.matches(Regex("""\d+\.\d+\.\d+.*""")),
            "Version '$version' should match semver pattern"
        )
    }
}
