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
    Biometric as CoreBiometric, CertHashAlgorithm as CoreCertHashAlgorithm,
    CertificateHash as CoreCertificateHash, Claim169 as CoreClaim169, CwtMeta as CoreCwtMeta,
    Gender, MaritalStatus, PhotoFormat, X509Headers as CoreX509Headers,
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

fn detected_compression_to_string(dc: &claim169_core::DetectedCompression) -> &'static str {
    match dc {
        claim169_core::DetectedCompression::Zlib => "zlib",
        #[cfg(feature = "compression-brotli")]
        claim169_core::DetectedCompression::Brotli => "brotli",
        claim169_core::DetectedCompression::None => "none",
    }
}

fn algorithm_to_string(alg: &iana::Algorithm) -> String {
    use coset::iana::EnumI64;
    match alg {
        iana::Algorithm::EdDSA => "EdDSA".to_string(),
        iana::Algorithm::ES256 => "ES256".to_string(),
        iana::Algorithm::ES384 => "ES384".to_string(),
        iana::Algorithm::ES512 => "ES512".to_string(),
        iana::Algorithm::A128GCM => "A128GCM".to_string(),
        iana::Algorithm::A192GCM => "A192GCM".to_string(),
        iana::Algorithm::A256GCM => "A256GCM".to_string(),
        other => format!("COSE_ALG_{}", other.to_i64()),
    }
}

fn core_decode_result_to_py(result: claim169_core::DecodeResult) -> DecodeResult {
    DecodeResult {
        claim169: Claim169::from(&result.claim169),
        cwt_meta: CwtMeta::from(&result.cwt_meta),
        verification_status: format!("{}", result.verification_status),
        x509_headers: X509Headers::from(&result.x509_headers),
        detected_compression: detected_compression_to_string(&result.detected_compression)
            .to_string(),
        key_id: result.key_id,
        algorithm: result.algorithm.as_ref().map(algorithm_to_string),
    }
}

// ============================================================================
// Python Data Classes
// ============================================================================

/// Biometric data extracted from a Claim 169 credential.
///
/// Contains raw biometric data (fingerprint, iris, face, palm, or voice)
/// along with format metadata and issuer information.
///
/// Attributes:
///     data (bytes): Raw biometric data bytes.
///     format (int | None): Biometric format code (0=Image, 1=Template, 2=Sound, 3=BioHash).
///     sub_format (int | None): Sub-format code (depends on format type).
///     issuer (str | None): Biometric data issuer/provider identifier.
#[pyclass]
#[derive(Clone)]
pub struct Biometric {
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
    #[new]
    #[pyo3(signature = (data, format=None, sub_format=None, issuer=None))]
    fn new(
        data: Vec<u8>,
        format: Option<i64>,
        sub_format: Option<i64>,
        issuer: Option<String>,
    ) -> Self {
        Biometric {
            data,
            format,
            sub_format,
            issuer,
        }
    }

    #[getter]
    fn data<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.data)
    }

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

impl From<&Biometric> for CoreBiometric {
    fn from(py: &Biometric) -> Self {
        use claim169_core::{BiometricFormat, BiometricSubFormat};

        let format = py.format.and_then(|f| BiometricFormat::try_from(f).ok());
        let sub_format = match (format, py.sub_format) {
            (Some(fmt), Some(sf)) => Some(BiometricSubFormat::from_format_and_value(fmt, sf)),
            (None, Some(sf)) => Some(BiometricSubFormat::Raw(sf)),
            _ => None,
        };

        CoreBiometric {
            data: py.data.clone(),
            format,
            sub_format,
            issuer: py.issuer.clone(),
        }
    }
}

/// CWT (CBOR Web Token) metadata from a decoded credential.
///
/// Contains standard JWT/CWT claims providing information about the
/// credential's validity period, issuer, and subject.
///
/// Attributes:
///     issuer (str | None): Token issuer (URL or identifier).
///     subject (str | None): Token subject (credential holder's ID).
///     expires_at (int | None): Expiration timestamp (Unix epoch seconds).
///     not_before (int | None): Not-before timestamp (Unix epoch seconds).
///     issued_at (int | None): Issued-at timestamp (Unix epoch seconds).
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

/// X.509 certificate hash (COSE_CertHash).
///
/// Contains a hash algorithm identifier and the hash value, used for
/// X.509 certificate thumbprint verification.
///
/// Attributes:
///     algorithm (str): Hash algorithm identifier (numeric COSE algorithm ID or string name).
///     hash_value (bytes): Hash value bytes.
#[pyclass]
#[derive(Clone)]
pub struct CertificateHash {
    /// Hash algorithm identifier (numeric COSE algorithm ID or string name)
    #[pyo3(get)]
    pub algorithm: String,
    /// Hash value bytes
    pub hash_value: Vec<u8>,
}

