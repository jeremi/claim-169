use std::io::Read;

use flate2::read::ZlibDecoder;

use crate::error::{Claim169Error, Result};

/// Decompress zlib-compressed data with a size limit
///
/// This function protects against "zip bomb" attacks by limiting
/// the maximum decompressed size.
pub fn decompress(input: &[u8], max_bytes: usize) -> Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(input);

    // Read with size limit
    let mut output = Vec::new();
    let mut buffer = [0u8; 8192];
    let mut total_read = 0usize;

    loop {
        let bytes_read = decoder
            .read(&mut buffer)
            .map_err(|e| Claim169Error::Decompress(e.to_string()))?;

        if bytes_read == 0 {
            break;
        }

        total_read += bytes_read;
        if total_read > max_bytes {
            return Err(Claim169Error::DecompressLimitExceeded { max_bytes });
        }

        output.extend_from_slice(&buffer[..bytes_read]);
    }

    Ok(output)
}

/// Compress data using zlib
///
/// Used for generating test vectors.
pub fn compress(input: &[u8]) -> Vec<u8> {
    use flate2::write::ZlibEncoder;
    use flate2::Compression;
    use std::io::Write;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(input)
        .expect("compression should not fail");
    encoder.finish().expect("compression should not fail")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress_valid() {
        let original = b"Hello, this is test data for compression!";
        let compressed = compress(original);
        let decompressed = decompress(&compressed, 1000).unwrap();
        assert_eq!(decompressed, original);
    }

    #[test]
    fn test_decompress_empty() {
        let compressed = compress(b"");
        let decompressed = decompress(&compressed, 1000).unwrap();
        assert!(decompressed.is_empty());
    }

    #[test]
    fn test_decompress_limit_exceeded() {
        let original = vec![0u8; 1000];
        let compressed = compress(&original);

        // Set limit lower than actual data
        let result = decompress(&compressed, 500);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Claim169Error::DecompressLimitExceeded { max_bytes: 500 }
        ));
    }

    #[test]
    fn test_decompress_invalid_data() {
        let invalid = b"not valid zlib data";
        let result = decompress(invalid, 1000);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Claim169Error::Decompress(_)));
    }

    #[test]
    fn test_roundtrip_binary() {
        let original: Vec<u8> = (0..=255).collect();
        let compressed = compress(&original);
        let decompressed = decompress(&compressed, 1000).unwrap();
        assert_eq!(decompressed, original);
    }

    #[test]
    fn test_compression_reduces_size() {
        // Highly compressible data
        let original = vec![b'A'; 10000];
        let compressed = compress(&original);

        // zlib should significantly reduce size for repetitive data
        assert!(compressed.len() < original.len() / 10);
    }
}
