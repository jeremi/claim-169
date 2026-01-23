# TypeScript API Reference

## Installation

```bash
npm install claim169
```

## Quick reference

```ts
import {
  // Errors
  Claim169Error,
  // Convenience API (unverified by default)
  decode,
  type DecodeOptions,
  // Builder API (recommended in production)
  Decoder,
  Encoder,
  // Types
  type Claim169Input,
  type CwtMetaInput,
  type DecodeResult,
  // Utilities
  hexToBytes,
  bytesToHex,
  generateNonce,
  version,
  isLoaded,
} from "claim169";
```

!!! warning "About `decode()`"
    `decode()` is a convenience wrapper that decodes **without signature verification** unless you pass a verification key in `DecodeOptions`. Use the `Decoder` builder with `.verifyWithEd25519()` / `.verifyWithEcdsaP256()` in production.

## `decode(qrText, options?)`

```ts
decode(qrText: string, options?: DecodeOptions): DecodeResult
```

Common options:

- `verifyWithEd25519` / `verifyWithEcdsaP256`
- `allowUnverified` (defaults to `true` if no key is provided)
- `decryptWithAes256` / `decryptWithAes128`
- `skipBiometrics`
- `validateTimestamps` (disabled by default in WASM)
- `clockSkewToleranceSeconds`
- `maxDecompressedBytes`

## Decoder (builder)

```ts
new Decoder(qrText: string)
```

### Verification

- `verifyWithEd25519(publicKey: Uint8Array)` (32 bytes)
- `verifyWithEcdsaP256(publicKey: Uint8Array)` (33 or 65 bytes SEC1)
- `allowUnverified()` (testing only)

### Decryption

- `decryptWithAes256(key: Uint8Array)` (32 bytes)
- `decryptWithAes128(key: Uint8Array)` (16 bytes)

### Options

- `skipBiometrics()`
- `withTimestampValidation()`
- `clockSkewTolerance(seconds: number)`
- `maxDecompressedBytes(bytes: number)`

### Execute

- `decode(): DecodeResult`

## Encoder (builder)

```ts
new Encoder(claim169: Claim169Input, cwtMeta: CwtMetaInput)
```

### Signing

- `signWithEd25519(privateKey: Uint8Array)` (32 bytes)
- `signWithEcdsaP256(privateKey: Uint8Array)` (32 bytes scalar)
- `allowUnsigned()` (testing only)

### Encryption

- `encryptWithAes256(key: Uint8Array)` (32 bytes)
- `encryptWithAes128(key: Uint8Array)` (16 bytes)

### Options

- `skipBiometrics()`

### Execute

- `encode(): string`

## Utilities

- `hexToBytes(hex: string): Uint8Array`
- `bytesToHex(bytes: Uint8Array): string`
- `generateNonce(): Uint8Array` (12 bytes)
- `version(): string`
- `isLoaded(): boolean`

## Browser Usage

The SDK works in modern browsers with WebAssembly support:

```html
<script type="module">
  import { Decoder } from 'claim169';

  const result = new Decoder(qrData)
    .allowUnverified()
    .decode();

  document.getElementById('name').textContent = result.claim169.fullName;
</script>
```

## Node.js Usage

Node.js 16+ is required for WebAssembly support:

```javascript
import { Decoder } from 'claim169';

const result = new Decoder(qrData)
  .allowUnverified()
  .decode();

console.log(result.claim169.fullName);
```