#[pymethods]
impl CertificateHash {
    #[getter]
    fn hash_value<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.hash_value)
    }

    fn __repr__(&self) -> String {
        format!(
            "CertificateHash(algorithm={}, hash_len={})",
            self.algorithm,
            self.hash_value.len()
        )
    }
}

impl From<&CoreCertificateHash> for CertificateHash {
    fn from(c: &CoreCertificateHash) -> Self {
        let algorithm = match &c.algorithm {
            CoreCertHashAlgorithm::Numeric(n) => n.to_string(),
            CoreCertHashAlgorithm::Named(s) => s.clone(),
        };
        CertificateHash {
            algorithm,
            hash_value: c.hash_value.clone(),
        }
    }
}

/// X.509 headers extracted from COSE protected/unprotected headers.
///
/// Contains X.509 certificate information for signature verification,
/// including certificate bags, chains, thumbprints, and URIs.
///
/// Attributes:
///     x5bag (list[bytes] | None): Unordered bag of X.509 certificates (DER-encoded).
///     x5chain (list[bytes] | None): Ordered chain of X.509 certificates (DER-encoded).
///     x5t (CertificateHash | None): Certificate thumbprint hash.
///     x5u (str | None): URI pointing to an X.509 certificate.
#[pyclass]
#[derive(Clone)]
pub struct X509Headers {
    /// x5bag (label 32): Unordered bag of X.509 certificates (DER-encoded)
    pub x5bag: Option<Vec<Vec<u8>>>,
    /// x5chain (label 33): Ordered chain of X.509 certificates (DER-encoded)
    pub x5chain: Option<Vec<Vec<u8>>>,
    /// x5t (label 34): Certificate thumbprint hash
    #[pyo3(get)]
    pub x5t: Option<CertificateHash>,
    /// x5u (label 35): URI pointing to an X.509 certificate
    #[pyo3(get)]
    pub x5u: Option<String>,
}

#[pymethods]
impl X509Headers {
    #[getter]
    fn x5bag<'py>(&self, py: Python<'py>) -> Option<Vec<Bound<'py, PyBytes>>> {
        self.x5bag
            .as_ref()
            .map(|bags| bags.iter().map(|b| PyBytes::new(py, b)).collect())
    }

    #[getter]
    fn x5chain<'py>(&self, py: Python<'py>) -> Option<Vec<Bound<'py, PyBytes>>> {
        self.x5chain
            .as_ref()
            .map(|bags| bags.iter().map(|b| PyBytes::new(py, b)).collect())
    }

    fn __repr__(&self) -> String {
        let parts: Vec<&str> = [
            self.x5bag.as_ref().map(|_| "x5bag"),
            self.x5chain.as_ref().map(|_| "x5chain"),
            self.x5t.as_ref().map(|_| "x5t"),
            self.x5u.as_ref().map(|_| "x5u"),
        ]
        .into_iter()
        .flatten()
        .collect();

        if parts.is_empty() {
            "X509Headers(empty)".to_string()
        } else {
            format!("X509Headers({})", parts.join(", "))
        }
    }

    /// Check if any X.509 headers are present
    fn has_any(&self) -> bool {
        self.x5bag.is_some() || self.x5chain.is_some() || self.x5t.is_some() || self.x5u.is_some()
    }
}

impl From<&CoreX509Headers> for X509Headers {
    fn from(h: &CoreX509Headers) -> Self {
        X509Headers {
            x5bag: h.x5bag.clone(),
            x5chain: h.x5chain.clone(),
            x5t: h.x5t.as_ref().map(CertificateHash::from),
            x5u: h.x5u.clone(),
        }
    }
}

