/**
 * MOSIP Claim 169 QR Code library for TypeScript/JavaScript.
 *
 * This library provides classes to encode and decode MOSIP Claim 169 identity
 * credentials from QR codes. It uses WebAssembly for high-performance binary
 * parsing and cryptographic operations.
 *
 * ## Installation
 *
 * ```bash
 * npm install claim169
 * ```
 *
 * ## Decoding with Verification (Recommended)
 *
 * ```typescript
 * import { Decoder } from 'claim169';
 *
 * // Decode with Ed25519 signature verification
 * const result = new Decoder(qrText)
 *   .verifyWithEd25519(publicKey)
 *   .decode();
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
 * console.log(result.verificationStatus); // "verified"
 * ```
 *
 * ## Decoding without Verification (Testing Only)
 *
 * ```typescript
 * const result = new Decoder(qrText)
 *   .allowUnverified()  // Explicit opt-out required
 *   .decode();
 * ```
 *
 * ## Decoding Encrypted Credentials
 *
 * ```typescript
 * const result = new Decoder(qrText)
 *   .decryptWithAes256(aesKey)
 *   .verifyWithEd25519(publicKey)
 *   .decode();
 * ```
 *
 * ## Encoding Credentials
 *
 * ```typescript
 * import { Encoder } from 'claim169';
 *
 * const qrData = new Encoder(claim169, cwtMeta)
 *   .signWithEd25519(privateKey)
 *   .encode();
 * ```
 *
 * ## Error Handling
 *
 * ```typescript
 * import { Decoder, Claim169Error } from 'claim169';
 *
 * try {
 *   const result = new Decoder(qrText)
 *     .verifyWithEd25519(publicKey)
 *     .decode();
 * } catch (error) {
 *   if (error instanceof Claim169Error) {
 *     console.error('Decoding failed:', error.message);
 *   }
 * }
 * ```
 *
 * ## Notes
 *
 * - **Timestamp validation**: Disabled by default because WASM doesn't have
 *   reliable access to system time. Enable with `.withTimestampValidation()`.
 *
 * @module claim169
 */

export type {
  Biometric,
  Claim169,
  Claim169Input,
  CwtMeta,
  CwtMetaInput,
  DecodeResult,
  IDecoder,
  IEncoder,
  VerificationStatus,
} from "./types.js";

export { Claim169Error } from "./types.js";

import type {
  Claim169Input,
  CwtMetaInput,
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
 * Supports signature verification with Ed25519 and ECDSA P-256, as well as
 * AES-GCM decryption for encrypted credentials.
 *
 * @example
 * ```typescript
 * // With verification (recommended for production)
 * const result = new Decoder(qrText)
 *   .verifyWithEd25519(publicKey)
 *   .decode();
 *
 * // Without verification (testing only)
 * const result = new Decoder(qrText)
 *   .allowUnverified()
 *   .skipBiometrics()
 *   .decode();
 *
 * // With decryption and verification
 * const result = new Decoder(qrText)
 *   .decryptWithAes256(aesKey)
 *   .verifyWithEd25519(publicKey)
 *   .decode();
 * ```
 */
export class Decoder implements IDecoder {
  private wasmDecoder: wasm.WasmDecoder;

  /**
   * Create a new Decoder instance.
   * @param qrText - The QR code text content (Base45 encoded)
   */
  constructor(qrText: string) {
    this.wasmDecoder = new wasm.WasmDecoder(qrText);
  }

  /**
   * Verify signature with Ed25519 public key.
   * @param publicKey - 32-byte Ed25519 public key
   * @returns The decoder instance for chaining
   * @throws {Claim169Error} If the public key is invalid
   */
  verifyWithEd25519(publicKey: Uint8Array): Decoder {
    try {
      this.wasmDecoder = this.wasmDecoder.verifyWithEd25519(publicKey);
      return this;
    } catch (error) {
      if (error instanceof Error) {
        throw new Claim169Error(error.message);
      }
      throw new Claim169Error(String(error));
    }
  }

  /**
   * Verify signature with ECDSA P-256 public key.
   * @param publicKey - SEC1-encoded P-256 public key (33 or 65 bytes)
   * @returns The decoder instance for chaining
   * @throws {Claim169Error} If the public key is invalid
   */
  verifyWithEcdsaP256(publicKey: Uint8Array): Decoder {
    try {
      this.wasmDecoder = this.wasmDecoder.verifyWithEcdsaP256(publicKey);
      return this;
    } catch (error) {
      if (error instanceof Error) {
        throw new Claim169Error(error.message);
      }
      throw new Claim169Error(String(error));
    }
  }

  /**
   * Allow decoding without signature verification.
   * WARNING: Unverified credentials cannot be trusted. Use for testing only.
   * @returns The decoder instance for chaining
   */
  allowUnverified(): Decoder {
    this.wasmDecoder = this.wasmDecoder.allowUnverified();
    return this;
  }

  /**
   * Decrypt with AES-256-GCM.
   * @param key - 32-byte AES-256 key
   * @returns The decoder instance for chaining
   * @throws {Claim169Error} If the key is invalid
   */
  decryptWithAes256(key: Uint8Array): Decoder {
    try {
      this.wasmDecoder = this.wasmDecoder.decryptWithAes256(key);
      return this;
    } catch (error) {
      if (error instanceof Error) {
        throw new Claim169Error(error.message);
      }
      throw new Claim169Error(String(error));
    }
  }

  /**
   * Decrypt with AES-128-GCM.
   * @param key - 16-byte AES-128 key
   * @returns The decoder instance for chaining
   * @throws {Claim169Error} If the key is invalid
   */
  decryptWithAes128(key: Uint8Array): Decoder {
    try {
      this.wasmDecoder = this.wasmDecoder.decryptWithAes128(key);
      return this;
    } catch (error) {
      if (error instanceof Error) {
        throw new Claim169Error(error.message);
      }
      throw new Claim169Error(String(error));
    }
  }

  /**
   * Skip biometric data during decoding.
   * Useful when only demographic data is needed for faster parsing.
   * @returns The decoder instance for chaining
   */
  skipBiometrics(): Decoder {
    this.wasmDecoder = this.wasmDecoder.skipBiometrics();
    return this;
  }

  /**
   * Enable timestamp validation.
   * When enabled, expired or not-yet-valid credentials will throw an error.
   * Disabled by default because WASM doesn't have reliable access to system time.
   * @returns The decoder instance for chaining
   */
  withTimestampValidation(): Decoder {
    this.wasmDecoder = this.wasmDecoder.withTimestampValidation();
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
    this.wasmDecoder = this.wasmDecoder.clockSkewTolerance(seconds);
    return this;
  }

  /**
   * Set maximum decompressed size in bytes.
   * Protects against decompression bomb attacks.
   * @param bytes - The maximum size in bytes (default: 65536)
   * @returns The decoder instance for chaining
   */
  maxDecompressedBytes(bytes: number): Decoder {
    this.wasmDecoder = this.wasmDecoder.maxDecompressedBytes(bytes);
    return this;
  }

  /**
   * Decode the QR code with the configured options.
   * Requires either a verifier (verifyWithEd25519/verifyWithEcdsaP256) or
   * explicit allowUnverified() to be called first.
   * @returns The decoded result
   * @throws {Claim169Error} If decoding fails or no verification method specified
   */
  decode(): DecodeResult {
    try {
      const result = this.wasmDecoder.decode();
      return transformResult(result);
    } catch (error) {
      if (error instanceof Error) {
        throw new Claim169Error(error.message);
      }
      throw new Claim169Error(String(error));
    }
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
