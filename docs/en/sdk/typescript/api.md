# API Reference

Complete API documentation for the claim169 TypeScript SDK.

## Classes

### Decoder

Builder-pattern decoder for Claim 169 QR codes.

```typescript
class Decoder implements IDecoder {
  constructor(qrText: string);

  verifyWithEd25519(publicKey: Uint8Array): Decoder;
  verifyWithEcdsaP256(publicKey: Uint8Array): Decoder;
  verifyWith(verifier: VerifierCallback): Decoder;
  allowUnverified(): Decoder;

  decryptWithAes256(key: Uint8Array): Decoder;
  decryptWithAes128(key: Uint8Array): Decoder;
  decryptWith(decryptor: DecryptorCallback): Decoder;

  skipBiometrics(): Decoder;
  withTimestampValidation(): Decoder;
  withoutTimestampValidation(): Decoder;
  clockSkewTolerance(seconds: number): Decoder;
  maxDecompressedBytes(bytes: number): Decoder;

  decode(): DecodeResult;
}
```

#### Constructor

```typescript
new Decoder(qrText: string)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `qrText` | string | Base45-encoded QR code content |

#### Methods

##### `verifyWithEd25519(publicKey)`

Verify signature with Ed25519 public key.

| Parameter | Type | Description |
|-----------|------|-------------|
| `publicKey` | Uint8Array | 32-byte Ed25519 public key |

**Returns**: `Decoder` for chaining

**Throws**: `Claim169Error` if key is invalid

##### `verifyWithEcdsaP256(publicKey)`

Verify signature with ECDSA P-256 public key.

| Parameter | Type | Description |
|-----------|------|-------------|
| `publicKey` | Uint8Array | SEC1-encoded P-256 key (33 or 65 bytes) |

**Returns**: `Decoder` for chaining

**Throws**: `Claim169Error` if key is invalid

##### `verifyWith(verifier)`

Verify with a custom verifier callback.

| Parameter | Type | Description |
|-----------|------|-------------|
| `verifier` | VerifierCallback | Custom verification function |

**Returns**: `Decoder` for chaining

##### `allowUnverified()`

Skip signature verification. **WARNING**: Use for testing only.

**Returns**: `Decoder` for chaining

##### `decryptWithAes256(key)`

Decrypt with AES-256-GCM.

| Parameter | Type | Description |
|-----------|------|-------------|
| `key` | Uint8Array | 32-byte AES-256 key |

**Returns**: `Decoder` for chaining

**Throws**: `Claim169Error` if key is invalid

##### `decryptWithAes128(key)`

Decrypt with AES-128-GCM.

| Parameter | Type | Description |
|-----------|------|-------------|
| `key` | Uint8Array | 16-byte AES-128 key |

**Returns**: `Decoder` for chaining

**Throws**: `Claim169Error` if key is invalid

##### `decryptWith(decryptor)`

Decrypt with a custom decryptor callback.

| Parameter | Type | Description |
|-----------|------|-------------|
| `decryptor` | DecryptorCallback | Custom decryption function |

**Returns**: `Decoder` for chaining

##### `skipBiometrics()`

Skip biometric data parsing.

**Returns**: `Decoder` for chaining

##### `withTimestampValidation()`

Enable exp/nbf timestamp validation (host-side in JavaScript).

**Returns**: `Decoder` for chaining

##### `withoutTimestampValidation()`

Disable exp/nbf timestamp validation.

**Returns**: `Decoder` for chaining

##### `clockSkewTolerance(seconds)`

Set clock skew tolerance.

| Parameter | Type | Description |
|-----------|------|-------------|
| `seconds` | number | Tolerance in seconds |

**Returns**: `Decoder` for chaining

##### `maxDecompressedBytes(bytes)`

Set maximum decompressed size.

| Parameter | Type | Description |
|-----------|------|-------------|
| `bytes` | number | Maximum size (default: 65536) |

**Returns**: `Decoder` for chaining

##### `decode()`

Execute the decode operation.

**Returns**: `DecodeResult`

**Throws**: `Claim169Error` on decode failure

---

### Encoder

Builder-pattern encoder for Claim 169 credentials.

```typescript
class Encoder implements IEncoder {
  constructor(claim169: Claim169Input, cwtMeta: CwtMetaInput);

  signWithEd25519(privateKey: Uint8Array): Encoder;
  signWithEcdsaP256(privateKey: Uint8Array): Encoder;
  signWith(signer: SignerCallback, algorithm: "EdDSA" | "ES256"): Encoder;
  allowUnsigned(): Encoder;

