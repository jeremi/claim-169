# Claim 169

<div class="hero-section" markdown>
<h1 class="hero-title">MOSIP Claim 169 QR Code Library</h1>
<p>Encode and verify offline-verifiable digital identity credentials</p>
</div>

## What is Claim 169?

Claim 169 is an [IANA-registered](https://www.iana.org/assignments/cwt/cwt.xhtml) CBOR Web Token (CWT) claim for encoding identity credentials in QR codes. It enables **offline verification** of identity documents without network connectivity.

<div class="pipeline" markdown>
<span class="pipeline-step">Identity Data</span>
<span class="pipeline-arrow">→</span>
<span class="pipeline-step">CBOR</span>
<span class="pipeline-arrow">→</span>
<span class="pipeline-step">CWT</span>
<span class="pipeline-arrow">→</span>
<span class="pipeline-step">COSE Sign</span>
<span class="pipeline-arrow">→</span>
<span class="pipeline-step">zlib</span>
<span class="pipeline-arrow">→</span>
<span class="pipeline-step">Base45</span>
<span class="pipeline-arrow">→</span>
<span class="pipeline-step">QR Code</span>
</div>

## Choose Your SDK

<div class="sdk-grid" markdown>

<div class="sdk-card" markdown>
<div class="sdk-icon">🐍</div>
<h3>Python</h3>
<p>Native bindings with full type hints</p>
<a href="sdk/python/" class="md-button">Get Started</a>
<code>pip install claim169</code>
</div>

<div class="sdk-card" markdown>
<div class="sdk-icon">🦀</div>
<h3>Rust</h3>
<p>Core library with zero-copy parsing</p>
<a href="sdk/rust/" class="md-button">Get Started</a>
<code>cargo add claim169-core</code>
</div>

<div class="sdk-card" markdown>
<div class="sdk-icon">📜</div>
<h3>TypeScript</h3>
<p>Browser & Node.js via WebAssembly</p>
<a href="sdk/typescript/" class="md-button">Get Started</a>
<code>npm install claim169</code>
</div>

</div>

## Key Features

<div class="feature-grid" markdown>

<div class="feature-card" markdown>
### Offline Verification
Verify credentials without network access using embedded cryptographic signatures.
</div>

<div class="feature-card" markdown>
### Compact Encoding
Optimized for QR codes using CBOR, zlib compression, and Base45 encoding.
</div>

<div class="feature-card" markdown>
### Strong Cryptography
Ed25519 and ECDSA P-256 signatures with optional AES-GCM encryption.
</div>

<div class="feature-card" markdown>
### HSM/KMS Ready
Bring your own crypto provider for hardware security modules or cloud KMS.
</div>

</div>

## Quick Example

=== "Python"

    ```python
    from claim169 import Decoder

    qr_data = "..."  # Base45 from QR code
    public_key = bytes.fromhex("...")  # Issuer's Ed25519 public key

    result = (
        Decoder(qr_data)
        .verify_with_ed25519(public_key)
        .decode()
    )

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

## Try It Now

Test encoding and decoding in your browser with the [Interactive Playground](playground.md).

## Learn More

- [Specification](core/specification.md) — Wire format, CBOR keys, field tables
- [Security](core/security.md) — Threat model, safe defaults, validation
- [Glossary](core/glossary.md) — CBOR, COSE, CWT terminology
