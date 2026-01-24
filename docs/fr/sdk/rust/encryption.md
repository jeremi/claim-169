# Chiffrement

La spécification Claim 169 supporte un chiffrement AES-GCM optionnel pour protéger le contenu des identifiants. Cela ajoute une couche COSE_Encrypt0 au-dessus de la structure COSE_Sign1 signée.

## Vue d’ensemble

Lorsque le chiffrement est utilisé, le flux est :

```text
Encoding: Claim169 -> CBOR -> CWT -> COSE_Sign1 -> COSE_Encrypt0 -> zlib -> Base45
Decoding: Base45 -> zlib -> COSE_Encrypt0 -> COSE_Sign1 -> CWT -> Claim169
```

L’identifiant est toujours **signé d’abord, puis chiffré** (sign-then-encrypt).

## Algorithmes supportés

| Algorithme | Taille de clé | Taille de nonce | Description |
|-----------|---------------|-----------------|-------------|
| A256GCM | 32 octets | 12 octets | AES-256-GCM (recommandé) |
| A128GCM | 16 octets | 12 octets | AES-128-GCM |

## Encoder avec chiffrement

### AES-256-GCM (recommandé)

```rust
use claim169_core::{Encoder, Claim169, CwtMeta};

let claim169 = Claim169::minimal("ID-001", "Jane Doe");
let cwt_meta = CwtMeta::new().with_issuer("https://issuer.example.com");

// Clé de chiffrement 32 octets
let aes_key: [u8; 32] = [/* your key bytes */];

let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&signing_key)?
    .encrypt_with_aes256(&aes_key)?  // Nonce aléatoire généré
    .encode()?;
```

### AES-128-GCM

```rust
// Clé de chiffrement 16 octets
let aes_key: [u8; 16] = [/* your key bytes */];

let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&signing_key)?
    .encrypt_with_aes128(&aes_key)?
    .encode()?;
```

### Nonce explicite (tests uniquement)

Pour obtenir une sortie déterministe en tests :

```rust
let aes_key: [u8; 32] = [/* your key */];
let nonce: [u8; 12] = [/* unique nonce */];

let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&signing_key)?
    .encrypt_with_aes256_nonce(&aes_key, &nonce)?
    .encode()?;
```

**Avertissement** : ne jamais réutiliser un nonce avec la même clé. La réutilisation de nonce casse totalement la sécurité d’AES-GCM.

### Générer des nonces aléatoires

La bibliothèque fournit un helper pour générer des nonces aléatoires sûrs :

```rust
use claim169_core::generate_random_nonce;

let nonce = generate_random_nonce();  // Returns [u8; 12]
```

## Décoder avec déchiffrement

### AES-256-GCM

```rust
use claim169_core::Decoder;

let aes_key: [u8; 32] = [/* your key */];

let result = Decoder::new(qr_content)
    .decrypt_with_aes256(&aes_key)?
    .verify_with_ed25519(&public_key)?
    .decode()?;
```

### AES-128-GCM

```rust
let aes_key: [u8; 16] = [/* your key */];

let result = Decoder::new(qr_content)
    .decrypt_with_aes128(&aes_key)?
    .verify_with_ed25519(&public_key)?
    .decode()?;
```

## Chiffrement/déchiffrement personnalisés

Pour une intégration HSM ou KMS cloud, implémentez les traits `Encryptor` et `Decryptor` :

### Trait Encryptor

```rust
use claim169_core::{Encryptor, CryptoResult};
use coset::iana;

struct MyHsmEncryptor {
    key_id: Vec<u8>,
    // ... détails de connexion HSM
}

impl Encryptor for MyHsmEncryptor {
    fn encrypt(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        plaintext: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        // Appeler le HSM pour chiffrer
        // Retourner le ciphertext avec tag d’authentification ajouté
        todo!()
    }
}
```

### Trait Decryptor

```rust
use claim169_core::{Decryptor, CryptoResult};
use coset::iana;

struct MyHsmDecryptor {
    key_id: Vec<u8>,
}

impl Decryptor for MyHsmDecryptor {
    fn decrypt(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        ciphertext: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        // Appeler le HSM pour déchiffrer
        // ciphertext inclut le tag d’authentification
        todo!()
    }
}
```

