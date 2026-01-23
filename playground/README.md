# Claim 169 Playground

An interactive web application for working with MOSIP Claim 169 QR codes, inspired by [jwt.io](https://jwt.io).

## Features

### Unified Two-Panel Layout

**Left Panel - Identity & Settings:**
- Identity form with demographic fields (name, DOB, email, address, etc.)
- Credential Settings grouping Token and Cryptography together
- Auto-generated fresh keys when switching signing/encryption methods
- Load examples dropdown with demo data and test vectors

**Right Panel - QR Code & Verification:**
- Live-updating QR code as you edit fields
- Verification badge showing signature status
- Base45 data display with copy button
- Camera-based QR scanning
- Expandable pipeline details

### Live Bidirectional Sync

- Edit identity fields → QR code regenerates automatically
- Paste/scan QR data → Identity fields populate automatically
- No manual "Generate" or "Decode" buttons needed

### Key Management

- **Signing**: Ed25519 or ECDSA P-256 with auto-generated keys
- **Encryption**: AES-128 or AES-256 with auto-generated keys
- **Verification**: Paste public key to verify scanned QR codes
- Fresh keys generated per-session for security

## Development

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## Tech Stack

- **React 18** with TypeScript
- **Vite** for fast development and building
- **Tailwind CSS** with shadcn/ui components
- **claim169** SDK via WebAssembly
- **Web Crypto API** for key generation (Ed25519, ECDSA P-256, AES)
- **html5-qrcode** for camera scanning
- **qrcode.react** for QR code generation
- **i18next** for internationalization (EN, FR, ES)

## Deployment

The playground is automatically deployed to GitHub Pages when changes are pushed to the `main` branch.

Live at: https://jeremi.github.io/claim-169/

## License

MIT
