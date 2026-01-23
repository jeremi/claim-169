//! WebAssembly bindings for MOSIP Claim 169 QR encoder and decoder
//!
//! This module provides WASM bindings using wasm-bindgen for the claim169-core library.
//! It supports encoding (signing, encryption) and decoding of Claim 169 QR codes.
//!
//! ## Custom Crypto Providers
//!
//! For integration with external crypto providers (HSM, cloud KMS, remote signing services,
//! smart cards, TPMs, etc.), the decoder and encoder support custom callback functions:
//!
//! - `verifyWith(callback)` - Custom signature verification
//! - `decryptWith(callback)` - Custom decryption
//! - `signWith(callback, algorithm)` - Custom signing
//! - `encryptWith(callback, algorithm)` - Custom encryption

use js_sys::{Function, Uint8Array};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use claim169_core::crypto::traits::{
    Decryptor as CoreDecryptor, Encryptor as CoreEncryptor,
    SignatureVerifier as CoreSignatureVerifier, Signer as CoreSigner,
};
use claim169_core::error::{CryptoError, CryptoResult};
use claim169_core::model::{
    Biometric as CoreBiometric, Claim169 as CoreClaim169, CwtMeta as CoreCwtMeta,
};
use claim169_core::{Decoder, Encoder};
use coset::iana;

// Install panic hook on module load
#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

// ============================================================================
// JavaScript Error Types
// ============================================================================

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

// ============================================================================
// JavaScript Callback Wrappers for Custom Crypto Providers
// ============================================================================

/// Convert algorithm enum to JavaScript string
fn algorithm_to_string(alg: iana::Algorithm) -> String {
    match alg {
        iana::Algorithm::EdDSA => "EdDSA".to_string(),
        iana::Algorithm::ES256 => "ES256".to_string(),
        iana::Algorithm::A128GCM => "A128GCM".to_string(),
        iana::Algorithm::A256GCM => "A256GCM".to_string(),
        other => format!("{:?}", other),
    }
}

/// JavaScript signature verifier callback wrapper.
///
/// Calls a JavaScript function with signature:
/// `function(algorithm: string, keyId: Uint8Array | null, data: Uint8Array, signature: Uint8Array): void`
///
/// Throws if verification fails.
struct JsSignatureVerifier {
    callback: Function,
}

impl CoreSignatureVerifier for JsSignatureVerifier {
    fn verify(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
        signature: &[u8],
    ) -> CryptoResult<()> {
        let alg_str = algorithm_to_string(algorithm);
        let key_id_js: JsValue = match key_id {
            Some(k) => Uint8Array::from(k).into(),
            None => JsValue::NULL,
        };
        let data_js = Uint8Array::from(data);
        let sig_js = Uint8Array::from(signature);

        let result = self.callback.call4(
            &JsValue::UNDEFINED,
            &JsValue::from_str(&alg_str),
            &key_id_js,
            &data_js.into(),
            &sig_js.into(),
        );

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = e
                    .as_string()
                    .unwrap_or_else(|| "verification callback failed".to_string());
                Err(CryptoError::Other(msg))
            }
        }
    }
}

// Implement Send + Sync for WASM single-threaded context
unsafe impl Send for JsSignatureVerifier {}
unsafe impl Sync for JsSignatureVerifier {}

/// JavaScript decryptor callback wrapper.
///
/// Calls a JavaScript function with signature:
/// `function(algorithm: string, keyId: Uint8Array | null, nonce: Uint8Array, aad: Uint8Array, ciphertext: Uint8Array): Uint8Array`
///
/// Returns the decrypted plaintext.
struct JsDecryptor {
    callback: Function,
}

