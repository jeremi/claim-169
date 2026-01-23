# Instalación

Instala el SDK Claim 169 para tu lenguaje preferido.

## Rust

Añade a tu `Cargo.toml`:

```toml
[dependencies]
claim169-core = "0.1.0-alpha"
```

O usa cargo:

```bash
cargo add claim169-core
```

### Features

El crate Rust expone la siguiente feature opcional:

| Feature | Descripción | Por defecto |
|---------|-------------|------------|
| `software-crypto` | Implementaciones en software de Ed25519, ECDSA P-256 y AES-GCM | Sí |

Para integrar con HSM/KMS (firma/verificación/descifrado personalizados), desactiva las features por defecto:

```toml
[dependencies]
claim169-core = { version = "0.1.0-alpha", default-features = false }
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

Se requiere Node.js 16 o superior para WebAssembly (Node 18+ recomendado).

## Compilación desde el código fuente

### Requisitos previos

- Rust 1.75+ con cargo
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
    print(f"Versión claim169: {claim169.version()}")
    ```

=== "TypeScript"

    ```typescript
    import { Decoder } from 'claim169';
    console.log('¡claim169 instalado correctamente!');
    ```
