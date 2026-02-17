# Claim 169

<div class="hero-section" markdown>
<h1 class="hero-title">Claim 169</h1>
<p class="hero-subtitle">Vérification d'identité hors ligne, par QR code</p>
<p>Un SDK ouvert pour encoder et vérifier des identifiants d'identité signés selon le standard MOSIP Claim 169. Conçu pour les systèmes d'identité gouvernementaux, les programmes humanitaires et les scénarios hors ligne.</p>
<div class="cta-group">
<a href="playground/" class="cta-button">Essayer le Playground</a>
<a href="getting-started/" class="cta-button cta-button-secondary">Bien démarrer</a>
</div>
</div>

## Qu'est-ce que Claim 169 ?

Claim 169 est une revendication (claim) **CWT** (CBOR Web Token) [enregistrée auprès de l'IANA](https://www.iana.org/assignments/cwt/cwt.xhtml), destinée à encoder des identifiants d'identité dans des QR codes. Elle permet une **vérification hors ligne** de documents d'identité, sans connectivité réseau.

![](../assets/img/encapsulation-fr.drawio)

## Exemple rapide

=== "Python"

    ```python
    import claim169

    qr_data = "..."  # Base45 depuis le QR code
    public_key = bytes.fromhex("...")  # Clé publique Ed25519 de l'émetteur

    result = claim169.decode(qr_data, verify_with_ed25519=public_key)

    print(f"Nom : {result.claim169.full_name}")
    print(f"ID : {result.claim169.id}")
    print(f"Vérifié : {result.verification_status}")
    ```

=== "Rust"

    ```rust
    use claim169_core::Decoder;

    let qr_data = "...";  // Base45 depuis le QR code
    let public_key = hex::decode("...")?;  // Clé publique Ed25519 de l'émetteur

    let result = Decoder::new(qr_data)
        .verify_with_ed25519(&public_key)?
        .decode()?;

    println!("Nom : {:?}", result.claim169.full_name);
    println!("ID : {:?}", result.claim169.id);
    println!("Vérifié : {}", result.verification_status);
    ```

=== "TypeScript"

    ```typescript
    import { Decoder } from 'claim169';

    const qrData = "...";  // Base45 depuis le QR code
    const publicKey = new Uint8Array([...]);  // Clé publique Ed25519 de l'émetteur

    const result = new Decoder(qrData)
      .verifyWithEd25519(publicKey)
      .decode();

    console.log(`Nom : ${result.claim169.fullName}`);
    console.log(`ID : ${result.claim169.id}`);
    console.log(`Vérifié : ${result.verificationStatus}`);
    ```

=== "Kotlin"

    ```kotlin
    import fr.acn.claim169.Claim169

    val qrData = "..."  // Base45 depuis le QR code
    val publicKey = hexToByteArray("...")  // Clé publique Ed25519 de l'émetteur

    val result = Claim169.decode(qrData) {
        verifyWithEd25519(publicKey)
    }

    println("Nom : ${result.claim169.fullName}")
    println("ID : ${result.claim169.id}")
    println("Vérifié : ${result.verificationStatus}")
    ```

<div class="playground-cta" markdown>

**Essayez en direct** — Encodez et décodez des identifiants dans votre navigateur avec le [Playground interactif](playground.md).

</div>

## Pourquoi Claim 169 ?

<div class="feature-grid" markdown>

<div class="feature-card" markdown>
### Standard enregistré IANA
Non propriétaire. Le tag CBOR 169 est [enregistré auprès de l'IANA](https://www.iana.org/assignments/cwt/cwt.xhtml), garantissant l'interopérabilité entre implémentations.
</div>

<div class="feature-card" markdown>
### Optimisé pour les QR codes
CBOR + zlib + Base45 maintient des charges utiles assez compactes pour les QR codes standard, même avec des données biométriques.
</div>

<div class="feature-card" markdown>
### Vérification hors ligne prioritaire
Les signatures Ed25519 et ECDSA P-256 se vérifient sans accès réseau. Aucun appel serveur requis.
</div>

<div class="feature-card" markdown>
### Cryptographie modulaire
Utilisez votre propre HSM, KMS cloud ou clés logicielles. La bibliothèque ne manipule jamais les clés directement.
</div>

</div>

## Installation

=== "Python"

    ```bash
    pip install claim169
    ```

    [Documentation SDK Python →](sdk/python/index.md){ .md-button }

=== "Rust"

    ```bash
    cargo add claim169-core
    ```

    [Documentation SDK Rust →](sdk/rust/index.md){ .md-button }

=== "TypeScript"

    ```bash
    npm install claim169
    ```

    [Documentation SDK TypeScript →](sdk/typescript/index.md){ .md-button }

=== "Kotlin / Java"

    ```kotlin
    implementation("fr.acn.claim169:claim169-core:<version>")
    ```

    [Documentation SDK Kotlin →](sdk/kotlin/index.md){ .md-button }
    [Documentation SDK Java →](sdk/java/index.md){ .md-button }

## Pour aller plus loin

<div class="quick-links">

<a href="https://github.com/mosip/id-claim-169/tree/main" class="quick-link">
<strong>Spécification MOSIP</strong>
La spécification officielle Claim 169 : clés CBOR, tables de champs et règles d'encodage
</a>

<a href="core/security/" class="quick-link">
<strong>Sécurité</strong>
Modèle de menaces, choix d'algorithmes et valeurs sûres pour la production
</a>

<a href="core/glossary/" class="quick-link">
<strong>Glossaire</strong>
Définitions de CBOR, COSE, CWT et autres termes utilisés dans la documentation
</a>

</div>
