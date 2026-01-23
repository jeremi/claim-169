# Rust API Reference

Full API documentation is available at [docs.rs/claim169-core](https://docs.rs/claim169-core).

## Features

The crate ships with software crypto enabled by default:

- `software-crypto` (default): Ed25519, ECDSA P-256, AES-GCM helpers

Disable default features if you want to plug in your own crypto (HSM/KMS) via the `Signer` / `SignatureVerifier` / `Encryptor` / `Decryptor` traits.

## Core types

### Decoder

Builder for decoding QR data.

```rust
use claim169_core::Decoder;

let decoder = Decoder::new(qr_data);
```

#### Methods

| Method | Description |
|--------|-------------|
| `new(qr_text)` | Create decoder from Base45 text |
| `verify_with(verifier)` | Use a custom verifier (HSM/KMS integration) |
| `verify_with_ed25519(public_key)` | Verify Ed25519 (requires `software-crypto`) |
| `verify_with_ed25519_pem(pem)` | Verify Ed25519 from PEM/SPKI (requires `software-crypto`) |
| `verify_with_ecdsa_p256(public_key)` | Verify ECDSA P-256 from SEC1 bytes (requires `software-crypto`) |
| `verify_with_ecdsa_p256_pem(pem)` | Verify ECDSA P-256 from PEM/SPKI (requires `software-crypto`) |
| `decrypt_with(decryptor)` | Use a custom decryptor (HSM/KMS integration) |
| `decrypt_with_aes256(key)` | Decrypt AES-256-GCM (requires `software-crypto`) |
| `decrypt_with_aes128(key)` | Decrypt AES-128-GCM (requires `software-crypto`) |
| `allow_unverified()` | Skip signature verification (testing only) |
| `skip_biometrics()` | Skip biometric parsing for speed |
| `without_timestamp_validation()` | Disable `exp`/`nbf` checks |
| `clock_skew_tolerance(seconds)` | Allow clock skew for timestamp checks |
| `max_decompressed_bytes(bytes)` | Set the decompression size limit |
| `decode()` | Execute decoding pipeline |

### Encoder

Builder for encoding credentials.

```rust
use claim169_core::{Encoder, Claim169, CwtMeta};

let encoder = Encoder::new(claim, meta);
```

#### Methods

| Method | Description |
|--------|-------------|
| `new(claim169, cwt_meta)` | Create encoder |
| `sign_with(signer, algorithm)` | Use a custom signer (HSM/KMS integration) |
| `sign_with_ed25519(private_key)` | Sign with Ed25519 (requires `software-crypto`) |
| `sign_with_ecdsa_p256(private_key)` | Sign with ECDSA P-256 (requires `software-crypto`) |
| `allow_unsigned()` | Skip signing (testing only) |
| `encrypt_with(encryptor, algorithm)` | Use a custom encryptor |
| `encrypt_with_aes256(key)` | Encrypt with AES-256-GCM (requires `software-crypto`) |
| `encrypt_with_aes128(key)` | Encrypt with AES-128-GCM (requires `software-crypto`) |
| `encrypt_with_aes256_nonce(key, nonce)` | AES-256-GCM with explicit nonce (testing only) |
| `encrypt_with_aes128_nonce(key, nonce)` | AES-128-GCM with explicit nonce (testing only) |
| `skip_biometrics()` | Skip biometric fields during encoding |
| `encode()` | Execute encoding pipeline |

### DecodeResult

Result of successful decoding.

```rust
pub struct DecodeResult {
    pub claim169: Claim169,
    pub cwt_meta: CwtMeta,
    pub verification_status: VerificationStatus,
    pub warnings: Vec<Warning>,
}
```

## Errors

High-level operations return `claim169_core::Result<T>` which is a `Result<T, Claim169Error>`.

Common `Claim169Error` cases:

- `DecodingConfig(...)` (no verifier and no `allow_unverified()`)
- `EncodingConfig(...)` (no signer and no `allow_unsigned()`)
- `SignatureInvalid(...)`
- `DecryptionFailed(...)`
- `Expired(ts)` / `NotYetValid(ts)`
- `DecompressLimitExceeded { max_bytes }`

## Example

```rust
use claim169_core::{
    Decoder, Encoder,
    Claim169, CwtMeta,
    Gender,
};

fn main() -> Result<(), claim169_core::Claim169Error> {
    // Create a credential using builder pattern
    let claim = Claim169::new()
        .with_id("USER-001")
        .with_full_name("Alice Smith")
        .with_gender(Gender::Female);

    let meta = CwtMeta::new()
        .with_issuer("https://example.com");

    // Encode (unsigned for demo)
    let qr_data = Encoder::new(claim, meta)
        .allow_unsigned()
        .encode()?;

    // Decode
    let result = Decoder::new(&qr_data)
        .allow_unverified()
        .decode()?;

    println!("Name: {:?}", result.claim169.full_name);
    Ok(())
}
```

## Custom Crypto Providers

The library provides traits for integrating external cryptographic providers. This enables use with:

- **Hardware Security Modules (HSMs)** - Thales, AWS CloudHSM, Azure Dedicated HSM
- **Cloud KMS** - AWS KMS, Google Cloud KMS, Azure Key Vault
- **Smart Cards & TPMs** - PKCS#11 devices, PIV cards
- **Remote Signing Services** - DigiCert, DocuSign, custom signing APIs

All traits require `Send + Sync` for thread-safe usage.

### SignatureVerifier

Used during credential verification to validate COSE_Sign1 signatures.

```rust
use claim169_core::crypto::SignatureVerifier;
use claim169_core::error::CryptoResult;
use coset::iana;

pub trait SignatureVerifier: Send + Sync {
    /// Verify a signature over the given data
    ///
    /// # Arguments
    /// * `algorithm` - COSE algorithm (EdDSA, ES256, etc.)
    /// * `key_id` - Optional key identifier from COSE header
    /// * `data` - The Sig_structure bytes that were signed
    /// * `signature` - The signature bytes to verify
    ///
    /// # Returns
    /// * `Ok(())` if valid, `Err(CryptoError::VerificationFailed)` if invalid
    fn verify(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
        signature: &[u8],
    ) -> CryptoResult<()>;
}
```

### Signer

Used during credential issuance to create COSE_Sign1 signatures.

```rust
use claim169_core::crypto::Signer;
use claim169_core::error::CryptoResult;
use coset::iana;

pub trait Signer: Send + Sync {
    /// Sign data and return signature bytes
    fn sign(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
    ) -> CryptoResult<Vec<u8>>;

    /// Get the key ID for this signer (optional)
    fn key_id(&self) -> Option<&[u8]> {
        None
    }
}
```

### Decryptor

Used during credential verification to decrypt COSE_Encrypt0 payloads.

```rust
use claim169_core::crypto::Decryptor;
use claim169_core::error::CryptoResult;
use coset::iana;

pub trait Decryptor: Send + Sync {
    /// Decrypt ciphertext using AEAD
    ///
    /// # Arguments
    /// * `algorithm` - COSE algorithm (A128GCM, A256GCM)
    /// * `key_id` - Optional key identifier from COSE header
    /// * `nonce` - The IV/nonce for decryption
    /// * `aad` - Additional authenticated data (Enc_structure)
    /// * `ciphertext` - Ciphertext including auth tag
    fn decrypt(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        ciphertext: &[u8],
    ) -> CryptoResult<Vec<u8>>;
}
```

### Encryptor

Used during credential issuance to encrypt payloads with COSE_Encrypt0.

```rust
use claim169_core::crypto::Encryptor;
use claim169_core::error::CryptoResult;
use coset::iana;

pub trait Encryptor: Send + Sync {
    /// Encrypt plaintext using AEAD
    ///
    /// # Arguments
    /// * `algorithm` - COSE algorithm (A128GCM, A256GCM)
    /// * `key_id` - Optional key identifier
    /// * `nonce` - The IV/nonce for encryption
    /// * `aad` - Additional authenticated data
    /// * `plaintext` - Data to encrypt
    ///
    /// # Returns
    /// * Ciphertext with authentication tag appended
    fn encrypt(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        plaintext: &[u8],
    ) -> CryptoResult<Vec<u8>>;
}
```

### Example: AWS KMS Integration

```rust
use aws_sdk_kms::Client as KmsClient;
use claim169_core::crypto::{Signer, SignatureVerifier};
use claim169_core::error::{CryptoError, CryptoResult};
use coset::iana;

/// AWS KMS signer for Ed25519 keys
pub struct AwsKmsSigner {
    client: KmsClient,
    key_id: String,
}

impl AwsKmsSigner {
    pub fn new(client: KmsClient, key_id: String) -> Self {
        Self { client, key_id }
    }
}

impl Signer for AwsKmsSigner {
    fn sign(
        &self,
        algorithm: iana::Algorithm,
        _key_id: Option<&[u8]>,
        data: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        // Validate algorithm matches our key type
        if algorithm != iana::Algorithm::EdDSA {
            return Err(CryptoError::UnsupportedAlgorithm(algorithm));
        }

        // Call KMS synchronously (in production, use async runtime)
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| CryptoError::Other(e.to_string()))?;

        rt.block_on(async {
            let response = self.client
                .sign()
                .key_id(&self.key_id)
                .signing_algorithm(aws_sdk_kms::types::SigningAlgorithmSpec::EcdsaSha256)
                .message(aws_sdk_kms::primitives::Blob::new(data))
                .message_type(aws_sdk_kms::types::MessageType::Raw)
                .send()
                .await
                .map_err(|e| CryptoError::Other(e.to_string()))?;

            response.signature()
                .map(|s| s.as_ref().to_vec())
                .ok_or(CryptoError::Other("No signature returned".into()))
        })
    }

    fn key_id(&self) -> Option<&[u8]> {
        Some(self.key_id.as_bytes())
    }
}

// Usage with Encoder
fn issue_credential_with_kms(
    claim: Claim169,
    meta: CwtMeta,
    kms_signer: &AwsKmsSigner,
) -> Result<String, Claim169Error> {
    Encoder::new(claim, meta)
        .sign_with(kms_signer, iana::Algorithm::EdDSA)
        .encode()
}
```

### Example: PKCS#11 HSM Integration

```rust
use claim169_core::crypto::SignatureVerifier;
use claim169_core::error::{CryptoError, CryptoResult};
use coset::iana;
use cryptoki::{
    context::Pkcs11,
    mechanism::Mechanism,
    session::Session,
    types::AuthPin,
};

/// PKCS#11 verifier for HSM-stored keys
pub struct Pkcs11Verifier {
    session: Session,
    public_key_handle: u64,
}

impl SignatureVerifier for Pkcs11Verifier {
    fn verify(
        &self,
        algorithm: iana::Algorithm,
        _key_id: Option<&[u8]>,
        data: &[u8],
        signature: &[u8],
    ) -> CryptoResult<()> {
        let mechanism = match algorithm {
            iana::Algorithm::EdDSA => Mechanism::Eddsa,
            iana::Algorithm::ES256 => Mechanism::Ecdsa,
            _ => return Err(CryptoError::UnsupportedAlgorithm(algorithm)),
        };

        self.session
            .verify(&mechanism, self.public_key_handle, data, signature)
            .map_err(|_| CryptoError::VerificationFailed)
    }
}

// Usage with Decoder
fn verify_credential_with_hsm(
    qr_data: &str,
    hsm_verifier: &Pkcs11Verifier,
) -> Result<DecodeResult, Claim169Error> {
    Decoder::new(qr_data)
        .verify_with(hsm_verifier)
        .decode()
}
```

### Example: Azure Key Vault Integration

```rust
use azure_security_keyvault::KeyClient;
use claim169_core::crypto::Decryptor;
use claim169_core::error::{CryptoError, CryptoResult};
use coset::iana;

/// Azure Key Vault decryptor
pub struct AzureKeyVaultDecryptor {
    client: KeyClient,
    key_name: String,
}

impl Decryptor for AzureKeyVaultDecryptor {
    fn decrypt(
        &self,
        algorithm: iana::Algorithm,
        _key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        ciphertext: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        let alg = match algorithm {
            iana::Algorithm::A128GCM => "A128GCM",
            iana::Algorithm::A256GCM => "A256GCM",
            _ => return Err(CryptoError::UnsupportedAlgorithm(algorithm)),
        };

        // Azure Key Vault decrypt call (simplified)
        // In production, handle async properly and include AAD/nonce
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| CryptoError::Other(e.to_string()))?;

        rt.block_on(async {
            self.client
                .decrypt(&self.key_name, alg, ciphertext)
                .await
                .map(|r| r.result)
                .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
        })
    }
}
```

### Disabling Default Crypto

To use only custom providers without the built-in software crypto:

```toml
[dependencies]
claim169-core = { version = "0.1", default-features = false }
```

This removes the `software-crypto` feature and its dependencies (ed25519-dalek, p256, aes-gcm), reducing binary size for embedded or HSM-only deployments.
