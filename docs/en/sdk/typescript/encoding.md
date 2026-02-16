# Encoding Credentials

The `Encoder` class creates MOSIP Claim 169 QR code data from identity information using a fluent builder API.

## Basic Encoding

### Signed Credential (Recommended)

```typescript
import { Encoder, Gender, type Claim169Input, type CwtMetaInput } from 'claim169';

const claim169: Claim169Input = {
  id: "MOSIP-123456789",
  fullName: "John Doe",
  dateOfBirth: "1985-03-15",
  gender: Gender.Male,
};

const cwtMeta: CwtMetaInput = {
  issuer: "https://issuer.example.com",
  issuedAt: Math.floor(Date.now() / 1000),
  expiresAt: Math.floor(Date.now() / 1000) + 86400 * 365,
};

// Ed25519 private key (32 bytes)
const privateKey = new Uint8Array(32);

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();
```

### Unsigned Credential (Testing Only)

```typescript
// WARNING: Unsigned credentials cannot be verified
const qrData = new Encoder(claim169, cwtMeta)
  .allowUnsigned()
  .encode();
```

## Signature Algorithms

### Ed25519 (EdDSA)

The recommended algorithm for most use cases:

```typescript
// 32-byte Ed25519 private key
const privateKey = new Uint8Array(32);

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();
```

### ECDSA P-256 (ES256)

For compatibility with systems requiring NIST curves:

```typescript
// 32-byte ECDSA P-256 private key (scalar)
const privateKey = new Uint8Array(32);

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEcdsaP256(privateKey)
  .encode();
```

## Full Demographics Example

```typescript
import {
  Encoder, Gender, MaritalStatus,
  type Claim169Input, type CwtMetaInput,
} from 'claim169';

const claim169: Claim169Input = {
  // Core identity
  id: "MOSIP-987654321",
  version: "1.0.0",
  language: "en",

  // Name fields
  fullName: "Maria Garcia-Rodriguez",
  firstName: "Maria",
  middleName: "Elena",
  lastName: "Garcia-Rodriguez",

  // Demographics
  dateOfBirth: "1992-08-22",
  gender: Gender.Female,

  // Contact
  address: "123 Main Street, Apt 4B, Springfield, IL 62701",
  email: "maria.garcia@example.com",
  phone: "+1-555-123-4567",

  // Legal
  nationality: "US",
  maritalStatus: MaritalStatus.Married,
  guardian: "Carlos Garcia",
  legalStatus: "citizen",
  countryOfIssuance: "US",

  // Location
  locationCode: "US-IL-62701",

  // Secondary language
  secondaryFullName: "Maria Garcia-Rodriguez",
  secondaryLanguage: "es",
};

const cwtMeta: CwtMetaInput = {
  issuer: "https://gov.example.com/identity",
  subject: "maria.garcia@example.com",
  issuedAt: Math.floor(Date.now() / 1000),
  expiresAt: Math.floor(Date.now() / 1000) + 86400 * 365 * 5, // 5 years
  notBefore: Math.floor(Date.now() / 1000),
};

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();
```

## Adding Photos

```typescript
const claim169: Claim169Input = {
  id: "PHOTO-001",
  fullName: "Photo Test",

  // Photo as JPEG
  photo: await readPhotoAsBytes('photo.jpg'),
  photoFormat: PhotoFormat.Jpeg,  // or PhotoFormat.Jpeg2000, .Avif, .Webp
};

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();
```

## Encrypted Credentials

### AES-256-GCM Encryption

```typescript
// Sign then encrypt
const signKey = new Uint8Array(32);   // Ed25519 private key
const encryptKey = new Uint8Array(32); // AES-256 key

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(signKey)
  .encryptWithAes256(encryptKey)
  .encode();
```

### AES-128-GCM Encryption

```typescript
const signKey = new Uint8Array(32);   // Ed25519 private key
const encryptKey = new Uint8Array(16); // AES-128 key

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(signKey)
  .encryptWithAes128(encryptKey)
  .encode();
```

## Encoder Options

### Compression

By default, the encoder uses zlib compression (spec-compliant). You can choose a different compression mode:

```typescript
// No compression (for tiny payloads where zlib adds overhead)
const result = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .compression("none")
  .encode();

// Adaptive: uses zlib if it reduces size, otherwise stores raw
const result = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .compression("adaptive")
  .encode();
```

Supported compression modes:

| Mode | Spec-compliant | Description |
|------|:-:|-------------|
| `"zlib"` | Yes | Default. Standard zlib/DEFLATE compression |
| `"none"` | No | No compression |
| `"adaptive"` | No | Picks zlib if it reduces size, otherwise stores raw |
| `"brotli:N"` | No | Brotli at quality N (0–11). Requires brotli feature |
| `"adaptive-brotli:N"` | No | Picks brotli if it reduces size, otherwise stores raw |

