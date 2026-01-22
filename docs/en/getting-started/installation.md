# Installation

Install the Claim 169 SDK for your preferred language.

## Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
claim169-core = "0.1"
```

Or use cargo:

```bash
cargo add claim169-core
```

### Features

The Rust crate supports these optional features:

| Feature | Description | Default |
|---------|-------------|---------|
| `std` | Standard library support | Yes |
| `alloc` | Allocation support (no_std) | Yes |

## Python

Install from PyPI:

```bash
pip install claim169
```

Or using uv:

```bash
uv add claim169
```

### Requirements

- Python 3.8 or later
- Supported platforms: Linux (x86_64, aarch64), macOS (x86_64, arm64), Windows (x86_64)

## TypeScript / JavaScript

Install from npm:

```bash
npm install claim169
```

Or using yarn:

```bash
yarn add claim169
```

Or using pnpm:

```bash
pnpm add claim169
```

### Browser Support

The TypeScript SDK uses WebAssembly and works in all modern browsers:

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

### Node.js Support

Node.js 16 or later is required for WebAssembly support.

## Building from Source

### Prerequisites

- Rust 1.70+ with cargo
- Python 3.8+ with maturin (for Python bindings)
- Node.js 18+ with npm (for TypeScript SDK)
- wasm-pack (for WebAssembly bindings)

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/jeremi/claim-169.git
cd claim-169

# Build Rust libraries
cargo build --release

# Run tests
cargo test --all-features

# Build Python bindings
cd core/claim169-python
maturin develop --release

# Build WASM and TypeScript SDK
cd ../../sdks/typescript
npm install
npm run build
```

## Verifying Installation

=== "Rust"

    ```rust
    use claim169_core::Decoder;

    fn main() {
        println!("claim169-core installed successfully!");
    }
    ```

=== "Python"

    ```python
    import claim169
    print(f"claim169 version: {claim169.__version__}")
    ```

=== "TypeScript"

    ```typescript
    import { Decoder } from 'claim169';
    console.log('claim169 installed successfully!');
    ```
