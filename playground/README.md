# Claim 169 Playground

An interactive web application for working with MOSIP Claim 169 QR codes, similar to [jwt.io](https://jwt.io).

## Features

### Decode Tab
- **Paste QR Data**: Input Base45-encoded QR code data
- **Scan QR Code**: Use your camera to scan QR codes directly
- **Signature Verification**: Verify with Ed25519 or ECDSA P-256 public keys
- **Decryption**: Decrypt AES-128 or AES-256 encrypted credentials
- **Result Display**: View decoded identity data and CWT metadata

### Encode Tab
- **Identity Form**: Fill in demographic fields (name, DOB, email, etc.)
- **CWT Metadata**: Set issuer, subject, and timestamps
- **Signing**: Sign with Ed25519 or ECDSA P-256 private keys
- **Encryption**: Optionally encrypt with AES-128 or AES-256
- **QR Generation**: Generate scannable QR codes

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
- **html5-qrcode** for camera scanning
- **qrcode.react** for QR code generation

## Deployment

The playground is automatically deployed to GitHub Pages when changes are pushed to the `main` branch. The deployment workflow builds the WASM module, TypeScript SDK, and the playground app.

Live at: https://jeremi.github.io/claim-169/

## License

MIT
