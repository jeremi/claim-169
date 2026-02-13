use std::io::Read;

use flate2::read::ZlibDecoder;

use crate::error::{Claim169Error, Result};

/// Compression mode for encoding.
///
/// Controls which compression algorithm is used when creating QR codes.
/// The default is `Zlib`, which is spec-compliant. Other modes require
/// explicit opt-in and produce non-standard output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Compression {
    /// zlib compression (spec-compliant, default)
    #[default]
    Zlib,
    /// Brotli compression at the given quality level (0-11, non-standard)
    #[cfg(feature = "compression-brotli")]
    Brotli(u32),
    /// No compression (non-standard)
    None,
    /// Use zlib if it reduces size, otherwise store raw (non-standard)
    Adaptive,
    /// Use brotli if it reduces size, otherwise store raw (non-standard)
    #[cfg(feature = "compression-brotli")]
    AdaptiveBrotli(u32),
}

/// The compression format detected during decoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectedCompression {
    /// zlib (identified by `0x78` prefix)
    Zlib,
    /// Brotli (identified by successful brotli decompression producing valid COSE)
    #[cfg(feature = "compression-brotli")]
    Brotli,
    /// No compression (raw COSE bytes)
    None,
}

impl std::fmt::Display for DetectedCompression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DetectedCompression::Zlib => write!(f, "zlib"),
            #[cfg(feature = "compression-brotli")]
            DetectedCompression::Brotli => write!(f, "brotli"),
            DetectedCompression::None => write!(f, "none"),
        }
    }
}

/// Decompress data with auto-detection and a size limit.
///
/// Detection logic:
/// 1. If the first byte is `0x78` (zlib magic), attempt zlib decompression.
/// 2. If brotli feature is enabled, attempt brotli decompression and validate
///    the result starts with a COSE tag (`0xD2` for COSE_Sign1 or `0xD0` for COSE_Sign).
/// 3. Otherwise, treat as raw (uncompressed) COSE bytes.
///
/// This function protects against decompression bomb attacks by limiting
/// the maximum decompressed size.
pub fn decompress(input: &[u8], max_bytes: usize) -> Result<(Vec<u8>, DetectedCompression)> {
    if input.is_empty() {
        return Ok((Vec::new(), DetectedCompression::None));
    }

    // Detect zlib by magic byte
    if input[0] == 0x78 {
        let decompressed = decompress_zlib(input, max_bytes)?;
        return Ok((decompressed, DetectedCompression::Zlib));
    }

    // Try brotli if the feature is enabled
    #[cfg(feature = "compression-brotli")]
    {
        match decompress_brotli(input, max_bytes) {
            Ok(decompressed) => {
                // Validate that the result looks like valid COSE (tagged CBOR)
                if is_valid_cose_prefix(&decompressed) {
                    return Ok((decompressed, DetectedCompression::Brotli));
                }
                // Decompressed successfully but doesn't look like COSE — not brotli data
            }
            Err(Claim169Error::DecompressLimitExceeded { max_bytes }) => {
                // Brotli decompression was producing output but hit the limit — propagate
                return Err(Claim169Error::DecompressLimitExceeded { max_bytes });
            }
            Err(_) => {
                // Brotli decompression failed entirely — not brotli data, try raw
            }
        }
    }

    // Treat as raw (uncompressed) COSE bytes
    if input.len() > max_bytes {
        return Err(Claim169Error::DecompressLimitExceeded { max_bytes });
    }
    Ok((input.to_vec(), DetectedCompression::None))
}

