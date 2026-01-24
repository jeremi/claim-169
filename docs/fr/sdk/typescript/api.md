# Référence API

Documentation API complète pour le SDK TypeScript claim169.

## Classes

### Decoder

Décodeur (pattern builder) pour les QR codes Claim 169.

```typescript
class Decoder implements IDecoder {
  constructor(qrText: string);

  verifyWithEd25519(publicKey: Uint8Array): Decoder;
  verifyWithEcdsaP256(publicKey: Uint8Array): Decoder;
  verifyWith(verifier: VerifierCallback): Decoder;
  allowUnverified(): Decoder;

  decryptWithAes256(key: Uint8Array): Decoder;
  decryptWithAes128(key: Uint8Array): Decoder;
  decryptWith(decryptor: DecryptorCallback): Decoder;

  skipBiometrics(): Decoder;
  withTimestampValidation(): Decoder;
  clockSkewTolerance(seconds: number): Decoder;
  maxDecompressedBytes(bytes: number): Decoder;

  decode(): DecodeResult;
}
```

#### Constructeur

```typescript
new Decoder(qrText: string)
```

| Paramètre | Type | Description |
|-----------|------|-------------|
| `qrText` | string | Contenu QR encodé en Base45 |

#### Méthodes

##### `verifyWithEd25519(publicKey)`

Vérifier la signature avec une clé publique Ed25519.

| Paramètre | Type | Description |
|-----------|------|-------------|
| `publicKey` | Uint8Array | Clé publique Ed25519 (32 octets) |

**Renvoie** : `Decoder` (chaînage)

**Lève** : `Claim169Error` si la clé est invalide

##### `verifyWithEcdsaP256(publicKey)`

Vérifier la signature avec une clé publique ECDSA P-256.

| Paramètre | Type | Description |
|-----------|------|-------------|
| `publicKey` | Uint8Array | Clé P-256 encodée SEC1 (33 ou 65 octets) |

**Renvoie** : `Decoder` (chaînage)

**Lève** : `Claim169Error` si la clé est invalide

##### `verifyWith(verifier)`

Vérifier via un callback de vérification personnalisé.

| Paramètre | Type | Description |
|-----------|------|-------------|
| `verifier` | VerifierCallback | Fonction de vérification personnalisée |

**Renvoie** : `Decoder` (chaînage)

##### `allowUnverified()`

Ignorer la vérification de signature. **ATTENTION** : tests uniquement.

**Renvoie** : `Decoder` (chaînage)

##### `decryptWithAes256(key)`

Déchiffrer avec AES-256-GCM.

| Paramètre | Type | Description |
|-----------|------|-------------|
| `key` | Uint8Array | Clé AES-256 (32 octets) |

**Renvoie** : `Decoder` (chaînage)

**Lève** : `Claim169Error` si la clé est invalide

##### `decryptWithAes128(key)`

Déchiffrer avec AES-128-GCM.

| Paramètre | Type | Description |
|-----------|------|-------------|
| `key` | Uint8Array | Clé AES-128 (16 octets) |

**Renvoie** : `Decoder` (chaînage)

**Lève** : `Claim169Error` si la clé est invalide

##### `decryptWith(decryptor)`

Déchiffrer via un callback de déchiffrement personnalisé.

| Paramètre | Type | Description |
|-----------|------|-------------|
| `decryptor` | DecryptorCallback | Fonction de déchiffrement personnalisée |

**Renvoie** : `Decoder` (chaînage)

##### `skipBiometrics()`

Ignorer le parsing biométrique.

**Renvoie** : `Decoder` (chaînage)

##### `withTimestampValidation()`

Activer la validation des horodatages exp/nbf.

**Renvoie** : `Decoder` (chaînage)

##### `clockSkewTolerance(seconds)`

Définir une tolérance à la dérive d’horloge.

| Paramètre | Type | Description |
|-----------|------|-------------|
| `seconds` | number | Tolérance en secondes |

**Renvoie** : `Decoder` (chaînage)

##### `maxDecompressedBytes(bytes)`

Définir la taille maximale après décompression.

| Paramètre | Type | Description |
|-----------|------|-------------|
| `bytes` | number | Taille maximale (par défaut : 65536) |

**Renvoie** : `Decoder` (chaînage)

##### `decode()`

Exécuter l’opération de décodage.

**Renvoie** : `DecodeResult`

**Lève** : `Claim169Error` en cas d’échec de décodage

---

### Encoder

Encodeur (pattern builder) pour des identifiants Claim 169.

```typescript
class Encoder implements IEncoder {
  constructor(claim169: Claim169Input, cwtMeta: CwtMetaInput);

  signWithEd25519(privateKey: Uint8Array): Encoder;
  signWithEcdsaP256(privateKey: Uint8Array): Encoder;
  signWith(signer: SignerCallback, algorithm: "EdDSA" | "ES256"): Encoder;
  allowUnsigned(): Encoder;

  encryptWithAes256(key: Uint8Array): Encoder;
  encryptWithAes128(key: Uint8Array): Encoder;
  encryptWith(encryptor: EncryptorCallback, algorithm: "A256GCM" | "A128GCM"): Encoder;

  skipBiometrics(): Encoder;

  encode(): string;
}
```

