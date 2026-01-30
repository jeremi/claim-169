# Getting Started

Claim 169 QR codes are **Base45-encoded** strings that carry signed (and optionally encrypted) identity data.

!!! warning "Do not trim Base45"
    The Base45 alphabet includes a literal space character (`" "`). Preserve scanned QR text exactly as-is (no trimming or whitespace normalization), or you can corrupt valid credentials.

## Choose Your Path

- **Verifier (read)**: decode a scanned QR code and verify it with the issuer's public key.
- **Issuer (write)**: build a Claim 169 payload, sign it with the issuer's private key, and encode it for a QR code.

## Verifier (Read)

You need:

- The scanned QR text (Base45)
- The issuer public key (and the right algorithm: Ed25519 or ECDSA P-256)

Start here:

- Python: `sdk/python/quick-start.md`
- Rust: `sdk/rust/quick-start.md`
- TypeScript: `sdk/typescript/quick-start.md`
- Kotlin: `sdk/kotlin/quick-start.md`

## Issuer (Write)

You need:

- An issuer **private key** (Ed25519 recommended)
- CWT metadata (at least `issuer`, and usually `issuedAt`/`expiresAt`)
- A minimal Claim 169 payload (often `id` + `fullName`)

Start here:

- Python: `sdk/python/encoding.md`
- Rust: `sdk/rust/encoding.md`
- TypeScript: `sdk/typescript/encoding.md`
- Kotlin: `sdk/kotlin/encoding.md`

## Known-Good Inputs (Test Vectors)

If you want a ready-made QR payload to decode (or to validate another implementation), use the repo's test vectors:

- `test-vectors/valid/ed25519-signed.json` (signed)
- `test-vectors/valid/ecdsa-p256-signed.json` (signed)
- `test-vectors/valid/encrypted-signed.json` (encrypted + signed)

The specification reference is in `core/specification.md`.

