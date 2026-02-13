# Décodage

`Decoder` fournit une API fluide (builder) pour lire et vérifier des QR codes Claim 169.

## Pipeline de décodage

Le décodage inverse le pipeline d’encodage :

```text
Base45 -> zlib -> COSE_Encrypt0 (si chiffré) -> COSE_Sign1 -> CWT -> Claim169
```

## Utilisation basique

```rust
use claim169_core::Decoder;

let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .decode()?;

println!("Name: {:?}", result.claim169.full_name);
println!("Issuer: {:?}", result.cwt_meta.issuer);
```

## Structure DecodeResult

La méthode `decode()` renvoie un `DecodeResult` :

```rust
pub struct DecodeResult {
    /// The extracted Claim 169 identity data
    pub claim169: Claim169,

    /// CWT metadata (issuer, expiration, etc.)
    pub cwt_meta: CwtMeta,

    /// Signature verification status
    pub verification_status: VerificationStatus,

    /// Compression format detected during decoding
    pub detected_compression: DetectedCompression,

    /// Non-fatal warnings
    pub warnings: Vec<Warning>,
}
```

### Accéder aux données d’identité

```rust
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .decode()?;

// Démographie
if let Some(id) = &result.claim169.id {
    println!("ID: {}", id);
}
if let Some(name) = &result.claim169.full_name {
    println!("Name: {}", name);
}
if let Some(dob) = &result.claim169.date_of_birth {
    println!("DOB: {}", dob);
}
if let Some(gender) = &result.claim169.gender {
    println!("Gender: {:?}", gender);
}

// Contact
if let Some(email) = &result.claim169.email {
    println!("Email: {}", email);
}
if let Some(phone) = &result.claim169.phone {
    println!("Phone: {}", phone);
}

// Vérifier la présence de biométrie
if result.claim169.has_biometrics() {
    println!("Contains {} biometric entries", result.claim169.biometric_count());
}
```

### Accéder aux métadonnées CWT

```rust
if let Some(issuer) = &result.cwt_meta.issuer {
    println!("Issued by: {}", issuer);
}
if let Some(subject) = &result.cwt_meta.subject {
    println!("Subject: {}", subject);
}
if let Some(exp) = result.cwt_meta.expires_at {
    println!("Expires at: {}", exp);
}
if let Some(iat) = result.cwt_meta.issued_at {
    println!("Issued at: {}", iat);
}

// Vérifier la validité à un instant donné
let current_time = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_secs() as i64;

if result.cwt_meta.is_time_valid(current_time) {
    println!("Credential is currently valid");
}
if result.cwt_meta.is_expired(current_time) {
    println!("Credential has expired");
}
```

### Vérifier le statut de vérification

```rust
use claim169_core::VerificationStatus;

match result.verification_status {
    VerificationStatus::Verified => {
        println!("Signature verified successfully");
    }
    VerificationStatus::Skipped => {
        println!("Verification was skipped (allow_unverified was called)");
    }
    VerificationStatus::Failed => {
        println!("Signature verification failed");
    }
}
```

### Gérer les avertissements

```rust
use claim169_core::WarningCode;

for warning in &result.warnings {
    match warning.code {
        WarningCode::ExpiringSoon => {
            println!("Credential expiring soon: {}", warning.message);
        }
        WarningCode::UnknownFields => {
            println!("Unknown fields found: {}", warning.message);
        }
        WarningCode::TimestampValidationSkipped => {
            println!("Timestamps not validated: {}", warning.message);
        }
        WarningCode::BiometricsSkipped => {
            println!("Biometrics skipped: {}", warning.message);
        }
        WarningCode::NonStandardCompression => {
            println!("Non-standard compression: {}", warning.message);
        }
    }
}
```

## Vérification de signature

### Ed25519 (recommandé)

```rust
// Depuis des octets bruts (32 octets)
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .decode()?;

// Depuis un PEM
let pem = r#"-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEA...
-----END PUBLIC KEY-----"#;

let result = Decoder::new(qr_content)
    .verify_with_ed25519_pem(pem)?
    .decode()?;
```

### ECDSA P-256

