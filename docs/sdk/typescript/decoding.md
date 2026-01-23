# Decoding Credentials

The `Decoder` class parses MOSIP Claim 169 QR codes and extracts identity data using a fluent builder API.

## Basic Decoding

### With Ed25519 Verification (Recommended)

```typescript
import { Decoder } from 'claim169';

const qrText = "6BF5YZB2..."; // QR code content
const publicKey = new Uint8Array(32); // Ed25519 public key

const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

console.log('ID:', result.claim169.id);
console.log('Name:', result.claim169.fullName);
console.log('Verified:', result.verificationStatus); // "verified"
```

### With ECDSA P-256 Verification

```typescript
// Compressed (33 bytes) or uncompressed (65 bytes) SEC1-encoded public key
const publicKey = new Uint8Array(33); // or 65 bytes

const result = new Decoder(qrText)
  .verifyWithEcdsaP256(publicKey)
  .decode();
```

### Without Verification (Testing Only)

```typescript
// WARNING: Never use in production!
const result = new Decoder(qrText)
  .allowUnverified()
  .decode();

console.log('Status:', result.verificationStatus); // "skipped"
```

## Decoding Encrypted Credentials

### AES-256-GCM Decryption

```typescript
const aesKey = new Uint8Array(32); // 32-byte AES-256 key
const publicKey = new Uint8Array(32); // Ed25519 public key

// Order matters: decrypt first, then verify
const result = new Decoder(qrText)
  .decryptWithAes256(aesKey)
  .verifyWithEd25519(publicKey)
  .decode();
```

### AES-128-GCM Decryption

```typescript
const aesKey = new Uint8Array(16); // 16-byte AES-128 key

const result = new Decoder(qrText)
  .decryptWithAes128(aesKey)
  .verifyWithEd25519(publicKey)
  .decode();
```

## Decoder Options

### Skip Biometrics

Skip biometric data parsing for faster decoding when only demographics are needed:

```typescript
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .skipBiometrics()
  .decode();

// Biometric fields will be undefined
console.log(result.claim169.face); // undefined
```

### Timestamp Validation

Enable validation of `exp` (expires at) and `nbf` (not before) claims:

```typescript
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .withTimestampValidation()
  .decode();
```

Note: Timestamp validation is disabled by default because WebAssembly does not have reliable access to system time.

### Clock Skew Tolerance

Allow some tolerance for clock drift between systems:

```typescript
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .withTimestampValidation()
  .clockSkewTolerance(60) // Allow 60 seconds drift
  .decode();
```

### Maximum Decompressed Size

Set a limit on decompressed data size to prevent decompression bombs:

```typescript
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .maxDecompressedBytes(32768) // 32KB limit
  .decode();
```

The default limit is 65536 bytes (64KB).

## Accessing Decoded Data

### Identity Data (Claim169)

```typescript
const { claim169 } = result;

// Demographics
console.log('ID:', claim169.id);
console.log('Full Name:', claim169.fullName);
console.log('First Name:', claim169.firstName);
console.log('Last Name:', claim169.lastName);
console.log('DOB:', claim169.dateOfBirth);
console.log('Gender:', claim169.gender);
console.log('Email:', claim169.email);
console.log('Phone:', claim169.phone);
console.log('Address:', claim169.address);
console.log('Nationality:', claim169.nationality);

// Photo
if (claim169.photo) {
  console.log('Photo size:', claim169.photo.byteLength);
  console.log('Photo format:', claim169.photoFormat);
}

// Biometrics
if (claim169.face && claim169.face.length > 0) {
  const faceData = claim169.face[0];
  console.log('Face biometric format:', faceData.format);
  console.log('Face data size:', faceData.data.byteLength);
}
```

### CWT Metadata

```typescript
const { cwtMeta } = result;

console.log('Issuer:', cwtMeta.issuer);
console.log('Subject:', cwtMeta.subject);
console.log('Issued At:', cwtMeta.issuedAt);
console.log('Expires At:', cwtMeta.expiresAt);
console.log('Not Before:', cwtMeta.notBefore);

// Check expiration
if (cwtMeta.expiresAt) {
  const expiresDate = new Date(cwtMeta.expiresAt * 1000);
  const isExpired = expiresDate < new Date();
  console.log('Expired:', isExpired);
}
```

### Verification Status

```typescript
const { verificationStatus } = result;

switch (verificationStatus) {
  case 'verified':
    console.log('Signature verified successfully');
    break;
  case 'skipped':
    console.log('Verification was skipped (allowUnverified)');
    break;
  case 'failed':
    console.log('Signature verification failed');
    break;
}
```

