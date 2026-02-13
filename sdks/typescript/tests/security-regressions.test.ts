import { describe, expect, test } from "vitest";
import * as fs from "node:fs";
import * as path from "node:path";
import { fileURLToPath } from "node:url";
import { decode, Encoder, Claim169Error, hexToBytes } from "../src/index";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const TEST_VECTORS_PATH = path.join(__dirname, "../../../test-vectors");

interface SigningKey {
  algorithm: string;
  public_key_hex: string;
  private_key_hex?: string;
}

interface TestVector {
  qr_data: string;
  signing_key?: SigningKey;
}

function loadTestVector(category: string, name: string): TestVector {
  const filePath = path.join(TEST_VECTORS_PATH, category, `${name}.json`);
  const content = fs.readFileSync(filePath, "utf-8");
  return JSON.parse(content);
}

describe("security regressions", () => {
  test("timestamp validation is host-side and enabled by default", () => {
    const vector = loadTestVector("valid", "ed25519-signed");
    if (!vector.signing_key?.private_key_hex) {
      throw new Error("Test vector missing private key");
    }

    const privateKey = hexToBytes(vector.signing_key.private_key_hex);
    const publicKey = hexToBytes(vector.signing_key.public_key_hex);

    // Definitely expired (independent of system clock, as long as now > 1).
    const qrData = new Encoder(
      { id: "SECURITY-EXPIRED", fullName: "Expired Credential" },
      { issuer: "https://security.example", issuedAt: 0, expiresAt: 1 }
    )
      .signWithEd25519(privateKey)
      .encode().qrData;

    // Default behavior: timestamp validation happens in JS (host-side), so expired
    // credentials are rejected even though signature verification succeeds.
    expect(() => decode(qrData, { verifyWithEd25519: publicKey })).toThrow(
      Claim169Error
    );
    expect(() => decode(qrData, { verifyWithEd25519: publicKey })).toThrow(
      /expired/i
    );

    // Opt-out: callers can explicitly disable timestamp validation.
    const accepted = decode(qrData, {
      verifyWithEd25519: publicKey,
      validateTimestamps: false,
    });
    expect(accepted.verificationStatus).toBe("verified");
  });
});
