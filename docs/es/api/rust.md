# Referencia API Rust

La documentación completa de la API está disponible en [docs.rs/claim169-core](https://docs.rs/claim169-core).

## Features

Por defecto, el crate activa crypto en software:

- `software-crypto` (por defecto): helpers para Ed25519, ECDSA P-256 y AES-GCM

Desactiva las features por defecto si quieres integrar tu propia crypto (HSM/KMS) vía los traits `Signer` / `SignatureVerifier` / `Encryptor` / `Decryptor`.

## Tipos principales

### `Decoder`

Builder para decodificar QR Claim 169 (Base45).

```rust
use claim169_core::Decoder;

let decoder = Decoder::new(qr_text);
```

Métodos (resumen):

- `verify_with_ed25519(public_key)` / `verify_with_ed25519_pem(pem)` (con `software-crypto`)
- `verify_with_ecdsa_p256(public_key)` / `verify_with_ecdsa_p256_pem(pem)` (con `software-crypto`)
- `verify_with(verifier)` (verificación personalizada HSM/KMS)
- `decrypt_with_aes256(key)` / `decrypt_with_aes128(key)` (con `software-crypto`)
- `decrypt_with(decryptor)` (descifrado personalizado)
- `allow_unverified()` (solo pruebas)
- `skip_biometrics()`
- `without_timestamp_validation()`
- `clock_skew_tolerance(seconds)`
- `max_decompressed_bytes(bytes)`
- `decode()`

### `Encoder`

Builder para codificar credenciales.

```rust
use claim169_core::{Claim169, CwtMeta, Encoder};

let encoder = Encoder::new(claim169, cwt_meta);
```

Métodos (resumen):

- `sign_with_ed25519(private_key)` / `sign_with_ecdsa_p256(private_key)` (con `software-crypto`)
- `sign_with(signer, algorithm)` (firma personalizada HSM/KMS)
- `allow_unsigned()` (solo pruebas)
- `encrypt_with_aes256(key)` / `encrypt_with_aes128(key)` (con `software-crypto`)
- `encrypt_with(encryptor, algorithm)` (cifrado personalizado)
- `encrypt_with_aes256_nonce(key, nonce)` / `encrypt_with_aes128_nonce(key, nonce)` (solo pruebas)
- `skip_biometrics()`
- `encode()`

### `DecodeResult`

El resultado de decodificación incluye:

- `claim169` (datos de identidad)
- `cwt_meta` (issuer, exp/nbf/iat…)
- `verification_status` (`verified` / `skipped` / `failed`)
- `warnings` (no fatales)

## Errores

Las operaciones retornan `claim169_core::Result<T>` (alias `Result<T, Claim169Error>`).

Casos comunes:

- `DecodingConfig(...)` (ni verificación ni `allow_unverified()`)
- `EncodingConfig(...)` (ni firma ni `allow_unsigned()`)
- `SignatureInvalid(...)`, `DecryptionFailed(...)`
- `Expired(ts)` / `NotYetValid(ts)`
- `DecompressLimitExceeded { max_bytes }`