  encryptWithAes256(key: Uint8Array): Encoder;
  encryptWithAes128(key: Uint8Array): Encoder;
  encryptWith(encryptor: EncryptorCallback, algorithm: "A256GCM" | "A128GCM"): Encoder;

  skipBiometrics(): Encoder;

  encode(): string;
}
```

#### Constructor

```typescript
new Encoder(claim169: Claim169Input, cwtMeta: CwtMetaInput)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `claim169` | Claim169Input | Identity data to encode |
| `cwtMeta` | CwtMetaInput | CWT metadata |

#### Methods

##### `signWithEd25519(privateKey)`

Sign with Ed25519 private key.

| Parameter | Type | Description |
|-----------|------|-------------|
| `privateKey` | Uint8Array | 32-byte Ed25519 private key |

**Returns**: `Encoder` for chaining

**Throws**: `Claim169Error` if key is invalid

##### `signWithEcdsaP256(privateKey)`

Sign with ECDSA P-256 private key.

| Parameter | Type | Description |
|-----------|------|-------------|
| `privateKey` | Uint8Array | 32-byte P-256 private key (scalar) |

**Returns**: `Encoder` for chaining

**Throws**: `Claim169Error` if key is invalid

##### `signWith(signer, algorithm)`

Sign with a custom signer callback.

| Parameter | Type | Description |
|-----------|------|-------------|
| `signer` | SignerCallback | Custom signing function |
| `algorithm` | "EdDSA" \| "ES256" | Signature algorithm |

**Returns**: `Encoder` for chaining

##### `allowUnsigned()`

Skip signing. **WARNING**: Use for testing only.

**Returns**: `Encoder` for chaining

##### `encryptWithAes256(key)`

Encrypt with AES-256-GCM.

| Parameter | Type | Description |
|-----------|------|-------------|
| `key` | Uint8Array | 32-byte AES-256 key |

**Returns**: `Encoder` for chaining

**Throws**: `Claim169Error` if key is invalid

##### `encryptWithAes128(key)`

Encrypt with AES-128-GCM.

| Parameter | Type | Description |
|-----------|------|-------------|
| `key` | Uint8Array | 16-byte AES-128 key |

**Returns**: `Encoder` for chaining

**Throws**: `Claim169Error` if key is invalid

##### `encryptWith(encryptor, algorithm)`

Encrypt with a custom encryptor callback.

| Parameter | Type | Description |
|-----------|------|-------------|
| `encryptor` | EncryptorCallback | Custom encryption function |
| `algorithm` | "A256GCM" \| "A128GCM" | Encryption algorithm |

**Returns**: `Encoder` for chaining

##### `skipBiometrics()`

Skip biometric fields during encoding.

**Returns**: `Encoder` for chaining

##### `encode()`

Produce the QR string.

**Returns**: `string` - Base45-encoded QR data

**Throws**: `Claim169Error` on encode failure

---

### Claim169Error

Error class for SDK errors.

```typescript
class Claim169Error extends Error {
  constructor(message: string);
  name: "Claim169Error";
}
```

## Functions

### decode()

Convenience function for decoding.

```typescript
function decode(qrText: string, options?: DecodeOptions): DecodeResult;
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `qrText` | string | Base45-encoded QR content |
| `options` | DecodeOptions | Decode options |

**Returns**: `DecodeResult`

**Throws**: `Claim169Error` on failure

### version()

Get the library version.

```typescript
function version(): string;
```

**Returns**: Version string (e.g., "0.1.0-alpha.2")

### isLoaded()

Check if WASM module is loaded.

```typescript
function isLoaded(): boolean;
```

**Returns**: `true` if WASM is ready

### hexToBytes()

Convert hex string to bytes.

```typescript
function hexToBytes(hex: string): Uint8Array;
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `hex` | string | Hex string (optional 0x prefix, ignores whitespace) |

**Returns**: `Uint8Array`

**Throws**: `Claim169Error` if invalid hex

### bytesToHex()

Convert bytes to hex string.

```typescript
function bytesToHex(bytes: Uint8Array): string;
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `bytes` | Uint8Array | Bytes to convert |

**Returns**: Lowercase hex string

### generateNonce()

Generate a random 12-byte nonce.

```typescript
function generateNonce(): Uint8Array;
```

**Returns**: 12-byte `Uint8Array` for AES-GCM

## Interfaces

### DecodeResult

```typescript
interface DecodeResult {
  claim169: Claim169;
  cwtMeta: CwtMeta;
  verificationStatus: VerificationStatus;
}
```

### Claim169

```typescript
interface Claim169 {
  // Core identity
  id?: string;
  version?: string;
  language?: string;

