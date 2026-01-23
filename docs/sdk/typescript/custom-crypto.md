# Custom Crypto Providers

The TypeScript SDK supports custom cryptographic providers for integrating with Hardware Security Modules (HSMs), smart cards, TPMs, and other external crypto providers.

## Overview

Instead of providing raw keys, you can provide callback functions that the SDK calls during cryptographic operations. This keeps private keys within secure boundaries.

!!! important "Callbacks are synchronous"
    All custom crypto callbacks **must run synchronously** and return their result directly.
    Do not use `async` functions and do not return Promises — WebAssembly cannot `await` during encode/decode.

## Callback Types

### SignerCallback

Called during credential signing:

```typescript
type SignerCallback = (
  algorithm: string,      // "EdDSA" or "ES256"
  keyId: Uint8Array | null, // Key identifier from COSE header
  data: Uint8Array        // Data to sign (COSE Sig_structure)
) => Uint8Array;          // Returns signature bytes
```

### VerifierCallback

Called during signature verification:

```typescript
type VerifierCallback = (
  algorithm: string,      // "EdDSA" or "ES256"
  keyId: Uint8Array | null, // Key identifier from COSE header
  data: Uint8Array,       // Signed data (COSE Sig_structure)
  signature: Uint8Array   // Signature to verify
) => void;                // Throws on failure, returns nothing on success
```

### EncryptorCallback

Called during credential encryption:

```typescript
type EncryptorCallback = (
  algorithm: string,      // "A256GCM" or "A128GCM"
  keyId: Uint8Array | null, // Key identifier
  nonce: Uint8Array,      // 12-byte nonce/IV
  aad: Uint8Array,        // Additional authenticated data
  plaintext: Uint8Array   // Data to encrypt
) => Uint8Array;          // Returns ciphertext + auth tag
```

### DecryptorCallback

Called during credential decryption:

```typescript
type DecryptorCallback = (
  algorithm: string,      // "A256GCM" or "A128GCM"
  keyId: Uint8Array | null, // Key identifier
  nonce: Uint8Array,      // 12-byte nonce/IV
  aad: Uint8Array,        // Additional authenticated data
  ciphertext: Uint8Array  // Ciphertext + auth tag
) => Uint8Array;          // Returns decrypted plaintext
```

## Custom Signer

### Basic Example

```typescript
import { Encoder, type SignerCallback, type Claim169Input, type CwtMetaInput } from 'claim169';

const mySigner: SignerCallback = (algorithm, keyId, data) => {
  console.log('Signing with algorithm:', algorithm); // "EdDSA" or "ES256"
  console.log('Key ID:', keyId); // null or key identifier bytes

  // Call your crypto provider
  const signature = myHsm.sign({
    algorithm,
    keyId,
    data,
  });

  return signature; // Must be 64 bytes for EdDSA or ES256
};

const claim: Claim169Input = { id: "HSM-001", fullName: "HSM Signed User" };
const meta: CwtMetaInput = { issuer: "https://hsm.example" };

const qrData = new Encoder(claim, meta)
  .signWith(mySigner, "EdDSA")  // Specify algorithm
  .encode();
```

### Cloud KMS (recommended architecture)

Most cloud KMS SDKs are **async** and cannot be called directly from these callbacks.

For cloud KMS-backed signing/encryption, use one of these patterns:

1. **Do encoding/decoding server-side** (Rust or Python) where you can integrate with KMS/HSM and return the final QR payload to the client.
2. **Use a local signing agent** that exposes a synchronous API (for example, a native HSM library or a local daemon with a sync bridge), then call that from the callback.

## Custom Verifier

### Basic Example

```typescript
import { Decoder, type VerifierCallback } from 'claim169';

const myVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
  console.log('Verifying with algorithm:', algorithm);

  const result = myHsm.verify({
    algorithm,
    keyId,
    data,
    signature,
  });

  if (!result.valid) {
    throw new Error('Signature verification failed');
  }
  // Return nothing on success
};

const result = new Decoder(qrText)
  .verifyWith(myVerifier)
  .decode();

console.log('Verified:', result.verificationStatus); // "verified"
```

!!! note "Verification vs. trust"
    Custom verification can validate signatures, but you still need an application-level trust model (issuer identification, key distribution/rotation, and policy decisions).

## Custom Encryptor

### Basic Example

```typescript
import { Encoder, type EncryptorCallback } from 'claim169';

const myEncryptor: EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => {
  console.log('Encrypting with algorithm:', algorithm); // "A256GCM" or "A128GCM"

  const ciphertext = myHsm.encrypt({
    algorithm,
    keyId,
    nonce,
    aad,
    plaintext,
  });

  return ciphertext; // Must include 16-byte auth tag at end
};

const qrData = new Encoder(claim, meta)
  .signWithEd25519(signingKey)
  .encryptWith(myEncryptor, "A256GCM")
  .encode();
```

## Custom Decryptor

### Basic Example

