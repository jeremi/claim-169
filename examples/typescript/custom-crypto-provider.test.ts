/**
 * Custom Crypto Provider Example - using external signing/encryption providers.
 *
 * This example demonstrates how to integrate claim169 with external cryptographic
 * providers such as:
 *
 * - **Hardware Security Modules (HSMs)** - Thales Luna, nCipher, AWS CloudHSM
 * - **Cloud Key Management Services** - AWS KMS, Google Cloud KMS, Azure Key Vault
 * - **Smart Cards / TPMs** - PKCS#11 tokens, Trusted Platform Modules
 * - **Remote Signing Services** - DocuSign, DigiCert, custom PKI
 *
 * The custom callback pattern allows you to keep private keys in secure hardware
 * while still using claim169 for credential encoding/decoding.
 *
 * ## Callback Signatures
 *
 * ### SignerCallback
 * ```typescript
 * (algorithm: string, keyId: Uint8Array | null, data: Uint8Array) => Uint8Array
 * ```
 * - `algorithm`: COSE algorithm identifier ("EdDSA" or "ES256")
 * - `keyId`: Optional key identifier from the COSE header (for key selection)
 * - `data`: The COSE Sig_structure to sign (already formatted, just sign it)
 * - Returns: The signature bytes (64 bytes for EdDSA/ES256)
 *
 * ### VerifierCallback
 * ```typescript
 * (algorithm: string, keyId: Uint8Array | null, data: Uint8Array, signature: Uint8Array) => void
 * ```
 * - `algorithm`: COSE algorithm identifier from the credential
 * - `keyId`: Optional key identifier (can be used to look up the public key)
 * - `data`: The COSE Sig_structure that was signed
 * - `signature`: The signature to verify
 * - Throws: Should throw an error if verification fails
 *
 * ### EncryptorCallback
 * ```typescript
 * (algorithm: string, keyId: Uint8Array | null, nonce: Uint8Array, aad: Uint8Array, plaintext: Uint8Array) => Uint8Array
 * ```
 * - `algorithm`: COSE algorithm identifier ("A256GCM" or "A128GCM")
 * - `keyId`: Optional key identifier for key selection
 * - `nonce`: 12-byte nonce/IV for AES-GCM
 * - `aad`: Additional authenticated data (COSE Enc_structure)
 * - `plaintext`: Data to encrypt
 * - Returns: Ciphertext with authentication tag appended
 *
 * ### DecryptorCallback
 * ```typescript
 * (algorithm: string, keyId: Uint8Array | null, nonce: Uint8Array, aad: Uint8Array, ciphertext: Uint8Array) => Uint8Array
 * ```
 * - `algorithm`: COSE algorithm identifier
 * - `keyId`: Optional key identifier
 * - `nonce`: 12-byte nonce/IV from the COSE header
 * - `aad`: Additional authenticated data
 * - `ciphertext`: Ciphertext with authentication tag
 * - Returns: Decrypted plaintext
 */

import { describe, it, expect } from "vitest";
import {
  Encoder,
  Decoder,
  Claim169Input,
  CwtMetaInput,
  hexToBytes,
  bytesToHex,
} from "claim169";
import type {
  SignerCallback,
  VerifierCallback,
  EncryptorCallback,
  DecryptorCallback,
} from "claim169";

// Import noble crypto libraries for HSM simulation
// In production, you would use your actual HSM/KMS SDK instead
import * as ed25519 from "@noble/ed25519";
import { gcm } from "@noble/ciphers/aes";
import { sha512 } from "@noble/hashes/sha512";

// Configure ed25519 to use sync sha512 (required for synchronous signing)
ed25519.etc.sha512Sync = (...m) => sha512(ed25519.etc.concatBytes(...m));

// =============================================================================
// Mock HSM/KMS Implementation
// =============================================================================
//
// In a real implementation, these would call out to your HSM/KMS SDK:
//
// AWS KMS Example:
//   const kmsClient = new KMSClient({ region: "us-east-1" });
//   const response = await kmsClient.send(new SignCommand({
//     KeyId: "arn:aws:kms:...",
//     Message: data,
//     SigningAlgorithm: algorithm === "EdDSA" ? "..." : "ECDSA_SHA_256",
//   }));
//   return new Uint8Array(response.Signature);
//
// Google Cloud KMS Example:
//   const [signResponse] = await kmsClient.asymmetricSign({
//     name: `projects/.../cryptoKeyVersions/1`,
//     digest: { sha256: sha256(data) },
//   });
//   return new Uint8Array(signResponse.signature);
//
// Azure Key Vault Example:
//   const result = await cryptographyClient.sign("ES256", sha256(data));
//   return new Uint8Array(result.result);