/// Decoded Claim 169 identity data.
///
/// Contains all demographic and biometric fields from a decoded
/// MOSIP Claim 169 QR code credential. All fields are read-only.
///
/// Attributes:
///     id (str | None): Unique identifier.
///     version (str | None): Credential version.
///     language (str | None): Primary language code (ISO 639-1).
///     full_name (str | None): Full name.
///     first_name (str | None): First/given name.
///     middle_name (str | None): Middle name.
///     last_name (str | None): Last/family name.
///     date_of_birth (str | None): Date of birth (YYYY-MM-DD).
///     gender (int | None): Gender (1=Male, 2=Female, 3=Other).
///     address (str | None): Full address.
///     email (str | None): Email address.
///     phone (str | None): Phone number.
///     nationality (str | None): Nationality code.
///     marital_status (int | None): Marital status (1=Unmarried, 2=Married, 3=Divorced).
///     guardian (str | None): Guardian name.
///     photo (bytes | None): Photo data bytes.
///     photo_format (int | None): Photo format (1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP).
///     best_quality_fingers (bytes | None): Best quality finger indices (0-10).
///     secondary_full_name (str | None): Name in secondary language.
///     secondary_language (str | None): Secondary language code.
///     location_code (str | None): Location code.
///     legal_status (str | None): Legal status.
///     country_of_issuance (str | None): Issuing country code.
///     right_thumb (list[Biometric] | None): Right thumb biometric data.
///     right_pointer_finger (list[Biometric] | None): Right pointer finger biometric data.
///     right_middle_finger (list[Biometric] | None): Right middle finger biometric data.
///     right_ring_finger (list[Biometric] | None): Right ring finger biometric data.
///     right_little_finger (list[Biometric] | None): Right little finger biometric data.
///     left_thumb (list[Biometric] | None): Left thumb biometric data.
///     left_pointer_finger (list[Biometric] | None): Left pointer finger biometric data.
///     left_middle_finger (list[Biometric] | None): Left middle finger biometric data.
///     left_ring_finger (list[Biometric] | None): Left ring finger biometric data.
///     left_little_finger (list[Biometric] | None): Left little finger biometric data.
///     right_iris (list[Biometric] | None): Right iris biometric data.
///     left_iris (list[Biometric] | None): Left iris biometric data.
///     face (list[Biometric] | None): Face biometric data.
///     right_palm (list[Biometric] | None): Right palm biometric data.
///     left_palm (list[Biometric] | None): Left palm biometric data.
///     voice (list[Biometric] | None): Voice biometric data.
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
    pub photo: Option<Vec<u8>>,
    #[pyo3(get)]
    pub photo_format: Option<i64>,
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
    #[getter]
    fn photo<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        self.photo.as_ref().map(|p| PyBytes::new(py, p))
    }

    #[getter]
    fn best_quality_fingers<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        self.best_quality_fingers
            .as_ref()
            .map(|f| PyBytes::new(py, f))
    }

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
        if let Some(ref v) = self.version {
            dict.set_item("version", v)?;
        }
        if let Some(ref v) = self.language {
            dict.set_item("language", v)?;
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
        if let Some(ref v) = self.guardian {
            dict.set_item("guardian", v)?;
        }
        if let Some(ref v) = self.photo {
            dict.set_item("photo", PyBytes::new(py, v))?;
        }
        if let Some(v) = self.photo_format {
            dict.set_item("photoFormat", v)?;
        }
        if let Some(ref v) = self.best_quality_fingers {
            dict.set_item("bestQualityFingers", PyBytes::new(py, v))?;
        }
        if let Some(ref v) = self.secondary_full_name {
            dict.set_item("secondaryFullName", v)?;
        }
        if let Some(ref v) = self.secondary_language {
            dict.set_item("secondaryLanguage", v)?;
        }
        if let Some(ref v) = self.location_code {
            dict.set_item("locationCode", v)?;
        }
        if let Some(ref v) = self.legal_status {
            dict.set_item("legalStatus", v)?;
        }
        if let Some(ref v) = self.country_of_issuance {
            dict.set_item("countryOfIssuance", v)?;
        }

        Ok(dict)
    }
}

fn convert_biometrics(biometrics: &Option<Vec<CoreBiometric>>) -> Option<Vec<Biometric>> {
    biometrics
        .as_ref()
        .map(|v| v.iter().map(Biometric::from).collect())
}

fn convert_biometrics_to_core(biometrics: &Option<Vec<Biometric>>) -> Option<Vec<CoreBiometric>> {
    biometrics
        .as_ref()
        .map(|v| v.iter().map(CoreBiometric::from).collect())
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

/// Result of decoding a Claim 169 QR code.
///
/// Contains the decoded identity data, CWT metadata, verification status,
/// and any X.509 headers from the COSE structure.
///
/// Attributes:
///     claim169 (Claim169): Decoded identity data.
///     cwt_meta (CwtMeta): CWT metadata (issuer, timestamps, etc.).
///     verification_status (str): Signature verification status ("verified", "skipped", etc.).
///     x509_headers (X509Headers): X.509 certificate headers from COSE structure.
#[pyclass]
#[derive(Clone)]
pub struct DecodeResult {
    #[pyo3(get)]
    pub claim169: Claim169,
    #[pyo3(get)]
    pub cwt_meta: CwtMeta,
    #[pyo3(get)]
    pub verification_status: String,
    #[pyo3(get)]
    pub x509_headers: X509Headers,
    #[pyo3(get)]
    pub detected_compression: String,
    /// Key ID from the COSE header, if present.
    pub key_id: Option<Vec<u8>>,
    /// COSE algorithm name (e.g., "EdDSA", "ES256"), if present.
    #[pyo3(get)]
    pub algorithm: Option<String>,
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

    /// Key ID from the COSE header (bytes), or None if not present.
    #[getter]
    fn key_id<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        self.key_id.as_ref().map(|kid| PyBytes::new(py, kid))
    }
}

// ============================================================================
// Crypto Hook Wrappers
// ============================================================================

