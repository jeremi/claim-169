# Custom Crypto Providers

The `claim169-core` library supports pluggable cryptographic backends through traits. This enables integration with:

- Hardware Security Modules (HSMs)
- Cloud KMS (AWS KMS, Google Cloud KMS, Azure Key Vault)
- Remote signing services
- Smart cards and TPMs
- Custom software keystores

## Architecture

The library defines four cryptographic traits:

| Trait | Purpose | Used By |
|-------|---------|---------|
| `Signer` | Sign credentials | `Encoder` |
| `SignatureVerifier` | Verify signatures | `Decoder` |
| `Encryptor` | Encrypt credentials | `Encoder` |
| `Decryptor` | Decrypt credentials | `Decoder` |

All traits require `Send + Sync` for thread safety.

## Disabling Software Crypto

For HSM-only deployments, disable the default software implementations:

```toml
[dependencies]
claim169-core = { version = "0.1", default-features = false }
```

This removes `Ed25519Signer`, `EcdsaP256Signer`, `AesGcmEncryptor`, etc.

## SignatureVerifier Trait

Implement `SignatureVerifier` for custom signature verification:

```rust
use claim169_core::{SignatureVerifier, CryptoResult, CryptoError};
use coset::iana;

pub struct HsmVerifier {
    hsm_session: HsmSession,
    key_label: String,
}

impl SignatureVerifier for HsmVerifier {
    fn verify(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
        signature: &[u8],
    ) -> CryptoResult<()> {
        // Check algorithm support
        let hsm_algo = match algorithm {
            iana::Algorithm::EdDSA => HsmAlgorithm::Ed25519,
            iana::Algorithm::ES256 => HsmAlgorithm::EcdsaP256,
            other => return Err(CryptoError::UnsupportedAlgorithm(
                format!("{:?}", other)
            )),
        };

        // Resolve key by ID or use default
        let key_handle = if let Some(kid) = key_id {
            self.hsm_session.find_key_by_id(kid)
                .map_err(|_| CryptoError::KeyNotFound)?
        } else {
            self.hsm_session.find_key_by_label(&self.key_label)
                .map_err(|_| CryptoError::KeyNotFound)?
        };

        // Perform verification
        let result = self.hsm_session.verify(
            key_handle,
            hsm_algo,
            data,
            signature,
        );

        match result {
            Ok(true) => Ok(()),
            Ok(false) => Err(CryptoError::VerificationFailed),
            Err(e) => Err(CryptoError::Other(e.to_string())),
        }
    }
}

// Usage
let verifier = HsmVerifier::new(hsm_session, "my-signing-key");
let result = Decoder::new(qr_content)
    .verify_with(verifier)
    .decode()?;
```

## Signer Trait

Implement `Signer` for custom signing:

```rust
use claim169_core::{Signer, CryptoResult, CryptoError};
use coset::iana;

pub struct HsmSigner {
    hsm_session: HsmSession,
    key_label: String,
    key_id: Option<Vec<u8>>,
}

impl Signer for HsmSigner {
    fn sign(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        let hsm_algo = match algorithm {
            iana::Algorithm::EdDSA => HsmAlgorithm::Ed25519,
            iana::Algorithm::ES256 => HsmAlgorithm::EcdsaP256,
            other => return Err(CryptoError::UnsupportedAlgorithm(
                format!("{:?}", other)
            )),
        };

        let key_handle = self.hsm_session
            .find_key_by_label(&self.key_label)
            .map_err(|_| CryptoError::KeyNotFound)?;

        self.hsm_session
            .sign(key_handle, hsm_algo, data)
            .map_err(|e| CryptoError::SigningFailed(e.to_string()))
    }

    fn key_id(&self) -> Option<&[u8]> {
        self.key_id.as_deref()
    }
}

// Usage
let signer = HsmSigner::new(hsm_session, "my-signing-key");
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with(signer, iana::Algorithm::EdDSA)
    .encode()?;
```

## Decryptor Trait

Implement `Decryptor` for custom decryption:

```rust
use claim169_core::{Decryptor, CryptoResult, CryptoError};
use coset::iana;

pub struct KmsDecryptor {
    kms_client: KmsClient,
    key_arn: String,
}

impl Decryptor for KmsDecryptor {
    fn decrypt(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        ciphertext: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        // Validate algorithm
        match algorithm {
            iana::Algorithm::A256GCM | iana::Algorithm::A128GCM => {}
            other => return Err(CryptoError::UnsupportedAlgorithm(
                format!("{:?}", other)
            )),
        }

        // For AES-GCM, the ciphertext includes the auth tag
        // AWS KMS expects them separately (last 16 bytes = tag)
        if ciphertext.len() < 16 {
            return Err(CryptoError::DecryptionFailed(
                "Ciphertext too short".to_string()
            ));
        }

        let (ct, tag) = ciphertext.split_at(ciphertext.len() - 16);

        // Build KMS request
        let request = DecryptRequest {
            key_id: self.key_arn.clone(),
            ciphertext_blob: ct.to_vec(),
            encryption_algorithm: "AES_256_GCM".to_string(),
            encryption_context: None,
            // Pass nonce and AAD as appropriate for your KMS
        };

        self.kms_client
            .decrypt(request)
            .map(|resp| resp.plaintext)
            .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
    }
}

// Usage
let decryptor = KmsDecryptor::new(kms_client, "arn:aws:kms:...");
let result = Decoder::new(qr_content)
    .decrypt_with(decryptor)
    .verify_with_ed25519(&public_key)?
    .decode()?;
```

## Encryptor Trait

Implement `Encryptor` for custom encryption:

```rust
use claim169_core::{Encryptor, CryptoResult, CryptoError};
use coset::iana;

pub struct KmsEncryptor {
    kms_client: KmsClient,
    key_arn: String,
}

impl Encryptor for KmsEncryptor {
    fn encrypt(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        plaintext: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        match algorithm {
            iana::Algorithm::A256GCM | iana::Algorithm::A128GCM => {}
            other => return Err(CryptoError::UnsupportedAlgorithm(
                format!("{:?}", other)
            )),
        }

        let request = EncryptRequest {
            key_id: self.key_arn.clone(),
            plaintext: plaintext.to_vec(),
            encryption_algorithm: "AES_256_GCM".to_string(),
            // Pass nonce and AAD
        };

        let response = self.kms_client
            .encrypt(request)
            .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

        // Return ciphertext with auth tag appended
        let mut result = response.ciphertext_blob;
        result.extend_from_slice(&response.auth_tag);
        Ok(result)
    }
}

// Usage
let encryptor = KmsEncryptor::new(kms_client, "arn:aws:kms:...");
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&signing_key)?
    .encrypt_with(encryptor, iana::Algorithm::A256GCM)
    .encode()?;
```

## KeyResolver Trait

For scenarios where you need to look up keys dynamically:

```rust
use claim169_core::{KeyResolver, SignatureVerifier, Decryptor, CryptoResult};
use coset::iana;

pub struct KeyVaultResolver {
    vault_client: KeyVaultClient,
}

impl KeyResolver for KeyVaultResolver {
    fn resolve_verifier(
        &self,
        key_id: Option<&[u8]>,
        algorithm: iana::Algorithm,
    ) -> CryptoResult<Box<dyn SignatureVerifier>> {
        let kid = key_id
            .map(|k| String::from_utf8_lossy(k).to_string())
            .ok_or(CryptoError::KeyNotFound)?;

        let key = self.vault_client.get_key(&kid)?;

        Ok(Box::new(KeyVaultVerifier::new(self.vault_client.clone(), key)))
    }

    fn resolve_decryptor(
        &self,
        key_id: Option<&[u8]>,
        algorithm: iana::Algorithm,
    ) -> CryptoResult<Box<dyn Decryptor>> {
        let kid = key_id
            .map(|k| String::from_utf8_lossy(k).to_string())
            .ok_or(CryptoError::KeyNotFound)?;

        let key = self.vault_client.get_key(&kid)?;

        Ok(Box::new(KeyVaultDecryptor::new(self.vault_client.clone(), key)))
    }
}
```

## AWS KMS Example

A complete example for AWS KMS:

