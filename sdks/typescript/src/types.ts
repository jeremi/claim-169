/**
 * Type definitions for MOSIP Claim 169 QR Code decoder.
 *
 * This module contains TypeScript interfaces and types for the decoded
 * Claim 169 identity data.
 *
 * @module claim169/types
 */

/**
 * A single biometric data entry from a Claim 169 credential.
 *
 * Biometric data can be fingerprints, iris scans, face images, or voice samples.
 * Each entry contains the raw data and metadata about its format.
 *
 * @example
 * ```typescript
 * // Access face biometric data
 * if (claim.face && claim.face.length > 0) {
 *   const faceData = claim.face[0];
 *   console.log(`Format: ${faceData.format}`);
 *   console.log(`Data size: ${faceData.data.byteLength} bytes`);
 * }
 * ```
 */
export interface Biometric {
  /** Raw biometric data bytes (image, template, or audio) */
  data: Uint8Array;
  /**
   * Biometric format code:
   * - 0: Image
   * - 1: Template
   * - 2: Sound
   * - 3: BioHash
   */
  format: number;
  /**
   * Sub-format code (depends on format type):
   * - For Image: 0=PNG, 1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP, 5=TIFF, 6=WSQ
   * - For Template: 0=ANSI378, 1=ISO19794-2, 2=NIST
   * - For Sound: 0=WAV, 1=MP3
   */
  subFormat?: number;
  /** Biometric data issuer/provider identifier */
  issuer?: string;
}

/**
 * CWT (CBOR Web Token) metadata from the credential.
 *
 * Contains standard JWT/CWT claims that provide information about the
 * credential's validity, issuer, and subject.
 *
 * @example
 * ```typescript
 * // Check if credential is expired
 * const now = Math.floor(Date.now() / 1000);
 * if (result.cwtMeta.expiresAt && result.cwtMeta.expiresAt < now) {
 *   console.log('Credential has expired!');
 * }
 *
 * // Check issuer
 * if (result.cwtMeta.issuer === 'https://mosip.io') {
 *   console.log('Issued by MOSIP');
 * }
 * ```
 */
export interface CwtMeta {
  /** Token issuer (typically a URL or identifier) */
  issuer?: string;
  /** Token subject (typically the credential holder's ID) */
  subject?: string;
  /** Expiration timestamp (Unix seconds) - credential invalid after this time */
  expiresAt?: number;
  /** Not-before timestamp (Unix seconds) - credential invalid before this time */
  notBefore?: number;
  /** Issued-at timestamp (Unix seconds) - when the credential was created */
  issuedAt?: number;
}

/**
 * Decoded Claim 169 identity data.
 *
 * This interface contains all identity fields defined in the MOSIP Claim 169
 * specification. All fields are optional since credentials may contain only
 * a subset of the available fields.
 *
 * Fields are organized into:
 * - **Demographics** (id, name, DOB, address, etc.)
 * - **Biometrics** (fingerprints, iris, face, voice)
 *
 * @example
 * ```typescript
 * // Access demographic data
 * console.log(`Name: ${claim.fullName}`);
 * console.log(`DOB: ${claim.dateOfBirth}`);
 *
 * // Check for biometrics
 * const hasFace = claim.face && claim.face.length > 0;
 * const hasFingerprints = claim.rightThumb || claim.leftThumb;
 * ```
 */
export interface Claim169 {
  /** Unique identifier (CBOR key 1) */
  id?: string;
  /** Claim version */
  version?: string;
  /** Primary language code */
  language?: string;
  /** Full name */
  fullName?: string;
  /** First name */
  firstName?: string;
  /** Middle name */
  middleName?: string;
  /** Last name */
  lastName?: string;
  /** Date of birth (ISO 8601 format) */
  dateOfBirth?: string;
  /** Gender code (1=Male, 2=Female, 3=Other) */
  gender?: number;
  /** Address */
  address?: string;
  /** Email address */
  email?: string;
  /** Phone number */
  phone?: string;
  /** Nationality */
  nationality?: string;
  /** Marital status code */
  maritalStatus?: number;
  /** Guardian name */
  guardian?: string;
  /** Photo data */
  photo?: Uint8Array;
  /** Photo format code */
  photoFormat?: number;
  /** Best quality fingers indicator */
  bestQualityFingers?: Uint8Array;
  /** Secondary full name */
  secondaryFullName?: string;
  /** Secondary language code */
  secondaryLanguage?: string;
  /** Location code */
  locationCode?: string;
  /** Legal status */
  legalStatus?: string;
  /** Country of issuance */
  countryOfIssuance?: string;

