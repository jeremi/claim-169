# Démarrage rapide

Ce guide montre comment encoder et décoder des QR codes Claim 169 en quelques minutes.

## Encodage de base

Créer un QR code signé à partir de données d’identité :

```rust
use claim169_core::{Encoder, Claim169, CwtMeta, Gender};

fn create_credential(private_key: &[u8]) -> claim169_core::Result<String> {
    // Construire les données d’identité
    let claim169 = Claim169::new()
        .with_id("ID-12345-ABCDE")
        .with_full_name("Jane Marie Smith")
        .with_date_of_birth("19900515")
        .with_gender(Gender::Female)
        .with_email("jane.smith@example.com");

    // Définir les métadonnées CWT
    let cwt_meta = CwtMeta::new()
        .with_issuer("https://issuer.example.com")
        .with_subject("jane.smith")
        .with_expires_at(1800000000)  // Unix timestamp
        .with_issued_at(1700000000);

    // Encoder avec signature Ed25519
    let qr_data = Encoder::new(claim169, cwt_meta)
        .sign_with_ed25519(private_key)?
        .encode()?;

    Ok(qr_data)
}
```

## Décodage de base

Lire et vérifier un QR code :

```rust
use claim169_core::{Decoder, VerificationStatus};

fn verify_credential(qr_content: &str, public_key: &[u8]) -> claim169_core::Result<()> {
    let result = Decoder::new(qr_content)
        .verify_with_ed25519(public_key)?
        .decode()?;

    // Vérifier le statut de vérification
    match result.verification_status {
        VerificationStatus::Verified => println!("Signature verified"),
        VerificationStatus::Skipped => println!("Verification skipped"),
        VerificationStatus::Failed => println!("Verification failed"),
    }

    // Accéder aux données d’identité
    println!("ID: {:?}", result.claim169.id);
    println!("Name: {:?}", result.claim169.full_name);
    println!("DOB: {:?}", result.claim169.date_of_birth);
    println!("Email: {:?}", result.claim169.email);

    // Accéder aux métadonnées CWT
    println!("Issuer: {:?}", result.cwt_meta.issuer);
    println!("Expires: {:?}", result.cwt_meta.expires_at);

    // Afficher les avertissements éventuels
    for warning in &result.warnings {
        println!("Warning: {}", warning.message);
    }

    Ok(())
}
```

## Exemple roundtrip complet

Un exemple complet avec génération de clés :

```rust
use claim169_core::{
    Encoder, Decoder, Claim169, CwtMeta, Gender, MaritalStatus,
    Ed25519Signer, VerificationStatus,
};

fn main() -> claim169_core::Result<()> {
    // Générer une paire de clés Ed25519
    let signer = Ed25519Signer::generate();
    let public_key = signer.public_key_bytes();

    // Créer des données d’identité
    let claim169 = Claim169::new()
        .with_id("CITIZEN-2024-001")
        .with_full_name("Maria Garcia Lopez")
        .with_first_name("Maria")
        .with_last_name("Garcia Lopez")
        .with_date_of_birth("19850320")
        .with_gender(Gender::Female)
        .with_nationality("ES")
        .with_marital_status(MaritalStatus::Married)
        .with_address("Calle Principal 123\nMadrid, 28001\nSpain")
        .with_email("maria.garcia@example.com")
        .with_phone("+34 612 345 678");

    // Créer les métadonnées CWT
    let cwt_meta = CwtMeta::new()
        .with_issuer("https://id.government.es")
        .with_subject("maria.garcia")
        .with_expires_at(1893456000)  // 2030-01-01
        .with_issued_at(1704067200);  // 2024-01-01

    // Encoder en chaîne QR
    let qr_data = Encoder::new(claim169.clone(), cwt_meta.clone())
        .sign_with(signer, coset::iana::Algorithm::EdDSA)
        .encode()?;

    println!("QR Data ({} chars): {}...", qr_data.len(), &qr_data[..50]);

    // Décoder et vérifier
    let result = Decoder::new(&qr_data)
        .verify_with_ed25519(&public_key)?
        .decode()?;

    // Vérifier le roundtrip
    assert_eq!(result.verification_status, VerificationStatus::Verified);
    assert_eq!(result.claim169.id, claim169.id);
    assert_eq!(result.claim169.full_name, claim169.full_name);
    assert_eq!(result.cwt_meta.issuer, cwt_meta.issuer);

    println!("Roundtrip successful!");
    println!("Verified: {:?}", result.verification_status);
    println!("Name: {:?}", result.claim169.full_name);
    println!("Issuer: {:?}", result.cwt_meta.issuer);

    Ok(())
}
```

## Tester sans signatures

Pour le développement et les tests, vous pouvez ignorer les signatures :

```rust
use claim169_core::{Encoder, Decoder, Claim169, CwtMeta};

fn test_without_signatures() -> claim169_core::Result<()> {
    let claim169 = Claim169::minimal("test-123", "Test User");
    let cwt_meta = CwtMeta::new().with_issuer("test");

    // Encoder sans signature (tests uniquement)
    let qr_data = Encoder::new(claim169, cwt_meta)
        .allow_unsigned()
        .encode()?;

    // Décoder sans vérification (tests uniquement)
    let result = Decoder::new(&qr_data)
        .allow_unverified()
        .decode()?;

    println!("Decoded: {:?}", result.claim169.full_name);
    Ok(())
}
```

**Avertissement** : ne jamais utiliser `allow_unsigned()` ou `allow_unverified()` en production. Des identifiants non vérifiés ne peuvent pas être considérés fiables.

## Étapes suivantes

- [Encodage](./encoding.md) — toutes les options d’encodage
- [Décodage](./decoding.md) — toutes les options de décodage
- [Chiffrement](./encryption.md) — ajouter le chiffrement AES-GCM
- [Crypto personnalisée](./custom-crypto.md) — intégration HSM/KMS

