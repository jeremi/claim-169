# Fournisseurs crypto personnalisés

La bibliothèque `claim169-core` supporte des backends cryptographiques branchables via des traits. Cela permet une intégration avec :

- Hardware Security Modules (HSM)
- Cloud KMS (AWS KMS, Google Cloud KMS, Azure Key Vault)
- Services de signature distants
- Cartes à puce et TPM
- Keystores logiciels personnalisés

## Architecture

La bibliothèque définit quatre traits cryptographiques :

| Trait | Rôle | Utilisé par |
|-------|------|-------------|
| `Signer` | Signer des identifiants | `Encoder` |
| `SignatureVerifier` | Vérifier des signatures | `Decoder` |
| `Encryptor` | Chiffrer des identifiants | `Encoder` |
| `Decryptor` | Déchiffrer des identifiants | `Decoder` |

Tous les traits exigent `Send + Sync` pour la sûreté vis-à-vis des threads.

## Désactiver la crypto logicielle

Pour des déploiements « HSM-only », désactivez les implémentations logicielles par défaut :

```toml
[dependencies]
claim169-core = { version = "0.1", default-features = false }
```

Cela supprime `Ed25519Signer`, `EcdsaP256Signer`, `AesGcmEncryptor`, etc.

## Trait SignatureVerifier {#signatureverifier-trait}

Implémentez `SignatureVerifier` pour une vérification de signature personnalisée :

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
        // Vérifier le support d’algorithme
        let hsm_algo = match algorithm {
            iana::Algorithm::EdDSA => HsmAlgorithm::Ed25519,
            iana::Algorithm::ES256 => HsmAlgorithm::EcdsaP256,
            other => return Err(CryptoError::UnsupportedAlgorithm(
                format!("{:?}", other)
            )),
        };

        // Résoudre la clé via key_id, ou utiliser la clé par défaut
        let key_handle = if let Some(kid) = key_id {
            self.hsm_session.find_key_by_id(kid)
                .map_err(|_| CryptoError::KeyNotFound)?
        } else {
            self.hsm_session.find_key_by_label(&self.key_label)
                .map_err(|_| CryptoError::KeyNotFound)?
        };

        // Vérifier
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

## Trait Signer {#signer-trait}

Implémentez `Signer` pour une signature personnalisée :

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

## Trait Decryptor {#decryptor-trait}

Implémentez `Decryptor` pour un déchiffrement personnalisé :

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
        // Valider l’algorithme
        match algorithm {
            iana::Algorithm::A256GCM | iana::Algorithm::A128GCM => {}
            other => return Err(CryptoError::UnsupportedAlgorithm(
                format!("{:?}", other)
            )),
        }

        // En AES-GCM, ciphertext inclut le tag d’authentification
        // AWS KMS les attend séparément (16 derniers octets = tag)
        if ciphertext.len() < 16 {
            return Err(CryptoError::DecryptionFailed(
                "Ciphertext too short".to_string()
            ));
        }

        let (ct, tag) = ciphertext.split_at(ciphertext.len() - 16);

        // Construire la requête KMS
        let request = DecryptRequest {
            key_id: self.key_arn.clone(),
            ciphertext_blob: ct.to_vec(),
            encryption_algorithm: "AES_256_GCM".to_string(),
            encryption_context: None,
            // Passer nonce et AAD selon votre KMS
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

## Trait Encryptor {#encryptor-trait}

Implémentez `Encryptor` pour un chiffrement personnalisé :

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
            // Passer nonce et AAD
        };

        let response = self.kms_client
            .encrypt(request)
            .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

        // Retourner le ciphertext avec tag d’authentification ajouté
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

## Trait KeyResolver

Pour des scénarios où vous devez résoudre des clés dynamiquement :

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

## Exemple AWS KMS

Exemple complet pour AWS KMS :

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

        // Opération Sign AWS KMS
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
    .sign_with(signer, iana::Algorithm::ES256)  // AWS KMS utilise ECDSA
    .encode()?;
```

## Gestion des erreurs

Retourner les variantes `CryptoError` appropriées :

```rust
use claim169_core::CryptoError;

// Clé introuvable
Err(CryptoError::KeyNotFound)

// Algorithme non supporté
Err(CryptoError::UnsupportedAlgorithm("ChaCha20 not supported".into()))

// Vérification échouée
Err(CryptoError::VerificationFailed)

// Déchiffrement échoué
Err(CryptoError::DecryptionFailed("Authentication tag mismatch".into()))

// Signature échouée
Err(CryptoError::SigningFailed("HSM communication error".into()))

// Chiffrement échoué
Err(CryptoError::EncryptionFailed("Key not suitable for encryption".into()))

// Format de clé invalide
Err(CryptoError::InvalidKeyFormat("Expected 32-byte key".into()))

// Erreur générique
Err(CryptoError::Other("Unexpected error".into()))
```

## Sûreté vis-à-vis des threads

Tous les traits exigent `Send + Sync`. Assurez-vous que vos implémentations sont thread-safe :

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
            // Signer en verrouillant la session
            session.sign(&self.key_label, data)
        })
        .map_err(|e| CryptoError::SigningFailed(e.to_string()))
    }
}
```

## Tester des implémentations personnalisées

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