impl CoreDecryptor for JsDecryptor {
    fn decrypt(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        ciphertext: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        let alg_str = algorithm_to_string(algorithm);
        let key_id_js: JsValue = match key_id {
            Some(k) => Uint8Array::from(k).into(),
            None => JsValue::NULL,
        };
        let nonce_js = Uint8Array::from(nonce);
        let aad_js = Uint8Array::from(aad);
        let ct_js = Uint8Array::from(ciphertext);

        let result = self.callback.call5(
            &JsValue::UNDEFINED,
            &JsValue::from_str(&alg_str),
            &key_id_js,
            &nonce_js.into(),
            &aad_js.into(),
            &ct_js.into(),
        );

        match result {
            Ok(val) => {
                let arr = Uint8Array::from(val);
                Ok(arr.to_vec())
            }
            Err(e) => {
                let msg = e
                    .as_string()
                    .unwrap_or_else(|| "decryption callback failed".to_string());
                Err(CryptoError::DecryptionFailed(msg))
            }
        }
    }
}

unsafe impl Send for JsDecryptor {}
unsafe impl Sync for JsDecryptor {}

/// JavaScript signer callback wrapper.
///
/// Calls a JavaScript function with signature:
/// `function(algorithm: string, keyId: Uint8Array | null, data: Uint8Array): Uint8Array`
///
/// Returns the signature bytes.
struct JsSigner {
    callback: Function,
    key_id: Option<Vec<u8>>,
}

impl CoreSigner for JsSigner {
    fn sign(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        let alg_str = algorithm_to_string(algorithm);
        let key_id_js: JsValue = match key_id {
            Some(k) => Uint8Array::from(k).into(),
            None => JsValue::NULL,
        };
        let data_js = Uint8Array::from(data);

        let result = self.callback.call3(
            &JsValue::UNDEFINED,
            &JsValue::from_str(&alg_str),
            &key_id_js,
            &data_js.into(),
        );

        match result {
            Ok(val) => {
                let arr = Uint8Array::from(val);
                Ok(arr.to_vec())
            }
            Err(e) => {
                let msg = e
                    .as_string()
                    .unwrap_or_else(|| "signer callback failed".to_string());
                Err(CryptoError::SigningFailed(msg))
            }
        }
    }

    fn key_id(&self) -> Option<&[u8]> {
        self.key_id.as_deref()
    }
}

unsafe impl Send for JsSigner {}
unsafe impl Sync for JsSigner {}

/// JavaScript encryptor callback wrapper.
///
/// Calls a JavaScript function with signature:
/// `function(algorithm: string, keyId: Uint8Array | null, nonce: Uint8Array, aad: Uint8Array, plaintext: Uint8Array): Uint8Array`
///
/// Returns the ciphertext with auth tag.
struct JsEncryptor {
    callback: Function,
}

impl CoreEncryptor for JsEncryptor {
    fn encrypt(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        plaintext: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        let alg_str = algorithm_to_string(algorithm);
        let key_id_js: JsValue = match key_id {
            Some(k) => Uint8Array::from(k).into(),
            None => JsValue::NULL,
        };
        let nonce_js = Uint8Array::from(nonce);
        let aad_js = Uint8Array::from(aad);
        let pt_js = Uint8Array::from(plaintext);

        let result = self.callback.call5(
            &JsValue::UNDEFINED,
            &JsValue::from_str(&alg_str),
            &key_id_js,
            &nonce_js.into(),
            &aad_js.into(),
            &pt_js.into(),
        );

        match result {
            Ok(val) => {
                let arr = Uint8Array::from(val);
                Ok(arr.to_vec())
            }
            Err(e) => {
                let msg = e
                    .as_string()
                    .unwrap_or_else(|| "encryptor callback failed".to_string());
                Err(CryptoError::EncryptionFailed(msg))
            }
        }
    }
}

unsafe impl Send for JsEncryptor {}
unsafe impl Sync for JsEncryptor {}

// ============================================================================
// Data Structures
// ============================================================================

/// Biometric data from Claim 169
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsBiometric {
    pub data: Vec<u8>,
    pub format: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_format: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
}

impl From<&CoreBiometric> for JsBiometric {
    fn from(b: &CoreBiometric) -> Self {
        JsBiometric {
            data: b.data.clone(),
            format: b.format.map(|f| f as i64).unwrap_or(0),
            sub_format: b.sub_format.as_ref().map(|s| s.to_value()),
            issuer: b.issuer.clone(),
        }
    }
}

