# Décoder des identifiants

La classe `Decoder` parse des QR codes MOSIP Claim 169 et extrait les données d’identité via une API fluide (builder).

## Décodage basique

### Avec vérification Ed25519 (recommandé)

```typescript
import { Decoder } from 'claim169';

const qrText = "6BF5YZB2..."; // Contenu du QR code
const publicKey = new Uint8Array(32); // Clé publique Ed25519

const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

console.log('ID:', result.claim169.id);
console.log('Name:', result.claim169.fullName);
console.log('Verified:', result.verificationStatus); // "verified"
```

### Avec vérification ECDSA P-256

```typescript
// Clé publique encodée SEC1 (33 octets compressée) ou 65 octets (non compressée)
const publicKey = new Uint8Array(33); // ou 65 octets

const result = new Decoder(qrText)
  .verifyWithEcdsaP256(publicKey)
  .decode();
```

### Avec des clés publiques PEM

Si vous avez des clés publiques au format PEM (p. ex. depuis OpenSSL), vous pouvez utiliser les méthodes de vérification PEM :

```typescript
// Ed25519 avec PEM
const ed25519Pem = `-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEA11qYAYKxCrfVS/7TyWQHOg7hcvPapjJa8CCWX4cBURo=
-----END PUBLIC KEY-----`;

const result = new Decoder(qrText)
  .verifyWithEd25519Pem(ed25519Pem)
  .decode();

// ECDSA P-256 avec PEM
const ecdsaPem = `-----BEGIN PUBLIC KEY-----
MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE...
-----END PUBLIC KEY-----`;

const result = new Decoder(qrText)
  .verifyWithEcdsaP256Pem(ecdsaPem)
  .decode();
```

### Sans vérification (tests uniquement)

```typescript
// WARNING: ne jamais utiliser en production !
const result = new Decoder(qrText)
  .allowUnverified()
  .decode();

console.log('Status:', result.verificationStatus); // "skipped"
```

## Décoder des identifiants chiffrés

### Déchiffrement AES-256-GCM

```typescript
const aesKey = new Uint8Array(32); // Clé AES-256 (32 octets)
const publicKey = new Uint8Array(32); // Clé publique Ed25519

// L’ordre compte : d’abord déchiffrer, puis vérifier
const result = new Decoder(qrText)
  .decryptWithAes256(aesKey)
  .verifyWithEd25519(publicKey)
  .decode();
```

### Déchiffrement AES-128-GCM

```typescript
const aesKey = new Uint8Array(16); // Clé AES-128 (16 octets)

const result = new Decoder(qrText)
  .decryptWithAes128(aesKey)
  .verifyWithEd25519(publicKey)
  .decode();
```

## Options du décodeur

### Ignorer la biométrie

Ignorer le parsing biométrique pour accélérer lorsque seule la démographie est nécessaire :

```typescript
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .skipBiometrics()
  .decode();

// Les champs biométriques seront undefined
console.log(result.claim169.face); // undefined
```

### Validation des horodatages

La validation des horodatages `exp` (expires at) et `nbf` (not before) est activée par défaut (côté hôte en JavaScript). Désactivez-la uniquement si vous souhaitez volontairement ignorer les contrôles de temps :

```typescript
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .withoutTimestampValidation()
  .decode();
```

Note : la validation des horodatages est effectuée côté hôte (JavaScript) afin d’éviter les limitations de temps propres à l’exécution WASM.

### Tolérance à la dérive d’horloge

Autoriser une tolérance aux écarts d’horloge :

```typescript
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .clockSkewTolerance(60) // Autoriser 60 secondes de dérive
  .decode();
```

### Taille maximale après décompression

Définir une limite de taille décompressée pour se protéger des bombes de décompression :

```typescript
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .maxDecompressedBytes(32768) // Limite 32KB
  .decode();
```

La limite par défaut est 65536 octets (64KB).

## Accéder aux données décodées

### Données d’identité (Claim169)

```typescript
const { claim169 } = result;

// Démographie
console.log('ID:', claim169.id);
console.log('Full Name:', claim169.fullName);
console.log('First Name:', claim169.firstName);
console.log('Last Name:', claim169.lastName);
console.log('DOB:', claim169.dateOfBirth);
console.log('Gender:', claim169.gender);
console.log('Email:', claim169.email);
console.log('Phone:', claim169.phone);
console.log('Address:', claim169.address);
console.log('Nationality:', claim169.nationality);

// Photo
if (claim169.photo) {
  console.log('Photo size:', claim169.photo.byteLength);
  console.log('Photo format:', claim169.photoFormat);
}

// Biométrie
if (claim169.face && claim169.face.length > 0) {
  const faceData = claim169.face[0];
  console.log('Face biometric format:', faceData.format);
  console.log('Face data size:', faceData.data.byteLength);
}
```

### Métadonnées CWT

```typescript
const { cwtMeta } = result;

console.log('Issuer:', cwtMeta.issuer);
console.log('Subject:', cwtMeta.subject);
console.log('Issued At:', cwtMeta.issuedAt);
console.log('Expires At:', cwtMeta.expiresAt);
console.log('Not Before:', cwtMeta.notBefore);

// Vérifier l’expiration
if (cwtMeta.expiresAt) {
  const expiresDate = new Date(cwtMeta.expiresAt * 1000);
  const isExpired = expiresDate < new Date();
  console.log('Expired:', isExpired);
}
```

