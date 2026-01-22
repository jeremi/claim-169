import { describe, it, expect } from "vitest";
import * as fs from "node:fs";
import * as path from "node:path";
import { fileURLToPath } from "node:url";
import {
  Encoder,
  Decoder,
  generateNonce,
  Claim169Error,
  type Claim169Input,
  type CwtMetaInput,
} from "../src/index";

// Get directory path for ESM
const __dirname = path.dirname(fileURLToPath(import.meta.url));

// Test vectors path
const TEST_VECTORS_PATH = path.join(__dirname, "../../../test-vectors");

interface TestVector {
  name: string;
  description: string;
  category: string;
  qr_data: string;
  signing_key?: {
    algorithm: string;
    public_key_hex: string;
    private_key_hex?: string;
  };
  encryption_key?: {
    algorithm: string;
    symmetric_key_hex: string;
  };
  expected_claim169?: Record<string, unknown>;
  expected_cwt_meta?: Record<string, unknown>;
}

// Load test vector file
function loadTestVector(category: string, name: string): TestVector {
  const filePath = path.join(TEST_VECTORS_PATH, category, `${name}.json`);
  const content = fs.readFileSync(filePath, "utf-8");
  return JSON.parse(content);
}

// Helper to convert hex string to Uint8Array
function hexToBytes(hex: string): Uint8Array {
  const bytes = new Uint8Array(hex.length / 2);
  for (let i = 0; i < hex.length; i += 2) {
    bytes[i / 2] = parseInt(hex.substr(i, 2), 16);
  }
  return bytes;
}

describe("generateNonce", () => {
  it("should return a Uint8Array", () => {
    const nonce = generateNonce();
    expect(nonce).toBeInstanceOf(Uint8Array);
  });

  it("should return 12 bytes (AES-GCM standard)", () => {
    const nonce = generateNonce();
    expect(nonce.length).toBe(12);
  });

  it("should generate unique nonces", () => {
    const nonces = new Set<string>();
    for (let i = 0; i < 100; i++) {
      const nonce = generateNonce();
      const hex = Array.from(nonce)
        .map((b) => b.toString(16).padStart(2, "0"))
        .join("");
      nonces.add(hex);
    }
    // All 100 nonces should be unique
    expect(nonces.size).toBe(100);
  });
});