/**
 * Simulated HSM key storage.
 * In production, keys would be stored in the HSM and never leave it.
 */
const HSM_KEYS = {
  // Ed25519 key pair (for demonstration - real HSM keys never leave the device)
  ed25519: {
    privateKey: hexToBytes(
      "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
    ),
    publicKey: hexToBytes(
      "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
    ),
  },
  // AES-256 key for encryption
  aes256: hexToBytes(
    "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
  ),
};

/**
 * Mock HSM signing operation.
 * In production, this would call your HSM SDK.
 */
function hsmSign(
  keyId: string,
  algorithm: string,
  data: Uint8Array
): Uint8Array {
  console.log(`  [HSM] Sign request: algorithm=${algorithm}, keyId=${keyId}`);
  console.log(`  [HSM] Data to sign: ${data.length} bytes`);

  // Use synchronous signing (ed25519.sign is async, but sync version works
  // when sha512Sync is configured)
  const signature = ed25519.sign(data, HSM_KEYS.ed25519.privateKey);

  // Handle both sync and async cases
  if (signature instanceof Promise) {
    throw new Error("Async signing not supported - configure sha512Sync");
  }

  console.log(
    `  [HSM] Signature generated: ${bytesToHex(signature).slice(0, 32)}...`
  );
  return signature;
}

/**
 * Mock HSM verification operation.
 * In production, this would call your HSM SDK or use the public key.
 */
function hsmVerify(
  keyId: string,
  algorithm: string,
  data: Uint8Array,
  signature: Uint8Array
): boolean {
  console.log(`  [HSM] Verify request: algorithm=${algorithm}, keyId=${keyId}`);
  console.log(
    `  [HSM] Data: ${data.length} bytes, Signature: ${signature.length} bytes`
  );

  // Use synchronous verification
  const valid = ed25519.verify(signature, data, HSM_KEYS.ed25519.publicKey);

  // Handle both sync and async cases
  if (valid instanceof Promise) {
    throw new Error("Async verification not supported - configure sha512Sync");
  }

  console.log(`  [HSM] Verification result: ${valid ? "VALID" : "INVALID"}`);
  return valid;
}

/**
 * Mock HSM encryption operation.
 * In production, this would use your HSM's encryption capabilities.
 */
function hsmEncrypt(
  keyId: string,
  algorithm: string,
  nonce: Uint8Array,
  aad: Uint8Array,
  plaintext: Uint8Array
): Uint8Array {
  console.log(`  [HSM] Encrypt request: algorithm=${algorithm}, keyId=${keyId}`);
  console.log(
    `  [HSM] Plaintext: ${plaintext.length} bytes, AAD: ${aad.length} bytes`
  );

  // Use AES-GCM encryption
  const cipher = gcm(HSM_KEYS.aes256, nonce, aad);
  const ciphertext = cipher.encrypt(plaintext);

  console.log(`  [HSM] Ciphertext generated: ${ciphertext.length} bytes`);
  return ciphertext;
}

/**
 * Mock HSM decryption operation.
 * In production, this would use your HSM's decryption capabilities.
 */
function hsmDecrypt(
  keyId: string,
  algorithm: string,
  nonce: Uint8Array,
  aad: Uint8Array,
  ciphertext: Uint8Array
): Uint8Array {
  console.log(`  [HSM] Decrypt request: algorithm=${algorithm}, keyId=${keyId}`);
  console.log(
    `  [HSM] Ciphertext: ${ciphertext.length} bytes, AAD: ${aad.length} bytes`
  );

  const decipher = gcm(HSM_KEYS.aes256, nonce, aad);
  const plaintext = decipher.decrypt(ciphertext);

  console.log(`  [HSM] Plaintext recovered: ${plaintext.length} bytes`);
  return plaintext;
}

// =============================================================================
// Custom Crypto Provider Callbacks
// =============================================================================

