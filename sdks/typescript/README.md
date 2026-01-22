# claim169

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

```typescript
import { decode } from 'claim169';

// Decode a QR code (without signature verification)
const qrText = "6BF5YZB2...";  // Base45-encoded QR content
const result = decode(qrText);

// Access identity data
console.log(`ID: ${result.claim169.id}`);
console.log(`Name: ${result.claim169.fullName}`);
console.log(`DOB: ${result.claim169.dateOfBirth}`);

// Access CWT metadata
console.log(`Issuer: ${result.cwtMeta.issuer}`);
console.log(`Expires: ${result.cwtMeta.expiresAt}`);
```

## Decode Options

```typescript
// Skip biometric data for faster parsing
const result = decode(qrText, { skipBiometrics: true });

// Limit decompressed size (default: 64KB)
const result = decode(qrText, { maxDecompressedBytes: 32768 });

// Enable timestamp validation (disabled by default in WASM)
const result = decode(qrText, { validateTimestamps: true });
```

## Data Model

### DecodeResult

```typescript
interface DecodeResult {
  claim169: Claim169;           // Identity data
  cwtMeta: CwtMeta;             // Token metadata
  verificationStatus: VerificationStatus;  // "verified" | "skipped" | "failed"
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
  rightPalm?: Biometric[];
  leftPalm?: Biometric[];
  voice?: Biometric[];
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
  const result = decode(qrText);
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

## Browser Usage

```html
<script type="module">
  import { decode } from './node_modules/claim169/dist/index.js';

  const qrText = "6BF5YZB2...";
  const result = decode(qrText);
  console.log(result.claim169.fullName);
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

## Limitations

### Signature Verification

The current TypeScript SDK does not include built-in signature verification or decryption capabilities due to WASM/JavaScript callback complexity. For use cases requiring verification:

1. **Use the Python SDK** for server-side verification
2. **Verify externally** by extracting the signature and data from the decoded result
3. **Use WebCrypto API** for custom verification after decoding

Future versions may add WebCrypto-based verification hooks.

### Timestamp Validation

Timestamp validation is disabled by default in the WASM build because WebAssembly does not have reliable access to system time. Enable it explicitly if your environment supports it:

```typescript
const result = decode(qrText, { validateTimestamps: true });
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
