/**
 * Basic decoding example - decode without signature verification.
 *
 * WARNING: This is for testing/development only. In production, always
 * verify signatures using verifyWithEd25519() or verifyWithEcdsaP256().
 */

import { describe, it, expect } from "vitest";
import { Decoder } from "claim169";

// Sample QR code data (Base45 encoded)
// This is from test-vectors/valid/minimal.json
const QR_DATA =
  "6BFL70+30FFWJWG.FKJ0587B0XKA8FA9DIWENPEJ/5P$DPQE88EB$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL680 B94W5QF60R6KW5/G8HS83P0KI949DP34W3ER68HSLK1";

describe("Basic Decode Example", () => {
  it("decodes a QR code without verification", () => {
    console.log("=== Basic Decode Example ===\n");

    // Decode without verification (testing only!)
    const result = new Decoder(QR_DATA).allowUnverified().decode();

    // Access identity data
    console.log("Identity Data:");
    console.log(`  ID: ${result.claim169.id}`);
    console.log(`  Full Name: ${result.claim169.fullName}`);

    // Access CWT metadata
    console.log("\nCWT Metadata:");
    console.log(`  Issuer: ${result.cwtMeta.issuer}`);
    console.log(`  Expires At: ${result.cwtMeta.expiresAt}`);

    // Check verification status
    console.log(`\nVerification Status: ${result.verificationStatus}`);

    // Assertions
    expect(result.claim169.id).toBe("ID-12345-ABCDE");
    expect(result.claim169.fullName).toBe("John Doe");
    expect(result.cwtMeta.issuer).toBe("https://mosip.example.org");
    expect(result.verificationStatus).toBe("skipped");
  });
});
