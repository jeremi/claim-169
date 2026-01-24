# Configuration WASM

Le SDK TypeScript utilise WebAssembly (WASM) pour une cryptographie performante et un parsing CBOR efficace. La plupart des bundlers nécessitent une configuration pour gérer correctement les modules WASM.

## Vite

Vite requiert deux plugins pour le support WASM :

```bash
npm install -D vite-plugin-wasm vite-plugin-top-level-await
```

```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [wasm(), topLevelAwait()],
});
```

### Vite + React

```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [react(), wasm(), topLevelAwait()],
});
```

### Vite + Vue

```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [vue(), wasm(), topLevelAwait()],
});
```

## Webpack

Webpack 5 a un support intégré pour WebAssembly asynchrone :

```javascript
// webpack.config.js
module.exports = {
  experiments: {
    asyncWebAssembly: true,
  },
};
```

### Webpack avec Create React App

Si vous utilisez Create React App sans eject, utilisez `react-app-rewired` :

```bash
npm install -D react-app-rewired
```

```javascript
// config-overrides.js
module.exports = function override(config) {
  config.experiments = {
    ...config.experiments,
    asyncWebAssembly: true,
  };
  return config;
};
```

Mettre à jour `package.json` :

```json
{
  "scripts": {
    "start": "react-app-rewired start",
    "build": "react-app-rewired build",
    "test": "react-app-rewired test"
  }
}
```

## Next.js

### Next.js 13+ (App Router)

```javascript
// next.config.js
/** @type {import('next').NextConfig} */
const nextConfig = {
  webpack: (config) => {
    config.experiments = {
      ...config.experiments,
      asyncWebAssembly: true,
    };
    return config;
  },
};

module.exports = nextConfig;
```

### Import côté client uniquement

Comme WASM requiert des APIs navigateur, utilisez des imports dynamiques pour les composants client :

```typescript
// components/QRDecoder.tsx
'use client';

import { useEffect, useState } from 'react';

export function QRDecoder({ qrText }: { qrText: string }) {
  const [result, setResult] = useState<string | null>(null);

  useEffect(() => {
    async function decode() {
      const { Decoder } = await import('claim169');

      const decoded = new Decoder(qrText)
        .allowUnverified()
        .decode();

      setResult(decoded.claim169.fullName ?? 'Unknown');
    }
    decode();
  }, [qrText]);

  return <div>{result}</div>;
}
```

## Rollup

```bash
npm install -D @rollup/plugin-wasm
```

```javascript
// rollup.config.js
import wasm from '@rollup/plugin-wasm';

export default {
  input: 'src/index.js',
  output: {
    dir: 'dist',
    format: 'esm',
  },
  plugins: [wasm()],
};
```

## esbuild

esbuild n’a pas de support WASM natif. Utilisez un plugin :

```bash
npm install -D esbuild-plugin-wasm
```

```javascript
// build.js
const esbuild = require('esbuild');
const wasmPlugin = require('esbuild-plugin-wasm');

esbuild.build({
  entryPoints: ['src/index.ts'],
  bundle: true,
  outfile: 'dist/bundle.js',
  plugins: [wasmPlugin.default()],
});
```

## Parcel

Parcel 2 supporte WASM « out of the box ». Aucune configuration nécessaire.

```bash
parcel build src/index.html
```

## Node.js

Node.js 16+ supporte WASM nativement. Aucune configuration spéciale n’est nécessaire, mais assurez-vous d’utiliser des ES modules :

```json
// package.json
{
  "type": "module"
}
```

Ou utilisez l’extension `.mjs` pour vos fichiers.

## Cloudflare Workers

Cloudflare Workers supporte des modules WASM :

```javascript
// wrangler.toml
[build]
command = "npm run build"

[build.upload]
format = "modules"
main = "./dist/worker.js"
```

```typescript
// worker.ts
import { Decoder } from 'claim169';

export default {
  async fetch(request: Request): Promise<Response> {
    const url = new URL(request.url);
    const qrText = url.searchParams.get('qr') ?? '';

    try {
      const result = new Decoder(qrText)
        .allowUnverified()
        .decode();

      return new Response(JSON.stringify(result.claim169), {
        headers: { 'Content-Type': 'application/json' },
      });
    } catch (error) {
      return new Response('Decode failed', { status: 400 });
    }
  },
};
```

## AWS Lambda

Le runtime Node.js de AWS Lambda supporte WASM :

```typescript
// handler.ts
import { Decoder } from 'claim169';

export async function handler(event: any) {
  const qrText = event.queryStringParameters?.qr ?? '';

  try {
    const result = new Decoder(qrText)
      .allowUnverified()
      .decode();

    return {
      statusCode: 200,
      body: JSON.stringify(result.claim169),
    };
  } catch (error) {
    return {
      statusCode: 400,
      body: 'Decode failed',
    };
  }
}
```

Bundlez avec esbuild ou webpack avant déploiement.

## Navigateur (sans bundler)

Pour un usage direct navigateur sans bundler :

```html
<!DOCTYPE html>
<html>
<head>
  <title>Claim 169 Demo</title>
</head>
<body>
  <script type="module">
    // Import depuis un CDN ou un chemin local
    import { Decoder } from './node_modules/claim169/dist/index.js';

    const qrText = "6BF5YZB2...";
    const result = new Decoder(qrText)
      .allowUnverified()
      .decode();

    console.log('Name:', result.claim169.fullName);
  </script>
</body>
</html>
```

Note : servir localement peut nécessiter un serveur web à cause des restrictions CORS sur les URL file://.

## Dépannage des problèmes WASM

### "WebAssembly module is not defined"

Vérifiez que votre bundler est configuré pour WASM. Voir la section correspondante plus haut.

### "Cannot use import statement outside a module"

Ajoutez `"type": "module"` dans package.json ou utilisez l’extension `.mjs`.

### "SharedArrayBuffer is not defined"

Certains modules WASM exigent SharedArrayBuffer. Ajoutez ces en-têtes côté serveur :

```
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

### WASM ne se charge pas en production

Assurez-vous que le déploiement inclut le fichier `.wasm`. Vérifiez :
- La sortie de build inclut `claim169_wasm_bg.wasm`
- Le type MIME serveur pour `.wasm` est `application/wasm`
- Aucune CSP ne bloque l’exécution WASM

### Optimisation de taille

Le binaire WASM fait environ 300KB. Pour des bundles plus petits :

1. Activer la compression gzip/brotli sur le serveur
2. Utiliser du code splitting pour charger le SDK à la demande
3. Envisager le tree-shaking des exports inutilisés

## Vérifier le chargement WASM

```typescript
import { isLoaded, version } from 'claim169';

if (isLoaded()) {
  console.log('WASM loaded successfully, version:', version());
} else {
  console.error('WASM failed to load');
}
```

## Étapes suivantes

- [Installation](installation.md) - Guide d’installation complet
- [Démarrage rapide](quick-start.md) - Commencer rapidement
- [Dépannage](troubleshooting.md) - Problèmes courants
