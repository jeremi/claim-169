# Référence API Rust

La documentation API complète est disponible sur [docs.rs/claim169-core](https://docs.rs/claim169-core).

## Features

Par défaut, la crate active le support crypto logiciel :

- `software-crypto` (par défaut) : helpers Ed25519, ECDSA P-256 et AES-GCM

Désactivez les features par défaut si vous souhaitez brancher votre propre crypto (HSM/KMS) via les traits `Signer` / `SignatureVerifier` / `Encryptor` / `Decryptor`.

## Types principaux

### `Decoder`

Builder pour décoder des QR Claim 169 (Base45).

```rust
use claim169_core::Decoder;

let decoder = Decoder::new(qr_text);
```

Méthodes (résumé) :

- `verify_with_ed25519(public_key)` / `verify_with_ed25519_pem(pem)` (avec `software-crypto`)
- `verify_with_ecdsa_p256(public_key)` / `verify_with_ecdsa_p256_pem(pem)` (avec `software-crypto`)
- `verify_with(verifier)` (vérification personnalisée HSM/KMS)
- `decrypt_with_aes256(key)` / `decrypt_with_aes128(key)` (avec `software-crypto`)
- `decrypt_with(decryptor)` (déchiffrement personnalisé)
- `allow_unverified()` (tests uniquement)
- `skip_biometrics()`
- `without_timestamp_validation()`
- `clock_skew_tolerance(seconds)`
- `max_decompressed_bytes(bytes)`
- `decode()`

### `Encoder`

Builder pour encoder des identifiants.

```rust
use claim169_core::{Claim169, CwtMeta, Encoder};

let encoder = Encoder::new(claim169, cwt_meta);
```

Méthodes (résumé) :

- `sign_with_ed25519(private_key)` / `sign_with_ecdsa_p256(private_key)` (avec `software-crypto`)
- `sign_with(signer, algorithm)` (signature personnalisée HSM/KMS)
- `allow_unsigned()` (tests uniquement)
- `encrypt_with_aes256(key)` / `encrypt_with_aes128(key)` (avec `software-crypto`)
- `encrypt_with(encryptor, algorithm)` (chiffrement personnalisé)
- `encrypt_with_aes256_nonce(key, nonce)` / `encrypt_with_aes128_nonce(key, nonce)` (tests uniquement)
- `skip_biometrics()`
- `encode()`

### `DecodeResult`

Le résultat de décodage contient :

- `claim169` (données d’identité)
- `cwt_meta` (issuer, exp/nbf/iat…)
- `verification_status` (`verified` / `skipped` / `failed`)
- `warnings` (non bloquants)

## Erreurs

Les opérations renvoient `claim169_core::Result<T>` (alias `Result<T, Claim169Error>`).

Cas fréquents :

- `DecodingConfig(...)` (ni vérification ni `allow_unverified()`)
- `EncodingConfig(...)` (ni signature ni `allow_unsigned()`)
- `SignatureInvalid(...)`, `DecryptionFailed(...)`
- `Expired(ts)` / `NotYetValid(ts)`
- `DecompressLimitExceeded { max_bytes }`

## Exemple

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

## Fournisseurs Cryptographiques Personnalisés

La bibliothèque fournit des traits pour intégrer des fournisseurs cryptographiques externes. Cela permet l'utilisation avec :

- **Modules de Sécurité Matériels (HSM)** - Thales, AWS CloudHSM, Azure Dedicated HSM
- **KMS Cloud** - AWS KMS, Google Cloud KMS, Azure Key Vault
- **Cartes à Puce & TPM** - Dispositifs PKCS#11, cartes PIV
- **Services de Signature à Distance** - DigiCert, DocuSign, APIs de signature personnalisées

Tous les traits requièrent `Send + Sync` pour une utilisation thread-safe.

### Trait SignatureVerifier

Utilisé lors de la vérification des identifiants pour valider les signatures COSE_Sign1.

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

### Trait Signer

Utilisé lors de l'émission des identifiants pour créer des signatures COSE_Sign1.

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

### Trait Decryptor

Utilisé lors de la vérification des identifiants pour déchiffrer les payloads COSE_Encrypt0.

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

### Trait Encryptor

Utilisé lors de l'émission des identifiants pour chiffrer les payloads avec COSE_Encrypt0.

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

### Exemple : Intégration AWS KMS

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

### Exemple : Intégration HSM PKCS#11

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

### Exemple : Intégration Azure Key Vault

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

### Désactiver le Crypto par Défaut

Pour utiliser uniquement des fournisseurs personnalisés sans le crypto logiciel intégré :

```toml
[dependencies]
claim169-core = { version = "0.1", default-features = false }
```

Cela supprime la feature `software-crypto` et ses dépendances (ed25519-dalek, p256, aes-gcm), réduisant la taille du binaire pour les déploiements embarqués ou exclusivement HSM.
