# API Reference

Complete API documentation for the claim169 TypeScript SDK, auto-generated from source code.


---

# Class: Decoder

Defined in: [src/index.ts:447](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L447)

Builder-pattern decoder for Claim 169 QR codes.

Provides a fluent API for configuring decoding options and executing the decode.
Supports signature verification with Ed25519 and ECDSA P-256, as well as
AES-GCM decryption for encrypted credentials.

## Example

```typescript
// With verification (recommended for production)
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Without verification (testing only)
const result = new Decoder(qrText)
  .allowUnverified()
  .skipBiometrics()
  .decode();

// With decryption and verification
const result = new Decoder(qrText)
  .decryptWithAes256(aesKey)
  .verifyWithEd25519(publicKey)
  .decode();
```

## Implements

- `IDecoder`

## Constructors

### Constructor

```ts
new Decoder(qrText): Decoder;
```

Defined in: [src/index.ts:456](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L456)

Create a new Decoder instance.

#### Parameters

##### qrText

`string`

The QR code text content (Base45 encoded)

#### Returns

`Decoder`

## Methods

### allowUnverified()

```ts
allowUnverified(): Decoder;
```

Defined in: [src/index.ts:540](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L540)

Allow decoding without signature verification.
WARNING: Credentials decoded with verification skipped (`verificationStatus === "skipped"`)
cannot be trusted. Use for testing only.

#### Returns

`Decoder`

The decoder instance for chaining

#### Implementation of

`IDecoder`.`allowUnverified`

***

### clockSkewTolerance()

```ts
clockSkewTolerance(seconds): Decoder;
```

Defined in: [src/index.ts:618](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L618)

Set clock skew tolerance in seconds.
Allows credentials to be accepted when clocks are slightly out of sync.
Applies when timestamp validation is enabled.

#### Parameters

##### seconds

`number`

The tolerance in seconds

#### Returns

`Decoder`

The decoder instance for chaining

#### Implementation of

`IDecoder`.`clockSkewTolerance`

***

### decode()

```ts
decode(): DecodeResult;
```

Defined in: [src/index.ts:699](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L699)

Decode the QR code with the configured options.
Requires either a verifier (verifyWithEd25519/verifyWithEcdsaP256) or
explicit allowUnverified() to be called first.

#### Returns

`DecodeResult`

The decoded result

#### Throws

If decoding fails or no verification method specified

#### Implementation of

`IDecoder`.`decode`

***

### decryptWith()

```ts
decryptWith(decryptor): Decoder;
```

Defined in: [src/index.ts:680](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L680)

Decrypt with a custom decryptor callback.
Use for external crypto providers (HSM, cloud KMS, etc.)

#### Parameters

##### decryptor

`DecryptorCallback`

Function that decrypts ciphertext

#### Returns

`Decoder`

The decoder instance for chaining

#### Example

```typescript
const result = new Decoder(qrText)
  .decryptWith((algorithm, keyId, nonce, aad, ciphertext) => {
    // Call your crypto provider here
    return myKms.decrypt(keyId, ciphertext, { nonce, aad });
  })
  .decode();
```

#### Implementation of

`IDecoder`.`decryptWith`

***

### decryptWithAes128()

```ts
decryptWithAes128(key): Decoder;
```

Defined in: [src/index.ts:569](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L569)

Decrypt with AES-128-GCM.

#### Parameters

##### key

`Uint8Array`

16-byte AES-128 key

#### Returns

`Decoder`

The decoder instance for chaining

#### Throws

If the key is invalid

#### Implementation of

`IDecoder`.`decryptWithAes128`

***

### decryptWithAes256()

```ts
decryptWithAes256(key): Decoder;
```

Defined in: [src/index.ts:551](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L551)

Decrypt with AES-256-GCM.

#### Parameters

##### key

`Uint8Array`

32-byte AES-256 key

#### Returns

`Decoder`

The decoder instance for chaining

#### Throws

If the key is invalid

#### Implementation of

`IDecoder`.`decryptWithAes256`

***

### maxDecompressedBytes()

```ts
maxDecompressedBytes(bytes): Decoder;
```

Defined in: [src/index.ts:629](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L629)

Set maximum decompressed size in bytes.
Protects against decompression bomb attacks.

#### Parameters

##### bytes

`number`

The maximum size in bytes (default: 65536)

#### Returns

`Decoder`

The decoder instance for chaining

#### Implementation of

`IDecoder`.`maxDecompressedBytes`

***

### skipBiometrics()

```ts
skipBiometrics(): Decoder;
```

Defined in: [src/index.ts:586](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L586)

Skip biometric data during decoding.
Useful when only demographic data is needed for faster parsing.

#### Returns

`Decoder`

The decoder instance for chaining

#### Implementation of

`IDecoder`.`skipBiometrics`

***

### verifyWith()

```ts
verifyWith(verifier): Decoder;
```

Defined in: [src/index.ts:651](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L651)

Verify signature with a custom verifier callback.
Use for external crypto providers (HSM, cloud KMS, remote signing, etc.)

#### Parameters

##### verifier

`VerifierCallback`

Function that verifies signatures

#### Returns

`Decoder`

The decoder instance for chaining

#### Example

```typescript
const result = new Decoder(qrText)
  .verifyWith((algorithm, keyId, data, signature) => {
    // Call your crypto provider here
    myKms.verify(keyId, data, signature);
  })
  .decode();
```

#### Implementation of

`IDecoder`.`verifyWith`

***

### verifyWithEcdsaP256()

```ts
verifyWithEcdsaP256(publicKey): Decoder;
```

Defined in: [src/index.ts:484](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L484)

Verify signature with ECDSA P-256 public key.

#### Parameters

##### publicKey

`Uint8Array`

SEC1-encoded P-256 public key (33 or 65 bytes)

#### Returns

`Decoder`

The decoder instance for chaining

#### Throws

If the public key is invalid

#### Implementation of

`IDecoder`.`verifyWithEcdsaP256`

***

### verifyWithEcdsaP256Pem()

```ts
verifyWithEcdsaP256Pem(pem): Decoder;
```

Defined in: [src/index.ts:522](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L522)

Verify signature with ECDSA P-256 public key in PEM format.
Supports SPKI format with "BEGIN PUBLIC KEY" headers.

#### Parameters

##### pem

`string`

PEM-encoded P-256 public key

#### Returns

`Decoder`

The decoder instance for chaining

#### Throws

If the PEM is invalid

#### Implementation of

`IDecoder`.`verifyWithEcdsaP256Pem`

***

### verifyWithEd25519()

```ts
verifyWithEd25519(publicKey): Decoder;
```

Defined in: [src/index.ts:466](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L466)

Verify signature with Ed25519 public key.

#### Parameters

##### publicKey

`Uint8Array`

32-byte Ed25519 public key

#### Returns

`Decoder`

The decoder instance for chaining

#### Throws

If the public key is invalid

#### Implementation of

`IDecoder`.`verifyWithEd25519`

***

### verifyWithEd25519Pem()

```ts
verifyWithEd25519Pem(pem): Decoder;
```

Defined in: [src/index.ts:503](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L503)

Verify signature with Ed25519 public key in PEM format.
Supports SPKI format with "BEGIN PUBLIC KEY" headers.

#### Parameters

##### pem

`string`

PEM-encoded Ed25519 public key

#### Returns

`Decoder`

The decoder instance for chaining

#### Throws

If the PEM is invalid

#### Implementation of

`IDecoder`.`verifyWithEd25519Pem`

***

### withoutTimestampValidation()

```ts
withoutTimestampValidation(): Decoder;
```

Defined in: [src/index.ts:606](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L606)

Disable timestamp validation.

#### Returns

`Decoder`

The decoder instance for chaining

#### Implementation of

`IDecoder`.`withoutTimestampValidation`

***

### withTimestampValidation()

```ts
withTimestampValidation(): Decoder;
```

Defined in: [src/index.ts:597](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L597)

Enable timestamp validation.
When enabled, expired or not-yet-valid credentials will throw an error.
Implemented in the host (JavaScript) to avoid WASM runtime time limitations.

