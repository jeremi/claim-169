# Référence API

Documentation API complète pour le crate `claim169-core`.

## Types principaux

### Encoder

Builder pour encoder des identifiants Claim 169 en chaînes prêtes pour QR.

```rust
pub struct Encoder { /* private fields */ }

impl Encoder {
    /// Create a new encoder with claim data and CWT metadata
    pub fn new(claim169: Claim169, cwt_meta: CwtMeta) -> Self;

    // Signing methods
    pub fn sign_with<S: Signer + 'static>(self, signer: S, algorithm: iana::Algorithm) -> Self;
    pub fn sign_with_ed25519(self, private_key: &[u8]) -> Result<Self>;         // 32 bytes
    pub fn sign_with_ecdsa_p256(self, private_key: &[u8]) -> Result<Self>;      // 32 bytes

    // Encryption methods
    pub fn encrypt_with<E: Encryptor + 'static>(self, encryptor: E, algorithm: iana::Algorithm) -> Self;
    pub fn encrypt_with_aes256(self, key: &[u8]) -> Result<Self>;               // 32 bytes
    pub fn encrypt_with_aes128(self, key: &[u8]) -> Result<Self>;               // 16 bytes
    pub fn encrypt_with_aes256_nonce(self, key: &[u8], nonce: &[u8; 12]) -> Result<Self>;
    pub fn encrypt_with_aes128_nonce(self, key: &[u8], nonce: &[u8; 12]) -> Result<Self>;

    // Options
    pub fn allow_unsigned(self) -> Self;
    pub fn skip_biometrics(self) -> Self;

    // Build
    pub fn encode(self) -> Result<String>;
}
```

### Decoder

Builder pour décoder des identifiants Claim 169 depuis des chaînes QR.

```rust
pub struct Decoder { /* private fields */ }

impl Decoder {
    /// Create a new decoder with QR text content
    pub fn new(qr_text: impl Into<String>) -> Self;

    // Verification methods
    pub fn verify_with<V: SignatureVerifier + 'static>(self, verifier: V) -> Self;
    pub fn verify_with_ed25519(self, public_key: &[u8]) -> Result<Self>;        // 32 bytes
    pub fn verify_with_ed25519_pem(self, pem: &str) -> Result<Self>;
    pub fn verify_with_ecdsa_p256(self, public_key: &[u8]) -> Result<Self>;     // 33 or 65 bytes
    pub fn verify_with_ecdsa_p256_pem(self, pem: &str) -> Result<Self>;

    // Decryption methods
    pub fn decrypt_with<D: Decryptor + 'static>(self, decryptor: D) -> Self;
    pub fn decrypt_with_aes256(self, key: &[u8]) -> Result<Self>;               // 32 bytes
    pub fn decrypt_with_aes128(self, key: &[u8]) -> Result<Self>;               // 16 bytes

    // Options
    pub fn allow_unverified(self) -> Self;
    pub fn skip_biometrics(self) -> Self;
    pub fn without_timestamp_validation(self) -> Self;
    pub fn clock_skew_tolerance(self, seconds: i64) -> Self;
    pub fn max_decompressed_bytes(self, bytes: usize) -> Self;                  // default: 65536

    // Build
    pub fn decode(self) -> Result<DecodeResult>;
}
```

### DecodeResult

Résultat d’un décodage Claim 169 réussi.

```rust
pub struct DecodeResult {
    /// The extracted Claim 169 identity data
    pub claim169: Claim169,

    /// CWT metadata (issuer, expiration, etc.)
    pub cwt_meta: CwtMeta,

    /// Signature verification status
    pub verification_status: VerificationStatus,

    /// Non-fatal warnings
    pub warnings: Vec<Warning>,
}
```

### Claim169

Structure principale des données d’identité.

