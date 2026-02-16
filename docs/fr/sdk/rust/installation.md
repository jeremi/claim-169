# Installation

## Ajouter à votre projet

Ajoutez `claim169-core` à votre `Cargo.toml` :

```toml
[dependencies]
claim169-core = "0.2.0-alpha"
```

Ou utilisez cargo add :

```bash
cargo add claim169-core
```

## Feature flags

Le crate expose un feature flag :

| Feature | Par défaut | Description |
|---------|------------|-------------|
| `software-crypto` | Oui | Implémentations logicielles de Ed25519, ECDSA P-256 et AES-GCM |

### Configuration par défaut

Par défaut, le feature `software-crypto` est activé, ce qui fournit des implémentations crypto prêtes à l’emploi :

```toml
[dependencies]
claim169-core = "0.2.0-alpha"
```

Cela inclut :
- `Ed25519Signer` / `Ed25519Verifier` — signature et vérification Ed25519
- `EcdsaP256Signer` / `EcdsaP256Verifier` — signature et vérification ECDSA P-256
- `AesGcmEncryptor` / `AesGcmDecryptor` — chiffrement et déchiffrement AES-128/256-GCM

### Intégration HSM/KMS (sans features par défaut)

Pour une intégration HSM ou KMS cloud, désactivez les features par défaut :

```toml
[dependencies]
claim169-core = { version = "0.2.0-alpha", default-features = false }
```

Cela supprime les dépendances crypto logicielles et vous oblige à implémenter les traits cryptographiques :

- [`Signer`](./custom-crypto.md#signer-trait) — pour signer des identifiants
- [`SignatureVerifier`](./custom-crypto.md#signatureverifier-trait) — pour vérifier des signatures
- [`Encryptor`](./custom-crypto.md#encryptor-trait) — pour chiffrer des identifiants
- [`Decryptor`](./custom-crypto.md#decryptor-trait) — pour déchiffrer des identifiants

Voir [Crypto personnalisée](./custom-crypto.md) pour les détails d’implémentation.

## Version minimale de Rust

Le crate supporte Rust 1.75 et plus.

## Dépendances

Lorsque `software-crypto` est activé, le crate dépend de :

- `ed25519-dalek` — signatures Ed25519
- `p256` — signatures ECDSA P-256
- `aes-gcm` — chiffrement AES-GCM
- `rand` — génération aléatoire
- `zeroize` — effacement sécurisé en mémoire

Dépendances cœur (toujours incluses) :

- `coset` — parsing COSE
- `ciborium` — encodage/décodage CBOR
- `base45` — encodage Base45
- `flate2` — compression zlib
- `serde` / `serde_json` — sérialisation
- `thiserror` — gestion d’erreurs

## Vérifier l’installation

Créez un petit test pour vérifier l’installation :

```rust
use claim169_core::{Claim169, CwtMeta, Encoder, Decoder};

fn main() -> claim169_core::Result<()> {
    // Créer une identité minimale
    let claim169 = Claim169::minimal("test-id", "Test User");
    let cwt_meta = CwtMeta::new().with_issuer("test");

    // Encoder (sans signature pour tests)
    let qr_data = Encoder::new(claim169.clone(), cwt_meta)
        .allow_unsigned()
        .encode()?;

    // Décoder
    let result = Decoder::new(&qr_data)
        .allow_unverified()
        .decode()?;

    assert_eq!(result.claim169.id, claim169.id);
    println!("Installation verified successfully!");

    Ok(())
}
```

Exécuter avec :

```bash
cargo run
```
