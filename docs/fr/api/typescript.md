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

## Fournisseurs Cryptographiques Personnalisés

Les fournisseurs cryptographiques personnalisés permettent l'intégration avec des systèmes cryptographiques externes tels que les Modules de Sécurité Matérielle (HSM), les Services de Gestion de Clés cloud (AWS KMS, Google Cloud KMS, Azure Key Vault), les cartes à puce, les Modules de Plateforme Sécurisée (TPM) et les services de signature à distance.

### Types de Callback

```ts
// Callback de signature : signe les données et retourne la signature
// Lève une exception en cas d'échec
type SignerCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  data: Uint8Array
) => Uint8Array;

// Callback de vérification : vérifie une signature, lève une exception si invalide
type VerifierCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  data: Uint8Array,
  signature: Uint8Array
) => void;

// Callback de chiffrement : chiffre le texte en clair et retourne le texte chiffré avec le tag d'authentification
type EncryptorCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  nonce: Uint8Array,
  aad: Uint8Array,
  plaintext: Uint8Array
) => Uint8Array;

// Callback de déchiffrement : déchiffre le texte chiffré et retourne le texte en clair
type DecryptorCallback = (
  algorithm: string,
  keyId: Uint8Array | null,
  nonce: Uint8Array,
  aad: Uint8Array,
  ciphertext: Uint8Array
) => Uint8Array;
```

### Identifiants d'Algorithmes

- `"EdDSA"` - Signatures Ed25519
- `"ES256"` - Signatures ECDSA P-256
- `"A128GCM"` - Chiffrement AES-128-GCM
- `"A256GCM"` - Chiffrement AES-256-GCM

### Decoder avec Fournisseurs Personnalisés

#### `verifyWith(callback)`

Vérifier les signatures en utilisant un callback de vérification personnalisé :

```ts
verifyWith(callback: VerifierCallback): Decoder
```

Exemple avec AWS KMS :

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

Déchiffrer les identifiants en utilisant un callback de déchiffrement personnalisé :

```ts
decryptWith(callback: DecryptorCallback): Decoder
```

Exemple avec Google Cloud KMS :

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
  .allowUnverified()  // Ou utilisez .verifyWith() pour la vérification de signature
  .decode();
```

### Encoder avec Fournisseurs Personnalisés

#### `signWith(callback, algorithm)`

Signer les identifiants en utilisant un callback de signature personnalisé :

```ts
signWith(callback: SignerCallback, algorithm: string): Encoder
```

Exemple avec Azure Key Vault :

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

Chiffrer les identifiants en utilisant un callback de chiffrement personnalisé :

```ts
encryptWith(callback: EncryptorCallback, algorithm: string): Encoder
```

Exemple avec HSM pour la signature et le chiffrement :

```ts
const hsmSigner: SignerCallback = (algorithm, keyId, data) => {
  // Utiliser le HSM pour signer
  return hsm.sign(keyId, data, algorithm);
};

const hsmEncryptor: EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => {
  // Utiliser le HSM pour chiffrer (retourne texte chiffré + tag d'authentification)
  return hsm.encryptAead(keyId, nonce, aad, plaintext, algorithm);
};

const qrText = new Encoder(claim169, cwtMeta)
  .signWith(hsmSigner, "EdDSA")
  .encryptWith(hsmEncryptor, "A256GCM")
  .encode();
```

### Exemple Combiné avec Fournisseurs Personnalisés

Exemple complet utilisant des fournisseurs personnalisés pour l'encodage et le décodage :

```ts
// Encoder avec signature personnalisée (ex: TPM)
const tpmSigner: SignerCallback = (algorithm, keyId, data) => {
  return tpm.sign(data, { algorithm, keyHandle: keyId });
};

const signedQr = new Encoder(claim169, cwtMeta)
  .signWith(tpmSigner, "ES256")
  .encode();

// Plus tard : décoder avec vérification personnalisée
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
