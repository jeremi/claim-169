import { describe, it, expect } from "vitest";
import * as fs from "node:fs";
import * as path from "node:path";
import { fileURLToPath } from "node:url";
import {
  inspect,
  Encoder,
  Claim169Error,
  hexToBytes,
  type Claim169Input,
  type CwtMetaInput,
  type InspectResult,
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
  expected_claim169?: Record<string, unknown>;
  expected_cwt_meta?: Record<string, unknown>;
  signing_key?: {
    algorithm: string;
    public_key_hex: string;
    private_key_hex?: string;
  };
  encryption_key?: {
    algorithm: string;
    symmetric_key_hex: string;
  };
}

function loadTestVector(category: string, name: string): TestVector {
  const filePath = path.join(TEST_VECTORS_PATH, category, `${name}.json`);
  const content = fs.readFileSync(filePath, "utf-8");
  return JSON.parse(content);
}

describe("inspect", () => {
  describe("signed credentials", () => {
    it("should return metadata from Ed25519 signed credential", () => {
      const vector = loadTestVector("valid", "ed25519-signed");
      const result = inspect(vector.qr_data);

      expect(result.issuer).toBe(vector.expected_cwt_meta?.issuer);
      expect(result.algorithm).toBe("EdDSA");
      expect(result.coseType).toBe("Sign1");
    });

    it("should return expiration timestamp", () => {
      const vector = loadTestVector("valid", "ed25519-signed");
      const result = inspect(vector.qr_data);

      expect(result.expiresAt).toBe(vector.expected_cwt_meta?.expiresAt);
    });

    it("should inspect ECDSA P-256 signed credential", () => {
      const vector = loadTestVector("valid", "ecdsa-p256-signed");
      const result = inspect(vector.qr_data);

      expect(result.issuer).toBe(vector.expected_cwt_meta?.issuer);
      expect(result.algorithm).toBe("ES256");
      expect(result.coseType).toBe("Sign1");
    });
  });

  describe("unsigned credentials", () => {
    it("should inspect unsigned credential", () => {
      const vector = loadTestVector("valid", "minimal");
      const result = inspect(vector.qr_data);

      expect(result.issuer).toBe(vector.expected_cwt_meta?.issuer);
      expect(result.keyId).toBeUndefined();
      expect(result.coseType).toBe("Sign1");
    });

    it("should return subject from CWT claims", () => {
      const vector = loadTestVector("valid", "demographics-full");
      const result = inspect(vector.qr_data);

      if (vector.expected_cwt_meta?.subject) {
        expect(result.subject).toBe(vector.expected_cwt_meta.subject);
      }
    });
  });

  describe("encrypted credentials", () => {
    it("should identify Encrypt0 type for encrypted credentials", () => {
      const vector = loadTestVector("valid", "encrypted-aes256");
      const result = inspect(vector.qr_data);

      expect(result.coseType).toBe("Encrypt0");
    });

    it("should return undefined CWT fields for encrypted credentials", () => {
      const vector = loadTestVector("valid", "encrypted-aes256");
      const result = inspect(vector.qr_data);

      // CWT fields are not accessible for encrypted payloads
      expect(result.issuer).toBeUndefined();
      expect(result.subject).toBeUndefined();
      expect(result.expiresAt).toBeUndefined();
    });

    it("should return algorithm for encrypted credentials", () => {
      const vector = loadTestVector("valid", "encrypted-aes256");
      const result = inspect(vector.qr_data);

      expect(result.algorithm).toBeDefined();
    });
  });

  describe("error handling", () => {
    it("should throw on invalid Base45 input", () => {
      expect(() => inspect("NOT_VALID_BASE45!!!")).toThrow(Claim169Error);
    });

    it("should throw on empty string", () => {
      expect(() => inspect("")).toThrow(Claim169Error);
    });
  });

  describe("roundtrip with encoder", () => {
    it("should inspect a programmatically encoded credential", () => {
      const vector = loadTestVector("valid", "ed25519-signed");
      if (!vector.signing_key?.private_key_hex) {
        throw new Error("Test vector missing private key");
      }

      const privateKey = hexToBytes(vector.signing_key.private_key_hex);

      const claim: Claim169Input = {
        id: "INSPECT-RT-001",
        fullName: "Inspect Roundtrip Person",
      };
      const meta: CwtMetaInput = {
        issuer: "https://inspect.roundtrip.org",
        expiresAt: 1900000000,
      };

      const qrData = new Encoder(claim, meta)
        .signWithEd25519(privateKey)
        .encode().qrData;

      const result = inspect(qrData);

      expect(result.issuer).toBe("https://inspect.roundtrip.org");
      expect(result.algorithm).toBe("EdDSA");
      expect(result.coseType).toBe("Sign1");
      expect(result.expiresAt).toBe(1900000000);
    });

    it("should inspect an unsigned credential", () => {
      const claim: Claim169Input = {
        id: "INSPECT-UNSIGNED-001",
        fullName: "Unsigned Inspect Person",
      };
      const meta: CwtMetaInput = {
        issuer: "https://unsigned.inspect.org",
        expiresAt: 1900000000,
      };

      const qrData = new Encoder(claim, meta).allowUnsigned().encode().qrData;
      const result = inspect(qrData);

      expect(result.issuer).toBe("https://unsigned.inspect.org");
      expect(result.keyId).toBeUndefined();
      expect(result.coseType).toBe("Sign1");
      expect(result.expiresAt).toBe(1900000000);
    });
  });

  describe("x509Headers", () => {
    it("should include x509Headers field", () => {
      const vector = loadTestVector("valid", "minimal");
      const result = inspect(vector.qr_data);

      expect(result.x509Headers).toBeDefined();
    });
  });
});
