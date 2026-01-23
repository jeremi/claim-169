# Material de claves y formatos

Esta página explica qué material de clave espera la biblioteca (bytes “raw” vs PEM) y cómo se relaciona con las operaciones de MOSIP Claim 169.

## ¿Qué claves se usan?

- **Firma (autenticidad)**: Ed25519 (COSE `EdDSA`) o ECDSA P-256 (COSE `ES256`)
- **Cifrado (privacidad, opcional)**: AES-GCM (COSE `A256GCM` o `A128GCM`)

!!! warning "Gestión de claves en producción"
    Las claves de firma y cifrado son secretos críticos. En producción, guárdalas en un HSM/KMS y usa los mecanismos de “crypto personalizado” (si están disponibles) en lugar de cargar claves privadas raw en memoria.

## Formatos por algoritmo

### Ed25519

- **Clave pública**: 32 bytes
- **Clave privada**: 32 bytes (seed)

En Rust (feature `software-crypto` por defecto), el decodificador también soporta claves públicas **PEM/SPKI**:

```rust
use claim169_core::Decoder;

let result = Decoder::new(qr_text)
    .verify_with_ed25519_pem(ed25519_public_key_pem)?
    .decode()?;
```

### ECDSA P-256 (ES256)

- **Clave pública**: punto SEC1, puede ser:
  - **33 bytes** (comprimida, empieza con `0x02` o `0x03`), o
  - **65 bytes** (no comprimida, empieza con `0x04`)
- **Clave privada**: escalar de 32 bytes

Rust también soporta claves públicas **PEM/SPKI**:

```rust
use claim169_core::Decoder;

let result = Decoder::new(qr_text)
    .verify_with_ecdsa_p256_pem(p256_public_key_pem)?
    .decode()?;
```

### AES-GCM (A256GCM / A128GCM)

- **Clave AES-256-GCM**: 32 bytes
- **Clave AES-128-GCM**: 16 bytes
- **Nonce/IV**: 12 bytes (aleatorio por cifrado)

En uso normal no necesitas proporcionar un nonce: el encoder genera un nonce aleatorio automáticamente.

!!! danger "Reutilizar nonce rompe la seguridad"
    Nunca reutilices un nonce AES-GCM con la misma clave. Solo usa APIs de nonce explícito para pruebas.

## Generar claves de desarrollo (Rust)

Con la feature `software-crypto` (por defecto), puedes generar claves temporales para pruebas:

```rust
use claim169_core::{Ed25519Signer, EcdsaP256Signer};

let ed_signer = Ed25519Signer::generate();
let ed_public_key: [u8; 32] = ed_signer.public_key_bytes();

let p256_signer = EcdsaP256Signer::generate();
let p256_public_key_uncompressed: Vec<u8> = p256_signer.public_key_uncompressed(); // 65 bytes
```

## Generar claves AES (Python / TypeScript)

=== "Python"

    ```python
    import secrets

    aes256_key = secrets.token_bytes(32)
    aes128_key = secrets.token_bytes(16)
    ```

=== "TypeScript"

    ```ts
    // Browser
    const aes256Key = crypto.getRandomValues(new Uint8Array(32));

    // Node.js
    import { randomBytes } from "crypto";
    const aes256KeyNode = randomBytes(32);
    ```

## Vectores de prueba

Para claves de ejemplo (solo para pruebas), ver `test-vectors/valid/*.json`. Estos vectores incluyen `public_key_hex` y (en algunos casos) `private_key_hex`.