/**
 * Custom signer callback that delegates to a simulated HSM.
 *
 * This callback is invoked by the Encoder when it needs to sign the credential.
 * The callback receives the formatted COSE Sig_structure (already hashed if
 * required by the algorithm), and should return the raw signature bytes.
 */
const customSigner: SignerCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  data: Uint8Array
): Uint8Array => {
  // The keyId can be used to select which key to use in your HSM
  // Here we use a simple string identifier
  const keyIdentifier = keyId ? bytesToHex(keyId) : "default-signing-key";

  console.log(`\n[CustomSigner] Signing with HSM`);
  console.log(`  Algorithm: ${algorithm}`);
  console.log(`  Key ID: ${keyIdentifier}`);

  // Delegate to the HSM
  return hsmSign(keyIdentifier, algorithm, data);
};

/**
 * Custom verifier callback that delegates to a simulated HSM.
 *
 * This callback is invoked by the Decoder when verifying a credential signature.
 * It should throw an error if verification fails.
 */
const customVerifier: VerifierCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  data: Uint8Array,
  signature: Uint8Array
): void => {
  const keyIdentifier = keyId ? bytesToHex(keyId) : "default-signing-key";

  console.log(`\n[CustomVerifier] Verifying with HSM`);
  console.log(`  Algorithm: ${algorithm}`);
  console.log(`  Key ID: ${keyIdentifier}`);

  // Delegate to the HSM for verification
  const valid = hsmVerify(keyIdentifier, algorithm, data, signature);

  if (!valid) {
    throw new Error(`Signature verification failed for key: ${keyIdentifier}`);
  }
};

/**
 * Custom encryptor callback that delegates to a simulated HSM.
 *
 * This callback is invoked by the Encoder when encrypting the credential.
 * It receives the nonce, additional authenticated data, and plaintext,
 * and should return the ciphertext with the authentication tag appended.
 */
const customEncryptor: EncryptorCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  nonce: Uint8Array,
  aad: Uint8Array,
  plaintext: Uint8Array
): Uint8Array => {
  const keyIdentifier = keyId ? bytesToHex(keyId) : "default-encryption-key";

  console.log(`\n[CustomEncryptor] Encrypting with HSM`);
  console.log(`  Algorithm: ${algorithm}`);
  console.log(`  Key ID: ${keyIdentifier}`);
  console.log(`  Nonce: ${bytesToHex(nonce)}`);

  return hsmEncrypt(keyIdentifier, algorithm, nonce, aad, plaintext);
};

/**
 * Custom decryptor callback that delegates to a simulated HSM.
 *
 * This callback is invoked by the Decoder when decrypting an encrypted credential.
 * It should return the decrypted plaintext.
 */
const customDecryptor: DecryptorCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  nonce: Uint8Array,
  aad: Uint8Array,
  ciphertext: Uint8Array
): Uint8Array => {
  const keyIdentifier = keyId ? bytesToHex(keyId) : "default-encryption-key";

  console.log(`\n[CustomDecryptor] Decrypting with HSM`);
  console.log(`  Algorithm: ${algorithm}`);
  console.log(`  Key ID: ${keyIdentifier}`);
  console.log(`  Nonce: ${bytesToHex(nonce)}`);

  return hsmDecrypt(keyIdentifier, algorithm, nonce, aad, ciphertext);
};

// =============================================================================
// Test Data
// =============================================================================

function createTestClaim(): Claim169Input {
  return {
    id: "HSM-SIGNED-001",
    fullName: "Alice HSM-Signed",
    firstName: "Alice",
    lastName: "HSM-Signed",
    dateOfBirth: "1990-05-15",
    gender: 2, // Female
    email: "alice@example.com",
    nationality: "US",
  };
}

function createTestCwtMeta(): CwtMetaInput {
  const now = Math.floor(Date.now() / 1000);
  return {
    issuer: "https://hsm-issuer.example.com",
    subject: "HSM-SIGNED-001",
    expiresAt: now + 365 * 24 * 60 * 60, // 1 year
    issuedAt: now,
  };
}

// =============================================================================
// Tests
// =============================================================================