  // Name fields
  fullName?: string;
  firstName?: string;
  middleName?: string;
  lastName?: string;

  // Demographics
  dateOfBirth?: string;
  gender?: number;
  address?: string;
  email?: string;
  phone?: string;
  nationality?: string;
  maritalStatus?: number;
  guardian?: string;

  // Additional
  secondaryFullName?: string;
  secondaryLanguage?: string;
  locationCode?: string;
  legalStatus?: string;
  countryOfIssuance?: string;

  // Photo
  photo?: Uint8Array;
  photoFormat?: number;
  bestQualityFingers?: Uint8Array;

  // Biometrics
  rightThumb?: Biometric[];
  rightPointerFinger?: Biometric[];
  rightMiddleFinger?: Biometric[];
  rightRingFinger?: Biometric[];
  rightLittleFinger?: Biometric[];
  leftThumb?: Biometric[];
  leftPointerFinger?: Biometric[];
  leftMiddleFinger?: Biometric[];
  leftRingFinger?: Biometric[];
  leftLittleFinger?: Biometric[];
  rightIris?: Biometric[];
  leftIris?: Biometric[];
  face?: Biometric[];
  rightPalm?: Biometric[];
  leftPalm?: Biometric[];
  voice?: Biometric[];
}
```

### Claim169Input

Input interface for encoding (same fields as Claim169, all optional).

### CwtMeta

```typescript
interface CwtMeta {
  issuer?: string;
  subject?: string;
  expiresAt?: number;
  notBefore?: number;
  issuedAt?: number;
}
```

### CwtMetaInput

Input interface for encoding (same fields as CwtMeta, all optional).

### Biometric

```typescript
interface Biometric {
  data: Uint8Array;
  format: number;
  subFormat?: number;
  issuer?: string;
}
```

### DecodeOptions

```typescript
interface DecodeOptions {
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
```

Notes:
- `validateTimestamps` defaults to `true` (host-side timestamp validation).

## Types

### VerificationStatus

```typescript
type VerificationStatus = "verified" | "skipped" | "failed";
```

### Algorithm

```typescript
type Algorithm = "EdDSA" | "ES256" | "A256GCM" | "A128GCM";
```

### SignerCallback

```typescript
type SignerCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  data: Uint8Array
) => Uint8Array;
```

### VerifierCallback

```typescript
type VerifierCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  data: Uint8Array,
  signature: Uint8Array
) => void;
```

### EncryptorCallback

```typescript
type EncryptorCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  nonce: Uint8Array,
  aad: Uint8Array,
  plaintext: Uint8Array
) => Uint8Array;
```

### DecryptorCallback

```typescript
type DecryptorCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  nonce: Uint8Array,
  aad: Uint8Array,
  ciphertext: Uint8Array
) => Uint8Array;
```

## Enums

### Gender

| Value | Meaning |
|-------|---------|
| 1 | Male |
| 2 | Female |
| 3 | Other |

### Marital Status

| Value | Meaning |
|-------|---------|
| 1 | Unmarried |
| 2 | Married |
| 3 | Divorced |

### Photo Format

| Value | Format |
|-------|--------|
| 1 | JPEG |
| 2 | JPEG2000 |
| 3 | AVIF |
| 4 | WebP |

### Biometric Format

| Value | Type |
|-------|------|
| 0 | Image |
| 1 | Template |
| 2 | Sound |
| 3 | BioHash |

### Image Sub-Format

| Value | Format |
|-------|--------|
| 0 | PNG |
| 1 | JPEG |
| 2 | JPEG2000 |
| 3 | AVIF |
| 4 | WebP |
| 5 | TIFF |
| 6 | WSQ |

### Template Sub-Format

| Value | Format |
|-------|--------|
| 0 | ANSI378 |
| 1 | ISO19794-2 |
| 2 | NIST |

### Sound Sub-Format

| Value | Format |
|-------|--------|
| 0 | WAV |
| 1 | MP3 |

## Exports

All public APIs are exported from the main module:

```typescript
// Classes
export { Decoder, Encoder, Claim169Error };

// Functions
export { decode, version, isLoaded, hexToBytes, bytesToHex, generateNonce };

// Types
export type {
  Algorithm,
  Biometric,
  Claim169,
  Claim169Input,
  CwtMeta,
  CwtMetaInput,
  DecodeOptions,
  DecodeResult,
  DecryptorCallback,
  EncryptorCallback,
  IDecoder,
  IEncoder,
  SignerCallback,
  VerificationStatus,
  VerifierCallback,
};
```