#### Returns

`Decoder`

The decoder instance for chaining

#### Implementation of

`IDecoder`.`withTimestampValidation`

---

# Class: Encoder

Defined in: [src/index.ts:817](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L817)

Builder-pattern encoder for Claim 169 QR codes.

Provides a fluent API for configuring encoding options and generating QR data.

## Example

```typescript
// Signed credential (recommended)
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();

// Signed and encrypted
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encryptWithAes256(aesKey)
  .encode();

// Unsigned (testing only)
const qrData = new Encoder(claim169, cwtMeta)
  .allowUnsigned()
  .encode();
```

## Implements

- `IEncoder`

## Constructors

### Constructor

```ts
new Encoder(claim169, cwtMeta): Encoder;
```

Defined in: [src/index.ts:825](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L825)

Create a new Encoder instance.

#### Parameters

##### claim169

`Claim169Input`

The identity claim data to encode

##### cwtMeta

`CwtMetaInput`

CWT metadata including issuer, expiration, etc.

#### Returns

`Encoder`

## Methods

### allowUnsigned()

```ts
allowUnsigned(): Encoder;
```

Defined in: [src/index.ts:909](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L909)

Allow encoding without a signature.
WARNING: Unsigned credentials cannot be verified. Use for testing only.

#### Returns

`Encoder`

The encoder instance for chaining

#### Implementation of

`IEncoder`.`allowUnsigned`

***

### encode()

```ts
encode(): string;
```

Defined in: [src/index.ts:1000](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L1000)

Encode the credential to a Base45 QR string.

#### Returns

`string`

Base45-encoded string suitable for QR code generation

#### Throws

If encoding fails

#### Implementation of

`IEncoder`.`encode`

***

### encryptWith()

```ts
encryptWith(encryptor, algorithm): Encoder;
```

Defined in: [src/index.ts:980](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L980)

Encrypt with a custom encryptor callback.
Use for external crypto providers (HSM, cloud KMS, etc.)

#### Parameters

##### encryptor

`EncryptorCallback`

Function that encrypts data

##### algorithm

Encryption algorithm: "A256GCM" or "A128GCM"

`"A256GCM"` | `"A128GCM"`

#### Returns

`Encoder`

The encoder instance for chaining

#### Example

```typescript
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(signKey)
  .encryptWith((algorithm, keyId, nonce, aad, plaintext) => {
    return myKms.encrypt({ keyId, nonce, aad, plaintext });
  }, "A256GCM")
  .encode();
```

#### Implementation of

`IEncoder`.`encryptWith`

***

### encryptWithAes128()

```ts
encryptWithAes128(key): Encoder;
```

Defined in: [src/index.ts:892](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L892)

Encrypt with AES-128-GCM.

#### Parameters

##### key

`Uint8Array`

16-byte AES-128 key

#### Returns

`Encoder`

The encoder instance for chaining

#### Implementation of

`IEncoder`.`encryptWithAes128`

***

### encryptWithAes256()

```ts
encryptWithAes256(key): Encoder;
```

Defined in: [src/index.ts:875](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L875)

Encrypt with AES-256-GCM.

#### Parameters

##### key

`Uint8Array`

32-byte AES-256 key

#### Returns

`Encoder`

The encoder instance for chaining

#### Implementation of

`IEncoder`.`encryptWithAes256`

***

### signWith()

```ts
signWith(
   signer, 
   algorithm, 
   keyId?): Encoder;
```

Defined in: [src/index.ts:941](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L941)

Sign with a custom signer callback.
Use for external crypto providers (HSM, cloud KMS, remote signing, etc.)

#### Parameters

##### signer

`SignerCallback`

Function that signs data

##### algorithm

Signature algorithm: "EdDSA" or "ES256"

`"EdDSA"` | `"ES256"`

##### keyId?

Optional key identifier passed to the signer callback

`Uint8Array`\<`ArrayBufferLike`\> | `null`

#### Returns

`Encoder`

The encoder instance for chaining

#### Example

```typescript
const qrData = new Encoder(claim169, cwtMeta)
  .signWith((algorithm, keyId, data) => {
    return myKms.sign({ keyId, data });
  }, "EdDSA")
  .encode();
```

#### Implementation of

`IEncoder`.`signWith`

***

### signWithEcdsaP256()

```ts
signWithEcdsaP256(privateKey): Encoder;
```

Defined in: [src/index.ts:858](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L858)

Sign with ECDSA P-256 private key.

#### Parameters

##### privateKey

`Uint8Array`

32-byte ECDSA P-256 private key (scalar)

#### Returns

`Encoder`

The encoder instance for chaining

#### Implementation of

`IEncoder`.`signWithEcdsaP256`

***

### signWithEd25519()

```ts
signWithEd25519(privateKey): Encoder;
```

Defined in: [src/index.ts:841](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L841)

Sign with Ed25519 private key.

#### Parameters

##### privateKey

`Uint8Array`

32-byte Ed25519 private key

#### Returns

`Encoder`

The encoder instance for chaining

#### Implementation of

`IEncoder`.`signWithEd25519`

***

### skipBiometrics()

```ts
skipBiometrics(): Encoder;
```

Defined in: [src/index.ts:918](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L918)

Skip biometric fields during encoding.

#### Returns

`Encoder`

The encoder instance for chaining

#### Implementation of

`IEncoder`.`skipBiometrics`

---

# Function: bytesToHex()

```ts
function bytesToHex(bytes): string;
```

Defined in: [src/index.ts:173](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L173)

Convert bytes to a lowercase hex string.

## Parameters

### bytes

`Uint8Array`

## Returns

`string`

---

# Function: decode()

```ts
function decode(qrText, options): DecodeResult;
```

Defined in: [src/index.ts:749](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L749)

Decode a Claim 169 QR string.

This is a convenience wrapper around the `Decoder` builder.
Security:
- If you do not pass a verification key, you must set `allowUnverified: true` (testing only).

## Parameters

### qrText

`string`

### options

`DecodeOptions` = `{}`

## Returns

`DecodeResult`

---

# Function: generateNonce()

```ts
function generateNonce(): Uint8Array;
```

Defined in: [src/index.ts:1016](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L1016)

Generate a random 12-byte nonce for AES-GCM encryption.

## Returns

`Uint8Array`

A 12-byte Uint8Array suitable for use as a nonce

---

# Function: hexToBytes()

```ts
function hexToBytes(hex): Uint8Array;
```

Defined in: [src/index.ts:147](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L147)

Convert a hex string to bytes.

Accepts optional `0x` prefix and ignores whitespace.

## Parameters

### hex

`string`

## Returns

`Uint8Array`

## Throws

If the input is not valid hex

---

# Function: isLoaded()

```ts
function isLoaded(): boolean;
```

Defined in: [src/index.ts:136](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L136)

Check if the WASM module is loaded correctly

## Returns

`boolean`

---

# Function: version()

```ts
function version(): string;
```

Defined in: [src/index.ts:129](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L129)

Get the library version

## Returns

`string`

---

# Interface: DecodeOptions

Defined in: [src/index.ts:730](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L730)

Options for the `decode()` convenience function.

Notes:
- If you don't provide a verification key, you must explicitly set `allowUnverified: true` (testing only).
- Timestamp validation is enabled by default in JS (host-side). Set `validateTimestamps: false` to disable.

## Properties

### allowUnverified?

```ts
optional allowUnverified: boolean;
```

Defined in: [src/index.ts:735](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L735)

***

### clockSkewToleranceSeconds?

```ts
optional clockSkewToleranceSeconds: number;
```

Defined in: [src/index.ts:738](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L738)

***

### decryptWithAes128?

```ts
optional decryptWithAes128: Uint8Array<ArrayBufferLike>;
```

Defined in: [src/index.ts:734](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L734)

***

### decryptWithAes256?

```ts
optional decryptWithAes256: Uint8Array<ArrayBufferLike>;
```

Defined in: [src/index.ts:733](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L733)

***

### maxDecompressedBytes?

