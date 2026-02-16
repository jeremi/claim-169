# Quick Start

Get started with MOSIP Claim 169 in 5 minutes.

## Install the SDK

```bash
npm install claim169
```

## Decode a QR Code

The most common operation is decoding a QR code to extract identity data:

!!! warning "Do not trim Base45"
    The Base45 alphabet includes a literal space character (`" "`). Preserve the scanned QR text exactly as-is (no `.trim()`, whitespace collapsing, or normalization), or you can corrupt valid credentials.

```typescript
import { Decoder } from 'claim169';

// Your QR code content (Base45-encoded string)
const qrText = "6BF5YZB2K2RJMB2T...";

// Your issuer's Ed25519 public key (32 bytes)
const publicKey = new Uint8Array([
  0x3d, 0x2a, 0x1b, /* ... 32 bytes total */
]);

// Decode with signature verification
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Access identity data
console.log('ID:', result.claim169.id);
console.log('Name:', result.claim169.fullName);
console.log('DOB:', result.claim169.dateOfBirth);

// Access metadata
console.log('Issuer:', result.cwtMeta.issuer);
console.log('Expires:', new Date(result.cwtMeta.expiresAt! * 1000));

// Check verification status
console.log('Verified:', result.verificationStatus); // "verified"
```

## Encode a Credential

Create a signed credential QR code:

```typescript
import { Encoder, Gender, type Claim169Input, type CwtMetaInput } from 'claim169';

// Identity data to encode
const claim169: Claim169Input = {
  id: "123456789",
  fullName: "Jane Smith",
  dateOfBirth: "1990-05-20",
  gender: Gender.Female,
  email: "jane@example.com",
};

// Token metadata
const cwtMeta: CwtMetaInput = {
  issuer: "https://issuer.example.com",
  issuedAt: Math.floor(Date.now() / 1000),
  expiresAt: Math.floor(Date.now() / 1000) + 86400 * 365, // 1 year
};

// Your Ed25519 private key (32 bytes)
const privateKey = new Uint8Array([
  0x4a, 0x1c, 0x7b, /* ... 32 bytes total */
]);

// Create signed QR data
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();

console.log('QR Data:', qrData);
// Use this string to generate a QR code image
```

## Error Handling

Always wrap operations in try-catch:

```typescript
import { Decoder, Claim169Error } from 'claim169';

try {
  const result = new Decoder(qrText)
    .verifyWithEd25519(publicKey)
    .decode();

  console.log('Success:', result.claim169.fullName);
} catch (error) {
  if (error instanceof Claim169Error) {
    console.error('Decode failed:', error.message);
    console.error('Error code:', error.code); // e.g., "BASE45_DECODE", "SIGNATURE_INVALID"
  } else {
    throw error;
  }
}
```

## Decode Without Verification (Testing Only)

For development and testing, you can skip signature verification:

```typescript
import { Decoder } from 'claim169';

// WARNING: Only use for testing - never in production!
const result = new Decoder(qrText)
  .allowUnverified()
  .decode();

console.log('Status:', result.verificationStatus); // "skipped"
```

## Using the Convenience Function

For simple use cases, use the `decode()` function:

```typescript
import { decode } from 'claim169';

// With verification
const result = decode(qrText, {
  verifyWithEd25519: publicKey,
});

// Without verification (testing only)
const result = decode(qrText, {
  allowUnverified: true,
});
```

## Working with Hex Keys

If your keys are in hex format, use the utility functions:

```typescript
import { Decoder, hexToBytes, bytesToHex } from 'claim169';

// Convert hex to bytes
const publicKey = hexToBytes("3d2a1b4c5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b");

const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Convert bytes back to hex
const keyHex = bytesToHex(publicKey);
```

## Next Steps

- [Decoding Guide](decoding.md) - Full decoding options and examples
- [Encoding Guide](encoding.md) - Create credentials with all field types
- [Encryption](encryption.md) - Work with encrypted credentials
- [Custom Crypto](custom-crypto.md) - Integrate with HSMs and cloud KMS
- [API Reference](api.md) - Complete API documentation
