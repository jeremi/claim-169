# Démarrage rapide

Démarrez avec MOSIP Claim 169 en 5 minutes.

## Installer le SDK

```bash
npm install claim169
```

## Décoder un QR code

L’opération la plus courante consiste à décoder un QR code pour extraire des données d’identité :

!!! warning "Ne pas modifier la chaîne Base45"
    L’alphabet Base45 inclut un caractère espace (`" "`). Conservez le texte scanné tel quel (pas de `.trim()` ni normalisation des espaces), sinon vous risquez de corrompre des identifiants valides.

```typescript
import { Decoder } from 'claim169';

// Contenu du QR code (chaîne Base45)
const qrText = "6BF5YZB2K2RJMB2T...";

// Clé publique Ed25519 de l’émetteur (32 octets)
const publicKey = new Uint8Array([
  0x3d, 0x2a, 0x1b, /* ... 32 bytes total */
]);

// Décoder avec vérification de signature
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Accéder aux données d’identité
console.log('ID:', result.claim169.id);
console.log('Name:', result.claim169.fullName);
console.log('DOB:', result.claim169.dateOfBirth);

// Accéder aux métadonnées
console.log('Issuer:', result.cwtMeta.issuer);
console.log('Expires:', new Date(result.cwtMeta.expiresAt! * 1000));

// Vérifier le statut
console.log('Verified:', result.verificationStatus); // "verified"
```

## Encoder un identifiant

Créer un QR code d’identifiant signé :

```typescript
import { Encoder, Gender, type Claim169Input, type CwtMetaInput } from 'claim169';

// Données d'identité à encoder
const claim169: Claim169Input = {
  id: "123456789",
  fullName: "Jane Smith",
  dateOfBirth: "1990-05-20",
  gender: Gender.Female,
  email: "jane@example.com",
};

// Métadonnées du jeton
const cwtMeta: CwtMetaInput = {
  issuer: "https://issuer.example.com",
  issuedAt: Math.floor(Date.now() / 1000),
  expiresAt: Math.floor(Date.now() / 1000) + 86400 * 365, // 1 year
};

// Clé privée Ed25519 (32 octets)
const privateKey = new Uint8Array([
  0x4a, 0x1c, 0x7b, /* ... 32 bytes total */
]);

// Créer les données QR signées
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();

console.log('QR Data:', qrData);
// Utiliser cette chaîne pour générer une image de QR code
```

## Gestion des erreurs

Encapsulez systématiquement dans try/catch :

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
    console.error('Error code:', error.code); // ex. "BASE45_DECODE", "SIGNATURE_INVALID"
  } else {
    throw error;
  }
}
```

## Décoder sans vérification (tests uniquement)

Pour le développement et les tests, vous pouvez ignorer la vérification de signature :

```typescript
import { Decoder } from 'claim169';

// WARNING: tests uniquement - jamais en production !
const result = new Decoder(qrText)
  .allowUnverified()
  .decode();

console.log('Status:', result.verificationStatus); // "skipped"
```

## Utiliser la fonction de convenance

Pour des cas simples, utilisez la fonction `decode()` :

```typescript
import { decode } from 'claim169';

// Avec vérification
const result = decode(qrText, {
  verifyWithEd25519: publicKey,
});

// Sans vérification (tests uniquement)
const result = decode(qrText, {
  allowUnverified: true,
});
```

## Travailler avec des clés hexadécimales

Si vos clés sont en hex, utilisez les utilitaires :

```typescript
import { Decoder, hexToBytes, bytesToHex } from 'claim169';

// Convertir hex vers bytes
const publicKey = hexToBytes("3d2a1b4c5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b");

const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Convertir bytes vers hex
const keyHex = bytesToHex(publicKey);
```

## Étapes suivantes

- [Guide de décodage](decoding.md) - Options complètes de décodage
- [Guide d’encodage](encoding.md) - Créer des identifiants avec tous les types de champs
- [Chiffrement](encryption.md) - Travailler avec des identifiants chiffrés
- [Crypto personnalisée](custom-crypto.md) - Intégration HSM et KMS cloud
- [Référence API](api.md) - Documentation complète