  /** Right thumb biometrics */
  rightThumb?: Biometric[];
  /** Right pointer finger biometrics */
  rightPointerFinger?: Biometric[];
  /** Right middle finger biometrics */
  rightMiddleFinger?: Biometric[];
  /** Right ring finger biometrics */
  rightRingFinger?: Biometric[];
  /** Right little finger biometrics */
  rightLittleFinger?: Biometric[];
  /** Left thumb biometrics */
  leftThumb?: Biometric[];
  /** Left pointer finger biometrics */
  leftPointerFinger?: Biometric[];
  /** Left middle finger biometrics */
  leftMiddleFinger?: Biometric[];
  /** Left ring finger biometrics */
  leftRingFinger?: Biometric[];
  /** Left little finger biometrics */
  leftLittleFinger?: Biometric[];
  /** Right iris biometrics */
  rightIris?: Biometric[];
  /** Left iris biometrics */
  leftIris?: Biometric[];
  /** Face biometrics */
  face?: Biometric[];
  /** Right palm biometrics */
  rightPalm?: Biometric[];
  /** Left palm biometrics */
  leftPalm?: Biometric[];
  /** Voice biometrics */
  voice?: Biometric[];
}

/**
 * Signature verification status of the decoded credential.
 *
 * - `"verified"`: Signature was verified successfully with the provided public key
 * - `"skipped"`: Verification was explicitly skipped using `allowUnverified()`
 * - `"failed"`: Signature verification failed (invalid signature or wrong key)
 */
export type VerificationStatus = "verified" | "skipped" | "failed";

/**
 * Result of decoding a Claim 169 QR code.
 *
 * Contains the decoded identity data, CWT metadata, and verification status.
 *
 * @example
 * ```typescript
 * // Decode with verification
 * const result = new Decoder(qrText)
 *   .verifyWithEd25519(publicKey)
 *   .decode();
 *
 * // Access identity data
 * console.log(result.claim169.fullName);
 *
 * // Access metadata
 * console.log(result.cwtMeta.issuer);
 *
 * // Check verification status
 * console.log(result.verificationStatus); // "verified", "skipped", or "failed"
 * ```
 */
export interface DecodeResult {
  /** Decoded Claim 169 identity data */
  claim169: Claim169;
  /** CWT metadata (issuer, expiration, etc.) */
  cwtMeta: CwtMeta;
  /**
   * Signature verification status.
   * - "verified": Signature verified successfully with provided public key
   * - "skipped": Verification skipped (allowUnverified() or decode(..., { allowUnverified: true }))
   * - "failed": Signature verification failed
   */
  verificationStatus: VerificationStatus;
}

/**
 * Error thrown when decoding fails.
 *
 * @example
 * ```typescript
 * try {
 *   decode(qrText, { allowUnverified: true }); // testing only
 * } catch (error) {
 *   if (error instanceof Claim169Error) {
 *     console.error('Decoding failed:', error.message);
 *   }
 * }
 * ```
 */
export class Claim169Error extends Error {
  constructor(message: string) {
    super(message);
    this.name = "Claim169Error";
  }
}

// ============================================================================
// Encoder Types
// ============================================================================

/**
 * Input data for creating a Claim 169 credential.
 *
 * This interface contains all identity fields that can be encoded into
 * a Claim 169 QR code.
 *
 * @example
 * ```typescript
 * const claim169: Claim169Input = {
 *   id: "123456789",
 *   fullName: "John Doe",
 *   dateOfBirth: "1990-01-15",
 *   gender: 1,  // Male
 * };
 * ```
 */
export interface Claim169Input {
  /** Unique identifier */
  id?: string;
  /** Claim version */
  version?: string;
  /** Primary language code */
  language?: string;
  /** Full name */
  fullName?: string;
  /** First name */
  firstName?: string;
  /** Middle name */
  middleName?: string;
  /** Last name */
  lastName?: string;
  /** Date of birth (ISO 8601 format) */
  dateOfBirth?: string;
  /** Gender code (1=Male, 2=Female, 3=Other) */
  gender?: number;
  /** Address */
  address?: string;
  /** Email address */
  email?: string;
  /** Phone number */
  phone?: string;
  /** Nationality */
  nationality?: string;
  /** Marital status code (1=Unmarried, 2=Married, 3=Divorced) */
  maritalStatus?: number;
  /** Guardian name */
  guardian?: string;
  /** Photo data */
  photo?: Uint8Array;
  /** Photo format code (1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP) */
  photoFormat?: number;
  /** Secondary full name */
  secondaryFullName?: string;
  /** Secondary language code */
  secondaryLanguage?: string;
  /** Location code */
  locationCode?: string;
  /** Legal status */
  legalStatus?: string;
  /** Country of issuance */
  countryOfIssuance?: string;
}

/**
 * CWT metadata input for creating a Claim 169 credential.
 *
 * @example
 * ```typescript
 * const cwtMeta: CwtMetaInput = {
 *   issuer: "https://issuer.example.com",
 *   expiresAt: 1800000000,  // Unix timestamp
 * };
 * ```
 */
export interface CwtMetaInput {
  /** Token issuer (typically a URL or identifier) */
  issuer?: string;
  /** Token subject (typically the credential holder's ID) */
  subject?: string;
  /** Expiration timestamp (Unix seconds) */
  expiresAt?: number;
  /** Not-before timestamp (Unix seconds) */
  notBefore?: number;
  /** Issued-at timestamp (Unix seconds) */
  issuedAt?: number;
}

