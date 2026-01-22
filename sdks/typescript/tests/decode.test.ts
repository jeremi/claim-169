import { describe, it, expect } from "vitest";
import * as fs from "node:fs";
import * as path from "node:path";
import { fileURLToPath } from "node:url";
import { decode, version, isLoaded, Claim169Error } from "../src/index";

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
  expected_error?: string;
}

// Load test vector file
function loadTestVector(category: string, name: string): TestVector {
  const filePath = path.join(TEST_VECTORS_PATH, category, `${name}.json`);
  const content = fs.readFileSync(filePath, "utf-8");
  return JSON.parse(content);
}

describe("claim169", () => {
  describe("module", () => {
    it("should report as loaded", () => {
      expect(isLoaded()).toBe(true);
    });

    it("should return version", () => {
      const v = version();
      expect(v).toMatch(/^\d+\.\d+\.\d+/);
    });
  });

  describe("decode valid vectors", () => {
    it("should decode minimal vector", () => {
      const vector = loadTestVector("valid", "minimal");
      const result = decode(vector.qr_data);

      expect(result.claim169.id).toBe(vector.expected_claim169?.id);
      expect(result.claim169.fullName).toBe(vector.expected_claim169?.fullName);
      expect(result.verificationStatus).toBe("skipped");
    });

    it("should decode demographics-full vector", () => {
      const vector = loadTestVector("valid", "demographics-full");
      const result = decode(vector.qr_data);

      expect(result.claim169.id).toBe(vector.expected_claim169?.id);
      expect(result.claim169.fullName).toBe(vector.expected_claim169?.fullName);
      if (vector.expected_claim169?.firstName) {
        expect(result.claim169.firstName).toBe(
          vector.expected_claim169.firstName
        );
      }
      if (vector.expected_cwt_meta?.issuer) {
        expect(result.cwtMeta.issuer).toBe(vector.expected_cwt_meta.issuer);
      }
    });

    it("should decode with-face vector", () => {
      const vector = loadTestVector("valid", "with-face");
      const result = decode(vector.qr_data);

      expect(result.claim169.id).toBe(vector.expected_claim169?.id);
      expect(result.claim169.face).toBeDefined();
      expect(result.claim169.face!.length).toBeGreaterThan(0);
    });

    it("should decode with-fingerprints vector", () => {
      const vector = loadTestVector("valid", "with-fingerprints");
      const result = decode(vector.qr_data);

      expect(result.claim169.id).toBe(vector.expected_claim169?.id);
      // Check at least one fingerprint field is present
      const hasFingerprint =
        result.claim169.rightThumb !== undefined ||
        result.claim169.rightPointerFinger !== undefined ||
        result.claim169.leftThumb !== undefined;
      expect(hasFingerprint).toBe(true);
    });

    it("should decode claim169-example vector (from claim_169.md)", () => {
      const vector = loadTestVector("valid", "claim169-example");
      const result = decode(vector.qr_data);

      const expected = vector.expected_claim169!;
      expect(result.claim169.id).toBe(expected.id);
      expect(result.claim169.fullName).toBe(expected.fullName);
      expect(result.claim169.version).toBe(expected.version);
      expect(result.claim169.language).toBe(expected.language);
      expect(result.claim169.dateOfBirth).toBe(expected.dateOfBirth);
      expect(result.claim169.gender).toBe(expected.gender);
      expect(result.claim169.address).toBe(expected.address);
      expect(result.claim169.email).toBe(expected.email);
      expect(result.claim169.phone).toBe(expected.phone);
      expect(result.claim169.nationality).toBe(expected.nationality);
      expect(result.claim169.maritalStatus).toBe(expected.maritalStatus);
      expect(result.claim169.secondaryFullName).toBe(
        expected.secondaryFullName
      );
      expect(result.claim169.secondaryLanguage).toBe(
        expected.secondaryLanguage
      );
      expect(result.claim169.locationCode).toBe(expected.locationCode);
      expect(result.claim169.legalStatus).toBe(expected.legalStatus);
      expect(result.claim169.countryOfIssuance).toBe(
        expected.countryOfIssuance
      );

      // Check face biometric is present
      expect(result.claim169.face).toBeDefined();
      expect(result.claim169.face!.length).toBeGreaterThan(0);
      expect(result.claim169.face![0].format).toBe(
        expected.face[0].format
      );
      expect(result.claim169.face![0].subFormat).toBe(
        expected.face[0].subFormat
      );

      // Check CWT metadata matches example
      const expectedMeta = vector.expected_cwt_meta!;
      expect(result.cwtMeta.issuer).toBe(expectedMeta.issuer);
      expect(result.cwtMeta.issuedAt).toBe(expectedMeta.issuedAt);
      expect(result.cwtMeta.expiresAt).toBe(expectedMeta.expiresAt);
      expect(result.cwtMeta.notBefore).toBe(expectedMeta.notBefore);
    });
  });

  describe("decode invalid vectors", () => {
    it("should reject bad-base45", () => {
      const vector = loadTestVector("invalid", "bad-base45");
      expect(() => decode(vector.qr_data)).toThrow(Claim169Error);
    });

    it("should reject bad-zlib", () => {
      const vector = loadTestVector("invalid", "bad-zlib");
      expect(() => decode(vector.qr_data)).toThrow(Claim169Error);
    });

    it("should reject not-cose", () => {
      const vector = loadTestVector("invalid", "not-cose");
      expect(() => decode(vector.qr_data)).toThrow(Claim169Error);
    });

    it("should reject missing-169", () => {
      const vector = loadTestVector("invalid", "missing-169");
      expect(() => decode(vector.qr_data)).toThrow(Claim169Error);
    });
  });

  describe("decode edge cases", () => {
    it("should handle unknown-fields vector", () => {
      const vector = loadTestVector("edge", "unknown-fields");
      const result = decode(vector.qr_data);

      expect(result.claim169.id).toBe(vector.expected_claim169?.id);
      expect(result.claim169.fullName).toBe(vector.expected_claim169?.fullName);
    });
  });

  describe("decode options", () => {
    it("should skip biometrics when option set", () => {
      const vector = loadTestVector("valid", "with-face");
      const result = decode(vector.qr_data, { skipBiometrics: true });

      expect(result.claim169.id).toBe(vector.expected_claim169?.id);
      expect(result.claim169.face).toBeUndefined();
    });
  });
});
