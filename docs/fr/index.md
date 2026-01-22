# Claim 169

Une implémentation multi-langages de la spécification [MOSIP Claim 169](https://github.com/mosip/id-claim-169) pour l'encodage et la vérification des identifiants numériques dans les codes QR.

## Aperçu

MOSIP Claim 169 définit un format compact et sécurisé pour encoder les données d'identité dans les codes QR, optimisé pour la vérification hors ligne. Cette bibliothèque fournit :

- **Bibliothèque Rust** avec encodage, décodage, vérification de signature et chiffrement
- **SDK Python** pour l'intégration côté serveur
- **SDK TypeScript/JavaScript** via WebAssembly pour navigateur et Node.js
- **Playground interactif** pour expérimenter avec les codes QR

## Pipeline d'encodage

```
Données d'identité → CBOR → CWT → COSE_Sign1 → [COSE_Encrypt0] → zlib → Base45 → Code QR
```

## Algorithmes supportés

| Opération | Algorithmes |
|-----------|-------------|
| Signature | Ed25519, ECDSA P-256 (ES256) |
| Chiffrement | AES-128-GCM, AES-256-GCM |
| Compression | zlib (DEFLATE) |
| Encodage | Base45 |

## Exemple rapide

=== "Rust"

    ```rust
    use claim169_core::{Decoder, Encoder, Claim169Input, CwtMetaInput};

    // Décoder un code QR
    let result = Decoder::new(qr_content)
        .verify_with_ed25519(&public_key)?
        .decode()?;

    println!("Nom : {:?}", result.claim169.full_name);
    ```

=== "Python"

    ```python
    from claim169 import decode_with_ed25519

    result = decode_with_ed25519(qr_text, public_key_bytes)
    print(f"Nom : {result.claim169.full_name}")
    ```

=== "TypeScript"

    ```typescript
    import { Decoder } from 'claim169';

    const result = new Decoder(qrText)
      .verifyWithEd25519(publicKey)
      .decode();

    console.log(`Nom : ${result.claim169.fullName}`);
    ```

## Prochaines étapes

- [Installation](getting-started/installation.md) - Installer le SDK pour votre langage
- [Démarrage rapide](getting-started/quick-start.md) - Encoder et décoder votre premier identifiant
- [Playground](playground.md) - Essayez dans votre navigateur