```rust
pub struct Claim169 {
    // Demographics (keys 1-23)
    pub id: Option<String>,                        // key 1
    pub version: Option<String>,                   // key 2
    pub language: Option<String>,                  // key 3 (ISO 639-3)
    pub full_name: Option<String>,                 // key 4
    pub first_name: Option<String>,                // key 5
    pub middle_name: Option<String>,               // key 6
    pub last_name: Option<String>,                 // key 7
    pub date_of_birth: Option<String>,             // key 8 (YYYYMMDD)
    pub gender: Option<Gender>,                    // key 9
    pub address: Option<String>,                   // key 10 (\n separated)
    pub email: Option<String>,                     // key 11
    pub phone: Option<String>,                     // key 12 (E.123)
    pub nationality: Option<String>,               // key 13 (ISO 3166-1/2)
    pub marital_status: Option<MaritalStatus>,     // key 14
    pub guardian: Option<String>,                  // key 15
    pub photo: Option<Vec<u8>>,                    // key 16
    pub photo_format: Option<PhotoFormat>,         // key 17
    pub best_quality_fingers: Option<Vec<u8>>,     // key 18 (positions 0-10)
    pub secondary_full_name: Option<String>,       // key 19
    pub secondary_language: Option<String>,        // key 20 (ISO 639-3)
    pub location_code: Option<String>,             // key 21
    pub legal_status: Option<String>,              // key 22
    pub country_of_issuance: Option<String>,       // key 23

    // Biometrics (keys 50-65)
    pub right_thumb: Option<Vec<Biometric>>,           // key 50
    pub right_pointer_finger: Option<Vec<Biometric>>,  // key 51
    pub right_middle_finger: Option<Vec<Biometric>>,   // key 52
    pub right_ring_finger: Option<Vec<Biometric>>,     // key 53
    pub right_little_finger: Option<Vec<Biometric>>,   // key 54
    pub left_thumb: Option<Vec<Biometric>>,            // key 55
    pub left_pointer_finger: Option<Vec<Biometric>>,   // key 56
    pub left_middle_finger: Option<Vec<Biometric>>,    // key 57
    pub left_ring_finger: Option<Vec<Biometric>>,      // key 58
    pub left_little_finger: Option<Vec<Biometric>>,    // key 59
    pub right_iris: Option<Vec<Biometric>>,            // key 60
    pub left_iris: Option<Vec<Biometric>>,             // key 61
    pub face: Option<Vec<Biometric>>,                  // key 62
    pub right_palm: Option<Vec<Biometric>>,            // key 63
    pub left_palm: Option<Vec<Biometric>>,             // key 64
    pub voice: Option<Vec<Biometric>>,                 // key 65

    // Forward compatibility
    pub unknown_fields: HashMap<i64, serde_json::Value>,
}

impl Claim169 {
    pub fn new() -> Self;
    pub fn minimal(id: impl Into<String>, full_name: impl Into<String>) -> Self;

    // Builder methods for all fields
    pub fn with_id(self, id: impl Into<String>) -> Self;
    pub fn with_full_name(self, full_name: impl Into<String>) -> Self;
    // ... (builder method for each field)

    // Utility methods
    pub fn has_biometrics(&self) -> bool;
    pub fn biometric_count(&self) -> usize;
    pub fn without_biometrics(&self) -> Self;
}
```

### CwtMeta

Métadonnées des claims standards CWT (CBOR Web Token).

```rust
pub struct CwtMeta {
    pub issuer: Option<String>,        // CWT claim 1
    pub subject: Option<String>,       // CWT claim 2
    pub expires_at: Option<i64>,       // CWT claim 4 (Unix timestamp)
    pub not_before: Option<i64>,       // CWT claim 5 (Unix timestamp)
    pub issued_at: Option<i64>,        // CWT claim 6 (Unix timestamp)
}

impl CwtMeta {
    pub fn new() -> Self;

    // Builder methods
    pub fn with_issuer(self, issuer: impl Into<String>) -> Self;
    pub fn with_subject(self, subject: impl Into<String>) -> Self;
    pub fn with_expires_at(self, expires_at: i64) -> Self;
    pub fn with_not_before(self, not_before: i64) -> Self;
    pub fn with_issued_at(self, issued_at: i64) -> Self;

    // Utility methods
    pub fn is_time_valid(&self, current_time: i64) -> bool;
    pub fn is_expired(&self, current_time: i64) -> bool;
    pub fn is_not_yet_valid(&self, current_time: i64) -> bool;
}
```

### Biometric

Une entrée biométrique.

