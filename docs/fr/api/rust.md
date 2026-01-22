# Référence API Rust

La documentation complète de l'API est disponible sur [docs.rs/claim169-core](https://docs.rs/claim169-core).

## Types principaux

### Decoder

Constructeur pour décoder les données QR.

```rust
use claim169_core::Decoder;

let decoder = Decoder::new(qr_data);
```

### Encoder

Constructeur pour encoder les identifiants.

```rust
use claim169_core::{Encoder, Claim169Input, CwtMetaInput};

let encoder = Encoder::new(claim, meta);
```

Pour la documentation complète, consultez la [référence API en anglais](../../en/api/rust.md).