describe("Custom Crypto Provider Examples", () => {
  it("demonstrates custom signer callback (simulating HSM signing)", () => {
    console.log("\n=== Custom Signer Callback Example ===");
    console.log("Simulating HSM-based credential signing\n");

    const claim169 = createTestClaim();
    const cwtMeta = createTestCwtMeta();

    // Encode with custom signer
    // The signWith() method accepts:
    //   - signer: SignerCallback function
    //   - algorithm: "EdDSA" or "ES256"
    const qrData = new Encoder(claim169, cwtMeta)
      .signWith(customSigner, "EdDSA")
      .encode();

    console.log(
      `\nGenerated QR data (${qrData.length} chars): ${qrData.slice(0, 50)}...`
    );

    expect(qrData).toBeTruthy();
    expect(qrData.length).toBeGreaterThan(0);
  });

  it("demonstrates custom verifier callback (simulating HSM verification)", () => {
    console.log("\n=== Custom Verifier Callback Example ===");
    console.log("Simulating HSM-based signature verification\n");

    const claim169 = createTestClaim();
    const cwtMeta = createTestCwtMeta();

    // First, create a signed credential
    const qrData = new Encoder(claim169, cwtMeta)
      .signWith(customSigner, "EdDSA")
      .encode();

    // Decode with custom verifier
    // The verifyWith() method accepts a VerifierCallback function
    const result = new Decoder(qrData).verifyWith(customVerifier).decode();

    console.log(`\nVerification status: ${result.verificationStatus}`);
    console.log(`Decoded ID: ${result.claim169.id}`);
    console.log(`Decoded name: ${result.claim169.fullName}`);

    expect(result.verificationStatus).toBe("verified");
    expect(result.claim169.id).toBe("HSM-SIGNED-001");
    expect(result.claim169.fullName).toBe("Alice HSM-Signed");
  });

  it("demonstrates custom encryptor callback (simulating HSM encryption)", () => {
    console.log("\n=== Custom Encryptor Callback Example ===");
    console.log("Simulating HSM-based credential encryption\n");

    const claim169 = createTestClaim();
    const cwtMeta = createTestCwtMeta();

    // Encode with signing and custom encryption
    // The encryptWith() method accepts:
    //   - encryptor: EncryptorCallback function
    //   - algorithm: "A256GCM" or "A128GCM"
    const qrData = new Encoder(claim169, cwtMeta)
      .signWith(customSigner, "EdDSA")
      .encryptWith(customEncryptor, "A256GCM")
      .encode();

    console.log(
      `\nEncrypted QR data (${qrData.length} chars): ${qrData.slice(0, 50)}...`
    );

    expect(qrData).toBeTruthy();
    expect(qrData.length).toBeGreaterThan(0);
  });

  it("demonstrates custom decryptor callback (simulating HSM decryption)", () => {
    console.log("\n=== Custom Decryptor Callback Example ===");
    console.log("Simulating HSM-based credential decryption\n");

    const claim169 = createTestClaim();
    const cwtMeta = createTestCwtMeta();

    // Create an encrypted credential
    const qrData = new Encoder(claim169, cwtMeta)
      .signWith(customSigner, "EdDSA")
      .encryptWith(customEncryptor, "A256GCM")
      .encode();

    // Decode with custom decryptor and verifier
    // The decryptWith() method accepts a DecryptorCallback function
    const result = new Decoder(qrData)
      .decryptWith(customDecryptor)
      .verifyWith(customVerifier)
      .decode();

    console.log(`\nDecryption and verification completed`);
    console.log(`Verification status: ${result.verificationStatus}`);
    console.log(`Decoded ID: ${result.claim169.id}`);

    expect(result.verificationStatus).toBe("verified");
    expect(result.claim169.id).toBe("HSM-SIGNED-001");
  });

  it("full roundtrip: encode with custom signer -> decode with custom verifier", () => {
    console.log("\n=== Full Roundtrip: Sign & Verify ===");
    console.log("Complete credential lifecycle with custom crypto\n");

    const claim169: Claim169Input = {
      id: "ROUNDTRIP-001",
      fullName: "Bob Roundtrip",
      dateOfBirth: "1985-12-25",
      gender: 1, // Male
      phone: "+1-555-987-6543",
      address: "456 Oak Avenue, Springfield",
    };

    const cwtMeta: CwtMetaInput = {
      issuer: "https://roundtrip.example.com",
      subject: "ROUNDTRIP-001",
    };

    // Step 1: Encode with custom signer
    console.log("Step 1: Encoding credential with custom HSM signer...");
    const qrData = new Encoder(claim169, cwtMeta)
      .signWith(customSigner, "EdDSA")
      .encode();

    console.log(`\nGenerated QR: ${qrData.slice(0, 60)}...`);

    // Step 2: Decode with custom verifier
    console.log("\nStep 2: Decoding credential with custom HSM verifier...");
    const result = new Decoder(qrData).verifyWith(customVerifier).decode();

    // Step 3: Verify the roundtrip
    console.log("\nStep 3: Verifying roundtrip data integrity...");
    expect(result.verificationStatus).toBe("verified");
    expect(result.claim169.id).toBe(claim169.id);
    expect(result.claim169.fullName).toBe(claim169.fullName);
    expect(result.claim169.dateOfBirth).toBe(claim169.dateOfBirth);
    expect(result.claim169.gender).toBe(claim169.gender);
    expect(result.claim169.phone).toBe(claim169.phone);
    expect(result.claim169.address).toBe(claim169.address);
    expect(result.cwtMeta.issuer).toBe(cwtMeta.issuer);
    expect(result.cwtMeta.subject).toBe(cwtMeta.subject);

    console.log("\nRoundtrip successful! All fields match.");
    console.log(`  ID: ${result.claim169.id}`);
    console.log(`  Name: ${result.claim169.fullName}`);
    console.log(`  Issuer: ${result.cwtMeta.issuer}`);
  });

  it("encrypted roundtrip: encode with custom signer + encryptor -> decode with custom decryptor + verifier", () => {
    console.log("\n=== Encrypted Roundtrip: Sign, Encrypt, Decrypt, Verify ===");
    console.log("Complete encrypted credential lifecycle with custom crypto\n");

    const claim169: Claim169Input = {
      id: "ENCRYPTED-ROUNDTRIP-001",
      fullName: "Carol Encrypted",
      dateOfBirth: "1992-07-04",
      gender: 2, // Female
      email: "carol@secure.example.com",
      nationality: "CA",
    };

    const cwtMeta: CwtMetaInput = {
      issuer: "https://secure-vault.example.com",
      subject: "ENCRYPTED-ROUNDTRIP-001",
      expiresAt: Math.floor(Date.now() / 1000) + 86400, // 24 hours
    };

    // Step 1: Encode with custom signer AND custom encryptor
    console.log("Step 1: Encoding with custom HSM signing and encryption...");
    const qrData = new Encoder(claim169, cwtMeta)
      .signWith(customSigner, "EdDSA")
      .encryptWith(customEncryptor, "A256GCM")
      .encode();

    console.log(`\nGenerated encrypted QR: ${qrData.slice(0, 60)}...`);

    // Step 2: Decode with custom decryptor AND custom verifier
    console.log(
      "\nStep 2: Decoding with custom HSM decryption and verification..."
    );
    const result = new Decoder(qrData)
      .decryptWith(customDecryptor)
      .verifyWith(customVerifier)
      .decode();

    // Step 3: Verify the roundtrip
    console.log("\nStep 3: Verifying encrypted roundtrip data integrity...");
    expect(result.verificationStatus).toBe("verified");
    expect(result.claim169.id).toBe(claim169.id);
    expect(result.claim169.fullName).toBe(claim169.fullName);
    expect(result.claim169.dateOfBirth).toBe(claim169.dateOfBirth);
    expect(result.claim169.gender).toBe(claim169.gender);
    expect(result.claim169.email).toBe(claim169.email);
    expect(result.claim169.nationality).toBe(claim169.nationality);
    expect(result.cwtMeta.issuer).toBe(cwtMeta.issuer);
    expect(result.cwtMeta.subject).toBe(cwtMeta.subject);

    console.log("\nEncrypted roundtrip successful! All fields match.");
    console.log(`  ID: ${result.claim169.id}`);
    console.log(`  Name: ${result.claim169.fullName}`);
    console.log(`  Email: ${result.claim169.email}`);
    console.log(`  Issuer: ${result.cwtMeta.issuer}`);
  });

  it("demonstrates verification failure with tampered data", () => {
    console.log("\n=== Verification Failure Example ===");
    console.log("Demonstrating how verification catches tampering\n");

    const claim169 = createTestClaim();
    const cwtMeta = createTestCwtMeta();

    // Create a valid credential
    const qrData = new Encoder(claim169, cwtMeta)
      .signWith(customSigner, "EdDSA")
      .encode();

    // Create a verifier that always fails (simulating wrong key or tampering)
    const failingVerifier: VerifierCallback = (
      algorithm,
      keyId,
      data,
      signature
    ) => {
      console.log("  [FailingVerifier] Intentionally failing verification");
      throw new Error("Signature verification failed: key mismatch");
    };

    // Attempt to decode with the failing verifier
    // Note: The error message is wrapped by the WASM layer as "crypto error: verification callback failed"
    expect(() => {
      new Decoder(qrData).verifyWith(failingVerifier).decode();
    }).toThrow("verification");

    console.log("\nVerification correctly rejected the credential.");
  });
});

