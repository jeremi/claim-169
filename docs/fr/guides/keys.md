# Matériel de clés et formats

Cette page explique quel matériel de clé la bibliothèque attend (octets bruts vs PEM) et comment cela se mappe aux opérations MOSIP Claim 169.

## Quelles clés sont utilisées ?

- **Signature (authenticité)** : Ed25519 (COSE `EdDSA`) ou ECDSA P-256 (COSE `ES256`)
- **Chiffrement (confidentialité, optionnel)** : AES-GCM (COSE `A256GCM` ou `A128GCM`)

!!! warning "Gestion des clés en production"
    Les clés de signature et de chiffrement sont des secrets critiques. En production, conservez-les dans un HSM/KMS et utilisez les mécanismes « crypto personnalisée » (là où ils existent) plutôt que de charger des clés privées brutes en mémoire applicative.

## Formats de clés par algorithme

### Ed25519

- **Clé publique** : 32 octets
- **Clé privée** : 32 octets (seed)

Dans la crate Rust (feature `software-crypto`, activée par défaut), le décodeur supporte aussi les clés publiques **PEM/SPKI** :

```rust
use claim169_core::Decoder;

let result = Decoder::new(qr_text)
    .verify_with_ed25519_pem(ed25519_public_key_pem)?
    .decode()?;
```

### ECDSA P-256 (ES256)

- **Clé publique** : point SEC1 encodé, soit :
  - **33 octets** (compressé, commence par `0x02` ou `0x03`), ou
  - **65 octets** (non compressé, commence par `0x04`)
- **Clé privée** : scalaire de 32 octets

Rust supporte aussi les clés publiques **PEM/SPKI** :

```rust
use claim169_core::Decoder;

let result = Decoder::new(qr_text)
    .verify_with_ecdsa_p256_pem(p256_public_key_pem)?
    .decode()?;
```

### AES-GCM (A256GCM / A128GCM)

- **Clé AES-256-GCM** : 32 octets
- **Clé AES-128-GCM** : 16 octets
- **Nonce/IV** : 12 octets (aléatoire à chaque chiffrement)

En usage normal, vous n’avez **pas** besoin de fournir un nonce : l’encodeur génère un nonce aléatoire automatiquement.

!!! danger "La réutilisation de nonce casse la sécurité"
    Ne réutilisez jamais un nonce AES-GCM avec la même clé. N’utilisez des APIs « nonce explicite » que pour des tests.

## Générer des clés de développement (Rust)

Avec la feature `software-crypto` (par défaut), vous pouvez générer des clés temporaires pour vos tests locaux :

```rust
use claim169_core::{Ed25519Signer, EcdsaP256Signer};

let ed_signer = Ed25519Signer::generate();
let ed_public_key: [u8; 32] = ed_signer.public_key_bytes();

let p256_signer = EcdsaP256Signer::generate();
let p256_public_key_uncompressed: Vec<u8> = p256_signer.public_key_uncompressed(); // 65 octets
```

## Générer des clés AES (Python / TypeScript)

=== "Python"

    ```python
    import secrets

    aes256_key = secrets.token_bytes(32)
    aes128_key = secrets.token_bytes(16)
    ```

=== "TypeScript"

    ```ts
    // Navigateur
    const aes256Key = crypto.getRandomValues(new Uint8Array(32));

    // Node.js
    import { randomBytes } from "crypto";
    const aes256KeyNode = randomBytes(32);
    ```

## Vecteurs de test

Pour des exemples de clés connues (uniquement pour des tests), voir `test-vectors/valid/*.json`. Ces vecteurs incluent `public_key_hex` et (pour certains) `private_key_hex`.

