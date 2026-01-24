// Allow pyo3 internal cfg checks and macro-generated conversions
#![allow(unexpected_cfgs)]
#![allow(clippy::useless_conversion)]

//! Python bindings for MOSIP Claim 169 QR decoder
//!
//! This module provides Python bindings using PyO3 for the claim169-core library.
//! It supports custom crypto hooks for external crypto providers (HSM, cloud KMS,
//! remote signing services, smart cards, TPMs, etc.).

use pyo3::exceptions::{PyException, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};

use claim169_core::crypto::software::AesGcmDecryptor;
use claim169_core::crypto::traits::{
    Decryptor as CoreDecryptor, Encryptor as CoreEncryptor,
    SignatureVerifier as CoreSignatureVerifier, Signer as CoreSigner,
};
use claim169_core::error::{Claim169Error, CryptoError, CryptoResult};
use claim169_core::model::{
    Biometric as CoreBiometric, Claim169 as CoreClaim169, CwtMeta as CoreCwtMeta, Gender,
    MaritalStatus, PhotoFormat,
};
use claim169_core::{Decoder, Encoder as CoreEncoder};
use coset::iana;

// ============================================================================
// Python Exception Types
// ============================================================================

pyo3::create_exception!(claim169, Claim169Exception, PyException);
pyo3::create_exception!(claim169, Base45DecodeError, Claim169Exception);
pyo3::create_exception!(claim169, DecompressError, Claim169Exception);
pyo3::create_exception!(claim169, CoseParseError, Claim169Exception);
pyo3::create_exception!(claim169, CwtParseError, Claim169Exception);
pyo3::create_exception!(claim169, Claim169NotFoundError, Claim169Exception);
pyo3::create_exception!(claim169, SignatureError, Claim169Exception);
pyo3::create_exception!(claim169, DecryptionError, Claim169Exception);
pyo3::create_exception!(claim169, EncryptionError, Claim169Exception);

fn to_py_err(e: Claim169Error) -> PyErr {
    match e {
        Claim169Error::Base45Decode(_) => Base45DecodeError::new_err(e.to_string()),
        Claim169Error::Decompress(_) => DecompressError::new_err(e.to_string()),
        Claim169Error::DecompressLimitExceeded { .. } => DecompressError::new_err(e.to_string()),
        Claim169Error::CoseParse(_) => CoseParseError::new_err(e.to_string()),
        Claim169Error::CborParse(_) => CoseParseError::new_err(e.to_string()),
        Claim169Error::CwtParse(_) => CwtParseError::new_err(e.to_string()),
        Claim169Error::Claim169NotFound => Claim169NotFoundError::new_err(e.to_string()),
        Claim169Error::SignatureInvalid(_) => SignatureError::new_err(e.to_string()),
        Claim169Error::SignatureFailed(_) => SignatureError::new_err(e.to_string()),
        Claim169Error::Crypto(_) => SignatureError::new_err(e.to_string()),
        Claim169Error::DecryptionFailed(_) => DecryptionError::new_err(e.to_string()),
        Claim169Error::EncryptionFailed(_) => EncryptionError::new_err(e.to_string()),
        _ => Claim169Exception::new_err(e.to_string()),
    }
}

// ============================================================================
// Python Data Classes
// ============================================================================

/// Biometric data extracted from claim 169
#[pyclass]
#[derive(Clone)]
pub struct Biometric {
    #[pyo3(get)]
    pub data: Vec<u8>,
    #[pyo3(get)]
    pub format: Option<i64>,
    #[pyo3(get)]
    pub sub_format: Option<i64>,
    #[pyo3(get)]
    pub issuer: Option<String>,
}

#[pymethods]
impl Biometric {
    fn __repr__(&self) -> String {
        format!(
            "Biometric(format={:?}, sub_format={:?}, data_len={})",
            self.format,
            self.sub_format,
            self.data.len()
        )
    }
}

impl From<&CoreBiometric> for Biometric {
    fn from(b: &CoreBiometric) -> Self {
        Biometric {
            data: b.data.clone(),
            format: b.format.map(|f| f as i64),
            sub_format: b.sub_format.as_ref().map(|s| s.to_value()),
            issuer: b.issuer.clone(),
        }
    }
}

/// CWT metadata (issuer, subject, timestamps)
#[pyclass]
#[derive(Clone)]
pub struct CwtMeta {
    #[pyo3(get)]
    pub issuer: Option<String>,
    #[pyo3(get)]
    pub subject: Option<String>,
    #[pyo3(get)]
    pub expires_at: Option<i64>,
    #[pyo3(get)]
    pub not_before: Option<i64>,
    #[pyo3(get)]
    pub issued_at: Option<i64>,
}

#[pymethods]
impl CwtMeta {
    fn __repr__(&self) -> String {
        format!(
            "CwtMeta(issuer={:?}, subject={:?}, expires_at={:?})",
            self.issuer, self.subject, self.expires_at
        )
    }

    /// Check if the token is currently valid (not expired, not before nbf)
    fn is_valid_now(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        if let Some(exp) = self.expires_at {
            if now > exp {
                return false;
            }
        }
        if let Some(nbf) = self.not_before {
            if now < nbf {
                return false;
            }
        }
        true
    }

    /// Check if the token is expired
    fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        self.expires_at.map(|exp| now > exp).unwrap_or(false)
    }
}

impl From<&CoreCwtMeta> for CwtMeta {
    fn from(m: &CoreCwtMeta) -> Self {
        CwtMeta {
            issuer: m.issuer.clone(),
            subject: m.subject.clone(),
            expires_at: m.expires_at,
            not_before: m.not_before,
            issued_at: m.issued_at,
        }
    }
}

/// Decoded Claim 169 identity data
#[pyclass]
#[derive(Clone)]
pub struct Claim169 {
    // Demographics
    #[pyo3(get)]
    pub id: Option<String>,
    #[pyo3(get)]
    pub version: Option<String>,
    #[pyo3(get)]
    pub language: Option<String>,
    #[pyo3(get)]
    pub full_name: Option<String>,
    #[pyo3(get)]
    pub first_name: Option<String>,
    #[pyo3(get)]
    pub middle_name: Option<String>,
    #[pyo3(get)]
    pub last_name: Option<String>,
    #[pyo3(get)]
    pub date_of_birth: Option<String>,
    #[pyo3(get)]
    pub gender: Option<i64>,
    #[pyo3(get)]
    pub address: Option<String>,
    #[pyo3(get)]
    pub email: Option<String>,
    #[pyo3(get)]
    pub phone: Option<String>,
    #[pyo3(get)]
    pub nationality: Option<String>,
    #[pyo3(get)]
    pub marital_status: Option<i64>,
    #[pyo3(get)]
    pub guardian: Option<String>,
    #[pyo3(get)]
    pub photo: Option<Vec<u8>>,
    #[pyo3(get)]
    pub photo_format: Option<i64>,
    #[pyo3(get)]
    pub best_quality_fingers: Option<Vec<u8>>,
    #[pyo3(get)]
    pub secondary_full_name: Option<String>,
    #[pyo3(get)]
    pub secondary_language: Option<String>,
    #[pyo3(get)]
    pub location_code: Option<String>,
    #[pyo3(get)]
    pub legal_status: Option<String>,
    #[pyo3(get)]
    pub country_of_issuance: Option<String>,

