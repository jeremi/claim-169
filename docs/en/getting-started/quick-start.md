# Quick Start

This guide walks you through encoding and decoding your first Claim 169 credential.

## Decoding a QR Code

The most common operation is decoding and verifying a QR code credential.

### Without Verification (Testing Only)

For testing purposes, you can decode without verifying the signature:

=== "Rust"

    ```rust
    use claim169_core::Decoder;

    let qr_data = "6BF590B20F..."; // Base45-encoded QR data

    let result = Decoder::new(qr_data)
        .allow_unverified()
        .decode()?;

    println!("ID: {:?}", result.claim169.id);
    println!("Name: {:?}", result.claim169.full_name);
    println!("Issuer: {:?}", result.cwt_meta.issuer);
    ```

=== "Python"

    ```python
    from claim169 import decode_unverified

    qr_data = "6BF590B20F..."  # Base45-encoded QR data

    result = decode_unverified(qr_data)

    print(f"ID: {result.claim169.id}")
    print(f"Name: {result.claim169.full_name}")
    print(f"Issuer: {result.cwt_meta.issuer}")
    ```

=== "TypeScript"

    ```typescript
    import { Decoder } from 'claim169';

    const qrData = "6BF590B20F..."; // Base45-encoded QR data

    const result = new Decoder(qrData)
      .allowUnverified()
      .decode();

    console.log(`ID: ${result.claim169.id}`);
    console.log(`Name: ${result.claim169.fullName}`);
    console.log(`Issuer: ${result.cwtMeta.issuer}`);
    ```

### With Signature Verification

In production, always verify the signature:

=== "Rust"

    ```rust
    use claim169_core::Decoder;

    let qr_data = "6BF590B20F...";
    let public_key: [u8; 32] = /* Ed25519 public key */;

    let result = Decoder::new(qr_data)
        .verify_with_ed25519(&public_key)?
        .decode()?;

    // Signature verified - credential is authentic
    println!("Verified! Name: {:?}", result.claim169.full_name);
    ```

=== "Python"

    ```python
    from claim169 import decode_with_ed25519

    qr_data = "6BF590B20F..."
    public_key = bytes.fromhex("d75a980182b10ab7...")  # 32 bytes

    result = decode_with_ed25519(qr_data, public_key)

    # Signature verified - credential is authentic
    print(f"Verified! Name: {result.claim169.full_name}")
    ```

=== "TypeScript"

    ```typescript
    import { Decoder, hexToBytes } from 'claim169';

    const qrData = "6BF590B20F...";
    const publicKey = hexToBytes("d75a980182b10ab7..."); // 32 bytes

    const result = new Decoder(qrData)
      .verifyWithEd25519(publicKey)
      .decode();

    // Signature verified - credential is authentic
    console.log(`Verified! Name: ${result.claim169.fullName}`);
    ```

## Encoding a Credential

Create a new QR code credential with identity data.

=== "Rust"

    ```rust
    use claim169_core::{Encoder, Claim169, CwtMeta};

    let claim = Claim169::new()
        .with_id("USER-12345")
        .with_full_name("John Doe")
        .with_date_of_birth("19900115");

    let meta = CwtMeta::new()
        .with_issuer("https://example.com")
        .with_expires_at(1800000000);

    let private_key: [u8; 32] = /* Ed25519 private key */;

    let qr_data = Encoder::new(claim, meta)
        .sign_with_ed25519(&private_key)?
        .encode()?;

    println!("QR Data: {}", qr_data);
    ```

=== "Python"

    ```python
    from claim169 import Claim169Input, CwtMetaInput, encode_with_ed25519

    claim = Claim169Input(
        id="USER-12345",
        full_name="John Doe",
        date_of_birth="19900115"
    )

    meta = CwtMetaInput(
        issuer="https://example.com",
        expires_at=1800000000
    )

    private_key = bytes.fromhex("9d61b19deffd5a60...")  # 32 bytes

    qr_data = encode_with_ed25519(claim, meta, private_key)

    print(f"QR Data: {qr_data}")
    ```

=== "TypeScript"

    ```typescript
    import { Encoder, hexToBytes } from 'claim169';

    const claim = {
      id: "USER-12345",
      fullName: "John Doe",
      dateOfBirth: "19900115"
    };

    const meta = {
      issuer: "https://example.com",
      expiresAt: 1800000000
    };

    const privateKey = hexToBytes("9d61b19deffd5a60..."); // 32 bytes

    const qrData = new Encoder(claim, meta)
      .signWithEd25519(privateKey)
      .encode();

    console.log(`QR Data: ${qrData}`);
    ```

## Handling Encrypted Credentials

Some credentials are encrypted for privacy. You need the decryption key to read them.

=== "Rust"

    ```rust
    use claim169_core::Decoder;

    let qr_data = "6BFCA0410D..."; // Encrypted credential
    let encryption_key: [u8; 32] = /* AES-256 key */;
    let public_key: [u8; 32] = /* Ed25519 public key */;

    let result = Decoder::new(qr_data)
        .decrypt_with_aes256(&encryption_key)?
        .verify_with_ed25519(&public_key)?
        .decode()?;
    ```

=== "Python"

    ```python
    from claim169 import decode_encrypted_aes

    qr_data = "6BFCA0410D..."  # Encrypted credential
    encryption_key = bytes.fromhex("1011121314...")  # 32 bytes for AES-256

    # Testing only: decrypt but do not verify the nested signature
    result = decode_encrypted_aes(qr_data, encryption_key, allow_unverified=True)
    ```

=== "TypeScript"

    ```typescript
    import { Decoder, hexToBytes } from 'claim169';

    const qrData = "6BFCA0410D..."; // Encrypted credential
    const encryptionKey = hexToBytes("1011121314..."); // 32 bytes for AES-256
    const publicKey = hexToBytes("994c54604862...");

    const result = new Decoder(qrData)
      .decryptWithAes256(encryptionKey)
      .verifyWithEd25519(publicKey)
      .decode();
    ```

## Next Steps

- [Encoding Guide](../guides/encoding.md) - Learn about all available fields
- [Decoding Guide](../guides/decoding.md) - Handle edge cases and errors
- [Encryption Guide](../guides/encryption.md) - Secure sensitive data
- [Playground](../playground.md) - Try it interactively in your browser
