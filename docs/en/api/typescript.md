# TypeScript API Reference

## Installation

```bash
npm install claim169
```

## Quick Reference

```typescript
import {
  Decoder,
  Encoder,
  Claim169Input,
  CwtMetaInput,
  DecodeResult,
  hexToBytes,
  bytesToHex,
} from 'claim169';
```

## Decoder

### Constructor

```typescript
new Decoder(qrData: string): Decoder
```

Creates a new decoder from Base45-encoded QR data.

### Methods

#### verifyWithEd25519

```typescript
verifyWithEd25519(publicKey: Uint8Array): Decoder
```

Enable Ed25519 signature verification.

- `publicKey` - Ed25519 public key (32 bytes)

#### verifyWithEcdsaP256

```typescript
verifyWithEcdsaP256(publicKey: Uint8Array): Decoder
```

Enable ECDSA P-256 signature verification.

- `publicKey` - ECDSA P-256 public key (33 or 65 bytes)

#### allowUnverified

```typescript
allowUnverified(): Decoder
```

Skip signature verification (testing only).

#### decryptWithAes256

```typescript
decryptWithAes256(key: Uint8Array): Decoder
```

Enable AES-256-GCM decryption.

- `key` - AES-256 key (32 bytes)

#### decryptWithAes128

```typescript
decryptWithAes128(key: Uint8Array): Decoder
```

Enable AES-128-GCM decryption.

- `key` - AES-128 key (16 bytes)

#### decode

```typescript
decode(): DecodeResult
```

Execute the decoding pipeline and return the result.

**Throws:** Error if decoding fails

### Example

```typescript
import { Decoder, hexToBytes } from 'claim169';

const publicKey = hexToBytes("d75a980182b10ab7...");

const result = new Decoder(qrData)
  .verifyWithEd25519(publicKey)
  .decode();

console.log(result.claim169.fullName);
```

## Encoder

### Constructor

```typescript
new Encoder(claim: Claim169Input, meta: CwtMetaInput): Encoder
```

Creates a new encoder with identity data and metadata.

### Methods

#### signWithEd25519

```typescript
signWithEd25519(privateKey: Uint8Array): Encoder
```

Sign with Ed25519.

- `privateKey` - Ed25519 private key (32 bytes)

#### signWithEcdsaP256

```typescript
signWithEcdsaP256(privateKey: Uint8Array): Encoder
```

Sign with ECDSA P-256.

- `privateKey` - ECDSA P-256 private key (32 bytes)

#### allowUnsigned

```typescript
allowUnsigned(): Encoder
```

Skip signing (testing only).

#### encryptWithAes256

```typescript
encryptWithAes256(key: Uint8Array): Encoder
```

Encrypt with AES-256-GCM.

- `key` - AES-256 key (32 bytes)

#### encryptWithAes128

```typescript
encryptWithAes128(key: Uint8Array): Encoder
```

Encrypt with AES-128-GCM.

- `key` - AES-128 key (16 bytes)

#### encode

```typescript
encode(): string
```

Execute the encoding pipeline and return Base45 string.

**Throws:** Error if encoding fails

### Example

```typescript
import { Encoder, hexToBytes } from 'claim169';

const privateKey = hexToBytes("9d61b19deffd5a60...");

const qrData = new Encoder(claim, meta)
  .signWithEd25519(privateKey)
  .encode();
```

## Types

### Claim169Input

Input for encoding credentials.

```typescript
interface Claim169Input {
  id?: string;
  version?: string;
  language?: string;
  fullName?: string;
  firstName?: string;
  middleName?: string;
  lastName?: string;
  dateOfBirth?: string;
  gender?: number;          // 1=Male, 2=Female, 3=Other
  address?: string;
  email?: string;
  phone?: string;
  nationality?: string;
  maritalStatus?: number;   // 1=Unmarried, 2=Married, 3=Divorced
  guardian?: string;
  photo?: Uint8Array;
  photoFormat?: number;     // 1=JPEG, 2=JPEG2000, 3=AVIF
  legalStatus?: string;
  countryOfIssuance?: string;
  locationCode?: string;
  secondaryLanguage?: string;
  secondaryFullName?: string;
  bestQualityFingers?: number[];
}
```

### CwtMetaInput

CWT metadata for encoding.

```typescript
interface CwtMetaInput {
  issuer?: string;
  subject?: string;
  expiresAt?: number;    // Unix timestamp
  notBefore?: number;    // Unix timestamp
  issuedAt?: number;     // Unix timestamp
}
```

### DecodeResult

Result of successful decoding.

```typescript
interface DecodeResult {
  claim169: Claim169;
  cwtMeta: CwtMeta;
}
```

### Claim169

Decoded identity data.

```typescript
interface Claim169 {
  id?: string;
  version?: string;
  language?: string;
  fullName?: string;
  firstName?: string;
  middleName?: string;
  lastName?: string;
  dateOfBirth?: string;
  gender?: number;
  address?: string;
  email?: string;
  phone?: string;
  nationality?: string;
  maritalStatus?: number;
  guardian?: string;
  photo?: Uint8Array;
  photoFormat?: number;
  legalStatus?: string;
  countryOfIssuance?: string;
  locationCode?: string;
  secondaryLanguage?: string;
  secondaryFullName?: string;
  bestQualityFingers?: number[];
}
```

### CwtMeta

Decoded CWT metadata.

```typescript
interface CwtMeta {
  issuer?: string;
  subject?: string;
  expiresAt?: number;
  notBefore?: number;
  issuedAt?: number;
}
```

## Utility Functions

### hexToBytes

```typescript
function hexToBytes(hex: string): Uint8Array
```

Convert hex string to Uint8Array.

```typescript
const bytes = hexToBytes("d75a980182b10ab7...");
```

### bytesToHex

```typescript
function bytesToHex(bytes: Uint8Array): string
```

Convert Uint8Array to hex string.

```typescript
const hex = bytesToHex(publicKey);
```

## Error Handling

All methods that can fail throw standard JavaScript errors:

```typescript
try {
  const result = new Decoder(qrData)
    .verifyWithEd25519(publicKey)
    .decode();
} catch (error) {
  if (error instanceof Error) {
    console.error(`Decoding failed: ${error.message}`);
  }
}
```

## Complete Example

```typescript
import {
  Encoder,
  Decoder,
  Claim169Input,
  CwtMetaInput,
  hexToBytes,
} from 'claim169';

// Keys (for demo - use secure key management in production)
const privateKey = hexToBytes(
  "9d61b19deffd5a60ba844af492ec2cc4" +
  "4449c5697b326919703bac031cae7f60"
);
const publicKey = hexToBytes(
  "d75a980182b10ab7d54bfed3c964073a" +
  "0ee172f3daa62325af021a68f707511a"
);

// Create credential
const claim: Claim169Input = {
  id: "USER-12345",
  fullName: "Alice Smith",
  dateOfBirth: "19900101",
  gender: 2,
  email: "alice@example.com",
};

const meta: CwtMetaInput = {
  issuer: "https://identity.example.org",
  issuedAt: Math.floor(Date.now() / 1000),
  expiresAt: Math.floor(Date.now() / 1000) + 365 * 24 * 60 * 60,
};

// Encode
const qrData = new Encoder(claim, meta)
  .signWithEd25519(privateKey)
  .encode();

console.log(`Encoded: ${qrData.substring(0, 50)}...`);

// Decode and verify
const result = new Decoder(qrData)
  .verifyWithEd25519(publicKey)
  .decode();

console.log(`Name: ${result.claim169.fullName}`);
console.log(`Issuer: ${result.cwtMeta.issuer}`);
```

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
