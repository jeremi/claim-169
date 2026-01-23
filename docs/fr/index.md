# Claim 169

Encodez et vérifiez des identifiants numériques avec la spécification de code QR [MOSIP Claim 169](https://github.com/mosip/id-claim-169).

[Commencer](getting-started/installation.md){ .md-button .md-button--primary }
[Essayer le playground](playground.md){ .md-button }

## Qu'est-ce que Claim 169 ?

MOSIP Claim 169 définit un format compact et sécurisé pour encoder les données d'identité dans les codes QR, optimisé pour la vérification hors ligne. Ce dépôt fournit :

- **Bibliothèque Rust** (encodage, décodage, vérification, chiffrement)
- **SDK Python** (intégration côté serveur)
- **SDK TypeScript/JavaScript** (WASM pour navigateur et Node.js)
- **Playground interactif** (testez les vecteurs et construisez des payloads QR)

<div class="grid cards" markdown>

-   ### Rust Core
    Encodage/décodage haute performance avec vérification de signature et chiffrement optionnel.

    [API Rust](api/rust.md){ .md-button }

-   ### SDK Python
    Fonctions simples pour la vérification, le déchiffrement et les pipelines de décodage dans les services Python.

    [API Python](api/python.md){ .md-button }

-   ### TypeScript / JavaScript
    SDK alimenté par WebAssembly pour navigateur et Node.js.

    [API TypeScript](api/typescript.md){ .md-button }

-   ### Playground
    Encodez, décodez, déchiffrez et vérifiez sans rien installer.

    [Ouvrir le playground](playground.md){ .md-button }

</div>

## Pipeline d'encodage

```text
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
    use claim169_core::Decoder;

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
    import { Decoder } from "claim169";

    const result = new Decoder(qrText)
      .verifyWithEd25519(publicKey)
      .decode();

    console.log(`Nom : ${result.claim169.fullName}`);
    ```

## Liens rapides

<div class="grid cards" markdown>

-   ### Installation
    Installez le SDK pour votre langage.

    [Installer](getting-started/installation.md){ .md-button }

-   ### Démarrage rapide
    Encodez et décodez votre premier identifiant.

    [Démarrage rapide](getting-started/quick-start.md){ .md-button }

-   ### Clés
    Formats de clés et comment les fournir.

    [Clés](guides/keys.md){ .md-button }

-   ### Sécurité et validation
    Paramètres par défaut et options de politique.

    [Sécurité](guides/security.md){ .md-button }

-   ### Spécification
    Le format filaire et les structures.

    [Spécification](specification.md){ .md-button }

-   ### Dépannage
    Erreurs courantes et solutions.

    [Dépannage](guides/troubleshooting.md){ .md-button }

</div>

## Prochaines étapes

- [Démarrage rapide](getting-started/quick-start.md) — encodez et décodez votre premier identifiant
- [Matériel de clés et formats](guides/keys.md) — formats de clés et support PEM
- [Sécurité et validation](guides/security.md) — paramètres par défaut et options de politique
- [Glossaire](guides/glossary.md) — CBOR, COSE, CWT, etc.
- [Versioning](guides/versioning.md) — relation entre la doc et les versions
- [Dépannage](guides/troubleshooting.md) — erreurs courantes et solutions

**Besoin d'aide ?** Commencez par [Dépannage](guides/troubleshooting.md) ou consultez [Contribuer](guides/contributing.md) pour améliorer la documentation.
