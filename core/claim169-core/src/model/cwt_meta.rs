use serde::{Deserialize, Serialize};

/// CWT (CBOR Web Token) standard claims metadata
///
/// These are standard CWT claims defined outside of Claim 169,
/// extracted from the CWT payload for convenience.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CwtMeta {
    /// Issuer (CWT claim 1) - Identifier of the entity issuing the credential
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    /// Subject (CWT claim 2) - Identifier of the subject of the credential
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// Expiration Time (CWT claim 4) - Unix timestamp when credential expires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,

    /// Not Before (CWT claim 5) - Unix timestamp before which credential is invalid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<i64>,

    /// Issued At (CWT claim 6) - Unix timestamp when credential was issued
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<i64>,
}

impl CwtMeta {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_issuer(mut self, issuer: impl Into<String>) -> Self {
        self.issuer = Some(issuer.into());
        self
    }

    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    pub fn with_expires_at(mut self, expires_at: i64) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    pub fn with_not_before(mut self, not_before: i64) -> Self {
        self.not_before = Some(not_before);
        self
    }

    pub fn with_issued_at(mut self, issued_at: i64) -> Self {
        self.issued_at = Some(issued_at);
        self
    }

    /// Check if the credential is currently valid based on timestamps
    pub fn is_time_valid(&self, current_time: i64) -> bool {
        // Check expiration
        if let Some(exp) = self.expires_at {
            if current_time > exp {
                return false;
            }
        }

        // Check not-before
        if let Some(nbf) = self.not_before {
            if current_time < nbf {
                return false;
            }
        }

        true
    }

    /// Check if the credential is expired
    pub fn is_expired(&self, current_time: i64) -> bool {
        self.expires_at
            .map(|exp| current_time > exp)
            .unwrap_or(false)
    }

    /// Check if the credential is not yet valid
    pub fn is_not_yet_valid(&self, current_time: i64) -> bool {
        self.not_before
            .map(|nbf| current_time < nbf)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cwt_meta_builder() {
        let meta = CwtMeta::new()
            .with_issuer("https://mosip.io")
            .with_expires_at(1787912445)
            .with_issued_at(1756376445);

        assert_eq!(meta.issuer, Some("https://mosip.io".to_string()));
        assert_eq!(meta.expires_at, Some(1787912445));
        assert_eq!(meta.issued_at, Some(1756376445));
    }

    #[test]
    fn test_time_validity() {
        let meta = CwtMeta::new().with_not_before(1000).with_expires_at(2000);

        assert!(!meta.is_time_valid(500)); // too early
        assert!(meta.is_time_valid(1500)); // valid
        assert!(!meta.is_time_valid(2500)); // expired
    }

    #[test]
    fn test_expired_check() {
        let meta = CwtMeta::new().with_expires_at(1000);

        assert!(!meta.is_expired(500));
        assert!(meta.is_expired(1500));
    }

    #[test]
    fn test_json_serialization() {
        let meta = CwtMeta::new().with_issuer("test").with_expires_at(12345);

        let json = serde_json::to_string(&meta).unwrap();
        assert!(json.contains("\"issuer\":\"test\""));
        assert!(json.contains("\"expiresAt\":12345"));

        // Fields that are None should not appear
        assert!(!json.contains("subject"));
    }
}
