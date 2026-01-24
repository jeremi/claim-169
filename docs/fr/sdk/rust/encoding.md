# Encodage

`Encoder` fournit une API fluide (builder) pour créer des QR codes Claim 169 à partir de données d’identité.

## Pipeline d’encodage

Le processus d’encodage transforme les données via ce pipeline :

```text
Claim169 -> CBOR -> CWT -> COSE_Sign1 -> [COSE_Encrypt0] -> zlib -> Base45
```

## Utilisation basique

```rust
use claim169_core::{Encoder, Claim169, CwtMeta};

let claim169 = Claim169::minimal("ID-001", "Jane Doe");
let cwt_meta = CwtMeta::new().with_issuer("https://issuer.example.com");

let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&private_key)?
    .encode()?;
```

## Construire les données d’identité

La struct `Claim169` utilise un pattern builder pour tous les champs :

### Données démographiques

```rust
use claim169_core::{Claim169, Gender, MaritalStatus, PhotoFormat};

let claim = Claim169::new()
    // Identité de base
    .with_id("ID-12345-ABCDE")
    .with_version("1.0")
    .with_language("eng")

    // Nom
    .with_full_name("Jane Marie Smith")
    .with_first_name("Jane")
    .with_middle_name("Marie")
    .with_last_name("Smith")

    // Détails personnels
    .with_date_of_birth("19900515")  // Format YYYYMMDD
    .with_gender(Gender::Female)
    .with_marital_status(MaritalStatus::Married)
    .with_nationality("USA")

    // Contact
    .with_address("123 Main St\nNew York, NY 10001")
    .with_email("jane.smith@example.com")
    .with_phone("+1 555 123 4567")  // Format E.123

    // Compléments
    .with_guardian("John Smith Sr.")
    .with_location_code("US-NY-NYC")
    .with_legal_status("citizen")
    .with_country_of_issuance("USA");
```

### Photo

```rust
use claim169_core::{Claim169, PhotoFormat};

// Lire un fichier photo
let photo_bytes = std::fs::read("photo.jpg")?;

let claim = Claim169::new()
    .with_id("ID-001")
    .with_full_name("Jane Doe")
    .with_photo(photo_bytes)
    .with_photo_format(PhotoFormat::Jpeg);
```

Formats photo supportés :
- `PhotoFormat::Jpeg` (1)
- `PhotoFormat::Jpeg2000` (2)
- `PhotoFormat::Avif` (3)
- `PhotoFormat::Webp` (4)

### Langue secondaire

```rust
let claim = Claim169::new()
    .with_full_name("Maria Garcia")
    .with_language("spa")
    .with_secondary_full_name("Maria Garcia")  // Dans la langue secondaire
    .with_secondary_language("eng");
```

### Best Quality Fingers

```rust
// Indiquer quels doigts ont la meilleure qualité biométrique
// Les valeurs 0-10 correspondent à des positions de doigts
let claim = Claim169::new()
    .with_best_quality_fingers(vec![1, 6, 2, 7]);  // Index droit, index gauche, etc.
```

### Constructeur minimal

Pour des tests rapides ou des identifiants simples :

```rust
let claim = Claim169::minimal("ID-001", "Jane Doe");
// Équivalent à :
// Claim169::new().with_id("ID-001").with_full_name("Jane Doe")
```

## Métadonnées CWT

```rust
use claim169_core::CwtMeta;

let cwt_meta = CwtMeta::new()
    .with_issuer("https://issuer.example.com")      // iss - émetteur
    .with_subject("user-id-12345")                   // sub - sujet
    .with_expires_at(1893456000)                     // exp - expiration
    .with_not_before(1704067200)                     // nbf - pas valide avant
    .with_issued_at(1704067200);                     // iat - émis à
```

## Signature

### Ed25519 (recommandé)

```rust
// Depuis des octets bruts
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&private_key)?  // Clé 32 octets
    .encode()?;
```

### ECDSA P-256

```rust
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ecdsa_p256(&private_key)?  // Scalaire 32 octets
    .encode()?;
```

### Signataire personnalisé

Pour une intégration HSM ou KMS :

```rust
use claim169_core::{Encoder, Signer};
use coset::iana;

struct MyHsmSigner { /* ... */ }
impl Signer for MyHsmSigner { /* ... */ }

let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with(my_hsm_signer, iana::Algorithm::EdDSA)
    .encode()?;
```

Voir [Crypto personnalisée](./custom-crypto.md) pour les détails d’implémentation du trait `Signer`.

## Chiffrement

Ajouter une couche de chiffrement au-dessus de la signature :