#### Constructeur

```typescript
new Encoder(claim169: Claim169Input, cwtMeta: CwtMetaInput)
```

| Paramètre | Type | Description |
|-----------|------|-------------|
| `claim169` | Claim169Input | Données d’identité à encoder |
| `cwtMeta` | CwtMetaInput | Métadonnées CWT |

#### Méthodes

##### `signWithEd25519(privateKey)`

Signer avec une clé privée Ed25519.

| Paramètre | Type | Description |
|-----------|------|-------------|
| `privateKey` | Uint8Array | Clé privée Ed25519 (32 octets) |

**Renvoie** : `Encoder` (chaînage)

**Lève** : `Claim169Error` si la clé est invalide

##### `signWithEcdsaP256(privateKey)`

Signer avec une clé privée ECDSA P-256.

| Paramètre | Type | Description |
|-----------|------|-------------|
| `privateKey` | Uint8Array | Clé privée P-256 (32 octets, scalaire) |

**Renvoie** : `Encoder` (chaînage)

**Lève** : `Claim169Error` si la clé est invalide

##### `signWith(signer, algorithm)`

Signer via un callback de signature personnalisé.

| Paramètre | Type | Description |
|-----------|------|-------------|
| `signer` | SignerCallback | Fonction de signature personnalisée |
| `algorithm` | "EdDSA" \| "ES256" | Algorithme de signature |

**Renvoie** : `Encoder` (chaînage)

##### `allowUnsigned()`

Ignorer la signature. **ATTENTION** : tests uniquement.

**Renvoie** : `Encoder` (chaînage)

##### `encryptWithAes256(key)`

Chiffrer avec AES-256-GCM.

| Paramètre | Type | Description |
|-----------|------|-------------|
| `key` | Uint8Array | Clé AES-256 (32 octets) |

**Renvoie** : `Encoder` (chaînage)

**Lève** : `Claim169Error` si la clé est invalide

##### `encryptWithAes128(key)`

Chiffrer avec AES-128-GCM.

| Paramètre | Type | Description |
|-----------|------|-------------|
| `key` | Uint8Array | Clé AES-128 (16 octets) |

**Renvoie** : `Encoder` (chaînage)

**Lève** : `Claim169Error` si la clé est invalide

##### `encryptWith(encryptor, algorithm)`

Chiffrer via un callback de chiffrement personnalisé.

| Paramètre | Type | Description |
|-----------|------|-------------|
| `encryptor` | EncryptorCallback | Fonction de chiffrement personnalisée |
| `algorithm` | "A256GCM" \| "A128GCM" | Algorithme de chiffrement |

**Renvoie** : `Encoder` (chaînage)

##### `skipBiometrics()`

Ignorer les champs biométriques à l’encodage.

**Renvoie** : `Encoder` (chaînage)

##### `encode()`

Produire la chaîne QR.

**Renvoie** : `string` — données QR encodées en Base45

**Lève** : `Claim169Error` en cas d’échec d’encodage

---

### Claim169Error

Classe d’erreur pour les erreurs du SDK.

```typescript
class Claim169Error extends Error {
  constructor(message: string);
  name: "Claim169Error";
}
```

## Fonctions

### decode()

Fonction de convenance pour décoder.

```typescript
function decode(qrText: string, options?: DecodeOptions): DecodeResult;
```

| Paramètre | Type | Description |
|-----------|------|-------------|
| `qrText` | string | Contenu QR encodé en Base45 |
| `options` | DecodeOptions | Options de décodage |

**Renvoie** : `DecodeResult`

**Lève** : `Claim169Error` en cas d’échec

### version()

Récupérer la version de la bibliothèque.

```typescript
function version(): string;
```

**Renvoie** : chaîne de version (p. ex. "0.1.0-alpha.2")

### isLoaded()

Vérifier si le module WASM est chargé.

```typescript
function isLoaded(): boolean;
```

**Renvoie** : `true` si WASM est prêt

### hexToBytes()

Convertir une chaîne hex en bytes.

```typescript
function hexToBytes(hex: string): Uint8Array;
```

| Paramètre | Type | Description |
|-----------|------|-------------|
| `hex` | string | Chaîne hex (préfixe 0x optionnel, ignore les espaces) |

**Renvoie** : `Uint8Array`

**Lève** : `Claim169Error` si l’hex est invalide

### bytesToHex()

Convertir des bytes en chaîne hex.

```typescript
function bytesToHex(bytes: Uint8Array): string;
```

| Paramètre | Type | Description |
|-----------|------|-------------|
| `bytes` | Uint8Array | Bytes à convertir |

**Renvoie** : chaîne hex en minuscules

### generateNonce()

Générer un nonce aléatoire de 12 octets.

```typescript
function generateNonce(): Uint8Array;
```

**Renvoie** : `Uint8Array(12)` pour AES-GCM

## Interfaces

### DecodeResult

```typescript
interface DecodeResult {
  claim169: Claim169;
  cwtMeta: CwtMeta;
  verificationStatus: VerificationStatus;
}
```

