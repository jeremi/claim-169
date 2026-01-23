# Referencia API TypeScript

## Instalación

```bash
npm install claim169
```

## Referencia rápida

```ts
import {
  // Errores
  Claim169Error,
  // API de conveniencia (sin verificación por defecto)
  decode,
  type DecodeOptions,
  // API builder (recomendada en producción)
  Decoder,
  Encoder,
  // Tipos
  type Claim169Input,
  type CwtMetaInput,
  type DecodeResult,
  // Utilidades
  hexToBytes,
  bytesToHex,
  generateNonce,
  version,
  isLoaded,
} from "claim169";
```

!!! warning "Sobre `decode()`"
    `decode()` es una función de conveniencia que **no verifica** la firma si no proporcionas una clave en `DecodeOptions`. En producción, usa `new Decoder(...).verifyWithEd25519(...)` / `verifyWithEcdsaP256(...)`.

## `decode(qrText, options?)`

```ts
decode(qrText: string, options?: DecodeOptions): DecodeResult
```

Opciones comunes:

- `verifyWithEd25519` / `verifyWithEcdsaP256`
- `allowUnverified` (por defecto `true` si no se proporciona clave)
- `decryptWithAes256` / `decryptWithAes128`
- `skipBiometrics`
- `validateTimestamps` (desactivado por defecto en WASM)
- `clockSkewToleranceSeconds`
- `maxDecompressedBytes`

## Decoder (builder)

```ts
new Decoder(qrText: string)
```

### Verificación

- `verifyWithEd25519(publicKey: Uint8Array)` (32 bytes)
- `verifyWithEcdsaP256(publicKey: Uint8Array)` (33 o 65 bytes, SEC1)
- `allowUnverified()` (solo pruebas)

### Descifrado

- `decryptWithAes256(key: Uint8Array)` (32 bytes)
- `decryptWithAes128(key: Uint8Array)` (16 bytes)

### Opciones

- `skipBiometrics()`
- `withTimestampValidation()`
- `clockSkewTolerance(seconds: number)`
- `maxDecompressedBytes(bytes: number)`

### Ejecutar

- `decode(): DecodeResult`

## Encoder (builder)

```ts
new Encoder(claim169: Claim169Input, cwtMeta: CwtMetaInput)
```

### Firma

- `signWithEd25519(privateKey: Uint8Array)` (32 bytes)
- `signWithEcdsaP256(privateKey: Uint8Array)` (32 bytes, escalar)
- `allowUnsigned()` (solo pruebas)

### Cifrado

- `encryptWithAes256(key: Uint8Array)` (32 bytes)
- `encryptWithAes128(key: Uint8Array)` (16 bytes)

### Opciones

- `skipBiometrics()`

### Ejecutar

- `encode(): string`

## Utilidades

- `hexToBytes(hex: string): Uint8Array`
- `bytesToHex(bytes: Uint8Array): string`
- `generateNonce(): Uint8Array` (12 bytes)
- `version(): string`
- `isLoaded(): boolean`

## Uso en navegador

El SDK funciona en navegadores modernos con soporte WebAssembly:

```html
<script type="module">
  import { Decoder } from "claim169";

  const result = new Decoder(qrData).allowUnverified().decode();
  document.getElementById("name").textContent = result.claim169.fullName;
</script>
```

## Uso en Node.js

Se requiere Node.js 16+ para WebAssembly:

```js
import { Decoder } from "claim169";

const result = new Decoder(qrData).allowUnverified().decode();
console.log(result.claim169.fullName);
```

