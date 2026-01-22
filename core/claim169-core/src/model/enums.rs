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

    #[test]
    fn test_gender_conversion() {
        assert_eq!(i64::from(Gender::Male), 1);
        assert_eq!(Gender::try_from(2).unwrap(), Gender::Female);
        assert!(Gender::try_from(99).is_err());
    }

    #[test]
    fn test_marital_status_conversion() {
        assert_eq!(i64::from(MaritalStatus::Married), 2);
        assert_eq!(MaritalStatus::try_from(3).unwrap(), MaritalStatus::Divorced);
    }

    #[test]
    fn test_image_subformat_vendor_specific() {
        let vendor = ImageSubFormat::try_from(150).unwrap();
        assert!(matches!(vendor, ImageSubFormat::VendorSpecific(150)));
        assert_eq!(i64::from(vendor), 150);
    }

    #[test]
    fn test_biometric_subformat_from_format() {
        let sub = BiometricSubFormat::from_format_and_value(BiometricFormat::Image, 6);
        assert!(matches!(
            sub,
            BiometricSubFormat::Image(ImageSubFormat::Wsq)
        ));

        let sub = BiometricSubFormat::from_format_and_value(BiometricFormat::Template, 1);
        assert!(matches!(
            sub,
            BiometricSubFormat::Template(TemplateSubFormat::Iso19794_2)
        ));
    }
}
