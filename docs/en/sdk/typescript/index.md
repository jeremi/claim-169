# TypeScript SDK

[![npm](https://img.shields.io/npm/v/claim169.svg)](https://www.npmjs.com/package/claim169)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-blue.svg)](https://www.typescriptlang.org/)

A TypeScript/JavaScript library for encoding and decoding MOSIP Claim 169 QR codes. Built on Rust/WebAssembly for performance and security.

## Why TypeScript?

- **Type Safety**: Full TypeScript definitions with comprehensive interfaces
- **WebAssembly Performance**: Rust-based cryptography compiled to WASM for near-native speed
- **Browser and Node.js**: Works in browsers, Node.js, and serverless environments
- **Secure by Default**: Requires explicit signature verification or opt-out
- **Builder Pattern**: Fluent API for configuring encoding and decoding operations

## Platform Support

| Platform | Support |
|----------|---------|
| Node.js 18+ | Full support |
| Modern Browsers | Full support (Chrome, Firefox, Safari, Edge) |
| React/Vue/Angular | Full support with bundler configuration |
| Deno | Experimental |
| Cloudflare Workers | Full support (WASM compatible) |
| AWS Lambda | Full support (Node.js runtime) |

## Features

- **Decoding**: Parse MOSIP Claim 169 QR codes with signature verification
- **Encoding**: Create signed and optionally encrypted credentials
- **Signature Verification**: Ed25519 and ECDSA P-256 support
- **Encryption**: AES-128-GCM and AES-256-GCM for encrypted credentials
- **Custom Crypto Providers**: Integrate with HSMs, cloud KMS, and smart cards
- **Biometric Data**: Parse fingerprint, iris, face, palm, and voice biometrics
- **Security Features**: Decompression bomb protection, timestamp validation

## Getting Started

```typescript
import { Decoder, Encoder } from 'claim169';

// Decode with signature verification
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

console.log(result.claim169.fullName);
console.log(result.verificationStatus); // "verified"

// Encode a credential
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();
```

## Documentation

- [Installation](installation.md) - Install and configure the SDK
- [Quick Start](quick-start.md) - Get started in 5 minutes
- [Decoding](decoding.md) - Decode QR codes with verification
- [Encoding](encoding.md) - Create signed credentials
- [Encryption](encryption.md) - Work with encrypted credentials
- [Custom Crypto](custom-crypto.md) - Integrate HSMs and cloud KMS
- [WASM Configuration](wasm.md) - Configure bundlers for WebAssembly
- [API Reference](api.md) - Complete API documentation
- [Troubleshooting](troubleshooting.md) - Common issues and solutions

## Security Considerations

The SDK is designed to be secure by default:

1. **Verification Required**: You must explicitly call a verification method or `allowUnverified()` before decoding
2. **No Algorithm Defaults**: COSE algorithm headers are mandatory (prevents algorithm confusion attacks)
3. **Decompression Limits**: Default 64KB limit protects against decompression bombs
4. **Weak Key Rejection**: Invalid Ed25519 and ECDSA keys are rejected

## License

MIT License - See [LICENSE](https://github.com/jeremi/claim-169/blob/main/LICENSE) for details.
