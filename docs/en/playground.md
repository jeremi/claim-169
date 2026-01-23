# Interactive Playground

Try encoding and decoding Claim 169 credentials directly in your browser.

<div class="playground-link" markdown>
[Open Playground](https://jeremi.github.io/claim-169/){ .md-button .md-button--primary }
</div>

## Features

The playground uses a unified two-panel layout inspired by [jwt.io](https://jwt.io), with live bidirectional sync between the panels.

### Left Panel - Identity & Settings

- **Identity fields** - Fill in demographic data (name, DOB, email, address, etc.)
- **Credential Settings** - Grouped configuration for:
    - **Token Settings** - Issuer, subject, and timestamps (collapsible)
    - **Cryptography** - Signing and encryption options
- **Auto-generated keys** - Fresh cryptographic keys generated when switching methods
- **Load examples** - Pre-filled test data and sample QR codes

### Right Panel - QR Code & Verification

- **QR Code display** - Live-updating QR code as you edit fields
- **Verification badge** - Shows signature status (verified, unverified, invalid)
- **Base45 data** - Raw encoded data with copy button
- **QR Scanner** - Use your camera to scan existing QR codes
- **Pipeline details** - Expandable view of encoding stages

## Live Sync

Changes flow automatically in both directions:

- **Edit identity fields** → QR code regenerates instantly
- **Paste/scan QR data** → Identity fields populate automatically

No "Generate" or "Decode" buttons needed.

## Quick Start

### Creating a Credential

1. Open the [Playground](../)
2. Select **Load example → Demo Identity** to populate test data
3. Modify the identity fields as needed
4. The QR code updates automatically
5. Download PNG or copy the Base45 data

### Verifying a Credential

1. Click **Scan** to scan a QR code, or paste Base45 data
2. The identity fields populate automatically
3. To verify the signature:
    - Paste the issuer's public key in the **Public Key** field
    - Select the correct algorithm (Ed25519 or ECDSA P-256)
4. The verification badge shows the result

### Key Management

- **Generate button** - Creates fresh keys for the selected algorithm
- **Public key** - Auto-derived when encoding, editable for verification
- Keys are generated per-session for security (never reuse playground keys)

## Technology

The playground runs entirely in your browser using:

- **WebAssembly** - claim169 SDK compiled to WASM
- **React** - Modern UI framework
- **Web Crypto API** - Key generation (Ed25519, ECDSA P-256, AES)
- **html5-qrcode** - Camera-based QR scanning

No data is sent to any server.

## Screenshot

![Playground Screenshot](../assets/img/playground.png)

## Source Code

The playground source is available at:
[github.com/jeremi/claim-169/tree/main/playground](https://github.com/jeremi/claim-169/tree/main/playground)
