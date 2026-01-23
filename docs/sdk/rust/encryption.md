# Encryption

The Claim 169 specification supports optional AES-GCM encryption for protecting credential contents. This adds a COSE_Encrypt0 layer on top of the signed COSE_Sign1 structure.

## Overview

When encryption is used, the data flow is:

```text
Encoding: Claim169 -> CBOR -> CWT -> COSE_Sign1 -> COSE_Encrypt0 -> zlib -> Base45
Decoding: Base45 -> zlib -> COSE_Encrypt0 -> COSE_Sign1 -> CWT -> Claim169
```

The credential is always **signed first, then encrypted** (sign-then-encrypt).

## Supported Algorithms

| Algorithm | Key Size | Nonce Size | Description |
|-----------|----------|------------|-------------|
| A256GCM | 32 bytes | 12 bytes | AES-256-GCM (recommended) |
| A128GCM | 16 bytes | 12 bytes | AES-128-GCM |

## Encoding with Encryption

### AES-256-GCM (Recommended)

```rust
use claim169_core::{Encoder, Claim169, CwtMeta};

let claim169 = Claim169::minimal("ID-001", "Jane Doe");
let cwt_meta = CwtMeta::new().with_issuer("https://issuer.example.com");

// 32-byte encryption key
let aes_key: [u8; 32] = [/* your key bytes */];

let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&signing_key)?
    .encrypt_with_aes256(&aes_key)?  // Random nonce generated
    .encode()?;
```

### AES-128-GCM

```rust
// 16-byte encryption key
let aes_key: [u8; 16] = [/* your key bytes */];

let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&signing_key)?
    .encrypt_with_aes128(&aes_key)?
    .encode()?;
```

### Explicit Nonce (Testing Only)

For deterministic output in testing:

```rust
let aes_key: [u8; 32] = [/* your key */];
let nonce: [u8; 12] = [/* unique nonce */];

let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&signing_key)?
    .encrypt_with_aes256_nonce(&aes_key, &nonce)?
    .encode()?;
```

**Warning**: Never reuse a nonce with the same key. Nonce reuse completely breaks AES-GCM security.

### Generating Random Nonces

The library provides a helper for generating secure random nonces:

```rust
use claim169_core::generate_random_nonce;

let nonce = generate_random_nonce();  // Returns [u8; 12]
```

## Decoding with Decryption

### AES-256-GCM

```rust
use claim169_core::Decoder;

let aes_key: [u8; 32] = [/* your key */];

let result = Decoder::new(qr_content)
    .decrypt_with_aes256(&aes_key)?
    .verify_with_ed25519(&public_key)?
    .decode()?;
```

### AES-128-GCM

```rust
let aes_key: [u8; 16] = [/* your key */];

let result = Decoder::new(qr_content)
    .decrypt_with_aes128(&aes_key)?
    .verify_with_ed25519(&public_key)?
    .decode()?;
```

## Custom Encryption/Decryption

For HSM or cloud KMS integration, implement the `Encryptor` and `Decryptor` traits:

### Encryptor Trait

```rust
use claim169_core::{Encryptor, CryptoResult};
use coset::iana;

struct MyHsmEncryptor {
    key_id: Vec<u8>,
    // ... HSM connection details
}

impl Encryptor for MyHsmEncryptor {
    fn encrypt(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        plaintext: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        // Call HSM to encrypt
        // Return ciphertext with authentication tag appended
        todo!()
    }
}
```

### Decryptor Trait

```rust
use claim169_core::{Decryptor, CryptoResult};
use coset::iana;

struct MyHsmDecryptor {
    key_id: Vec<u8>,
}

impl Decryptor for MyHsmDecryptor {
    fn decrypt(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        ciphertext: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        // Call HSM to decrypt
        // ciphertext includes the authentication tag
        todo!()
    }
}
```

### Using Custom Crypto

```rust
// Encoding
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&signing_key)?
    .encrypt_with(my_hsm_encryptor, iana::Algorithm::A256GCM)
    .encode()?;

// Decoding
let result = Decoder::new(qr_content)
    .decrypt_with(my_hsm_decryptor)
    .verify_with_ed25519(&public_key)?
    .decode()?;
```

See [Custom Crypto](./custom-crypto.md) for complete HSM integration examples.

## Key Management Best Practices

### Key Generation

```rust
use rand::RngCore;

fn generate_aes256_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

fn generate_aes128_key() -> [u8; 16] {
    let mut key = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut key);
    key
}
```

### Key Storage

- Never hardcode encryption keys in source code
- Use environment variables for development
- Use a secrets manager (AWS Secrets Manager, HashiCorp Vault) in production
- Consider using HSM for high-security applications

### Key Rotation

When rotating encryption keys:

1. Support multiple decryption keys during transition
2. Re-encrypt existing credentials with new keys
3. Deprecate old keys after transition period

## Error Handling

```rust
use claim169_core::{Decoder, Claim169Error};

match Decoder::new(qr_content)
    .decrypt_with_aes256(&aes_key)?
    .verify_with_ed25519(&public_key)?
    .decode()
{
    Ok(result) => {
        println!("Decrypted and verified: {:?}", result.claim169.full_name);
    }
    Err(Claim169Error::DecryptionFailed(msg)) => {
        eprintln!("Decryption failed: {}", msg);
        eprintln!("Possible causes:");
        eprintln!("  - Wrong encryption key");
        eprintln!("  - Corrupted ciphertext");
        eprintln!("  - Tampered data (authentication failed)");
    }
    Err(Claim169Error::Crypto(msg)) => {
        eprintln!("Crypto error: {}", msg);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Complete Example

```rust
use claim169_core::{
    Encoder, Decoder, Claim169, CwtMeta,
    Ed25519Signer, VerificationStatus,
    generate_random_nonce,
};
use rand::RngCore;

fn encrypted_roundtrip() -> claim169_core::Result<()> {
    // Generate keys
    let signer = Ed25519Signer::generate();
    let public_key = signer.public_key_bytes();

    let mut aes_key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut aes_key);

    // Create credential
    let claim169 = Claim169::new()
        .with_id("SECURE-001")
        .with_full_name("Alice Secure")
        .with_email("alice@secure.example.com");

    let cwt_meta = CwtMeta::new()
        .with_issuer("https://secure.issuer.com")
        .with_expires_at(1893456000);

    // Encode with signing and encryption
    let qr_data = Encoder::new(claim169.clone(), cwt_meta)
        .sign_with(signer, coset::iana::Algorithm::EdDSA)
        .encrypt_with_aes256(&aes_key)?
        .encode()?;

    println!("Encrypted QR: {} chars", qr_data.len());

    // Decode with decryption and verification
    let result = Decoder::new(&qr_data)
        .decrypt_with_aes256(&aes_key)?
        .verify_with_ed25519(&public_key)?
        .decode()?;

    assert_eq!(result.verification_status, VerificationStatus::Verified);
    assert_eq!(result.claim169.id, claim169.id);
    assert_eq!(result.claim169.full_name, claim169.full_name);

    println!("Decrypted and verified successfully!");
    println!("Name: {:?}", result.claim169.full_name);

    Ok(())
}
```

## Security Considerations

1. **Key Size**: Use AES-256-GCM for maximum security margin
2. **Nonce Uniqueness**: Never reuse nonces with the same key
3. **Key Protection**: Store keys securely, consider HSM for production
4. **Authenticated Encryption**: AES-GCM provides both confidentiality and integrity
5. **Side Channels**: The software implementation is not hardened against timing attacks; use HSM for high-security scenarios