```ts
optional maxDecompressedBytes: number;
```

Defined in: [src/index.ts:739](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L739)

***

### skipBiometrics?

```ts
optional skipBiometrics: boolean;
```

Defined in: [src/index.ts:736](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L736)

***

### validateTimestamps?

```ts
optional validateTimestamps: boolean;
```

Defined in: [src/index.ts:737](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L737)

***

### verifyWithEcdsaP256?

```ts
optional verifyWithEcdsaP256: Uint8Array<ArrayBufferLike>;
```

Defined in: [src/index.ts:732](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L732)

***

### verifyWithEd25519?

```ts
optional verifyWithEd25519: Uint8Array<ArrayBufferLike>;
```

Defined in: [src/index.ts:731](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/index.ts#L731)

---

# claim169

MOSIP Claim 169 QR Code library for TypeScript/JavaScript.

This library provides classes to encode and decode MOSIP Claim 169 identity
credentials from QR codes. It uses WebAssembly for high-performance binary
parsing and cryptographic operations.

## Installation

```bash
npm install claim169
```

## Decoding with Verification (Recommended)

```typescript
import { Decoder } from 'claim169';

// Decode with Ed25519 signature verification
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Access identity data
console.log(result.claim169.fullName);
console.log(result.claim169.dateOfBirth);

// Access metadata
console.log(result.cwtMeta.issuer);
console.log(result.cwtMeta.expiresAt);

// Check verification status
console.log(result.verificationStatus); // "verified"
```

## Decoding without Verification (Testing Only)

```typescript
const result = new Decoder(qrText)
  .allowUnverified()  // Explicit opt-out required
  .decode();
```

## Decoding Encrypted Credentials

```typescript
const result = new Decoder(qrText)
  .decryptWithAes256(aesKey)
  .verifyWithEd25519(publicKey)
  .decode();
```

## Encoding Credentials

```typescript
import { Encoder } from 'claim169';

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();
```

## Error Handling

```typescript
import { Decoder, Claim169Error } from 'claim169';

try {
  const result = new Decoder(qrText)
    .verifyWithEd25519(publicKey)
    .decode();
} catch (error) {
  if (error instanceof Claim169Error) {
    console.error('Decoding failed:', error.message);
  }
}
```

## Notes

- **Timestamp validation**: Enabled by default in JS (host-side). Disable with
  `.withoutTimestampValidation()` if you intentionally want to skip time checks.

## Classes

- Decoder
- Encoder

## Interfaces

- DecodeOptions

## Functions

- bytesToHex
- decode
- generateNonce
- hexToBytes
- isLoaded
- version

## References

### Algorithm

Re-exports Algorithm

***

### AlgorithmName

Re-exports AlgorithmName

***

### Biometric

Re-exports Biometric

***

### CertificateHash

Re-exports CertificateHash

***

### Claim169

Re-exports Claim169

***

### Claim169Error

Re-exports Claim169Error

***

### Claim169Input

Re-exports Claim169Input

***

### CwtMeta

Re-exports CwtMeta

***

### CwtMetaInput

Re-exports CwtMetaInput

***

### DecodeResult

Re-exports DecodeResult

***

### DecryptorCallback

Re-exports DecryptorCallback

***

### EncryptorCallback

Re-exports EncryptorCallback

***

### IDecoder

Re-exports IDecoder

***

### IEncoder

Re-exports IEncoder

***

### SignerCallback

Re-exports SignerCallback

***

### VerificationStatus

Re-exports VerificationStatus

***

### VerifierCallback

Re-exports VerifierCallback

***

### X509Headers

Re-exports X509Headers

---

# Class: Claim169Error

Defined in: [src/types.ts:433](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L433)

Error thrown when decoding fails.

## Example

```typescript
try {
  decode(qrText, { allowUnverified: true }); // testing only
} catch (error) {
  if (error instanceof Claim169Error) {
    console.error('Decoding failed:', error.message);
  }
}
```

## Extends

- `Error`

## Constructors

### Constructor

```ts
new Claim169Error(message): Claim169Error;
```

Defined in: [src/types.ts:434](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L434)

#### Parameters

##### message

`string`

#### Returns

`Claim169Error`

#### Overrides

```ts
Error.constructor
```

## Properties

### message

```ts
message: string;
```

Defined in: node\_modules/typescript/lib/lib.es5.d.ts:1077

#### Inherited from

```ts
Error.message
```

***

### name

```ts
name: string;
```

Defined in: node\_modules/typescript/lib/lib.es5.d.ts:1076

#### Inherited from

```ts
Error.name
```

***

### stack?

```ts
optional stack: string;
```

Defined in: node\_modules/typescript/lib/lib.es5.d.ts:1078

#### Inherited from

```ts
Error.stack
```

---

# Interface: Biometric

Defined in: [src/types.ts:26](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L26)

A single biometric data entry from a Claim 169 credential.

Biometric data can be fingerprints, iris scans, face images, or voice samples.
Each entry contains the raw data and metadata about its format.

## Example

```typescript
// Access face biometric data
if (claim.face && claim.face.length > 0) {
  const faceData = claim.face[0];
  console.log(`Format: ${faceData.format}`);
  console.log(`Data size: ${faceData.data.byteLength} bytes`);
}
```

## Properties

### data

```ts
data: Uint8Array;
```

Defined in: [src/types.ts:28](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L28)

Raw biometric data bytes (image, template, or audio)

***

### format

```ts
format: number;
```

Defined in: [src/types.ts:36](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L36)

Biometric format code:
- 0: Image
- 1: Template
- 2: Sound
- 3: BioHash

***

### issuer?

```ts
optional issuer: string;
```

Defined in: [src/types.ts:45](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L45)

Biometric data issuer/provider identifier

***

### subFormat?

```ts
optional subFormat: number;
```

Defined in: [src/types.ts:43](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L43)

Sub-format code (depends on format type):
- For Image: 0=PNG, 1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP, 5=TIFF, 6=WSQ
- For Template: 0=ANSI378, 1=ISO19794-2, 2=NIST
- For Sound: 0=WAV, 1=MP3

---

# Interface: CertificateHash

Defined in: [src/types.ts:87](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L87)

X.509 certificate hash (COSE_CertHash).

Contains a hash algorithm identifier and the hash value.
Used in the x5t (thumbprint) header parameter.

## Properties

### algorithm

```ts
algorithm: string;
```

Defined in: [src/types.ts:92](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L92)

Hash algorithm identifier.
Can be a numeric COSE algorithm ID (e.g., "-16" for SHA-256) or a named algorithm.

***

### hashValue

```ts
hashValue: Uint8Array;
```

Defined in: [src/types.ts:94](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L94)

Hash value bytes

---

# Interface: Claim169

Defined in: [src/types.ts:166](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L166)

Decoded Claim 169 identity data.

This interface contains all identity fields defined in the MOSIP Claim 169
specification. All fields are optional since credentials may contain only
a subset of the available fields.

Fields are organized into:
- **Demographics** (id, name, DOB, address, etc.)
- **Biometrics** (fingerprints, iris, face, voice)

## Example

```typescript
// Access demographic data
console.log(`Name: ${claim.fullName}`);
console.log(`DOB: ${claim.dateOfBirth}`);

// Check for biometrics
const hasFace = claim.face && claim.face.length > 0;
const hasFingerprints = claim.rightThumb || claim.leftThumb;
```

## Properties

### address?

```ts
optional address: string;
```

Defined in: [src/types.ts:186](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L186)

Address

***

### bestQualityFingers?

```ts
optional bestQualityFingers: Uint8Array<ArrayBufferLike>;
```

Defined in: [src/types.ts:202](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L202)

Best quality fingers indicator

***

### countryOfIssuance?

```ts
optional countryOfIssuance: string;
```

Defined in: [src/types.ts:212](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L212)

Country of issuance

***

### dateOfBirth?

```ts
optional dateOfBirth: string;
```

Defined in: [src/types.ts:182](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L182)

Date of birth (ISO 8601 format)

***

### email?

```ts
optional email: string;
```

Defined in: [src/types.ts:188](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L188)

Email address

***

### face?

```ts
optional face: Biometric[];
```

Defined in: [src/types.ts:239](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L239)

Face biometrics

***

### firstName?

```ts
optional firstName: string;
```

Defined in: [src/types.ts:176](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L176)

First name

***

### fullName?

```ts
optional fullName: string;
```

Defined in: [src/types.ts:174](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L174)

Full name

***

### gender?

```ts
optional gender: number;
```

Defined in: [src/types.ts:184](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L184)

Gender code (1=Male, 2=Female, 3=Other)

***

### guardian?

```ts
optional guardian: string;
```

Defined in: [src/types.ts:196](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L196)

Guardian name

***

### id?

```ts
optional id: string;
```

Defined in: [src/types.ts:168](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L168)

Unique identifier (CBOR key 1)

***

### language?

```ts
optional language: string;
```

Defined in: [src/types.ts:172](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L172)

Primary language code

***

### lastName?

```ts
optional lastName: string;
```

Defined in: [src/types.ts:180](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L180)

Last name

***

### leftIris?

```ts
optional leftIris: Biometric[];
```

Defined in: [src/types.ts:237](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L237)

Left iris biometrics

***

### leftLittleFinger?

```ts
optional leftLittleFinger: Biometric[];
```

Defined in: [src/types.ts:233](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L233)

Left little finger biometrics

***

### leftMiddleFinger?

```ts
optional leftMiddleFinger: Biometric[];
```

Defined in: [src/types.ts:229](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L229)

Left middle finger biometrics

***

### leftPalm?

```ts
optional leftPalm: Biometric[];
```

Defined in: [src/types.ts:243](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L243)

Left palm biometrics

***

### leftPointerFinger?

```ts
optional leftPointerFinger: Biometric[];
```

Defined in: [src/types.ts:227](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L227)

Left pointer finger biometrics

***

### leftRingFinger?

```ts
optional leftRingFinger: Biometric[];
```

Defined in: [src/types.ts:231](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L231)

Left ring finger biometrics

***

### leftThumb?

```ts
optional leftThumb: Biometric[];
```

Defined in: [src/types.ts:225](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L225)

Left thumb biometrics

***

### legalStatus?

```ts
optional legalStatus: string;
```

Defined in: [src/types.ts:210](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L210)

Legal status

***

### locationCode?

```ts
optional locationCode: string;
```

Defined in: [src/types.ts:208](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L208)

Location code

***

### maritalStatus?

```ts
optional maritalStatus: number;
```

Defined in: [src/types.ts:194](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L194)

Marital status code

***

### middleName?

```ts
optional middleName: string;
```

Defined in: [src/types.ts:178](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L178)

Middle name

***

### nationality?

```ts
optional nationality: string;
```

Defined in: [src/types.ts:192](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L192)

Nationality

***

### phone?

```ts
optional phone: string;
```

Defined in: [src/types.ts:190](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L190)

Phone number

***

### photo?

```ts
optional photo: Uint8Array<ArrayBufferLike>;
```

Defined in: [src/types.ts:198](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L198)

Photo data

***

### photoFormat?

```ts
optional photoFormat: number;
```

Defined in: [src/types.ts:200](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L200)

Photo format code

***

### rightIris?

```ts
optional rightIris: Biometric[];
```

Defined in: [src/types.ts:235](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L235)

Right iris biometrics

***

### rightLittleFinger?

```ts
optional rightLittleFinger: Biometric[];
```

Defined in: [src/types.ts:223](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L223)

Right little finger biometrics

***

### rightMiddleFinger?

```ts
optional rightMiddleFinger: Biometric[];
```

Defined in: [src/types.ts:219](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L219)

Right middle finger biometrics

***

### rightPalm?

```ts
optional rightPalm: Biometric[];
```

Defined in: [src/types.ts:241](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L241)

Right palm biometrics

***

### rightPointerFinger?

```ts
optional rightPointerFinger: Biometric[];
```

Defined in: [src/types.ts:217](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L217)

Right pointer finger biometrics

***

### rightRingFinger?

```ts
optional rightRingFinger: Biometric[];
```

Defined in: [src/types.ts:221](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L221)

Right ring finger biometrics

***

### rightThumb?

```ts
optional rightThumb: Biometric[];
```

Defined in: [src/types.ts:215](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L215)

Right thumb biometrics

***

### secondaryFullName?

```ts
optional secondaryFullName: string;
```

Defined in: [src/types.ts:204](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L204)

Secondary full name

***

### secondaryLanguage?

```ts
optional secondaryLanguage: string;
```

Defined in: [src/types.ts:206](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L206)

Secondary language code

***

### version?

```ts
optional version: string;
```

Defined in: [src/types.ts:170](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L170)

Claim version

***

### voice?

```ts
optional voice: Biometric[];
```

Defined in: [src/types.ts:245](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L245)

Voice biometrics

---

# Interface: Claim169Input

Defined in: [src/types.ts:460](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L460)

Input data for creating a Claim 169 credential.

This interface contains all identity fields that can be encoded into
a Claim 169 QR code.

## Example

```typescript
const claim169: Claim169Input = {
  id: "123456789",
  fullName: "John Doe",
  dateOfBirth: "1990-01-15",
  gender: 1,  // Male
};
```

## Properties

### address?

```ts
optional address: string;
```

Defined in: [src/types.ts:480](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L480)

Address

***

### countryOfIssuance?

```ts
optional countryOfIssuance: string;
```

Defined in: [src/types.ts:504](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L504)

Country of issuance

***

### dateOfBirth?

```ts
optional dateOfBirth: string;
```

Defined in: [src/types.ts:476](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L476)

Date of birth (ISO 8601 format)

***

### email?

```ts
optional email: string;
```

Defined in: [src/types.ts:482](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L482)

Email address

***

### firstName?

```ts
optional firstName: string;
```

Defined in: [src/types.ts:470](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L470)

First name

***

### fullName?

```ts
optional fullName: string;
```

Defined in: [src/types.ts:468](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L468)

Full name

***

### gender?

```ts
optional gender: number;
```

Defined in: [src/types.ts:478](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L478)

Gender code (1=Male, 2=Female, 3=Other)

***

### guardian?

```ts
optional guardian: string;
```

Defined in: [src/types.ts:490](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L490)

Guardian name

***

### id?

```ts
optional id: string;
```

Defined in: [src/types.ts:462](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L462)

Unique identifier

***

### language?

```ts
optional language: string;
```

Defined in: [src/types.ts:466](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L466)

Primary language code

***

### lastName?

```ts
optional lastName: string;
```

Defined in: [src/types.ts:474](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L474)

Last name

***

### legalStatus?

```ts
optional legalStatus: string;
```

Defined in: [src/types.ts:502](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L502)

Legal status

***

### locationCode?

```ts
optional locationCode: string;
```

Defined in: [src/types.ts:500](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L500)

Location code

***

### maritalStatus?

```ts
optional maritalStatus: number;
```

Defined in: [src/types.ts:488](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L488)

Marital status code (1=Unmarried, 2=Married, 3=Divorced)

***

### middleName?

```ts
optional middleName: string;
```

Defined in: [src/types.ts:472](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L472)

Middle name

***

### nationality?

```ts
optional nationality: string;
```

Defined in: [src/types.ts:486](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L486)

Nationality

***

### phone?

```ts
optional phone: string;
```

Defined in: [src/types.ts:484](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L484)

Phone number

***

### photo?

```ts
optional photo: Uint8Array<ArrayBufferLike>;
```

Defined in: [src/types.ts:492](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L492)

Photo data

***

### photoFormat?

```ts
optional photoFormat: number;
```

Defined in: [src/types.ts:494](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L494)

Photo format code (1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP)

***

### secondaryFullName?

```ts
optional secondaryFullName: string;
```

Defined in: [src/types.ts:496](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L496)

Secondary full name

***

### secondaryLanguage?

```ts
optional secondaryLanguage: string;
```

Defined in: [src/types.ts:498](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L498)

Secondary language code

***

### version?

```ts
optional version: string;
```

Defined in: [src/types.ts:464](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L464)

Claim version

---

# Interface: CwtMeta

Defined in: [src/types.ts:68](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L68)

CWT (CBOR Web Token) metadata from the credential.

Contains standard JWT/CWT claims that provide information about the
credential's validity, issuer, and subject.

## Example

```typescript
// Check if credential is expired
const now = Math.floor(Date.now() / 1000);
if (result.cwtMeta.expiresAt && result.cwtMeta.expiresAt < now) {
  console.log('Credential has expired!');
}

// Check issuer
if (result.cwtMeta.issuer === 'https://mosip.io') {
  console.log('Issued by MOSIP');
}
```

## Properties

### expiresAt?

```ts
optional expiresAt: number;
```

Defined in: [src/types.ts:74](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L74)

Expiration timestamp (Unix seconds) - credential invalid after this time

***

### issuedAt?

```ts
optional issuedAt: number;
```

Defined in: [src/types.ts:78](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L78)

Issued-at timestamp (Unix seconds) - when the credential was created

***

### issuer?

```ts
optional issuer: string;
```

Defined in: [src/types.ts:70](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L70)

Token issuer (typically a URL or identifier)

***

### notBefore?

```ts
optional notBefore: number;
```

Defined in: [src/types.ts:76](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L76)

Not-before timestamp (Unix seconds) - credential invalid before this time

***

### subject?

```ts
optional subject: string;
```

Defined in: [src/types.ts:72](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L72)

Token subject (typically the credential holder's ID)

---

# Interface: CwtMetaInput

Defined in: [src/types.ts:518](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L518)

CWT metadata input for creating a Claim 169 credential.

## Example

```typescript
const cwtMeta: CwtMetaInput = {
  issuer: "https://issuer.example.com",
  expiresAt: 1800000000,  // Unix timestamp
};
```

## Properties

### expiresAt?

```ts
optional expiresAt: number;
```

Defined in: [src/types.ts:524](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L524)

Expiration timestamp (Unix seconds)

***

### issuedAt?

```ts
optional issuedAt: number;
```

Defined in: [src/types.ts:528](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L528)

Issued-at timestamp (Unix seconds)

***

### issuer?

```ts
optional issuer: string;
```

Defined in: [src/types.ts:520](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L520)

Token issuer (typically a URL or identifier)

***

### notBefore?

```ts
optional notBefore: number;
```

Defined in: [src/types.ts:526](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L526)

Not-before timestamp (Unix seconds)

***

### subject?

```ts
optional subject: string;
```

Defined in: [src/types.ts:522](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L522)

Token subject (typically the credential holder's ID)

---

# Interface: DecodeResult

Defined in: [src/types.ts:400](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L400)

Result of decoding a Claim 169 QR code.

Contains the decoded identity data, CWT metadata, and verification status.

## Example

```typescript
// Decode with verification
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Access identity data
console.log(result.claim169.fullName);

// Access metadata
console.log(result.cwtMeta.issuer);

// Check verification status
console.log(result.verificationStatus); // "verified", "skipped", or "failed"
```

## Properties

### claim169

```ts
claim169: Claim169;
```

Defined in: [src/types.ts:402](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L402)

Decoded Claim 169 identity data

***

### cwtMeta

```ts
cwtMeta: CwtMeta;
```

Defined in: [src/types.ts:404](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L404)

CWT metadata (issuer, expiration, etc.)

***

### verificationStatus

```ts
verificationStatus: VerificationStatus;
```

Defined in: [src/types.ts:411](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L411)

Signature verification status.
- "verified": Signature verified successfully with provided public key
- "skipped": Verification skipped (allowUnverified() or decode(..., { allowUnverified: true }))
- "failed": Signature verification failed

***

### x509Headers

```ts
x509Headers: X509Headers;
```

Defined in: [src/types.ts:416](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L416)

X.509 headers from COSE protected/unprotected headers.
Contains certificate information for signature verification.

---

# Interface: IDecoder

Defined in: [src/types.ts:669](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L669)

Interface for the Decoder builder class.

Provides a fluent API for configuring and decoding Claim 169 QR codes.

## Example

```typescript
// With verification (recommended)
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Without verification (testing only)
const result = new Decoder(qrText)
  .allowUnverified()
  .skipBiometrics()
  .decode();

// With decryption and verification
const result = new Decoder(qrText)
  .decryptWithAes256(aesKey)
  .verifyWithEd25519(publicKey)
  .decode();
```

## Methods

### allowUnverified()

```ts
allowUnverified(): IDecoder;
```

Defined in: [src/types.ts:710](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L710)

Allow decoding without signature verification.
WARNING: Credentials decoded with verification skipped (`verificationStatus === "skipped"`)
cannot be trusted. Use for testing only.

#### Returns

`IDecoder`

The decoder instance for chaining

***

### clockSkewTolerance()

```ts
clockSkewTolerance(seconds): IDecoder;
```

Defined in: [src/types.ts:755](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L755)

Set clock skew tolerance in seconds.
Allows credentials to be accepted when clocks are slightly out of sync.
Applies when timestamp validation is enabled.

#### Parameters

##### seconds

`number`

The tolerance in seconds

#### Returns

`IDecoder`

The decoder instance for chaining

***

### decode()

```ts
decode(): DecodeResult;
```

Defined in: [src/types.ts:810](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L810)

Decode the QR code with the configured options.
Requires either a verifier (verifyWithEd25519/verifyWithEcdsaP256) or
explicit allowUnverified() to be called first.

#### Returns

`DecodeResult`

The decoded result

#### Throws

If decoding fails or no verification method specified

***

### decryptWith()

```ts
decryptWith(decryptor): IDecoder;
```

Defined in: [src/types.ts:801](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L801)

Decrypt with a custom decryptor callback.
Use for external crypto providers (HSM, cloud KMS, etc.)

#### Parameters

##### decryptor

`DecryptorCallback`

Function that decrypts ciphertext

#### Returns

`IDecoder`

The decoder instance for chaining

#### Example

```typescript
const result = new Decoder(qrText)
  .decryptWith((algorithm, keyId, nonce, aad, ciphertext) => {
    // Call your crypto provider here
    return myKms.decrypt(keyId, ciphertext, { nonce, aad });
  })
  .decode();
```

***

### decryptWithAes128()

```ts
decryptWithAes128(key): IDecoder;
```

Defined in: [src/types.ts:726](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L726)

Decrypt with AES-128-GCM.

#### Parameters

##### key

`Uint8Array`

16-byte AES-128 key

#### Returns

`IDecoder`

The decoder instance for chaining

#### Throws

If the key is invalid

***

### decryptWithAes256()

```ts
decryptWithAes256(key): IDecoder;
```

Defined in: [src/types.ts:718](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L718)

Decrypt with AES-256-GCM.

#### Parameters

##### key

`Uint8Array`

32-byte AES-256 key

#### Returns

`IDecoder`

The decoder instance for chaining

#### Throws

If the key is invalid

***

### maxDecompressedBytes()

```ts
maxDecompressedBytes(bytes): IDecoder;
```

Defined in: [src/types.ts:763](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L763)

Set maximum decompressed size in bytes.
Protects against decompression bomb attacks.

#### Parameters

##### bytes

`number`

The maximum size in bytes (default: 65536)

#### Returns

`IDecoder`

The decoder instance for chaining

***

### skipBiometrics()

```ts
skipBiometrics(): IDecoder;
```

Defined in: [src/types.ts:733](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L733)

Skip biometric data during decoding.
Useful when only demographic data is needed.

#### Returns

`IDecoder`

The decoder instance for chaining

***

### verifyWith()

```ts
verifyWith(verifier): IDecoder;
```

Defined in: [src/types.ts:782](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L782)

Verify signature with a custom verifier callback.
Use for external crypto providers (HSM, cloud KMS, remote signing, etc.)

#### Parameters

##### verifier

`VerifierCallback`

Function that verifies signatures

#### Returns

`IDecoder`

The decoder instance for chaining

#### Example

```typescript
const result = new Decoder(qrText)
  .verifyWith((algorithm, keyId, data, signature) => {
    // Call your crypto provider here
    myKms.verify(keyId, data, signature);
  })
  .decode();
```

***

### verifyWithEcdsaP256()

```ts
verifyWithEcdsaP256(publicKey): IDecoder;
```

Defined in: [src/types.ts:684](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L684)

Verify signature with ECDSA P-256 public key.

#### Parameters

##### publicKey

`Uint8Array`

SEC1-encoded P-256 public key (33 or 65 bytes)

#### Returns

`IDecoder`

The decoder instance for chaining

#### Throws

If the public key is invalid

***

### verifyWithEcdsaP256Pem()

```ts
verifyWithEcdsaP256Pem(pem): IDecoder;
```

Defined in: [src/types.ts:702](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L702)

Verify signature with ECDSA P-256 public key in PEM format.
Supports SPKI format with "BEGIN PUBLIC KEY" headers.

#### Parameters

##### pem

`string`

PEM-encoded P-256 public key

#### Returns

`IDecoder`

The decoder instance for chaining

#### Throws

If the PEM is invalid

***

### verifyWithEd25519()

```ts
verifyWithEd25519(publicKey): IDecoder;
```

Defined in: [src/types.ts:676](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L676)

Verify signature with Ed25519 public key.

#### Parameters

##### publicKey

`Uint8Array`

32-byte Ed25519 public key

#### Returns

`IDecoder`

The decoder instance for chaining

#### Throws

If the public key is invalid

***

### verifyWithEd25519Pem()

```ts
verifyWithEd25519Pem(pem): IDecoder;
```

Defined in: [src/types.ts:693](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L693)

Verify signature with Ed25519 public key in PEM format.
Supports SPKI format with "BEGIN PUBLIC KEY" headers.

#### Parameters

##### pem

`string`

PEM-encoded Ed25519 public key

#### Returns

`IDecoder`

The decoder instance for chaining

#### Throws

If the PEM is invalid

***

### withoutTimestampValidation()

```ts
withoutTimestampValidation(): IDecoder;
```

Defined in: [src/types.ts:746](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L746)

Disable timestamp validation.

#### Returns

`IDecoder`

The decoder instance for chaining

***

### withTimestampValidation()

```ts
withTimestampValidation(): IDecoder;
```

Defined in: [src/types.ts:740](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L740)

Enable timestamp validation.
When enabled, expired or not-yet-valid credentials will throw an error.

#### Returns

`IDecoder`

The decoder instance for chaining

---

# Interface: IEncoder

Defined in: [src/types.ts:543](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L543)

Interface for the Encoder builder class.

Provides a fluent API for configuring and encoding Claim 169 credentials.

## Example

```typescript
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();
```

## Methods

### allowUnsigned()

```ts
allowUnsigned(): IEncoder;
```

Defined in: [src/types.ts:577](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L577)

Allow encoding without a signature.
WARNING: Unsigned credentials cannot be verified. Use for testing only.

#### Returns

`IEncoder`

The encoder instance for chaining

***

### encode()

```ts
encode(): string;
```

Defined in: [src/types.ts:637](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L637)

Encode the credential to a Base45 QR string.

#### Returns

`string`

Base45-encoded string suitable for QR code generation

#### Throws

If encoding fails

***

### encryptWith()

```ts
encryptWith(encryptor, algorithm): IEncoder;
```

Defined in: [src/types.ts:627](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L627)

Encrypt with a custom encryptor callback.
Use for external crypto providers (HSM, cloud KMS, etc.)

#### Parameters

##### encryptor

`EncryptorCallback`

Function that encrypts data

##### algorithm

Encryption algorithm: "A256GCM" or "A128GCM"

`"A256GCM"` | `"A128GCM"`

#### Returns

`IEncoder`

The encoder instance for chaining

#### Example

```typescript
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(signKey)
  .encryptWith((algorithm, keyId, nonce, aad, plaintext) => {
    return myKms.encrypt({ keyId, nonce, aad, plaintext });
  }, "A256GCM")
  .encode();
```

***

### encryptWithAes128()

```ts
encryptWithAes128(key): IEncoder;
```

Defined in: [src/types.ts:570](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L570)

Encrypt with AES-128-GCM.

#### Parameters

##### key

`Uint8Array`

16-byte AES-128 key

#### Returns

`IEncoder`

The encoder instance for chaining

***

### encryptWithAes256()

```ts
encryptWithAes256(key): IEncoder;
```

Defined in: [src/types.ts:563](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L563)

Encrypt with AES-256-GCM.

#### Parameters

##### key

`Uint8Array`

32-byte AES-256 key

#### Returns

`IEncoder`

The encoder instance for chaining

***

### signWith()

```ts
signWith(
   signer, 
   algorithm, 
   keyId?): IEncoder;
```

Defined in: [src/types.ts:603](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L603)

Sign with a custom signer callback.
Use for external crypto providers (HSM, cloud KMS, remote signing, etc.)

#### Parameters

##### signer

`SignerCallback`

Function that signs data

##### algorithm

Signature algorithm: "EdDSA" or "ES256"

`"EdDSA"` | `"ES256"`

##### keyId?

Optional key identifier passed to the signer callback

`Uint8Array`\<`ArrayBufferLike`\> | `null`

#### Returns

`IEncoder`

The encoder instance for chaining

#### Example

```typescript
const qrData = new Encoder(claim169, cwtMeta)
  .signWith((algorithm, keyId, data) => {
    return myKms.sign({ keyId, data });
  }, "EdDSA")
  .encode();
```

***

### signWithEcdsaP256()

```ts
signWithEcdsaP256(privateKey): IEncoder;
```

Defined in: [src/types.ts:556](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L556)

Sign with ECDSA P-256 private key.

#### Parameters

##### privateKey

`Uint8Array`

32-byte ECDSA P-256 private key (scalar)

#### Returns

`IEncoder`

The encoder instance for chaining

***

### signWithEd25519()

```ts
signWithEd25519(privateKey): IEncoder;
```

Defined in: [src/types.ts:549](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L549)

Sign with Ed25519 private key.

#### Parameters

##### privateKey

`Uint8Array`

32-byte Ed25519 private key

#### Returns

`IEncoder`

The encoder instance for chaining

***

### skipBiometrics()

```ts
skipBiometrics(): IEncoder;
```

Defined in: [src/types.ts:583](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L583)

Skip biometric fields during encoding.

#### Returns

`IEncoder`

The encoder instance for chaining

---

# Interface: X509Headers

Defined in: [src/types.ts:120](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L120)

X.509 headers extracted from COSE protected/unprotected headers.

These headers provide X.509 certificate information for signature verification
as defined in RFC 9360.

## Example

```typescript
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Check for certificate chain
if (result.x509Headers.x5chain) {
  console.log(`Certificate chain has ${result.x509Headers.x5chain.length} certificates`);
}

// Check for certificate URL
if (result.x509Headers.x5u) {
  console.log(`Certificate URL: ${result.x509Headers.x5u}`);
}
```

## Properties

### x5bag?

```ts
optional x5bag: Uint8Array<ArrayBufferLike>[];
```

Defined in: [src/types.ts:125](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L125)

x5bag (COSE label 32): Unordered bag of X.509 certificates.
Each certificate is DER-encoded.

***

### x5chain?

```ts
optional x5chain: Uint8Array<ArrayBufferLike>[];
```

Defined in: [src/types.ts:131](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L131)

x5chain (COSE label 33): Ordered chain of X.509 certificates.
The first certificate contains the public key used for verification.
Each certificate is DER-encoded.

***

### x5t?

```ts
optional x5t: CertificateHash;
```

Defined in: [src/types.ts:136](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L136)

x5t (COSE label 34): Certificate thumbprint hash.
Used to identify the certificate by its hash.

***

### x5u?

```ts
optional x5u: string;
```

Defined in: [src/types.ts:141](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L141)

x5u (COSE label 35): URI pointing to an X.509 certificate.
Can be used to fetch the certificate for verification.

---

# claim169/types

Type definitions for MOSIP Claim 169 QR Code decoder.

This module contains TypeScript interfaces and types for the decoded
Claim 169 identity data.

## Classes

- Claim169Error

## Interfaces

- Biometric
- CertificateHash
- Claim169
- Claim169Input
- CwtMeta
- CwtMetaInput
- DecodeResult
- IDecoder
- IEncoder
- X509Headers

## Type Aliases

- Algorithm
- AlgorithmName
- DecryptorCallback
- EncryptorCallback
- SignerCallback
- VerificationStatus
- VerifierCallback

---

# Type Alias: Algorithm

```ts
type Algorithm = "EdDSA" | "ES256" | "A256GCM" | "A128GCM";
```

Defined in: [src/types.ts:268](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L268)

Algorithm identifier for COSE algorithms.
- "EdDSA" - Edwards-curve Digital Signature Algorithm (Ed25519)
- "ES256" - ECDSA with P-256 and SHA-256
- "A256GCM" - AES-256-GCM encryption
- "A128GCM" - AES-128-GCM encryption

---

# Type Alias: AlgorithmName

```ts
type AlgorithmName = 
  | Algorithm
  | string & object;
```

Defined in: [src/types.ts:276](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L276)

Algorithm identifier as surfaced by the underlying WASM bindings.

This preserves autocomplete for known values while still allowing
unknown strings for forwards compatibility.

---

# Type Alias: DecryptorCallback()

```ts
type DecryptorCallback = (algorithm, keyId, nonce, aad, ciphertext) => Uint8Array;
```

Defined in: [src/types.ts:322](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L322)

Custom decryptor callback.
Use for external crypto providers (HSM, cloud KMS, etc.)

## Parameters

### algorithm

`AlgorithmName`

COSE algorithm identifier (e.g., "A256GCM", "A128GCM")

### keyId

Optional key identifier from the COSE header

`Uint8Array` | `null`

### nonce

`Uint8Array`

Nonce/IV for decryption (12 bytes for AES-GCM)

### aad

`Uint8Array`

Additional authenticated data

### ciphertext

`Uint8Array`

Ciphertext with authentication tag

## Returns

`Uint8Array`

Decrypted plaintext

## Example

```typescript
const myDecryptor: DecryptorCallback = (algorithm, keyId, nonce, aad, ciphertext) => {
  return myKms.decrypt({ keyId, nonce, aad, ciphertext });
};
```

---

# Type Alias: EncryptorCallback()

```ts
type EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => Uint8Array;
```

Defined in: [src/types.ts:370](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L370)

Custom encryptor callback.
Use for external crypto providers (HSM, cloud KMS, etc.)

## Parameters

### algorithm

`AlgorithmName`

COSE algorithm identifier (e.g., "A256GCM", "A128GCM")

### keyId

Optional key identifier

`Uint8Array` | `null`

### nonce

`Uint8Array`

Nonce/IV for encryption (12 bytes for AES-GCM)

### aad

`Uint8Array`

Additional authenticated data

### plaintext

`Uint8Array`

Data to encrypt

## Returns

`Uint8Array`

Ciphertext with authentication tag

## Example

```typescript
const myEncryptor: EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => {
  return myKms.encrypt({ keyId, nonce, aad, plaintext });
};
```

---

# Type Alias: SignerCallback()

```ts
type SignerCallback = (algorithm, keyId, data) => Uint8Array;
```

Defined in: [src/types.ts:346](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L346)

Custom signer callback.
Use for external crypto providers (HSM, cloud KMS, remote signing, etc.)

## Parameters

### algorithm

`AlgorithmName`

COSE algorithm identifier (e.g., "EdDSA", "ES256")

### keyId

Optional key identifier

`Uint8Array` | `null`

### data

`Uint8Array`

Data to sign (the COSE Sig_structure)

## Returns

`Uint8Array`

Signature bytes

## Example

```typescript
const mySigner: SignerCallback = (algorithm, keyId, data) => {
  return myKms.sign({ keyId, algorithm, data });
};
```

---

# Type Alias: VerificationStatus

```ts
type VerificationStatus = "verified" | "skipped" | "failed";
```

Defined in: [src/types.ts:255](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L255)

Signature verification status of the decoded credential.

- `"verified"`: Signature was verified successfully with the provided public key
- `"skipped"`: Verification was explicitly skipped using `allowUnverified()`
- `"failed"`: Signature verification failed (invalid signature or wrong key)

---

# Type Alias: VerifierCallback()

```ts
type VerifierCallback = (algorithm, keyId, data, signature) => void;
```

Defined in: [src/types.ts:297](https://github.com/jeremi/claim-169/blob/948e64606129dc0eba7b2a54cf002a27676158b4/sdks/typescript/src/types.ts#L297)

Custom signature verifier callback.
Use for external crypto providers (HSM, cloud KMS, remote signing, etc.)

The callback should throw an error if verification fails.

## Parameters

### algorithm

`AlgorithmName`

COSE algorithm identifier (e.g., "EdDSA", "ES256")

### keyId

Optional key identifier from the COSE header

`Uint8Array` | `null`

### data

`Uint8Array`

Data that was signed (the COSE Sig_structure)

### signature

`Uint8Array`

Signature to verify

## Returns

`void`

## Example

```typescript
const myVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
  const result = myKms.verify({ keyId, algorithm, data, signature });
  if (!result.valid) throw new Error("Verification failed");
};
```

---

# claim169

## Modules

- claim169
- claim169/types

---

# claim169

> **Alpha Software**: This library is under active development. APIs may change without notice. Not recommended for production use without thorough testing.

[![npm](https://img.shields.io/npm/v/claim169.svg)](https://www.npmjs.com/package/claim169)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

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

### Builder Pattern (Recommended)

```typescript
import { Decoder } from 'claim169';

// Decode with Ed25519 signature verification (recommended)
const publicKey = new Uint8Array(32);  // Your 32-byte public key
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

console.log(`ID: ${result.claim169.id}`);
console.log(`Name: ${result.claim169.fullName}`);
console.log(`Issuer: ${result.cwtMeta.issuer}`);
console.log(`Verified: ${result.verificationStatus}`);  // "verified"

// Decode without verification (testing only)
const result = new Decoder(qrText)
  .allowUnverified()
  .decode();
```

### `decode()` Convenience Function

> **Security note**: `decode()` requires a verification key unless you explicitly set `allowUnverified: true` (testing only). Use the `Decoder` builder API in production.

```typescript
import { decode, type DecodeOptions } from 'claim169';

// Simple decode (testing only)
const result = decode(qrText, { allowUnverified: true });

// With options
const options: DecodeOptions = {
  maxDecompressedBytes: 32768,  // 32KB limit
  skipBiometrics: true,         // Skip biometric parsing
  // Timestamp validation is enabled by default (host-side). Set to false to disable:
  validateTimestamps: false,
  allowUnverified: true,        // Explicit opt-out (testing only)
};

const result = decode(qrText, options);
```

## Decoder Class

The `Decoder` class provides a fluent builder API:

```typescript
import { Decoder } from 'claim169';

// Decode with Ed25519 verification
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Decode with ECDSA P-256 verification
const result = new Decoder(qrText)
  .verifyWithEcdsaP256(publicKey)  // 33 or 65 bytes SEC1 encoded
  .decode();

// Decrypt then verify (for encrypted credentials)
const result = new Decoder(qrText)
  .decryptWithAes256(aesKey)
  .verifyWithEd25519(publicKey)
  .decode();

// With additional options
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .skipBiometrics()              // Skip biometric data
  .clockSkewTolerance(60)        // 60 seconds tolerance
  .maxDecompressedBytes(32768)   // 32KB max size
  .decode();
```

### Decoder Methods

| Method | Description |
|--------|-------------|
| `verifyWithEd25519(publicKey)` | Verify with Ed25519 (32 bytes) |
| `verifyWithEcdsaP256(publicKey)` | Verify with ECDSA P-256 (33 or 65 bytes) |
| `verifyWith(callback)` | Verify with custom callback (HSM, cloud KMS, etc.) |
| `decryptWithAes256(key)` | Decrypt with AES-256-GCM (32 bytes) |
| `decryptWithAes128(key)` | Decrypt with AES-128-GCM (16 bytes) |
| `decryptWith(callback)` | Decrypt with custom callback (HSM, cloud KMS, etc.) |
| `allowUnverified()` | Skip verification (testing only) |
| `skipBiometrics()` | Skip biometric data parsing |
| `withTimestampValidation()` | Enable timestamp validation (host-side) |
| `withoutTimestampValidation()` | Disable timestamp validation |
| `clockSkewTolerance(seconds)` | Set clock skew tolerance |
| `maxDecompressedBytes(bytes)` | Set max decompressed size |
| `decode()` | Execute the decode operation |

## Encoding

The `Encoder` class creates MOSIP Claim 169 QR code data from identity information.
In production, keys should be provisioned and managed externally (HSM/KMS or secure key management). The examples below assume you already have key material.

```typescript
import { Encoder, Decoder, Claim169Input, CwtMetaInput, generateNonce } from 'claim169';

// Create identity data
const claim169: Claim169Input = {
  id: "123456789",
  fullName: "John Doe",
  dateOfBirth: "1990-01-15",
  gender: 1,  // Male
};

// Create CWT metadata
const cwtMeta: CwtMetaInput = {
  issuer: "https://issuer.example.com",
  expiresAt: 1800000000,
};

// Encode with Ed25519 signature
const privateKey = new Uint8Array(32);  // Your 32-byte private key
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();

// Encode with signature and AES-256 encryption
const aesKey = new Uint8Array(32);  // Your 32-byte AES key
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encryptWithAes256(aesKey)
  .encode();

// Unsigned (testing only)
const qrData = new Encoder(claim169, cwtMeta)
  .allowUnsigned()
  .encode();
```

### Encoder Methods

| Method | Description |
|--------|-------------|
| `signWithEd25519(privateKey)` | Sign with Ed25519 |
| `signWithEcdsaP256(privateKey)` | Sign with ECDSA P-256 |
| `signWith(callback, algorithm, keyId?)` | Sign with custom callback (HSM, cloud KMS, etc.) |
| `encryptWithAes256(key)` | Encrypt with AES-256-GCM |
| `encryptWithAes128(key)` | Encrypt with AES-128-GCM |
| `encryptWith(callback, algorithm)` | Encrypt with custom callback (HSM, cloud KMS, etc.) |
| `allowUnsigned()` | Allow unsigned (testing only) |
| `skipBiometrics()` | Skip biometric fields |
| `encode()` | Produce the QR string |

### generateNonce()

Generate a cryptographically secure random nonce for encryption:

```typescript
import { generateNonce } from 'claim169';

const nonce = generateNonce();  // Returns 12-byte Uint8Array
```

## Custom Crypto Providers

For integrating with external key management systems like HSMs, cloud KMS (AWS KMS, Google Cloud KMS, Azure Key Vault), smart cards, TPMs, or remote signing services, use the custom callback methods.

### Custom Signer

```typescript
import { Encoder, SignerCallback, Claim169Input, CwtMetaInput } from 'claim169';

// Example: Sign with a cloud KMS
const mySigner: SignerCallback = (algorithm, keyId, data) => {
  // Call your crypto provider
  // algorithm: "EdDSA" or "ES256"
  // keyId: optional key identifier (Uint8Array or null)
  // data: the COSE Sig_structure to sign (Uint8Array)
  const signature = myKms.sign({ keyId, data, algorithm });
  return signature;  // Uint8Array: 64 bytes for EdDSA, 64 bytes for ES256
};

const claim: Claim169Input = { id: "123", fullName: "John Doe" };
const meta: CwtMetaInput = { issuer: "https://issuer.example" };

const qrData = new Encoder(claim, meta)
  .signWith(mySigner, "EdDSA", new Uint8Array([1, 2, 3])) // optional keyId
  .encode();
```

### Custom Verifier

```typescript
import { Decoder, VerifierCallback } from 'claim169';

// Example: Verify with an HSM
const myVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
  // Call your crypto provider
  // Throw an error if verification fails
  const result = myHsm.verify({ keyId, data, signature, algorithm });
  if (!result.valid) {
    throw new Error("Signature verification failed");
  }
};

const result = new Decoder(qrText)
  .verifyWith(myVerifier)
  .decode();
```

### Custom Encryptor

```typescript
import { Encoder, EncryptorCallback } from 'claim169';

// Example: Encrypt with cloud KMS
const myEncryptor: EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => {
  // algorithm: "A256GCM" or "A128GCM"
  // nonce: 12-byte IV
  // aad: additional authenticated data
  // plaintext: data to encrypt
  const ciphertext = myKms.encrypt({ keyId, nonce, aad, plaintext });
  return ciphertext;  // Uint8Array: plaintext + 16-byte auth tag
};

const qrData = new Encoder(claim, meta)
  .signWithEd25519(signingKey)
  .encryptWith(myEncryptor, "A256GCM")
  .encode();
```

### Custom Decryptor

```typescript
import { Decoder, DecryptorCallback } from 'claim169';

// Example: Decrypt with cloud KMS
const myDecryptor: DecryptorCallback = (algorithm, keyId, nonce, aad, ciphertext) => {
  // algorithm: "A256GCM" or "A128GCM"
  // ciphertext includes the auth tag
  const plaintext = myKms.decrypt({ keyId, nonce, aad, ciphertext });
  return plaintext;  // Uint8Array
};

const result = new Decoder(qrText)
  .decryptWith(myDecryptor)
  .verifyWithEd25519(publicKey)
  .decode();
```

### Callback Type Definitions

```typescript
// Signer: (algorithm, keyId, data) => signature
type SignerCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  data: Uint8Array
) => Uint8Array;

// Verifier: (algorithm, keyId, data, signature) => void (throw on failure)
type VerifierCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  data: Uint8Array,
  signature: Uint8Array
) => void;

// Encryptor: (algorithm, keyId, nonce, aad, plaintext) => ciphertext
type EncryptorCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  nonce: Uint8Array,
  aad: Uint8Array,
  plaintext: Uint8Array
) => Uint8Array;

// Decryptor: (algorithm, keyId, nonce, aad, ciphertext) => plaintext
type DecryptorCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  nonce: Uint8Array,
  aad: Uint8Array,
  ciphertext: Uint8Array
) => Uint8Array;
```

## Data Model

### DecodeResult

```typescript
interface DecodeResult {
  claim169: Claim169;                    // Identity data
  cwtMeta: CwtMeta;                      // Token metadata
  verificationStatus: VerificationStatus; // "verified" | "skipped" | "failed"
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
  // ... (all finger/iris/palm biometrics)
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
  const result = decode(qrText, { allowUnverified: true });
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

## Notes

### Timestamp Validation

Timestamp validation is performed in the host (JavaScript) and is enabled by default. Disable it explicitly if you intentionally want to skip time checks:

```typescript
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .withoutTimestampValidation()
  .clockSkewTolerance(60)  // Allow 60 seconds clock drift
  .decode();
```

### Secure by Default

The decoder requires explicit verification configuration. You must call one of:
- `verifyWithEd25519(publicKey)` - Verify with Ed25519
- `verifyWithEcdsaP256(publicKey)` - Verify with ECDSA P-256
- `allowUnverified()` - Explicitly skip verification (testing only)

This prevents accidentally processing unverified credentials.

## Browser Usage

```html
<script type="module">
  import { Decoder } from './node_modules/claim169/dist/index.js';

  // Your issuer's public key (32 bytes for Ed25519)
  const publicKey = new Uint8Array([/* ... */]);

  const qrText = "6BF5YZB2...";
  const result = new Decoder(qrText)
    .verifyWithEd25519(publicKey)
    .decode();

  if (result.verificationStatus === "verified") {
    console.log("Verified:", result.claim169.fullName);
  }
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