/// CWT metadata
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsCwtMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<i64>,
}

impl From<&CoreCwtMeta> for JsCwtMeta {
    fn from(m: &CoreCwtMeta) -> Self {
        JsCwtMeta {
            issuer: m.issuer.clone(),
            subject: m.subject.clone(),
            expires_at: m.expires_at,
            not_before: m.not_before,
            issued_at: m.issued_at,
        }
    }
}

/// Claim 169 identity data
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsClaim169 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardian: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_format: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_quality_fingers: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_of_issuance: Option<String>,

    // Biometrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_thumb: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_pointer_finger: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_middle_finger: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_ring_finger: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_little_finger: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_thumb: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_pointer_finger: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_middle_finger: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_ring_finger: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_little_finger: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_iris: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_iris: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_palm: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_palm: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Vec<JsBiometric>>,
}

fn convert_biometrics(biometrics: &Option<Vec<CoreBiometric>>) -> Option<Vec<JsBiometric>> {
    biometrics
        .as_ref()
        .map(|v| v.iter().map(JsBiometric::from).collect())
}

impl From<&CoreClaim169> for JsClaim169 {
    fn from(c: &CoreClaim169) -> Self {
        JsClaim169 {
            id: c.id.clone(),
            version: c.version.clone(),
            language: c.language.clone(),
            full_name: c.full_name.clone(),
            first_name: c.first_name.clone(),
            middle_name: c.middle_name.clone(),
            last_name: c.last_name.clone(),
            date_of_birth: c.date_of_birth.clone(),
            gender: c.gender.map(|g| g as i64),
            address: c.address.clone(),
            email: c.email.clone(),
            phone: c.phone.clone(),
            nationality: c.nationality.clone(),
            marital_status: c.marital_status.map(|m| m as i64),
            guardian: c.guardian.clone(),
            photo: c.photo.clone(),
            photo_format: c.photo_format.map(|f| f as i64),
            best_quality_fingers: c.best_quality_fingers.clone(),
            secondary_full_name: c.secondary_full_name.clone(),
            secondary_language: c.secondary_language.clone(),
            location_code: c.location_code.clone(),
            legal_status: c.legal_status.clone(),
            country_of_issuance: c.country_of_issuance.clone(),
            right_thumb: convert_biometrics(&c.right_thumb),
            right_pointer_finger: convert_biometrics(&c.right_pointer_finger),
            right_middle_finger: convert_biometrics(&c.right_middle_finger),
            right_ring_finger: convert_biometrics(&c.right_ring_finger),
            right_little_finger: convert_biometrics(&c.right_little_finger),
            left_thumb: convert_biometrics(&c.left_thumb),
            left_pointer_finger: convert_biometrics(&c.left_pointer_finger),
            left_middle_finger: convert_biometrics(&c.left_middle_finger),
            left_ring_finger: convert_biometrics(&c.left_ring_finger),
            left_little_finger: convert_biometrics(&c.left_little_finger),
            right_iris: convert_biometrics(&c.right_iris),
            left_iris: convert_biometrics(&c.left_iris),
            face: convert_biometrics(&c.face),
            right_palm: convert_biometrics(&c.right_palm),
            left_palm: convert_biometrics(&c.left_palm),
            voice: convert_biometrics(&c.voice),
        }
    }
}

/// Decode result
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsDecodeResult {
    pub claim169: JsClaim169,
    pub cwt_meta: JsCwtMeta,
    pub verification_status: String,
}

// ============================================================================
// WASM Decoder Builder
// ============================================================================

/// Internal state for decoder verification configuration
enum VerifyConfig {
    None,
    Ed25519(Vec<u8>),
    EcdsaP256(Vec<u8>),
    Custom(JsSignatureVerifier),
    Unverified,
}

/// Internal state for decoder decryption configuration
enum DecryptConfig {
    Software { key: Vec<u8>, algorithm: DecryptAlgorithm },
    Custom(JsDecryptor),
}

#[derive(Clone, Copy)]
enum DecryptAlgorithm {
    Aes128Gcm,
    Aes256Gcm,
}