/// Custom signature verifier for external crypto providers.
///
/// Wraps a Python callback for integration with HSMs, Cloud KMS,
/// remote signing services, smart cards, and TPMs.
///
/// Args:
///     callback: A callable ``(algorithm, key_id, data, signature) -> None``
///         that raises an exception if verification fails.
#[pyclass(name = "SignatureVerifier")]
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

/// Custom decryptor for external crypto providers.
///
/// Wraps a Python callback for integration with HSMs, Cloud KMS,
/// and custom software keystores.
///
/// Args:
///     callback: A callable ``(algorithm, key_id, nonce, aad, ciphertext) -> bytes``
///         that returns decrypted plaintext bytes.
#[pyclass(name = "Decryptor")]
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
#[pyclass(name = "Signer")]
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

/// Custom encryptor for external crypto providers.
///
/// Wraps a Python callback for integration with HSMs, Cloud KMS
/// (AWS KMS, Google Cloud KMS, Azure Key Vault), and custom
/// software keystores.
///
/// Args:
///     callback: A callable ``(algorithm, key_id, nonce, aad, plaintext) -> bytes``
///         that returns encrypted ciphertext bytes.
#[pyclass(name = "Encryptor")]
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
#[pyo3(
    text_signature = "(qr_text, skip_biometrics=False, max_decompressed_bytes=65536, validate_timestamps=True, clock_skew_tolerance_seconds=0)",
    signature = (qr_text, skip_biometrics=false, max_decompressed_bytes=65536, validate_timestamps=true, clock_skew_tolerance_seconds=0)
)]
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

    Ok(core_decode_result_to_py(result))
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
#[pyo3(
    text_signature = "(qr_text, public_key, skip_biometrics=False, max_decompressed_bytes=65536, validate_timestamps=True, clock_skew_tolerance_seconds=0)",
    signature = (qr_text, public_key, skip_biometrics=false, max_decompressed_bytes=65536, validate_timestamps=true, clock_skew_tolerance_seconds=0)
)]
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

    Ok(core_decode_result_to_py(result))
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
#[pyo3(
    text_signature = "(qr_text, public_key, skip_biometrics=False, max_decompressed_bytes=65536, validate_timestamps=True, clock_skew_tolerance_seconds=0)",
    signature = (qr_text, public_key, skip_biometrics=false, max_decompressed_bytes=65536, validate_timestamps=true, clock_skew_tolerance_seconds=0)
)]
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

    Ok(core_decode_result_to_py(result))
}

/// Decode a Claim 169 QR code with Ed25519 signature verification using PEM format
///
/// Args:
///     qr_text: The QR code text content
///     pem: PEM-encoded Ed25519 public key (SPKI format)
///     skip_biometrics: If True, skip decoding biometric data (default: False)
///     max_decompressed_bytes: Maximum decompressed size (default: 65536)
///     validate_timestamps: If True, validate exp/nbf timestamps (default: True)
///     clock_skew_tolerance_seconds: Tolerance for timestamp validation (default: 0)
///
/// Returns:
///     DecodeResult with verification_status indicating if signature is valid
#[pyfunction]
#[pyo3(
    text_signature = "(qr_text, pem, skip_biometrics=False, max_decompressed_bytes=65536, validate_timestamps=True, clock_skew_tolerance_seconds=0)",
    signature = (qr_text, pem, skip_biometrics=false, max_decompressed_bytes=65536, validate_timestamps=true, clock_skew_tolerance_seconds=0)
)]
fn decode_with_ed25519_pem(
    qr_text: &str,
    pem: &str,
    skip_biometrics: bool,
    max_decompressed_bytes: usize,
    validate_timestamps: bool,
    clock_skew_tolerance_seconds: i64,
) -> PyResult<DecodeResult> {
    let mut decoder = Decoder::new(qr_text)
        .verify_with_ed25519_pem(pem)
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

    Ok(core_decode_result_to_py(result))
}

