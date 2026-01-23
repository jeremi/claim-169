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
    from claim169 import Claim169Input, CwtMetaInput, encode_signed_encrypted

    signing_key = bytes.fromhex("9d61b19d...")      # 32 bytes
    encryption_key = bytes.fromhex("10111213...")   # 32 bytes

    qr_data = encode_signed_encrypted(claim, meta, signing_key, encryption_key)
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
    # Note: the Python bindings currently expose AES-256-GCM encryption via
    # encode_signed_encrypted(). AES-128-GCM encoding is not exposed yet.
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
    from claim169 import decode_encrypted_aes

    encryption_key = bytes.fromhex("10111213...")

    # Testing only: decrypt without nested signature verification
    result = decode_encrypted_aes(qr_data, encryption_key, allow_unverified=True)

    # Production: provide a verifier callback (HSM/KMS) to verify the nested COSE_Sign1
    # def my_verifier(algorithm, key_id, data, signature):
    #     hsm.verify(key_id, data, signature)
    # result = decode_encrypted_aes(qr_data, encryption_key, verifier=my_verifier)
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
from claim169 import Claim169Input, encode_signed_encrypted, decode_encrypted_aes

# Issuer encrypts with verifier's key
qr_data = encode_signed_encrypted(claim, meta, issuer_signing_key, verifier_encryption_key)

# Only the authorized verifier can decrypt
result = decode_encrypted_aes(qr_data, verifier_encryption_key, verifier=my_verifier)
```

### Selective Disclosure

Create multiple credentials with different encryption keys for different verifiers:

```python
from claim169 import Claim169Input, encode_signed_encrypted

# Full credential for government agencies
full_qr = encode_signed_encrypted(full_claim, meta, signing_key, government_key)

# Minimal credential for age verification
minimal_claim = Claim169Input(
    id=full_claim.id,
    date_of_birth=full_claim.date_of_birth
)
age_qr = encode_signed_encrypted(minimal_claim, meta, signing_key, merchant_key)
```

## Error Handling

Decryption can fail for several reasons:

=== "Python"

    ```python
    import claim169

    def verifier(algorithm, key_id, data, signature):
        # Verify nested signature here (HSM/KMS or software verifier).
        # Raise if verification fails.
        ...

    try:
        result = claim169.decode_encrypted_aes(qr_data, encryption_key, verifier=verifier)
    except claim169.DecryptionError as e:
        print(f"Decryption failed (wrong key?): {e}")
    except claim169.SignatureError as e:
        print(f"Signature verification failed: {e}")
    except claim169.Claim169Exception as e:
        print(f"Claim 169 error: {e}")
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
