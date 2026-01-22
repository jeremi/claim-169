# claim169-wasm

> **Alpha Software**: This library is under active development. APIs may change without notice. Not recommended for production use without thorough testing.

[![npm](https://img.shields.io/npm/v/claim169.svg)](https://www.npmjs.com/package/claim169)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

WebAssembly bindings for the MOSIP Claim 169 QR code decoder.

This crate compiles the `claim169-core` library to WebAssembly, enabling Claim 169 decoding in browsers and Node.js environments.

## Installation

This package is typically consumed through the [TypeScript SDK](../../sdks/typescript). For direct WASM usage:

```bash
npm install claim169
```

## Usage

### Builder Pattern (Recommended)

```typescript
import init, { Decoder } from 'claim169';

// Initialize WASM module
await init();

// Decode with Ed25519 signature verification (recommended)
const publicKey = new Uint8Array(32);  // Your 32-byte public key
const result = new Decoder(qrText)
    .verifyWithEd25519(publicKey)
    .decode();

console.log(result.claim169.fullName);
console.log(result.verificationStatus);  // "verified"

// Decode encrypted credential with verification
const result = new Decoder(qrText)
    .decryptWithAes256(aesKey)
    .verifyWithEd25519(publicKey)
    .decode();

// Decode without verification (testing only)
const result = new Decoder(qrText)
    .allowUnverified()
    .decode();
```

### Encoding

```javascript
import init, { Encoder, Decoder, generateNonce } from 'claim169';

await init();

// Create identity data
const claim169 = {
  id: "123456789",
  fullName: "John Doe",
  dateOfBirth: "1990-01-15",
  gender: 1,
};

// Create CWT metadata
const cwtMeta = {
  issuer: "https://issuer.example.com",
  expiresAt: 1800000000,
};

// Encode with Ed25519 signature
// (keys should be provisioned externally in production)
const privateKey = new Uint8Array(32);
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();

// Encode with signature and encryption
const aesKey = new Uint8Array(32);
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encryptWithAes256(aesKey)
  .encode();
```

## API

### Decoder

Builder-pattern class for decoding Claim 169 QR codes.

```javascript
const decoder = new Decoder(qrText);

// Verification methods (one required)
decoder.verifyWithEd25519(publicKey);   // Verify with Ed25519 (32 bytes)
decoder.verifyWithEcdsaP256(publicKey); // Verify with ECDSA P-256 (33 or 65 bytes)
decoder.allowUnverified();              // Skip verification (testing only)

// Decryption methods (optional)
decoder.decryptWithAes256(key);         // Decrypt with AES-256-GCM (32 bytes)
decoder.decryptWithAes128(key);         // Decrypt with AES-128-GCM (16 bytes)

// Configuration methods (chainable)
decoder.skipBiometrics();               // Skip biometric data parsing
decoder.withTimestampValidation();      // Enable timestamp validation
decoder.clockSkewTolerance(60);         // Set clock skew tolerance (seconds)
decoder.maxDecompressedBytes(32768);    // Set max decompressed size

// Execute decode
const result = decoder.decode();
```

| Method | Description |
|--------|-------------|
| `verifyWithEd25519(publicKey)` | Verify with Ed25519 (32 bytes) |
| `verifyWithEcdsaP256(publicKey)` | Verify with ECDSA P-256 (33 or 65 bytes SEC1) |
| `allowUnverified()` | Skip verification (testing only) |
| `decryptWithAes256(key)` | Decrypt with AES-256-GCM (32 bytes) |
| `decryptWithAes128(key)` | Decrypt with AES-128-GCM (16 bytes) |
| `skipBiometrics()` | Skip biometric data parsing |
| `withTimestampValidation()` | Enable exp/nbf validation |
| `clockSkewTolerance(seconds)` | Set clock skew tolerance |
| `maxDecompressedBytes(bytes)` | Set max decompressed size |
| `decode()` | Execute the decode operation |

### Encoder

Builder-pattern class for encoding Claim 169 credentials.

| Method | Description |
|--------|-------------|
| `signWithEd25519(privateKey)` | Sign with Ed25519 (32 bytes) |
| `signWithEcdsaP256(privateKey)` | Sign with ECDSA P-256 (32 bytes) |
| `encryptWithAes256(key)` | Encrypt with AES-256-GCM (32 bytes) |
| `encryptWithAes128(key)` | Encrypt with AES-128-GCM (16 bytes) |
| `allowUnsigned()` | Allow unsigned (testing only) |
| `skipBiometrics()` | Skip biometric fields |
| `encode()` | Produce the QR string |

### Utility Functions

| Function | Description |
|----------|-------------|
| `generateNonce()` | Generate a cryptographically secure 12-byte nonce |
| `version()` | Get library version |
| `isLoaded()` | Check if WASM module is ready |

## Notes

### Timestamp Validation

Timestamp validation is disabled by default because WebAssembly doesn't have reliable access to system time. Enable it explicitly if your environment provides accurate time:

```javascript
const result = new Decoder(qrText)
    .verifyWithEd25519(publicKey)
    .withTimestampValidation()
    .clockSkewTolerance(60)
    .decode();
```

### Secure by Default

The decoder requires explicit verification configuration. You must call one of:
- `verifyWithEd25519(publicKey)` - Verify with Ed25519
- `verifyWithEcdsaP256(publicKey)` - Verify with ECDSA P-256
- `allowUnverified()` - Explicitly skip verification (testing only)

This prevents accidentally processing unverified credentials.

## Building from Source

### Prerequisites

- Rust 1.70+
- wasm-pack (`cargo install wasm-pack`)

### Build

```bash
# Build for bundler (recommended for npm packages)
wasm-pack build --target bundler --release

# Build for web (direct browser usage)
wasm-pack build --target web --release

# Build for Node.js
wasm-pack build --target nodejs --release
```

### Output

The build produces:
- `pkg/claim169_wasm.js` - JavaScript glue code
- `pkg/claim169_wasm_bg.wasm` - WebAssembly binary
- `pkg/claim169_wasm.d.ts` - TypeScript definitions

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

### Webpack 5

```javascript
// webpack.config.js
module.exports = {
  experiments: {
    asyncWebAssembly: true,
  },
};
```

### Rollup

```javascript
// rollup.config.js
import { wasm } from '@rollup/plugin-wasm';

export default {
  plugins: [wasm()],
};
```

## Security Considerations

The WASM bindings include full signature verification and decryption support:

- **Ed25519** and **ECDSA P-256** signature verification
- **AES-128-GCM** and **AES-256-GCM** decryption
- Secure-by-default: verification is required unless explicitly disabled

Best practices:
- Always verify credentials using `verifyWithEd25519()` or `verifyWithEcdsaP256()`
- Only use `allowUnverified()` for testing or when verification is handled elsewhere
- Enable timestamp validation in production to reject expired credentials
- Set appropriate `maxDecompressedBytes` limits to prevent denial-of-service

## Testing

```bash
# Run Rust tests
cargo test

# Build and test in Node.js
wasm-pack build --target nodejs
node -e "const wasm = require('./pkg'); console.log(wasm.version());"
```

## License

MIT License - See [LICENSE](../../LICENSE) for details.
