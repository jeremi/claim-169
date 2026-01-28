//! X.509 certificate-related types for COSE headers.
//!
//! This module contains types for X.509 certificate information
//! that can be included in COSE protected/unprotected headers.
//!
//! ## COSE X.509 Header Parameters (RFC 9360)
//!
//! | Label | Name | Description |
//! |-------|------|-------------|
//! | 32 | x5bag | Unordered bag of X.509 certificates (DER-encoded) |
//! | 33 | x5chain | Ordered chain of X.509 certificates (DER-encoded) |
//! | 34 | x5t | Certificate thumbprint (hash) |
//! | 35 | x5u | URI pointing to X.509 certificate |

use serde::{Deserialize, Serialize};

/// Algorithm identifier for certificate hash.
///
/// Can be either a numeric COSE algorithm ID or a named string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CertHashAlgorithm {
    /// Numeric COSE algorithm identifier (e.g., -16 for SHA-256)
    Numeric(i64),
    /// Named algorithm string (for compatibility)
    Named(String),
}

impl Default for CertHashAlgorithm {
    fn default() -> Self {
        // SHA-256 is the most common default
        CertHashAlgorithm::Numeric(-16)
    }
}

/// X.509 certificate hash (COSE_CertHash).
///
/// Contains the hash algorithm and the hash value of a certificate.
/// Used with the x5t (label 34) header parameter.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateHash {
    /// Hash algorithm identifier
    pub algorithm: CertHashAlgorithm,

    /// Hash value (raw bytes, serialized as base64 in JSON)
    #[serde(with = "crate::serde_utils::base64_bytes")]
    pub hash_value: Vec<u8>,
}

/// X.509 headers extracted from COSE protected/unprotected headers.
///
/// These headers provide certificate information for signature verification.
/// Fields are extracted from both protected and unprotected headers,
/// with protected taking precedence.
///
/// ## Header Labels (RFC 9360)
///
/// - **x5bag (32)**: Unordered bag of X.509 certificates
/// - **x5chain (33)**: Ordered certificate chain (leaf first, root last)
/// - **x5t (34)**: Certificate thumbprint for key lookup
/// - **x5u (35)**: URI to retrieve the certificate
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct X509Headers {
    /// Unordered bag of X.509 certificates (DER-encoded, base64 in JSON).
    ///
    /// COSE label: 32 (x5bag)
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "crate::serde_utils::option_vec_base64"
    )]
    pub x5bag: Option<Vec<Vec<u8>>>,

    /// Ordered X.509 certificate chain (DER-encoded, base64 in JSON).
    ///
    /// The first certificate is the leaf (end-entity), and the last
    /// is the root or trust anchor.
    ///
    /// COSE label: 33 (x5chain)
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "crate::serde_utils::option_vec_base64"
    )]
    pub x5chain: Option<Vec<Vec<u8>>>,

    /// Certificate thumbprint hash for key lookup.
    ///
    /// COSE label: 34 (x5t)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x5t: Option<CertificateHash>,

    /// URI pointing to an X.509 certificate or certificate chain.
    ///
    /// COSE label: 35 (x5u)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x5u: Option<String>,
}

impl X509Headers {
    /// Create empty X509Headers with no certificates.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any X.509 headers are present.
    pub fn is_empty(&self) -> bool {
        self.x5bag.is_none() && self.x5chain.is_none() && self.x5t.is_none() && self.x5u.is_none()
    }

    /// Check if any certificates are present (x5bag or x5chain).
    pub fn has_certificates(&self) -> bool {
        self.x5bag.is_some() || self.x5chain.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x509_headers_default() {
        let headers = X509Headers::default();
        assert!(headers.is_empty());
        assert!(!headers.has_certificates());
    }

    #[test]
    fn test_x509_headers_with_x5bag() {
        let headers = X509Headers {
            x5bag: Some(vec![vec![1, 2, 3]]),
            ..Default::default()
        };
        assert!(!headers.is_empty());
        assert!(headers.has_certificates());
    }

    #[test]
    fn test_x509_headers_with_x5chain() {
        let headers = X509Headers {
            x5chain: Some(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            ..Default::default()
        };
        assert!(!headers.is_empty());
        assert!(headers.has_certificates());
    }

    #[test]
    fn test_x509_headers_with_x5t() {
        let headers = X509Headers {
            x5t: Some(CertificateHash {
                algorithm: CertHashAlgorithm::Numeric(-16),
                hash_value: vec![0xab; 32],
            }),
            ..Default::default()
        };
        assert!(!headers.is_empty());
        assert!(!headers.has_certificates()); // x5t is not a certificate
    }

    #[test]
    fn test_x509_headers_with_x5u() {
        let headers = X509Headers {
            x5u: Some("https://example.com/cert.pem".to_string()),
            ..Default::default()
        };
        assert!(!headers.is_empty());
        assert!(!headers.has_certificates()); // x5u is just a URI
    }

    #[test]
    fn test_cert_hash_algorithm_serde() {
        // Numeric
        let numeric = CertHashAlgorithm::Numeric(-16);
        let json = serde_json::to_string(&numeric).unwrap();
        assert_eq!(json, "-16");

        // Named
        let named = CertHashAlgorithm::Named("sha-256".to_string());
        let json = serde_json::to_string(&named).unwrap();
        assert_eq!(json, r#""sha-256""#);
    }

    #[test]
    fn test_x509_headers_json_serialization() {
        let headers = X509Headers {
            x5chain: Some(vec![vec![1, 2, 3]]),
            x5u: Some("https://example.com/cert.pem".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&headers).unwrap();
        assert!(json.contains("x5chain"));
        assert!(json.contains("x5u"));
        assert!(!json.contains("x5bag")); // should be skipped
        assert!(!json.contains("x5t")); // should be skipped
    }

    #[test]
    fn test_x509_headers_equality() {
        let h1 = X509Headers {
            x5u: Some("https://example.com".to_string()),
            ..Default::default()
        };
        let h2 = X509Headers {
            x5u: Some("https://example.com".to_string()),
            ..Default::default()
        };
        let h3 = X509Headers {
            x5u: Some("https://other.com".to_string()),
            ..Default::default()
        };

        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
    }
}