// =============================================================================
// Additional Documentation: Real-World Integration Examples
// =============================================================================

/**
 * EXAMPLE: AWS KMS Integration
 *
 * ```typescript
 * import { KMSClient, SignCommand, VerifyCommand } from "@aws-sdk/client-kms";
 *
 * const kmsClient = new KMSClient({ region: "us-east-1" });
 * const KMS_KEY_ID = "arn:aws:kms:us-east-1:123456789:key/abc-123";
 *
 * const awsKmsSigner: SignerCallback = async (algorithm, keyId, data) => {
 *   const response = await kmsClient.send(new SignCommand({
 *     KeyId: KMS_KEY_ID,
 *     Message: data,
 *     MessageType: "RAW",
 *     SigningAlgorithm: algorithm === "ES256" ? "ECDSA_SHA_256" : "...",
 *   }));
 *   return new Uint8Array(response.Signature!);
 * };
 *
 * const awsKmsVerifier: VerifierCallback = async (algorithm, keyId, data, signature) => {
 *   const response = await kmsClient.send(new VerifyCommand({
 *     KeyId: KMS_KEY_ID,
 *     Message: data,
 *     MessageType: "RAW",
 *     Signature: signature,
 *     SigningAlgorithm: "ECDSA_SHA_256",
 *   }));
 *   if (!response.SignatureValid) {
 *     throw new Error("AWS KMS verification failed");
 *   }
 * };
 * ```
 */

