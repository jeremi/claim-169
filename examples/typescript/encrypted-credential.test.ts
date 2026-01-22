/**
 * Encrypted credential example - decode an encrypted and signed credential.
 *
 * Demonstrates handling credentials that are both signed and encrypted.
 */

import { describe, it, expect } from "vitest";
import { Encoder, Decoder, Claim169Input, CwtMetaInput } from "claim169";

// Keys for demonstration (never use in production!)
const SIGN_PRIVATE_KEY = new Uint8Array(
  Buffer.from(
    "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60",
    "hex"
  )
);
const SIGN_PUBLIC_KEY = new Uint8Array(
  Buffer.from(
    "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
    "hex"
  )
);
const ENCRYPT_KEY = new Uint8Array(
  Buffer.from(
    "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
    "hex"
  )
);

function createEncryptedCredential(): string {
  const claim169: Claim169Input = {
    id: "ENCRYPTED-001",
    fullName: "Encrypted Person",
    email: "encrypted@example.com",
  };

  const cwtMeta: CwtMetaInput = {
    issuer: "https://secure.example.com",
    expiresAt: 1900000000,
  };

  // Create signed and encrypted credential
  return new Encoder(claim169, cwtMeta)
    .signWithEd25519(SIGN_PRIVATE_KEY)
    .encryptWithAes256(ENCRYPT_KEY)
    .encode();
}

describe("Encrypted Credential Example", () => {
  it("decrypts and verifies a credential", () => {
    console.log("=== Encrypted Credential Example ===\n");

    // Create an encrypted credential for this example
    const qrData = createEncryptedCredential();
    console.log(
      `Encrypted credential (${qrData.length} chars): ${qrData.substring(0, 50)}...`
    );
    console.log();

    // Decrypt and verify
    console.log("Decrypt with AES-256, verify with Ed25519");
    console.log("-".repeat(50));

    const result = new Decoder(qrData)
      .decryptWithAes256(ENCRYPT_KEY)
      .verifyWithEd25519(SIGN_PUBLIC_KEY)
      .decode();

    console.log(`  Verification status: ${result.verificationStatus}`);
    console.log(`  ID: ${result.claim169.id}`);
    console.log(`  Name: ${result.claim169.fullName}`);
    console.log(`  Issuer: ${result.cwtMeta.issuer}`);

    expect(result.verificationStatus).toBe("verified");
    expect(result.claim169.id).toBe("ENCRYPTED-001");
    expect(result.claim169.fullName).toBe("Encrypted Person");
    expect(result.cwtMeta.issuer).toBe("https://secure.example.com");
  });

  it("fails with wrong decryption key", () => {
    console.log("\n\nWrong Decryption Key Example");
    console.log("-".repeat(50));

    const qrData = createEncryptedCredential();
    const wrongKey = new Uint8Array(32); // All zeros

    expect(() => {
      new Decoder(qrData)
        .decryptWithAes256(wrongKey)
        .allowUnverified()
        .decode();
    }).toThrow();

    console.log("  Expected error - decryption failed with wrong key");
  });
});
