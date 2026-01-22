/**
 * Encoding example - create a signed credential.
 *
 * Demonstrates how to create a MOSIP Claim 169 QR code with identity data.
 */

import { describe, it, expect } from "vitest";
import { Encoder, Decoder, Claim169Input, CwtMetaInput } from "claim169";

// Example Ed25519 key pair (for demonstration only!)
// In production, use proper key management (HSM/KMS)
const PRIVATE_KEY = new Uint8Array(
  Buffer.from(
    "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60",
    "hex"
  )
);
const PUBLIC_KEY = new Uint8Array(
  Buffer.from(
    "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
    "hex"
  )
);

describe("Encode Credential Example", () => {
  it("creates and verifies a signed credential", () => {
    console.log("=== Encode Credential Example ===\n");

    // Create identity data
    const claim169: Claim169Input = {
      id: "USER-12345",
      fullName: "Jane Smith",
      firstName: "Jane",
      lastName: "Smith",
      dateOfBirth: "1985-03-15",
      gender: 2, // Female
      email: "jane.smith@example.com",
      phone: "+1-555-123-4567",
      address: "123 Main Street, Anytown, USA",
      nationality: "US",
    };

    // Create CWT metadata
    const now = Math.floor(Date.now() / 1000);
    const expires = now + 365 * 24 * 60 * 60; // 1 year from now

    const cwtMeta: CwtMetaInput = {
      issuer: "https://identity.example.com",
      subject: "USER-12345",
      expiresAt: expires,
      issuedAt: now,
    };

    // Encode with Ed25519 signature
    const qrData = new Encoder(claim169, cwtMeta)
      .signWithEd25519(PRIVATE_KEY)
      .encode();

    console.log(`Generated QR data (${qrData.length} characters):`);
    console.log(`  ${qrData.substring(0, 60)}...`);
    console.log();

    // Verify by decoding
    console.log("Verifying the generated credential...");
    const result = new Decoder(qrData).verifyWithEd25519(PUBLIC_KEY).decode();

    expect(result.verificationStatus).toBe("verified");
    console.log("Credential verified successfully!");

    console.log("\nDecoded data:");
    console.log(`  ID: ${result.claim169.id}`);
    console.log(`  Name: ${result.claim169.fullName}`);
    console.log(`  DOB: ${result.claim169.dateOfBirth}`);
    console.log(`  Issuer: ${result.cwtMeta.issuer}`);

    // Assertions
    expect(result.claim169.id).toBe("USER-12345");
    expect(result.claim169.fullName).toBe("Jane Smith");
    expect(result.cwtMeta.issuer).toBe("https://identity.example.com");
  });

  it("creates ECDSA P-256 signed credential", () => {
    console.log("\n=== ECDSA P-256 Example ===\n");

    // ECDSA P-256 key (32-byte scalar)
    const privateKey = new Uint8Array(
      Buffer.from(
        "c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721",
        "hex"
      )
    );

    const claim169: Claim169Input = {
      id: "ECDSA-001",
      fullName: "ECDSA Test",
    };

    const cwtMeta: CwtMetaInput = {
      issuer: "https://ecdsa.example.com",
    };

    const qrData = new Encoder(claim169, cwtMeta)
      .signWithEcdsaP256(privateKey)
      .encode();

    console.log(
      `Generated ECDSA-signed credential: ${qrData.substring(0, 50)}...`
    );

    expect(qrData).toBeTruthy();
    expect(qrData.length).toBeGreaterThan(0);
  });
});
