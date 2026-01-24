# SDK Rust

[![Crates.io](https://img.shields.io/crates/v/claim169-core.svg)](https://crates.io/crates/claim169-core)
[![Documentation](https://docs.rs/claim169-core/badge.svg)](https://docs.rs/claim169-core)
[![License](https://img.shields.io/crates/l/claim169-core.svg)](https://github.com/mosip/id-claim-169)

Une bibliothèque Rust pour encoder et décoder des QR codes MOSIP Claim 169, conçue pour la vérification hors ligne d’identifiants d’identité numérique.

## Vue d’ensemble

Le crate `claim169-core` fournit une implémentation complète de la spécification [MOSIP Claim 169](https://github.com/mosip/id-claim-169). Il gère tout le pipeline d’encodage/décodage :

```text
Identity Data -> CBOR -> CWT -> COSE_Sign1 -> [COSE_Encrypt0] -> zlib -> Base45 -> QR Code
```

## Fonctionnalités

- **Encodage et décodage** — support complet pour créer et lire des QR codes Claim 169
- **Signatures numériques** — support des signatures Ed25519 et ECDSA P-256
- **Chiffrement** — couche optionnelle AES-128-GCM et AES-256-GCM
- **Pattern builder** — API fluide pour la configuration
- **Intégration HSM** — crypto basée sur des traits pour HSM et KMS cloud
- **Sécurité d’abord** — vérification de signature requise par défaut, protection contre bombes de décompression
- **Compatibilité ascendante** — conservation des champs inconnus pour extensions futures

## Algorithmes supportés

| Type | Algorithmes |
|------|------------|
| Signature | Ed25519 (EdDSA), ECDSA P-256 (ES256) |
| Chiffrement | AES-128-GCM (A128GCM), AES-256-GCM (A256GCM) |

## Documentation

- [Installation](./installation.md) — ajouter le crate au projet
- [Démarrage rapide](./quick-start.md) — exemples basiques encode/decode
- [Encodage](./encoding.md) — créer des QR codes avec `Encoder`
- [Décodage](./decoding.md) — lire des QR codes avec `Decoder`
- [Chiffrement](./encryption.md) — utiliser le chiffrement AES-GCM
- [Crypto personnalisée](./custom-crypto.md) — intégration HSM et KMS cloud
- [Référence API](./api.md) — documentation complète
- [Dépannage](./troubleshooting.md) — erreurs fréquentes et solutions

## Exemple rapide

```rust
use claim169_core::{Encoder, Decoder, Claim169, CwtMeta};

// Créer des données d’identité
let claim169 = Claim169::new()
    .with_id("ID-12345")
    .with_full_name("Jane Doe")
    .with_date_of_birth("19900115");

let cwt_meta = CwtMeta::new()
    .with_issuer("https://issuer.example.com")
    .with_expires_at(1800000000);

// Encoder en chaîne QR (signée Ed25519)
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&private_key)?
    .encode()?;

// Décoder et vérifier
let result = Decoder::new(&qr_data)
    .verify_with_ed25519(&public_key)?
    .decode()?;

println!("Name: {:?}", result.claim169.full_name);
println!("Issuer: {:?}", result.cwt_meta.issuer);
```

## Modèle de sécurité

La bibliothèque impose des valeurs sûres :

- **Signatures requises** — encoder sans signature exige un `allow_unsigned()` explicite
- **Vérification requise** — décoder sans vérification exige un `allow_unverified()` explicite
- **Limites de décompression** — protection contre les attaques type zip bomb (par défaut : 64KB)
- **Validation des horodatages** — les identifiants expirés / pas encore valides sont rejetés par défaut
- **Rejet de clés faibles** — les clés nulles et les points d’ordre faible sont rejetés

## Licence

Ce projet est distribué sous licence MIT.
