# claim169

> **Alpha Software**: This library is under active development. APIs may change without notice. Not recommended for production use without thorough testing.

[![npm](https://img.shields.io/npm/v/claim169.svg)](https://www.npmjs.com/package/claim169)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A TypeScript/JavaScript library for decoding MOSIP Claim 169 QR codes. Built on Rust/WebAssembly for performance and security.

## Installation

```bash
npm install claim169
```

## Overview

MOSIP Claim 169 defines a standard for encoding identity data in QR codes using:
- CBOR encoding with numeric keys for compactness
- CWT (CBOR Web Token) for standard claims
- COSE_Sign1 for digital signatures
- COSE_Encrypt0 for optional encryption
- zlib compression + Base45 encoding for QR-friendly output

## Quick Start

### Builder Pattern (Recommended)

```typescript
import { Decoder } from 'claim169';

// Decode with Ed25519 signature verification (recommended)
const publicKey = new Uint8Array(32);  // Your 32-byte public key
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

console.log(`ID: ${result.claim169.id}`);
console.log(`Name: ${result.claim169.fullName}`);
console.log(`Issuer: ${result.cwtMeta.issuer}`);
console.log(`Verified: ${result.verificationStatus}`);  // "verified"

// Decode without verification (testing only)
const result = new Decoder(qrText)
  .allowUnverified()
  .decode();
```

### `decode()` Convenience Function

> **Security note**: `decode()` requires a verification key unless you explicitly set `allowUnverified: true` (testing only). Use the `Decoder` builder API in production.

```typescript
import { decode, type DecodeOptions } from 'claim169';

// Simple decode (testing only)
const result = decode(qrText, { allowUnverified: true });

// With options
const options: DecodeOptions = {
  maxDecompressedBytes: 32768,  // 32KB limit
  skipBiometrics: true,         // Skip biometric parsing
  validateTimestamps: false,    // Disabled by default in WASM
  allowUnverified: true,        // Explicit opt-out (testing only)
};

const result = decode(qrText, options);
```

## Decoder Class

The `Decoder` class provides a fluent builder API:

```typescript
import { Decoder } from 'claim169';

// Decode with Ed25519 verification
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Decode with ECDSA P-256 verification
const result = new Decoder(qrText)
  .verifyWithEcdsaP256(publicKey)  // 33 or 65 bytes SEC1 encoded
  .decode();

// Decrypt then verify (for encrypted credentials)
const result = new Decoder(qrText)
  .decryptWithAes256(aesKey)
  .verifyWithEd25519(publicKey)
  .decode();

// With additional options
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .skipBiometrics()              // Skip biometric data
  .withTimestampValidation()     // Enable timestamp validation
  .clockSkewTolerance(60)        // 60 seconds tolerance
  .maxDecompressedBytes(32768)   // 32KB max size
  .decode();
```

### Decoder Methods

| Method | Description |
|--------|-------------|
| `verifyWithEd25519(publicKey)` | Verify with Ed25519 (32 bytes) |
| `verifyWithEcdsaP256(publicKey)` | Verify with ECDSA P-256 (33 or 65 bytes) |
| `verifyWith(callback)` | Verify with custom callback (HSM, cloud KMS, etc.) |
| `decryptWithAes256(key)` | Decrypt with AES-256-GCM (32 bytes) |
| `decryptWithAes128(key)` | Decrypt with AES-128-GCM (16 bytes) |
| `decryptWith(callback)` | Decrypt with custom callback (HSM, cloud KMS, etc.) |
| `allowUnverified()` | Skip verification (testing only) |
| `skipBiometrics()` | Skip biometric data parsing |
| `withTimestampValidation()` | Enable exp/nbf validation |
| `clockSkewTolerance(seconds)` | Set clock skew tolerance |
| `maxDecompressedBytes(bytes)` | Set max decompressed size |
| `decode()` | Execute the decode operation |

## Encoding

The `Encoder` class creates MOSIP Claim 169 QR code data from identity information.
In production, keys should be provisioned and managed externally (HSM/KMS or secure key management). The examples below assume you already have key material.

```typescript
import { Encoder, Decoder, Claim169Input, CwtMetaInput, generateNonce } from 'claim169';

// Create identity data
const claim169: Claim169Input = {
  id: "123456789",
  fullName: "John Doe",
  dateOfBirth: "1990-01-15",
  gender: 1,  // Male
};

// Create CWT metadata
const cwtMeta: CwtMetaInput = {
  issuer: "https://issuer.example.com",
  expiresAt: 1800000000,
};

// Encode with Ed25519 signature
const privateKey = new Uint8Array(32);  // Your 32-byte private key
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();

// Encode with signature and AES-256 encryption
const aesKey = new Uint8Array(32);  // Your 32-byte AES key
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encryptWithAes256(aesKey)
  .encode();

// Unsigned (testing only)
const qrData = new Encoder(claim169, cwtMeta)
  .allowUnsigned()
  .encode();
```

### Encoder Methods

| Method | Description |
|--------|-------------|
| `signWithEd25519(privateKey)` | Sign with Ed25519 |
| `signWithEcdsaP256(privateKey)` | Sign with ECDSA P-256 |
| `signWith(callback, algorithm)` | Sign with custom callback (HSM, cloud KMS, etc.) |
| `encryptWithAes256(key)` | Encrypt with AES-256-GCM |
| `encryptWithAes128(key)` | Encrypt with AES-128-GCM |
| `encryptWith(callback, algorithm)` | Encrypt with custom callback (HSM, cloud KMS, etc.) |
| `allowUnsigned()` | Allow unsigned (testing only) |
| `skipBiometrics()` | Skip biometric fields |
| `encode()` | Produce the QR string |

### generateNonce()

Generate a cryptographically secure random nonce for encryption:

```typescript
import { generateNonce } from 'claim169';

const nonce = generateNonce();  // Returns 12-byte Uint8Array
```

## Custom Crypto Providers

For integrating with external key management systems like HSMs, cloud KMS (AWS KMS, Google Cloud KMS, Azure Key Vault), smart cards, TPMs, or remote signing services, use the custom callback methods.

### Custom Signer

```typescript
import { Encoder, SignerCallback, Claim169Input, CwtMetaInput } from 'claim169';

// Example: Sign with a cloud KMS
const mySigner: SignerCallback = (algorithm, keyId, data) => {
  // Call your crypto provider
  // algorithm: "EdDSA" or "ES256"
  // keyId: optional key identifier (Uint8Array or null)
  // data: the COSE Sig_structure to sign (Uint8Array)
  const signature = myKms.sign({ keyId, data, algorithm });
  return signature;  // Uint8Array: 64 bytes for EdDSA, 64 bytes for ES256
};

const claim: Claim169Input = { id: "123", fullName: "John Doe" };
const meta: CwtMetaInput = { issuer: "https://issuer.example" };

const qrData = new Encoder(claim, meta)
  .signWith(mySigner, "EdDSA")
  .encode();
```

### Custom Verifier

```typescript
import { Decoder, VerifierCallback } from 'claim169';

// Example: Verify with an HSM
const myVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
  // Call your crypto provider
  // Throw an error if verification fails
  const result = myHsm.verify({ keyId, data, signature, algorithm });
  if (!result.valid) {
    throw new Error("Signature verification failed");
  }
};

const result = new Decoder(qrText)
  .verifyWith(myVerifier)
  .decode();
```

### Custom Encryptor

```typescript
import { Encoder, EncryptorCallback } from 'claim169';

// Example: Encrypt with cloud KMS
const myEncryptor: EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => {
  // algorithm: "A256GCM" or "A128GCM"
  // nonce: 12-byte IV
  // aad: additional authenticated data
  // plaintext: data to encrypt
  const ciphertext = myKms.encrypt({ keyId, nonce, aad, plaintext });
  return ciphertext;  // Uint8Array: plaintext + 16-byte auth tag
};

const qrData = new Encoder(claim, meta)
  .signWithEd25519(signingKey)
  .encryptWith(myEncryptor, "A256GCM")
  .encode();
```

### Custom Decryptor

```typescript
import { Decoder, DecryptorCallback } from 'claim169';

// Example: Decrypt with cloud KMS
const myDecryptor: DecryptorCallback = (algorithm, keyId, nonce, aad, ciphertext) => {
  // algorithm: "A256GCM" or "A128GCM"
  // ciphertext includes the auth tag
  const plaintext = myKms.decrypt({ keyId, nonce, aad, ciphertext });
  return plaintext;  // Uint8Array
};

const result = new Decoder(qrText)
  .decryptWith(myDecryptor)
  .verifyWithEd25519(publicKey)
  .decode();
```

### Callback Type Definitions

```typescript
// Signer: (algorithm, keyId, data) => signature
type SignerCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  data: Uint8Array
) => Uint8Array;

// Verifier: (algorithm, keyId, data, signature) => void (throw on failure)
type VerifierCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  data: Uint8Array,
  signature: Uint8Array
) => void;

// Encryptor: (algorithm, keyId, nonce, aad, plaintext) => ciphertext
type EncryptorCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  nonce: Uint8Array,
  aad: Uint8Array,
  plaintext: Uint8Array
) => Uint8Array;

// Decryptor: (algorithm, keyId, nonce, aad, ciphertext) => plaintext
type DecryptorCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  nonce: Uint8Array,
  aad: Uint8Array,
  ciphertext: Uint8Array
) => Uint8Array;
```

## Data Model

### DecodeResult

```typescript
interface DecodeResult {
  claim169: Claim169;                    // Identity data
  cwtMeta: CwtMeta;                      // Token metadata
  verificationStatus: VerificationStatus; // "verified" | "skipped" | "failed"
}
```

### Claim169

```typescript
interface Claim169 {
  // Demographics
  id?: string;                  // Unique identifier
  fullName?: string;            // Full name
  firstName?: string;           // First name
  middleName?: string;          // Middle name
  lastName?: string;            // Last name
  dateOfBirth?: string;         // ISO 8601 format
  gender?: number;              // 1=Male, 2=Female, 3=Other
  address?: string;             // Address
  email?: string;               // Email address
  phone?: string;               // Phone number
  nationality?: string;         // Nationality code
  maritalStatus?: number;       // Marital status code
  guardian?: string;            // Guardian name

  // Additional fields
  version?: string;             // Claim version
  language?: string;            // Primary language code
  secondaryFullName?: string;   // Secondary full name
  secondaryLanguage?: string;   // Secondary language code
  locationCode?: string;        // Location code
  legalStatus?: string;         // Legal status
  countryOfIssuance?: string;   // Country of issuance

  // Photo
  photo?: Uint8Array;           // Photo data
  photoFormat?: number;         // Photo format code

  // Biometrics (when present)
  face?: Biometric[];           // Face biometrics
  rightThumb?: Biometric[];     // Right thumb fingerprint
  // ... (all finger/iris/palm biometrics)
}
```

### CwtMeta

```typescript
interface CwtMeta {
  issuer?: string;              // Token issuer
  subject?: string;             // Token subject
  expiresAt?: number;           // Expiration timestamp (Unix seconds)
  notBefore?: number;           // Not-before timestamp
  issuedAt?: number;            // Issued-at timestamp
}
```

### Biometric

```typescript
interface Biometric {
  data: Uint8Array;             // Raw biometric data
  format: number;               // Biometric format code
  subFormat?: number;           // Sub-format code
  issuer?: string;              // Issuer identifier
}
```

## Error Handling

```typescript
import { decode, Claim169Error } from 'claim169';

try {
  const result = decode(qrText, { allowUnverified: true });
} catch (error) {
  if (error instanceof Claim169Error) {
    // Handle decode error
    console.error("Decode failed:", error.message);
  }
}
```

Error messages indicate the specific failure:
- `Base45Decode`: Invalid Base45 encoding
- `Decompress`: zlib decompression failed
- `CoseParse`: Invalid COSE structure
- `CwtParse`: Invalid CWT structure
- `Claim169NotFound`: Missing claim 169
- `SignatureError`: Signature verification failed
- `DecryptionError`: Decryption failed

## Notes

### Timestamp Validation

Timestamp validation is disabled by default because WebAssembly does not have reliable access to system time. Enable it explicitly if your environment provides accurate time:

```typescript
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .withTimestampValidation()
  .clockSkewTolerance(60)  // Allow 60 seconds clock drift
  .decode();
```

### Secure by Default

The decoder requires explicit verification configuration. You must call one of:
- `verifyWithEd25519(publicKey)` - Verify with Ed25519
- `verifyWithEcdsaP256(publicKey)` - Verify with ECDSA P-256
- `allowUnverified()` - Explicitly skip verification (testing only)

This prevents accidentally processing unverified credentials.

## Browser Usage

```html
<script type="module">
  import { Decoder } from './node_modules/claim169/dist/index.js';

  // Your issuer's public key (32 bytes for Ed25519)
  const publicKey = new Uint8Array([/* ... */]);

  const qrText = "6BF5YZB2...";
  const result = new Decoder(qrText)
    .verifyWithEd25519(publicKey)
    .decode();

  if (result.verificationStatus === "verified") {
    console.log("Verified:", result.claim169.fullName);
  }
</script>
```

## Bundler Configuration

### Vite

```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [wasm(), topLevelAwait()],
});
```

### Webpack

```javascript
// webpack.config.js
module.exports = {
  experiments: {
    asyncWebAssembly: true,
  },
};
```

## Utility Functions

```typescript
import { version, isLoaded } from 'claim169';

// Get library version
console.log(version());  // "0.1.0"

// Check if WASM module is loaded
console.log(isLoaded());  // true
```

## Development

### Building from Source

```bash
# Install dependencies
npm install

# Build WASM (requires Rust and wasm-pack)
npm run build:wasm

# Build TypeScript
npm run build:ts

# Or build everything
npm run build
```

### Running Tests

```bash
npm test
```

### Prerequisites

- Node.js 18+
- Rust toolchain (for building WASM)
- wasm-pack (`cargo install wasm-pack`)

## License

MIT License - See LICENSE file for details.