```rust
pub struct Biometric {
    pub data: Vec<u8>,                           // key 0
    pub format: Option<BiometricFormat>,         // key 1
    pub sub_format: Option<BiometricSubFormat>,  // key 2
    pub issuer: Option<String>,                  // key 3
}

impl Biometric {
    pub fn new(data: Vec<u8>) -> Self;
    pub fn with_format(self, format: BiometricFormat) -> Self;
    pub fn with_sub_format(self, sub_format: BiometricSubFormat) -> Self;
    pub fn with_issuer(self, issuer: impl Into<String>) -> Self;
}
```

## Énumérations

### Gender

```rust
pub enum Gender {
    Male = 1,
    Female = 2,
    Other = 3,
}
```

### MaritalStatus

```rust
pub enum MaritalStatus {
    Unmarried = 1,
    Married = 2,
    Divorced = 3,
}
```

### PhotoFormat

```rust
pub enum PhotoFormat {
    Jpeg = 1,
    Jpeg2000 = 2,
    Avif = 3,
    Webp = 4,
}
```

### BiometricFormat

```rust
pub enum BiometricFormat {
    Image = 0,
    Template = 1,
    Sound = 2,
    BioHash = 3,
}
```

### BiometricSubFormat

```rust
pub enum BiometricSubFormat {
    Image(ImageSubFormat),
    Template(TemplateSubFormat),
    Sound(SoundSubFormat),
    Raw(i64),
}

pub enum ImageSubFormat {
    Png = 0,
    Jpeg = 1,
    Jpeg2000 = 2,
    Avif = 3,
    Webp = 4,
    Tiff = 5,
    Wsq = 6,
    VendorSpecific(i64),  // 100-200
}

pub enum TemplateSubFormat {
    Ansi378 = 0,
    Iso19794_2 = 1,
    Nist = 2,
    VendorSpecific(i64),  // 100-200
}

pub enum SoundSubFormat {
    Wav = 0,
    Mp3 = 1,
}
```

### VerificationStatus

```rust
pub enum VerificationStatus {
    Verified,   // Signature verified successfully
    Failed,     // Signature verification failed
    Skipped,    // Verification was skipped (allow_unverified)
}
```

### WarningCode

```rust
pub enum WarningCode {
    ExpiringSoon,               // Credential expires soon
    UnknownFields,              // Unknown CBOR keys found
    TimestampValidationSkipped, // Timestamps not validated
    BiometricsSkipped,          // Biometrics not parsed
}
```

## Types d’erreur

### Claim169Error

```rust
pub enum Claim169Error {
    // Decoding errors
    Base45Decode(String),
    Decompress(String),
    DecompressLimitExceeded { max_bytes: usize },
    CoseParse(String),
    UnsupportedCoseType(String),
    CborParse(String),
    CwtParse(String),
    Claim169NotFound,
    Claim169Invalid(String),

    // Security errors
    SignatureInvalid(String),
    DecryptionFailed(String),
    UnsupportedAlgorithm(String),
    KeyNotFound(Option<Vec<u8>>),

    // Timestamp errors
    Expired(i64),
    NotYetValid(i64),

    // Encoding errors
    CborEncode(String),
    SignatureFailed(String),
    EncryptionFailed(String),

    // Configuration errors
    EncodingConfig(String),
    DecodingConfig(String),

    // Other
    Crypto(String),
    Io(std::io::Error),
}
```

### CryptoError

```rust
pub enum CryptoError {
    InvalidKeyFormat(String),
    KeyNotFound,
    VerificationFailed,
    DecryptionFailed(String),
    SigningFailed(String),
    EncryptionFailed(String),
    UnsupportedAlgorithm(String),
    Other(String),
}
```

## Traits crypto

### SignatureVerifier

```rust
pub trait SignatureVerifier: Send + Sync {
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

```rust
pub trait Signer: Send + Sync {
    fn sign(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
    ) -> CryptoResult<Vec<u8>>;

    fn key_id(&self) -> Option<&[u8]> {
        None
    }
}
```

### Decryptor

```rust
pub trait Decryptor: Send + Sync {
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

```rust
pub trait Encryptor: Send + Sync {
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

### KeyResolver

```rust
pub trait KeyResolver: Send + Sync {
    fn resolve_verifier(
        &self,
        key_id: Option<&[u8]>,
        algorithm: iana::Algorithm,
    ) -> CryptoResult<Box<dyn SignatureVerifier>>;

