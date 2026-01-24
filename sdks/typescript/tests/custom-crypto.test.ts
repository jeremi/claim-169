/**
 * Tests for custom crypto provider callbacks (signer, verifier, encryptor, decryptor).
 */

import { describe, it, expect } from "vitest";
import {
  Encoder,
  Decoder,
  Claim169Error,
  type Claim169Input,
  type CwtMetaInput,
  type SignerCallback,
  type VerifierCallback,
  type EncryptorCallback,
  type DecryptorCallback,
} from "../src/index";

// Simple in-memory "crypto provider" for testing
// Uses Ed25519 private key bytes to derive signature via XOR (not real crypto!)
// This simulates what a real HSM or cloud KMS would do - receive data, return signature.

describe("Custom Crypto Providers", () => {
  // Create a simple mock key for testing
  const mockKey = new Uint8Array(32);
  for (let i = 0; i < 32; i++) {
    mockKey[i] = (i * 7 + 13) % 256;
  }

  describe("Custom Signer", () => {
    it("should encode with custom signer callback for EdDSA", () => {
      // Track that our callback was called
      let signerCalled = false;
      let receivedAlgorithm: string | null = null;

      // Simple mock signer that returns a 64-byte "signature"
      const mySigner: SignerCallback = (algorithm, keyId, data) => {
        signerCalled = true;
        receivedAlgorithm = algorithm;
        // Return a mock 64-byte signature (EdDSA signature size)
        const sig = new Uint8Array(64);
        for (let i = 0; i < 64; i++) {
          sig[i] = (data[i % data.length] ^ mockKey[i % 32]) % 256;
        }
        return sig;
      };

      const claim: Claim169Input = {
        id: "CUSTOM-SIGNER-001",
        fullName: "Custom Signer Test",
      };
      const meta: CwtMetaInput = {
        issuer: "https://custom-signer.example",
        expiresAt: 1900000000,
      };

      const qrData = new Encoder(claim, meta).signWith(mySigner, "EdDSA").encode();

      expect(signerCalled).toBe(true);
      expect(receivedAlgorithm).toBe("EdDSA");
      expect(typeof qrData).toBe("string");
      expect(qrData.length).toBeGreaterThan(0);

      // Decode (allow unverified since it's a mock signature)
      const result = new Decoder(qrData).allowUnverified().decode();
      expect(result.claim169.id).toBe("CUSTOM-SIGNER-001");
      expect(result.claim169.fullName).toBe("Custom Signer Test");
    });

    it("should encode with custom signer callback for ES256", () => {
      let receivedAlgorithm: string | null = null;

      // Simple mock signer that returns a 64-byte "signature"
      const mySigner: SignerCallback = (algorithm, keyId, data) => {
        receivedAlgorithm = algorithm;
        // Return a mock 64-byte signature (ES256 signature size is r||s = 32+32)
        const sig = new Uint8Array(64);
        for (let i = 0; i < 64; i++) {
          sig[i] = (data[i % data.length] ^ mockKey[i % 32]) % 256;
        }
        return sig;
      };

      const claim: Claim169Input = {
        id: "CUSTOM-SIGNER-ES256",
        fullName: "ES256 Signer Test",
      };
      const meta: CwtMetaInput = {
        issuer: "https://es256.example",
        expiresAt: 1900000000,
      };

      const qrData = new Encoder(claim, meta).signWith(mySigner, "ES256").encode();

      expect(receivedAlgorithm).toBe("ES256");
      expect(typeof qrData).toBe("string");

      // Decode (allow unverified since it's a mock signature)
      const result = new Decoder(qrData).allowUnverified().decode();
      expect(result.claim169.id).toBe("CUSTOM-SIGNER-ES256");
    });

    it("should pass keyId to signer callback when set", () => {
      let receivedKeyId: Uint8Array | null = null;

      const mySigner: SignerCallback = (algorithm, keyId, data) => {
        receivedKeyId = keyId;
        return new Uint8Array(64); // Minimal mock signature
      };

      const claim: Claim169Input = {
        id: "KEY-ID-TEST",
        fullName: "Key ID Test",
      };
      const meta: CwtMetaInput = { expiresAt: 1900000000 };

      const expectedKeyId = new Uint8Array([1, 2, 3, 4, 5]);
      const qrData = new Encoder(claim, meta)
        .signWith(mySigner, "EdDSA", expectedKeyId)
        .encode();

      expect(typeof qrData).toBe("string");
      expect(receivedKeyId).not.toBeNull();
      expect(Array.from(receivedKeyId!)).toEqual(Array.from(expectedKeyId));
    });

    it("should propagate signer callback errors", () => {
      const failingSigner: SignerCallback = (algorithm, keyId, data) => {
        throw new Error("HSM not available");
      };

      const claim: Claim169Input = { id: "FAIL", fullName: "Fail Test" };
      const meta: CwtMetaInput = { expiresAt: 1900000000 };

      expect(() => {
        new Encoder(claim, meta).signWith(failingSigner, "EdDSA").encode();
      }).toThrow();
    });
  });

  describe("Custom Verifier", () => {
    it("should decode with custom verifier callback", () => {
      // First encode with a regular signer
      const privateKey = new Uint8Array(32);
      for (let i = 0; i < 32; i++) {
        privateKey[i] = (i * 7 + 13) % 256;
      }

      const claim: Claim169Input = {
        id: "CUSTOM-VERIFIER-001",
        fullName: "Custom Verifier Test",
      };
      const meta: CwtMetaInput = {
        issuer: "https://verifier.example",
        expiresAt: 1900000000,
      };

      const qrData = new Encoder(claim, meta).signWithEd25519(privateKey).encode();

      // Now decode with a custom verifier that always passes
      let verifierCalled = false;
      let receivedAlgorithm: string | null = null;

      const myVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
        verifierCalled = true;
        receivedAlgorithm = algorithm;
        // Accept any signature (for testing purposes)
      };

      const result = new Decoder(qrData).verifyWith(myVerifier).decode();

      expect(verifierCalled).toBe(true);
      expect(receivedAlgorithm).toBe("EdDSA");
      expect(result.claim169.id).toBe("CUSTOM-VERIFIER-001");
      expect(result.verificationStatus).toBe("verified");
    });

    it("should propagate verifier callback errors as verification failure", () => {
      // Encode a credential
      const privateKey = new Uint8Array(32);
      for (let i = 0; i < 32; i++) {
        privateKey[i] = (i * 7 + 13) % 256;
      }

      const claim: Claim169Input = {
        id: "FAIL-VERIFY",
        fullName: "Fail Verify Test",
      };
      const meta: CwtMetaInput = { expiresAt: 1900000000 };

      const qrData = new Encoder(claim, meta).signWithEd25519(privateKey).encode();

      // Verifier that always throws
      const failingVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
        throw new Error("Invalid signature");
      };

      expect(() => {
        new Decoder(qrData).verifyWith(failingVerifier).decode();
      }).toThrow();
    });
  });

  describe("Custom Encryptor", () => {
    it("should encode with custom encryptor callback for A256GCM", () => {
      let encryptorCalled = false;
      let receivedAlgorithm: string | null = null;

      // Mock encryptor that returns ciphertext with auth tag
      const myEncryptor: EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => {
        encryptorCalled = true;
        receivedAlgorithm = algorithm;
        // Simple XOR "encryption" + 16-byte "auth tag" for testing
        const ciphertext = new Uint8Array(plaintext.length + 16);
        for (let i = 0; i < plaintext.length; i++) {
          ciphertext[i] = plaintext[i] ^ mockKey[i % 32];
        }
        // Add fake auth tag
        for (let i = 0; i < 16; i++) {
          ciphertext[plaintext.length + i] = (nonce[i % nonce.length] ^ mockKey[i]) % 256;
        }
        return ciphertext;
      };

      const claim: Claim169Input = {
        id: "CUSTOM-ENCRYPTOR-001",
        fullName: "Custom Encryptor Test",
      };
      const meta: CwtMetaInput = {
        issuer: "https://encryptor.example",
        expiresAt: 1900000000,
      };

      // Need a signer first
      const privateKey = new Uint8Array(32);
      for (let i = 0; i < 32; i++) {
        privateKey[i] = (i * 7 + 13) % 256;
      }

      const qrData = new Encoder(claim, meta)
        .signWithEd25519(privateKey)
        .encryptWith(myEncryptor, "A256GCM")
        .encode();

      expect(encryptorCalled).toBe(true);
      expect(receivedAlgorithm).toBe("A256GCM");
      expect(typeof qrData).toBe("string");
      expect(qrData.length).toBeGreaterThan(0);
    });

    it("should encode with custom encryptor callback for A128GCM", () => {
      let receivedAlgorithm: string | null = null;

      const myEncryptor: EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => {
        receivedAlgorithm = algorithm;
        const ciphertext = new Uint8Array(plaintext.length + 16);
        for (let i = 0; i < plaintext.length; i++) {
          ciphertext[i] = plaintext[i] ^ mockKey[i % 16];
        }
        for (let i = 0; i < 16; i++) {
          ciphertext[plaintext.length + i] = nonce[i % nonce.length];
        }
        return ciphertext;
      };

      const claim: Claim169Input = {
        id: "CUSTOM-ENCRYPTOR-A128",
        fullName: "AES-128 Encryptor Test",
      };
      const meta: CwtMetaInput = { expiresAt: 1900000000 };

      const privateKey = new Uint8Array(32);
      for (let i = 0; i < 32; i++) {
        privateKey[i] = (i * 7 + 13) % 256;
      }

      const qrData = new Encoder(claim, meta)
        .signWithEd25519(privateKey)
        .encryptWith(myEncryptor, "A128GCM")
        .encode();

      expect(receivedAlgorithm).toBe("A128GCM");
      expect(typeof qrData).toBe("string");
    });
  });

  describe("Custom Decryptor", () => {
    it("should decode with custom decryptor callback", () => {
      // First create an encrypted credential with standard encryption
      const privateKey = new Uint8Array(32);
      const encryptKey = new Uint8Array(32);
      for (let i = 0; i < 32; i++) {
        privateKey[i] = (i * 7 + 13) % 256;
        encryptKey[i] = (i * 11 + 5) % 256;
      }

      const claim: Claim169Input = {
        id: "CUSTOM-DECRYPTOR-001",
        fullName: "Custom Decryptor Test",
      };
      const meta: CwtMetaInput = {
        issuer: "https://decryptor.example",
        expiresAt: 1900000000,
      };

      const qrData = new Encoder(claim, meta)
        .signWithEd25519(privateKey)
        .encryptWithAes256(encryptKey)
        .encode();

      // Now decode with custom decryptor that delegates to our known key
      let decryptorCalled = false;
      let receivedAlgorithm: string | null = null;

      // This is a mock decryptor - in practice it would call an HSM or cloud KMS
      // For testing, we'll just decode with the standard key and verify the callback is invoked
      const myDecryptor: DecryptorCallback = (algorithm, keyId, nonce, aad, ciphertext) => {
        decryptorCalled = true;
        receivedAlgorithm = algorithm;
        // We can't actually decrypt here without implementing AES-GCM,
        // but we can verify the callback is invoked with correct parameters
        throw new Error("Mock decryptor - use standard decode for actual decryption");
      };

      // This will fail because our mock decryptor throws
      expect(() => {
        new Decoder(qrData).decryptWith(myDecryptor).allowUnverified().decode();
      }).toThrow();

      expect(decryptorCalled).toBe(true);
      expect(receivedAlgorithm).toBe("A256GCM");
    });
  });

  describe("Roundtrip with custom signer and verifier", () => {
    it("should roundtrip encode with custom signer and decode with custom verifier", () => {
      // This test simulates what a real HSM integration would look like:
      // - Custom signer that calls the HSM to sign
      // - Custom verifier that calls the HSM to verify

      let signedData: Uint8Array | null = null;
      let signatureProduced: Uint8Array | null = null;

      // Custom signer stores data and produces a deterministic signature
      const mySigner: SignerCallback = (algorithm, keyId, data) => {
        signedData = new Uint8Array(data);
        // Produce a deterministic 64-byte signature
        const sig = new Uint8Array(64);
        for (let i = 0; i < 64; i++) {
          sig[i] = (data[i % data.length] + mockKey[i % 32]) % 256;
        }
        signatureProduced = new Uint8Array(sig);
        return sig;
      };

      // Custom verifier checks that signature matches what we produced
      const myVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
        // Recompute expected signature
        const expected = new Uint8Array(64);
        for (let i = 0; i < 64; i++) {
          expected[i] = (data[i % data.length] + mockKey[i % 32]) % 256;
        }
        // Compare
        for (let i = 0; i < 64; i++) {
          if (signature[i] !== expected[i]) {
            throw new Error("Signature mismatch");
          }
        }
        // Verification passed
      };

      const claim: Claim169Input = {
        id: "ROUNDTRIP-CUSTOM-001",
        fullName: "Roundtrip Custom Crypto",
        email: "roundtrip@custom.example",
        gender: 1,
      };
      const meta: CwtMetaInput = {
        issuer: "https://roundtrip-custom.example",
        expiresAt: 1900000000,
      };

      // Encode with custom signer
      const qrData = new Encoder(claim, meta).signWith(mySigner, "EdDSA").encode();

      expect(signedData).not.toBeNull();
      expect(signatureProduced).not.toBeNull();

      // Decode with custom verifier
      const result = new Decoder(qrData).verifyWith(myVerifier).decode();

      expect(result.claim169.id).toBe("ROUNDTRIP-CUSTOM-001");
      expect(result.claim169.fullName).toBe("Roundtrip Custom Crypto");
      expect(result.claim169.email).toBe("roundtrip@custom.example");
      expect(result.claim169.gender).toBe(1);
      expect(result.cwtMeta.issuer).toBe("https://roundtrip-custom.example");
      expect(result.verificationStatus).toBe("verified");
    });
  });

  describe("Combined custom signer and encryptor", () => {
    it("should encode with both custom signer and encryptor", () => {
      let signerCalled = false;
      let encryptorCalled = false;

      const mySigner: SignerCallback = (algorithm, keyId, data) => {
        signerCalled = true;
        // Return mock signature
        const sig = new Uint8Array(64);
        for (let i = 0; i < 64; i++) {
          sig[i] = data[i % data.length] ^ mockKey[i % 32];
        }
        return sig;
      };

      const myEncryptor: EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => {
        encryptorCalled = true;
        // Return mock ciphertext with auth tag
        const ciphertext = new Uint8Array(plaintext.length + 16);
        for (let i = 0; i < plaintext.length; i++) {
          ciphertext[i] = plaintext[i] ^ mockKey[i % 32];
        }
        for (let i = 0; i < 16; i++) {
          ciphertext[plaintext.length + i] = nonce[i % nonce.length];
        }
        return ciphertext;
      };

      const claim: Claim169Input = {
        id: "COMBINED-CUSTOM-001",
        fullName: "Combined Custom Crypto",
      };
      const meta: CwtMetaInput = {
        issuer: "https://combined.example",
        expiresAt: 1900000000,
      };

      const qrData = new Encoder(claim, meta)
        .signWith(mySigner, "EdDSA")
        .encryptWith(myEncryptor, "A256GCM")
        .encode();

      expect(signerCalled).toBe(true);
      expect(encryptorCalled).toBe(true);
      expect(typeof qrData).toBe("string");
      expect(qrData.length).toBeGreaterThan(0);
    });
  });

  describe("Error handling", () => {
    it("should propagate signer exceptions", () => {
      const errorMessage = "HSM connection failed";
      const failingSigner: SignerCallback = (algorithm, keyId, data) => {
        throw new Error(errorMessage);
      };

      const claim: Claim169Input = { id: "FAIL", fullName: "Fail Test" };
      const meta: CwtMetaInput = { expiresAt: 1900000000 };

      expect(() => {
        new Encoder(claim, meta).signWith(failingSigner, "EdDSA").encode();
      }).toThrow();
    });

    it("should propagate encryptor exceptions", () => {
      const errorMessage = "KMS encryption failed";
      const failingEncryptor: EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => {
        throw new Error(errorMessage);
      };

      const privateKey = new Uint8Array(32);
      for (let i = 0; i < 32; i++) {
        privateKey[i] = i;
      }

      const claim: Claim169Input = { id: "FAIL", fullName: "Fail Test" };
      const meta: CwtMetaInput = { expiresAt: 1900000000 };

      expect(() => {
        new Encoder(claim, meta)
          .signWithEd25519(privateKey)
          .encryptWith(failingEncryptor, "A256GCM")
          .encode();
      }).toThrow();
    });
  });
});
