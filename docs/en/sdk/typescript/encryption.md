# Encryption

MOSIP Claim 169 supports AES-GCM encryption for protecting credential payloads. This guide covers encrypting and decrypting credentials.

## Overview

Encryption in Claim 169 works at the COSE layer:
1. Identity data is encoded into CBOR and wrapped in a CWT
2. The CWT is signed with COSE_Sign1
3. The signed payload is encrypted with COSE_Encrypt0 (AES-GCM)
4. The result is compressed and Base45-encoded

When decoding, the process is reversed: decrypt first, then verify the signature.

## AES-256-GCM

AES-256-GCM uses a 32-byte (256-bit) key with a 12-byte nonce and produces a 16-byte authentication tag.

### Encoding with AES-256-GCM

```typescript
import { Encoder, type Claim169Input, type CwtMetaInput } from 'claim169';

const claim169: Claim169Input = {
  id: "SECURE-001",
  fullName: "Secure User",
  dateOfBirth: "1990-01-01",
};

const cwtMeta: CwtMetaInput = {
  issuer: "https://secure.example.com",
  expiresAt: Math.floor(Date.now() / 1000) + 86400 * 365,
};

// Keys
const signingKey = new Uint8Array(32);  // Ed25519 private key
const encryptionKey = new Uint8Array(32); // AES-256 key

// Sign then encrypt
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(signingKey)
  .encryptWithAes256(encryptionKey)
  .encode();
```

### Decoding with AES-256-GCM

```typescript
import { Decoder } from 'claim169';

const encryptionKey = new Uint8Array(32); // Same AES-256 key
const verificationKey = new Uint8Array(32); // Ed25519 public key

// Decrypt then verify
const result = new Decoder(qrText)
  .decryptWithAes256(encryptionKey)
  .verifyWithEd25519(verificationKey)
  .decode();

console.log('Decrypted:', result.claim169.fullName);
console.log('Verified:', result.verificationStatus);
```

## AES-128-GCM

AES-128-GCM uses a 16-byte (128-bit) key. It's faster than AES-256 but provides less security margin.

### Encoding with AES-128-GCM

```typescript
const signingKey = new Uint8Array(32);  // Ed25519 private key
const encryptionKey = new Uint8Array(16); // AES-128 key (16 bytes)

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(signingKey)
  .encryptWithAes128(encryptionKey)
  .encode();
```

### Decoding with AES-128-GCM

```typescript
const encryptionKey = new Uint8Array(16); // AES-128 key
const verificationKey = new Uint8Array(32); // Ed25519 public key

const result = new Decoder(qrText)
  .decryptWithAes128(encryptionKey)
  .verifyWithEd25519(verificationKey)
  .decode();
```

## Generating Keys

### Generating an AES Key

```typescript
// Generate a random AES-256 key
const aes256Key = crypto.getRandomValues(new Uint8Array(32));

// Generate a random AES-128 key
const aes128Key = crypto.getRandomValues(new Uint8Array(16));
```

### Generating a Nonce

The SDK handles nonce generation internally, but you can also generate nonces for custom crypto providers:

```typescript
import { generateNonce } from 'claim169';

const nonce = generateNonce(); // Returns 12-byte Uint8Array
console.log('Nonce length:', nonce.length); // 12
```

## Using the Convenience Function

```typescript
import { decode } from 'claim169';

// Decrypt and verify in one call
const result = decode(qrText, {
  decryptWithAes256: encryptionKey,
  verifyWithEd25519: verificationKey,
});

// Or with AES-128
const result = decode(qrText, {
  decryptWithAes128: encryptionKey,
  verifyWithEd25519: verificationKey,
});
```

## Error Handling

### Decryption Errors

```typescript
import { Decoder, Claim169Error } from 'claim169';

try {
  const result = new Decoder(qrText)
    .decryptWithAes256(encryptionKey)
    .verifyWithEd25519(verificationKey)
    .decode();
} catch (error) {
  if (error instanceof Claim169Error) {
    if (error.message.includes('decryption') || error.message.includes('Decryption')) {
      console.error('Decryption failed - wrong key or corrupted data');
    } else if (error.message.includes('signature')) {
      console.error('Signature verification failed');
    } else {
      console.error('Decode failed:', error.message);
    }
  }
}
```

### Common Encryption Errors

| Error | Cause |
|-------|-------|
| `AES-256 key must be 32 bytes` | Invalid key size for AES-256 |
| `AES-128 key must be 16 bytes` | Invalid key size for AES-128 |
| `Decryption failed` | Wrong key or corrupted ciphertext |
| `Authentication failed` | Auth tag verification failed (tampered data) |

## Key Management Best Practices

### Don't Hardcode Keys

```typescript
// BAD - Never hardcode keys
const key = new Uint8Array([0x01, 0x02, ...]);

// GOOD - Load from environment or key management system
const key = await loadKeyFromKMS('encryption-key-id');
```

### Use Key Derivation

For deriving encryption keys from passwords:

```typescript
// Use Web Crypto API for key derivation
async function deriveKey(password: string, salt: Uint8Array): Promise<Uint8Array> {
  const encoder = new TextEncoder();
  const keyMaterial = await crypto.subtle.importKey(
    'raw',
    encoder.encode(password),
    'PBKDF2',
    false,
    ['deriveBits']
  );

  const derivedBits = await crypto.subtle.deriveBits(
    {
      name: 'PBKDF2',
      salt,
      iterations: 100000,
      hash: 'SHA-256',
    },
    keyMaterial,
    256 // 32 bytes
  );

  return new Uint8Array(derivedBits);
}
```

### Rotate Keys Regularly

Implement key rotation by supporting multiple encryption keys:

```typescript
async function decodeWithKeyRotation(
  qrText: string,
  keys: { id: string; key: Uint8Array }[],
  verifyKey: Uint8Array
): Promise<DecodeResult> {
  for (const { id, key } of keys) {
    try {
      return new Decoder(qrText)
        .decryptWithAes256(key)
        .verifyWithEd25519(verifyKey)
        .decode();
    } catch (error) {
      // Try next key
      continue;
    }
  }
  throw new Error('Decryption failed with all available keys');
}
```

## Custom Encryption Providers

For HSM or cloud KMS integration, see the [Custom Crypto](custom-crypto.md) guide:

```typescript
import { Encoder, Decoder, type EncryptorCallback, type DecryptorCallback } from 'claim169';

// Custom encryptor for HSM/KMS
const myEncryptor: EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => {
  return myKms.encrypt({ keyId, nonce, aad, plaintext, algorithm });
};

// Custom decryptor for HSM/KMS
const myDecryptor: DecryptorCallback = (algorithm, keyId, nonce, aad, ciphertext) => {
  return myKms.decrypt({ keyId, nonce, aad, ciphertext, algorithm });
};

// Use with encoder
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(signingKey)
  .encryptWith(myEncryptor, "A256GCM")
  .encode();

// Use with decoder
const result = new Decoder(qrText)
  .decryptWith(myDecryptor)
  .verifyWithEd25519(verifyKey)
  .decode();
```

## Algorithm Selection

| Algorithm | Key Size | Security Level | Performance |
|-----------|----------|----------------|-------------|
| AES-128-GCM | 16 bytes | 128-bit | Faster |
| AES-256-GCM | 32 bytes | 256-bit | Recommended |

For most applications, **AES-256-GCM** is recommended as it provides a larger security margin against future cryptanalytic advances.

## Next Steps

- [Custom Crypto](custom-crypto.md) - HSM and cloud KMS integration
- [API Reference](api.md) - Complete encryption API
- [Troubleshooting](troubleshooting.md) - Common encryption issues
