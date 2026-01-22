#![no_main]

use libfuzzer_sys::fuzz_target;
use claim169_core::pipeline::{compress, decompress};

fuzz_target!(|data: &[u8]| {
    // Test decompression with various limits
    // Small limit - tests the limit enforcement
    let _ = decompress(data, 1024);

    // Medium limit
    let _ = decompress(data, 65536);

    // Test compression/decompression round-trip
    let compressed = compress(data);
    if let Ok(decompressed) = decompress(&compressed, data.len() + 1024) {
        assert_eq!(decompressed, data, "Round-trip failed");
    }
});