    // Biometrics
    #[pyo3(get)]
    pub right_thumb: Option<Vec<Biometric>>,
    #[pyo3(get)]
    pub right_pointer_finger: Option<Vec<Biometric>>,
    #[pyo3(get)]
    pub right_middle_finger: Option<Vec<Biometric>>,
    #[pyo3(get)]
    pub right_ring_finger: Option<Vec<Biometric>>,
    #[pyo3(get)]
    pub right_little_finger: Option<Vec<Biometric>>,
    #[pyo3(get)]
    pub left_thumb: Option<Vec<Biometric>>,
    #[pyo3(get)]
    pub left_pointer_finger: Option<Vec<Biometric>>,
    #[pyo3(get)]
    pub left_middle_finger: Option<Vec<Biometric>>,
    #[pyo3(get)]
    pub left_ring_finger: Option<Vec<Biometric>>,
    #[pyo3(get)]
    pub left_little_finger: Option<Vec<Biometric>>,
    #[pyo3(get)]
    pub right_iris: Option<Vec<Biometric>>,
    #[pyo3(get)]
    pub left_iris: Option<Vec<Biometric>>,
    #[pyo3(get)]
    pub face: Option<Vec<Biometric>>,
    #[pyo3(get)]
    pub right_palm: Option<Vec<Biometric>>,
    #[pyo3(get)]
    pub left_palm: Option<Vec<Biometric>>,
    #[pyo3(get)]
    pub voice: Option<Vec<Biometric>>,
}

#[pymethods]
impl Claim169 {
    fn __repr__(&self) -> String {
        format!("Claim169(id={:?}, full_name={:?})", self.id, self.full_name)
    }

    /// Check if this claim has any biometric data
    fn has_biometrics(&self) -> bool {
        self.right_thumb.is_some()
            || self.right_pointer_finger.is_some()
            || self.right_middle_finger.is_some()
            || self.right_ring_finger.is_some()
            || self.right_little_finger.is_some()
            || self.left_thumb.is_some()
            || self.left_pointer_finger.is_some()
            || self.left_middle_finger.is_some()
            || self.left_ring_finger.is_some()
            || self.left_little_finger.is_some()
            || self.right_iris.is_some()
            || self.left_iris.is_some()
            || self.face.is_some()
            || self.right_palm.is_some()
            || self.left_palm.is_some()
            || self.voice.is_some()
    }

    /// Convert to a Python dictionary
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);

        if let Some(ref v) = self.id {
            dict.set_item("id", v)?;
        }
        if let Some(ref v) = self.full_name {
            dict.set_item("fullName", v)?;
        }
        if let Some(ref v) = self.first_name {
            dict.set_item("firstName", v)?;
        }
        if let Some(ref v) = self.middle_name {
            dict.set_item("middleName", v)?;
        }
        if let Some(ref v) = self.last_name {
            dict.set_item("lastName", v)?;
        }
        if let Some(ref v) = self.date_of_birth {
            dict.set_item("dateOfBirth", v)?;
        }
        if let Some(v) = self.gender {
            dict.set_item("gender", v)?;
        }
        if let Some(ref v) = self.address {
            dict.set_item("address", v)?;
        }
        if let Some(ref v) = self.email {
            dict.set_item("email", v)?;
        }
        if let Some(ref v) = self.phone {
            dict.set_item("phone", v)?;
        }
        if let Some(ref v) = self.nationality {
            dict.set_item("nationality", v)?;
        }
        if let Some(v) = self.marital_status {
            dict.set_item("maritalStatus", v)?;
        }

        Ok(dict)
    }
}

fn convert_biometrics(biometrics: &Option<Vec<CoreBiometric>>) -> Option<Vec<Biometric>> {
    biometrics
        .as_ref()
        .map(|v| v.iter().map(Biometric::from).collect())
}

