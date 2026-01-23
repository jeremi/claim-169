# Security & Validation

This page summarizes the library’s security defaults and the knobs you can tune for different deployment contexts.

## Always verify in production

Claim 169 QR data is just bytes until you validate it.

- **Production**: verify signatures (`Ed25519` or `ECDSA P-256`)
- **Testing only**: you may decode without verification, but treat the result as untrusted

!!! danger "Unverified decode is insecure"
    If you skip signature verification, a QR code can be forged. Only use unverified decode for test vectors, debugging, or when verification is done elsewhere.

## Timestamp validation (exp/nbf)

CWT timestamps help you reject expired or not-yet-valid credentials:

- `exp` (expiration)
- `nbf` (not before)

### Defaults differ by SDK

- **Rust**: timestamp validation is **enabled by default**
- **Python**: timestamp validation is **enabled by default**
- **TypeScript/WASM**: timestamp validation is **disabled by default** (WASM cannot reliably access system time)

## Compression limits

The payload is zlib-compressed. To prevent “zip bomb” style attacks, decoding enforces a maximum decompressed size.

- Default limit: **64 KB** (`65536` bytes)

If you raise the limit, do so cautiously and only if you control the input source.

## Biometric parsing

Biometric payloads can be large. If you only need demographics (name, DOB, etc.), you can skip biometric decoding for faster parsing and lower memory use.

=== "Rust"

    ```rust
    let result = claim169_core::Decoder::new(qr_text)
        .skip_biometrics()
        .allow_unverified()
        .decode()?;
    ```

=== "Python"

    ```python
    import claim169

    result = claim169.decode_unverified(qr_text, skip_biometrics=True)
    ```

=== "TypeScript"

    ```ts
    import { Decoder } from "claim169";

    const result = new Decoder(qrText)
      .skipBiometrics()
      .allowUnverified()
      .decode();
    ```

## Encryption order (sign then encrypt)

When encrypting, credentials are encoded as:

1. **Sign** the CWT (`COSE_Sign1`)
2. **Encrypt** the signed payload (`COSE_Encrypt0`)

When decoding encrypted credentials, the reverse applies:

1. **Decrypt**
2. **Verify**

## Recommended patterns

### Verifier (production)

- Require signature verification
- Enable timestamp validation (or provide an explicit policy if clocks are unreliable)
- Keep decompression limits in place

### Issuer (production)

- Always sign
- Encrypt only when there is a secure key distribution plan
- Never reuse AES-GCM nonces with the same key