/**
 * Interface for the Encoder builder class.
 *
 * Provides a fluent API for configuring and encoding Claim 169 credentials.
 *
 * @example
 * ```typescript
 * const qrData = new Encoder(claim169, cwtMeta)
 *   .signWithEd25519(privateKey)
 *   .encode();
 * ```
 */
export interface IEncoder {
  /**
   * Sign with Ed25519 private key.
   * @param privateKey - 32-byte Ed25519 private key
   * @returns The encoder instance for chaining
   */
  signWithEd25519(privateKey: Uint8Array): IEncoder;

  /**
   * Sign with ECDSA P-256 private key.
   * @param privateKey - 32-byte ECDSA P-256 private key (scalar)
   * @returns The encoder instance for chaining
   */
  signWithEcdsaP256(privateKey: Uint8Array): IEncoder;

  /**
   * Encrypt with AES-256-GCM.
   * @param key - 32-byte AES-256 key
   * @returns The encoder instance for chaining
   */
  encryptWithAes256(key: Uint8Array): IEncoder;

  /**
   * Encrypt with AES-128-GCM.
   * @param key - 16-byte AES-128 key
   * @returns The encoder instance for chaining
   */
  encryptWithAes128(key: Uint8Array): IEncoder;

  /**
   * Allow encoding without a signature.
   * WARNING: Unsigned credentials cannot be verified. Use for testing only.
   * @returns The encoder instance for chaining
   */
  allowUnsigned(): IEncoder;

  /**
   * Skip biometric fields during encoding.
   * @returns The encoder instance for chaining
   */
  skipBiometrics(): IEncoder;

  /**
   * Encode the credential to a Base45 QR string.
   * @returns Base45-encoded string suitable for QR code generation
   * @throws {Claim169Error} If encoding fails
   */
  encode(): string;
}

// ============================================================================
// Decoder Types
// ============================================================================

/**
 * Interface for the Decoder builder class.
 *
 * Provides a fluent API for configuring and decoding Claim 169 QR codes.
 *
 * @example
 * ```typescript
 * // With verification (recommended)
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
export interface IDecoder {
  /**
   * Verify signature with Ed25519 public key.
   * @param publicKey - 32-byte Ed25519 public key
   * @returns The decoder instance for chaining
   * @throws {Claim169Error} If the public key is invalid
   */
  verifyWithEd25519(publicKey: Uint8Array): IDecoder;

  /**
   * Verify signature with ECDSA P-256 public key.
   * @param publicKey - SEC1-encoded P-256 public key (33 or 65 bytes)
   * @returns The decoder instance for chaining
   * @throws {Claim169Error} If the public key is invalid
   */
  verifyWithEcdsaP256(publicKey: Uint8Array): IDecoder;

  /**
   * Allow decoding without signature verification.
   * WARNING: Unverified credentials cannot be trusted. Use for testing only.
   * @returns The decoder instance for chaining
   */
  allowUnverified(): IDecoder;

  /**
   * Decrypt with AES-256-GCM.
   * @param key - 32-byte AES-256 key
   * @returns The decoder instance for chaining
   * @throws {Claim169Error} If the key is invalid
   */
  decryptWithAes256(key: Uint8Array): IDecoder;

  /**
   * Decrypt with AES-128-GCM.
   * @param key - 16-byte AES-128 key
   * @returns The decoder instance for chaining
   * @throws {Claim169Error} If the key is invalid
   */
  decryptWithAes128(key: Uint8Array): IDecoder;

  /**
   * Skip biometric data during decoding.
   * Useful when only demographic data is needed.
   * @returns The decoder instance for chaining
   */
  skipBiometrics(): IDecoder;

  /**
   * Enable timestamp validation.
   * When enabled, expired or not-yet-valid credentials will throw an error.
   * @returns The decoder instance for chaining
   */
  withTimestampValidation(): IDecoder;

  /**
   * Set clock skew tolerance in seconds.
   * Allows credentials to be accepted when clocks are slightly out of sync.
   * Only applies when timestamp validation is enabled.
   * @param seconds - The tolerance in seconds
   * @returns The decoder instance for chaining
   */
  clockSkewTolerance(seconds: number): IDecoder;

  /**
   * Set maximum decompressed size in bytes.
   * Protects against decompression bomb attacks.
   * @param bytes - The maximum size in bytes (default: 65536)
   * @returns The decoder instance for chaining
   */
  maxDecompressedBytes(bytes: number): IDecoder;

  /**
   * Decode the QR code with the configured options.
   * Requires either a verifier (verifyWithEd25519/verifyWithEcdsaP256) or
   * explicit allowUnverified() to be called first.
   * @returns The decoded result
   * @throws {Claim169Error} If decoding fails or no verification method specified
   */
  decode(): DecodeResult;
}