```rust
// Depuis des octets SEC1 (33 ou 65 octets)
let result = Decoder::new(qr_content)
    .verify_with_ecdsa_p256(&public_key)?
    .decode()?;

// Depuis un PEM
let result = Decoder::new(qr_content)
    .verify_with_ecdsa_p256_pem(pem)?
    .decode()?;
```

### Vérificateur personnalisé

Pour une intégration HSM ou KMS :

```rust
use claim169_core::{Decoder, SignatureVerifier};

struct MyHsmVerifier { /* ... */ }
impl SignatureVerifier for MyHsmVerifier { /* ... */ }

let result = Decoder::new(qr_content)
    .verify_with(my_hsm_verifier)
    .decode()?;
```

Voir [Crypto personnalisée](./custom-crypto.md) pour les détails.

## Déchiffrement

Pour des identifiants chiffrés, fournissez la clé de déchiffrement :

```rust
// AES-256-GCM
let result = Decoder::new(qr_content)
    .decrypt_with_aes256(&aes_key)?  // Clé 32 octets
    .verify_with_ed25519(&public_key)?
    .decode()?;

// AES-128-GCM
let result = Decoder::new(qr_content)
    .decrypt_with_aes128(&aes_key)?  // Clé 16 octets
    .verify_with_ed25519(&public_key)?
    .decode()?;
```

### Déchiffreur personnalisé

```rust
use claim169_core::{Decoder, Decryptor};

struct MyHsmDecryptor { /* ... */ }
impl Decryptor for MyHsmDecryptor { /* ... */ }

let result = Decoder::new(qr_content)
    .decrypt_with(my_hsm_decryptor)
    .verify_with_ed25519(&public_key)?
    .decode()?;
```

## Options du décodeur

### Ignorer la vérification (tests uniquement)

```rust
let result = Decoder::new(qr_content)
    .allow_unverified()  // Requis pour décoder sans vérification
    .decode()?;
```

**Avertissement** : ne jamais ignorer la vérification en production.

### Ignorer la biométrie

Pour accélérer le décodage lorsque la biométrie n’est pas nécessaire :

```rust
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .skip_biometrics()  // Ne pas parser la biométrie
    .decode()?;
```

### Désactiver la validation des horodatages

Pour des scénarios hors ligne ou des tests :

```rust
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .without_timestamp_validation()  // Ne pas vérifier exp/nbf
    .decode()?;
```

### Tolérance à la dérive d’horloge

Autoriser des écarts d’horloge entre systèmes :

```rust
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .clock_skew_tolerance(300)  // Tolérance 5 minutes
    .decode()?;
```

### Limite de décompression

Ajuster la taille maximale après décompression (par défaut : 64KB) :

```rust
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .max_decompressed_bytes(131072)  // 128KB
    .decode()?;
```

Cela protège contre les attaques type zip bomb.

### Compression stricte

Par défaut, le décodeur détecte automatiquement et accepte tout format de compression (zlib, brotli, ou aucun). Pour imposer la conformité à la spécification et rejeter les données non-zlib :

```rust
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .strict_compression()  // Rejeter la compression non-zlib
    .decode()?;
```

Cela est utile pour les validateurs qui doivent imposer la conformité à la spécification. Sans `strict_compression()`, les identifiants non-zlib se décodent normalement mais produisent un avertissement `NonStandardCompression`.

### Vérifier la compression détectée

Après décodage, vous pouvez inspecter quel format de compression a été utilisé :

```rust
use claim169_core::DetectedCompression;

let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .decode()?;

match result.detected_compression {
    DetectedCompression::Zlib => println!("Standard zlib compression"),
    DetectedCompression::None => println!("No compression"),
    #[cfg(feature = "compression-brotli")]
    DetectedCompression::Brotli => println!("Brotli compression"),
}
```

## Ordre des opérations

Le décodeur traite toujours dans cet ordre, quel que soit l’ordre d’appel des méthodes :

1. Décoder Base45
2. Décompresser (détecte automatiquement zlib, brotli, ou brut)
3. Déchiffrer COSE_Encrypt0 (si un déchiffreur est fourni)
4. Vérifier COSE_Sign1 (si un vérificateur est fourni)
5. Parser le CWT
6. Valider les horodatages (sauf si désactivé)
7. Transformer Claim 169

