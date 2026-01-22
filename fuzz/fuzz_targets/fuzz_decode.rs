#![no_main]

use libfuzzer_sys::fuzz_target;
use claim169_core::{decode, DecodeOptions};

fuzz_target!(|data: &[u8]| {
    // Try to decode arbitrary bytes as a QR code string
    if let Ok(s) = std::str::from_utf8(data) {
        // Use permissive options to maximize code path coverage
        let opts = DecodeOptions::permissive();

        // We don't care about the result - we're looking for panics/crashes
        let _ = decode(s, opts);
    }
});
