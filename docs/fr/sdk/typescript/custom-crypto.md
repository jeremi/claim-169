# Fournisseurs crypto personnalisés

Le SDK TypeScript supporte des fournisseurs cryptographiques personnalisés pour s’intégrer avec des Hardware Security Modules (HSM), cartes à puce, TPM et autres fournisseurs externes.

## Vue d’ensemble

Au lieu de fournir des clés brutes, vous pouvez fournir des fonctions de callback que le SDK appellera lors des opérations cryptographiques. Cela permet de garder les clés privées dans un périmètre sécurisé.

!!! important "Les callbacks sont synchrones"
    Tous les callbacks crypto personnalisés **doivent être synchrones** et retourner leur résultat directement.
    N’utilisez pas de fonctions `async` et ne retournez pas de Promises — WebAssembly ne peut pas `await` pendant encode/decode.

## Types de callbacks

### SignerCallback

Appelé lors de la signature :

```typescript
type SignerCallback = (
  algorithm: string,      // "EdDSA" ou "ES256"
  keyId: Uint8Array | null, // Identifiant de clé depuis l’en-tête COSE
  data: Uint8Array        // Données à signer (COSE Sig_structure)
) => Uint8Array;          // Octets de signature
```

### VerifierCallback

Appelé lors de la vérification :

```typescript
type VerifierCallback = (
  algorithm: string,      // "EdDSA" ou "ES256"
  keyId: Uint8Array | null, // Identifiant de clé depuis l’en-tête COSE
  data: Uint8Array,       // Données signées (COSE Sig_structure)
  signature: Uint8Array   // Signature à vérifier
) => void;                // Lève une exception en cas d’échec, sinon ne retourne rien
```

### EncryptorCallback

Appelé lors du chiffrement :

```typescript
type EncryptorCallback = (
  algorithm: string,      // "A256GCM" ou "A128GCM"
  keyId: Uint8Array | null, // Identifiant de clé
  nonce: Uint8Array,      // Nonce/IV 12 octets
  aad: Uint8Array,        // Additional authenticated data
  plaintext: Uint8Array   // Données à chiffrer
) => Uint8Array;          // Ciphertext + tag d’authentification
```

### DecryptorCallback

Appelé lors du déchiffrement :

```typescript
type DecryptorCallback = (
  algorithm: string,      // "A256GCM" ou "A128GCM"
  keyId: Uint8Array | null, // Identifiant de clé
  nonce: Uint8Array,      // Nonce/IV 12 octets
  aad: Uint8Array,        // Additional authenticated data
  ciphertext: Uint8Array  // Ciphertext + tag d’authentification
) => Uint8Array;          // Texte clair déchiffré
```

## Signataire personnalisé

### Exemple basique

```typescript
import { Encoder, type SignerCallback, type Claim169Input, type CwtMetaInput } from 'claim169';

const mySigner: SignerCallback = (algorithm, keyId, data) => {
  console.log('Signing with algorithm:', algorithm); // "EdDSA" ou "ES256"
  console.log('Key ID:', keyId); // null ou octets d’identifiant de clé

  // Appeler votre fournisseur crypto
  const signature = myHsm.sign({
    algorithm,
    keyId,
    data,
  });

  return signature; // Doit faire 64 octets pour EdDSA ou ES256
};

const claim: Claim169Input = { id: "HSM-001", fullName: "HSM Signed User" };
const meta: CwtMetaInput = { issuer: "https://hsm.example" };

const qrData = new Encoder(claim, meta)
  .signWith(mySigner, "EdDSA")  // Spécifier l’algorithme
  .encode();
```

### Cloud KMS (architecture recommandée)

La plupart des SDK cloud KMS sont **async** et ne peuvent pas être appelés directement depuis ces callbacks.

Pour une signature/chiffrement adossé à un KMS cloud, utilisez l’un de ces patterns :

1. **Faire l’encodage/décodage côté serveur** (Rust ou Python) où vous pouvez intégrer KMS/HSM et renvoyer la charge utile QR finale au client.
2. **Utiliser un agent de signature local** exposant une API synchrone (p. ex. une librairie HSM native ou un daemon local avec un pont sync), puis l’appeler depuis le callback.

## Vérificateur personnalisé

### Exemple basique

```typescript
import { Decoder, type VerifierCallback } from 'claim169';

const myVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
  console.log('Verifying with algorithm:', algorithm);

  const result = myHsm.verify({
    algorithm,
    keyId,
    data,
    signature,
  });

  if (!result.valid) {
    throw new Error('Signature verification failed');
  }
  // Ne rien retourner en cas de succès
};

const result = new Decoder(qrText)
  .verifyWith(myVerifier)
  .decode();

console.log('Verified:', result.verificationStatus); // "verified"
```

!!! note "Vérification vs confiance"
    La vérification personnalisée peut valider des signatures, mais il vous faut quand même un modèle de confiance applicatif (identification d’émetteur, distribution/rotation de clés, décisions de politique).

## Chiffreur personnalisé

### Exemple basique

```typescript
import { Encoder, type EncryptorCallback } from 'claim169';

const myEncryptor: EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => {
  console.log('Encrypting with algorithm:', algorithm); // "A256GCM" ou "A128GCM"

  const ciphertext = myHsm.encrypt({
    algorithm,
    keyId,
    nonce,
    aad,
    plaintext,
  });

  return ciphertext; // Doit inclure le tag 16 octets à la fin
};

const qrData = new Encoder(claim, meta)
  .signWithEd25519(signingKey)
  .encryptWith(myEncryptor, "A256GCM")
  .encode();
```