/**
 * EXAMPLE: Google Cloud KMS Integration
 *
 * ```typescript
 * import { KeyManagementServiceClient } from "@google-cloud/kms";
 * import { createHash } from "crypto";
 *
 * const kmsClient = new KeyManagementServiceClient();
 * const keyName = "projects/my-project/locations/us/keyRings/my-ring/cryptoKeys/my-key/cryptoKeyVersions/1";
 *
 * const gcpKmsSigner: SignerCallback = async (algorithm, keyId, data) => {
 *   // For ECDSA, GCP KMS requires pre-hashed data
 *   const digest = createHash("sha256").update(data).digest();
 *
 *   const [response] = await kmsClient.asymmetricSign({
 *     name: keyName,
 *     digest: { sha256: digest },
 *   });
 *   return new Uint8Array(response.signature!);
 * };
 * ```
 */

/**
 * EXAMPLE: Azure Key Vault Integration
 *
 * ```typescript
 * import { CryptographyClient } from "@azure/keyvault-keys";
 * import { DefaultAzureCredential } from "@azure/identity";
 *
 * const credential = new DefaultAzureCredential();
 * const keyVaultUrl = "https://my-vault.vault.azure.net";
 * const keyName = "my-signing-key";
 *
 * const cryptoClient = new CryptographyClient(
 *   `${keyVaultUrl}/keys/${keyName}`,
 *   credential
 * );
 *
 * const azureKvSigner: SignerCallback = async (algorithm, keyId, data) => {
 *   const result = await cryptoClient.sign("ES256", data);
 *   return new Uint8Array(result.result);
 * };
 *
 * const azureKvVerifier: VerifierCallback = async (algorithm, keyId, data, signature) => {
 *   const result = await cryptoClient.verify("ES256", data, signature);
 *   if (!result.result) {
 *     throw new Error("Azure Key Vault verification failed");
 *   }
 * };
 * ```
 */