/// Decoder builder for Claim 169 credentials
#[wasm_bindgen]
pub struct WasmDecoder {
    qr_text: String,
    skip_biometrics: bool,
    validate_timestamps: bool,
    clock_skew_tolerance_seconds: i64,
    max_decompressed_bytes: usize,
    verify_config: VerifyConfig,
    decrypt_config: Option<DecryptConfig>,
}

#[wasm_bindgen]
impl WasmDecoder {
    #[wasm_bindgen(constructor)]
    pub fn new(qr_text: &str) -> WasmDecoder {
        WasmDecoder {
            qr_text: qr_text.to_string(),
            skip_biometrics: false,
            validate_timestamps: false, // WASM doesn't have reliable system time
            clock_skew_tolerance_seconds: 0,
            max_decompressed_bytes: 65536,
            verify_config: VerifyConfig::None,
            decrypt_config: None,
        }
    }

    /// Verify with Ed25519 public key (32 bytes)
    #[wasm_bindgen(js_name = "verifyWithEd25519")]
    pub fn verify_with_ed25519(mut self, public_key: &[u8]) -> Result<WasmDecoder, JsError> {
        if public_key.len() != 32 {
            return Err(JsError::new("Ed25519 public key must be 32 bytes"));
        }
        self.verify_config = VerifyConfig::Ed25519(public_key.to_vec());
        Ok(self)
    }

    /// Verify with ECDSA P-256 public key (33 or 65 bytes, SEC1 encoded)
    #[wasm_bindgen(js_name = "verifyWithEcdsaP256")]
    pub fn verify_with_ecdsa_p256(mut self, public_key: &[u8]) -> Result<WasmDecoder, JsError> {
        if public_key.len() != 33 && public_key.len() != 65 {
            return Err(JsError::new(
                "ECDSA P-256 public key must be 33 bytes (compressed) or 65 bytes (uncompressed)",
            ));
        }
        self.verify_config = VerifyConfig::EcdsaP256(public_key.to_vec());
        Ok(self)
    }

    /// Allow decoding without signature verification (INSECURE - testing only)
    #[wasm_bindgen(js_name = "allowUnverified")]
    pub fn allow_unverified(mut self) -> WasmDecoder {
        self.verify_config = VerifyConfig::Unverified;
        self
    }

    /// Decrypt with AES-256-GCM (32-byte key)
    #[wasm_bindgen(js_name = "decryptWithAes256")]
    pub fn decrypt_with_aes256(mut self, key: &[u8]) -> Result<WasmDecoder, JsError> {
        if key.len() != 32 {
            return Err(JsError::new("AES-256 key must be 32 bytes"));
        }
        self.decrypt_config = Some(DecryptConfig::Software {
            key: key.to_vec(),
            algorithm: DecryptAlgorithm::Aes256Gcm,
        });
        Ok(self)
    }

    /// Decrypt with AES-128-GCM (16-byte key)
    #[wasm_bindgen(js_name = "decryptWithAes128")]
    pub fn decrypt_with_aes128(mut self, key: &[u8]) -> Result<WasmDecoder, JsError> {
        if key.len() != 16 {
            return Err(JsError::new("AES-128 key must be 16 bytes"));
        }
        self.decrypt_config = Some(DecryptConfig::Software {
            key: key.to_vec(),
            algorithm: DecryptAlgorithm::Aes128Gcm,
        });
        Ok(self)
    }

    /// Verify with a custom verifier callback.
    ///
    /// Use this for integration with external crypto providers (HSM, cloud KMS,
    /// remote signing services, smart cards, TPMs, etc.).
    ///
    /// The callback signature is:
    /// `function(algorithm: string, keyId: Uint8Array | null, data: Uint8Array, signature: Uint8Array): void`
    ///
    /// The callback should throw an error if verification fails.
    #[wasm_bindgen(js_name = "verifyWith")]
    pub fn verify_with(mut self, verifier: Function) -> WasmDecoder {
        self.verify_config = VerifyConfig::Custom(JsSignatureVerifier { callback: verifier });
        self
    }