describe("Encoder", () => {
  describe("unsigned encoding", () => {
    it("should encode a minimal claim", () => {
      const claim: Claim169Input = {
        id: "ENCODE-TEST-001",
        fullName: "Encode Test Person",
      };
      const meta: CwtMetaInput = {
        issuer: "https://test.example.org",
        issuedAt: 1700000000,
        expiresAt: 1900000000,
      };

      const qrData = new Encoder(claim, meta).allowUnsigned().encode();

      expect(typeof qrData).toBe("string");
      expect(qrData.length).toBeGreaterThan(0);
    });

    it("should roundtrip encode->decode", () => {
      const originalId = "ROUNDTRIP-001";
      const originalName = "Roundtrip Test Person";
      const originalEmail = "roundtrip@test.org";

      const claim: Claim169Input = {
        id: originalId,
        fullName: originalName,
        email: originalEmail,
      };
      const meta: CwtMetaInput = {
        issuer: "https://roundtrip.example.org",
        issuedAt: 1700000000,
        expiresAt: 1900000000,
      };

      const qrData = new Encoder(claim, meta).allowUnsigned().encode();
      const result = new Decoder(qrData).allowUnverified().decode();

      expect(result.claim169.id).toBe(originalId);
      expect(result.claim169.fullName).toBe(originalName);
      expect(result.claim169.email).toBe(originalEmail);
      expect(result.cwtMeta.issuer).toBe("https://roundtrip.example.org");
      expect(result.verificationStatus).toBe("skipped");
    });

    it("should encode with all demographics", () => {
      const claim: Claim169Input = {
        id: "FULL-DEMO-001",
        version: "1.0.0",
        language: "en",
        fullName: "Full Demographics Person",
        firstName: "Full",
        middleName: "Demo",
        lastName: "Person",
        dateOfBirth: "1985-06-15",
        gender: 2, // Female
        address: "456 Demo Avenue, Test City",
        email: "full@demo.org",
        phone: "+1987654321",
        nationality: "CA",
        maritalStatus: 2, // Married
        secondaryFullName: "Nom Complet",
        secondaryLanguage: "fr",
        locationCode: "CA-QC",
        legalStatus: "permanent_resident",
        countryOfIssuance: "CA",
      };
      const meta: CwtMetaInput = {
        issuer: "https://demographics.example.org",
        subject: "demo-subject",
        issuedAt: 1700000000,
        expiresAt: 1900000000,
        notBefore: 1700000000,
      };

      const qrData = new Encoder(claim, meta).allowUnsigned().encode();
      const result = new Decoder(qrData).allowUnverified().decode();

      expect(result.claim169.id).toBe("FULL-DEMO-001");
      expect(result.claim169.version).toBe("1.0.0");
      expect(result.claim169.language).toBe("en");
      expect(result.claim169.fullName).toBe("Full Demographics Person");
      expect(result.claim169.firstName).toBe("Full");
      expect(result.claim169.middleName).toBe("Demo");
      expect(result.claim169.lastName).toBe("Person");
      expect(result.claim169.dateOfBirth).toBe("1985-06-15");
      expect(result.claim169.gender).toBe(2);
      expect(result.claim169.address).toBe("456 Demo Avenue, Test City");
      expect(result.claim169.email).toBe("full@demo.org");
      expect(result.claim169.phone).toBe("+1987654321");
      expect(result.claim169.nationality).toBe("CA");
      expect(result.claim169.maritalStatus).toBe(2);
      expect(result.claim169.secondaryFullName).toBe("Nom Complet");
      expect(result.claim169.secondaryLanguage).toBe("fr");
      expect(result.claim169.locationCode).toBe("CA-QC");
      expect(result.claim169.legalStatus).toBe("permanent_resident");
      expect(result.claim169.countryOfIssuance).toBe("CA");
    });

    it("should encode with empty optional fields", () => {
      const claim: Claim169Input = {};
      const meta: CwtMetaInput = { expiresAt: 1900000000 };

      const qrData = new Encoder(claim, meta).allowUnsigned().encode();
      const result = new Decoder(qrData).allowUnverified().decode();

      expect(result.claim169.id).toBeUndefined();
      expect(result.claim169.fullName).toBeUndefined();
      expect(result.claim169.email).toBeUndefined();
    });

    it("should handle Unicode in fields", () => {
      const unicodeName = "日本語テスト";
      const unicodeAddress = "东京都渋谷区";

      const claim: Claim169Input = {
        id: "UNICODE-001",
        fullName: unicodeName,
        address: unicodeAddress,
      };
      const meta: CwtMetaInput = { expiresAt: 1900000000 };

      const qrData = new Encoder(claim, meta).allowUnsigned().encode();
      const result = new Decoder(qrData).allowUnverified().decode();

      expect(result.claim169.fullName).toBe(unicodeName);
      expect(result.claim169.address).toBe(unicodeAddress);
    });

    it("should handle large field values", () => {
      const longName = "A".repeat(200);
      const longAddress = "B".repeat(500);

      const claim: Claim169Input = {
        id: "LARGE-001",
        fullName: longName,
        address: longAddress,
      };
      const meta: CwtMetaInput = { expiresAt: 1900000000 };

      const qrData = new Encoder(claim, meta).allowUnsigned().encode();
      const result = new Decoder(qrData).allowUnverified().decode();

      expect(result.claim169.fullName).toBe(longName);
      expect(result.claim169.address).toBe(longAddress);
    });

    it("should handle special characters in fields", () => {
      const specialEmail = "test+special@example.org";
      const specialPhone = "+1 (234) 567-8900";

      const claim: Claim169Input = {
        id: "SPECIAL-001",
        fullName: "O'Brien-Smith",
        email: specialEmail,
        phone: specialPhone,
      };
      const meta: CwtMetaInput = { expiresAt: 1900000000 };

      const qrData = new Encoder(claim, meta).allowUnsigned().encode();
      const result = new Decoder(qrData).allowUnverified().decode();

      expect(result.claim169.fullName).toBe("O'Brien-Smith");
      expect(result.claim169.email).toBe(specialEmail);
      expect(result.claim169.phone).toBe(specialPhone);
    });
  });

  describe("Ed25519 signed encoding", () => {
    it("should encode and be decodable", () => {
      const vector = loadTestVector("valid", "ed25519-signed");
      if (!vector.signing_key?.private_key_hex) {
        throw new Error("Test vector missing private key");
      }

      const privateKey = hexToBytes(vector.signing_key.private_key_hex);

      const claim: Claim169Input = {
        id: "ED25519-TEST-001",
        fullName: "Ed25519 Signed Person",
      };
      const meta: CwtMetaInput = {
        issuer: "https://ed25519.example.org",
        issuedAt: 1700000000,
        expiresAt: 1900000000,
      };

      const qrData = new Encoder(claim, meta)
        .signWithEd25519(privateKey)
        .encode();

      // Verify it can be decoded
      const result = new Decoder(qrData).allowUnverified().decode();
      expect(result.claim169.id).toBe("ED25519-TEST-001");
      expect(result.claim169.fullName).toBe("Ed25519 Signed Person");
    });

    it("should reject invalid key length", () => {
      const claim: Claim169Input = { id: "TEST", fullName: "Test" };
      const meta: CwtMetaInput = {};

      expect(() => {
        new Encoder(claim, meta)
          .signWithEd25519(new Uint8Array(16)) // Too short
          .encode();
      }).toThrow(Claim169Error);
    });
  });

  describe("ECDSA P-256 signed encoding", () => {
    it("should encode and be decodable", () => {
      // Use a deterministic test key (random bytes, 32 bytes for private scalar)
      const privateKey = new Uint8Array(32);
      for (let i = 0; i < 32; i++) {
        privateKey[i] = (i * 7 + 13) % 256; // Deterministic non-zero key
      }

      const claim: Claim169Input = {
        id: "ECDSA-TEST-001",
        fullName: "ECDSA Signed Person",
      };
      const meta: CwtMetaInput = {
        issuer: "https://ecdsa.example.org",
        issuedAt: 1700000000,
        expiresAt: 1900000000,
      };

      const qrData = new Encoder(claim, meta)
        .signWithEcdsaP256(privateKey)
        .encode();

      // Verify it can be decoded
      const result = new Decoder(qrData).allowUnverified().decode();
      expect(result.claim169.id).toBe("ECDSA-TEST-001");
      expect(result.claim169.fullName).toBe("ECDSA Signed Person");
    });

    it("should reject invalid key length", () => {
      const claim: Claim169Input = { id: "TEST", fullName: "Test" };
      const meta: CwtMetaInput = {};

      expect(() => {
        new Encoder(claim, meta)
          .signWithEcdsaP256(new Uint8Array(16)) // Too short
          .encode();
      }).toThrow(Claim169Error);
    });
  });

  describe("encrypted encoding", () => {
    it("should encode with AES-256 encryption", () => {
      const vector = loadTestVector("valid", "encrypted-signed");
      if (!vector.signing_key?.private_key_hex) {
        throw new Error("Test vector missing private key");
      }
      if (!vector.encryption_key?.symmetric_key_hex) {
        throw new Error("Test vector missing encryption key");
      }

      const signKey = hexToBytes(vector.signing_key.private_key_hex);
      const encryptKey = hexToBytes(vector.encryption_key.symmetric_key_hex);

      const claim: Claim169Input = {
        id: "ENC-TEST-001",
        fullName: "Encrypted Person",
      };
      const meta: CwtMetaInput = {
        issuer: "https://encrypted.example.org",
        issuedAt: 1700000000,
        expiresAt: 1900000000,
      };

      const qrData = new Encoder(claim, meta)
        .signWithEd25519(signKey)
        .encryptWithAes256(encryptKey)
        .encode();

      expect(typeof qrData).toBe("string");
      expect(qrData.length).toBeGreaterThan(0);
      // Note: Cannot decode encrypted payloads without decryption support in WASM
    });

    it("should encode with AES-128 encryption", () => {
      const vector = loadTestVector("valid", "ed25519-signed");
      if (!vector.signing_key?.private_key_hex) {
        throw new Error("Test vector missing private key");
      }

      const signKey = hexToBytes(vector.signing_key.private_key_hex);
      // 16-byte AES-128 key
      const encryptKey = new Uint8Array(16);
      for (let i = 0; i < 16; i++) {
        encryptKey[i] = (i * 11 + 5) % 256;
      }

      const claim: Claim169Input = {
        id: "AES128-TEST-001",
        fullName: "AES-128 Encrypted Person",
      };
      const meta: CwtMetaInput = {
        issuer: "https://aes128.example.org",
        issuedAt: 1700000000,
        expiresAt: 1900000000,
      };

      const qrData = new Encoder(claim, meta)
        .signWithEd25519(signKey)
        .encryptWithAes128(encryptKey)
        .encode();

      expect(typeof qrData).toBe("string");
      expect(qrData.length).toBeGreaterThan(0);
    });

    it("should reject invalid AES-256 key length", () => {
      const claim: Claim169Input = { id: "TEST", fullName: "Test" };
      const meta: CwtMetaInput = {};

      expect(() => {
        new Encoder(claim, meta)
          .allowUnsigned()
          .encryptWithAes256(new Uint8Array(16)) // Should be 32
          .encode();
      }).toThrow(Claim169Error);
    });

    it("should reject invalid AES-128 key length", () => {
      const claim: Claim169Input = { id: "TEST", fullName: "Test" };
      const meta: CwtMetaInput = {};

      expect(() => {
        new Encoder(claim, meta)
          .allowUnsigned()
          .encryptWithAes128(new Uint8Array(32)) // Should be 16
          .encode();
      }).toThrow(Claim169Error);
    });
  });

  describe("error handling", () => {
    it("should throw if no signing method specified", () => {
      const claim: Claim169Input = { id: "TEST", fullName: "Test" };
      const meta: CwtMetaInput = {};

      expect(() => {
        new Encoder(claim, meta).encode();
      }).toThrow(Claim169Error);
    });

    it("should throw Claim169Error with message", () => {
      const claim: Claim169Input = { id: "TEST", fullName: "Test" };
      const meta: CwtMetaInput = {};

      try {
        new Encoder(claim, meta).encode();
        expect.fail("Should have thrown");
      } catch (error) {
        expect(error).toBeInstanceOf(Claim169Error);
        expect((error as Claim169Error).message).toContain("sign");
      }
    });
  });

  describe("builder pattern", () => {
    it("should support method chaining", () => {
      const vector = loadTestVector("valid", "ed25519-signed");
      if (!vector.signing_key?.private_key_hex) {
        throw new Error("Test vector missing private key");
      }

      const privateKey = hexToBytes(vector.signing_key.private_key_hex);
      const encryptKey = new Uint8Array(32);
      for (let i = 0; i < 32; i++) {
        encryptKey[i] = (i * 3 + 7) % 256;
      }

      const claim: Claim169Input = {
        id: "CHAIN-TEST-001",
        fullName: "Chain Test Person",
      };
      const meta: CwtMetaInput = { expiresAt: 1900000000 };

      // All methods should return the encoder for chaining
      const qrData = new Encoder(claim, meta)
        .signWithEd25519(privateKey)
        .skipBiometrics()
        .encryptWithAes256(encryptKey)
        .encode();

      expect(typeof qrData).toBe("string");
    });
  });
});

describe("Decoder builder", () => {
  it("should work with builder pattern", () => {
    const vector = loadTestVector("valid", "minimal");
    const result = new Decoder(vector.qr_data).allowUnverified().decode();

    expect(result.claim169.id).toBe(vector.expected_claim169?.id);
    expect(result.claim169.fullName).toBe(vector.expected_claim169?.fullName);
  });

  it("should support skipBiometrics", () => {
    const vector = loadTestVector("valid", "with-face");
    const result = new Decoder(vector.qr_data).allowUnverified().skipBiometrics().decode();

    expect(result.claim169.id).toBe(vector.expected_claim169?.id);
    expect(result.claim169.face).toBeUndefined();
  });

  it("should support method chaining", () => {
    const vector = loadTestVector("valid", "minimal");
    const result = new Decoder(vector.qr_data)
      .allowUnverified()
      .skipBiometrics()
      .maxDecompressedBytes(65536)
      .decode();

    expect(result.claim169.id).toBe(vector.expected_claim169?.id);
  });
});