## Gestion des erreurs

```rust
use claim169_core::{Decoder, Claim169Error};

match Decoder::new(qr_content).verify_with_ed25519(&key)?.decode() {
    Ok(result) => {
        println!("Decoded: {:?}", result.claim169.full_name);
    }

    // Erreurs d’encodage/format
    Err(Claim169Error::Base45Decode(msg)) => {
        eprintln!("Invalid QR format: {}", msg);
    }
    Err(Claim169Error::Decompress(msg)) => {
        eprintln!("Decompression failed: {}", msg);
    }
    Err(Claim169Error::DecompressLimitExceeded { max_bytes }) => {
        eprintln!("Data too large (max {} bytes)", max_bytes);
    }

    // Erreurs de structure
    Err(Claim169Error::CoseParse(msg)) => {
        eprintln!("Invalid COSE structure: {}", msg);
    }
    Err(Claim169Error::CwtParse(msg)) => {
        eprintln!("Invalid CWT: {}", msg);
    }
    Err(Claim169Error::Claim169NotFound) => {
        eprintln!("Claim 169 not found in payload");
    }

    // Erreurs de sécurité
    Err(Claim169Error::SignatureInvalid(msg)) => {
        eprintln!("Signature verification failed: {}", msg);
    }
    Err(Claim169Error::DecryptionFailed(msg)) => {
        eprintln!("Decryption failed: {}", msg);
    }

    // Erreurs d’horodatage
    Err(Claim169Error::Expired(ts)) => {
        eprintln!("Credential expired at {}", ts);
    }
    Err(Claim169Error::NotYetValid(ts)) => {
        eprintln!("Credential not valid until {}", ts);
    }

    // Erreurs de configuration
    Err(Claim169Error::DecodingConfig(msg)) => {
        eprintln!("Configuration error: {}", msg);
    }

    // Autres erreurs
    Err(e) => eprintln!("Error: {}", e),
}
```

## Accéder aux champs inconnus

Pour la compatibilité ascendante, les clés CBOR inconnues sont conservées :

```rust
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .decode()?;

// Vérifier les champs inconnus
if !result.claim169.unknown_fields.is_empty() {
    println!("Unknown fields found:");
    for (key, value) in &result.claim169.unknown_fields {
        println!("  Key {}: {:?}", key, value);
    }
}
```

## Exemple complet

```rust
use claim169_core::{Decoder, VerificationStatus, WarningCode, Claim169Error};

fn verify_credential(
    qr_content: &str,
    public_key: &[u8],
    aes_key: Option<&[u8]>,
) -> claim169_core::Result<()> {
    // Construire le décodeur
    let mut decoder = Decoder::new(qr_content);

    // Ajouter le déchiffrement si une clé est fournie
    if let Some(key) = aes_key {
        decoder = decoder.decrypt_with_aes256(key)?;
    }

    // Décoder avec vérification
    let result = decoder
        .verify_with_ed25519(public_key)?
        .clock_skew_tolerance(60)  // Tolérance 1 minute
        .decode()?;

    // Vérifier la signature
    if result.verification_status != VerificationStatus::Verified {
        return Err(Claim169Error::SignatureInvalid(
            "Signature not verified".to_string()
        ));
    }

    // Reporter les avertissements
    for warning in &result.warnings {
        eprintln!("Warning [{}]: {}",
            match warning.code {
                WarningCode::ExpiringSoon => "EXPIRING",
                WarningCode::UnknownFields => "UNKNOWN_FIELDS",
                WarningCode::TimestampValidationSkipped => "NO_TIMESTAMP_CHECK",
                WarningCode::BiometricsSkipped => "NO_BIOMETRICS",
                WarningCode::NonStandardCompression => "NON_STANDARD_COMPRESSION",
            },
            warning.message
        );
    }

    // Afficher l’identifiant
    println!("=== Verified Credential ===");
    println!("ID: {:?}", result.claim169.id);
    println!("Name: {:?}", result.claim169.full_name);
    println!("DOB: {:?}", result.claim169.date_of_birth);
    println!("Issuer: {:?}", result.cwt_meta.issuer);
    println!("Expires: {:?}", result.cwt_meta.expires_at);

    Ok(())
}
```
