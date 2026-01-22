/**
 * MOSIP Claim 169 QR Code decoder library
 *
 * @example
 * ```typescript
 * import { decode } from 'claim169';
 *
 * const result = decode(qrText);
 * console.log(result.claim169.fullName);
 * ```
 */

export type {
  Biometric,
  Claim169,
  CwtMeta,
  DecodeOptions,
  DecodeResult,
  VerificationStatus,
} from "./types.js";

export { Claim169Error } from "./types.js";

import type { DecodeOptions, DecodeResult } from "./types.js";
import { Claim169Error } from "./types.js";

// Import WASM module (auto-initialized with bundler target)
import * as wasm from "../wasm/claim169_wasm.js";

/**
 * Get the library version
 */
export function version(): string {
  return wasm.version();
}

/**
 * Check if the WASM module is loaded correctly
 */
export function isLoaded(): boolean {
  return wasm.isLoaded();
}

/**
 * Decode a Claim 169 QR code without signature verification.
 *
 * @param qrText - The QR code text content (Base45 encoded)
 * @param options - Optional decode options
 * @returns The decoded result
 * @throws {Claim169Error} If decoding fails
 *
 * @example
 * ```typescript
 * const result = decode(qrText);
 * console.log(result.claim169.fullName);
 * console.log(result.cwtMeta.issuer);
 * ```
 */
export function decode(qrText: string, options?: DecodeOptions): DecodeResult {
  try {
    if (options) {
      let wasmOptions = new wasm.WasmDecodeOptions();

      if (options.maxDecompressedBytes !== undefined) {
        wasmOptions = wasmOptions.setMaxDecompressedBytes(
          options.maxDecompressedBytes
        );
      }
      if (options.skipBiometrics !== undefined) {
        wasmOptions = wasmOptions.setSkipBiometrics(options.skipBiometrics);
      }
      if (options.validateTimestamps !== undefined) {
        wasmOptions = wasmOptions.setValidateTimestamps(
          options.validateTimestamps
        );
      }
      if (options.clockSkewToleranceSeconds !== undefined) {
        wasmOptions = wasmOptions.setClockSkewToleranceSeconds(
          options.clockSkewToleranceSeconds
        );
      }

      const result = wasm.decodeWithOptions(qrText, wasmOptions);
      return transformResult(result);
    } else {
      const result = wasm.decode(qrText);
      return transformResult(result);
    }
  } catch (error) {
    if (error instanceof Error) {
      throw new Claim169Error(error.message);
    }
    throw new Claim169Error(String(error));
  }
}

/**
 * Transform raw WASM result to typed DecodeResult
 */
function transformResult(raw: unknown): DecodeResult {
  const result = raw as {
    claim169: Record<string, unknown>;
    cwtMeta: Record<string, unknown>;
    verificationStatus: string;
  };

  return {
    claim169: transformClaim169(result.claim169),
    cwtMeta: {
      issuer: result.cwtMeta.issuer as string | undefined,
      subject: result.cwtMeta.subject as string | undefined,
      expiresAt: result.cwtMeta.expiresAt as number | undefined,
      notBefore: result.cwtMeta.notBefore as number | undefined,
      issuedAt: result.cwtMeta.issuedAt as number | undefined,
    },
    verificationStatus: result.verificationStatus as
      | "verified"
      | "skipped"
      | "failed",
  };
}

/**
 * Transform raw claim169 object to typed Claim169
 */
function transformClaim169(
  raw: Record<string, unknown>
): import("./types.js").Claim169 {
  return {
    id: raw.id as string | undefined,
    version: raw.version as string | undefined,
    language: raw.language as string | undefined,
    fullName: raw.fullName as string | undefined,
    firstName: raw.firstName as string | undefined,
    middleName: raw.middleName as string | undefined,
    lastName: raw.lastName as string | undefined,
    dateOfBirth: raw.dateOfBirth as string | undefined,
    gender: raw.gender as number | undefined,
    address: raw.address as string | undefined,
    email: raw.email as string | undefined,
    phone: raw.phone as string | undefined,
    nationality: raw.nationality as string | undefined,
    maritalStatus: raw.maritalStatus as number | undefined,
    guardian: raw.guardian as string | undefined,
    photo: raw.photo as Uint8Array | undefined,
    photoFormat: raw.photoFormat as number | undefined,
    bestQualityFingers: raw.bestQualityFingers as Uint8Array | undefined,
    secondaryFullName: raw.secondaryFullName as string | undefined,
    secondaryLanguage: raw.secondaryLanguage as string | undefined,
    locationCode: raw.locationCode as string | undefined,
    legalStatus: raw.legalStatus as string | undefined,
    countryOfIssuance: raw.countryOfIssuance as string | undefined,
    rightThumb: transformBiometrics(raw.rightThumb),
    rightPointerFinger: transformBiometrics(raw.rightPointerFinger),
    rightMiddleFinger: transformBiometrics(raw.rightMiddleFinger),
    rightRingFinger: transformBiometrics(raw.rightRingFinger),
    rightLittleFinger: transformBiometrics(raw.rightLittleFinger),
    leftThumb: transformBiometrics(raw.leftThumb),
    leftPointerFinger: transformBiometrics(raw.leftPointerFinger),
    leftMiddleFinger: transformBiometrics(raw.leftMiddleFinger),
    leftRingFinger: transformBiometrics(raw.leftRingFinger),
    leftLittleFinger: transformBiometrics(raw.leftLittleFinger),
    rightIris: transformBiometrics(raw.rightIris),
    leftIris: transformBiometrics(raw.leftIris),
    face: transformBiometrics(raw.face),
    rightPalm: transformBiometrics(raw.rightPalm),
    leftPalm: transformBiometrics(raw.leftPalm),
    voice: transformBiometrics(raw.voice),
  };
}

/**
 * Transform raw biometrics array
 */
function transformBiometrics(
  raw: unknown
): import("./types.js").Biometric[] | undefined {
  if (!raw || !Array.isArray(raw)) {
    return undefined;
  }

  return raw.map((b: Record<string, unknown>) => ({
    data: b.data as Uint8Array,
    format: b.format as number,
    subFormat: b.subFormat as number | undefined,
    issuer: b.issuer as string | undefined,
  }));
}