    /// Decrypt with a custom decryptor callback.
    ///
    /// Use this for integration with external crypto providers (HSM, cloud KMS,
    /// remote signing services, smart cards, TPMs, etc.).
    ///
    /// The callback signature is:
    /// `function(algorithm: string, keyId: Uint8Array | null, nonce: Uint8Array, aad: Uint8Array, ciphertext: Uint8Array): Uint8Array`
    ///
    /// The callback should return the decrypted plaintext.
    #[wasm_bindgen(js_name = "decryptWith")]
    pub fn decrypt_with(mut self, decryptor: Function) -> WasmDecoder {
        self.decrypt_config = Some(DecryptConfig::Custom(JsDecryptor { callback: decryptor }));
        self
    }

    #[wasm_bindgen(js_name = "skipBiometrics")]
    pub fn skip_biometrics(mut self) -> WasmDecoder {
        self.skip_biometrics = true;
        self
    }

    #[wasm_bindgen(js_name = "withTimestampValidation")]
    pub fn with_timestamp_validation(mut self) -> WasmDecoder {
        self.validate_timestamps = true;
        self
    }

    #[wasm_bindgen(js_name = "clockSkewTolerance")]
    pub fn clock_skew_tolerance(mut self, seconds: i32) -> WasmDecoder {
        self.clock_skew_tolerance_seconds = i64::from(seconds).max(0);
        self
    }

    #[wasm_bindgen(js_name = "maxDecompressedBytes")]
    pub fn max_decompressed_bytes(mut self, bytes: usize) -> WasmDecoder {
        self.max_decompressed_bytes = bytes;
        self
    }

    /// Decode the QR code with configured verification and decryption
    pub fn decode(self) -> Result<JsValue, JsError> {
        let mut decoder =
            Decoder::new(&self.qr_text).max_decompressed_bytes(self.max_decompressed_bytes);

        // Apply decryption configuration
        if let Some(decrypt_config) = self.decrypt_config {
            decoder = match decrypt_config {
                DecryptConfig::Software { key, algorithm } => match algorithm {
                    DecryptAlgorithm::Aes256Gcm => decoder
                        .decrypt_with_aes256(&key)
                        .map_err(|e| JsError::new(&e.to_string()))?,
                    DecryptAlgorithm::Aes128Gcm => decoder
                        .decrypt_with_aes128(&key)
                        .map_err(|e| JsError::new(&e.to_string()))?,
                },
                DecryptConfig::Custom(decryptor) => decoder.decrypt_with(decryptor),
            };
        }

        // Apply verification configuration
        decoder = match self.verify_config {
            VerifyConfig::None => {
                return Err(JsError::new(
                    "Must call verifyWithEd25519(), verifyWithEcdsaP256(), verifyWith(), or allowUnverified() before decode()",
                ));
            }
            VerifyConfig::Ed25519(key) => decoder
                .verify_with_ed25519(&key)
                .map_err(|e| JsError::new(&e.to_string()))?,
            VerifyConfig::EcdsaP256(key) => decoder
                .verify_with_ecdsa_p256(&key)
                .map_err(|e| JsError::new(&e.to_string()))?,
            VerifyConfig::Custom(verifier) => decoder.verify_with(verifier),
            VerifyConfig::Unverified => decoder.allow_unverified(),
        };

        if self.skip_biometrics {
            decoder = decoder.skip_biometrics();
        }

        if !self.validate_timestamps {
            decoder = decoder.without_timestamp_validation();
        } else {
            decoder = decoder.clock_skew_tolerance(self.clock_skew_tolerance_seconds);
        }

        let result = decoder.decode().map_err(|e| JsError::new(&e.to_string()))?;

        let js_result = JsDecodeResult {
            claim169: JsClaim169::from(&result.claim169),
            cwt_meta: JsCwtMeta::from(&result.cwt_meta),
            verification_status: format!("{}", result.verification_status),
        };

        serde_wasm_bindgen::to_value(&js_result).map_err(|e| JsError::new(&e.to_string()))
    }
}

// ============================================================================
// JS Input Types for Encoder
// ============================================================================