/// Decode a Claim 169 QR code with ECDSA P-256 signature verification using PEM format
///
/// Args:
///     qr_text: The QR code text content
///     pem: PEM-encoded ECDSA P-256 public key (SPKI format)
///     skip_biometrics: If True, skip decoding biometric data (default: False)
///     max_decompressed_bytes: Maximum decompressed size (default: 65536)
///     validate_timestamps: If True, validate exp/nbf timestamps (default: True)
///     clock_skew_tolerance_seconds: Tolerance for timestamp validation (default: 0)
///
/// Returns:
///     DecodeResult with verification_status indicating if signature is valid
#[pyfunction]
#[pyo3(
    text_signature = "(qr_text, pem, skip_biometrics=False, max_decompressed_bytes=65536, validate_timestamps=True, clock_skew_tolerance_seconds=0)",
    signature = (qr_text, pem, skip_biometrics=false, max_decompressed_bytes=65536, validate_timestamps=true, clock_skew_tolerance_seconds=0)
)]
fn decode_with_ecdsa_p256_pem(
    qr_text: &str,
    pem: &str,
    skip_biometrics: bool,
    max_decompressed_bytes: usize,
    validate_timestamps: bool,
    clock_skew_tolerance_seconds: i64,
) -> PyResult<DecodeResult> {
    let mut decoder = Decoder::new(qr_text)
        .verify_with_ecdsa_p256_pem(pem)
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

    Ok(core_decode_result_to_py(result))
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
///     skip_biometrics: If True, skip decoding biometric data (default: False)
///     max_decompressed_bytes: Maximum decompressed size (default: 65536)
///     validate_timestamps: If True, validate exp/nbf timestamps (default: True)
///     clock_skew_tolerance_seconds: Tolerance for timestamp validation (default: 0)
///
/// Example:
///     def my_verify(algorithm, key_id, data, signature):
///         # Call your crypto provider here
///         crypto_provider.verify(key_id, data, signature)
///
///     result = decode_with_verifier(qr_text, my_verify)
#[pyfunction]
#[pyo3(
    name = "decode_with_verifier",
    text_signature = "(qr_text, verifier, skip_biometrics=False, max_decompressed_bytes=65536, validate_timestamps=True, clock_skew_tolerance_seconds=0)",
    signature = (qr_text, verifier, skip_biometrics=false, max_decompressed_bytes=65536, validate_timestamps=true, clock_skew_tolerance_seconds=0)
)]
fn py_decode_with_verifier(
    qr_text: &str,
    verifier: Py<PyAny>,
    skip_biometrics: bool,
    max_decompressed_bytes: usize,
    validate_timestamps: bool,
    clock_skew_tolerance_seconds: i64,
) -> PyResult<DecodeResult> {
    let py_verifier = PySignatureVerifier::new(verifier);
    let mut decoder = Decoder::new(qr_text)
        .verify_with(py_verifier)
        .max_decompressed_bytes(max_decompressed_bytes)
        .clock_skew_tolerance(clock_skew_tolerance_seconds);

    if skip_biometrics {
        decoder = decoder.skip_biometrics();
    }

    if !validate_timestamps {
        decoder = decoder.without_timestamp_validation();
    }

    let result = decoder.decode().map_err(to_py_err)?;

    Ok(core_decode_result_to_py(result))
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
#[pyo3(
    text_signature = "(qr_text, key, verifier=None, allow_unverified=False)",
    signature = (qr_text, key, verifier=None, allow_unverified=false)
)]
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

    Ok(core_decode_result_to_py(result))
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
#[pyo3(
    text_signature = "(qr_text, key, verifier=None, allow_unverified=False)",
    signature = (qr_text, key, verifier=None, allow_unverified=false)
)]
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
#[pyo3(
    text_signature = "(qr_text, key, verifier=None, allow_unverified=False)",
    signature = (qr_text, key, verifier=None, allow_unverified=false)
)]
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
#[pyo3(
    text_signature = "(qr_text, decryptor, verifier=None, allow_unverified=False)",
    signature = (qr_text, decryptor, verifier=None, allow_unverified=false)
)]
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

    Ok(core_decode_result_to_py(result))
}

/// Get the library version.
///
/// Returns:
///     str: Version string in semver format (e.g., "0.1.0").
#[pyfunction]
#[pyo3(text_signature = "()")]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

// ============================================================================
// Encoder Classes and Functions
// ============================================================================

