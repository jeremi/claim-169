# Troubleshooting

Common issues and solutions for the TypeScript SDK.

## Installation Issues

### "Cannot find module 'claim169'"

**Cause**: Package not installed or module resolution issue.

**Solutions**:
1. Install the package: `npm install claim169`
2. Ensure `"type": "module"` is in package.json
3. Check TypeScript moduleResolution: use `"bundler"` or `"node16"`

### "WebAssembly module is not defined"

**Cause**: Bundler not configured for WASM.

**Solutions**:
- **Vite**: Install and add `vite-plugin-wasm` and `vite-plugin-top-level-await`
- **Webpack**: Add `experiments: { asyncWebAssembly: true }`
- **Next.js**: Configure webpack in `next.config.js`

See the [WASM Configuration](wasm.md) guide for details.

### TypeScript compilation errors

**Cause**: TypeScript configuration incompatibility.

**Solution**: Update tsconfig.json:

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "esModuleInterop": true
  }
}
```

## Decoding Errors

### "Must call verifyWith...() or allowUnverified()"

**Cause**: No verification method specified before `decode()`.

**Solution**: Add a verification method:

```typescript
// With verification (recommended)
new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Without verification (testing only)
new Decoder(qrText)
  .allowUnverified()
  .decode();
```

### "Ed25519 public key must be 32 bytes"

**Cause**: Public key is wrong size.

**Solution**: Ensure key is exactly 32 bytes:

```typescript
const publicKey = new Uint8Array(32);
console.log('Key length:', publicKey.length); // Must be 32
```

### "ECDSA P-256 public key must be 33 bytes (compressed) or 65 bytes (uncompressed)"

**Cause**: ECDSA key is wrong size or format.

**Solutions**:
- Use compressed format (33 bytes, starts with 0x02 or 0x03)
- Use uncompressed format (65 bytes, starts with 0x04)

### "AES-256 key must be 32 bytes" / "AES-128 key must be 16 bytes"

**Cause**: Encryption key is wrong size.

**Solution**: Check key size matches algorithm:
- AES-256: 32 bytes
- AES-128: 16 bytes

### "Base45Decode" error

**Cause**: Invalid Base45 encoding in QR data.

**Solutions**:
1. Ensure the QR code was scanned correctly
2. Check for truncated data
3. Verify the source produces valid Claim 169 data

### "Decompress" error

**Cause**: zlib decompression failed.

**Solutions**:
1. Data may be corrupted
2. Data may exceed decompression limit

Increase the limit if needed:
```typescript
new Decoder(qrText)
  .maxDecompressedBytes(131072) // 128KB
  .allowUnverified()
  .decode();
```

### "CoseParse" error

**Cause**: Invalid COSE structure.

**Solutions**:
1. Verify the data is a valid MOSIP Claim 169 credential
2. Check that encryption/decryption is applied correctly

### "Claim169NotFound" error

**Cause**: CWT does not contain claim 169.

**Solutions**:
1. Verify the credential uses MOSIP Claim 169 format
2. Check that the correct claim ID (169) is present

### "SignatureError" or verification failed

**Cause**: Signature verification failed.

**Solutions**:
1. Verify you're using the correct public key
2. Check key matches the signing algorithm (Ed25519 vs ECDSA)
3. Ensure data hasn't been tampered with

### "DecryptionError"

**Cause**: Decryption failed.

**Solutions**:
1. Verify the encryption key is correct
2. Check algorithm matches (AES-256 vs AES-128)
3. Ensure data wasn't corrupted

## Encoding Errors

### "Must call signWith...() or allowUnsigned()"

**Cause**: No signing method specified before `encode()`.

**Solution**: Add a signing method:

```typescript
// With signing (recommended)
new Encoder(claim, meta)
  .signWithEd25519(privateKey)
  .encode();

// Without signing (testing only)
new Encoder(claim, meta)
  .allowUnsigned()
  .encode();
```

### "Ed25519 private key must be 32 bytes"

**Cause**: Private key is wrong size.

**Solution**: Ensure key is exactly 32 bytes.

### "Invalid claim169" or "Invalid cwtMeta"

**Cause**: Input data doesn't match expected schema.

**Solution**: Use proper TypeScript types:

```typescript
import type { Claim169Input, CwtMetaInput } from 'claim169';

const claim: Claim169Input = {
  id: "123",
  fullName: "Test",
};

