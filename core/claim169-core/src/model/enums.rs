use serde::{Deserialize, Serialize};

/// Gender values as defined in MOSIP Claim 169
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "i64", try_from = "i64")]
pub enum Gender {
    Male = 1,
    Female = 2,
    Other = 3,
}

impl From<Gender> for i64 {
    fn from(g: Gender) -> i64 {
        g as i64
    }
}

impl TryFrom<i64> for Gender {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Gender::Male),
            2 => Ok(Gender::Female),
            3 => Ok(Gender::Other),
            _ => Err("invalid gender value"),
        }
    }
}

/// Marital status values as defined in MOSIP Claim 169
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "i64", try_from = "i64")]
pub enum MaritalStatus {
    Unmarried = 1,
    Married = 2,
    Divorced = 3,
}

impl From<MaritalStatus> for i64 {
    fn from(m: MaritalStatus) -> i64 {
        m as i64
    }
}

impl TryFrom<i64> for MaritalStatus {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MaritalStatus::Unmarried),
            2 => Ok(MaritalStatus::Married),
            3 => Ok(MaritalStatus::Divorced),
            _ => Err("invalid marital status value"),
        }
    }
}

/// Photo format for the binary image field (key 17)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "i64", try_from = "i64")]
pub enum PhotoFormat {
    Jpeg = 1,
    Jpeg2000 = 2,
    Avif = 3,
    Webp = 4,
}

impl From<PhotoFormat> for i64 {
    fn from(f: PhotoFormat) -> i64 {
        f as i64
    }
}

impl TryFrom<i64> for PhotoFormat {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(PhotoFormat::Jpeg),
            2 => Ok(PhotoFormat::Jpeg2000),
            3 => Ok(PhotoFormat::Avif),
            4 => Ok(PhotoFormat::Webp),
            _ => Err("invalid photo format value"),
        }
    }
}

/// Biometric data format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "i64", try_from = "i64")]
pub enum BiometricFormat {
    Image = 0,
    Template = 1,
    Sound = 2,
    BioHash = 3,
}

impl From<BiometricFormat> for i64 {
    fn from(f: BiometricFormat) -> i64 {
        f as i64
    }
}

impl TryFrom<i64> for BiometricFormat {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BiometricFormat::Image),
            1 => Ok(BiometricFormat::Template),
            2 => Ok(BiometricFormat::Sound),
            3 => Ok(BiometricFormat::BioHash),
            _ => Err("invalid biometric format value"),
        }
    }
}

/// Image sub-formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "i64", try_from = "i64")]
pub enum ImageSubFormat {
    Png,
    Jpeg,
    Jpeg2000,
    Avif,
    Webp,
    Tiff,
    Wsq,
    VendorSpecific(i64),
}

impl From<ImageSubFormat> for i64 {
    fn from(f: ImageSubFormat) -> i64 {
        match f {
            ImageSubFormat::Png => 0,
            ImageSubFormat::Jpeg => 1,
            ImageSubFormat::Jpeg2000 => 2,
            ImageSubFormat::Avif => 3,
            ImageSubFormat::Webp => 4,
            ImageSubFormat::Tiff => 5,
            ImageSubFormat::Wsq => 6,
            ImageSubFormat::VendorSpecific(v) => v,
        }
    }
}

impl TryFrom<i64> for ImageSubFormat {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ImageSubFormat::Png),
            1 => Ok(ImageSubFormat::Jpeg),
            2 => Ok(ImageSubFormat::Jpeg2000),
            3 => Ok(ImageSubFormat::Avif),
            4 => Ok(ImageSubFormat::Webp),
            5 => Ok(ImageSubFormat::Tiff),
            6 => Ok(ImageSubFormat::Wsq),
            100..=200 => Ok(ImageSubFormat::VendorSpecific(value)),
            _ => Err("invalid image sub-format value"),
        }
    }
}

/// Template sub-formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "i64", try_from = "i64")]
pub enum TemplateSubFormat {
    Ansi378,
    Iso19794_2,
    Nist,
    VendorSpecific(i64),
}