/// Input data class for encoding a Claim 169 credential.
///
/// Create an instance and set the desired demographic fields before
/// passing to an encode function.
///
/// Args:
///     id: Unique identifier.
///     full_name: Full name.
///
/// Attributes:
///     id (str | None): Unique identifier.
///     version (str | None): Credential version.
///     language (str | None): Primary language code.
///     full_name (str | None): Full name.
///     first_name (str | None): First/given name.
///     middle_name (str | None): Middle name.
///     last_name (str | None): Last/family name.
///     date_of_birth (str | None): Date of birth (YYYY-MM-DD).
///     gender (int | None): Gender (1=Male, 2=Female, 3=Other).
///     address (str | None): Full address.
///     email (str | None): Email address.
///     phone (str | None): Phone number.
///     nationality (str | None): Nationality code.
///     marital_status (int | None): Marital status (1=Unmarried, 2=Married, 3=Divorced).
///     guardian (str | None): Guardian name.
///     photo (bytes | None): Photo data bytes.
///     photo_format (int | None): Photo format (1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP).
///     secondary_full_name (str | None): Name in secondary language.
///     secondary_language (str | None): Secondary language code.
///     location_code (str | None): Location code.
///     legal_status (str | None): Legal status.
///     country_of_issuance (str | None): Issuing country code.
///
/// Example:
///     >>> claim = Claim169Input(id="12345", full_name="Jane Doe")
///     >>> claim.date_of_birth = "1990-01-15"
///     >>> claim.gender = 2  # Female
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
    #[pyo3(set)]
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

    // Biometrics
    #[pyo3(get, set)]
    pub right_thumb: Option<Vec<Biometric>>,
    #[pyo3(get, set)]
    pub right_pointer_finger: Option<Vec<Biometric>>,
    #[pyo3(get, set)]
    pub right_middle_finger: Option<Vec<Biometric>>,
    #[pyo3(get, set)]
    pub right_ring_finger: Option<Vec<Biometric>>,
    #[pyo3(get, set)]
    pub right_little_finger: Option<Vec<Biometric>>,
    #[pyo3(get, set)]
    pub left_thumb: Option<Vec<Biometric>>,
    #[pyo3(get, set)]
    pub left_pointer_finger: Option<Vec<Biometric>>,
    #[pyo3(get, set)]
    pub left_middle_finger: Option<Vec<Biometric>>,
    #[pyo3(get, set)]
    pub left_ring_finger: Option<Vec<Biometric>>,
    #[pyo3(get, set)]
    pub left_little_finger: Option<Vec<Biometric>>,
    #[pyo3(get, set)]
    pub right_iris: Option<Vec<Biometric>>,
    #[pyo3(get, set)]
    pub left_iris: Option<Vec<Biometric>>,
    #[pyo3(get, set)]
    pub face: Option<Vec<Biometric>>,
    #[pyo3(get, set)]
    pub right_palm: Option<Vec<Biometric>>,
    #[pyo3(get, set)]
    pub left_palm: Option<Vec<Biometric>>,
    #[pyo3(get, set)]
    pub voice: Option<Vec<Biometric>>,
}

#[pymethods]
impl Claim169Input {
    #[getter]
    fn photo<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        self.photo.as_ref().map(|p| PyBytes::new(py, p))
    }

    #[new]
    #[pyo3(signature = (
        id=None, version=None, language=None, full_name=None,
        first_name=None, middle_name=None, last_name=None,
        date_of_birth=None, gender=None, address=None,
        email=None, phone=None, nationality=None,
        marital_status=None, guardian=None, photo=None,
        photo_format=None, secondary_full_name=None,
        secondary_language=None, location_code=None,
        legal_status=None, country_of_issuance=None,
        right_thumb=None, right_pointer_finger=None, right_middle_finger=None,
        right_ring_finger=None, right_little_finger=None,
        left_thumb=None, left_pointer_finger=None, left_middle_finger=None,
        left_ring_finger=None, left_little_finger=None,
        right_iris=None, left_iris=None, face=None,
        right_palm=None, left_palm=None, voice=None,
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
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
        right_thumb: Option<Vec<Biometric>>,
        right_pointer_finger: Option<Vec<Biometric>>,
        right_middle_finger: Option<Vec<Biometric>>,
        right_ring_finger: Option<Vec<Biometric>>,
        right_little_finger: Option<Vec<Biometric>>,
        left_thumb: Option<Vec<Biometric>>,
        left_pointer_finger: Option<Vec<Biometric>>,
        left_middle_finger: Option<Vec<Biometric>>,
        left_ring_finger: Option<Vec<Biometric>>,
        left_little_finger: Option<Vec<Biometric>>,
        right_iris: Option<Vec<Biometric>>,
        left_iris: Option<Vec<Biometric>>,
        face: Option<Vec<Biometric>>,
        right_palm: Option<Vec<Biometric>>,
        left_palm: Option<Vec<Biometric>>,
        voice: Option<Vec<Biometric>>,
    ) -> Self {
        Claim169Input {
            id,
            full_name,
            version,
            language,
            first_name,
            middle_name,
            last_name,
            date_of_birth,
            gender,
            address,
            email,
            phone,
            nationality,
            marital_status,
            guardian,
            photo,
            photo_format,
            secondary_full_name,
            secondary_language,
            location_code,
            legal_status,
            country_of_issuance,
            right_thumb,
            right_pointer_finger,
            right_middle_finger,
            right_ring_finger,
            right_little_finger,
            left_thumb,
            left_pointer_finger,
            left_middle_finger,
            left_ring_finger,
            left_little_finger,
            right_iris,
            left_iris,
            face,
            right_palm,
            left_palm,
            voice,
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
            right_thumb: convert_biometrics_to_core(&py.right_thumb),
            right_pointer_finger: convert_biometrics_to_core(&py.right_pointer_finger),
            right_middle_finger: convert_biometrics_to_core(&py.right_middle_finger),
            right_ring_finger: convert_biometrics_to_core(&py.right_ring_finger),
            right_little_finger: convert_biometrics_to_core(&py.right_little_finger),
            left_thumb: convert_biometrics_to_core(&py.left_thumb),
            left_pointer_finger: convert_biometrics_to_core(&py.left_pointer_finger),
            left_middle_finger: convert_biometrics_to_core(&py.left_middle_finger),
            left_ring_finger: convert_biometrics_to_core(&py.left_ring_finger),
            left_little_finger: convert_biometrics_to_core(&py.left_little_finger),
            right_iris: convert_biometrics_to_core(&py.right_iris),
            left_iris: convert_biometrics_to_core(&py.left_iris),
            face: convert_biometrics_to_core(&py.face),
            right_palm: convert_biometrics_to_core(&py.right_palm),
            left_palm: convert_biometrics_to_core(&py.left_palm),
            voice: convert_biometrics_to_core(&py.voice),
            ..Default::default()
        }
    }
}

