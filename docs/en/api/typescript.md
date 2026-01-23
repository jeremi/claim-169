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
  // Convenience API (verification required by default)
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
    `decode()` requires a verification key by default. To explicitly decode without verification (testing only), pass `{ allowUnverified: true }`. Use the `Decoder` builder with `.verifyWithEd25519()` / `.verifyWithEcdsaP256()` in production.

## `decode(qrText, options?)`

```ts
decode(qrText: string, options?: DecodeOptions): DecodeResult
```

Common options:

- `verifyWithEd25519` / `verifyWithEcdsaP256`
- `allowUnverified` (must be set to `true` if no key is provided)
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

## Custom Crypto Providers

Custom crypto providers enable integration with external cryptographic systems such as Hardware Security Modules (HSMs), cloud Key Management Services (AWS KMS, Google Cloud KMS, Azure Key Vault), smart cards, Trusted Platform Modules (TPMs), and remote signing services.

### Callback Types

```ts
// Signer callback: signs data and returns the signature
// Throws an exception on failure
type SignerCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  data: Uint8Array
) => Uint8Array;

// Verifier callback: verifies a signature, throws if invalid
type VerifierCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  data: Uint8Array,
  signature: Uint8Array
) => void;

// Encryptor callback: encrypts plaintext and returns ciphertext with auth tag
type EncryptorCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  nonce: Uint8Array,
  aad: Uint8Array,
  plaintext: Uint8Array
) => Uint8Array;

// Decryptor callback: decrypts ciphertext and returns plaintext
type DecryptorCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  nonce: Uint8Array,
  aad: Uint8Array,
  ciphertext: Uint8Array
) => Uint8Array;
```

### Algorithm Identifiers

- `"EdDSA"` - Ed25519 signatures
- `"ES256"` - ECDSA P-256 signatures
- `"A128GCM"` - AES-128-GCM encryption
- `"A256GCM"` - AES-256-GCM encryption

### Decoder with Custom Providers

#### `verifyWith(callback)`

Verify signatures using a custom verifier callback:

```ts
verifyWith(callback: VerifierCallback): Decoder
```

Example with AWS KMS:

```ts
import { KMSClient, VerifyCommand } from "@aws-sdk/client-kms";

const kms = new KMSClient({ region: "us-east-1" });

const awsVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
  const command = new VerifyCommand({
    KeyId: "arn:aws:kms:us-east-1:123456789:key/example-key-id",
    Message: data,
    MessageType: "RAW",
    Signature: signature,
    SigningAlgorithm: algorithm === "ES256" ? "ECDSA_SHA_256" : "EDDSA",
  });

  const response = await kms.send(command);
  if (!response.SignatureValid) {
    throw new Error("Signature verification failed");
  }
};

const result = new Decoder(qrText)
  .verifyWith(awsVerifier)
  .decode();
```

#### `decryptWith(callback)`

Decrypt credentials using a custom decryptor callback:

```ts
decryptWith(callback: DecryptorCallback): Decoder
```

Example with Google Cloud KMS:

```ts
import { KeyManagementServiceClient } from "@google-cloud/kms";

const kmsClient = new KeyManagementServiceClient();

const gcpDecryptor: DecryptorCallback = (algorithm, keyId, nonce, aad, ciphertext) => {
  const keyName = kmsClient.cryptoKeyPath(
    "my-project",
    "global",
    "my-keyring",
    "my-key"
  );

  const [result] = await kmsClient.decrypt({
    name: keyName,
    ciphertext: ciphertext,
    additionalAuthenticatedData: aad,
  });

  return new Uint8Array(result.plaintext);
};

const result = new Decoder(encryptedQrText)
  .decryptWith(gcpDecryptor)
  .allowUnverified()  // Or use .verifyWith() for signature verification
  .decode();
```

### Encoder with Custom Providers

#### `signWith(callback, algorithm)`

Sign credentials using a custom signer callback:

```ts
signWith(callback: SignerCallback, algorithm: string): Encoder
```

Example with Azure Key Vault:

```ts
import { CryptographyClient } from "@azure/keyvault-keys";
import { DefaultAzureCredential } from "@azure/identity";

const credential = new DefaultAzureCredential();
const cryptoClient = new CryptographyClient(
  "https://my-vault.vault.azure.net/keys/my-key/version",
  credential
);

const azureSigner: SignerCallback = (algorithm, keyId, data) => {
  const signAlgorithm = algorithm === "ES256" ? "ES256" : "EdDSA";
  const result = await cryptoClient.sign(signAlgorithm, data);
  return new Uint8Array(result.result);
};

const qrText = new Encoder(claim169, cwtMeta)
  .signWith(azureSigner, "ES256")
  .encode();
```

#### `encryptWith(callback, algorithm)`

Encrypt credentials using a custom encryptor callback:

```ts
encryptWith(callback: EncryptorCallback, algorithm: string): Encoder
```

Example with HSM for both signing and encryption:

```ts
const hsmSigner: SignerCallback = (algorithm, keyId, data) => {
  // Use HSM to sign
  return hsm.sign(keyId, data, algorithm);
};

const hsmEncryptor: EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => {
  // Use HSM to encrypt (returns ciphertext + auth tag)
  return hsm.encryptAead(keyId, nonce, aad, plaintext, algorithm);
};

const qrText = new Encoder(claim169, cwtMeta)
  .signWith(hsmSigner, "EdDSA")
  .encryptWith(hsmEncryptor, "A256GCM")
  .encode();
```

### Combined Custom Provider Example

Full example using custom providers for both encoding and decoding:

```ts
// Encode with custom signing (e.g., TPM)
const tpmSigner: SignerCallback = (algorithm, keyId, data) => {
  return tpm.sign(data, { algorithm, keyHandle: keyId });
};

const signedQr = new Encoder(claim169, cwtMeta)
  .signWith(tpmSigner, "ES256")
  .encode();

// Later: decode with custom verification
const tpmVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
  const valid = tpm.verify(data, signature, { algorithm, keyHandle: keyId });
  if (!valid) {
    throw new Error("TPM signature verification failed");
  }
};

const decoded = new Decoder(signedQr)
  .verifyWith(tpmVerifier)
  .decode();
```

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
