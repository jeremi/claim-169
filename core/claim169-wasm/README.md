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

// Simple decode
const result = new Decoder(qrText).decode();
console.log(result.claim169.fullName);

// With options
const result = new Decoder(qrText)
    .skipBiometrics()
    .maxDecompressedBytes(32768)
    .decode();
```

### Direct WASM Usage

```javascript
import init, { Decoder, decode } from 'claim169';

// Initialize WASM module
await init();

// Simple decode
const result = decode(qrText);
console.log(result.claim169.fullName);

// Builder pattern
const result = new Decoder(qrText)
    .skipBiometrics()
    .maxDecompressedBytes(32768)
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

// Configuration methods (chainable)
decoder.skipBiometrics();              // Skip biometric data parsing
decoder.withTimestampValidation();     // Enable timestamp validation
decoder.clockSkewTolerance(60);        // Set clock skew tolerance (seconds)
decoder.maxDecompressedBytes(32768);   // Set max decompressed size

// Execute decode
const result = decoder.decode();
```

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

### Functions

| Function | Description |
|----------|-------------|
| `decode(qrText)` | Decode a QR code without verification |
| `decodeWithOptions(qrText, skipBiometrics, maxDecompressedBytes)` | Decode with options |
| `generateNonce()` | Generate a cryptographically secure random nonce |
| `version()` | Get library version |
| `isLoaded()` | Check if WASM module is ready |

## Limitations

### No Signature Verification

The WASM bindings do not currently support signature verification. For verified decoding:

1. Use the [Python bindings](../claim169-python) for server-side verification
2. Use the [Rust core library](../claim169-core) directly
3. Implement client-side verification using WebCrypto after decoding

### No System Time Access

Timestamp validation is disabled by default because WebAssembly doesn't have reliable access to system time. Enable it explicitly if your environment supports it:

```javascript
const result = new Decoder(qrText)
    .withTimestampValidation()
    .decode();
```

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

Since signature verification is not available in WASM, decoded credentials should be treated as **untrusted** unless:

1. Verification is performed server-side before sending to the client
2. External verification is implemented using WebCrypto
3. The use case doesn't require cryptographic authenticity

For security-sensitive applications, consider:

- Server-side decoding and verification using Python or Rust
- Passing only verified, serialized data to the browser
- Implementing verification using the WebCrypto API

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