```typescript
import { Decoder, type DecryptorCallback } from 'claim169';

const myDecryptor: DecryptorCallback = (algorithm, keyId, nonce, aad, ciphertext) => {
  console.log('Decrypting with algorithm:', algorithm);

  const plaintext = myHsm.decrypt({
    algorithm,
    keyId,
    nonce,
    aad,
    ciphertext, // Includes auth tag
  });

  return plaintext;
};

const result = new Decoder(qrText)
  .decryptWith(myDecryptor)
  .verifyWithEd25519(publicKey)
  .decode();
```

## Combined Custom Signer and Encryptor

```typescript
import { Encoder, type SignerCallback, type EncryptorCallback } from 'claim169';

// Both operations use the same HSM
const hsmSigner: SignerCallback = (algorithm, keyId, data) => {
  return myHsm.sign({ operation: 'sign', algorithm, keyId, data });
};

const hsmEncryptor: EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => {
  return myHsm.encrypt({ operation: 'encrypt', algorithm, keyId, nonce, aad, plaintext });
};

const qrData = new Encoder(claim, meta)
  .signWith(hsmSigner, "EdDSA")
  .encryptWith(hsmEncryptor, "A256GCM")
  .encode();
```

## Combined Custom Verifier and Decryptor

```typescript
import { Decoder, type VerifierCallback, type DecryptorCallback } from 'claim169';

const hsmVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
  const valid = myHsm.verify({ algorithm, keyId, data, signature });
  if (!valid) throw new Error('Verification failed');
};

const hsmDecryptor: DecryptorCallback = (algorithm, keyId, nonce, aad, ciphertext) => {
  return myHsm.decrypt({ algorithm, keyId, nonce, aad, ciphertext });
};

const result = new Decoder(qrText)
  .decryptWith(hsmDecryptor)
  .verifyWith(hsmVerifier)
  .decode();
```

## Roundtrip with Custom Providers

```typescript
import { Encoder, Decoder, type SignerCallback, type VerifierCallback } from 'claim169';

// Simulated HSM storage
const hsmKeys = new Map<string, Uint8Array>();

// Custom signer using "HSM"
const mySigner: SignerCallback = (algorithm, keyId, data) => {
  // Deterministic signature for demo (real HSM would use proper crypto)
  const sig = new Uint8Array(64);
  for (let i = 0; i < 64; i++) {
    sig[i] = (data[i % data.length] + (keyId?.[i % keyId.length] ?? 0)) % 256;
  }
  return sig;
};

// Custom verifier that matches our signer
const myVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
  // Recompute expected signature
  const expected = new Uint8Array(64);
  for (let i = 0; i < 64; i++) {
    expected[i] = (data[i % data.length] + (keyId?.[i % keyId.length] ?? 0)) % 256;
  }

  // Compare
  for (let i = 0; i < 64; i++) {
    if (signature[i] !== expected[i]) {
      throw new Error('Signature mismatch');
    }
  }
};

// Encode
const claim = { id: "ROUNDTRIP-001", fullName: "Roundtrip User" };
const meta = { issuer: "https://roundtrip.example" };

const qrData = new Encoder(claim, meta)
  .signWith(mySigner, "EdDSA")
  .encode();

// Decode
const result = new Decoder(qrData)
  .verifyWith(myVerifier)
  .decode();

console.log('ID:', result.claim169.id); // "ROUNDTRIP-001"
console.log('Verified:', result.verificationStatus); // "verified"
```

## Error Handling

Errors thrown from callbacks are propagated:

```typescript
const failingSigner: SignerCallback = (algorithm, keyId, data) => {
  throw new Error('HSM connection timeout');
};

try {
  const qrData = new Encoder(claim, meta)
    .signWith(failingSigner, "EdDSA")
    .encode();
} catch (error) {
  console.error('Signing failed:', error.message);
  // "HSM connection timeout" or wrapped error
}
```

## Supported Algorithms

### Signing Algorithms

| Algorithm | Description | Signature Size |
|-----------|-------------|----------------|
| `EdDSA` | Ed25519 | 64 bytes |
| `ES256` | ECDSA P-256 with SHA-256 | 64 bytes (r \|\| s) |

### Encryption Algorithms

| Algorithm | Description | Key Size |
|-----------|-------------|----------|
| `A256GCM` | AES-256-GCM | 32 bytes |
| `A128GCM` | AES-128-GCM | 16 bytes |

## Best Practices

1. **Never Log Sensitive Data**: Don't log private keys or full signatures in production
2. **Keep Callbacks Synchronous**: Don’t use `async`/Promises inside callbacks; if your provider is async, move crypto to a backend or a sync bridge
3. **Handle Timeouts**: HSM and KMS operations can timeout; implement retry logic
4. **Audit Logging**: Log key usage for compliance (key ID, operation, timestamp)
5. **Key Rotation**: Support multiple keys for smooth rotation

## Next Steps

- [Encryption](encryption.md) - Standard encryption examples
- [API Reference](api.md) - Complete callback type definitions
- [Troubleshooting](troubleshooting.md) - Common custom crypto issues
