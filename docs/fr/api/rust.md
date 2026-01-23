# Référence API Rust

La documentation API complète est disponible sur [docs.rs/claim169-core](https://docs.rs/claim169-core).

## Features

Par défaut, la crate active le support crypto logiciel :

- `software-crypto` (par défaut) : helpers Ed25519, ECDSA P-256 et AES-GCM

Désactivez les features par défaut si vous souhaitez brancher votre propre crypto (HSM/KMS) via les traits `Signer` / `SignatureVerifier` / `Encryptor` / `Decryptor`.

## Types principaux

### `Decoder`

Builder pour décoder des QR Claim 169 (Base45).

```rust
use claim169_core::Decoder;

let decoder = Decoder::new(qr_text);
```

Méthodes (résumé) :

- `verify_with_ed25519(public_key)` / `verify_with_ed25519_pem(pem)` (avec `software-crypto`)
- `verify_with_ecdsa_p256(public_key)` / `verify_with_ecdsa_p256_pem(pem)` (avec `software-crypto`)
- `verify_with(verifier)` (vérification personnalisée HSM/KMS)
- `decrypt_with_aes256(key)` / `decrypt_with_aes128(key)` (avec `software-crypto`)
- `decrypt_with(decryptor)` (déchiffrement personnalisé)
- `allow_unverified()` (tests uniquement)
- `skip_biometrics()`
- `without_timestamp_validation()`
- `clock_skew_tolerance(seconds)`
- `max_decompressed_bytes(bytes)`
- `decode()`

### `Encoder`

Builder pour encoder des identifiants.

```rust
use claim169_core::{Claim169, CwtMeta, Encoder};

let encoder = Encoder::new(claim169, cwt_meta);
```

Méthodes (résumé) :

- `sign_with_ed25519(private_key)` / `sign_with_ecdsa_p256(private_key)` (avec `software-crypto`)
- `sign_with(signer, algorithm)` (signature personnalisée HSM/KMS)
- `allow_unsigned()` (tests uniquement)
- `encrypt_with_aes256(key)` / `encrypt_with_aes128(key)` (avec `software-crypto`)
- `encrypt_with(encryptor, algorithm)` (chiffrement personnalisé)
- `encrypt_with_aes256_nonce(key, nonce)` / `encrypt_with_aes128_nonce(key, nonce)` (tests uniquement)
- `skip_biometrics()`
- `encode()`

### `DecodeResult`

Le résultat de décodage contient :

- `claim169` (données d’identité)
- `cwt_meta` (issuer, exp/nbf/iat…)
- `verification_status` (`verified` / `skipped` / `failed`)
- `warnings` (non bloquants)

## Erreurs

Les opérations renvoient `claim169_core::Result<T>` (alias `Result<T, Claim169Error>`).

Cas fréquents :

- `DecodingConfig(...)` (ni vérification ni `allow_unverified()`)
- `EncodingConfig(...)` (ni signature ni `allow_unsigned()`)
- `SignatureInvalid(...)`, `DecryptionFailed(...)`
- `Expired(ts)` / `NotYetValid(ts)`
- `DecompressLimitExceeded { max_bytes }`

