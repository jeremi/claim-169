# Installation

## Gestionnaires de paquets

### npm

```bash
npm install claim169
```

### yarn

```bash
yarn add claim169
```

### pnpm

```bash
pnpm add claim169
```

## Prérequis

- **Node.js** : 18.0 ou supérieur
- **Navigateur** : navigateur moderne avec support WebAssembly (Chrome 57+, Firefox 52+, Safari 11+, Edge 16+)

## ES Modules

Le SDK est distribué en tant que module ES. Assurez-vous que votre projet est configuré pour les ES modules :

```json
// package.json
{
  "type": "module"
}
```

Ou utilisez l’extension `.mjs` pour les fichiers de modules.

## Configuration TypeScript

Le SDK inclut les définitions TypeScript. Pour un typage optimal :

```json
// tsconfig.json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "strict": true,
    "esModuleInterop": true
  }
}
```

## Vérifier l’installation

```typescript
import { version, isLoaded } from 'claim169';

console.log('Version:', version());    // p. ex. "0.1.0-alpha.2"
console.log('WASM loaded:', isLoaded()); // true
```

## Configuration du bundler

Les modules WebAssembly nécessitent une configuration bundler. Voir le guide [Configuration WASM](wasm.md) pour :

- [Configuration Vite](wasm.md#vite)
- [Configuration Webpack](wasm.md#webpack)
- [Configuration esbuild](wasm.md#esbuild)
- [Configuration Rollup](wasm.md#rollup)
- [Configuration Next.js](wasm.md#nextjs)

## Construire depuis les sources

Si vous devez construire le SDK depuis les sources :

### Prérequis

1. **Toolchain Rust** : installer via [rustup](https://rustup.rs/)
2. **wasm-pack** : installer avec `cargo install wasm-pack`
3. **Node.js** : version 18 ou supérieure

### Étapes de build

```bash
# Cloner le dépôt
git clone https://github.com/jeremi/claim-169.git
cd claim-169/sdks/typescript

# Installer les dépendances
npm install

# Construire le module WASM (Rust requis)
npm run build:wasm

# Construire TypeScript
npm run build:ts

# Ou tout construire
npm run build
```

### Lancer les tests

```bash
# Générer d’abord les vecteurs de test (depuis la racine du dépôt)
cd ../..
cargo run -p generate-vectors

# Lancer les tests TypeScript
cd sdks/typescript
npm test
```

## Contenu du package

Le package publié inclut :

```
claim169/
  dist/
    index.js          # ESM bundle
    index.d.ts        # TypeScript definitions
    types.js          # Type definitions
    types.d.ts        # TypeScript type definitions
  wasm/
    claim169_wasm.js  # WASM JavaScript bindings
    claim169_wasm_bg.wasm  # WebAssembly binary
```

## Usage via CDN

Pour un usage navigateur sans bundler, vous pouvez utiliser un CDN :

```html
<script type="module">
  import { Decoder } from 'https://esm.sh/claim169';

  // Utiliser le SDK
  const result = new Decoder(qrText)
    .allowUnverified()
    .decode();
</script>
```

Note : l’usage via CDN peut nécessiter une configuration WASM additionnelle selon le fournisseur.
