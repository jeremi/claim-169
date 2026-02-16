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
 * X.509 certificate hash (COSE_CertHash).
 *
 * Contains a hash algorithm identifier and the hash value.
 * Used in the x5t (thumbprint) header parameter.
 */
export interface CertificateHash {
  /**
   * Hash algorithm identifier.
   * Can be a numeric COSE algorithm ID (e.g., "-16" for SHA-256) or a named algorithm.
   */
  algorithm: string;
  /** Hash value bytes */
  hashValue: Uint8Array;
}

/**
 * X.509 headers extracted from COSE protected/unprotected headers.
 *
 * These headers provide X.509 certificate information for signature verification
 * as defined in RFC 9360.
 *
 * @example
 * ```typescript
 * const result = new Decoder(qrText)
 *   .verifyWithEd25519(publicKey)
 *   .decode();
 *
 * // Check for certificate chain
 * if (result.x509Headers.x5chain) {
 *   console.log(`Certificate chain has ${result.x509Headers.x5chain.length} certificates`);
 * }
 *
 * // Check for certificate URL
 * if (result.x509Headers.x5u) {
 *   console.log(`Certificate URL: ${result.x509Headers.x5u}`);
 * }
 * ```
 */
export interface X509Headers {
  /**
   * x5bag (COSE label 32): Unordered bag of X.509 certificates.
   * Each certificate is DER-encoded.
   */
  x5bag?: Uint8Array[];
  /**
   * x5chain (COSE label 33): Ordered chain of X.509 certificates.
   * The first certificate contains the public key used for verification.
   * Each certificate is DER-encoded.
   */
  x5chain?: Uint8Array[];
  /**
   * x5t (COSE label 34): Certificate thumbprint hash.
   * Used to identify the certificate by its hash.
   */
  x5t?: CertificateHash;
  /**
   * x5u (COSE label 35): URI pointing to an X.509 certificate.
   * Can be used to fetch the certificate for verification.
   */
  x5u?: string;
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
 * Compression mode for encoding.
 *
 * - `"zlib"`: Standard zlib compression (default)
 * - `"none"`: No compression
 * - `"adaptive"`: Automatically choose best compression
 * - `` `brotli:${number}` ``: Brotli compression with quality level 0-11
 * - `` `adaptive-brotli:${number}` ``: Adaptive with Brotli at quality level 0-11
 */
export type CompressionMode =
  | "zlib"
  | "none"
  | "adaptive"
  | `brotli:${number}`
  | `adaptive-brotli:${number}`;

/**
 * Detected compression format from decoding.
 *
 * Known values are `"zlib"`, `"brotli"`, or `"none"`.
 * Unknown formats are preserved as strings for forward compatibility.
 */
export type DetectedCompression = "zlib" | "brotli" | "none" | (string & {});

/**
 * Signature verification status of the decoded credential.
 *
 * - `"verified"`: Signature was verified successfully with the provided public key
 * - `"skipped"`: Verification was explicitly skipped using `allowUnverified()`
 * - `"failed"`: Signature verification failed (invalid signature or wrong key)
 */
export type VerificationStatus = "verified" | "skipped" | "failed";

// ============================================================================
// Custom Crypto Provider Callback Types
// ============================================================================

/**
 * Algorithm identifier for COSE algorithms.
 * - "EdDSA" - Edwards-curve Digital Signature Algorithm (Ed25519)
 * - "ES256" - ECDSA with P-256 and SHA-256
 * - "A256GCM" - AES-256-GCM encryption
 * - "A128GCM" - AES-128-GCM encryption
 */
export type Algorithm = "EdDSA" | "ES256" | "A256GCM" | "A128GCM";

/**
 * Algorithm identifier as surfaced by the underlying WASM bindings.
 *
 * This preserves autocomplete for known values while still allowing
 * unknown strings for forwards compatibility.
 */
export type AlgorithmName = Algorithm | (string & {});

/**
 * Custom signature verifier callback.
 * Use for external crypto providers (HSM, cloud KMS, remote signing, etc.)
 *
 * The callback should throw an error if verification fails.
 *
 * @param algorithm - COSE algorithm identifier (e.g., "EdDSA", "ES256")
 * @param keyId - Optional key identifier from the COSE header
 * @param data - Data that was signed (the COSE Sig_structure)
 * @param signature - Signature to verify
 *
 * @example
 * ```typescript
 * const myVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
 *   const result = myKms.verify({ keyId, algorithm, data, signature });
 *   if (!result.valid) throw new Error("Verification failed");
 * };
 * ```
 */
export type VerifierCallback = (
  algorithm: AlgorithmName,
  keyId: Uint8Array | null,
  data: Uint8Array,
  signature: Uint8Array
) => void;

/**
 * Custom decryptor callback.
 * Use for external crypto providers (HSM, cloud KMS, etc.)
 *
 * @param algorithm - COSE algorithm identifier (e.g., "A256GCM", "A128GCM")
 * @param keyId - Optional key identifier from the COSE header
 * @param nonce - Nonce/IV for decryption (12 bytes for AES-GCM)
 * @param aad - Additional authenticated data
 * @param ciphertext - Ciphertext with authentication tag
 * @returns Decrypted plaintext
 *
 * @example
 * ```typescript
 * const myDecryptor: DecryptorCallback = (algorithm, keyId, nonce, aad, ciphertext) => {
 *   return myKms.decrypt({ keyId, nonce, aad, ciphertext });
 * };
 * ```
 */
export type DecryptorCallback = (
  algorithm: AlgorithmName,
  keyId: Uint8Array | null,
  nonce: Uint8Array,
  aad: Uint8Array,
  ciphertext: Uint8Array
) => Uint8Array;

/**
 * Custom signer callback.
 * Use for external crypto providers (HSM, cloud KMS, remote signing, etc.)
 *
 * @param algorithm - COSE algorithm identifier (e.g., "EdDSA", "ES256")
 * @param keyId - Optional key identifier
 * @param data - Data to sign (the COSE Sig_structure)
 * @returns Signature bytes
 *
 * @example
 * ```typescript
 * const mySigner: SignerCallback = (algorithm, keyId, data) => {
 *   return myKms.sign({ keyId, algorithm, data });
 * };
 * ```
 */
export type SignerCallback = (
  algorithm: AlgorithmName,
  keyId: Uint8Array | null,
  data: Uint8Array
) => Uint8Array;

/**
 * Custom encryptor callback.
 * Use for external crypto providers (HSM, cloud KMS, etc.)
 *
 * @param algorithm - COSE algorithm identifier (e.g., "A256GCM", "A128GCM")
 * @param keyId - Optional key identifier
 * @param nonce - Nonce/IV for encryption (12 bytes for AES-GCM)
 * @param aad - Additional authenticated data
 * @param plaintext - Data to encrypt
 * @returns Ciphertext with authentication tag
 *
 * @example
 * ```typescript
 * const myEncryptor: EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => {
 *   return myKms.encrypt({ keyId, nonce, aad, plaintext });
 * };
 * ```
 */
export type EncryptorCallback = (
  algorithm: AlgorithmName,
  keyId: Uint8Array | null,
  nonce: Uint8Array,
  aad: Uint8Array,
  plaintext: Uint8Array
) => Uint8Array;

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
  /**
   * X.509 headers from COSE protected/unprotected headers.
   * Contains certificate information for signature verification.
   */
  x509Headers: X509Headers;
  /** Compression format detected during decoding (e.g., "zlib", "brotli", "none") */
  detectedCompression: DetectedCompression;
  /** Warnings generated during decoding */
  warnings: EncodeWarning[];
}