!!! warning "Interoperability"
    Non-standard compression modes produce credentials that only this library can decode. Use them only in closed ecosystems.

### Skip Biometrics

Skip biometric fields during encoding:

```typescript
const result = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .skipBiometrics()
  .encode();
```

## Method Chaining

All encoder methods return the encoder instance for fluent chaining:

```typescript
const result = new Encoder(claim169, cwtMeta)
  .signWithEd25519(signKey)
  .skipBiometrics()
  .compression("adaptive")
  .encryptWithAes256(encryptKey)
  .encode();
```

## Encode Result

The `encode()` method returns an object with the QR data, compression info, and warnings:

```typescript
const result = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();

console.log('QR data:', result.qrData);
console.log('Compression used:', result.compressionUsed);  // "zlib", "brotli", or "none"

for (const warning of result.warnings) {
  console.log(`Warning [${warning.code}]: ${warning.message}`);
}
```

| Field | Type | Description |
|-------|------|-------------|
| `qrData` | `string` | The Base45-encoded string for QR code generation |
| `compressionUsed` | `string` | Compression applied: `"zlib"`, `"brotli"`, or `"none"` |
| `warnings` | `Array<{code, message}>` | Non-fatal warnings (e.g., `non_standard_compression`) |

## Error Handling

```typescript
import { Encoder, Claim169Error } from 'claim169';

try {
  const result = new Encoder(claim169, cwtMeta)
    .signWithEd25519(privateKey)
    .encode();
} catch (error) {
  if (error instanceof Claim169Error) {
    console.error('Encoding failed:', error.message);
  }
}
```

### Common Errors

| Error | Cause |
|-------|-------|
| `Ed25519 private key must be 32 bytes` | Invalid Ed25519 key size |
| `ECDSA P-256 private key must be 32 bytes` | Invalid ECDSA key size |
| `AES-256 key must be 32 bytes` | Invalid AES-256 key size |
| `AES-128 key must be 16 bytes` | Invalid AES-128 key size |
| `Must call signWith...() or allowUnsigned()` | No signing method specified |

## CWT Metadata Fields

| Field | Type | Description |
|-------|------|-------------|
| `issuer` | string | Token issuer URL or identifier |
| `subject` | string | Subject identifier (e.g., user email) |
| `issuedAt` | number | Unix timestamp when issued |
| `expiresAt` | number | Unix timestamp when credential expires |
| `notBefore` | number | Unix timestamp when credential becomes valid |

## Enum Constants

The SDK exports typed enum constants so you can avoid magic numbers:

### Gender

| Constant | Code | Value |
|----------|------|-------|
| `Gender.Male` | 1 | Male |
| `Gender.Female` | 2 | Female |
| `Gender.Other` | 3 | Other |

### Marital Status

| Constant | Code | Value |
|----------|------|-------|
| `MaritalStatus.Unmarried` | 1 | Unmarried |
| `MaritalStatus.Married` | 2 | Married |
| `MaritalStatus.Divorced` | 3 | Divorced |

### Photo Format

| Constant | Code | Format |
|----------|------|--------|
| `PhotoFormat.Jpeg` | 1 | JPEG |
| `PhotoFormat.Jpeg2000` | 2 | JPEG2000 |
| `PhotoFormat.Avif` | 3 | AVIF |
| `PhotoFormat.Webp` | 4 | WebP |

## Nonce Generation

For encryption operations, generate secure random nonces:

```typescript
import { generateNonce } from 'claim169';

// Generate a 12-byte nonce for AES-GCM
const nonce = generateNonce();
console.log('Nonce length:', nonce.length); // 12
```

## Roundtrip Example

Encode and decode to verify:

```typescript
import { Encoder, Decoder } from 'claim169';

// Encode
const claim169 = { id: "TEST-001", fullName: "Test User" };
const cwtMeta = { issuer: "https://test.example" };

const encodeResult = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();

// Decode and verify
const publicKey = derivePublicKey(privateKey);
const result = new Decoder(encodeResult.qrData)
  .verifyWithEd25519(publicKey)
  .decode();

console.log('Roundtrip:', result.claim169.id === claim169.id); // true
console.log('Verified:', result.verificationStatus); // "verified"
```

## Next Steps

- [Encryption](encryption.md) - Detailed encryption examples
- [Custom Crypto](custom-crypto.md) - HSM and cloud KMS integration
- [API Reference](api.md) - Complete Encoder API