```rust
// AES-256-GCM (recommandé)
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&sign_key)?
    .encrypt_with_aes256(&aes_key)?  // Clé 32 octets
    .encode()?;

// AES-128-GCM
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&sign_key)?
    .encrypt_with_aes128(&aes_key)?  // Clé 16 octets
    .encode()?;
```

Les nonces sont générés aléatoirement par défaut. Pour une sortie déterministe (tests uniquement) :

```rust
let nonce: [u8; 12] = [/* 12 random bytes */];

let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&sign_key)?
    .encrypt_with_aes256_nonce(&aes_key, &nonce)?
    .encode()?;
```

**Avertissement** : réutiliser des nonces avec la même clé est une vulnérabilité critique.

Voir [Chiffrement](./encryption.md) pour plus de détails.

## Exclure la biométrie

Pour réduire la taille du QR code, exclure les données biométriques :

```rust
let qr_data = Encoder::new(claim169, cwt_meta)
    .skip_biometrics()  // Exclure empreintes, iris, visage, etc.
    .sign_with_ed25519(&private_key)?
    .encode()?;
```

Ou créer une copie sans biométrie :

```rust
let claim_without_bio = claim169.without_biometrics();
```

## Encodage sans signature (tests uniquement)

```rust
let qr_data = Encoder::new(claim169, cwt_meta)
    .allow_unsigned()  // Requis pour encoder sans signature
    .encode()?;
```

**Avertissement** : ne jamais utiliser des identifiants non signés en production.

## Ordre des opérations

L’encodeur exécute toujours les opérations dans cet ordre, quel que soit l’ordre d’appel des méthodes :

1. Encoder Claim169 en CBOR
2. Envelopper dans un CWT avec métadonnées
3. Signer via COSE_Sign1 (si un signataire est fourni)
4. Chiffrer via COSE_Encrypt0 (si un chiffreur est fourni)
5. Compresser avec zlib
6. Encoder en Base45

Cela signifie que `sign_with()` et `encrypt_with()` peuvent être appelées dans n’importe quel ordre :

```rust
// Produisent une sortie identique :
Encoder::new(claim, meta)
    .sign_with_ed25519(&key)?
    .encrypt_with_aes256(&aes_key)?
    .encode()?;

Encoder::new(claim, meta)
    .encrypt_with_aes256(&aes_key)?
    .sign_with_ed25519(&key)?
    .encode()?;
```

## Gestion des erreurs

La méthode `encode()` renvoie `Result<String, Claim169Error>` :

```rust
use claim169_core::Claim169Error;

match Encoder::new(claim169, cwt_meta).sign_with_ed25519(&key)?.encode() {
    Ok(qr_data) => println!("Encoded: {}", qr_data),
    Err(Claim169Error::EncodingConfig(msg)) => {
        eprintln!("Configuration error: {}", msg);
    }
    Err(Claim169Error::SignatureFailed(msg)) => {
        eprintln!("Signing failed: {}", msg);
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
    Encoder, Claim169, CwtMeta, Gender, MaritalStatus,
    PhotoFormat, Ed25519Signer,
};
use std::time::{SystemTime, UNIX_EPOCH};

fn create_full_credential() -> claim169_core::Result<String> {
    // Générer une clé de signature
    let signer = Ed25519Signer::generate();

    // Temps courant
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Construire une identité complète
    let claim169 = Claim169::new()
        .with_id("NATIONAL-ID-2024-000123")
        .with_version("2.0")
        .with_language("eng")
        .with_full_name("Alexandra Maria Johnson")
        .with_first_name("Alexandra")
        .with_middle_name("Maria")
        .with_last_name("Johnson")
        .with_date_of_birth("19880312")
        .with_gender(Gender::Female)
        .with_nationality("USA")
        .with_marital_status(MaritalStatus::Married)
        .with_address("456 Oak Avenue\nApt 7B\nSan Francisco, CA 94102")
        .with_email("alex.johnson@example.com")
        .with_phone("+1 415 555 0123")
        .with_country_of_issuance("USA")
        .with_location_code("US-CA-SF")
        .with_legal_status("citizen");

    // Métadonnées CWT valides 1 an
    let cwt_meta = CwtMeta::new()
        .with_issuer("https://id.dmv.ca.gov")
        .with_subject("johnson.alexandra.1988")
        .with_issued_at(now)
        .with_not_before(now)
        .with_expires_at(now + 365 * 24 * 60 * 60);  // 1 year

    // Encoder avec signature
    let qr_data = Encoder::new(claim169, cwt_meta)
        .sign_with(signer, coset::iana::Algorithm::EdDSA)
        .encode()?;

    Ok(qr_data)
}
```
