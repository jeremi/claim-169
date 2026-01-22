# Encryption

This guide covers encrypting Claim 169 credentials to protect sensitive identity data.

## Overview

Encryption adds a layer of privacy by wrapping the signed COSE structure in a COSE_Encrypt0 envelope. Only parties with the symmetric key can decrypt and read the credential.

```
Signed Credential (COSE_Sign1) → COSE_Encrypt0 → zlib → Base45 → QR
```

## Supported Algorithms

| Algorithm | Key Size | Description |
|-----------|----------|-------------|
| AES-256-GCM | 32 bytes | Recommended for most use cases |
| AES-128-GCM | 16 bytes | Smaller key, still secure |

Both algorithms use Galois/Counter Mode (GCM) which provides authenticated encryption with associated data (AEAD).

## Encrypting Credentials

### AES-256-GCM (Recommended)

=== "Rust"

    ```rust
    use claim169_core::Encoder;

    let signing_key: [u8; 32] = /* Ed25519 private key */;
    let encryption_key: [u8; 32] = /* AES-256 key (32 bytes) */;

    let qr_data = Encoder::new(claim, meta)
        .sign_with_ed25519(&signing_key)?
        .encrypt_with_aes256(&encryption_key)?
        .encode()?;
    ```

=== "Python"

    ```python
    from claim169 import Encoder, Claim169Input, CwtMetaInput

    signing_key = bytes.fromhex("9d61b19d...")      # 32 bytes
    encryption_key = bytes.fromhex("10111213...")   # 32 bytes

    qr_data = (Encoder(claim, meta)
        .sign_with_ed25519(signing_key)
        .encrypt_with_aes256(encryption_key)
        .encode())
    ```

=== "TypeScript"

    ```typescript
    import { Encoder, hexToBytes } from 'claim169';

    const signingKey = hexToBytes("9d61b19d...");      // 32 bytes
    const encryptionKey = hexToBytes("10111213...");   // 32 bytes

    const qrData = new Encoder(claim, meta)
      .signWithEd25519(signingKey)
      .encryptWithAes256(encryptionKey)
      .encode();
    ```

### AES-128-GCM

=== "Rust"

    ```rust
    let encryption_key: [u8; 16] = /* AES-128 key (16 bytes) */;

    let qr_data = Encoder::new(claim, meta)
        .sign_with_ed25519(&signing_key)?
        .encrypt_with_aes128(&encryption_key)?
        .encode()?;
    ```

=== "Python"

    ```python
    encryption_key = bytes.fromhex("1011121314151617...")  # 16 bytes

    qr_data = (Encoder(claim, meta)
        .sign_with_ed25519(signing_key)
        .encrypt_with_aes128(encryption_key)
        .encode())
    ```

=== "TypeScript"

    ```typescript
    const encryptionKey = hexToBytes("1011121314151617...");  // 16 bytes

    const qrData = new Encoder(claim, meta)
      .signWithEd25519(signingKey)
      .encryptWithAes128(encryptionKey)
      .encode();
    ```

## Decrypting Credentials

When decrypting, you must specify the decryption method before verification:

=== "Rust"

    ```rust
    use claim169_core::Decoder;

    let encryption_key: [u8; 32] = /* AES-256 key */;
    let public_key: [u8; 32] = /* Ed25519 public key */;

    let result = Decoder::new(qr_data)
        .decrypt_with_aes256(&encryption_key)?  // Decrypt first
        .verify_with_ed25519(&public_key)?       // Then verify
        .decode()?;
    ```

=== "Python"

    ```python
    from claim169 import Decoder

    encryption_key = bytes.fromhex("10111213...")
    public_key = bytes.fromhex("d75a9801...")

    result = (Decoder(qr_data)
        .decrypt_with_aes256(encryption_key)
        .verify_with_ed25519(public_key)
        .decode())
    ```

=== "TypeScript"

    ```typescript
    import { Decoder, hexToBytes } from 'claim169';

    const encryptionKey = hexToBytes("10111213...");
    const publicKey = hexToBytes("d75a9801...");

    const result = new Decoder(qrData)
      .decryptWithAes256(encryptionKey)
      .verifyWithEd25519(publicKey)
      .decode();
    ```

## Key Management

### Generating Keys

Generate cryptographically secure random keys:

=== "Python"

    ```python
    import secrets

    # Generate AES-256 key
    aes_key = secrets.token_bytes(32)
    print(f"AES-256 key: {aes_key.hex()}")

    # Generate AES-128 key
    aes_key_128 = secrets.token_bytes(16)
    print(f"AES-128 key: {aes_key_128.hex()}")
    ```

=== "TypeScript"

    ```typescript
    // Browser
    const aesKey = crypto.getRandomValues(new Uint8Array(32));

    // Node.js
    import { randomBytes } from 'crypto';
    const aesKey = randomBytes(32);
    ```

### Key Distribution

!!! warning "Security Consideration"
    Symmetric encryption keys must be securely distributed to all parties who need to decrypt credentials. Consider:

    - **Key agreement protocols** (e.g., ECDH) for dynamic key exchange
    - **Key management systems** (KMS) for enterprise deployments
    - **Hardware security modules** (HSM) for high-security environments

## Use Cases

### Privacy-Preserving Credentials

Encrypt credentials so only authorized verifiers can read the contents:

```python
# Issuer encrypts with verifier's key
qr_data = (Encoder(claim, meta)
    .sign_with_ed25519(issuer_signing_key)
    .encrypt_with_aes256(verifier_encryption_key)
    .encode())

# Only the authorized verifier can decrypt
result = (Decoder(qr_data)
    .decrypt_with_aes256(verifier_encryption_key)
    .verify_with_ed25519(issuer_public_key)
    .decode())
```

### Selective Disclosure

Create multiple credentials with different encryption keys for different verifiers:

```python
# Full credential for government agencies
full_qr = (Encoder(full_claim, meta)
    .sign_with_ed25519(signing_key)
    .encrypt_with_aes256(government_key)
    .encode())

# Minimal credential for age verification
minimal_claim = Claim169Input(
    id=full_claim.id,
    date_of_birth=full_claim.date_of_birth
)
age_qr = (Encoder(minimal_claim, meta)
    .sign_with_ed25519(signing_key)
    .encrypt_with_aes256(merchant_key)
    .encode())
```

## Error Handling

Decryption can fail for several reasons:

=== "Python"

    ```python
    from claim169 import Decoder, Claim169Error

    try:
        result = (Decoder(qr_data)
            .decrypt_with_aes256(encryption_key)
            .verify_with_ed25519(public_key)
            .decode())
    except Claim169Error as e:
        if "decrypt" in str(e).lower():
            print("Decryption failed - wrong key?")
        elif "signature" in str(e).lower():
            print("Signature verification failed")
        else:
            print(f"Error: {e}")
    ```

=== "TypeScript"

    ```typescript
    try {
      const result = new Decoder(qrData)
        .decryptWithAes256(encryptionKey)
        .verifyWithEd25519(publicKey)
        .decode();
    } catch (error) {
      if (error.message.includes('decrypt')) {
        console.error('Decryption failed - wrong key?');
      } else if (error.message.includes('signature')) {
        console.error('Signature verification failed');
      } else {
        console.error(`Error: ${error.message}`);
      }
    }
    ```