/// Input Claim 169 data from JavaScript
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsClaim169Input {
    id: Option<String>,
    version: Option<String>,
    language: Option<String>,
    full_name: Option<String>,
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
    date_of_birth: Option<String>,
    gender: Option<i64>,
    address: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    nationality: Option<String>,
    marital_status: Option<i64>,
    guardian: Option<String>,
    photo: Option<Vec<u8>>,
    photo_format: Option<i64>,
    secondary_full_name: Option<String>,
    secondary_language: Option<String>,
    location_code: Option<String>,
    legal_status: Option<String>,
    country_of_issuance: Option<String>,
}

impl From<JsClaim169Input> for CoreClaim169 {
    fn from(js: JsClaim169Input) -> Self {
        use claim169_core::model::{Gender, MaritalStatus, PhotoFormat};

        CoreClaim169 {
            id: js.id,
            version: js.version,
            language: js.language,
            full_name: js.full_name,
            first_name: js.first_name,
            middle_name: js.middle_name,
            last_name: js.last_name,
            date_of_birth: js.date_of_birth,
            gender: js.gender.and_then(|g| match g {
                1 => Some(Gender::Male),
                2 => Some(Gender::Female),
                3 => Some(Gender::Other),
                _ => None,
            }),
            address: js.address,
            email: js.email,
            phone: js.phone,
            nationality: js.nationality,
            marital_status: js.marital_status.and_then(|m| match m {
                1 => Some(MaritalStatus::Unmarried),
                2 => Some(MaritalStatus::Married),
                3 => Some(MaritalStatus::Divorced),
                _ => None,
            }),
            guardian: js.guardian,
            photo: js.photo,
            photo_format: js.photo_format.and_then(|f| match f {
                1 => Some(PhotoFormat::Jpeg),
                2 => Some(PhotoFormat::Jpeg2000),
                3 => Some(PhotoFormat::Avif),
                4 => Some(PhotoFormat::Webp),
                _ => None,
            }),
            secondary_full_name: js.secondary_full_name,
            secondary_language: js.secondary_language,
            location_code: js.location_code,
            legal_status: js.legal_status,
            country_of_issuance: js.country_of_issuance,
            // Biometrics not supported in WASM encoder for now
            ..Default::default()
        }
    }
}

/// Input CWT metadata from JavaScript
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsCwtMetaInput {
    issuer: Option<String>,
    subject: Option<String>,
    expires_at: Option<i64>,
    not_before: Option<i64>,
    issued_at: Option<i64>,
}

impl From<JsCwtMetaInput> for CoreCwtMeta {
    fn from(js: JsCwtMetaInput) -> Self {
        CoreCwtMeta {
            issuer: js.issuer,
            subject: js.subject,
            expires_at: js.expires_at,
            not_before: js.not_before,
            issued_at: js.issued_at,
        }
    }
}

// ============================================================================
// WASM Encoder Builder
// ============================================================================

/// Internal state for encoder signing configuration
enum SignConfig {
    None,
    Ed25519(Vec<u8>),
    EcdsaP256(Vec<u8>),
    Custom { signer: JsSigner, algorithm: iana::Algorithm },
    Unsigned,
}

/// Internal state for encoder encryption configuration
enum WasmEncryptConfig {
    Software { key: Vec<u8>, algorithm: EncryptAlgorithm },
    Custom { encryptor: JsEncryptor, algorithm: iana::Algorithm },
}

#[derive(Clone, Copy)]
enum EncryptAlgorithm {
    Aes128Gcm,
    Aes256Gcm,
}

/// Encoder builder for creating Claim 169 QR codes
#[wasm_bindgen]
pub struct WasmEncoder {
    claim169: CoreClaim169,
    cwt_meta: CoreCwtMeta,
    sign_config: SignConfig,
    encrypt_config: Option<WasmEncryptConfig>,
    skip_biometrics: bool,
}

