/**
 * MOSIP Claim 169 QR Code decoder library for TypeScript/JavaScript.
 *
 * This library provides functions to decode MOSIP Claim 169 identity credentials
 * from QR codes. It uses WebAssembly for high-performance binary parsing.
 *
 * ## Installation
 *
 * ```bash
 * npm install claim169
 * ```
 *
 * ## Quick Start
 *
 * ```typescript
 * import { decode } from 'claim169';
 *
 * // Decode a QR code
 * const result = decode(qrText);
 *
 * // Access identity data
 * console.log(result.claim169.fullName);
 * console.log(result.claim169.dateOfBirth);
 *
 * // Access metadata
 * console.log(result.cwtMeta.issuer);
 * console.log(result.cwtMeta.expiresAt);
 *
 * // Check verification status
 * console.log(result.verificationStatus); // "skipped" | "verified" | "failed"
 * ```
 *
 * ## Builder Pattern API
 *
 * For a more fluent API, use the `Decoder` class:
 *
 * ```typescript
 * import { Decoder } from 'claim169';
 *
 * const result = new Decoder(qrText)
 *   .skipBiometrics()
 *   .withTimestampValidation()
 *   .clockSkewTolerance(60)
 *   .maxDecompressedBytes(32768)
 *   .decode();
 * ```
 *
 * ## With Options (Function API)
 *
 * ```typescript
 * import { decode, DecodeOptions } from 'claim169';
 *
 * const options: DecodeOptions = {
 *   maxDecompressedBytes: 32768,  // 32KB limit
 *   skipBiometrics: true,          // Skip biometric parsing
 *   validateTimestamps: false,     // Disabled by default in WASM
 * };
 *
 * const result = decode(qrText, options);
 * ```
 *
 * ## Error Handling
 *
 * ```typescript
 * import { decode, Claim169Error } from 'claim169';
 *
 * try {
 *   const result = decode(qrText);
 * } catch (error) {
 *   if (error instanceof Claim169Error) {
 *     console.error('Decoding failed:', error.message);
 *   }
 * }
 * ```
 *
 * ## Limitations
 *
 * - **No signature verification**: The WASM module does not support signature
 *   verification. Use the Rust or Python bindings for verified decoding.
 * - **Timestamp validation**: Disabled by default because WASM doesn't have
 *   reliable access to system time.
 *
 * @module claim169
 */

export type {
  Biometric,
  Claim169,
  Claim169Input,
  CwtMeta,
  CwtMetaInput,
  DecodeOptions,
  DecodeResult,
  IDecoder,
  IEncoder,
  VerificationStatus,
} from "./types.js";

export { Claim169Error } from "./types.js";

import type {
  Claim169Input,
  CwtMetaInput,
  DecodeOptions,
  DecodeResult,
  IDecoder,
  IEncoder,
} from "./types.js";
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
      // Use the WasmDecoder builder pattern
      let decoder = new wasm.WasmDecoder(qrText);

      if (options.maxDecompressedBytes !== undefined) {
        decoder = decoder.maxDecompressedBytes(options.maxDecompressedBytes);
      }
      if (options.skipBiometrics) {
        decoder = decoder.skipBiometrics();
      }
      if (options.validateTimestamps) {
        decoder = decoder.withTimestampValidation();
      }
      if (options.clockSkewToleranceSeconds !== undefined) {
        decoder = decoder.clockSkewTolerance(options.clockSkewToleranceSeconds);
      }

      const result = decoder.decode();
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

/**
 * Builder-pattern decoder for Claim 169 QR codes.
 *
 * Provides a fluent API for configuring decoding options and executing the decode.
 *
 * @example
 * ```typescript
 * // Basic usage
 * const result = new Decoder(qrText).decode();
 *
 * // With options
 * const result = new Decoder(qrText)
 *   .skipBiometrics()
 *   .withTimestampValidation()
 *   .clockSkewTolerance(60)
 *   .maxDecompressedBytes(32768)
 *   .decode();
 * ```
 */
export class Decoder implements IDecoder {
  private qrText: string;
  private _skipBiometrics: boolean = false;
  private _validateTimestamps: boolean = false;
  private _clockSkewTolerance: number = 0;
  private _maxDecompressedBytes: number = 65536;

  /**
   * Create a new Decoder instance.
   * @param qrText - The QR code text content (Base45 encoded)
   */
  constructor(qrText: string) {
    this.qrText = qrText;
  }

  /**
   * Skip biometric data during decoding.
   * Useful when only demographic data is needed for faster parsing.
   * @returns The decoder instance for chaining
   */
  skipBiometrics(): Decoder {
    this._skipBiometrics = true;
    return this;
  }

  /**
   * Enable timestamp validation.
   * When enabled, expired or not-yet-valid credentials will throw an error.
   * Disabled by default because WASM doesn't have reliable access to system time.
   * @returns The decoder instance for chaining
   */
  withTimestampValidation(): Decoder {
    this._validateTimestamps = true;
    return this;
  }

  /**
   * Set clock skew tolerance in seconds.
   * Allows credentials to be accepted when clocks are slightly out of sync.
   * Only applies when timestamp validation is enabled.
   * @param seconds - The tolerance in seconds
   * @returns The decoder instance for chaining
   */
  clockSkewTolerance(seconds: number): Decoder {
    this._clockSkewTolerance = seconds;
    return this;
  }

