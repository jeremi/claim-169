#![no_main]

use libfuzzer_sys::fuzz_target;
use claim169_core::pipeline::{base45_decode, base45_encode};

fuzz_target!(|data: &[u8]| {
    // Test decoding arbitrary bytes as base45
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = base45_decode(s);
    }

    // Test encoding and round-trip
    let encoded = base45_encode(data);
    if let Ok(decoded) = base45_decode(&encoded) {
        assert_eq!(decoded, data, "Round-trip failed");
    }
});