#[wasm_bindgen]
impl WasmEncoder {
    /// Create a new encoder with the given claim and CWT metadata.
    ///
    /// @param claim169 - The identity claim data to encode
    /// @param cwtMeta - CWT metadata including issuer, expiration, etc.
    #[wasm_bindgen(constructor)]
    pub fn new(claim169: JsValue, cwt_meta: JsValue) -> Result<WasmEncoder, JsError> {
        let js_claim: JsClaim169Input = serde_wasm_bindgen::from_value(claim169)
            .map_err(|e| JsError::new(&format!("Invalid claim169: {}", e)))?;
        let js_meta: JsCwtMetaInput = serde_wasm_bindgen::from_value(cwt_meta)
            .map_err(|e| JsError::new(&format!("Invalid cwtMeta: {}", e)))?;

        Ok(WasmEncoder {
            claim169: js_claim.into(),
            cwt_meta: js_meta.into(),
            sign_config: SignConfig::None,
            encrypt_config: None,
            skip_biometrics: false,
        })
    }

    /// Sign with Ed25519 private key (32 bytes)
    #[wasm_bindgen(js_name = "signWithEd25519")]
    pub fn sign_with_ed25519(mut self, private_key: &[u8]) -> Result<WasmEncoder, JsError> {
        if private_key.len() != 32 {
            return Err(JsError::new("Ed25519 private key must be 32 bytes"));
        }
        self.sign_config = SignConfig::Ed25519(private_key.to_vec());
        Ok(self)
    }

    /// Sign with ECDSA P-256 private key (32 bytes)
    #[wasm_bindgen(js_name = "signWithEcdsaP256")]
    pub fn sign_with_ecdsa_p256(mut self, private_key: &[u8]) -> Result<WasmEncoder, JsError> {
        if private_key.len() != 32 {
            return Err(JsError::new("ECDSA P-256 private key must be 32 bytes"));
        }
        self.sign_config = SignConfig::EcdsaP256(private_key.to_vec());
        Ok(self)
    }

    /// Encrypt with AES-256-GCM (32-byte key)
    #[wasm_bindgen(js_name = "encryptWithAes256")]
    pub fn encrypt_with_aes256(mut self, key: &[u8]) -> Result<WasmEncoder, JsError> {
        if key.len() != 32 {
            return Err(JsError::new("AES-256 key must be 32 bytes"));
        }
        self.encrypt_config = Some(WasmEncryptConfig::Software {
            key: key.to_vec(),
            algorithm: EncryptAlgorithm::Aes256Gcm,
        });
        Ok(self)
    }

    /// Encrypt with AES-128-GCM (16-byte key)
    #[wasm_bindgen(js_name = "encryptWithAes128")]
    pub fn encrypt_with_aes128(mut self, key: &[u8]) -> Result<WasmEncoder, JsError> {
        if key.len() != 16 {
            return Err(JsError::new("AES-128 key must be 16 bytes"));
        }
        self.encrypt_config = Some(WasmEncryptConfig::Software {
            key: key.to_vec(),
            algorithm: EncryptAlgorithm::Aes128Gcm,
        });
        Ok(self)
    }

    /// Sign with a custom signer callback.
    ///
    /// Use this for integration with external crypto providers (HSM, cloud KMS,
    /// remote signing services, smart cards, TPMs, etc.).
    ///
    /// The callback signature is:
    /// `function(algorithm: string, keyId: Uint8Array | null, data: Uint8Array): Uint8Array`
    ///
    /// The callback should return the signature bytes.
    ///
    /// @param signer - The signing callback function
    /// @param algorithm - "EdDSA" or "ES256"
    #[wasm_bindgen(js_name = "signWith")]
    pub fn sign_with(
        mut self,
        signer: Function,
        algorithm: &str,
    ) -> Result<WasmEncoder, JsError> {
        let alg = match algorithm {
            "EdDSA" => iana::Algorithm::EdDSA,
            "ES256" => iana::Algorithm::ES256,
            _ => {
                return Err(JsError::new(
                    "Unsupported sign algorithm. Use 'EdDSA' or 'ES256'",
                ))
            }
        };
        self.sign_config = SignConfig::Custom {
            signer: JsSigner {
                callback: signer,
                key_id: None,
            },
            algorithm: alg,
        };
        Ok(self)
    }