/// Input data class for CWT (CBOR Web Token) metadata used during encoding.
///
/// Args:
///     issuer: Credential issuer (URL or identifier).
///     expires_at: Expiration timestamp (Unix epoch seconds).
///
/// Attributes:
///     issuer (str | None): Credential issuer.
///     subject (str | None): Subject identifier.
///     expires_at (int | None): Expiration timestamp (Unix epoch seconds).
///     not_before (int | None): Not valid before timestamp (Unix epoch seconds).
///     issued_at (int | None): Issuance timestamp (Unix epoch seconds).
///
/// Example:
///     >>> import time
///     >>> meta = CwtMetaInput(issuer="https://example.org")
///     >>> meta.issued_at = int(time.time())
///     >>> meta.expires_at = meta.issued_at + 86400  # 24 hours
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
#[pyo3(
    text_signature = "(claim169, cwt_meta, private_key, skip_biometrics=False)",
    signature = (claim169, cwt_meta, private_key, skip_biometrics=false)
)]
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

    encoder.encode().map(|r| r.qr_data).map_err(to_py_err)
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
#[pyo3(
    text_signature = "(claim169, cwt_meta, private_key, skip_biometrics=False)",
    signature = (claim169, cwt_meta, private_key, skip_biometrics=false)
)]
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

    encoder.encode().map(|r| r.qr_data).map_err(to_py_err)
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
#[pyo3(
    text_signature = "(claim169, cwt_meta, sign_key, encrypt_key, skip_biometrics=False)",
    signature = (claim169, cwt_meta, sign_key, encrypt_key, skip_biometrics=false)
)]
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

    encoder.encode().map(|r| r.qr_data).map_err(to_py_err)
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
#[pyo3(
    text_signature = "(claim169, cwt_meta, sign_key, encrypt_key, skip_biometrics=False)",
    signature = (claim169, cwt_meta, sign_key, encrypt_key, skip_biometrics=false)
)]
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

    encoder.encode().map(|r| r.qr_data).map_err(to_py_err)
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
#[pyo3(
    text_signature = "(claim169, cwt_meta, skip_biometrics=False)",
    signature = (claim169, cwt_meta, skip_biometrics=false)
)]
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

    encoder.encode().map(|r| r.qr_data).map_err(to_py_err)
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
#[pyo3(
    text_signature = "(claim169, cwt_meta, signer, algorithm, key_id=None, skip_biometrics=False)",
    signature = (claim169, cwt_meta, signer, algorithm, key_id=None, skip_biometrics=false)
)]
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

    encoder.encode().map(|r| r.qr_data).map_err(to_py_err)
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
#[pyo3(
    text_signature = "(claim169, cwt_meta, signer, sign_algorithm, encryptor, encrypt_algorithm, key_id=None, skip_biometrics=False)",
    signature = (claim169, cwt_meta, signer, sign_algorithm, encryptor, encrypt_algorithm, key_id=None, skip_biometrics=false)
)]
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

    encoder.encode().map(|r| r.qr_data).map_err(to_py_err)
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
#[pyo3(
    text_signature = "(claim169, cwt_meta, sign_key, encryptor, encrypt_algorithm, skip_biometrics=False)",
    signature = (claim169, cwt_meta, sign_key, encryptor, encrypt_algorithm, skip_biometrics=false)
)]
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

    encoder.encode().map(|r| r.qr_data).map_err(to_py_err)
}

/// Generate a random 12-byte nonce for AES-GCM encryption.
///
/// Returns:
///     bytes: 12-byte nonce suitable for AES-GCM IV.
#[pyfunction]
#[pyo3(text_signature = "()")]
fn generate_nonce<'py>(py: Python<'py>) -> Bound<'py, PyBytes> {
    let nonce = claim169_core::generate_random_nonce();
    PyBytes::new(py, &nonce)
}

// ============================================================================
// Inspect (metadata extraction without full decode)
// ============================================================================

