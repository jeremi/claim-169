/**
 * Verified decoding example - decode with Ed25519 signature verification.
 *
 * This is the recommended approach for production use.
 */

import { describe, it, expect } from "vitest";
import { Decoder, Claim169Error } from "claim169";

// Sample QR code data with Ed25519 signature
// This is from test-vectors/valid/ed25519-signed.json
const QR_DATA =
  "6BF590B20FFWJWG.FKJ05H7B0XKA8FA9DIWENPEJ/5P$DPQE88EB$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL580 B9%W5*B9C+9%R8646%86HKESED1/DRTC5UA QE$345$CVQEX.DX88WBK0NG8PB4 O/38TL6XDALLKLPQATHO.3ZPJMUAVQFSB1:+B*21V FWMC6SU439YU774475LJ2U5T02$VBSIMLQ3:6J.E1-1STM$4";

// Ed25519 public key (32 bytes) - from test vector
const PUBLIC_KEY = new Uint8Array(
  Buffer.from(
    "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
    "hex"
  )
);

describe("Verified Decode Example", () => {
  it("decodes with Ed25519 signature verification", () => {
    console.log("=== Verified Decode Example ===\n");

    // Decode with Ed25519 signature verification
    const result = new Decoder(QR_DATA).verifyWithEd25519(PUBLIC_KEY).decode();

    // Check verification status
    expect(result.verificationStatus).toBe("verified");
    console.log("Signature verified successfully!");

    // Access identity data
    console.log("\nIdentity Data:");
    console.log(`  ID: ${result.claim169.id}`);
    console.log(`  Full Name: ${result.claim169.fullName}`);

    // Access CWT metadata
    console.log("\nCWT Metadata:");
    console.log(`  Issuer: ${result.cwtMeta.issuer}`);
    console.log(`  Expires At: ${result.cwtMeta.expiresAt}`);
    console.log(`  Issued At: ${result.cwtMeta.issuedAt}`);

    // Assertions
    expect(result.claim169.id).toBe("ID-SIGNED-001");
    expect(result.claim169.fullName).toBe("Signed Test Person");
    expect(result.cwtMeta.issuer).toBe("https://mosip.example.org");
  });

  it("fails with wrong key", () => {
    console.log("\n=== Wrong Key Example ===\n");

    // Wrong public key (all zeros)
    const wrongKey = new Uint8Array(32);

    expect(() => {
      new Decoder(QR_DATA).verifyWithEd25519(wrongKey).decode();
    }).toThrow();

    console.log("Expected error - verification failed with wrong key");
  });
});
