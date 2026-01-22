// Allow pyo3 internal cfg checks and macro-generated conversions
#![allow(unexpected_cfgs)]
#![allow(clippy::useless_conversion)]

//! Python bindings for MOSIP Claim 169 QR decoder
//!
//! This module provides Python bindings using PyO3 for the claim169-core library.
//! It supports custom crypto hooks for HSM integration.

use pyo3::exceptions::{PyException, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};

use claim169_core::crypto::software::{AesGcmDecryptor, EcdsaP256Verifier, Ed25519Verifier};
use claim169_core::crypto::traits::{
    Decryptor as CoreDecryptor, SignatureVerifier as CoreSignatureVerifier,
};
use claim169_core::error::{Claim169Error, CryptoError, CryptoResult};
use claim169_core::model::{
    Biometric as CoreBiometric, Claim169 as CoreClaim169, CwtMeta as CoreCwtMeta,
};
use claim169_core::{decode as core_decode, decode_encrypted, decode_with_verifier, DecodeOptions};
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

fn to_py_err(e: Claim169Error) -> PyErr {
    match e {
        Claim169Error::Base45Decode(_) => Base45DecodeError::new_err(e.to_string()),
        Claim169Error::Decompress(_) => DecompressError::new_err(e.to_string()),
        Claim169Error::DecompressLimitExceeded { .. } => DecompressError::new_err(e.to_string()),
        Claim169Error::CoseParse(_) => CoseParseError::new_err(e.to_string()),
        Claim169Error::CborParse(_) => CoseParseError::new_err(e.to_string()),
        Claim169Error::CwtParse(_) => CwtParseError::new_err(e.to_string()),
        Claim169Error::Claim169NotFound => Claim169NotFoundError::new_err(e.to_string()),
        Claim169Error::Crypto(_) => SignatureError::new_err(e.to_string()),
        Claim169Error::DecryptionFailed(_) => DecryptionError::new_err(e.to_string()),
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
    pub format: i64,
    #[pyo3(get)]
    pub sub_format: Option<i64>,
    #[pyo3(get)]
    pub issuer: Option<String>,
}

#[pymethods]
impl Biometric {
    fn __repr__(&self) -> String {
        format!(
            "Biometric(format={}, sub_format={:?}, data_len={})",
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
            format: b.format.map(|f| f as i64).unwrap_or(0),
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
        let dict = PyDict::new_bound(py);

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
    callback: PyObject,
}

#[pymethods]
impl PySignatureVerifier {
    #[new]
    fn new(callback: PyObject) -> Self {
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
        Python::with_gil(|py| {
            let alg_name = format!("{:?}", algorithm);
            let key_id_bytes: Option<Bound<'_, PyBytes>> =
                key_id.map(|k| PyBytes::new_bound(py, k));
            let data_bytes = PyBytes::new_bound(py, data);
            let sig_bytes = PyBytes::new_bound(py, signature);

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
    callback: PyObject,
}

#[pymethods]
impl PyDecryptor {
    #[new]
    fn new(callback: PyObject) -> Self {
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
        Python::with_gil(|py| {
            let alg_name = format!("{:?}", algorithm);
            let key_id_bytes: Option<Bound<'_, PyBytes>> =
                key_id.map(|k| PyBytes::new_bound(py, k));
            let nonce_bytes = PyBytes::new_bound(py, nonce);
            let aad_bytes = PyBytes::new_bound(py, aad);
            let ct_bytes = PyBytes::new_bound(py, ciphertext);

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

// ============================================================================
// Public API Functions
// ============================================================================

/// Decode a Claim 169 QR code without signature verification
///
/// Args:
///     qr_text: The QR code text content (Base45 encoded)
///     skip_biometrics: If True, skip decoding biometric data (default: False)
///     max_decompressed_bytes: Maximum decompressed size (default: 65536)
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
#[pyo3(signature = (qr_text, skip_biometrics=false, max_decompressed_bytes=65536))]
fn decode(
    qr_text: &str,
    skip_biometrics: bool,
    max_decompressed_bytes: usize,
) -> PyResult<DecodeResult> {
    let options = DecodeOptions {
        max_decompressed_bytes,
        skip_biometrics,
        validate_timestamps: true,
        allow_unverified: true,
        clock_skew_tolerance_seconds: 0,
    };

    let result = core_decode(qr_text, options).map_err(to_py_err)?;

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
///
/// Returns:
///     DecodeResult with verification_status indicating if signature is valid
#[pyfunction]
fn decode_with_ed25519(qr_text: &str, public_key: &[u8]) -> PyResult<DecodeResult> {
    let verifier = Ed25519Verifier::from_bytes(public_key)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    let options = DecodeOptions::default();
    let result = decode_with_verifier(qr_text, &verifier, options).map_err(to_py_err)?;

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
///
/// Returns:
///     DecodeResult with verification_status indicating if signature is valid
#[pyfunction]
fn decode_with_ecdsa_p256(qr_text: &str, public_key: &[u8]) -> PyResult<DecodeResult> {
    let verifier = EcdsaP256Verifier::from_sec1_bytes(public_key)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    let options = DecodeOptions::default();
    let result = decode_with_verifier(qr_text, &verifier, options).map_err(to_py_err)?;

    Ok(DecodeResult {
        claim169: Claim169::from(&result.claim169),
        cwt_meta: CwtMeta::from(&result.cwt_meta),
        verification_status: format!("{}", result.verification_status),
    })
}

/// Decode a Claim 169 QR code with a custom verifier hook
///
/// This allows integration with HSMs or other hardware security modules.
///
/// Args:
///     qr_text: The QR code text content
///     verifier: A callable that takes (algorithm, key_id, data, signature)
///               and raises an exception if verification fails
///
/// Example:
///     def my_hsm_verify(algorithm, key_id, data, signature):
///         # Call your HSM here
///         hsm.verify(key_id, data, signature)
///
///     result = decode_with_verifier(qr_text, my_hsm_verify)
#[pyfunction]
#[pyo3(name = "decode_with_verifier")]
fn py_decode_with_verifier(qr_text: &str, verifier: PyObject) -> PyResult<DecodeResult> {
    let py_verifier = PySignatureVerifier::new(verifier);
    let options = DecodeOptions::default();
    let result = decode_with_verifier(qr_text, &py_verifier, options).map_err(to_py_err)?;

    Ok(DecodeResult {
        claim169: Claim169::from(&result.claim169),
        cwt_meta: CwtMeta::from(&result.cwt_meta),
        verification_status: format!("{}", result.verification_status),
    })
}

/// Decode an encrypted Claim 169 QR code
///
/// Args:
///     qr_text: The QR code text content
///     key: AES-GCM key bytes (16 bytes for AES-128, 32 bytes for AES-256)
///     verifier: Optional verifier callable for nested signature verification
///
/// Returns:
///     DecodeResult containing the decrypted and decoded claim
#[pyfunction]
#[pyo3(signature = (qr_text, key, verifier=None))]
fn decode_encrypted_aes(
    qr_text: &str,
    key: &[u8],
    verifier: Option<PyObject>,
) -> PyResult<DecodeResult> {
    let decryptor =
        AesGcmDecryptor::from_bytes(key).map_err(|e| PyValueError::new_err(e.to_string()))?;

    let options = DecodeOptions::default();

    let result = if let Some(v) = verifier {
        let py_verifier = PySignatureVerifier::new(v);
        decode_encrypted(qr_text, &decryptor, Some(&py_verifier), options).map_err(to_py_err)?
    } else {
        decode_encrypted(qr_text, &decryptor, None::<&Ed25519Verifier>, options)
            .map_err(to_py_err)?
    };

    Ok(DecodeResult {
        claim169: Claim169::from(&result.claim169),
        cwt_meta: CwtMeta::from(&result.cwt_meta),
        verification_status: format!("{}", result.verification_status),
    })
}

/// Decode an encrypted Claim 169 QR code with a custom decryptor hook
///
/// Args:
///     qr_text: The QR code text content
///     decryptor: A callable that takes (algorithm, key_id, nonce, aad, ciphertext)
///                and returns the decrypted plaintext bytes
///     verifier: Optional verifier callable for nested signature verification
///
/// Example:
///     def my_hsm_decrypt(algorithm, key_id, nonce, aad, ciphertext):
///         return hsm.decrypt(key_id, nonce, aad, ciphertext)
///
///     result = decode_with_decryptor(qr_text, my_hsm_decrypt)
#[pyfunction]
#[pyo3(signature = (qr_text, decryptor, verifier=None))]
fn decode_with_decryptor(
    qr_text: &str,
    decryptor: PyObject,
    verifier: Option<PyObject>,
) -> PyResult<DecodeResult> {
    let py_decryptor = PyDecryptor::new(decryptor);
    let options = DecodeOptions::default();

    let result = if let Some(v) = verifier {
        let py_verifier = PySignatureVerifier::new(v);
        decode_encrypted(qr_text, &py_decryptor, Some(&py_verifier), options).map_err(to_py_err)?
    } else {
        decode_encrypted(qr_text, &py_decryptor, None::<&Ed25519Verifier>, options)
            .map_err(to_py_err)?
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
// Module Definition
// ============================================================================

/// MOSIP Claim 169 QR Code decoder library
#[pymodule]
fn claim169(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add exception types
    m.add(
        "Claim169Exception",
        py.get_type_bound::<Claim169Exception>(),
    )?;
    m.add(
        "Base45DecodeError",
        py.get_type_bound::<Base45DecodeError>(),
    )?;
    m.add("DecompressError", py.get_type_bound::<DecompressError>())?;
    m.add("CoseParseError", py.get_type_bound::<CoseParseError>())?;
    m.add("CwtParseError", py.get_type_bound::<CwtParseError>())?;
    m.add(
        "Claim169NotFoundError",
        py.get_type_bound::<Claim169NotFoundError>(),
    )?;
    m.add("SignatureError", py.get_type_bound::<SignatureError>())?;
    m.add("DecryptionError", py.get_type_bound::<DecryptionError>())?;

    // Add classes
    m.add_class::<Biometric>()?;
    m.add_class::<CwtMeta>()?;
    m.add_class::<Claim169>()?;
    m.add_class::<DecodeResult>()?;
    m.add_class::<PySignatureVerifier>()?;
    m.add_class::<PyDecryptor>()?;

    // Add functions
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    m.add_function(wrap_pyfunction!(decode_with_ed25519, m)?)?;
    m.add_function(wrap_pyfunction!(decode_with_ecdsa_p256, m)?)?;
    m.add_function(wrap_pyfunction!(py_decode_with_verifier, m)?)?;
    m.add_function(wrap_pyfunction!(decode_encrypted_aes, m)?)?;
    m.add_function(wrap_pyfunction!(decode_with_decryptor, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;

    Ok(())
}
