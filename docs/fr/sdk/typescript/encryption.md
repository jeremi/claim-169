# Chiffrement

MOSIP Claim 169 supporte le chiffrement AES-GCM pour protéger les charges utiles d’identifiants. Ce guide couvre le chiffrement et le déchiffrement.

## Vue d’ensemble

Le chiffrement dans Claim 169 se fait au niveau COSE :
1. Les données d’identité sont encodées en CBOR et enveloppées dans un CWT
2. Le CWT est signé avec COSE_Sign1
3. La charge utile signée est chiffrée avec COSE_Encrypt0 (AES-GCM)
4. Le résultat est compressé puis encodé en Base45

Au décodage, on inverse le processus : d’abord déchiffrer, puis vérifier la signature.

## AES-256-GCM

AES-256-GCM utilise une clé de 32 octets (256 bits) avec un nonce de 12 octets et produit un tag d’authentification de 16 octets.

### Encoder avec AES-256-GCM

```typescript
import { Encoder, type Claim169Input, type CwtMetaInput } from 'claim169';

const claim169: Claim169Input = {
  id: "SECURE-001",
  fullName: "Secure User",
  dateOfBirth: "1990-01-01",
};

const cwtMeta: CwtMetaInput = {
  issuer: "https://secure.example.com",
  expiresAt: Math.floor(Date.now() / 1000) + 86400 * 365,
};

// Clés
const signingKey = new Uint8Array(32);  // Clé privée Ed25519
const encryptionKey = new Uint8Array(32); // Clé AES-256

// Signer puis chiffrer
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(signingKey)
  .encryptWithAes256(encryptionKey)
  .encode();
```

### Décoder avec AES-256-GCM

```typescript
import { Decoder } from 'claim169';

const encryptionKey = new Uint8Array(32); // Même clé AES-256
const verificationKey = new Uint8Array(32); // Clé publique Ed25519

// Déchiffrer puis vérifier
const result = new Decoder(qrText)
  .decryptWithAes256(encryptionKey)
  .verifyWithEd25519(verificationKey)
  .decode();

console.log('Decrypted:', result.claim169.fullName);
console.log('Verified:', result.verificationStatus);
```

## AES-128-GCM

AES-128-GCM utilise une clé de 16 octets (128 bits). C’est plus rapide que AES-256 mais avec une marge de sécurité plus faible.

### Encoder avec AES-128-GCM

```typescript
const signingKey = new Uint8Array(32);  // Clé privée Ed25519
const encryptionKey = new Uint8Array(16); // Clé AES-128 (16 octets)

const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(signingKey)
  .encryptWithAes128(encryptionKey)
  .encode();
```

### Décoder avec AES-128-GCM

```typescript
const encryptionKey = new Uint8Array(16); // Clé AES-128
const verificationKey = new Uint8Array(32); // Clé publique Ed25519

const result = new Decoder(qrText)
  .decryptWithAes128(encryptionKey)
  .verifyWithEd25519(verificationKey)
  .decode();
```

## Générer des clés

### Générer une clé AES

```typescript
// Générer une clé AES-256 aléatoire
const aes256Key = crypto.getRandomValues(new Uint8Array(32));

// Générer une clé AES-128 aléatoire
const aes128Key = crypto.getRandomValues(new Uint8Array(16));
```

## Formats de clé supportés

Les clés AES peuvent être fournies sous différents formats :

### Format hexadécimal

```typescript
// AES-256 (64 caractères hex)
const keyHex = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const key = hexToBytes(keyHex); // 32 octets

// AES-128 (32 caractères hex)
const keyHex128 = "0123456789abcdef0123456789abcdef";
const key128 = hexToBytes(keyHex128); // 16 octets
```

### Format Base64

```typescript
// AES-256 (44 caractères Base64)
const keyB64 = "ASNFZ4mrze8BI0VniavN7wEjRWeJq83vASNFZ4mrze8=";
const key = base64ToBytes(keyB64); // 32 octets

// AES-128 (24 caractères Base64)
const keyB64_128 = "ASNFZ4mrze8BI0VniavN7w==";
const key128 = base64ToBytes(keyB64_128); // 16 octets
```

Le playground détecte automatiquement le format (hex ou Base64) lors de la saisie de clés de chiffrement.

### Générer un nonce

Le SDK gère la génération de nonces en interne, mais vous pouvez en générer pour des fournisseurs crypto personnalisés :

```typescript
import { generateNonce } from 'claim169';

const nonce = generateNonce(); // Renvoie Uint8Array(12)
console.log('Nonce length:', nonce.length); // 12
```

## Utiliser la fonction de convenance

