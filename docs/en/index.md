# Claim 169

<div class="hero-section" markdown>
<h1 class="hero-title">MOSIP Claim 169 QR Code Library</h1>
<p>Encode and verify offline-verifiable digital identity credentials</p>
<div class="cta-group">
<a href="playground/" class="cta-button">Try the Playground</a>
<a href="core/specification/" class="cta-button cta-button-secondary">Read the Specification</a>
</div>
</div>

## What is Claim 169?

Claim 169 is an [IANA-registered](https://www.iana.org/assignments/cwt/cwt.xhtml) CBOR Web Token (CWT) claim for encoding identity credentials in QR codes. It enables **offline verification** of identity documents without network connectivity.

![](../assets/img/encapsulation-en.drawio)

## Choose Your SDK

<div class="sdk-grid" markdown>

<div class="sdk-card" markdown>
<h3>Python</h3>
<p>Native bindings with full type hints</p>
<a href="sdk/python/" class="md-button">Get Started</a>
<code>pip install claim169</code>
</div>

<div class="sdk-card" markdown>
<h3>Rust</h3>
<p>Core library with zero-copy parsing</p>
<a href="sdk/rust/" class="md-button">Get Started</a>
<code>cargo add claim169-core</code>
</div>

<div class="sdk-card" markdown>
<h3>TypeScript</h3>
<p>Browser & Node.js via WebAssembly</p>
<a href="sdk/typescript/" class="md-button">Get Started</a>
<code>npm install claim169</code>
</div>

<div class="sdk-card" markdown>
<h3>Kotlin</h3>
<p>Android & JVM via native bindings</p>
<a href="sdk/kotlin/" class="md-button">Get Started</a>
<code>implementation(&quot;fr.acn.claim169:claim169-core:&lt;version&gt;&quot;)</code>
</div>

<div class="sdk-card" markdown>
<h3>Java</h3>
<p>JDK 17+ with lambda API</p>
<a href="sdk/java/" class="md-button">Get Started</a>
<code>implementation(&quot;fr.acn.claim169:claim169-core:&lt;version&gt;&quot;)</code>
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
    from claim169 import decode

    qr_data = "..."  # Base45 from QR code
    public_key = bytes.fromhex("...")  # Issuer's Ed25519 public key

    result = decode(qr_data, verify_with_ed25519=public_key)

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

<div class="quick-links">

<a href="core/specification/" class="quick-link">
<strong>Specification</strong>
Wire format, CBOR keys, field tables
</a>

<a href="core/security/" class="quick-link">
<strong>Security</strong>
Threat model, safe defaults, validation
</a>

<a href="core/glossary/" class="quick-link">
<strong>Glossary</strong>
CBOR, COSE, CWT terminology
</a>

</div>
