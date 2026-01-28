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
 * - **Timestamp validation**: Enabled by default in JS (host-side). Disable with
 *   `.withoutTimestampValidation()` if you intentionally want to skip time checks.
 *
 * @module claim169
 */

export type {
  Algorithm,
  AlgorithmName,
  Biometric,
  CertificateHash,
  Claim169,
  Claim169Input,
  CwtMeta,
  CwtMetaInput,
  DecodeResult,
  DecryptorCallback,
  EncryptorCallback,
  IDecoder,
  IEncoder,
  SignerCallback,
  VerificationStatus,
  VerifierCallback,
  X509Headers,
} from "./types.js";

export { Claim169Error } from "./types.js";

import type {
  Claim169Input,
  CwtMetaInput,
  DecodeResult,
  DecryptorCallback,
  EncryptorCallback,
  IDecoder,
  IEncoder,
  SignerCallback,
  VerifierCallback,
  X509Headers,
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
 * Convert a hex string to bytes.
 *
 * Accepts optional `0x` prefix and ignores whitespace.
 *
 * @throws {Claim169Error} If the input is not valid hex
 */
export function hexToBytes(hex: string): Uint8Array {
  const normalized = hex.trim().replace(/^0x/i, "").replace(/\s+/g, "");

  if (normalized.length === 0) {
    return new Uint8Array();
  }

  if (normalized.length % 2 !== 0) {
    throw new Claim169Error("hex string must have even length");
  }

  const out = new Uint8Array(normalized.length / 2);
  for (let i = 0; i < out.length; i++) {
    const byteStr = normalized.slice(i * 2, i * 2 + 2);
    const value = Number.parseInt(byteStr, 16);
    if (!Number.isFinite(value) || Number.isNaN(value)) {
      throw new Claim169Error(`invalid hex byte: ${byteStr}`);
    }
    out[i] = value;
  }
  return out;
}

/**
 * Convert bytes to a lowercase hex string.
 */
export function bytesToHex(bytes: Uint8Array): string {
  let out = "";
  for (let i = 0; i < bytes.length; i++) {
    out += bytes[i].toString(16).padStart(2, "0");
  }
  return out;
}

/**
 * Transform raw X.509 headers from WASM result
 */
function transformX509Headers(raw: Record<string, unknown> | undefined): X509Headers {
  if (!raw) {
    return {};
  }

  const headers: X509Headers = {};

  if (raw.x5bag) {
    headers.x5bag = (raw.x5bag as Array<number[]>).map((cert) => new Uint8Array(cert));
  }
  if (raw.x5chain) {
    headers.x5chain = (raw.x5chain as Array<number[]>).map((cert) => new Uint8Array(cert));
  }
  if (raw.x5t) {
    const x5t = raw.x5t as { algorithm: string; hashValue: number[] };
    headers.x5t = {
      algorithm: x5t.algorithm,
      hashValue: new Uint8Array(x5t.hashValue),
    };
  }
  if (raw.x5u) {
    headers.x5u = raw.x5u as string;
  }

  return headers;
}

/**
 * Transform raw WASM result to typed DecodeResult
 */
function transformResult(raw: unknown): DecodeResult {
  const result = raw as {
    claim169: Record<string, unknown>;
    cwtMeta: Record<string, unknown>;
    verificationStatus: string;
    x509Headers: Record<string, unknown>;
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
    x509Headers: transformX509Headers(result.x509Headers),
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

function validateTimestampsHost(
  cwtMeta: { expiresAt?: number; notBefore?: number },
  clockSkewToleranceSeconds: number
): void {
  const skew = Math.max(0, Math.floor(clockSkewToleranceSeconds));
  const now = Math.floor(Date.now() / 1000);

  if (cwtMeta.expiresAt !== undefined && now > cwtMeta.expiresAt + skew) {
    throw new Claim169Error(
      `Credential expired at timestamp ${cwtMeta.expiresAt} (now ${now})`
    );
  }

  if (cwtMeta.notBefore !== undefined && now + skew < cwtMeta.notBefore) {
    throw new Claim169Error(
      `Credential not valid until timestamp ${cwtMeta.notBefore} (now ${now})`
    );
  }
}

/**
 * Validate date format is ISO 8601 and represents a valid date.
 * Accepts both extended format (YYYY-MM-DD) and basic format (YYYYMMDD).
 * @throws {Claim169Error} If the date format is invalid
 */
function validateDateFormat(
  date: string | null | undefined,
  fieldName: string
): void {
  if (!date) return;

  // Accept both ISO 8601 extended (YYYY-MM-DD) and basic (YYYYMMDD) formats
  const extendedPattern = /^\d{4}-\d{2}-\d{2}$/;
  const basicPattern = /^\d{8}$/;

  let year: number;
  let month: number;
  let day: number;

  if (extendedPattern.test(date)) {
    const parts = date.split("-");
    year = Number.parseInt(parts[0], 10);
    month = Number.parseInt(parts[1], 10);
    day = Number.parseInt(parts[2], 10);
  } else if (basicPattern.test(date)) {
    year = Number.parseInt(date.substring(0, 4), 10);
    month = Number.parseInt(date.substring(4, 6), 10);
    day = Number.parseInt(date.substring(6, 8), 10);
  } else {
    throw new Claim169Error(
      `Invalid ${fieldName} format: "${date}". Expected YYYY-MM-DD or YYYYMMDD.`
    );
  }

  // Validate the date values are actually valid (not 2024-13-45)
  const parsed = new Date(year, month - 1, day);
  if (
    parsed.getFullYear() !== year ||
    parsed.getMonth() !== month - 1 ||
    parsed.getDate() !== day
  ) {
    throw new Claim169Error(
      `Invalid ${fieldName} value: "${date}" is not a valid calendar date.`
    );
  }
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
  private validateTimestamps = true;
  private clockSkewToleranceSeconds = 0;

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
   * Verify signature with Ed25519 public key in PEM format.
   * Supports SPKI format with "BEGIN PUBLIC KEY" headers.
   * @param pem - PEM-encoded Ed25519 public key
   * @returns The decoder instance for chaining
   * @throws {Claim169Error} If the PEM is invalid
   */
  verifyWithEd25519Pem(pem: string): Decoder {
    try {
      this.wasmDecoder = this.wasmDecoder.verifyWithEd25519Pem(pem);
      return this;
    } catch (error) {
      if (error instanceof Error) {
        throw new Claim169Error(error.message);
      }
      throw new Claim169Error(String(error));
    }
  }

  /**
   * Verify signature with ECDSA P-256 public key in PEM format.
   * Supports SPKI format with "BEGIN PUBLIC KEY" headers.
   * @param pem - PEM-encoded P-256 public key
   * @returns The decoder instance for chaining
   * @throws {Claim169Error} If the PEM is invalid
   */
  verifyWithEcdsaP256Pem(pem: string): Decoder {
    try {
      this.wasmDecoder = this.wasmDecoder.verifyWithEcdsaP256Pem(pem);
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
   * Implemented in the host (JavaScript) to avoid WASM runtime time limitations.
   * @returns The decoder instance for chaining
   */
  withTimestampValidation(): Decoder {
    this.validateTimestamps = true;
    return this;
  }

  /**
   * Disable timestamp validation.
   * @returns The decoder instance for chaining
   */
  withoutTimestampValidation(): Decoder {
    this.validateTimestamps = false;
    return this;
  }

  /**
   * Set clock skew tolerance in seconds.
   * Allows credentials to be accepted when clocks are slightly out of sync.
   * Applies when timestamp validation is enabled.
   * @param seconds - The tolerance in seconds
   * @returns The decoder instance for chaining
   */
  clockSkewTolerance(seconds: number): Decoder {
    this.clockSkewToleranceSeconds = Math.max(0, Math.floor(seconds));
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
   * Verify signature with a custom verifier callback.
   * Use for external crypto providers (HSM, cloud KMS, remote signing, etc.)
   *
   * @param verifier - Function that verifies signatures
   * @returns The decoder instance for chaining
   *
   * @example
   * ```typescript
   * const result = new Decoder(qrText)
   *   .verifyWith((algorithm, keyId, data, signature) => {
   *     // Call your crypto provider here
   *     myKms.verify(keyId, data, signature);
   *   })
   *   .decode();
   * ```
   */
  verifyWith(verifier: VerifierCallback): Decoder {
    try {
      this.wasmDecoder = this.wasmDecoder.verifyWith(verifier);
      return this;
    } catch (error) {
      if (error instanceof Error) {
        throw new Claim169Error(error.message);
      }
      throw new Claim169Error(String(error));
    }
  }

  /**
   * Decrypt with a custom decryptor callback.
   * Use for external crypto providers (HSM, cloud KMS, etc.)
   *
   * @param decryptor - Function that decrypts ciphertext
   * @returns The decoder instance for chaining
   *
   * @example
   * ```typescript
   * const result = new Decoder(qrText)
   *   .decryptWith((algorithm, keyId, nonce, aad, ciphertext) => {
   *     // Call your crypto provider here
   *     return myKms.decrypt(keyId, ciphertext, { nonce, aad });
   *   })
   *   .decode();
   * ```
   */
  decryptWith(decryptor: DecryptorCallback): Decoder {
    try {
      this.wasmDecoder = this.wasmDecoder.decryptWith(decryptor);
      return this;
    } catch (error) {
      if (error instanceof Error) {
        throw new Claim169Error(error.message);
      }
      throw new Claim169Error(String(error));
    }
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
      const transformed = transformResult(result);

      // Validate date format (YYYY-MM-DD)
      validateDateFormat(transformed.claim169.dateOfBirth, "dateOfBirth");

      if (this.validateTimestamps) {
        validateTimestampsHost(
          transformed.cwtMeta,
          this.clockSkewToleranceSeconds
        );
      }
      return transformed;
    } catch (error) {
      if (error instanceof Error) {
        throw new Claim169Error(error.message);
      }
      throw new Claim169Error(String(error));
    }
  }
}

/**
 * Options for the `decode()` convenience function.
 *
 * Notes:
 * - If you don't provide a verification key, you must explicitly set `allowUnverified: true` (testing only).
 * - Timestamp validation is enabled by default in JS (host-side). Set `validateTimestamps: false` to disable.
 */
export interface DecodeOptions {
  verifyWithEd25519?: Uint8Array;
  verifyWithEcdsaP256?: Uint8Array;
  decryptWithAes256?: Uint8Array;
  decryptWithAes128?: Uint8Array;
  allowUnverified?: boolean;
  skipBiometrics?: boolean;
  validateTimestamps?: boolean;
  clockSkewToleranceSeconds?: number;
  maxDecompressedBytes?: number;
}

/**
 * Decode a Claim 169 QR string.
 *
 * This is a convenience wrapper around the `Decoder` builder.
 * Security:
 * - If you do not pass a verification key, you must set `allowUnverified: true` (testing only).
 */
export function decode(qrText: string, options: DecodeOptions = {}): DecodeResult {
  let decoder = new Decoder(qrText);

  if (options.decryptWithAes256) {
    decoder = decoder.decryptWithAes256(options.decryptWithAes256);
  }
  if (options.decryptWithAes128) {
    decoder = decoder.decryptWithAes128(options.decryptWithAes128);
  }

  if (options.skipBiometrics) {
    decoder = decoder.skipBiometrics();
  }

  if (options.validateTimestamps === false) {
    decoder = decoder.withoutTimestampValidation();
  }

  if (options.clockSkewToleranceSeconds !== undefined) {
    decoder = decoder.clockSkewTolerance(options.clockSkewToleranceSeconds);
  }

  if (options.maxDecompressedBytes !== undefined) {
    decoder = decoder.maxDecompressedBytes(options.maxDecompressedBytes);
  }

  if (options.verifyWithEd25519) {
    decoder = decoder.verifyWithEd25519(options.verifyWithEd25519);
  } else if (options.verifyWithEcdsaP256) {
    decoder = decoder.verifyWithEcdsaP256(options.verifyWithEcdsaP256);
  } else if (options.allowUnverified === true) {
    decoder = decoder.allowUnverified();
  } else {
    throw new Claim169Error(
      "decode() requires a verification key or allowUnverified: true"
    );
  }

  return decoder.decode();
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
   * Sign with a custom signer callback.
   * Use for external crypto providers (HSM, cloud KMS, remote signing, etc.)
   *
   * @param signer - Function that signs data
   * @param algorithm - Signature algorithm: "EdDSA" or "ES256"
   * @param keyId - Optional key identifier passed to the signer callback
   * @returns The encoder instance for chaining
   *
   * @example
   * ```typescript
   * const qrData = new Encoder(claim169, cwtMeta)
   *   .signWith((algorithm, keyId, data) => {
   *     return myKms.sign({ keyId, data });
   *   }, "EdDSA")
   *   .encode();
   * ```
   */
  signWith(
    signer: SignerCallback,
    algorithm: "EdDSA" | "ES256",
    keyId?: Uint8Array | null
  ): Encoder {
    try {
      // Wrap the callback so callers can optionally provide a keyId even if the
      // underlying WASM bindings don't expose key-id configuration yet.
      const signerWithKeyId: SignerCallback = (alg, wasmKeyId, data) =>
        signer(alg, keyId ?? wasmKeyId, data);

      this.wasmEncoder = this.wasmEncoder.signWith(signerWithKeyId, algorithm);
      return this;
    } catch (error) {
      if (error instanceof Error) {
        throw new Claim169Error(error.message);
      }
      throw new Claim169Error(String(error));
    }
  }

  /**
   * Encrypt with a custom encryptor callback.
   * Use for external crypto providers (HSM, cloud KMS, etc.)
   *
   * @param encryptor - Function that encrypts data
   * @param algorithm - Encryption algorithm: "A256GCM" or "A128GCM"
   * @returns The encoder instance for chaining
   *
   * @example
   * ```typescript
   * const qrData = new Encoder(claim169, cwtMeta)
   *   .signWithEd25519(signKey)
   *   .encryptWith((algorithm, keyId, nonce, aad, plaintext) => {
   *     return myKms.encrypt({ keyId, nonce, aad, plaintext });
   *   }, "A256GCM")
   *   .encode();
   * ```
   */
  encryptWith(
    encryptor: EncryptorCallback,
    algorithm: "A256GCM" | "A128GCM"
  ): Encoder {
    try {
      this.wasmEncoder = this.wasmEncoder.encryptWith(encryptor, algorithm);
      return this;
    } catch (error) {
      if (error instanceof Error) {
        throw new Claim169Error(error.message);
      }
      throw new Claim169Error(String(error));
    }
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
