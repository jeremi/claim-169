//! Configuration options for decoding Claim 169 QR codes.
//!
//! The [`DecodeOptions`] struct controls various aspects of the decoding process,
//! including security settings, size limits, and what data to parse.
//!
//! # Presets
//!
//! For convenience, several presets are available:
//!
//! | Preset | Security | Use Case |
//! |--------|----------|----------|
//! | [`DecodeOptions::default()`] | High | Production with verification |
//! | [`DecodeOptions::strict()`] | Highest | High-security environments |
//! | [`DecodeOptions::permissive()`] | Low | Development and debugging |
//!
//! # Builder Pattern
//!
//! Options can be customized using the builder pattern:
//!
//! ```rust
//! use claim169_core::DecodeOptions;
//!
//! let options = DecodeOptions::new()
//!     .with_max_decompressed_bytes(32768)  // 32KB limit
//!     .skip_biometrics()                    // Skip biometric parsing
//!     .with_clock_skew_tolerance(60);       // 60 seconds tolerance
//! ```
//!
//! # Security Defaults
//!
//! By default, `DecodeOptions` enforces secure settings:
//!
//! - **Verification required**: Decoding fails if no verifier is provided
//! - **Timestamp validation**: Expired or not-yet-valid credentials are rejected
//! - **Decompression limit**: 64KB max to prevent zip bomb attacks
//! - **No clock skew tolerance**: Strict timestamp checking

/// Options for decoding Claim 169 QR codes.
///
/// Controls security settings, size limits, and parsing behavior.
/// Use the builder pattern to customize options, or use presets like
/// [`strict()`](Self::strict) or [`permissive()`](Self::permissive).
///
/// # Example
///
/// ```rust
/// use claim169_core::DecodeOptions;
///
/// // Production settings with custom clock skew tolerance
/// let options = DecodeOptions::default()
///     .with_clock_skew_tolerance(30);
///
/// // Development settings (INSECURE)
/// let dev_options = DecodeOptions::permissive();
/// ```
#[derive(Debug, Clone)]
pub struct DecodeOptions {
    /// Maximum allowed size of decompressed data in bytes.
    /// This is a safety limit to prevent decompression bombs.
    /// Default: 65536 (64 KB)
    pub max_decompressed_bytes: usize,

    /// Skip parsing biometric data fields.
    /// Useful when you only need demographic data and want faster parsing.
    /// Default: false
    pub skip_biometrics: bool,

    /// Validate timestamp claims (exp, nbf) against current time.
    /// If true, expired or not-yet-valid credentials will return an error.
    /// Default: true
    pub validate_timestamps: bool,

    /// Allow parsing without signature verification.
    /// If true and no verifier is provided, the decode will succeed
    /// but verification_status will be "skipped".
    /// Default: false (secure by default - verification required)
    pub allow_unverified: bool,

    /// Clock skew tolerance in seconds for timestamp validation.
    /// This allows credentials to be accepted even if clocks are slightly
    /// out of sync between issuer and verifier.
    /// Default: 0 (no tolerance)
    pub clock_skew_tolerance_seconds: i64,
}

impl Default for DecodeOptions {
    fn default() -> Self {
        Self {
            max_decompressed_bytes: 65536, // 64 KB
            skip_biometrics: false,
            validate_timestamps: true,
            allow_unverified: false, // Secure default: require verification
            clock_skew_tolerance_seconds: 0, // No tolerance by default
        }
    }
}

impl DecodeOptions {
    /// Create options with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum decompressed bytes limit
    pub fn with_max_decompressed_bytes(mut self, bytes: usize) -> Self {
        self.max_decompressed_bytes = bytes;
        self
    }

    /// Skip biometric data parsing
    pub fn skip_biometrics(mut self) -> Self {
        self.skip_biometrics = true;
        self
    }

    /// Disable timestamp validation
    pub fn without_timestamp_validation(mut self) -> Self {
        self.validate_timestamps = false;
        self
    }