```typescript
import { decode } from 'claim169';

// Déchiffrer et vérifier en un seul appel
const result = decode(qrText, {
  decryptWithAes256: encryptionKey,
  verifyWithEd25519: verificationKey,
});

// Ou avec AES-128
const result = decode(qrText, {
  decryptWithAes128: encryptionKey,
  verifyWithEd25519: verificationKey,
});
```

## Gestion des erreurs

### Erreurs de déchiffrement

```typescript
import { Decoder, Claim169Error } from 'claim169';

try {
  const result = new Decoder(qrText)
    .decryptWithAes256(encryptionKey)
    .verifyWithEd25519(verificationKey)
    .decode();
} catch (error) {
  if (error instanceof Claim169Error) {
    switch (error.code) {
      case 'DECRYPTION':
        console.error('Decryption failed - wrong key or corrupted data');
        break;
      case 'SIGNATURE_INVALID':
        console.error('Signature verification failed');
        break;
      default:
        console.error('Decode failed:', error.code, error.message);
    }
  }
}
```

### Erreurs de chiffrement fréquentes

| Erreur | Cause |
|-------|------|
| `AES-256 key must be 32 bytes` | Taille de clé AES-256 invalide |
| `AES-128 key must be 16 bytes` | Taille de clé AES-128 invalide |
| `Decryption failed` | Mauvaise clé ou ciphertext corrompu |
| `Authentication failed` | Vérification du tag échouée (données altérées) |

## Bonnes pratiques de gestion des clés

### Ne pas hardcoder les clés

```typescript
// BAD - ne jamais hardcoder des clés
const key = new Uint8Array([0x01, 0x02, ...]);

// GOOD - charger depuis l’environnement ou un système de gestion de clés
const key = await loadKeyFromKMS('encryption-key-id');
```

### Dérivation de clé

Pour dériver des clés de chiffrement à partir de mots de passe :

```typescript
// Utiliser Web Crypto API pour la dérivation
async function deriveKey(password: string, salt: Uint8Array): Promise<Uint8Array> {
  const encoder = new TextEncoder();
  const keyMaterial = await crypto.subtle.importKey(
    'raw',
    encoder.encode(password),
    'PBKDF2',
    false,
    ['deriveBits']
  );

  const derivedBits = await crypto.subtle.deriveBits(
    {
      name: 'PBKDF2',
      salt,
      iterations: 100000,
      hash: 'SHA-256',
    },
    keyMaterial,
    256 // 32 bytes
  );

  return new Uint8Array(derivedBits);
}
```

### Rotation régulière

Implémenter une rotation en supportant plusieurs clés de chiffrement :

```typescript
async function decodeWithKeyRotation(
  qrText: string,
  keys: { id: string; key: Uint8Array }[],
  verifyKey: Uint8Array
): Promise<DecodeResult> {
  for (const { id, key } of keys) {
    try {
      return new Decoder(qrText)
        .decryptWithAes256(key)
        .verifyWithEd25519(verifyKey)
        .decode();
    } catch (error) {
      // Essayer la clé suivante
      continue;
    }
  }
  throw new Error('Decryption failed with all available keys');
}
```

## Fournisseurs de chiffrement personnalisés

Pour une intégration HSM ou KMS cloud, voir le guide [Crypto personnalisée](custom-crypto.md) :

```typescript
import { Encoder, Decoder, type EncryptorCallback, type DecryptorCallback } from 'claim169';

// Chiffreur personnalisé (HSM/KMS)
const myEncryptor: EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => {
  return myKms.encrypt({ keyId, nonce, aad, plaintext, algorithm });
};

// Déchiffreur personnalisé (HSM/KMS)
const myDecryptor: DecryptorCallback = (algorithm, keyId, nonce, aad, ciphertext) => {
  return myKms.decrypt({ keyId, nonce, aad, ciphertext, algorithm });
};

// Utiliser côté encodeur
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(signingKey)
  .encryptWith(myEncryptor, "A256GCM")
  .encode();

// Utiliser côté décodeur
const result = new Decoder(qrText)
  .decryptWith(myDecryptor)
  .verifyWithEd25519(verifyKey)
  .decode();
```

## Choix d’algorithme

| Algorithme | Taille de clé | Niveau de sécurité | Performance |
|-----------|---------------|--------------------|-------------|
| AES-128-GCM | 16 octets | 128 bits | Plus rapide |
| AES-256-GCM | 32 octets | 256 bits | Recommandé |

Pour la plupart des applications, **AES-256-GCM** est recommandé car il offre une plus grande marge de sécurité.

## Étapes suivantes

- [Crypto personnalisée](custom-crypto.md) - Intégration HSM et KMS cloud
- [Référence API](api.md) - API de chiffrement complète
- [Dépannage](troubleshooting.md) - Problèmes de chiffrement courants