## Déchiffreur personnalisé

### Exemple basique

```typescript
import { Decoder, type DecryptorCallback } from 'claim169';

const myDecryptor: DecryptorCallback = (algorithm, keyId, nonce, aad, ciphertext) => {
  console.log('Decrypting with algorithm:', algorithm);

  const plaintext = myHsm.decrypt({
    algorithm,
    keyId,
    nonce,
    aad,
    ciphertext, // Inclut le tag
  });

  return plaintext;
};

const result = new Decoder(qrText)
  .decryptWith(myDecryptor)
  .verifyWithEd25519(publicKey)
  .decode();
```

## Signer + chiffrer (callbacks combinés)

```typescript
import { Encoder, type SignerCallback, type EncryptorCallback } from 'claim169';

// Les deux opérations utilisent le même HSM
const hsmSigner: SignerCallback = (algorithm, keyId, data) => {
  return myHsm.sign({ operation: 'sign', algorithm, keyId, data });
};

const hsmEncryptor: EncryptorCallback = (algorithm, keyId, nonce, aad, plaintext) => {
  return myHsm.encrypt({ operation: 'encrypt', algorithm, keyId, nonce, aad, plaintext });
};

const qrData = new Encoder(claim, meta)
  .signWith(hsmSigner, "EdDSA")
  .encryptWith(hsmEncryptor, "A256GCM")
  .encode();
```

## Vérifier + déchiffrer (callbacks combinés)

```typescript
import { Decoder, type VerifierCallback, type DecryptorCallback } from 'claim169';

const hsmVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
  const valid = myHsm.verify({ algorithm, keyId, data, signature });
  if (!valid) throw new Error('Verification failed');
};

const hsmDecryptor: DecryptorCallback = (algorithm, keyId, nonce, aad, ciphertext) => {
  return myHsm.decrypt({ algorithm, keyId, nonce, aad, ciphertext });
};

const result = new Decoder(qrText)
  .decryptWith(hsmDecryptor)
  .verifyWith(hsmVerifier)
  .decode();
```

## Roundtrip avec fournisseurs personnalisés

```typescript
import { Encoder, Decoder, type SignerCallback, type VerifierCallback } from 'claim169';

// Simuler un stockage HSM
const hsmKeys = new Map<string, Uint8Array>();

// Signer personnalisé utilisant le "HSM"
const mySigner: SignerCallback = (algorithm, keyId, data) => {
  // Signature déterministe pour la démo (un vrai HSM utiliserait une vraie crypto)
  const sig = new Uint8Array(64);
  for (let i = 0; i < 64; i++) {
    sig[i] = (data[i % data.length] + (keyId?.[i % keyId.length] ?? 0)) % 256;
  }
  return sig;
};

// Vérificateur personnalisé qui correspond à notre signer
const myVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
  // Recalculer la signature attendue
  const expected = new Uint8Array(64);
  for (let i = 0; i < 64; i++) {
    expected[i] = (data[i % data.length] + (keyId?.[i % keyId.length] ?? 0)) % 256;
  }

  // Comparer
  for (let i = 0; i < 64; i++) {
    if (signature[i] !== expected[i]) {
      throw new Error('Signature mismatch');
    }
  }
};

// Encode
const claim = { id: "ROUNDTRIP-001", fullName: "Roundtrip User" };
const meta = { issuer: "https://roundtrip.example" };

const qrData = new Encoder(claim, meta)
  .signWith(mySigner, "EdDSA")
  .encode();

// Decode
const result = new Decoder(qrData)
  .verifyWith(myVerifier)
  .decode();

console.log('ID:', result.claim169.id); // "ROUNDTRIP-001"
console.log('Verified:', result.verificationStatus); // "verified"
```

## Gestion des erreurs

Les erreurs levées dans les callbacks sont propagées :

```typescript
const failingSigner: SignerCallback = (algorithm, keyId, data) => {
  throw new Error('HSM connection timeout');
};

try {
  const qrData = new Encoder(claim, meta)
    .signWith(failingSigner, "EdDSA")
    .encode();
} catch (error) {
  console.error('Signing failed:', error.message);
  // "HSM connection timeout" ou erreur encapsulée
}
```

## Algorithmes supportés

### Algorithmes de signature

| Algorithme | Description | Taille de signature |
|-----------|-------------|---------------------|
| `EdDSA` | Ed25519 | 64 octets |
| `ES256` | ECDSA P-256 avec SHA-256 | 64 octets (r \|\| s) |

### Algorithmes de chiffrement

| Algorithme | Description | Taille de clé |
|-----------|-------------|--------------|
| `A256GCM` | AES-256-GCM | 32 octets |
| `A128GCM` | AES-128-GCM | 16 octets |

## Bonnes pratiques

1. **Ne jamais logger de données sensibles** : ne loggez pas de clés privées ni de signatures complètes en production
2. **Garder les callbacks synchrones** : pas de `async`/Promises ; si votre fournisseur est async, déplacer la crypto côté backend ou via un pont sync
3. **Gérer les timeouts** : opérations HSM/KMS susceptibles de timeouts ; implémenter des retries
4. **Audit logging** : journaliser l’usage des clés (key ID, opération, timestamp)
5. **Rotation de clés** : supporter plusieurs clés pour une rotation fluide

## Étapes suivantes

- [Chiffrement](encryption.md) - Exemples standard
- [Référence API](api.md) - Définitions complètes des types de callbacks
- [Dépannage](troubleshooting.md) - Problèmes fréquents de crypto personnalisée

