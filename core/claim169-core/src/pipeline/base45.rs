use crate::error::{Claim169Error, Result};

/// Decode a Base45-encoded string to bytes
///
/// Base45 is used in MOSIP Claim 169 QR codes because it's optimized
/// for QR code alphanumeric mode, which has higher data density than
/// binary mode for the Base45 character set.
pub fn decode(input: &str) -> Result<Vec<u8>> {
    base45::decode(input).map_err(|e| Claim169Error::Base45Decode(e.to_string()))
}

/// Encode bytes to Base45 string
///
/// Used for generating test vectors and encoding payloads.
pub fn encode(input: &[u8]) -> String {
    base45::encode(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_valid() {
        // "Hello!!" in Base45 is "%69 VD92EX0"
        let decoded = decode("%69 VD92EX0").unwrap();
        assert_eq!(decoded, b"Hello!!");
    }

    #[test]
    fn test_decode_empty() {
        let decoded = decode("").unwrap();
        assert!(decoded.is_empty());
    }

    #[test]
    fn test_decode_invalid() {
        // lowercase letters are invalid in Base45
        let result = decode("abc");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Claim169Error::Base45Decode(_)
        ));
    }

    #[test]
    fn test_roundtrip() {
        let original = b"Test data for roundtrip encoding";
        let encoded = encode(original);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_encode_binary() {
        let binary = vec![0x00, 0x01, 0x02, 0xFF, 0xFE];
        let encoded = encode(&binary);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, binary);
    }
}