/**
 * Warning from the encode/decode pipeline.
 */
export interface EncodeWarning {
  /** Warning code (e.g., "non_standard_compression") */
  code: string;
  /** Human-readable warning message */
  message: string;
}

/**
 * Result of encoding a Claim 169 credential.
 *
 * Contains the QR-ready Base45 string, the compression method that was
 * actually used, and any warnings generated during encoding.
 *
 * @example
 * ```typescript
 * const result = new Encoder(claim169, cwtMeta)
 *   .signWithEd25519(privateKey)
 *   .compression("zlib")
 *   .encode();
 *
 * console.log(result.qrData);           // Base45 string
 * console.log(result.compressionUsed);  // "zlib"
 * console.log(result.warnings);         // []
 * ```
 */
export interface EncodeResult {
  /** Base45-encoded string suitable for QR code generation */
  qrData: string;
  /** Compression method that was actually used (e.g., "zlib", "brotli", "none") */
  compressionUsed: string;
  /** Warnings generated during encoding */
  warnings: EncodeWarning[];
}

// ============================================================================
// Spec Enum Constants
// ============================================================================

/**
 * Gender values as defined in MOSIP Claim 169 (1-indexed).
 *
 * @example
 * ```typescript
 * if (claim.gender === Gender.Female) { ... }
 * ```
 */
