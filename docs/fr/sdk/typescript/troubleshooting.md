# Dépannage

Problèmes courants et solutions pour le SDK TypeScript.

## Problèmes d’installation

### "Cannot find module 'claim169'"

**Cause** : package non installé ou problème de résolution de module.

**Solutions** :
1. Installer le package : `npm install claim169`
2. Vérifier que `"type": "module"` est présent dans package.json
3. Vérifier TypeScript `moduleResolution` : utiliser `"bundler"` ou `"node16"`

### "WebAssembly module is not defined"

**Cause** : bundler non configuré pour WASM.

**Solutions** :
- **Vite** : installer et ajouter `vite-plugin-wasm` et `vite-plugin-top-level-await`
- **Webpack** : ajouter `experiments: { asyncWebAssembly: true }`
- **Next.js** : configurer webpack dans `next.config.js`

Voir le guide [Configuration WASM](wasm.md) pour les détails.

### Erreurs de compilation TypeScript

**Cause** : incompatibilité de configuration TypeScript.

**Solution** : mettre à jour tsconfig.json :

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "esModuleInterop": true
  }
}
```

## Erreurs de décodage

### "Must call verifyWith...() or allowUnverified()"

**Cause** : aucune méthode de vérification n’a été spécifiée avant `decode()`.

**Solution** : ajouter une méthode de vérification :

```typescript
// Avec vérification (recommandé)
new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

// Sans vérification (tests uniquement)
new Decoder(qrText)
  .allowUnverified()
  .decode();
```

### "Ed25519 public key must be 32 bytes"

**Cause** : la clé publique a une mauvaise taille.

**Solution** : s’assurer que la clé fait exactement 32 octets :

```typescript
const publicKey = new Uint8Array(32);
console.log('Key length:', publicKey.length); // Doit être 32
```

### "ECDSA P-256 public key must be 33 bytes (compressed) or 65 bytes (uncompressed)"

**Cause** : la clé ECDSA a une mauvaise taille ou un mauvais format.

**Solutions** :
- Utiliser le format compressé (33 octets, commence par 0x02 ou 0x03)
- Utiliser le format non compressé (65 octets, commence par 0x04)

### "AES-256 key must be 32 bytes" / "AES-128 key must be 16 bytes"

**Cause** : la clé de chiffrement a une mauvaise taille.

**Solution** : vérifier que la taille correspond à l’algorithme :
- AES-256 : 32 octets
- AES-128 : 16 octets

### Erreur "Base45Decode"

**Cause** : encodage Base45 invalide dans les données QR.

**Solutions** :
1. Vérifier que le QR code a été scanné correctement
2. Vérifier que les données ne sont pas tronquées
3. Vérifier que la source produit bien des données Claim 169 valides

### Erreur "Decompress"

**Cause** : échec de décompression zlib.

**Solutions** :
1. Les données peuvent être corrompues
2. Les données peuvent dépasser la limite de décompression

Augmenter la limite si nécessaire :
```typescript
new Decoder(qrText)
  .maxDecompressedBytes(131072) // 128KB
  .allowUnverified()
  .decode();
```

### Erreur "CoseParse"

**Cause** : structure COSE invalide.

**Solutions** :
1. Vérifier que les données sont un identifiant MOSIP Claim 169 valide
2. Vérifier que chiffrement/déchiffrement est appliqué correctement

### Erreur "Claim169NotFound"

**Cause** : le CWT ne contient pas le claim 169.

**Solutions** :
1. Vérifier que l’identifiant est au format MOSIP Claim 169
2. Vérifier que l’ID de claim correct (169) est présent

### "SignatureError" ou échec de vérification

**Cause** : la vérification de signature a échoué.

**Solutions** :
1. Vérifier que vous utilisez la bonne clé publique
2. Vérifier que la clé correspond à l’algorithme (Ed25519 vs ECDSA)
3. Vérifier que les données n’ont pas été altérées

### "DecryptionError"

**Cause** : le déchiffrement a échoué.

**Solutions** :
1. Vérifier que la clé de chiffrement est correcte
2. Vérifier que l’algorithme correspond (AES-256 vs AES-128)
3. Vérifier que les données ne sont pas corrompues

## Erreurs d’encodage

### "Must call signWith...() or allowUnsigned()"

**Cause** : aucune méthode de signature n’a été spécifiée avant `encode()`.

**Solution** : ajouter une méthode de signature :

```typescript
// Avec signature (recommandé)
new Encoder(claim, meta)
  .signWithEd25519(privateKey)
  .encode();

// Sans signature (tests uniquement)
new Encoder(claim, meta)
  .allowUnsigned()
  .encode();
```

### "Ed25519 private key must be 32 bytes"

**Cause** : la clé privée a une mauvaise taille.

**Solution** : s’assurer que la clé fait exactement 32 octets.

### "Invalid claim169" ou "Invalid cwtMeta"

**Cause** : les données d’entrée ne correspondent pas au schéma attendu.

**Solution** : utiliser les types TypeScript :

```typescript
import type { Claim169Input, CwtMetaInput } from 'claim169';

const claim: Claim169Input = {
  id: "123",
  fullName: "Test",
};