### Claim169

```typescript
interface Claim169 {
  // Core identity
  id?: string;
  version?: string;
  language?: string;

  // Name fields
  fullName?: string;
  firstName?: string;
  middleName?: string;
  lastName?: string;

  // Demographics
  dateOfBirth?: string;
  gender?: number;
  address?: string;
  email?: string;
  phone?: string;
  nationality?: string;
  maritalStatus?: number;
  guardian?: string;

  // Additional
  secondaryFullName?: string;
  secondaryLanguage?: string;
  locationCode?: string;
  legalStatus?: string;
  countryOfIssuance?: string;

  // Photo
  photo?: Uint8Array;
  photoFormat?: number;
  bestQualityFingers?: Uint8Array;

  // Biometrics
  rightThumb?: Biometric[];
  rightPointerFinger?: Biometric[];
  rightMiddleFinger?: Biometric[];
  rightRingFinger?: Biometric[];
  rightLittleFinger?: Biometric[];
  leftThumb?: Biometric[];
  leftPointerFinger?: Biometric[];
  leftMiddleFinger?: Biometric[];
  leftRingFinger?: Biometric[];
  leftLittleFinger?: Biometric[];
  rightIris?: Biometric[];
  leftIris?: Biometric[];
  face?: Biometric[];
  rightPalm?: Biometric[];
  leftPalm?: Biometric[];
  voice?: Biometric[];
}
```

### Claim169Input

Interface d’entrée pour l’encodage (mêmes champs que Claim169, tous optionnels).

### CwtMeta

```typescript
interface CwtMeta {
  issuer?: string;
  subject?: string;
  expiresAt?: number;
  notBefore?: number;
  issuedAt?: number;
}
```

### CwtMetaInput

Interface d’entrée pour l’encodage (mêmes champs que CwtMeta, tous optionnels).

### Biometric

```typescript
interface Biometric {
  data: Uint8Array;
  format: number;
  subFormat?: number;
  issuer?: string;
}
```

### DecodeOptions

```typescript
interface DecodeOptions {
  verifyWithEd25519?: Uint8Array;
  verifyWithEcdsaP256?: Uint8Array;
  decryptWithAes256?: Uint8Array;
  decryptWithAes128?: Uint8Array;
  allowUnverified?: boolean;
  skipBiometrics?: boolean;
  validateTimestamps?: boolean;
  clockSkewToleranceSeconds?: number;
  maxDecompressedBytes?: number;
}
```

## Types

### VerificationStatus

```typescript
type VerificationStatus = "verified" | "skipped" | "failed";
```

### Algorithm

```typescript
type Algorithm = "EdDSA" | "ES256" | "A256GCM" | "A128GCM";
```

### SignerCallback

```typescript
type SignerCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  data: Uint8Array
) => Uint8Array;
```

### VerifierCallback

```typescript
type VerifierCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  data: Uint8Array,
  signature: Uint8Array
) => void;
```

### EncryptorCallback

```typescript
type EncryptorCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  nonce: Uint8Array,
  aad: Uint8Array,
  plaintext: Uint8Array
) => Uint8Array;
```

### DecryptorCallback

```typescript
type DecryptorCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  nonce: Uint8Array,
  aad: Uint8Array,
  ciphertext: Uint8Array
) => Uint8Array;
```

## Énumérations

### Gender

| Valeur | Signification |
|-------|---------------|
| 1 | Male |
| 2 | Female |
| 3 | Other |

### Marital Status

| Valeur | Signification |
|-------|---------------|
| 1 | Unmarried |
| 2 | Married |
| 3 | Divorced |

### Photo Format

| Valeur | Format |
|-------|--------|
| 1 | JPEG |
| 2 | JPEG2000 |
| 3 | AVIF |
| 4 | WebP |

### Biometric Format

| Valeur | Type |
|-------|------|
| 0 | Image |
| 1 | Template |
| 2 | Sound |
| 3 | BioHash |

### Image Sub-Format

| Valeur | Format |
|-------|--------|
| 0 | PNG |
| 1 | JPEG |
| 2 | JPEG2000 |
| 3 | AVIF |
| 4 | WebP |
| 5 | TIFF |
| 6 | WSQ |

### Template Sub-Format

| Valeur | Format |
|-------|--------|
| 0 | ANSI378 |
| 1 | ISO19794-2 |
| 2 | NIST |

### Sound Sub-Format

| Valeur | Format |
|-------|--------|
| 0 | WAV |
| 1 | MP3 |

## Exports

Toutes les APIs publiques sont exportées depuis le module principal :

```typescript
// Classes
export { Decoder, Encoder, Claim169Error };

// Functions
export { decode, version, isLoaded, hexToBytes, bytesToHex, generateNonce };

// Types
export type {
  Algorithm,
  Biometric,
  Claim169,
  Claim169Input,
  CwtMeta,
  CwtMetaInput,
  DecodeOptions,
  DecodeResult,
  DecryptorCallback,
  EncryptorCallback,
  IDecoder,
  IEncoder,
  SignerCallback,
  VerificationStatus,
  VerifierCallback,
};
```