    fn resolve_decryptor(
        &self,
        key_id: Option<&[u8]>,
        algorithm: iana::Algorithm,
    ) -> CryptoResult<Box<dyn Decryptor>>;
}
```

## Crypto logicielle (feature : software-crypto)

Disponible lorsque le feature `software-crypto` est activé (par défaut).

### Ed25519Signer

```rust
pub struct Ed25519Signer { /* private */ }

impl Ed25519Signer {
    pub fn generate() -> Self;
    pub fn from_bytes(private_key: &[u8]) -> CryptoResult<Self>;  // 32 bytes
    pub fn public_key_bytes(&self) -> [u8; 32];
    pub fn verifying_key(&self) -> Ed25519Verifier;
}

impl Signer for Ed25519Signer { /* ... */ }
```

### Ed25519Verifier

```rust
pub struct Ed25519Verifier { /* private */ }

impl Ed25519Verifier {
    pub fn from_bytes(public_key: &[u8]) -> CryptoResult<Self>;  // 32 bytes
    pub fn from_pem(pem: &str) -> CryptoResult<Self>;
}

impl SignatureVerifier for Ed25519Verifier { /* ... */ }
```

### EcdsaP256Signer

```rust
pub struct EcdsaP256Signer { /* private */ }

impl EcdsaP256Signer {
    pub fn generate() -> Self;
    pub fn from_bytes(private_key: &[u8]) -> CryptoResult<Self>;  // 32 bytes
    pub fn verifying_key(&self) -> EcdsaP256Verifier;
}

impl Signer for EcdsaP256Signer { /* ... */ }
```

### EcdsaP256Verifier

```rust
pub struct EcdsaP256Verifier { /* private */ }

impl EcdsaP256Verifier {
    pub fn from_sec1_bytes(public_key: &[u8]) -> CryptoResult<Self>;  // 33 or 65 bytes
    pub fn from_pem(pem: &str) -> CryptoResult<Self>;
}

impl SignatureVerifier for EcdsaP256Verifier { /* ... */ }
```

### AesGcmEncryptor

```rust
pub struct AesGcmEncryptor { /* private */ }

impl AesGcmEncryptor {
    pub fn aes256(key: &[u8]) -> CryptoResult<Self>;  // 32 bytes
    pub fn aes128(key: &[u8]) -> CryptoResult<Self>;  // 16 bytes
}

impl Encryptor for AesGcmEncryptor { /* ... */ }
```

### AesGcmDecryptor

```rust
pub struct AesGcmDecryptor { /* private */ }

impl AesGcmDecryptor {
    pub fn aes256(key: &[u8]) -> CryptoResult<Self>;  // 32 bytes
    pub fn aes128(key: &[u8]) -> CryptoResult<Self>;  // 16 bytes
}

impl Decryptor for AesGcmDecryptor { /* ... */ }
```

## Fonctions utilitaires

### generate_random_nonce

```rust
/// Generate a random 12-byte nonce for AES-GCM encryption.
/// Available when software-crypto feature is enabled.
pub fn generate_random_nonce() -> [u8; 12];
```

## Alias de types

```rust
/// Result type for Claim 169 operations
pub type Result<T> = std::result::Result<T, Claim169Error>;

/// Result type for crypto operations
pub type CryptoResult<T> = std::result::Result<T, CryptoError>;
```

## Ré-export

Le crate ré-exporte des types courants à la racine pour plus de commodité :

```rust
pub use decode::Decoder;
pub use encode::Encoder;
pub use crypto::traits::{Decryptor, Encryptor, KeyResolver, SignatureVerifier, Signer};
pub use error::{Claim169Error, CryptoError, CryptoResult, Result};
pub use model::{
    Biometric, BiometricFormat, BiometricSubFormat, Claim169, CwtMeta,
    Gender, MaritalStatus, PhotoFormat, VerificationStatus,
};

// When software-crypto is enabled:
pub use crypto::{
    AesGcmDecryptor, AesGcmEncryptor, EcdsaP256Signer, EcdsaP256Verifier,
    Ed25519Signer, Ed25519Verifier,
};
pub use encode::generate_random_nonce;
```
