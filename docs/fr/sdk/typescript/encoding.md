# Encoder des identifiants

La classe `Encoder` crée des données QR code MOSIP Claim 169 à partir d’informations d’identité via une API fluide (builder).

## Encodage basique

### Identifiant signé (recommandé)

```typescript
import { Encoder, type Claim169Input, type CwtMetaInput } from 'claim169';

const claim169: Claim169Input = {
  id: "MOSIP-123456789",
  fullName: "John Doe",
  dateOfBirth: "1985-03-15",
  gender: 1,  // Male
};

const cwtMeta: CwtMetaInput = {
  issuer: "https://issuer.example.com",
  issuedAt: Math.floor(Date.now() / 1000),
  expiresAt: Math.floor(Date.now() / 1000) + 86400 * 365,
};

// Clé privée Ed25519 (32 octets)
const privateKey = new Uint8Array(32);

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();
```

### Identifiant non signé (tests uniquement)

```typescript
// WARNING: un identifiant non signé ne peut pas être vérifié
const qrData = new Encoder(claim169, cwtMeta)
  .allowUnsigned()
  .encode();
```

## Algorithmes de signature

### Ed25519 (EdDSA)

Algorithme recommandé pour la plupart des cas :

```typescript
// Clé privée Ed25519 de 32 octets
const privateKey = new Uint8Array(32);

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();
```

### ECDSA P-256 (ES256)

Pour la compatibilité avec des systèmes imposant les courbes NIST :

```typescript
// Clé privée ECDSA P-256 de 32 octets (scalaire)
const privateKey = new Uint8Array(32);

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEcdsaP256(privateKey)
  .encode();
```

## Exemple complet (démographie)

```typescript
import { Encoder, type Claim169Input, type CwtMetaInput } from 'claim169';

const claim169: Claim169Input = {
  // Identité
  id: "MOSIP-987654321",
  version: "1.0.0",
  language: "en",

  // Champs de nom
  fullName: "Maria Garcia-Rodriguez",
  firstName: "Maria",
  middleName: "Elena",
  lastName: "Garcia-Rodriguez",

  // Démographie
  dateOfBirth: "1992-08-22",
  gender: 2,  // Female (1=Male, 2=Female, 3=Other)

  // Contact
  address: "123 Main Street, Apt 4B, Springfield, IL 62701",
  email: "maria.garcia@example.com",
  phone: "+1-555-123-4567",

  // Légal
  nationality: "US",
  maritalStatus: 2,  // Married (1=Unmarried, 2=Married, 3=Divorced)
  guardian: "Carlos Garcia",
  legalStatus: "citizen",
  countryOfIssuance: "US",

  // Localisation
  locationCode: "US-IL-62701",

  // Langue secondaire
  secondaryFullName: "Maria Garcia-Rodriguez",
  secondaryLanguage: "es",
};

const cwtMeta: CwtMetaInput = {
  issuer: "https://gov.example.com/identity",
  subject: "maria.garcia@example.com",
  issuedAt: Math.floor(Date.now() / 1000),
  expiresAt: Math.floor(Date.now() / 1000) + 86400 * 365 * 5, // 5 years
  notBefore: Math.floor(Date.now() / 1000),
};

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();
```

## Ajouter des photos

```typescript
const claim169: Claim169Input = {
  id: "PHOTO-001",
  fullName: "Photo Test",

  // Photo en JPEG (photoFormat: 1)
  photo: await readPhotoAsBytes('photo.jpg'),
  photoFormat: 1,  // 1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP
};

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();
```

## Identifiants chiffrés

### Chiffrement AES-256-GCM

```typescript
// Signer puis chiffrer
const signKey = new Uint8Array(32);   // Clé privée Ed25519
const encryptKey = new Uint8Array(32); // Clé AES-256

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(signKey)
  .encryptWithAes256(encryptKey)
  .encode();
```

### Chiffrement AES-128-GCM

```typescript
const signKey = new Uint8Array(32);   // Clé privée Ed25519
const encryptKey = new Uint8Array(16); // Clé AES-128

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(signKey)
  .encryptWithAes128(encryptKey)
  .encode();
```

## Options de l’encodeur

### Ignorer la biométrie

Ignorer les champs biométriques à l’encodage :

```typescript
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .skipBiometrics()
  .encode();
```

## Chaînage des méthodes

Toutes les méthodes renvoient l’instance d’encodeur pour un chaînage fluide :

```typescript
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(signKey)
  .skipBiometrics()
  .encryptWithAes256(encryptKey)
  .encode();
```

## Gestion des erreurs

```typescript
import { Encoder, Claim169Error } from 'claim169';

try {
  const qrData = new Encoder(claim169, cwtMeta)
    .signWithEd25519(privateKey)
    .encode();
} catch (error) {
  if (error instanceof Claim169Error) {
    console.error('Encoding failed:', error.message);
  }
}
```

### Erreurs fréquentes

| Erreur | Cause |
|-------|-------|
| `Ed25519 private key must be 32 bytes` | Taille de clé Ed25519 invalide |
| `ECDSA P-256 private key must be 32 bytes` | Taille de clé ECDSA invalide |
| `AES-256 key must be 32 bytes` | Taille de clé AES-256 invalide |
| `AES-128 key must be 16 bytes` | Taille de clé AES-128 invalide |
| `Must call signWith...() or allowUnsigned()` | Aucune méthode de signature spécifiée |

## Champs de métadonnées CWT

| Champ | Type | Description |
|-------|------|-------------|
| `issuer` | string | URL/identifiant de l’émetteur |
| `subject` | string | Identifiant sujet (p. ex. email) |
| `issuedAt` | number | Timestamp Unix d’émission |
| `expiresAt` | number | Timestamp Unix d’expiration |
| `notBefore` | number | Timestamp Unix de début de validité |

## Codes genre et état civil

### Genre

| Code | Valeur |
|------|--------|
| 1 | Male |
| 2 | Female |
| 3 | Other |

### État civil

| Code | Valeur |
|------|--------|
| 1 | Unmarried |
| 2 | Married |
| 3 | Divorced |

### Format photo

| Code | Format |
|------|--------|
| 1 | JPEG |
| 2 | JPEG2000 |
| 3 | AVIF |
| 4 | WebP |

## Génération de nonce

Pour les opérations de chiffrement, générez des nonces aléatoires sûrs :

```typescript
import { generateNonce } from 'claim169';

// Générer un nonce de 12 octets pour AES-GCM
const nonce = generateNonce();
console.log('Nonce length:', nonce.length); // 12
```

## Exemple roundtrip

Encoder puis décoder pour vérifier :

```typescript
import { Encoder, Decoder } from 'claim169';

// Encode
const claim169 = { id: "TEST-001", fullName: "Test User" };
const cwtMeta = { issuer: "https://test.example" };

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();

// Decode and verify
const publicKey = derivePublicKey(privateKey);
const result = new Decoder(qrData)
  .verifyWithEd25519(publicKey)
  .decode();

console.log('Roundtrip:', result.claim169.id === claim169.id); // true
console.log('Verified:', result.verificationStatus); // "verified"
```

## Étapes suivantes

- [Chiffrement](encryption.md) - Exemples détaillés
- [Crypto personnalisée](custom-crypto.md) - Intégration HSM et KMS cloud
- [Référence API](api.md) - API complète d’Encoder