impl From<TemplateSubFormat> for i64 {
    fn from(f: TemplateSubFormat) -> i64 {
        match f {
            TemplateSubFormat::Ansi378 => 0,
            TemplateSubFormat::Iso19794_2 => 1,
            TemplateSubFormat::Nist => 2,
            TemplateSubFormat::VendorSpecific(v) => v,
        }
    }
}

impl TryFrom<i64> for TemplateSubFormat {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TemplateSubFormat::Ansi378),
            1 => Ok(TemplateSubFormat::Iso19794_2),
            2 => Ok(TemplateSubFormat::Nist),
            100..=200 => Ok(TemplateSubFormat::VendorSpecific(value)),
            _ => Err("invalid template sub-format value"),
        }
    }
}

/// Sound sub-formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(into = "i64", try_from = "i64")]
pub enum SoundSubFormat {
    Wav = 0,
    Mp3 = 1,
}

impl From<SoundSubFormat> for i64 {
    fn from(f: SoundSubFormat) -> i64 {
        f as i64
    }
}

impl TryFrom<i64> for SoundSubFormat {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SoundSubFormat::Wav),
            1 => Ok(SoundSubFormat::Mp3),
            _ => Err("invalid sound sub-format value"),
        }
    }
}

/// Biometric sub-format (unified across format types)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BiometricSubFormat {
    Image(ImageSubFormat),
    Template(TemplateSubFormat),
    Sound(SoundSubFormat),
    Raw(i64),
}

impl BiometricSubFormat {
    pub fn from_format_and_value(format: BiometricFormat, value: i64) -> Self {
        match format {
            BiometricFormat::Image => ImageSubFormat::try_from(value)
                .map(BiometricSubFormat::Image)
                .unwrap_or(BiometricSubFormat::Raw(value)),
            BiometricFormat::Template => TemplateSubFormat::try_from(value)
                .map(BiometricSubFormat::Template)
                .unwrap_or(BiometricSubFormat::Raw(value)),
            BiometricFormat::Sound => SoundSubFormat::try_from(value)
                .map(BiometricSubFormat::Sound)
                .unwrap_or(BiometricSubFormat::Raw(value)),
            BiometricFormat::BioHash => BiometricSubFormat::Raw(value),
        }
    }

    pub fn to_value(&self) -> i64 {
        match self {
            BiometricSubFormat::Image(f) => (*f).into(),
            BiometricSubFormat::Template(f) => (*f).into(),
            BiometricSubFormat::Sound(f) => (*f).into(),
            BiometricSubFormat::Raw(v) => *v,
        }
    }
}

/// Verification status of the decoded QR
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VerificationStatus {
    Verified,
    Failed,
    Skipped,
}

