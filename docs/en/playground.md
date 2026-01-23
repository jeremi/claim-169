# Interactive Playground

Try encoding and decoding Claim 169 credentials directly in your browser.

<div class="playground-link" markdown>
[Open Playground](https://jeremi.github.io/claim-169/){ .md-button .md-button--primary }
</div>

## Features

### Encode Tab

Create new credentials with:

- **Identity form** - Fill in demographic fields (name, DOB, email, etc.)
- **CWT metadata** - Set issuer, subject, and timestamps
- **Signing** - Sign with Ed25519 or ECDSA P-256
- **Encryption** - Optionally encrypt with AES-128 or AES-256
- **QR generation** - Generate scannable QR codes
- **Sample data** - Load pre-filled test data with demo keys

### Decode Tab

Verify existing credentials:

- **Paste QR data** - Input Base45-encoded QR data
- **Scan QR code** - Use your camera to scan QR codes
- **Verification** - Verify Ed25519 or ECDSA P-256 signatures
- **Decryption** - Decrypt AES-encrypted credentials
- **Example data** - Load pre-made examples from test vectors

## Quick Start

### Encoding a Credential

1. Open the [Playground](../)
2. Click **Load Sample** to populate test data
3. Modify the identity fields as needed
4. Click **Generate QR Code**
5. Scan the QR code or copy the Base45 data

### Verifying a Credential

1. Switch to the **Decode** tab
2. Select an example from the dropdown, or paste your own QR data
3. Enter the public key (shown when encoding)
4. Click **Decode**
5. View the verified identity data

## Technology

The playground runs entirely in your browser using:

- **WebAssembly** - claim169 SDK compiled to WASM
- **React** - Modern UI framework
- **html5-qrcode** - Camera-based QR scanning

No data is sent to any server.

## Screenshots

### Encode View

Generate credentials with sample data and test keys:

```
┌─────────────────────────────────────────────────────────────────┐
│  🔐 Claim 169 Playground                                        │
├─────────────────────────────────────────────────────────────────┤
│  [Decode]  [Encode]                                             │
├────────────────────────────┬────────────────────────────────────┤
│  Identity Data             │  Generated QR Code                 │
│  ─────────────             │  ─────────────────                 │
│  ID: ID-12345-DEMO         │                                    │
│  Full Name: Jane Smith     │       ▄▄▄▄▄▄▄▄▄▄▄▄▄              │
│  DOB: 1990-05-15           │       ██▀▀▀▀▀▀▀▀██              │
│  Gender: Female            │       ██ ▄▄▄▄▄ ██              │
│  Email: jane@example.com   │       ██ █   █ ██              │
│                            │       ██▄▄▄▄▄▄▄██              │
│  [Load Sample]             │                                    │
├────────────────────────────┤  Base45 Data                       │
│  Signing: Ed25519          │  6BF590B20FFWJWG...               │
│  Key: 9d61b19d...          │                                    │
│                            │  [Copy] [Download PNG]             │
│  [Generate QR Code]        │                                    │
└────────────────────────────┴────────────────────────────────────┘
```

### Decode View

Verify credentials with signature verification:

```
┌─────────────────────────────────────────────────────────────────┐
│  🔐 Claim 169 Playground                                        │
├─────────────────────────────────────────────────────────────────┤
│  [Decode]  [Encode]                                             │
├────────────────────────────┬────────────────────────────────────┤
│  QR Data Input             │  DECODED DATA                      │
│  ─────────────             │  ────────────────                  │
│  [Example: Ed25519 Signed] │  Identity:                         │
│                            │    ID: ID-SIGNED-001               │
│  6BF590B20FFWJWG...        │    Name: Signed Test Person        │
│                            │                                    │
│  [📷 Scan QR Code]         │  CWT Metadata:                     │
│                            │    Issuer: https://mosip.example   │
│  Verification: Ed25519     │    Expires: 2027-01-13             │
│  Public Key:               │                                    │
│  d75a980182b10ab7...       │  Status: ✅ Signature Verified     │
│                            │                                    │
│  [Decode]                  │                                    │
└────────────────────────────┴────────────────────────────────────┘
```

## Source Code

The playground source is available at:
[github.com/jeremi/claim-169/tree/main/playground](https://github.com/jeremi/claim-169/tree/main/playground)