```rust
use aws_sdk_kms::{Client as KmsClient, types::SigningAlgorithmSpec};
use claim169_core::{Signer, SignatureVerifier, CryptoResult, CryptoError};
use coset::iana;

pub struct AwsKmsSigner {
    client: KmsClient,
    key_id: String,
}

impl AwsKmsSigner {
    pub fn new(client: KmsClient, key_id: impl Into<String>) -> Self {
        Self {
            client,
            key_id: key_id.into(),
        }
    }
}

impl Signer for AwsKmsSigner {
    fn sign(
        &self,
        algorithm: iana::Algorithm,
        _key_id: Option<&[u8]>,
        data: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        let signing_algorithm = match algorithm {
            iana::Algorithm::ES256 => SigningAlgorithmSpec::EcdsaSha256,
            other => return Err(CryptoError::UnsupportedAlgorithm(
                format!("{:?} not supported by AWS KMS", other)
            )),
        };

        // AWS KMS Sign operation
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(async {
            self.client
                .sign()
                .key_id(&self.key_id)
                .signing_algorithm(signing_algorithm)
                .message(aws_sdk_kms::primitives::Blob::new(data))
                .send()
                .await
        });

        match result {
            Ok(output) => {
                let sig = output.signature()
                    .ok_or(CryptoError::SigningFailed("No signature returned".into()))?;
                Ok(sig.as_ref().to_vec())
            }
            Err(e) => Err(CryptoError::SigningFailed(e.to_string())),
        }
    }

    fn key_id(&self) -> Option<&[u8]> {
        Some(self.key_id.as_bytes())
    }
}

// Usage
let kms_client = aws_sdk_kms::Client::new(&aws_config);
let signer = AwsKmsSigner::new(kms_client, "alias/my-signing-key");

let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with(signer, iana::Algorithm::ES256)  // AWS KMS uses ECDSA
    .encode()?;
```

## Error Handling

Return appropriate `CryptoError` variants:

```rust
use claim169_core::CryptoError;

// Key not found
Err(CryptoError::KeyNotFound)

// Unsupported algorithm
Err(CryptoError::UnsupportedAlgorithm("ChaCha20 not supported".into()))

// Verification failed
Err(CryptoError::VerificationFailed)

// Decryption failed
Err(CryptoError::DecryptionFailed("Authentication tag mismatch".into()))

// Signing failed
Err(CryptoError::SigningFailed("HSM communication error".into()))

// Encryption failed
Err(CryptoError::EncryptionFailed("Key not suitable for encryption".into()))

// Invalid key format
Err(CryptoError::InvalidKeyFormat("Expected 32-byte key".into()))

// Generic error
Err(CryptoError::Other("Unexpected error".into()))
```

## Thread Safety

All traits require `Send + Sync`. Ensure your implementations are thread-safe:

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ThreadSafeHsmSigner {
    session: Arc<Mutex<HsmSession>>,
    key_label: String,
}

impl Signer for ThreadSafeHsmSigner {
    fn sign(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let session = self.session.lock().await;
            // Perform signing with locked session
            session.sign(&self.key_label, data)
        })
        .map_err(|e| CryptoError::SigningFailed(e.to_string()))
    }
}
```

## Testing Custom Implementations

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use claim169_core::{Encoder, Decoder, Claim169, CwtMeta};

    #[test]
    fn test_custom_signer_roundtrip() {
        let signer = MyCustomSigner::new(/* ... */);
        let verifier = MyCustomVerifier::new(/* ... */);

        let claim = Claim169::minimal("test", "Test User");
        let meta = CwtMeta::new().with_issuer("test");

        // Encode
        let qr_data = Encoder::new(claim.clone(), meta)
            .sign_with(signer, iana::Algorithm::EdDSA)
            .encode()
            .expect("Encoding failed");

        // Decode
        let result = Decoder::new(&qr_data)
            .verify_with(verifier)
            .decode()
            .expect("Decoding failed");

        assert_eq!(result.claim169.id, claim.id);
        assert_eq!(
            result.verification_status,
            claim169_core::VerificationStatus::Verified
        );
    }
}
```
