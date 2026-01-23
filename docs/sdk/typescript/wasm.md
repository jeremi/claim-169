# WASM Configuration

The TypeScript SDK uses WebAssembly (WASM) for high-performance cryptography and CBOR parsing. Most bundlers require configuration to properly handle WASM modules.

## Vite

Vite requires two plugins for WASM support:

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

### Vite with React

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

### Vite with Vue

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

Webpack 5 has built-in support for async WebAssembly:

```javascript
// webpack.config.js
module.exports = {
  experiments: {
    asyncWebAssembly: true,
  },
};
```

### Webpack with Create React App

If using Create React App without ejecting, use `react-app-rewired`:

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

Update `package.json`:

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

### Client-Side Only Import

Since WASM requires browser APIs, use dynamic imports for client components:

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

esbuild doesn't have built-in WASM support. Use a plugin:

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

Parcel 2 supports WASM out of the box. No configuration needed.

```bash
parcel build src/index.html
```

## Node.js

Node.js 16+ supports WASM natively. No special configuration is needed, but ensure you're using ES modules:

```json
// package.json
{
  "type": "module"
}
```

Or use the `.mjs` extension for your files.

## Cloudflare Workers

Cloudflare Workers support WASM modules:

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

AWS Lambda Node.js runtime supports WASM:

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

Bundle with esbuild or webpack before deployment.

## Browser (No Bundler)

For direct browser usage without a bundler:

```html
<!DOCTYPE html>
<html>
<head>
  <title>Claim 169 Demo</title>
</head>
<body>
  <script type="module">
    // Import from CDN or local path
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

Note: Serving locally may require a web server due to CORS restrictions on file:// URLs.

## Troubleshooting WASM Issues

### "WebAssembly module is not defined"

Ensure your bundler is configured for WASM. See the configuration for your specific bundler above.

### "Cannot use import statement outside a module"

Add `"type": "module"` to package.json or use `.mjs` extension.

### "SharedArrayBuffer is not defined"

Some WASM modules require SharedArrayBuffer. Add these headers to your server:

```
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

### WASM fails to load in production

Ensure your deployment includes the `.wasm` file. Check:
- Build output includes `claim169_wasm_bg.wasm`
- Server MIME type for `.wasm` is `application/wasm`
- No CSP blocking WASM execution

### Size Optimization

The WASM binary is approximately 300KB. For smaller bundles:

1. Enable gzip/brotli compression on your server
2. Use code splitting to lazy-load the SDK
3. Consider tree-shaking unused exports

## Verifying WASM Loading

```typescript
import { isLoaded, version } from 'claim169';

if (isLoaded()) {
  console.log('WASM loaded successfully, version:', version());
} else {
  console.error('WASM failed to load');
}
```

## Next Steps

- [Installation](installation.md) - Full installation guide
- [Quick Start](quick-start.md) - Get started quickly
- [Troubleshooting](troubleshooting.md) - Common issues