  /**
   * Set maximum decompressed size in bytes.
   * Protects against decompression bomb attacks.
   * @param bytes - The maximum size in bytes (default: 65536)
   * @returns The decoder instance for chaining
   */
  maxDecompressedBytes(bytes: number): Decoder {
    this._maxDecompressedBytes = bytes;
    return this;
  }

  /**
   * Decode the QR code with the configured options.
   * @returns The decoded result
   * @throws {Claim169Error} If decoding fails
   */
  decode(): DecodeResult {
    const options: DecodeOptions = {
      skipBiometrics: this._skipBiometrics,
      validateTimestamps: this._validateTimestamps,
      clockSkewToleranceSeconds: this._clockSkewTolerance,
      maxDecompressedBytes: this._maxDecompressedBytes,
    };

    // Use the existing decode function with our built options
    return decode(this.qrText, options);
  }
}

/**
 * Builder-pattern encoder for Claim 169 QR codes.
 *
 * Provides a fluent API for configuring encoding options and generating QR data.
 *
 * @example
 * ```typescript
 * // Signed credential (recommended)
 * const qrData = new Encoder(claim169, cwtMeta)
 *   .signWithEd25519(privateKey)
 *   .encode();
 *
 * // Signed and encrypted
 * const qrData = new Encoder(claim169, cwtMeta)
 *   .signWithEd25519(privateKey)
 *   .encryptWithAes256(aesKey)
 *   .encode();
 *
 * // Unsigned (testing only)
 * const qrData = new Encoder(claim169, cwtMeta)
 *   .allowUnsigned()
 *   .encode();
 * ```
 */
export class Encoder implements IEncoder {
  private wasmEncoder: wasm.WasmEncoder;

  /**
   * Create a new Encoder instance.
   * @param claim169 - The identity claim data to encode
   * @param cwtMeta - CWT metadata including issuer, expiration, etc.
   */
  constructor(claim169: Claim169Input, cwtMeta: CwtMetaInput) {
    try {
      this.wasmEncoder = new wasm.WasmEncoder(claim169, cwtMeta);
    } catch (error) {
      if (error instanceof Error) {
        throw new Claim169Error(error.message);
      }
      throw new Claim169Error(String(error));
    }
  }

  /**
   * Sign with Ed25519 private key.
   * @param privateKey - 32-byte Ed25519 private key
   * @returns The encoder instance for chaining
   */
  signWithEd25519(privateKey: Uint8Array): Encoder {
    try {
      this.wasmEncoder = this.wasmEncoder.signWithEd25519(privateKey);
      return this;
    } catch (error) {
      if (error instanceof Error) {
        throw new Claim169Error(error.message);
      }
      throw new Claim169Error(String(error));
    }
  }

  /**
   * Sign with ECDSA P-256 private key.
   * @param privateKey - 32-byte ECDSA P-256 private key (scalar)
   * @returns The encoder instance for chaining
   */
  signWithEcdsaP256(privateKey: Uint8Array): Encoder {
    try {
      this.wasmEncoder = this.wasmEncoder.signWithEcdsaP256(privateKey);
      return this;
    } catch (error) {
      if (error instanceof Error) {
        throw new Claim169Error(error.message);
      }
      throw new Claim169Error(String(error));
    }
  }

  /**
   * Encrypt with AES-256-GCM.
   * @param key - 32-byte AES-256 key
   * @returns The encoder instance for chaining
   */
  encryptWithAes256(key: Uint8Array): Encoder {
    try {
      this.wasmEncoder = this.wasmEncoder.encryptWithAes256(key);
      return this;
    } catch (error) {
      if (error instanceof Error) {
        throw new Claim169Error(error.message);
      }
      throw new Claim169Error(String(error));
    }
  }

  /**
   * Encrypt with AES-128-GCM.
   * @param key - 16-byte AES-128 key
   * @returns The encoder instance for chaining
   */
  encryptWithAes128(key: Uint8Array): Encoder {
    try {
      this.wasmEncoder = this.wasmEncoder.encryptWithAes128(key);
      return this;
    } catch (error) {
      if (error instanceof Error) {
        throw new Claim169Error(error.message);
      }
      throw new Claim169Error(String(error));
    }
  }

  /**
   * Allow encoding without a signature.
   * WARNING: Unsigned credentials cannot be verified. Use for testing only.
   * @returns The encoder instance for chaining
   */
  allowUnsigned(): Encoder {
    this.wasmEncoder = this.wasmEncoder.allowUnsigned();
    return this;
  }

  /**
   * Skip biometric fields during encoding.
   * @returns The encoder instance for chaining
   */
  skipBiometrics(): Encoder {
    this.wasmEncoder = this.wasmEncoder.skipBiometrics();
    return this;
  }

  /**
   * Encode the credential to a Base45 QR string.
   * @returns Base45-encoded string suitable for QR code generation
   * @throws {Claim169Error} If encoding fails
   */
  encode(): string {
    try {
      return this.wasmEncoder.encode();
    } catch (error) {
      if (error instanceof Error) {
        throw new Claim169Error(error.message);
      }
      throw new Claim169Error(String(error));
    }
  }
}

/**
 * Generate a random 12-byte nonce for AES-GCM encryption.
 * @returns A 12-byte Uint8Array suitable for use as a nonce
 */
export function generateNonce(): Uint8Array {
  return new Uint8Array(wasm.generateNonce());
}
