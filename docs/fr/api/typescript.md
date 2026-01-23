# Référence API TypeScript

## Installation

```bash
npm install claim169
```

## Référence rapide

```ts
import {
  // Erreurs
  Claim169Error,
  // API de convenance (vérification requise par défaut)
  decode,
  type DecodeOptions,
  // API builder (recommandée en production)
  Decoder,
  Encoder,
  // Types
  type Claim169Input,
  type CwtMetaInput,
  type DecodeResult,
  // Utilitaires
  hexToBytes,
  bytesToHex,
  generateNonce,
  version,
  isLoaded,
} from "claim169";
```

!!! warning "À propos de `decode()`"
    `decode()` exige une clé de vérification par défaut. Pour décoder sans vérification explicitement (tests uniquement), passez `{ allowUnverified: true }`. En production, utilisez `new Decoder(...).verifyWithEd25519(...)` / `verifyWithEcdsaP256(...)`.

## `decode(qrText, options?)`

```ts
decode(qrText: string, options?: DecodeOptions): DecodeResult
```

Options courantes :

- `verifyWithEd25519` / `verifyWithEcdsaP256`
- `allowUnverified` (doit être `true` si aucune clé n’est fournie)
- `decryptWithAes256` / `decryptWithAes128`
- `skipBiometrics`
- `validateTimestamps` (désactivé par défaut en WASM)
- `clockSkewToleranceSeconds`
- `maxDecompressedBytes`

## Decoder (builder)

```ts
new Decoder(qrText: string)
```

### Vérification

- `verifyWithEd25519(publicKey: Uint8Array)` (32 octets)
- `verifyWithEcdsaP256(publicKey: Uint8Array)` (33 ou 65 octets, SEC1)
- `allowUnverified()` (tests uniquement)

### Déchiffrement

- `decryptWithAes256(key: Uint8Array)` (32 octets)
- `decryptWithAes128(key: Uint8Array)` (16 octets)

### Options

- `skipBiometrics()`
- `withTimestampValidation()`
- `clockSkewTolerance(seconds: number)`
- `maxDecompressedBytes(bytes: number)`

### Exécuter

- `decode(): DecodeResult`

## Encoder (builder)

```ts
new Encoder(claim169: Claim169Input, cwtMeta: CwtMetaInput)
```

### Signature

- `signWithEd25519(privateKey: Uint8Array)` (32 octets)
- `signWithEcdsaP256(privateKey: Uint8Array)` (32 octets, scalaire)
- `allowUnsigned()` (tests uniquement)

### Chiffrement

- `encryptWithAes256(key: Uint8Array)` (32 octets)
- `encryptWithAes128(key: Uint8Array)` (16 octets)

### Options

- `skipBiometrics()`

### Exécuter

- `encode(): string`

## Utilitaires

- `hexToBytes(hex: string): Uint8Array`
- `bytesToHex(bytes: Uint8Array): string`
- `generateNonce(): Uint8Array` (12 octets)
- `version(): string`
- `isLoaded(): boolean`

## Utilisation navigateur

Le SDK fonctionne dans les navigateurs modernes avec support WebAssembly :

```html
<script type="module">
  import { Decoder } from "claim169";

  const result = new Decoder(qrData).allowUnverified().decode();
  document.getElementById("name").textContent = result.claim169.fullName;
</script>
```

## Utilisation Node.js

Node.js 16+ est requis pour WebAssembly :

```js
import { Decoder } from "claim169";

const result = new Decoder(qrData).allowUnverified().decode();
console.log(result.claim169.fullName);
```