const meta: CwtMetaInput = {
  issuer: "https://example.com",
};
```

## Problèmes de crypto personnalisée

### Les erreurs de callback ne se propagent pas

**Cause** : les exceptions peuvent être encapsulées.

**Solution** : inspecter le message d’erreur :

```typescript
try {
  new Encoder(claim, meta)
    .signWith(failingSigner, "EdDSA")
    .encode();
} catch (error) {
  if (error instanceof Claim169Error) {
    // Le message original peut être inclus
    console.error('Full error:', error.message);
  }
}
```

### La vérification échoue avec un verifier personnalisé

**Cause** : la logique de vérification ne correspond pas au signataire.

**Solutions** :
1. Vérifier que le verifier calcule la même signature que le signer
2. Logger l’algorithme et les données pour debug :

```typescript
const debugVerifier: VerifierCallback = (algorithm, keyId, data, signature) => {
  console.log('Algorithm:', algorithm);
  console.log('Data length:', data.length);
  console.log('Signature length:', signature.length);
  // Votre logique de vérification
};
```

### Les callbacks async ne fonctionnent pas

**Cause** : les callbacks sont synchrones ; les fonctions async retournent des promesses.

**Solution** : les callbacks doivent être synchrones. Pour des opérations async, encapsuler l’encode/decode complet :

```typescript
async function encodeWithAsyncSigning() {
  // Pré-charger la signature depuis le KMS
  const signature = await fetchSignatureFromKMS(data);

  // Utiliser un callback synchrone avec donnée pré-calculée
  const signer: SignerCallback = (alg, keyId, data) => {
    return signature; // Pré-calculée
  };

  return new Encoder(claim, meta)
    .signWith(signer, "EdDSA")
    .encode();
}
```

## Problèmes navigateur

### Erreurs CORS

**Cause** : le fetch WASM est bloqué par CORS.

**Solutions** :
1. Servir le WASM depuis la même origine
2. Ajouter des en-têtes CORS corrects côté serveur
3. Utiliser un bundler qui inline le WASM

### "SharedArrayBuffer is not defined"

**Cause** : certaines fonctionnalités WASM requièrent l’isolation cross-origin.

**Solution** : ajouter des en-têtes serveur :
```
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

### WASM ne se charge pas en production

**Cause** : fichier WASM absent du build ou mauvais type MIME.

**Solutions** :
1. Vérifier que la sortie de build inclut le fichier `.wasm`
2. Configurer le type MIME serveur : `application/wasm`
3. Vérifier que la CSP autorise l’exécution WASM

## Problèmes de performance

### Chargement initial lent

**Cause** : surcoût de chargement du module WASM.

**Solutions** :
1. Charger le SDK à la demande :
   ```typescript
   const { Decoder } = await import('claim169');
   ```
2. Activer la compilation streaming WASM
3. Mettre en cache le module compilé

### Décodage lent avec biométrie

**Cause** : grosse biométrie.

**Solution** : ignorer la biométrie si inutile :
```typescript
new Decoder(qrText)
  .skipBiometrics()
  .allowUnverified()
  .decode();
```

## Problèmes Node.js

### "Cannot use import statement outside a module"

**Cause** : mismatch CommonJS/ESM.

**Solutions** :
1. Ajouter `"type": "module"` dans package.json
2. Utiliser l’extension `.mjs`
3. Utiliser un import dynamique : `const { Decoder } = await import('claim169');`

### Module introuvable dans les tests

**Cause** : runner de test non configuré pour ESM.

**Solution** pour Vitest :
```typescript
// vitest.config.ts
import { defineConfig } from 'vitest/config';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [wasm(), topLevelAwait()],
});
```

## Conseils de debug

### Activer des logs verbeux

```typescript
import { Decoder, Claim169Error } from 'claim169';

try {
  const result = new Decoder(qrText)
    .verifyWithEd25519(publicKey)
    .decode();

  console.log('Decode result:', JSON.stringify(result, null, 2));
} catch (error) {
  if (error instanceof Claim169Error) {
    console.error('Claim169Error:', error.message);
    console.error('Error code:', error.code); // ex. "BASE45_DECODE", "SIGNATURE_INVALID"
    console.error('Stack:', error.stack);
  } else {
    console.error('Unknown error:', error);
  }
}
```

### Vérifier le chargement WASM

```typescript
import { isLoaded, version } from 'claim169';

console.log('WASM loaded:', isLoaded());
console.log('Version:', version());
```

### Valider les formats de clé

```typescript
function validateKey(key: Uint8Array, expectedLength: number, name: string) {
  console.log(`${name} length: ${key.length} (expected ${expectedLength})`);
  console.log(`${name} first bytes: ${Array.from(key.slice(0, 4)).map(b => b.toString(16).padStart(2, '0')).join(' ')}`);

  if (key.length !== expectedLength) {
    throw new Error(`Invalid ${name} length`);
  }
}

validateKey(publicKey, 32, 'Ed25519 public key');
validateKey(aesKey, 32, 'AES-256 key');
```

## Obtenir de l’aide

Si vous avez toujours des problèmes :

1. Consulter les [GitHub Issues](https://github.com/jeremi/claim-169/issues) pour les problèmes connus
2. Chercher une issue existante avant d’en créer une nouvelle
3. Inclure dans l’issue :
   - Version du SDK (`version()`)
   - Version Node.js/navigateur
   - Bundler + version
   - Code minimal de reproduction
   - Message d’erreur complet + stack trace