export const Gender = { Male: 1, Female: 2, Other: 3 } as const;
/** Gender code type (1=Male, 2=Female, 3=Other) */
export type Gender = (typeof Gender)[keyof typeof Gender];

/**
 * Marital status values as defined in MOSIP Claim 169 (1-indexed).
 *
 * @example
 * ```typescript
 * if (claim.maritalStatus === MaritalStatus.Married) { ... }
 * ```
 */
export const MaritalStatus = {
  Unmarried: 1,
  Married: 2,
  Divorced: 3,
} as const;
/** Marital status code type (1=Unmarried, 2=Married, 3=Divorced) */
export type MaritalStatus = (typeof MaritalStatus)[keyof typeof MaritalStatus];

/**
 * Photo format values as defined in MOSIP Claim 169 (1-indexed).
 *
 * @example
 * ```typescript
 * if (claim.photoFormat === PhotoFormat.Jpeg) { ... }
 * ```
 */
export const PhotoFormat = {
  Jpeg: 1,
  Jpeg2000: 2,
  Avif: 3,
  Webp: 4,
} as const;
/** Photo format code type (1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP) */
export type PhotoFormat = (typeof PhotoFormat)[keyof typeof PhotoFormat];

/**
 * Biometric data format values as defined in MOSIP Claim 169 (0-indexed).
 *
 * @example
 * ```typescript
 * if (biometric.format === BiometricFormat.Image) { ... }
 * ```
 */
export const BiometricFormat = {
  Image: 0,
  Template: 1,
  Sound: 2,
  BioHash: 3,
} as const;
/** Biometric format code type (0=Image, 1=Template, 2=Sound, 3=BioHash) */
export type BiometricFormat =
  (typeof BiometricFormat)[keyof typeof BiometricFormat];

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
/**
 * Error code for programmatic error handling.
 *
 * Maps to error types from the Rust core library pipeline stages.
 */
export type Claim169ErrorCode =
  | "BASE45_DECODE"
  | "DECOMPRESS"
  | "DECOMPRESS_LIMIT"
  | "COSE_PARSE"
  | "UNSUPPORTED_COSE_TYPE"
  | "SIGNATURE_INVALID"
  | "DECRYPTION_FAILED"
  | "CBOR_PARSE"
  | "CWT_PARSE"
  | "CLAIM169_NOT_FOUND"
  | "CLAIM169_INVALID"
  | "UNSUPPORTED_ALGORITHM"
  | "KEY_NOT_FOUND"
  | "EXPIRED"
  | "NOT_YET_VALID"
  | "CRYPTO"
  | "CBOR_ENCODE"
  | "SIGNATURE_FAILED"
  | "ENCRYPTION_FAILED"
  | "ENCODING_CONFIG"
  | "DECODING_CONFIG"
  | "UNKNOWN";

/**
 * Parse a WASM error message prefix to determine the error code.
 */
