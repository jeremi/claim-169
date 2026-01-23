# Installation

Installez le SDK Claim 169 pour votre langage préféré.

## Rust

Ajoutez à votre `Cargo.toml` :

```toml
[dependencies]
claim169-core = "0.1.0-alpha"
```

Ou utilisez cargo :

```bash
cargo add claim169-core
```

### Fonctionnalités (features)

La crate Rust expose la feature optionnelle suivante :

| Feature | Description | Défaut |
|---------|-------------|--------|
| `software-crypto` | Implémentations logicielles Ed25519, ECDSA P-256 et AES-GCM | Oui |

Pour une intégration HSM/KMS (signature/vérification/déchiffrement personnalisés), désactivez les features par défaut :

```toml
[dependencies]
claim169-core = { version = "0.1.0-alpha", default-features = false }
```

## Python

Installez depuis PyPI :

```bash
pip install claim169
```

Ou avec uv :

```bash
uv add claim169
```

### Prérequis

- Python 3.8 ou supérieur
- Plateformes supportées : Linux (x86_64, aarch64), macOS (x86_64, arm64), Windows (x86_64)

## TypeScript / JavaScript

Installez depuis npm :

```bash
npm install claim169
```

Ou avec yarn :

```bash
yarn add claim169
```

### Support navigateur

Le SDK TypeScript utilise WebAssembly et fonctionne dans tous les navigateurs modernes :

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

### Support Node.js

Node.js 16 ou supérieur est requis pour WebAssembly (Node 18+ recommandé).

## Compilation depuis les sources

### Prérequis

- Rust 1.75+ avec cargo
- Python 3.8+ avec maturin (pour les bindings Python)
- Node.js 18+ avec npm (pour le SDK TypeScript)
- wasm-pack (pour les bindings WebAssembly)

### Cloner et compiler

```bash
# Cloner le dépôt
git clone https://github.com/jeremi/claim-169.git
cd claim-169

# Compiler les bibliothèques Rust
cargo build --release

# Exécuter les tests
cargo test --all-features

# Compiler les bindings Python
cd core/claim169-python
maturin develop --release

# Compiler WASM et le SDK TypeScript
cd ../../sdks/typescript
npm install
npm run build
```

## Vérification de l'installation

=== "Rust"

    ```rust
    use claim169_core::Decoder;

    fn main() {
        println!("claim169-core installé avec succès !");
    }
    ```

=== "Python"

    ```python
    import claim169
    print(f"Version claim169 : {claim169.version()}")
    ```

=== "TypeScript"

    ```typescript
    import { Decoder } from 'claim169';
    console.log('claim169 installé avec succès !');
    ```
