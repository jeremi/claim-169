# Installation

## Package Managers

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

## Requirements

- **Node.js**: 18.0 or higher
- **Browser**: Modern browser with WebAssembly support (Chrome 57+, Firefox 52+, Safari 11+, Edge 16+)

## ES Modules

The SDK is distributed as an ES module. Ensure your project is configured for ES modules:

```json
// package.json
{
  "type": "module"
}
```

Or use the `.mjs` extension for module files.

## TypeScript Configuration

The SDK includes TypeScript definitions. For optimal type checking:

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

## Verifying Installation

```typescript
import { version, isLoaded } from 'claim169';

console.log('Version:', version());    // e.g., "0.1.0-alpha.2"
console.log('WASM loaded:', isLoaded()); // true
```

## Bundler Configuration

WebAssembly modules require bundler configuration. See the [WASM Configuration](wasm.md) guide for:

- [Vite configuration](wasm.md#vite)
- [Webpack configuration](wasm.md#webpack)
- [esbuild configuration](wasm.md#esbuild)
- [Rollup configuration](wasm.md#rollup)
- [Next.js configuration](wasm.md#nextjs)

## Building from Source

If you need to build the SDK from source:

### Prerequisites

1. **Rust toolchain**: Install via [rustup](https://rustup.rs/)
2. **wasm-pack**: Install with `cargo install wasm-pack`
3. **Node.js**: Version 18 or higher

### Build Steps

```bash
# Clone the repository
git clone https://github.com/jeremi/claim-169.git
cd claim-169/sdks/typescript

# Install dependencies
npm install

# Build WASM module (requires Rust)
npm run build:wasm

# Build TypeScript
npm run build:ts

# Or build everything
npm run build
```

### Running Tests

```bash
# Generate test vectors first (from repo root)
cd ../..
cargo run -p generate-vectors

# Run TypeScript tests
cd sdks/typescript
npm test
```

## Package Contents

The published package includes:

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

## CDN Usage

For browser usage without a bundler, you can use a CDN:

```html
<script type="module">
  import { Decoder } from 'https://esm.sh/claim169';

  // Use the SDK
  const result = new Decoder(qrText)
    .allowUnverified()
    .decode();
</script>
```

Note: CDN usage may require additional WASM configuration depending on the CDN provider.