const meta: CwtMetaInput = {
  issuer: "https://example.com",
};
```

## Custom Crypto Issues

### Callback errors not propagating

**Cause**: Callback exceptions may be wrapped.

**Solution**: Check the error message for wrapped content:

```typescript
try {
  new Encoder(claim, meta)
    .signWith(failingSigner, "EdDSA")
    .encode();
} catch (error) {
  if (error instanceof Claim169Error) {
    // Original error message may be inside
    console.error('Full error:', error.message);
  }
}
```

### Signature verification fails with custom verifier

**Cause**: Verifier logic doesn't match signer.

**Solutions**:
1. Ensure verifier computes the same signature as signer
2. Log algorithm and data to debug:

```typescript
const debugVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
  console.log('Algorithm:', algorithm);
  console.log('Data length:', data.length);
  console.log('Signature length:', signature.length);
  // Your verification logic
};
```

### Async callbacks not working

**Cause**: Callbacks are synchronous; async functions return promises.

**Solution**: Callbacks must be synchronous. For async operations, wrap the entire encode/decode:

```typescript
async function encodeWithAsyncSigning() {
  // Pre-fetch key from KMS
  const signature = await fetchSignatureFromKMS(data);

  // Use synchronous callback with pre-fetched data
  const signer: SignerCallback = (alg, keyId, data) => {
    return signature; // Pre-computed
  };

  return new Encoder(claim, meta)
    .signWith(signer, "EdDSA")
    .encode();
}
```

## Browser Issues

### CORS errors

**Cause**: WASM fetch blocked by CORS.

**Solutions**:
1. Serve WASM from same origin
2. Add proper CORS headers to WASM server
3. Use a bundler that inlines WASM

### "SharedArrayBuffer is not defined"

**Cause**: Some WASM features require cross-origin isolation.

**Solution**: Add server headers:
```
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

### WASM not loading in production

**Cause**: WASM file not included in build or wrong MIME type.

**Solutions**:
1. Check build output includes `.wasm` file
2. Configure server MIME type: `application/wasm`
3. Check CSP allows WASM execution

## Performance Issues

### Slow initial load

**Cause**: WASM module loading overhead.

**Solutions**:
1. Lazy load the SDK:
   ```typescript
   const { Decoder } = await import('claim169');
   ```
2. Enable WASM streaming compilation
3. Cache the compiled module

### Slow decoding with biometrics

**Cause**: Large biometric data.

**Solution**: Skip biometrics if not needed:
```typescript
new Decoder(qrText)
  .skipBiometrics()
  .allowUnverified()
  .decode();
```

## Node.js Issues

### "Cannot use import statement outside a module"

**Cause**: CommonJS/ESM mismatch.

**Solutions**:
1. Add `"type": "module"` to package.json
2. Use `.mjs` extension
3. Use dynamic import: `const { Decoder } = await import('claim169');`

### Module not found in tests

**Cause**: Test runner not configured for ESM.

**Solution** for Vitest:
```typescript
// vitest.config.ts
import { defineConfig } from 'vitest/config';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [wasm(), topLevelAwait()],
});
```

## Debugging Tips

### Enable verbose logging

```typescript
import { Decoder, Claim169Error } from 'claim169';

try {
  const result = new Decoder(qrText)
    .verifyWithEd25519(publicKey)
    .decode();

  console.log('Decode result:', JSON.stringify(result, null, 2));
} catch (error) {
  if (error instanceof Claim169Error) {
    console.error('Claim169Error:', error.message);
    console.error('Error code:', error.code); // e.g., "BASE45_DECODE", "SIGNATURE_INVALID"
    console.error('Stack:', error.stack);
  } else {
    console.error('Unknown error:', error);
  }
}
```

### Check WASM loading

```typescript
import { isLoaded, version } from 'claim169';

console.log('WASM loaded:', isLoaded());
console.log('Version:', version());
```

### Validate key formats

```typescript
function validateKey(key: Uint8Array, expectedLength: number, name: string) {
  console.log(`${name} length: ${key.length} (expected ${expectedLength})`);
  console.log(`${name} first bytes: ${Array.from(key.slice(0, 4)).map(b => b.toString(16).padStart(2, '0')).join(' ')}`);

  if (key.length !== expectedLength) {
    throw new Error(`Invalid ${name} length`);
  }
}

validateKey(publicKey, 32, 'Ed25519 public key');
validateKey(aesKey, 32, 'AES-256 key');
```

## Getting Help

If you're still having issues:

1. Check the [GitHub Issues](https://github.com/jeremi/claim-169/issues) for known problems
2. Search existing issues before creating a new one
3. Include in your issue:
   - SDK version (`version()`)
   - Node.js/browser version
   - Bundler and version
   - Minimal reproduction code
   - Full error message and stack trace