impl From<&CoreClaim169> for Claim169 {
    fn from(c: &CoreClaim169) -> Self {
        Claim169 {
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

/// Result of decoding a Claim 169 QR code
#[pyclass]
#[derive(Clone)]
pub struct DecodeResult {
    #[pyo3(get)]
    pub claim169: Claim169,
    #[pyo3(get)]
    pub cwt_meta: CwtMeta,
    #[pyo3(get)]
    pub verification_status: String,
}

#[pymethods]
impl DecodeResult {
    fn __repr__(&self) -> String {
        format!(
            "DecodeResult(id={:?}, verification={})",
            self.claim169.id, self.verification_status
        )
    }

    /// Check if signature was verified
    fn is_verified(&self) -> bool {
        self.verification_status == "verified"
    }
}

// ============================================================================
// Crypto Hook Wrappers
// ============================================================================

/// Python-callable signature verifier hook
#[pyclass]
pub struct PySignatureVerifier {
    callback: Py<PyAny>,
}

#[pymethods]
impl PySignatureVerifier {
    #[new]
    fn new(callback: Py<PyAny>) -> Self {
        PySignatureVerifier { callback }
    }
}

impl CoreSignatureVerifier for PySignatureVerifier {
    fn verify(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
        signature: &[u8],
    ) -> CryptoResult<()> {
        Python::attach(|py| {
            let alg_name = format!("{:?}", algorithm);
            let key_id_bytes: Option<Bound<'_, PyBytes>> = key_id.map(|k| PyBytes::new(py, k));
            let data_bytes = PyBytes::new(py, data);
            let sig_bytes = PyBytes::new(py, signature);

            let result = self
                .callback
                .call1(py, (alg_name, key_id_bytes, data_bytes, sig_bytes));

            match result {
                Ok(_) => Ok(()),
                Err(_e) => Err(CryptoError::VerificationFailed),
            }
        })
    }
}

/// Python-callable decryptor hook
#[pyclass]
pub struct PyDecryptor {
    callback: Py<PyAny>,
}

#[pymethods]
impl PyDecryptor {
    #[new]
    fn new(callback: Py<PyAny>) -> Self {
        PyDecryptor { callback }
    }
}

impl CoreDecryptor for PyDecryptor {
    fn decrypt(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        ciphertext: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        Python::attach(|py| {
            let alg_name = format!("{:?}", algorithm);
            let key_id_bytes: Option<Bound<'_, PyBytes>> = key_id.map(|k| PyBytes::new(py, k));
            let nonce_bytes = PyBytes::new(py, nonce);
            let aad_bytes = PyBytes::new(py, aad);
            let ct_bytes = PyBytes::new(py, ciphertext);

            let result = self.callback.call1(
                py,
                (alg_name, key_id_bytes, nonce_bytes, aad_bytes, ct_bytes),
            );

            match result {
                Ok(obj) => {
                    let bytes: Vec<u8> = obj.extract(py).map_err(|_| {
                        CryptoError::DecryptionFailed(
                            "decryptor callback must return bytes".to_string(),
                        )
                    })?;
                    Ok(bytes)
                }
                Err(e) => Err(CryptoError::DecryptionFailed(e.to_string())),
            }
        })
    }
}

/// Python-callable signer hook for custom crypto providers
///
/// Use this to integrate with external key management systems like:
/// - Hardware Security Modules (HSMs)
/// - Cloud KMS (AWS KMS, Google Cloud KMS, Azure Key Vault)
/// - Remote signing services
/// - Smart cards and TPMs
#[pyclass]
pub struct PySigner {
    callback: Py<PyAny>,
    key_id: Option<Vec<u8>>,
}

#[pymethods]
impl PySigner {
    #[new]
    #[pyo3(signature = (callback, key_id=None))]
    fn new(callback: Py<PyAny>, key_id: Option<Vec<u8>>) -> Self {
        PySigner { callback, key_id }
    }
}

impl CoreSigner for PySigner {
    fn sign(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        Python::attach(|py| {
            let alg_name = format!("{:?}", algorithm);
            let key_id_bytes: Option<Bound<'_, PyBytes>> = key_id.map(|k| PyBytes::new(py, k));
            let data_bytes = PyBytes::new(py, data);

            let result = self
                .callback
                .call1(py, (alg_name, key_id_bytes, data_bytes));

            match result {
                Ok(obj) => {
                    let bytes: Vec<u8> = obj.extract(py).map_err(|_| {
                        CryptoError::SigningFailed("signer callback must return bytes".to_string())
                    })?;
                    Ok(bytes)
                }
                Err(e) => Err(CryptoError::SigningFailed(e.to_string())),
            }
        })
    }

    fn key_id(&self) -> Option<&[u8]> {
        self.key_id.as_deref()
    }
}

/// Python-callable encryptor hook for custom crypto providers
///
/// Use this to integrate with external key management systems like:
/// - Hardware Security Modules (HSMs)
/// - Cloud KMS (AWS KMS, Google Cloud KMS, Azure Key Vault)
/// - Custom software keystores
#[pyclass]
pub struct PyEncryptor {
    callback: Py<PyAny>,
}

#[pymethods]
impl PyEncryptor {
    #[new]
    fn new(callback: Py<PyAny>) -> Self {
        PyEncryptor { callback }
    }
}

impl CoreEncryptor for PyEncryptor {
    fn encrypt(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        nonce: &[u8],
        aad: &[u8],
        plaintext: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        Python::attach(|py| {
            let alg_name = format!("{:?}", algorithm);
            let key_id_bytes: Option<Bound<'_, PyBytes>> = key_id.map(|k| PyBytes::new(py, k));
            let nonce_bytes = PyBytes::new(py, nonce);
            let aad_bytes = PyBytes::new(py, aad);
            let pt_bytes = PyBytes::new(py, plaintext);

            let result = self.callback.call1(
                py,
                (alg_name, key_id_bytes, nonce_bytes, aad_bytes, pt_bytes),
            );

            match result {
                Ok(obj) => {
                    let bytes: Vec<u8> = obj.extract(py).map_err(|_| {
                        CryptoError::EncryptionFailed(
                            "encryptor callback must return bytes".to_string(),
                        )
                    })?;
                    Ok(bytes)
                }
                Err(e) => Err(CryptoError::EncryptionFailed(e.to_string())),
            }
        })
    }
}

// ============================================================================
// Public API Functions
// ============================================================================

/// Decode a Claim 169 QR code without signature verification (INSECURE)
///
/// WARNING: This function skips signature verification. Unverified credentials
/// cannot be trusted. Use decode_with_ed25519() or decode_with_ecdsa_p256()
/// for production use.
///
/// Args:
///     qr_text: The QR code text content (Base45 encoded)
///     skip_biometrics: If True, skip decoding biometric data (default: False)
///     max_decompressed_bytes: Maximum decompressed size (default: 65536)
///     validate_timestamps: If True, validate exp/nbf timestamps (default: True)
///     clock_skew_tolerance_seconds: Tolerance for timestamp validation (default: 0)
///
/// Returns:
///     DecodeResult containing the decoded claim and CWT metadata
///
/// Raises:
///     Base45DecodeError: If Base45 decoding fails
///     DecompressError: If zlib decompression fails
///     CoseParseError: If COSE parsing fails
///     CwtParseError: If CWT parsing fails
///     Claim169NotFoundError: If claim 169 is not present
#[pyfunction]
#[pyo3(signature = (qr_text, skip_biometrics=false, max_decompressed_bytes=65536, validate_timestamps=true, clock_skew_tolerance_seconds=0))]
fn decode_unverified(
    qr_text: &str,
    skip_biometrics: bool,
    max_decompressed_bytes: usize,
    validate_timestamps: bool,
    clock_skew_tolerance_seconds: i64,
) -> PyResult<DecodeResult> {
    let mut decoder = Decoder::new(qr_text)
        .allow_unverified()
        .max_decompressed_bytes(max_decompressed_bytes)
        .clock_skew_tolerance(clock_skew_tolerance_seconds);

    if skip_biometrics {
        decoder = decoder.skip_biometrics();
    }

    if !validate_timestamps {
        decoder = decoder.without_timestamp_validation();
    }

    let result = decoder.decode().map_err(to_py_err)?;

    Ok(DecodeResult {
        claim169: Claim169::from(&result.claim169),
        cwt_meta: CwtMeta::from(&result.cwt_meta),
        verification_status: format!("{}", result.verification_status),
    })
}

/// Decode a Claim 169 QR code with Ed25519 signature verification
///
/// Args:
///     qr_text: The QR code text content
///     public_key: Ed25519 public key bytes (32 bytes)
///     skip_biometrics: If True, skip decoding biometric data (default: False)
///     max_decompressed_bytes: Maximum decompressed size (default: 65536)
///     validate_timestamps: If True, validate exp/nbf timestamps (default: True)
///     clock_skew_tolerance_seconds: Tolerance for timestamp validation (default: 0)
///
/// Returns:
///     DecodeResult with verification_status indicating if signature is valid
#[pyfunction]
#[pyo3(signature = (qr_text, public_key, skip_biometrics=false, max_decompressed_bytes=65536, validate_timestamps=true, clock_skew_tolerance_seconds=0))]
fn decode_with_ed25519(
    qr_text: &str,
    public_key: &[u8],
    skip_biometrics: bool,
    max_decompressed_bytes: usize,
    validate_timestamps: bool,
    clock_skew_tolerance_seconds: i64,
) -> PyResult<DecodeResult> {
    let mut decoder = Decoder::new(qr_text)
        .verify_with_ed25519(public_key)
        .map_err(|e| SignatureError::new_err(e.to_string()))?
        .max_decompressed_bytes(max_decompressed_bytes)
        .clock_skew_tolerance(clock_skew_tolerance_seconds);

    if skip_biometrics {
        decoder = decoder.skip_biometrics();
    }

    if !validate_timestamps {
        decoder = decoder.without_timestamp_validation();
    }

    let result = decoder.decode().map_err(to_py_err)?;

    Ok(DecodeResult {
        claim169: Claim169::from(&result.claim169),
        cwt_meta: CwtMeta::from(&result.cwt_meta),
        verification_status: format!("{}", result.verification_status),
    })
}

/// Decode a Claim 169 QR code with ECDSA P-256 signature verification
///
/// Args:
///     qr_text: The QR code text content
///     public_key: SEC1 encoded P-256 public key bytes (33 or 65 bytes)
///     skip_biometrics: If True, skip decoding biometric data (default: False)
///     max_decompressed_bytes: Maximum decompressed size (default: 65536)
///     validate_timestamps: If True, validate exp/nbf timestamps (default: True)
///     clock_skew_tolerance_seconds: Tolerance for timestamp validation (default: 0)
///
/// Returns:
///     DecodeResult with verification_status indicating if signature is valid
#[pyfunction]
#[pyo3(signature = (qr_text, public_key, skip_biometrics=false, max_decompressed_bytes=65536, validate_timestamps=true, clock_skew_tolerance_seconds=0))]
fn decode_with_ecdsa_p256(
    qr_text: &str,
    public_key: &[u8],
    skip_biometrics: bool,
    max_decompressed_bytes: usize,
    validate_timestamps: bool,
    clock_skew_tolerance_seconds: i64,
) -> PyResult<DecodeResult> {
    let mut decoder = Decoder::new(qr_text)
        .verify_with_ecdsa_p256(public_key)
        .map_err(|e| SignatureError::new_err(e.to_string()))?
        .max_decompressed_bytes(max_decompressed_bytes)
        .clock_skew_tolerance(clock_skew_tolerance_seconds);

    if skip_biometrics {
        decoder = decoder.skip_biometrics();
    }

    if !validate_timestamps {
        decoder = decoder.without_timestamp_validation();
    }

    let result = decoder.decode().map_err(to_py_err)?;

    Ok(DecodeResult {
        claim169: Claim169::from(&result.claim169),
        cwt_meta: CwtMeta::from(&result.cwt_meta),
        verification_status: format!("{}", result.verification_status),
    })
}

/// Decode a Claim 169 QR code with a custom verifier callback
///
/// Use this for integration with external crypto providers such as:
/// - Hardware Security Modules (HSMs)
/// - Cloud KMS (AWS KMS, Google Cloud KMS, Azure Key Vault)
/// - Remote signing services
/// - Smart cards and TPMs
///
/// Args:
///     qr_text: The QR code text content
///     verifier: A callable that takes (algorithm, key_id, data, signature)
///               and raises an exception if verification fails
///
/// Example:
///     def my_verify(algorithm, key_id, data, signature):
///         # Call your crypto provider here
///         crypto_provider.verify(key_id, data, signature)
///
///     result = decode_with_verifier(qr_text, my_verify)
#[pyfunction]
#[pyo3(name = "decode_with_verifier")]
fn py_decode_with_verifier(qr_text: &str, verifier: Py<PyAny>) -> PyResult<DecodeResult> {
    let py_verifier = PySignatureVerifier::new(verifier);
    let result = Decoder::new(qr_text)
        .verify_with(py_verifier)
        .decode()
        .map_err(to_py_err)?;

    Ok(DecodeResult {
        claim169: Claim169::from(&result.claim169),
        cwt_meta: CwtMeta::from(&result.cwt_meta),
        verification_status: format!("{}", result.verification_status),
    })
}

/// Decode an encrypted Claim 169 QR code with AES-GCM
///
/// Supports both AES-128 and AES-256 based on key length.
///
/// Args:
///     qr_text: The QR code text content
///     key: AES-GCM key bytes (16 bytes for AES-128, 32 bytes for AES-256)
///     verifier: Optional verifier callable for nested signature verification
///     allow_unverified: If True, allow decoding without signature verification (INSECURE)
///
/// Returns:
///     DecodeResult containing the decrypted and decoded claim
#[pyfunction]
#[pyo3(signature = (qr_text, key, verifier=None, allow_unverified=false))]
fn decode_encrypted_aes(
    qr_text: &str,
    key: &[u8],
    verifier: Option<Py<PyAny>>,
    allow_unverified: bool,
) -> PyResult<DecodeResult> {
    let decryptor =
        AesGcmDecryptor::from_bytes(key).map_err(|e| PyValueError::new_err(e.to_string()))?;

    let decoder = Decoder::new(qr_text).decrypt_with(decryptor);

    let result = match (verifier, allow_unverified) {
        (Some(v), _) => {
            let py_verifier = PySignatureVerifier::new(v);
            decoder
                .verify_with(py_verifier)
                .decode()
                .map_err(to_py_err)?
        }
        (None, true) => decoder.allow_unverified().decode().map_err(to_py_err)?,
        (None, false) => {
            return Err(PyValueError::new_err(
                "decode_encrypted_aes() requires a verifier unless allow_unverified=True",
            ))
        }
    };

    Ok(DecodeResult {
        claim169: Claim169::from(&result.claim169),
        cwt_meta: CwtMeta::from(&result.cwt_meta),
        verification_status: format!("{}", result.verification_status),
    })
}

/// Decode an encrypted Claim 169 QR code with AES-256-GCM
///
/// Args:
///     qr_text: The QR code text content
///     key: AES-256 key bytes (32 bytes)
///     verifier: Optional verifier callable for nested signature verification
///     allow_unverified: If True, allow decoding without signature verification (INSECURE)
///
/// Returns:
///     DecodeResult containing the decrypted and decoded claim
#[pyfunction]
#[pyo3(signature = (qr_text, key, verifier=None, allow_unverified=false))]
fn decode_encrypted_aes256(
    qr_text: &str,
    key: &[u8],
    verifier: Option<Py<PyAny>>,
    allow_unverified: bool,
) -> PyResult<DecodeResult> {
    if key.len() != 32 {
        return Err(PyValueError::new_err(
            "AES-256 key must be exactly 32 bytes",
        ));
    }
    decode_encrypted_aes(qr_text, key, verifier, allow_unverified)
}

/// Decode an encrypted Claim 169 QR code with AES-128-GCM
///
/// Args:
///     qr_text: The QR code text content
///     key: AES-128 key bytes (16 bytes)
///     verifier: Optional verifier callable for nested signature verification
///     allow_unverified: If True, allow decoding without signature verification (INSECURE)
///
/// Returns:
///     DecodeResult containing the decrypted and decoded claim
#[pyfunction]
#[pyo3(signature = (qr_text, key, verifier=None, allow_unverified=false))]
fn decode_encrypted_aes128(
    qr_text: &str,
    key: &[u8],
    verifier: Option<Py<PyAny>>,
    allow_unverified: bool,
) -> PyResult<DecodeResult> {
    if key.len() != 16 {
        return Err(PyValueError::new_err(
            "AES-128 key must be exactly 16 bytes",
        ));
    }
    decode_encrypted_aes(qr_text, key, verifier, allow_unverified)
}

/// Decode an encrypted Claim 169 QR code with a custom decryptor callback
///
/// Use this for integration with external crypto providers such as:
/// - Hardware Security Modules (HSMs)
/// - Cloud KMS (AWS KMS, Google Cloud KMS, Azure Key Vault)
/// - Custom software keystores
///
/// Args:
///     qr_text: The QR code text content
///     decryptor: A callable that takes (algorithm, key_id, nonce, aad, ciphertext)
///                and returns the decrypted plaintext bytes
///     verifier: Optional verifier callable for nested signature verification
///     allow_unverified: If True, allow decoding without signature verification (INSECURE)
///
/// Example:
///     def my_decrypt(algorithm, key_id, nonce, aad, ciphertext):
///         return crypto_provider.decrypt(key_id, nonce, aad, ciphertext)
///
///     result = decode_with_decryptor(qr_text, my_decrypt)
#[pyfunction]
#[pyo3(signature = (qr_text, decryptor, verifier=None, allow_unverified=false))]
fn decode_with_decryptor(
    qr_text: &str,
    decryptor: Py<PyAny>,
    verifier: Option<Py<PyAny>>,
    allow_unverified: bool,
) -> PyResult<DecodeResult> {
    let py_decryptor = PyDecryptor::new(decryptor);

    let decoder = Decoder::new(qr_text).decrypt_with(py_decryptor);

    let result = match (verifier, allow_unverified) {
        (Some(v), _) => {
            let py_verifier = PySignatureVerifier::new(v);
            decoder
                .verify_with(py_verifier)
                .decode()
                .map_err(to_py_err)?
        }
        (None, true) => decoder.allow_unverified().decode().map_err(to_py_err)?,
        (None, false) => {
            return Err(PyValueError::new_err(
                "decode_with_decryptor() requires a verifier unless allow_unverified=True",
            ))
        }
    };

    Ok(DecodeResult {
        claim169: Claim169::from(&result.claim169),
        cwt_meta: CwtMeta::from(&result.cwt_meta),
        verification_status: format!("{}", result.verification_status),
    })
}

/// Get the library version
#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

// ============================================================================
// Encoder Classes and Functions
// ============================================================================

/// Input Claim 169 for encoding
#[pyclass]
#[derive(Clone)]
pub struct Claim169Input {
    #[pyo3(get, set)]
    pub id: Option<String>,
    #[pyo3(get, set)]
    pub version: Option<String>,
    #[pyo3(get, set)]
    pub language: Option<String>,
    #[pyo3(get, set)]
    pub full_name: Option<String>,
    #[pyo3(get, set)]
    pub first_name: Option<String>,
    #[pyo3(get, set)]
    pub middle_name: Option<String>,
    #[pyo3(get, set)]
    pub last_name: Option<String>,
    #[pyo3(get, set)]
    pub date_of_birth: Option<String>,
    #[pyo3(get, set)]
    pub gender: Option<i64>,
    #[pyo3(get, set)]
    pub address: Option<String>,
    #[pyo3(get, set)]
    pub email: Option<String>,
    #[pyo3(get, set)]
    pub phone: Option<String>,
    #[pyo3(get, set)]
    pub nationality: Option<String>,
    #[pyo3(get, set)]
    pub marital_status: Option<i64>,
    #[pyo3(get, set)]
    pub guardian: Option<String>,
    #[pyo3(get, set)]
    pub photo: Option<Vec<u8>>,
    #[pyo3(get, set)]
    pub photo_format: Option<i64>,
    #[pyo3(get, set)]
    pub secondary_full_name: Option<String>,
    #[pyo3(get, set)]
    pub secondary_language: Option<String>,
    #[pyo3(get, set)]
    pub location_code: Option<String>,
    #[pyo3(get, set)]
    pub legal_status: Option<String>,
    #[pyo3(get, set)]
    pub country_of_issuance: Option<String>,
}

#[pymethods]
impl Claim169Input {
    #[new]
    #[pyo3(signature = (id=None, full_name=None))]
    fn new(id: Option<String>, full_name: Option<String>) -> Self {
        Claim169Input {
            id,
            full_name,
            version: None,
            language: None,
            first_name: None,
            middle_name: None,
            last_name: None,
            date_of_birth: None,
            gender: None,
            address: None,
            email: None,
            phone: None,
            nationality: None,
            marital_status: None,
            guardian: None,
            photo: None,
            photo_format: None,
            secondary_full_name: None,
            secondary_language: None,
            location_code: None,
            legal_status: None,
            country_of_issuance: None,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Claim169Input(id={:?}, full_name={:?})",
            self.id, self.full_name
        )
    }
}

impl From<&Claim169Input> for CoreClaim169 {
    fn from(py: &Claim169Input) -> Self {
        CoreClaim169 {
            id: py.id.clone(),
            version: py.version.clone(),
            language: py.language.clone(),
            full_name: py.full_name.clone(),
            first_name: py.first_name.clone(),
            middle_name: py.middle_name.clone(),
            last_name: py.last_name.clone(),
            date_of_birth: py.date_of_birth.clone(),
            gender: py.gender.and_then(|g| match g {
                1 => Some(Gender::Male),
                2 => Some(Gender::Female),
                3 => Some(Gender::Other),
                _ => None,
            }),
            address: py.address.clone(),
            email: py.email.clone(),
            phone: py.phone.clone(),
            nationality: py.nationality.clone(),
            marital_status: py.marital_status.and_then(|m| match m {
                1 => Some(MaritalStatus::Unmarried),
                2 => Some(MaritalStatus::Married),
                3 => Some(MaritalStatus::Divorced),
                _ => None,
            }),
            guardian: py.guardian.clone(),
            photo: py.photo.clone(),
            photo_format: py.photo_format.and_then(|f| match f {
                1 => Some(PhotoFormat::Jpeg),
                2 => Some(PhotoFormat::Jpeg2000),
                3 => Some(PhotoFormat::Avif),
                4 => Some(PhotoFormat::Webp),
                _ => None,
            }),
            secondary_full_name: py.secondary_full_name.clone(),
            secondary_language: py.secondary_language.clone(),
            location_code: py.location_code.clone(),
            legal_status: py.legal_status.clone(),
            country_of_issuance: py.country_of_issuance.clone(),
            ..Default::default()
        }
    }
}

/// Input CWT metadata for encoding
#[pyclass]
#[derive(Clone)]
pub struct CwtMetaInput {
    #[pyo3(get, set)]
    pub issuer: Option<String>,
    #[pyo3(get, set)]
    pub subject: Option<String>,
    #[pyo3(get, set)]
    pub expires_at: Option<i64>,
    #[pyo3(get, set)]
    pub not_before: Option<i64>,
    #[pyo3(get, set)]
    pub issued_at: Option<i64>,
}

#[pymethods]
impl CwtMetaInput {
    #[new]
    #[pyo3(signature = (issuer=None, expires_at=None))]
    fn new(issuer: Option<String>, expires_at: Option<i64>) -> Self {
        CwtMetaInput {
            issuer,
            expires_at,
            subject: None,
            not_before: None,
            issued_at: None,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "CwtMetaInput(issuer={:?}, expires_at={:?})",
            self.issuer, self.expires_at
        )
    }
}

impl From<&CwtMetaInput> for CoreCwtMeta {
    fn from(py: &CwtMetaInput) -> Self {
        CoreCwtMeta {
            issuer: py.issuer.clone(),
            subject: py.subject.clone(),
            expires_at: py.expires_at,
            not_before: py.not_before,
            issued_at: py.issued_at,
        }
    }
}

/// Encode a Claim 169 credential with Ed25519 signature
///
/// Args:
///     claim169: Claim169Input containing the identity data
///     cwt_meta: CwtMetaInput containing token metadata
///     private_key: Ed25519 private key bytes (32 bytes)
///     skip_biometrics: If True, exclude biometric data to reduce QR size (default: False)
///
/// Returns:
///     Base45-encoded string suitable for QR code generation
#[pyfunction]
#[pyo3(signature = (claim169, cwt_meta, private_key, skip_biometrics=false))]
fn encode_with_ed25519(
    claim169: &Claim169Input,
    cwt_meta: &CwtMetaInput,
    private_key: &[u8],
    skip_biometrics: bool,
) -> PyResult<String> {
    let core_claim: CoreClaim169 = claim169.into();
    let core_meta: CoreCwtMeta = cwt_meta.into();

    let mut encoder = CoreEncoder::new(core_claim, core_meta)
        .sign_with_ed25519(private_key)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    if skip_biometrics {
        encoder = encoder.skip_biometrics();
    }

    encoder.encode().map_err(to_py_err)
}

/// Encode a Claim 169 credential with ECDSA P-256 signature
///
/// Args:
///     claim169: Claim169Input containing the identity data
///     cwt_meta: CwtMetaInput containing token metadata
///     private_key: ECDSA P-256 private key bytes (32 bytes)
///     skip_biometrics: If True, exclude biometric data to reduce QR size (default: False)
///
/// Returns:
///     Base45-encoded string suitable for QR code generation
#[pyfunction]
#[pyo3(signature = (claim169, cwt_meta, private_key, skip_biometrics=false))]
fn encode_with_ecdsa_p256(
    claim169: &Claim169Input,
    cwt_meta: &CwtMetaInput,
    private_key: &[u8],
    skip_biometrics: bool,
) -> PyResult<String> {
    let core_claim: CoreClaim169 = claim169.into();
    let core_meta: CoreCwtMeta = cwt_meta.into();

    let mut encoder = CoreEncoder::new(core_claim, core_meta)
        .sign_with_ecdsa_p256(private_key)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    if skip_biometrics {
        encoder = encoder.skip_biometrics();
    }

    encoder.encode().map_err(to_py_err)
}

/// Encode a Claim 169 credential with Ed25519 signature and AES-256-GCM encryption
///
/// Args:
///     claim169: Claim169Input containing the identity data
///     cwt_meta: CwtMetaInput containing token metadata
///     sign_key: Ed25519 private key bytes (32 bytes)
///     encrypt_key: AES-256 key bytes (32 bytes)
///     skip_biometrics: If True, exclude biometric data to reduce QR size (default: False)
///
/// Returns:
///     Base45-encoded string suitable for QR code generation
#[pyfunction]
#[pyo3(signature = (claim169, cwt_meta, sign_key, encrypt_key, skip_biometrics=false))]
fn encode_signed_encrypted(
    claim169: &Claim169Input,
    cwt_meta: &CwtMetaInput,
    sign_key: &[u8],
    encrypt_key: &[u8],
    skip_biometrics: bool,
) -> PyResult<String> {
    let core_claim: CoreClaim169 = claim169.into();
    let core_meta: CoreCwtMeta = cwt_meta.into();

    let mut encoder = CoreEncoder::new(core_claim, core_meta)
        .sign_with_ed25519(sign_key)
        .map_err(|e| PyValueError::new_err(e.to_string()))?
        .encrypt_with_aes256(encrypt_key)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    if skip_biometrics {
        encoder = encoder.skip_biometrics();
    }

    encoder.encode().map_err(to_py_err)
}

/// Encode a Claim 169 credential with Ed25519 signature and AES-128-GCM encryption
///
/// Args:
///     claim169: Claim169Input containing the identity data
///     cwt_meta: CwtMetaInput containing token metadata
///     sign_key: Ed25519 private key bytes (32 bytes)
///     encrypt_key: AES-128 key bytes (16 bytes)
///     skip_biometrics: If True, exclude biometric data to reduce QR size (default: False)
///
/// Returns:
///     Base45-encoded string suitable for QR code generation
#[pyfunction]
#[pyo3(signature = (claim169, cwt_meta, sign_key, encrypt_key, skip_biometrics=false))]
fn encode_signed_encrypted_aes128(
    claim169: &Claim169Input,
    cwt_meta: &CwtMetaInput,
    sign_key: &[u8],
    encrypt_key: &[u8],
    skip_biometrics: bool,
) -> PyResult<String> {
    let core_claim: CoreClaim169 = claim169.into();
    let core_meta: CoreCwtMeta = cwt_meta.into();

    let mut encoder = CoreEncoder::new(core_claim, core_meta)
        .sign_with_ed25519(sign_key)
        .map_err(|e| PyValueError::new_err(e.to_string()))?
        .encrypt_with_aes128(encrypt_key)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    if skip_biometrics {
        encoder = encoder.skip_biometrics();
    }

    encoder.encode().map_err(to_py_err)
}

/// Encode a Claim 169 credential without signature (INSECURE - testing only)
///
/// Args:
///     claim169: Claim169Input containing the identity data
///     cwt_meta: CwtMetaInput containing token metadata
///     skip_biometrics: If True, exclude biometric data to reduce QR size (default: False)
///
/// Returns:
///     Base45-encoded string suitable for QR code generation
#[pyfunction]
#[pyo3(signature = (claim169, cwt_meta, skip_biometrics=false))]
fn encode_unsigned(
    claim169: &Claim169Input,
    cwt_meta: &CwtMetaInput,
    skip_biometrics: bool,
) -> PyResult<String> {
    let core_claim: CoreClaim169 = claim169.into();
    let core_meta: CoreCwtMeta = cwt_meta.into();

    let mut encoder = CoreEncoder::new(core_claim, core_meta).allow_unsigned();

    if skip_biometrics {
        encoder = encoder.skip_biometrics();
    }

    encoder.encode().map_err(to_py_err)
}

/// Encode a Claim 169 credential with a custom signer callback
///
/// Use this for integration with external crypto providers such as:
/// - Hardware Security Modules (HSMs)
/// - Cloud KMS (AWS KMS, Google Cloud KMS, Azure Key Vault)
/// - Remote signing services
/// - Smart cards and TPMs
///
/// Args:
///     claim169: Claim169Input containing the identity data
///     cwt_meta: CwtMetaInput containing token metadata
///     signer: A callable that takes (algorithm, key_id, data) and returns signature bytes
///     algorithm: The signing algorithm ("EdDSA" or "ES256")
///     key_id: Optional key identifier bytes
///     skip_biometrics: If True, exclude biometric data to reduce QR size (default: False)
///
/// Returns:
///     Base45-encoded string suitable for QR code generation
///
/// Example:
///     def my_sign(algorithm, key_id, data):
///         return crypto_provider.sign(key_id, data)
///
///     qr_text = encode_with_signer(claim, meta, my_sign, "EdDSA")
#[pyfunction]
#[pyo3(signature = (claim169, cwt_meta, signer, algorithm, key_id=None, skip_biometrics=false))]
fn encode_with_signer(
    claim169: &Claim169Input,
    cwt_meta: &CwtMetaInput,
    signer: Py<PyAny>,
    algorithm: &str,
    key_id: Option<Vec<u8>>,
    skip_biometrics: bool,
) -> PyResult<String> {
    let core_claim: CoreClaim169 = claim169.into();
    let core_meta: CoreCwtMeta = cwt_meta.into();

    let alg = match algorithm {
        "EdDSA" => iana::Algorithm::EdDSA,
        "ES256" => iana::Algorithm::ES256,
        _ => {
            return Err(PyValueError::new_err(format!(
                "Unsupported algorithm: {}. Use 'EdDSA' or 'ES256'",
                algorithm
            )))
        }
    };

    let py_signer = PySigner::new(signer, key_id);
    let mut encoder = CoreEncoder::new(core_claim, core_meta).sign_with(py_signer, alg);

    if skip_biometrics {
        encoder = encoder.skip_biometrics();
    }

    encoder.encode().map_err(to_py_err)
}

/// Encode a Claim 169 credential with custom signer and encryptor callbacks
///
/// Use this for integration with external crypto providers such as:
/// - Hardware Security Modules (HSMs)
/// - Cloud KMS (AWS KMS, Google Cloud KMS, Azure Key Vault)
/// - Remote signing services
///
/// Args:
///     claim169: Claim169Input containing the identity data
///     cwt_meta: CwtMetaInput containing token metadata
///     signer: A callable that takes (algorithm, key_id, data) and returns signature bytes
///     sign_algorithm: The signing algorithm ("EdDSA" or "ES256")
///     encryptor: A callable that takes (algorithm, key_id, nonce, aad, plaintext)
///                and returns ciphertext bytes
///     encrypt_algorithm: The encryption algorithm ("A256GCM" or "A128GCM")
///     key_id: Optional key identifier bytes
///     skip_biometrics: If True, exclude biometric data to reduce QR size (default: False)
///
/// Returns:
///     Base45-encoded string suitable for QR code generation
///
/// Example:
///     def my_sign(algorithm, key_id, data):
///         return crypto_provider.sign(key_id, data)
///
///     def my_encrypt(algorithm, key_id, nonce, aad, plaintext):
///         return crypto_provider.encrypt(key_id, nonce, aad, plaintext)
///
///     qr_text = encode_with_signer_and_encryptor(
///         claim, meta, my_sign, "EdDSA", my_encrypt, "A256GCM"
///     )
#[pyfunction]
#[pyo3(signature = (claim169, cwt_meta, signer, sign_algorithm, encryptor, encrypt_algorithm, key_id=None, skip_biometrics=false))]
#[allow(clippy::too_many_arguments)]
fn encode_with_signer_and_encryptor(
    claim169: &Claim169Input,
    cwt_meta: &CwtMetaInput,
    signer: Py<PyAny>,
    sign_algorithm: &str,
    encryptor: Py<PyAny>,
    encrypt_algorithm: &str,
    key_id: Option<Vec<u8>>,
    skip_biometrics: bool,
) -> PyResult<String> {
    let core_claim: CoreClaim169 = claim169.into();
    let core_meta: CoreCwtMeta = cwt_meta.into();

    let sign_alg = match sign_algorithm {
        "EdDSA" => iana::Algorithm::EdDSA,
        "ES256" => iana::Algorithm::ES256,
        _ => {
            return Err(PyValueError::new_err(format!(
                "Unsupported sign algorithm: {}. Use 'EdDSA' or 'ES256'",
                sign_algorithm
            )))
        }
    };

    let encrypt_alg = match encrypt_algorithm {
        "A256GCM" => iana::Algorithm::A256GCM,
        "A128GCM" => iana::Algorithm::A128GCM,
        _ => {
            return Err(PyValueError::new_err(format!(
                "Unsupported encrypt algorithm: {}. Use 'A256GCM' or 'A128GCM'",
                encrypt_algorithm
            )))
        }
    };

    let py_signer = PySigner::new(signer, key_id);
    let py_encryptor = PyEncryptor::new(encryptor);

    let mut encoder = CoreEncoder::new(core_claim, core_meta)
        .sign_with(py_signer, sign_alg)
        .encrypt_with(py_encryptor, encrypt_alg);

    if skip_biometrics {
        encoder = encoder.skip_biometrics();
    }

    encoder.encode().map_err(to_py_err)
}

/// Encode with software signing and custom encryptor callback
///
/// Args:
///     claim169: Claim169Input containing the identity data
///     cwt_meta: CwtMetaInput containing token metadata
///     sign_key: Ed25519 private key bytes (32 bytes)
///     encryptor: A callable that takes (algorithm, key_id, nonce, aad, plaintext)
///                and returns ciphertext bytes
///     encrypt_algorithm: The encryption algorithm ("A256GCM" or "A128GCM")
///     skip_biometrics: If True, exclude biometric data to reduce QR size (default: False)
///
/// Returns:
///     Base45-encoded string suitable for QR code generation
#[pyfunction]
#[pyo3(signature = (claim169, cwt_meta, sign_key, encryptor, encrypt_algorithm, skip_biometrics=false))]
fn encode_with_encryptor(
    claim169: &Claim169Input,
    cwt_meta: &CwtMetaInput,
    sign_key: &[u8],
    encryptor: Py<PyAny>,
    encrypt_algorithm: &str,
    skip_biometrics: bool,
) -> PyResult<String> {
    let core_claim: CoreClaim169 = claim169.into();
    let core_meta: CoreCwtMeta = cwt_meta.into();

    let encrypt_alg = match encrypt_algorithm {
        "A256GCM" => iana::Algorithm::A256GCM,
        "A128GCM" => iana::Algorithm::A128GCM,
        _ => {
            return Err(PyValueError::new_err(format!(
                "Unsupported encrypt algorithm: {}. Use 'A256GCM' or 'A128GCM'",
                encrypt_algorithm
            )))
        }
    };

    let py_encryptor = PyEncryptor::new(encryptor);

    let mut encoder = CoreEncoder::new(core_claim, core_meta)
        .sign_with_ed25519(sign_key)
        .map_err(|e| PyValueError::new_err(e.to_string()))?
        .encrypt_with(py_encryptor, encrypt_alg);

    if skip_biometrics {
        encoder = encoder.skip_biometrics();
    }

    encoder.encode().map_err(to_py_err)
}

/// Generate a random 12-byte nonce for AES-GCM encryption
#[pyfunction]
fn generate_nonce() -> Vec<u8> {
    claim169_core::generate_random_nonce().to_vec()
}

// ============================================================================
// Module Definition
// ============================================================================

/// MOSIP Claim 169 QR Code decoder library
#[pymodule]
fn claim169(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add exception types
    m.add("Claim169Exception", py.get_type::<Claim169Exception>())?;
    m.add("Base45DecodeError", py.get_type::<Base45DecodeError>())?;
    m.add("DecompressError", py.get_type::<DecompressError>())?;
    m.add("CoseParseError", py.get_type::<CoseParseError>())?;
    m.add("CwtParseError", py.get_type::<CwtParseError>())?;
    m.add(
        "Claim169NotFoundError",
        py.get_type::<Claim169NotFoundError>(),
    )?;
    m.add("SignatureError", py.get_type::<SignatureError>())?;
    m.add("DecryptionError", py.get_type::<DecryptionError>())?;
    m.add("EncryptionError", py.get_type::<EncryptionError>())?;

    // Add classes
    m.add_class::<Biometric>()?;
    m.add_class::<CwtMeta>()?;
    m.add_class::<Claim169>()?;
    m.add_class::<DecodeResult>()?;
    m.add_class::<PySignatureVerifier>()?;
    m.add_class::<PyDecryptor>()?;
    m.add_class::<PySigner>()?;
    m.add_class::<PyEncryptor>()?;
    m.add_class::<Claim169Input>()?;
    m.add_class::<CwtMetaInput>()?;

    // Add decode functions
    m.add_function(wrap_pyfunction!(decode_unverified, m)?)?;
    m.add_function(wrap_pyfunction!(decode_with_ed25519, m)?)?;
    m.add_function(wrap_pyfunction!(decode_with_ecdsa_p256, m)?)?;
    m.add_function(wrap_pyfunction!(py_decode_with_verifier, m)?)?;
    m.add_function(wrap_pyfunction!(decode_encrypted_aes, m)?)?;
    m.add_function(wrap_pyfunction!(decode_encrypted_aes256, m)?)?;
    m.add_function(wrap_pyfunction!(decode_encrypted_aes128, m)?)?;
    m.add_function(wrap_pyfunction!(decode_with_decryptor, m)?)?;

    // Add encode functions
    m.add_function(wrap_pyfunction!(encode_with_ed25519, m)?)?;
    m.add_function(wrap_pyfunction!(encode_with_ecdsa_p256, m)?)?;
    m.add_function(wrap_pyfunction!(encode_signed_encrypted, m)?)?;
    m.add_function(wrap_pyfunction!(encode_signed_encrypted_aes128, m)?)?;
    m.add_function(wrap_pyfunction!(encode_unsigned, m)?)?;
    m.add_function(wrap_pyfunction!(encode_with_signer, m)?)?;
    m.add_function(wrap_pyfunction!(encode_with_signer_and_encryptor, m)?)?;
    m.add_function(wrap_pyfunction!(encode_with_encryptor, m)?)?;
    m.add_function(wrap_pyfunction!(generate_nonce, m)?)?;

    // Utilities
    m.add_function(wrap_pyfunction!(version, m)?)?;

    Ok(())
}