### Statut de vérification

```typescript
const { verificationStatus } = result;

switch (verificationStatus) {
  case 'verified':
    console.log('Signature verified successfully');
    break;
  case 'skipped':
    console.log('Verification was skipped (allowUnverified)');
    break;
  case 'failed':
    console.log('Signature verification failed');
    break;
}
```

## Utiliser la fonction de convenance

Pour des usages plus simples, utilisez la fonction `decode()` :

```typescript
import { decode, type DecodeOptions } from 'claim169';

// Avec vérification
const result = decode(qrText, {
  verifyWithEd25519: publicKey,
});

// Avec toutes les options
const options: DecodeOptions = {
  verifyWithEd25519: publicKey,
  // ou: verifyWithEcdsaP256: ecdsaPublicKey,
  // ou: allowUnverified: true,

  decryptWithAes256: aesKey,
  // ou: decryptWithAes128: aes128Key,

  skipBiometrics: true,
  validateTimestamps: true,
  clockSkewToleranceSeconds: 60,
  maxDecompressedBytes: 32768,
};

const result = decode(qrText, options);
```

## Gestion des erreurs

```typescript
import { Decoder, Claim169Error } from 'claim169';

try {
  const result = new Decoder(qrText)
    .verifyWithEd25519(publicKey)
    .decode();

  console.log('Decoded:', result.claim169.fullName);
} catch (error) {
  if (error instanceof Claim169Error) {
    console.error('Decode failed:', error.message);

    // Détecter des erreurs spécifiques
    if (error.message.includes('Base45')) {
      console.log('Invalid QR code encoding');
    } else if (error.message.includes('signature')) {
      console.log('Signature verification failed');
    } else if (error.message.includes('expired')) {
      console.log('Credential has expired');
    }
  }
}
```

### Erreurs fréquentes

| Erreur | Cause |
|-------|-------|
| `Ed25519 public key must be 32 bytes` | Taille de clé Ed25519 invalide |
| `ECDSA P-256 public key must be 33 bytes (compressed) or 65 bytes (uncompressed)` | Taille de clé ECDSA invalide |
| `AES-256 key must be 32 bytes` | Taille de clé de déchiffrement invalide |
| `Must call verifyWith...() or allowUnverified()` | Aucune méthode de vérification spécifiée |
| `Base45Decode` | Encodage Base45 invalide |
| `Decompress` | Échec de décompression zlib |
| `CoseParse` | Structure COSE invalide |
| `Claim169NotFound` | Claim 169 absent du CWT |
| `SignatureError` | Échec de vérification de signature |
| `DecryptionError` | Échec de déchiffrement (mauvaise clé ou données corrompues) |

## Exemple complet de chaînage

```typescript
const result = new Decoder(qrText)
  .decryptWithAes256(aesKey)
  .verifyWithEd25519(publicKey)
  .skipBiometrics()
  .clockSkewTolerance(120)
  .maxDecompressedBytes(65536)
  .decode();
```

## Travailler avec des données biométriques

Lorsque des données biométriques sont présentes :

```typescript
const { claim169 } = result;

// Empreintes
const fingerprints = [
  { name: 'Right Thumb', data: claim169.rightThumb },
  { name: 'Right Pointer', data: claim169.rightPointerFinger },
  { name: 'Right Middle', data: claim169.rightMiddleFinger },
  { name: 'Right Ring', data: claim169.rightRingFinger },
  { name: 'Right Little', data: claim169.rightLittleFinger },
  { name: 'Left Thumb', data: claim169.leftThumb },
  { name: 'Left Pointer', data: claim169.leftPointerFinger },
  { name: 'Left Middle', data: claim169.leftMiddleFinger },
  { name: 'Left Ring', data: claim169.leftRingFinger },
  { name: 'Left Little', data: claim169.leftLittleFinger },
];

for (const fp of fingerprints) {
  if (fp.data && fp.data.length > 0) {
    console.log(`${fp.name}: ${fp.data[0].data.byteLength} bytes`);
  }
}

// Iris
if (claim169.rightIris) {
  console.log('Right iris data available');
}
if (claim169.leftIris) {
  console.log('Left iris data available');
}

// Visage
if (claim169.face && claim169.face.length > 0) {
  const face = claim169.face[0];
  console.log('Face biometric:', {
    format: face.format,
    subFormat: face.subFormat,
    size: face.data.byteLength,
    issuer: face.issuer,
  });
}

// Voix
if (claim169.voice) {
  console.log('Voice biometric available');
}
```

### Codes de format biométrique

| Format | Description |
|--------|-------------|
| 0 | Image |
| 1 | Template |
| 2 | Sound |
| 3 | BioHash |

### Sous-formats d’image

| Code | Format |
|------|--------|
| 0 | PNG |
| 1 | JPEG |
| 2 | JPEG2000 |
| 3 | AVIF |
| 4 | WebP |
| 5 | TIFF |
| 6 | WSQ |

## Étapes suivantes

- [Chiffrement](encryption.md) - Travailler avec des identifiants chiffrés
- [Crypto personnalisée](custom-crypto.md) - Intégration HSM et KMS
- [Référence API](api.md) - API complète de Decoder
