/**
 * Biometric data from Claim 169
 */
export interface Biometric {
  /** Raw biometric data bytes */
  data: Uint8Array;
  /** Biometric format code */
  format: number;
  /** Biometric sub-format code */
  subFormat?: number;
  /** Issuer identifier */
  issuer?: string;
}

/**
 * CWT (CBOR Web Token) metadata
 */
export interface CwtMeta {
  /** Token issuer */
  issuer?: string;
  /** Token subject */
  subject?: string;
  /** Expiration timestamp (Unix seconds) */
  expiresAt?: number;
  /** Not-before timestamp (Unix seconds) */
  notBefore?: number;
  /** Issued-at timestamp (Unix seconds) */
  issuedAt?: number;
}

/**
 * Decoded Claim 169 identity data
 */
export interface Claim169 {
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
 * Verification status of the decoded QR code
 */
export type VerificationStatus = "verified" | "skipped" | "failed";

/**
 * Result of decoding a Claim 169 QR code
 */
export interface DecodeResult {
  /** Decoded identity claim */
  claim169: Claim169;
  /** CWT metadata */
  cwtMeta: CwtMeta;
  /** Signature verification status */
  verificationStatus: VerificationStatus;
}

/**
 * Options for decoding
 */
export interface DecodeOptions {
  /** Maximum decompressed size in bytes (default: 65536) */
  maxDecompressedBytes?: number;
  /** Skip biometric data parsing (default: false) */
  skipBiometrics?: boolean;
  /** Validate CWT timestamps (default: true) */
  validateTimestamps?: boolean;
  /** Clock skew tolerance in seconds for timestamp validation (default: 0) */
  clockSkewToleranceSeconds?: number;
}

/**
 * Error thrown when decoding fails
 */
export class Claim169Error extends Error {
  constructor(message: string) {
    super(message);
    this.name = "Claim169Error";
  }
}