## Using the Convenience Function

For simpler use cases, use the `decode()` function:

```typescript
import { decode, type DecodeOptions } from 'claim169';

// With verification
const result = decode(qrText, {
  verifyWithEd25519: publicKey,
});

// With all options
const options: DecodeOptions = {
  verifyWithEd25519: publicKey,
  // or: verifyWithEcdsaP256: ecdsaPublicKey,
  // or: allowUnverified: true,

  decryptWithAes256: aesKey,
  // or: decryptWithAes128: aes128Key,

  skipBiometrics: true,
  validateTimestamps: true,
  clockSkewToleranceSeconds: 60,
  maxDecompressedBytes: 32768,
};

const result = decode(qrText, options);
```

## Error Handling

```typescript
import { Decoder, Claim169Error } from 'claim169';

try {
  const result = new Decoder(qrText)
    .verifyWithEd25519(publicKey)
    .decode();

  console.log('Decoded:', result.claim169.fullName);
} catch (error) {
  if (error instanceof Claim169Error) {
    console.error('Decode failed:', error.message);

    // Check for specific error types
    if (error.message.includes('Base45')) {
      console.log('Invalid QR code encoding');
    } else if (error.message.includes('signature')) {
      console.log('Signature verification failed');
    } else if (error.message.includes('expired')) {
      console.log('Credential has expired');
    }
  }
}
```

### Common Errors

| Error | Cause |
|-------|-------|
| `Ed25519 public key must be 32 bytes` | Invalid Ed25519 key size |
| `ECDSA P-256 public key must be 33 bytes (compressed) or 65 bytes (uncompressed)` | Invalid ECDSA key size |
| `AES-256 key must be 32 bytes` | Invalid decryption key size |
| `Must call verifyWith...() or allowUnverified()` | No verification method specified |
| `Base45Decode` | Invalid Base45 encoding |
| `Decompress` | zlib decompression failed |
| `CoseParse` | Invalid COSE structure |
| `Claim169NotFound` | Missing claim 169 in CWT |
| `SignatureError` | Signature verification failed |
| `DecryptionError` | Decryption failed (wrong key or corrupted) |

## Full Builder Chain Example

```typescript
const result = new Decoder(qrText)
  .decryptWithAes256(aesKey)
  .verifyWithEd25519(publicKey)
  .skipBiometrics()
  .withTimestampValidation()
  .clockSkewTolerance(120)
  .maxDecompressedBytes(65536)
  .decode();
```

## Working with Biometric Data

When biometric data is present:

```typescript
const { claim169 } = result;

// Fingerprints
const fingerprints = [
  { name: 'Right Thumb', data: claim169.rightThumb },
  { name: 'Right Pointer', data: claim169.rightPointerFinger },
  { name: 'Right Middle', data: claim169.rightMiddleFinger },
  { name: 'Right Ring', data: claim169.rightRingFinger },
  { name: 'Right Little', data: claim169.rightLittleFinger },
  { name: 'Left Thumb', data: claim169.leftThumb },
  { name: 'Left Pointer', data: claim169.leftPointerFinger },
  { name: 'Left Middle', data: claim169.leftMiddleFinger },
  { name: 'Left Ring', data: claim169.leftRingFinger },
  { name: 'Left Little', data: claim169.leftLittleFinger },
];

for (const fp of fingerprints) {
  if (fp.data && fp.data.length > 0) {
    console.log(`${fp.name}: ${fp.data[0].data.byteLength} bytes`);
  }
}

// Iris
if (claim169.rightIris) {
  console.log('Right iris data available');
}
if (claim169.leftIris) {
  console.log('Left iris data available');
}

// Face
if (claim169.face && claim169.face.length > 0) {
  const face = claim169.face[0];
  console.log('Face biometric:', {
    format: face.format,
    subFormat: face.subFormat,
    size: face.data.byteLength,
    issuer: face.issuer,
  });
}

// Voice
if (claim169.voice) {
  console.log('Voice biometric available');
}
```

### Biometric Format Codes

| Format | Description |
|--------|-------------|
| 0 | Image |
| 1 | Template |
| 2 | Sound |
| 3 | BioHash |

### Image Sub-Formats

| Code | Format |
|------|--------|
| 0 | PNG |
| 1 | JPEG |
| 2 | JPEG2000 |
| 3 | AVIF |
| 4 | WebP |
| 5 | TIFF |
| 6 | WSQ |

## Next Steps

- [Encryption](encryption.md) - Working with encrypted credentials
- [Custom Crypto](custom-crypto.md) - HSM and KMS integration
- [API Reference](api.md) - Complete Decoder API