### Utiliser la crypto personnalisée

```rust
// Encodage
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&signing_key)?
    .encrypt_with(my_hsm_encryptor, iana::Algorithm::A256GCM)
    .encode()?;

// Décodage
let result = Decoder::new(qr_content)
    .decrypt_with(my_hsm_decryptor)
    .verify_with_ed25519(&public_key)?
    .decode()?;
```

Voir [Crypto personnalisée](./custom-crypto.md) pour des exemples HSM complets.

## Bonnes pratiques de gestion des clés

### Génération de clés

```rust
use rand::RngCore;

fn generate_aes256_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

fn generate_aes128_key() -> [u8; 16] {
    let mut key = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut key);
    key
}
```

### Stockage des clés

- Ne jamais hardcoder des clés de chiffrement dans le code
- Utiliser des variables d’environnement en développement
- Utiliser un secrets manager (AWS Secrets Manager, HashiCorp Vault) en production
- Envisager un HSM pour des applications à haute sécurité

### Rotation de clés

Lors d’une rotation :

1. Supporter plusieurs clés de déchiffrement pendant la transition
2. Re-chiffrer les identifiants existants avec les nouvelles clés
3. Déprécier les anciennes clés après la transition

## Gestion des erreurs

```rust
use claim169_core::{Decoder, Claim169Error};

match Decoder::new(qr_content)
    .decrypt_with_aes256(&aes_key)?
    .verify_with_ed25519(&public_key)?
    .decode()
{
    Ok(result) => {
        println!("Decrypted and verified: {:?}", result.claim169.full_name);
    }
    Err(Claim169Error::DecryptionFailed(msg)) => {
        eprintln!("Decryption failed: {}", msg);
        eprintln!("Possible causes:");
        eprintln!("  - Wrong encryption key");
        eprintln!("  - Corrupted ciphertext");
        eprintln!("  - Tampered data (authentication failed)");
    }
    Err(Claim169Error::Crypto(msg)) => {
        eprintln!("Crypto error: {}", msg);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Exemple complet

```rust
use claim169_core::{
    Encoder, Decoder, Claim169, CwtMeta,
    Ed25519Signer, VerificationStatus,
    generate_random_nonce,
};
use rand::RngCore;

fn encrypted_roundtrip() -> claim169_core::Result<()> {
    // Générer les clés
    let signer = Ed25519Signer::generate();
    let public_key = signer.public_key_bytes();

    let mut aes_key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut aes_key);

    // Créer l’identifiant
    let claim169 = Claim169::new()
        .with_id("SECURE-001")
        .with_full_name("Alice Secure")
        .with_email("alice@secure.example.com");

    let cwt_meta = CwtMeta::new()
        .with_issuer("https://secure.issuer.com")
        .with_expires_at(1893456000);

    // Encoder avec signature + chiffrement
    let qr_data = Encoder::new(claim169.clone(), cwt_meta)
        .sign_with(signer, coset::iana::Algorithm::EdDSA)
        .encrypt_with_aes256(&aes_key)?
        .encode()?;

    println!("Encrypted QR: {} chars", qr_data.len());

    // Décoder avec déchiffrement + vérification
    let result = Decoder::new(&qr_data)
        .decrypt_with_aes256(&aes_key)?
        .verify_with_ed25519(&public_key)?
        .decode()?;

    assert_eq!(result.verification_status, VerificationStatus::Verified);
    assert_eq!(result.claim169.id, claim169.id);
    assert_eq!(result.claim169.full_name, claim169.full_name);

    println!("Decrypted and verified successfully!");
    println!("Name: {:?}", result.claim169.full_name);

    Ok(())
}
```

## Considérations de sécurité

1. **Taille de clé** : utiliser AES-256-GCM pour une meilleure marge de sécurité
2. **Unicité des nonces** : ne jamais réutiliser de nonces avec la même clé
3. **Protection des clés** : stocker les clés de façon sûre, envisager un HSM en production
4. **Chiffrement authentifié** : AES-GCM fournit confidentialité et intégrité
5. **Canaux auxiliaires** : l’implémentation logicielle n’est pas durcie contre les attaques par timing ; utiliser un HSM pour des scénarios très sensibles