/// Check if bytes start with a valid COSE tag.
///
/// CBOR tag 18 (COSE_Sign1) encodes as `0xD2` (1-byte tag for value 18).
/// CBOR tag 16 (COSE_Sign) encodes as `0xD0` (1-byte tag for value 16).
/// CBOR tag 96 (COSE_Encrypt0) encodes as `0xD8 0x60`.
/// CBOR tag 97 (COSE_Encrypt) encodes as `0xD8 0x61`.
#[cfg(feature = "compression-brotli")]
fn is_valid_cose_prefix(data: &[u8]) -> bool {
    if data.is_empty() {
        return false;
    }
    match data[0] {
        0xD2 | 0xD0 => true,
        0xD8 if data.len() >= 2 => matches!(data[1], 0x60 | 0x61),
        _ => false,
    }
}

/// Decompress zlib-compressed data with a size limit.
fn decompress_zlib(input: &[u8], max_bytes: usize) -> Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(input);
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

/// Decompress brotli-compressed data with a size limit.
#[cfg(feature = "compression-brotli")]
fn decompress_brotli(input: &[u8], max_bytes: usize) -> Result<Vec<u8>> {
    let mut decoder = brotli::Decompressor::new(input, 8192);
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

/// Compress data using the specified compression mode.
///
/// Returns the compressed bytes and which compression was actually applied
/// (relevant for adaptive modes that may choose not to compress).
pub fn compress(input: &[u8], mode: Compression) -> (Vec<u8>, DetectedCompression) {
    match mode {
        Compression::Zlib => {
            let compressed = compress_zlib(input);
            (compressed, DetectedCompression::Zlib)
        }
        #[cfg(feature = "compression-brotli")]
        Compression::Brotli(quality) => {
            let compressed = compress_brotli(input, quality);
            (compressed, DetectedCompression::Brotli)
        }
        Compression::None => (input.to_vec(), DetectedCompression::None),
        Compression::Adaptive => {
            let compressed = compress_zlib(input);
            if compressed.len() < input.len() {
                (compressed, DetectedCompression::Zlib)
            } else {
                (input.to_vec(), DetectedCompression::None)
            }
        }
        #[cfg(feature = "compression-brotli")]
        Compression::AdaptiveBrotli(quality) => {
            let compressed = compress_brotli(input, quality);
            if compressed.len() < input.len() {
                (compressed, DetectedCompression::Brotli)
            } else {
                (input.to_vec(), DetectedCompression::None)
            }
        }
    }
}

/// Compress data using zlib.
pub fn compress_zlib(input: &[u8]) -> Vec<u8> {
    use flate2::write::ZlibEncoder;
    use flate2::Compression;
    use std::io::Write;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(input)
        .expect("compression should not fail");
    encoder.finish().expect("compression should not fail")
}

/// Compress data using brotli.
#[cfg(feature = "compression-brotli")]
pub fn compress_brotli(input: &[u8], quality: u32) -> Vec<u8> {
    let mut output = Vec::new();
    let params = brotli::enc::BrotliEncoderParams {
        quality: quality as i32,
        ..Default::default()
    };
    brotli::BrotliCompress(&mut &input[..], &mut output, &params)
        .expect("brotli compression should not fail");
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Zlib roundtrip tests (regression) ==========

    #[test]
    fn test_zlib_roundtrip() {
        let original = b"Hello, this is test data for compression!";
        let (compressed, detected) = compress(original, Compression::Zlib);
        assert_eq!(detected, DetectedCompression::Zlib);

        let (decompressed, detected) = decompress(&compressed, 1000).unwrap();
        assert_eq!(detected, DetectedCompression::Zlib);
        assert_eq!(decompressed, original);
    }

    #[test]
    fn test_zlib_roundtrip_binary() {
        let original: Vec<u8> = (0..=255).collect();
        let (compressed, _) = compress(&original, Compression::Zlib);
        let (decompressed, detected) = decompress(&compressed, 1000).unwrap();
        assert_eq!(detected, DetectedCompression::Zlib);
        assert_eq!(decompressed, original);
    }

    #[test]
    fn test_zlib_compression_reduces_size() {
        let original = vec![b'A'; 10000];
        let (compressed, _) = compress(&original, Compression::Zlib);
        assert!(compressed.len() < original.len() / 10);
    }

    #[test]
    fn test_decompress_limit_exceeded_zlib() {
        let original = vec![0u8; 1000];
        let (compressed, _) = compress(&original, Compression::Zlib);
        let result = decompress(&compressed, 500);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Claim169Error::DecompressLimitExceeded { max_bytes: 500 }
        ));
    }

    #[test]
    fn test_decompress_invalid_zlib() {
        // Starts with 0x78 (zlib magic) but is not valid zlib
        let invalid = [0x78, 0x9C, 0xFF, 0xFF];
        let result = decompress(&invalid, 1000);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Claim169Error::Decompress(_)));
    }

    // ========== Compression::None tests ==========

    #[test]
    fn test_compression_none_returns_input_unchanged() {
        let input = vec![0xD2, 0x84, 0x40, 0xA0]; // Fake COSE_Sign1 prefix
        let (compressed, detected) = compress(&input, Compression::None);
        assert_eq!(detected, DetectedCompression::None);
        assert_eq!(compressed, input);
    }

    #[test]
    fn test_no_compression_roundtrip() {
        // Input that starts with COSE tag (0xD2 = CBOR tag 18 = COSE_Sign1)
        let input = vec![0xD2, 0x84, 0x40, 0xA0, 0x41, 0x00, 0x40];
        let (compressed, enc_detected) = compress(&input, Compression::None);
        assert_eq!(enc_detected, DetectedCompression::None);
        assert_eq!(compressed, input);

        let (decompressed, dec_detected) = decompress(&compressed, 1000).unwrap();
        assert_eq!(dec_detected, DetectedCompression::None);
        assert_eq!(decompressed, input);
    }

    // ========== Adaptive compression tests ==========

    #[test]
    fn test_adaptive_picks_zlib_when_smaller() {
        // Highly compressible data
        let input = vec![b'A'; 10000];
        let (compressed, detected) = compress(&input, Compression::Adaptive);
        assert_eq!(detected, DetectedCompression::Zlib);
        assert!(compressed.len() < input.len());
    }

    #[test]
    fn test_adaptive_picks_raw_when_zlib_increases_size() {
        // Very small, incompressible data (zlib adds header overhead)
        let input = vec![0xD2, 0x01]; // 2 bytes - zlib will make it bigger
        let (compressed, detected) = compress(&input, Compression::Adaptive);
        assert_eq!(detected, DetectedCompression::None);
        assert_eq!(compressed, input);
    }

    // ========== Auto-detection tests ==========

    #[test]
    fn test_auto_detect_zlib_by_magic_byte() {
        let original = b"Test data for zlib detection";
        let (compressed, _) = compress(original, Compression::Zlib);
        assert_eq!(compressed[0], 0x78); // zlib magic byte
        let (decompressed, detected) = decompress(&compressed, 1000).unwrap();
        assert_eq!(detected, DetectedCompression::Zlib);
        assert_eq!(decompressed, original);
    }

    #[test]
    fn test_auto_detect_raw_for_unknown_bytes() {
        // Starts with COSE_Sign1 tag (0xD2), not 0x78
        let raw_cose = vec![0xD2, 0x84, 0x40, 0xA0, 0x41, 0x00, 0x40];
        let (decompressed, detected) = decompress(&raw_cose, 1000).unwrap();
        assert_eq!(detected, DetectedCompression::None);
        assert_eq!(decompressed, raw_cose);
    }

    // ========== Empty input tests ==========

    #[test]
    fn test_decompress_empty_input() {
        let (decompressed, detected) = decompress(&[], 1000).unwrap();
        assert!(decompressed.is_empty());
        assert_eq!(detected, DetectedCompression::None);
    }

    #[test]
    fn test_compress_empty_zlib() {
        let (compressed, detected) = compress(b"", Compression::Zlib);
        assert_eq!(detected, DetectedCompression::Zlib);
        let (decompressed, _) = decompress(&compressed, 1000).unwrap();
        assert!(decompressed.is_empty());
    }

    #[test]
    fn test_compress_empty_none() {
        let (compressed, detected) = compress(b"", Compression::None);
        assert_eq!(detected, DetectedCompression::None);
        assert!(compressed.is_empty());
    }

    // ========== Raw input size limit test ==========

    #[test]
    fn test_raw_input_exceeding_max_bytes() {
        // Raw (non-zlib) data that exceeds the max size limit
        let large_raw = vec![0xD2; 2000]; // Starts with COSE tag
        let result = decompress(&large_raw, 500);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Claim169Error::DecompressLimitExceeded { max_bytes: 500 }
        ));
    }

    // ========== Brotli tests (feature-gated) ==========

    #[cfg(feature = "compression-brotli")]
    mod brotli_tests {
        use super::*;

        #[test]
        fn test_brotli_roundtrip() {
            // Input must look like valid COSE when decompressed for auto-detect
            let original = {
                let mut v = vec![0xD2, 0x84]; // COSE_Sign1 tag + array start
                v.extend_from_slice(&vec![b'A'; 500]); // padding
                v
            };
            let (compressed, enc_detected) = compress(&original, Compression::Brotli(9));
            assert_eq!(enc_detected, DetectedCompression::Brotli);
            assert!(compressed.len() < original.len());

            let (decompressed, dec_detected) = decompress(&compressed, 10000).unwrap();
            assert_eq!(dec_detected, DetectedCompression::Brotli);
            assert_eq!(decompressed, original);
        }

        #[test]
        fn test_brotli_bomb_protection() {
            let original = {
                let mut v = vec![0xD2, 0x84];
                v.extend_from_slice(&vec![0u8; 10000]);
                v
            };
            let (compressed, _) = compress(&original, Compression::Brotli(9));

            let result = decompress(&compressed, 500);
            assert!(result.is_err());
            assert!(matches!(
                result.unwrap_err(),
                Claim169Error::DecompressLimitExceeded { max_bytes: 500 }
            ));
        }

        #[test]
        fn test_adaptive_brotli_picks_brotli_when_smaller() {
            let original = {
                let mut v = vec![0xD2, 0x84];
                v.extend_from_slice(&vec![b'A'; 5000]);
                v
            };
            let (compressed, detected) = compress(&original, Compression::AdaptiveBrotli(9));
            assert_eq!(detected, DetectedCompression::Brotli);
            assert!(compressed.len() < original.len());
        }

        #[test]
        fn test_adaptive_brotli_picks_raw_when_bigger() {
            let input = vec![0xD2, 0x01]; // Very small
            let (compressed, detected) = compress(&input, Compression::AdaptiveBrotli(9));
            assert_eq!(detected, DetectedCompression::None);
            assert_eq!(compressed, input);
        }

        #[test]
        fn test_brotli_compress_empty() {
            let (compressed, detected) = compress(b"", Compression::Brotli(9));
            assert_eq!(detected, DetectedCompression::Brotli);
            // Brotli can still produce output for empty input
            let decompressed = decompress_brotli(&compressed, 1000).unwrap();
            assert!(decompressed.is_empty());
        }
    }

    // ========== DetectedCompression Display ==========

    #[test]
    fn test_detected_compression_display() {
        assert_eq!(DetectedCompression::Zlib.to_string(), "zlib");
        assert_eq!(DetectedCompression::None.to_string(), "none");
    }

    #[cfg(feature = "compression-brotli")]
    #[test]
    fn test_detected_compression_display_brotli() {
        assert_eq!(DetectedCompression::Brotli.to_string(), "brotli");
    }

    // ========== Default compression ==========

    #[test]
    fn test_compression_default_is_zlib() {
        assert_eq!(Compression::default(), Compression::Zlib);
    }
}
