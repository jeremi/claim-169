# SDK TypeScript

[![npm](https://img.shields.io/npm/v/claim169.svg)](https://www.npmjs.com/package/claim169)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-blue.svg)](https://www.typescriptlang.org/)

Une bibliothèque TypeScript/JavaScript pour encoder et décoder des QR codes MOSIP Claim 169. Construite sur Rust/WebAssembly pour les performances et la sécurité.

## Pourquoi TypeScript ?

- **Sécurité de types** : définitions TypeScript complètes avec interfaces détaillées
- **Performances WebAssembly** : crypto en Rust compilée en WASM, proche du natif
- **Navigateur et Node.js** : fonctionne dans les navigateurs, Node.js et des environnements serverless
- **Sécurisé par défaut** : exige une vérification explicite de signature (ou un opt-out)
- **Pattern builder** : API fluide pour configurer encodage et décodage

## Support des plateformes

| Plateforme | Support |
|-----------|---------|
| Node.js 18+ | Support complet |
| Navigateurs modernes | Support complet (Chrome, Firefox, Safari, Edge) |
| React/Vue/Angular | Support complet avec configuration bundler |
| Deno | Expérimental |
| Cloudflare Workers | Support complet (WASM compatible) |
| AWS Lambda | Support complet (runtime Node.js) |

## Fonctionnalités

- **Décodage** : parser des QR codes MOSIP Claim 169 avec vérification de signature
- **Encodage** : créer des identifiants signés et optionnellement chiffrés
- **Vérification de signature** : support Ed25519 et ECDSA P-256
- **Chiffrement** : AES-128-GCM et AES-256-GCM pour identifiants chiffrés
- **Fournisseurs crypto personnalisés** : intégration HSM, KMS cloud et cartes à puce
- **Données biométriques** : parser empreintes, iris, visage, paume et voix
- **Énumérations typées** : constantes `Gender`, `MaritalStatus`, `PhotoFormat`, `BiometricFormat`
- **Codes d'erreur** : propriété programmatique `error.code` sur toutes les erreurs
- **Fonctions de sécurité** : protection bombes de décompression, validation d'horodatages

## Bien démarrer

```typescript
import { Decoder, Encoder } from 'claim169';

// Décoder avec vérification de signature
const result = new Decoder(qrText)
  .verifyWithEd25519(publicKey)
  .decode();

console.log(result.claim169.fullName);
console.log(result.verificationStatus); // "verified"

// Encoder un identifiant
const qrData = new Encoder(claim169, cwtMeta)
  .signWithEd25519(privateKey)
  .encode();
```

## Documentation

- [Installation](installation.md) - Installer et configurer le SDK
- [Démarrage rapide](quick-start.md) - Commencer en 5 minutes
- [Décodage](decoding.md) - Décoder des QR codes avec vérification
- [Encodage](encoding.md) - Créer des identifiants signés
- [Chiffrement](encryption.md) - Travailler avec des identifiants chiffrés
- [Crypto personnalisée](custom-crypto.md) - Intégrer HSM et KMS cloud
- [Configuration WASM](wasm.md) - Configurer les bundlers pour WebAssembly
- [Référence API](api.md) - Documentation complète
- [Dépannage](troubleshooting.md) - Problèmes courants et solutions

## Considérations de sécurité

Le SDK est conçu pour être sécurisé par défaut :

1. **Vérification requise** : vous devez appeler explicitement une méthode de vérification ou `allowUnverified()` avant `decode()`
2. **Pas de valeurs par défaut d’algorithme** : en-têtes d’algorithme COSE obligatoires (empêche la confusion d’algorithme)
3. **Limites de décompression** : limite par défaut 64KB contre les bombes de décompression
4. **Rejet de clés faibles** : clés Ed25519 et ECDSA invalides rejetées

## Licence

Licence MIT — voir [LICENSE](https://github.com/jeremi/claim-169/blob/main/LICENSE) pour les détails.
