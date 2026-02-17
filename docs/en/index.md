# Claim 169

<div class="hero-section" markdown>
<h1 class="hero-title">Claim 169</h1>
<p class="hero-subtitle">Offline identity verification, powered by QR codes</p>
<p>An open SDK for encoding and verifying signed identity credentials using the MOSIP Claim 169 standard. Built for government ID systems, humanitarian programs, and offline-first scenarios.</p>
<div class="cta-group">
<a href="playground/" class="cta-button">Try the Playground</a>
<a href="getting-started/" class="cta-button cta-button-secondary">Get Started</a>
</div>
</div>

## What is Claim 169?

Claim 169 is an [IANA-registered](https://www.iana.org/assignments/cwt/cwt.xhtml) CBOR Web Token (CWT) claim for encoding identity credentials in QR codes. It enables **offline verification** of identity documents without network connectivity.

![](../assets/img/encapsulation-en.drawio)

## Quick Example

=== "Python"

    ```python
    import claim169

    qr_data = "..."  # Base45 from QR code
    public_key = bytes.fromhex("...")  # Issuer's Ed25519 public key

    result = claim169.decode(qr_data, verify_with_ed25519=public_key)

    print(f"Name: {result.claim169.full_name}")
    print(f"ID: {result.claim169.id}")
    print(f"Verified: {result.verification_status}")
    ```

=== "Rust"

    ```rust
    use claim169_core::Decoder;

    let qr_data = "...";  // Base45 from QR code
    let public_key = hex::decode("...")?;  // Issuer's Ed25519 public key

    let result = Decoder::new(qr_data)
        .verify_with_ed25519(&public_key)?
        .decode()?;

    println!("Name: {:?}", result.claim169.full_name);
    println!("ID: {:?}", result.claim169.id);
    println!("Verified: {}", result.verification_status);
    ```

=== "TypeScript"

    ```typescript
    import { Decoder } from 'claim169';

    const qrData = "...";  // Base45 from QR code
    const publicKey = new Uint8Array([...]);  // Issuer's Ed25519 public key

    const result = new Decoder(qrData)
      .verifyWithEd25519(publicKey)
      .decode();

    console.log(`Name: ${result.claim169.fullName}`);
    console.log(`ID: ${result.claim169.id}`);
    console.log(`Verified: ${result.verificationStatus}`);
    ```

=== "Kotlin"

    ```kotlin
    import fr.acn.claim169.Claim169

    val qrData = "..."  // Base45 from QR code
    val publicKey = hexToByteArray("...")  // Issuer's Ed25519 public key

    val result = Claim169.decode(qrData) {
        verifyWithEd25519(publicKey)
    }

    println("Name: ${result.claim169.fullName}")
    println("ID: ${result.claim169.id}")
    println("Verified: ${result.verificationStatus}")
    ```

<div class="playground-cta" markdown>

**See it live** — Encode and decode credentials in your browser with the [Interactive Playground](playground.md).

</div>

## Why Claim 169?

<div class="feature-grid" markdown>

<div class="feature-card" markdown>
### IANA-Registered Standard
Not proprietary. CBOR tag 169 is [registered with IANA](https://www.iana.org/assignments/cwt/cwt.xhtml), ensuring interoperability across implementations.
</div>

<div class="feature-card" markdown>
### QR-Code Optimized
CBOR + zlib + Base45 keeps payloads compact enough for standard QR codes, even with biometric data.
</div>

<div class="feature-card" markdown>
### Offline-First Verification
Ed25519 and ECDSA P-256 signatures verify without network access. No phone-home required.
</div>

<div class="feature-card" markdown>
### Pluggable Cryptography
Bring your own HSM, cloud KMS, or software keys. The library never touches raw key material.
</div>

</div>

## Install

=== "Python"

    ```bash
    pip install claim169
    ```

    [Python SDK docs →](sdk/python/index.md){ .md-button }

=== "Rust"

    ```bash
    cargo add claim169-core
    ```

    [Rust SDK docs →](sdk/rust/index.md){ .md-button }

=== "TypeScript"

    ```bash
    npm install claim169
    ```

    [TypeScript SDK docs →](sdk/typescript/index.md){ .md-button }

=== "Kotlin / Java"

    ```kotlin
    implementation("fr.acn.claim169:claim169-core:<version>")
    ```

    [Kotlin SDK docs →](sdk/kotlin/index.md){ .md-button }
    [Java SDK docs →](sdk/java/index.md){ .md-button }

## Learn More

<div class="quick-links">

<a href="https://github.com/mosip/id-claim-169/tree/main" class="quick-link">
<strong>MOSIP Specification</strong>
The official Claim 169 spec: CBOR key mapping, field tables, and encoding rules
</a>

<a href="core/security/" class="quick-link">
<strong>Security</strong>
Threat model, algorithm choices, and safe defaults for production deployments
</a>

<a href="core/glossary/" class="quick-link">
<strong>Glossary</strong>
Definitions for CBOR, COSE, CWT, and other terms used throughout the docs
</a>

</div>