function parseErrorCode(message: string): Claim169ErrorCode {
  const lower = message.toLowerCase();
  if (lower.startsWith("invalid base45")) return "BASE45_DECODE";
  if (lower.startsWith("decompression limit")) return "DECOMPRESS_LIMIT";
  if (lower.startsWith("decompression failed")) return "DECOMPRESS";
  if (lower.startsWith("invalid cose")) return "COSE_PARSE";
  if (lower.startsWith("unsupported cose type")) return "UNSUPPORTED_COSE_TYPE";
  if (lower.startsWith("signature verification failed")) return "SIGNATURE_INVALID";
  if (lower.startsWith("decryption failed")) return "DECRYPTION_FAILED";
  if (lower.startsWith("invalid cbor")) return "CBOR_PARSE";
  if (lower.startsWith("cwt parsing failed")) return "CWT_PARSE";
  if (lower.startsWith("claim 169 not found")) return "CLAIM169_NOT_FOUND";
  if (lower.startsWith("invalid claim 169")) return "CLAIM169_INVALID";
  if (lower.startsWith("unsupported algorithm")) return "UNSUPPORTED_ALGORITHM";
  if (lower.startsWith("key not found")) return "KEY_NOT_FOUND";
  if (lower.startsWith("credential expired")) return "EXPIRED";
  if (lower.startsWith("credential not valid until")) return "NOT_YET_VALID";
  if (lower.startsWith("cbor encoding failed")) return "CBOR_ENCODE";
  if (lower.startsWith("signing failed")) return "SIGNATURE_FAILED";
  if (lower.startsWith("encryption failed")) return "ENCRYPTION_FAILED";
  if (lower.startsWith("encoding configuration")) return "ENCODING_CONFIG";
  if (lower.startsWith("decoding configuration")) return "DECODING_CONFIG";
  if (lower.startsWith("crypto error")) return "CRYPTO";
  return "UNKNOWN";
}

export class Claim169Error extends Error {
  /** Programmatic error code for matching error types. */
  readonly code: Claim169ErrorCode;

  constructor(message: string, code?: Claim169ErrorCode) {
    super(message);
    this.name = "Claim169Error";
    this.code = code ?? parseErrorCode(message);
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
  ): IEncoder;

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
  ): IEncoder;

  /**
   * Set compression mode for encoding.
   * @param mode - Compression mode: "zlib", "none", "adaptive", "brotli:N" (0-11), or "adaptive-brotli:N"
   * @returns The encoder instance for chaining
   * @throws {Claim169Error} If the mode is invalid or unsupported by the WASM build
   */
  compression(mode: CompressionMode): IEncoder;

  /**
   * Encode the credential to a QR-ready result object.
   * @returns Encode result with QR data, compression info, and warnings
   * @throws {Claim169Error} If encoding fails
   */
  encode(): EncodeResult;
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
   * Verify signature with Ed25519 public key in PEM format.
   * Supports SPKI format with "BEGIN PUBLIC KEY" headers.
   * @param pem - PEM-encoded Ed25519 public key
   * @returns The decoder instance for chaining
   * @throws {Claim169Error} If the PEM is invalid
   */
  verifyWithEd25519Pem(pem: string): IDecoder;

  /**
   * Verify signature with ECDSA P-256 public key in PEM format.
   * Supports SPKI format with "BEGIN PUBLIC KEY" headers.
   * @param pem - PEM-encoded P-256 public key
   * @returns The decoder instance for chaining
   * @throws {Claim169Error} If the PEM is invalid
   */
  verifyWithEcdsaP256Pem(pem: string): IDecoder;

  /**
   * Allow decoding without signature verification.
   * WARNING: Credentials decoded with verification skipped (`verificationStatus === "skipped"`)
   * cannot be trusted. Use for testing only.
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
   * Re-enable timestamp validation (enabled by default).
   * When enabled, expired or not-yet-valid credentials will throw an error.
   * @returns The decoder instance for chaining
   */
  withTimestampValidation(): IDecoder;

  /**
   * Disable timestamp validation.
   * @returns The decoder instance for chaining
   */
  withoutTimestampValidation(): IDecoder;

  /**
   * Set clock skew tolerance in seconds.
   * Allows credentials to be accepted when clocks are slightly out of sync.
   * Applies when timestamp validation is enabled.
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
  verifyWith(verifier: VerifierCallback): IDecoder;

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
  decryptWith(decryptor: DecryptorCallback): IDecoder;

  /**
   * Decode the QR code with the configured options.
   * Requires either a verifier (verifyWithEd25519/verifyWithEcdsaP256) or
   * explicit allowUnverified() to be called first.
   * @returns The decoded result
   * @throws {Claim169Error} If decoding fails or no verification method specified
   */
  decode(): DecodeResult;
}