impl std::fmt::Display for VerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationStatus::Verified => write!(f, "verified"),
            VerificationStatus::Failed => write!(f, "failed"),
            VerificationStatus::Skipped => write!(f, "skipped"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Gender Tests ==========
    #[test]
    fn test_gender_conversion() {
        assert_eq!(i64::from(Gender::Male), 1);
        assert_eq!(i64::from(Gender::Female), 2);
        assert_eq!(i64::from(Gender::Other), 3);
        assert_eq!(Gender::try_from(1).unwrap(), Gender::Male);
        assert_eq!(Gender::try_from(2).unwrap(), Gender::Female);
        assert_eq!(Gender::try_from(3).unwrap(), Gender::Other);
        assert!(Gender::try_from(0).is_err());
        assert!(Gender::try_from(4).is_err());
        assert!(Gender::try_from(99).is_err());
    }

    // ========== MaritalStatus Tests ==========
    #[test]
    fn test_marital_status_conversion() {
        assert_eq!(i64::from(MaritalStatus::Unmarried), 1);
        assert_eq!(i64::from(MaritalStatus::Married), 2);
        assert_eq!(i64::from(MaritalStatus::Divorced), 3);
        assert_eq!(
            MaritalStatus::try_from(1).unwrap(),
            MaritalStatus::Unmarried
        );
        assert_eq!(MaritalStatus::try_from(2).unwrap(), MaritalStatus::Married);
        assert_eq!(MaritalStatus::try_from(3).unwrap(), MaritalStatus::Divorced);
        assert!(MaritalStatus::try_from(0).is_err());
        assert!(MaritalStatus::try_from(4).is_err());
    }

    // ========== PhotoFormat Tests ==========
    #[test]
    fn test_photo_format_conversion() {
        assert_eq!(i64::from(PhotoFormat::Jpeg), 1);
        assert_eq!(i64::from(PhotoFormat::Jpeg2000), 2);
        assert_eq!(i64::from(PhotoFormat::Avif), 3);
        assert_eq!(i64::from(PhotoFormat::Webp), 4);
        assert_eq!(PhotoFormat::try_from(1).unwrap(), PhotoFormat::Jpeg);
        assert_eq!(PhotoFormat::try_from(2).unwrap(), PhotoFormat::Jpeg2000);
        assert_eq!(PhotoFormat::try_from(3).unwrap(), PhotoFormat::Avif);
        assert_eq!(PhotoFormat::try_from(4).unwrap(), PhotoFormat::Webp);
        assert!(PhotoFormat::try_from(0).is_err());
        assert!(PhotoFormat::try_from(5).is_err());
    }

    // ========== BiometricFormat Tests ==========
    #[test]
    fn test_biometric_format_conversion() {
        assert_eq!(i64::from(BiometricFormat::Image), 0);
        assert_eq!(i64::from(BiometricFormat::Template), 1);
        assert_eq!(i64::from(BiometricFormat::Sound), 2);
        assert_eq!(i64::from(BiometricFormat::BioHash), 3);
        assert_eq!(
            BiometricFormat::try_from(0).unwrap(),
            BiometricFormat::Image
        );
        assert_eq!(
            BiometricFormat::try_from(1).unwrap(),
            BiometricFormat::Template
        );
        assert_eq!(
            BiometricFormat::try_from(2).unwrap(),
            BiometricFormat::Sound
        );
        assert_eq!(
            BiometricFormat::try_from(3).unwrap(),
            BiometricFormat::BioHash
        );
        assert!(BiometricFormat::try_from(-1).is_err());
        assert!(BiometricFormat::try_from(4).is_err());
    }

    // ========== ImageSubFormat Tests ==========
    #[test]
    fn test_image_subformat_all_variants() {
        assert_eq!(i64::from(ImageSubFormat::Png), 0);
        assert_eq!(i64::from(ImageSubFormat::Jpeg), 1);
        assert_eq!(i64::from(ImageSubFormat::Jpeg2000), 2);
        assert_eq!(i64::from(ImageSubFormat::Avif), 3);
        assert_eq!(i64::from(ImageSubFormat::Webp), 4);
        assert_eq!(i64::from(ImageSubFormat::Tiff), 5);
        assert_eq!(i64::from(ImageSubFormat::Wsq), 6);
        assert_eq!(i64::from(ImageSubFormat::VendorSpecific(150)), 150);

        assert_eq!(ImageSubFormat::try_from(0).unwrap(), ImageSubFormat::Png);
        assert_eq!(ImageSubFormat::try_from(1).unwrap(), ImageSubFormat::Jpeg);
        assert_eq!(
            ImageSubFormat::try_from(2).unwrap(),
            ImageSubFormat::Jpeg2000
        );
        assert_eq!(ImageSubFormat::try_from(3).unwrap(), ImageSubFormat::Avif);
        assert_eq!(ImageSubFormat::try_from(4).unwrap(), ImageSubFormat::Webp);
        assert_eq!(ImageSubFormat::try_from(5).unwrap(), ImageSubFormat::Tiff);
        assert_eq!(ImageSubFormat::try_from(6).unwrap(), ImageSubFormat::Wsq);
    }

    #[test]
    fn test_image_subformat_vendor_specific() {
        let vendor = ImageSubFormat::try_from(150).unwrap();
        assert!(matches!(vendor, ImageSubFormat::VendorSpecific(150)));
        assert_eq!(i64::from(vendor), 150);

        // Test edge cases of vendor specific range
        let vendor_100 = ImageSubFormat::try_from(100).unwrap();
        assert!(matches!(vendor_100, ImageSubFormat::VendorSpecific(100)));

        let vendor_200 = ImageSubFormat::try_from(200).unwrap();
        assert!(matches!(vendor_200, ImageSubFormat::VendorSpecific(200)));
    }

    #[test]
    fn test_image_subformat_invalid_values() {
        // Values between standard formats and vendor range should fail
        assert!(ImageSubFormat::try_from(7).is_err());
        assert!(ImageSubFormat::try_from(50).is_err());
        assert!(ImageSubFormat::try_from(99).is_err());
        // Values outside vendor range should fail
        assert!(ImageSubFormat::try_from(201).is_err());
        assert!(ImageSubFormat::try_from(-1).is_err());
    }

    // ========== TemplateSubFormat Tests ==========
    #[test]
    fn test_template_subformat_all_variants() {
        assert_eq!(i64::from(TemplateSubFormat::Ansi378), 0);
        assert_eq!(i64::from(TemplateSubFormat::Iso19794_2), 1);
        assert_eq!(i64::from(TemplateSubFormat::Nist), 2);
        assert_eq!(i64::from(TemplateSubFormat::VendorSpecific(175)), 175);

        assert_eq!(
            TemplateSubFormat::try_from(0).unwrap(),
            TemplateSubFormat::Ansi378
        );
        assert_eq!(
            TemplateSubFormat::try_from(1).unwrap(),
            TemplateSubFormat::Iso19794_2
        );
        assert_eq!(
            TemplateSubFormat::try_from(2).unwrap(),
            TemplateSubFormat::Nist
        );
    }

    #[test]
    fn test_template_subformat_vendor_specific() {
        let vendor = TemplateSubFormat::try_from(100).unwrap();
        assert!(matches!(vendor, TemplateSubFormat::VendorSpecific(100)));

        let vendor = TemplateSubFormat::try_from(200).unwrap();
        assert!(matches!(vendor, TemplateSubFormat::VendorSpecific(200)));
    }

    #[test]
    fn test_template_subformat_invalid_values() {
        assert!(TemplateSubFormat::try_from(3).is_err());
        assert!(TemplateSubFormat::try_from(50).is_err());
        assert!(TemplateSubFormat::try_from(99).is_err());
        assert!(TemplateSubFormat::try_from(201).is_err());
        assert!(TemplateSubFormat::try_from(-1).is_err());
    }

    // ========== SoundSubFormat Tests ==========
    #[test]
    fn test_sound_subformat_conversion() {
        assert_eq!(i64::from(SoundSubFormat::Wav), 0);
        assert_eq!(i64::from(SoundSubFormat::Mp3), 1);
        assert_eq!(SoundSubFormat::try_from(0).unwrap(), SoundSubFormat::Wav);
        assert_eq!(SoundSubFormat::try_from(1).unwrap(), SoundSubFormat::Mp3);
        assert!(SoundSubFormat::try_from(2).is_err());
        assert!(SoundSubFormat::try_from(-1).is_err());
    }

    // ========== BiometricSubFormat Tests ==========
    #[test]
    fn test_biometric_subformat_from_format() {
        // Image format
        let sub = BiometricSubFormat::from_format_and_value(BiometricFormat::Image, 6);
        assert!(matches!(
            sub,
            BiometricSubFormat::Image(ImageSubFormat::Wsq)
        ));

        let sub = BiometricSubFormat::from_format_and_value(BiometricFormat::Image, 0);
        assert!(matches!(
            sub,
            BiometricSubFormat::Image(ImageSubFormat::Png)
        ));

        // Template format
        let sub = BiometricSubFormat::from_format_and_value(BiometricFormat::Template, 1);
        assert!(matches!(
            sub,
            BiometricSubFormat::Template(TemplateSubFormat::Iso19794_2)
        ));

        let sub = BiometricSubFormat::from_format_and_value(BiometricFormat::Template, 0);
        assert!(matches!(
            sub,
            BiometricSubFormat::Template(TemplateSubFormat::Ansi378)
        ));

        // Sound format
        let sub = BiometricSubFormat::from_format_and_value(BiometricFormat::Sound, 0);
        assert!(matches!(
            sub,
            BiometricSubFormat::Sound(SoundSubFormat::Wav)
        ));

        let sub = BiometricSubFormat::from_format_and_value(BiometricFormat::Sound, 1);
        assert!(matches!(
            sub,
            BiometricSubFormat::Sound(SoundSubFormat::Mp3)
        ));

        // BioHash format - always returns Raw
        let sub = BiometricSubFormat::from_format_and_value(BiometricFormat::BioHash, 0);
        assert!(matches!(sub, BiometricSubFormat::Raw(0)));

        let sub = BiometricSubFormat::from_format_and_value(BiometricFormat::BioHash, 42);
        assert!(matches!(sub, BiometricSubFormat::Raw(42)));
    }

    #[test]
    fn test_biometric_subformat_invalid_returns_raw() {
        // Invalid image sub-format returns Raw
        let sub = BiometricSubFormat::from_format_and_value(BiometricFormat::Image, 50);
        assert!(matches!(sub, BiometricSubFormat::Raw(50)));

        // Invalid template sub-format returns Raw
        let sub = BiometricSubFormat::from_format_and_value(BiometricFormat::Template, 50);
        assert!(matches!(sub, BiometricSubFormat::Raw(50)));

        // Invalid sound sub-format returns Raw
        let sub = BiometricSubFormat::from_format_and_value(BiometricFormat::Sound, 99);
        assert!(matches!(sub, BiometricSubFormat::Raw(99)));
    }

    #[test]
    fn test_biometric_subformat_to_value() {
        // Image
        let sub = BiometricSubFormat::Image(ImageSubFormat::Jpeg);
        assert_eq!(sub.to_value(), 1);

        let sub = BiometricSubFormat::Image(ImageSubFormat::VendorSpecific(150));
        assert_eq!(sub.to_value(), 150);

        // Template
        let sub = BiometricSubFormat::Template(TemplateSubFormat::Iso19794_2);
        assert_eq!(sub.to_value(), 1);

        let sub = BiometricSubFormat::Template(TemplateSubFormat::VendorSpecific(175));
        assert_eq!(sub.to_value(), 175);

        // Sound
        let sub = BiometricSubFormat::Sound(SoundSubFormat::Mp3);
        assert_eq!(sub.to_value(), 1);

        // Raw
        let sub = BiometricSubFormat::Raw(999);
        assert_eq!(sub.to_value(), 999);
    }

    // ========== VerificationStatus Tests ==========
    #[test]
    fn test_verification_status_display() {
        assert_eq!(format!("{}", VerificationStatus::Verified), "verified");
        assert_eq!(format!("{}", VerificationStatus::Failed), "failed");
        assert_eq!(format!("{}", VerificationStatus::Skipped), "skipped");
    }

    #[test]
    fn test_verification_status_json_serialization() {
        let verified = VerificationStatus::Verified;
        let json = serde_json::to_string(&verified).unwrap();
        assert_eq!(json, "\"verified\"");

        let failed = VerificationStatus::Failed;
        let json = serde_json::to_string(&failed).unwrap();
        assert_eq!(json, "\"failed\"");

        let skipped = VerificationStatus::Skipped;
        let json = serde_json::to_string(&skipped).unwrap();
        assert_eq!(json, "\"skipped\"");

        // Deserialization
        let parsed: VerificationStatus = serde_json::from_str("\"verified\"").unwrap();
        assert_eq!(parsed, VerificationStatus::Verified);
    }

    // ========== Enum Copy/Debug/Eq Tests ==========
    #[test]
    fn test_gender_traits() {
        let g1 = Gender::Male;
        let g2 = g1; // Copy
        assert_eq!(g1, g2);
        assert_ne!(g1, Gender::Female);
        // Debug
        assert!(format!("{:?}", g1).contains("Male"));
    }

    #[test]
    fn test_verification_status_traits() {
        let v1 = VerificationStatus::Verified;
        let v2 = v1; // Copy
        assert_eq!(v1, v2);
        assert_ne!(v1, VerificationStatus::Failed);
        // Debug
        assert!(format!("{:?}", v1).contains("Verified"));
    }

    #[test]
    fn test_biometric_subformat_copy() {
        let sub1 = BiometricSubFormat::Image(ImageSubFormat::Jpeg);
        let sub2 = sub1; // Copy
        assert_eq!(sub1, sub2);
    }
}
