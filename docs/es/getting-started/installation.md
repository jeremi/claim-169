# Instalación

Instala el SDK Claim 169 para tu lenguaje preferido.

## Rust

Añade a tu `Cargo.toml`:

```toml
[dependencies]
claim169-core = "0.1"
```

O usa cargo:

```bash
cargo add claim169-core
```

## Python

Instala desde PyPI:

```bash
pip install claim169
```

O usando uv:

```bash
uv add claim169
```

### Requisitos

- Python 3.8 o superior
- Plataformas soportadas: Linux (x86_64, aarch64), macOS (x86_64, arm64), Windows (x86_64)

## TypeScript / JavaScript

Instala desde npm:

```bash
npm install claim169
```

O usando yarn:

```bash
yarn add claim169
```

### Soporte de navegador

El SDK TypeScript usa WebAssembly y funciona en todos los navegadores modernos:

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

### Soporte Node.js

Se requiere Node.js 16 o superior para soporte de WebAssembly.

## Compilación desde el código fuente

### Requisitos previos

- Rust 1.70+ con cargo
- Python 3.8+ con maturin (para bindings Python)
- Node.js 18+ con npm (para SDK TypeScript)
- wasm-pack (para bindings WebAssembly)

### Clonar y compilar

```bash
# Clonar el repositorio
git clone https://github.com/jeremi/claim-169.git
cd claim-169

# Compilar bibliotecas Rust
cargo build --release

# Ejecutar tests
cargo test --all-features

# Compilar bindings Python
cd core/claim169-python
maturin develop --release

# Compilar WASM y SDK TypeScript
cd ../../sdks/typescript
npm install
npm run build
```

## Verificación de la instalación

=== "Rust"

    ```rust
    use claim169_core::Decoder;

    fn main() {
        println!("¡claim169-core instalado correctamente!");
    }
    ```

=== "Python"

    ```python
    import claim169
    print(f"Versión claim169: {claim169.__version__}")
    ```

=== "TypeScript"

    ```typescript
    import { Decoder } from 'claim169';
    console.log('¡claim169 instalado correctamente!');
    ```