/// Metadata extracted from a credential without full verification or decoding.
///
/// Useful for determining which key to use in multi-issuer scenarios.
///
/// Attributes:
///     issuer (str | None): Token issuer from CWT claims.
///     subject (str | None): Token subject from CWT claims.
///     key_id (bytes | None): Key ID from the COSE header.
///     algorithm (str | None): COSE algorithm name (e.g., "EdDSA", "ES256").
///     x509_headers (X509Headers): X.509 certificate headers from COSE structure.
///     expires_at (int | None): Expiration timestamp (Unix epoch seconds).
///     cose_type (str): COSE structure type: "Sign1" or "Encrypt0".
#[pyclass]
#[derive(Clone)]
pub struct InspectResult {
    #[pyo3(get)]
    pub issuer: Option<String>,
    #[pyo3(get)]
    pub subject: Option<String>,
    pub key_id_bytes: Option<Vec<u8>>,
    #[pyo3(get)]
    pub algorithm: Option<String>,
    #[pyo3(get)]
    pub x509_headers: X509Headers,
    #[pyo3(get)]
    pub expires_at: Option<i64>,
    #[pyo3(get)]
    pub cose_type: String,
}

#[pymethods]
impl InspectResult {
    /// Key ID from the COSE header (bytes), or None if not present.
    #[getter]
    fn key_id<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        self.key_id_bytes.as_ref().map(|kid| PyBytes::new(py, kid))
    }

    fn __repr__(&self) -> String {
        format!(
            "InspectResult(issuer={:?}, cose_type={}, algorithm={:?})",
            self.issuer, self.cose_type, self.algorithm
        )
    }
}

/// Inspect credential metadata without full decoding or verification.
///
/// Extracts metadata (issuer, key ID, algorithm, expiration) from a QR code
/// without verifying the signature. For encrypted credentials, only COSE-level
/// headers are available; CWT-level fields (issuer, subject, expires_at) will be None.
///
/// This is useful for multi-issuer or key-rotation scenarios where you need to
/// determine which verification key to use before decoding.
///
/// Args:
///     qr_text (str): Base45-encoded QR code content.
///
/// Returns:
///     InspectResult: Metadata extracted from the credential.
///
/// Raises:
///     Base45DecodeError: If the QR text is not valid Base45.
///     DecompressError: If decompression fails.
///     CoseParseError: If the COSE structure is invalid.
#[pyfunction]
#[pyo3(text_signature = "(qr_text)")]
fn inspect(qr_text: &str) -> PyResult<InspectResult> {
    let result = claim169_core::inspect(qr_text).map_err(to_py_err)?;
    let cose_type = match result.cose_type {
        claim169_core::pipeline::CoseType::Sign1 => "Sign1",
        claim169_core::pipeline::CoseType::Encrypt0 => "Encrypt0",
    };
    Ok(InspectResult {
        issuer: result.issuer,
        subject: result.subject,
        key_id_bytes: result.key_id,
        algorithm: result.algorithm.as_ref().map(algorithm_to_string),
        x509_headers: X509Headers::from(&result.x509_headers),
        expires_at: result.expires_at,
        cose_type: cose_type.to_string(),
    })
}

// ============================================================================
// Module Definition
// ============================================================================

/// MOSIP Claim 169 QR Code library for encoding and decoding identity credentials.
///
/// This module provides Python bindings for the claim169-core Rust library,
/// supporting QR code encoding, decoding, signature verification, and
/// encryption/decryption of MOSIP Claim 169 identity credentials.
///
/// Features:
///     - Decode identity credentials from Base45-encoded QR codes
///     - Verify signatures using Ed25519 or ECDSA P-256 (raw bytes or PEM format)
///     - Encrypt/decrypt credentials with AES-128/256-GCM
///     - Custom crypto hooks for HSM/KMS integration
///     - Encode identity data into QR-ready Base45 strings
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
    m.add_class::<CertificateHash>()?;
    m.add_class::<X509Headers>()?;
    m.add_class::<Claim169>()?;
    m.add_class::<DecodeResult>()?;
    m.add_class::<PySignatureVerifier>()?;
    m.add_class::<PyDecryptor>()?;
    m.add_class::<PySigner>()?;
    m.add_class::<PyEncryptor>()?;
    m.add_class::<Claim169Input>()?;
    m.add_class::<CwtMetaInput>()?;
    m.add_class::<InspectResult>()?;

    // Add decode functions
    m.add_function(wrap_pyfunction!(decode_unverified, m)?)?;
    m.add_function(wrap_pyfunction!(decode_with_ed25519, m)?)?;
    m.add_function(wrap_pyfunction!(decode_with_ecdsa_p256, m)?)?;
    m.add_function(wrap_pyfunction!(decode_with_ed25519_pem, m)?)?;
    m.add_function(wrap_pyfunction!(decode_with_ecdsa_p256_pem, m)?)?;
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

    // Inspect
    m.add_function(wrap_pyfunction!(inspect, m)?)?;

    // Utilities
    m.add_function(wrap_pyfunction!(version, m)?)?;

    Ok(())
}