    /// Encrypt with a custom encryptor callback.
    ///
    /// Use this for integration with external crypto providers (HSM, cloud KMS,
    /// remote signing services, smart cards, TPMs, etc.).
    ///
    /// The callback signature is:
    /// `function(algorithm: string, keyId: Uint8Array | null, nonce: Uint8Array, aad: Uint8Array, plaintext: Uint8Array): Uint8Array`
    ///
    /// The callback should return the ciphertext with auth tag.
    ///
    /// @param encryptor - The encryption callback function
    /// @param algorithm - "A256GCM" or "A128GCM"
    #[wasm_bindgen(js_name = "encryptWith")]
    pub fn encrypt_with(
        mut self,
        encryptor: Function,
        algorithm: &str,
    ) -> Result<WasmEncoder, JsError> {
        let alg = match algorithm {
            "A256GCM" => iana::Algorithm::A256GCM,
            "A128GCM" => iana::Algorithm::A128GCM,
            _ => {
                return Err(JsError::new(
                    "Unsupported encrypt algorithm. Use 'A256GCM' or 'A128GCM'",
                ))
            }
        };
        self.encrypt_config = Some(WasmEncryptConfig::Custom {
            encryptor: JsEncryptor { callback: encryptor },
            algorithm: alg,
        });
        Ok(self)
    }

    /// Allow encoding without a signature (INSECURE - testing only)
    #[wasm_bindgen(js_name = "allowUnsigned")]
    pub fn allow_unsigned(mut self) -> WasmEncoder {
        self.sign_config = SignConfig::Unsigned;
        self
    }

    /// Skip biometric fields during encoding
    #[wasm_bindgen(js_name = "skipBiometrics")]
    pub fn skip_biometrics(mut self) -> WasmEncoder {
        self.skip_biometrics = true;
        self
    }

    /// Encode the credential to a Base45 QR string
    pub fn encode(self) -> Result<String, JsError> {
        let mut encoder = Encoder::new(self.claim169, self.cwt_meta);

        if self.skip_biometrics {
            encoder = encoder.skip_biometrics();
        }

        // Apply signing
        encoder = match self.sign_config {
            SignConfig::None => {
                return Err(JsError::new(
                    "Must call signWithEd25519(), signWithEcdsaP256(), signWith(), or allowUnsigned() before encode()",
                ));
            }
            SignConfig::Ed25519(key) => encoder
                .sign_with_ed25519(&key)
                .map_err(|e| JsError::new(&e.to_string()))?,
            SignConfig::EcdsaP256(key) => encoder
                .sign_with_ecdsa_p256(&key)
                .map_err(|e| JsError::new(&e.to_string()))?,
            SignConfig::Custom { signer, algorithm } => encoder.sign_with(signer, algorithm),
            SignConfig::Unsigned => encoder.allow_unsigned(),
        };

        // Apply encryption
        if let Some(encrypt_config) = self.encrypt_config {
            encoder = match encrypt_config {
                WasmEncryptConfig::Software { key, algorithm } => match algorithm {
                    EncryptAlgorithm::Aes256Gcm => encoder
                        .encrypt_with_aes256(&key)
                        .map_err(|e| JsError::new(&e.to_string()))?,
                    EncryptAlgorithm::Aes128Gcm => encoder
                        .encrypt_with_aes128(&key)
                        .map_err(|e| JsError::new(&e.to_string()))?,
                },
                WasmEncryptConfig::Custom { encryptor, algorithm } => {
                    encoder.encrypt_with(encryptor, algorithm)
                }
            };
        }

        encoder.encode().map_err(|e| JsError::new(&e.to_string()))
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Get the library version
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Check if the WASM module is loaded correctly
#[wasm_bindgen(js_name = "isLoaded")]
pub fn is_loaded() -> bool {
    true
}

/// Generate a random 12-byte nonce for AES-GCM encryption
#[wasm_bindgen(js_name = "generateNonce")]
pub fn generate_nonce() -> Vec<u8> {
    claim169_core::generate_random_nonce().to_vec()
}