    /// Require signature verification (fail if no verifier)
    pub fn require_verification(mut self) -> Self {
        self.allow_unverified = false;
        self
    }

    /// Allow parsing without signature verification (INSECURE - use only for testing/debugging)
    ///
    /// WARNING: This disables signature verification. Only use this for:
    /// - Testing and development
    /// - Parsing credentials where you will verify signatures separately
    /// - Debugging credential contents
    pub fn allow_unverified(mut self) -> Self {
        self.allow_unverified = true;
        self
    }

    /// Set clock skew tolerance in seconds
    ///
    /// This allows credentials to be accepted even if clocks are slightly
    /// out of sync between issuer and verifier. A tolerance of 60 seconds
    /// is commonly used in production deployments.
    ///
    /// # Arguments
    /// * `seconds` - The number of seconds of tolerance (must be non-negative)
    pub fn with_clock_skew_tolerance(mut self, seconds: i64) -> Self {
        self.clock_skew_tolerance_seconds = seconds.max(0);
        self
    }

    /// Create strict options (small limit, require verification, validate timestamps)
    pub fn strict() -> Self {
        Self {
            max_decompressed_bytes: 32768, // 32 KB
            skip_biometrics: false,
            validate_timestamps: true,
            allow_unverified: false,
            clock_skew_tolerance_seconds: 0,
        }
    }

    /// Create permissive options (large limit, skip verification, no timestamp check)
    pub fn permissive() -> Self {
        Self {
            max_decompressed_bytes: 1024 * 1024, // 1 MB
            skip_biometrics: false,
            validate_timestamps: false,
            allow_unverified: true,
            clock_skew_tolerance_seconds: 300, // 5 minutes tolerance
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let opts = DecodeOptions::default();
        assert_eq!(opts.max_decompressed_bytes, 65536);
        assert!(!opts.skip_biometrics);
        assert!(opts.validate_timestamps);
        // Secure default: verification is required
        assert!(!opts.allow_unverified);
        // No clock skew tolerance by default
        assert_eq!(opts.clock_skew_tolerance_seconds, 0);
    }

    #[test]
    fn test_allow_unverified_explicit_opt_in() {
        let opts = DecodeOptions::new().allow_unverified();
        assert!(opts.allow_unverified);
    }

    #[test]
    fn test_builder_pattern() {
        let opts = DecodeOptions::new()
            .with_max_decompressed_bytes(1000)
            .skip_biometrics()
            .without_timestamp_validation();

        assert_eq!(opts.max_decompressed_bytes, 1000);
        assert!(opts.skip_biometrics);
        assert!(!opts.validate_timestamps);
    }

    #[test]
    fn test_strict_options() {
        let opts = DecodeOptions::strict();
        assert_eq!(opts.max_decompressed_bytes, 32768);
        assert!(!opts.allow_unverified);
    }

    #[test]
    fn test_permissive_options() {
        let opts = DecodeOptions::permissive();
        assert_eq!(opts.max_decompressed_bytes, 1024 * 1024);
        assert!(!opts.validate_timestamps);
        // Permissive options have 5 minutes tolerance
        assert_eq!(opts.clock_skew_tolerance_seconds, 300);
    }

    #[test]
    fn test_clock_skew_tolerance_builder() {
        let opts = DecodeOptions::new().with_clock_skew_tolerance(60);
        assert_eq!(opts.clock_skew_tolerance_seconds, 60);
    }

    #[test]
    fn test_clock_skew_tolerance_rejects_negative() {
        // Negative values should be clamped to 0
        let opts = DecodeOptions::new().with_clock_skew_tolerance(-100);
        assert_eq!(opts.clock_skew_tolerance_seconds, 0);
    }

    #[test]
    fn test_strict_options_no_clock_skew() {
        let opts = DecodeOptions::strict();
        assert_eq!(opts.clock_skew_tolerance_seconds, 0);
    }
}
