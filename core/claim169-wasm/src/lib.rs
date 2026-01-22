//! WebAssembly bindings for MOSIP Claim 169 QR decoder
//!
//! This module provides WASM bindings using wasm-bindgen for the claim169-core library.
//! It supports custom crypto hooks for integration with WebCrypto or external systems.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// Install panic hook on module load
#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

use claim169_core::model::{
    Biometric as CoreBiometric, Claim169 as CoreClaim169, CwtMeta as CoreCwtMeta,
};
use claim169_core::{decode as core_decode, DecodeOptions};

// ============================================================================
// JavaScript Error Types
// ============================================================================

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

// ============================================================================
// Data Structures
// ============================================================================

/// Biometric data from Claim 169
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsBiometric {
    pub data: Vec<u8>,
    pub format: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_format: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
}

impl From<&CoreBiometric> for JsBiometric {
    fn from(b: &CoreBiometric) -> Self {
        JsBiometric {
            data: b.data.clone(),
            format: b.format.map(|f| f as i64).unwrap_or(0),
            sub_format: b.sub_format.as_ref().map(|s| s.to_value()),
            issuer: b.issuer.clone(),
        }
    }
}

/// CWT metadata
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsCwtMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<i64>,
}

impl From<&CoreCwtMeta> for JsCwtMeta {
    fn from(m: &CoreCwtMeta) -> Self {
        JsCwtMeta {
            issuer: m.issuer.clone(),
            subject: m.subject.clone(),
            expires_at: m.expires_at,
            not_before: m.not_before,
            issued_at: m.issued_at,
        }
    }
}

/// Claim 169 identity data
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsClaim169 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardian: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_format: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_quality_fingers: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_of_issuance: Option<String>,

    // Biometrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_thumb: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_pointer_finger: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_middle_finger: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_ring_finger: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_little_finger: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_thumb: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_pointer_finger: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_middle_finger: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_ring_finger: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_little_finger: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_iris: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_iris: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_palm: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_palm: Option<Vec<JsBiometric>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Vec<JsBiometric>>,
}

fn convert_biometrics(biometrics: &Option<Vec<CoreBiometric>>) -> Option<Vec<JsBiometric>> {
    biometrics
        .as_ref()
        .map(|v| v.iter().map(JsBiometric::from).collect())
}

impl From<&CoreClaim169> for JsClaim169 {
    fn from(c: &CoreClaim169) -> Self {
        JsClaim169 {
            id: c.id.clone(),
            version: c.version.clone(),
            language: c.language.clone(),
            full_name: c.full_name.clone(),
            first_name: c.first_name.clone(),
            middle_name: c.middle_name.clone(),
            last_name: c.last_name.clone(),
            date_of_birth: c.date_of_birth.clone(),
            gender: c.gender.map(|g| g as i64),
            address: c.address.clone(),
            email: c.email.clone(),
            phone: c.phone.clone(),
            nationality: c.nationality.clone(),
            marital_status: c.marital_status.map(|m| m as i64),
            guardian: c.guardian.clone(),
            photo: c.photo.clone(),
            photo_format: c.photo_format.map(|f| f as i64),
            best_quality_fingers: c.best_quality_fingers.clone(),
            secondary_full_name: c.secondary_full_name.clone(),
            secondary_language: c.secondary_language.clone(),
            location_code: c.location_code.clone(),
            legal_status: c.legal_status.clone(),
            country_of_issuance: c.country_of_issuance.clone(),
            right_thumb: convert_biometrics(&c.right_thumb),
            right_pointer_finger: convert_biometrics(&c.right_pointer_finger),
            right_middle_finger: convert_biometrics(&c.right_middle_finger),
            right_ring_finger: convert_biometrics(&c.right_ring_finger),
            right_little_finger: convert_biometrics(&c.right_little_finger),
            left_thumb: convert_biometrics(&c.left_thumb),
            left_pointer_finger: convert_biometrics(&c.left_pointer_finger),
            left_middle_finger: convert_biometrics(&c.left_middle_finger),
            left_ring_finger: convert_biometrics(&c.left_ring_finger),
            left_little_finger: convert_biometrics(&c.left_little_finger),
            right_iris: convert_biometrics(&c.right_iris),
            left_iris: convert_biometrics(&c.left_iris),
            face: convert_biometrics(&c.face),
            right_palm: convert_biometrics(&c.right_palm),
            left_palm: convert_biometrics(&c.left_palm),
            voice: convert_biometrics(&c.voice),
        }
    }
}

/// Decode result
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsDecodeResult {
    pub claim169: JsClaim169,
    pub cwt_meta: JsCwtMeta,
    pub verification_status: String,
}

// ============================================================================
// Decode Options
// ============================================================================

/// Options for decoding
#[wasm_bindgen]
pub struct WasmDecodeOptions {
    max_decompressed_bytes: usize,
    skip_biometrics: bool,
    validate_timestamps: bool,
    clock_skew_tolerance_seconds: i64,
}

#[wasm_bindgen]
impl WasmDecodeOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmDecodeOptions {
        WasmDecodeOptions {
            max_decompressed_bytes: 65536,
            skip_biometrics: false,
            // WASM doesn't have access to system time, so disable timestamp validation by default
            validate_timestamps: false,
            clock_skew_tolerance_seconds: 0,
        }
    }

    /// Set maximum decompressed size in bytes
    #[wasm_bindgen(js_name = "setMaxDecompressedBytes")]
    pub fn set_max_decompressed_bytes(mut self, bytes: usize) -> WasmDecodeOptions {
        self.max_decompressed_bytes = bytes;
        self
    }

    /// Skip biometric data parsing
    #[wasm_bindgen(js_name = "setSkipBiometrics")]
    pub fn set_skip_biometrics(mut self, skip: bool) -> WasmDecodeOptions {
        self.skip_biometrics = skip;
        self
    }

    /// Set timestamp validation
    #[wasm_bindgen(js_name = "setValidateTimestamps")]
    pub fn set_validate_timestamps(mut self, validate: bool) -> WasmDecodeOptions {
        self.validate_timestamps = validate;
        self
    }

    /// Set clock skew tolerance in seconds for timestamp validation
    #[wasm_bindgen(js_name = "setClockSkewToleranceSeconds")]
    pub fn set_clock_skew_tolerance_seconds(mut self, seconds: i64) -> WasmDecodeOptions {
        self.clock_skew_tolerance_seconds = seconds.max(0);
        self
    }
}

// ============================================================================
// Public API
// ============================================================================

/// Decode a Claim 169 QR code without signature verification.
///
/// @param qrText - The QR code text content (Base45 encoded)
/// @returns The decoded result as a JavaScript object
/// @throws Error if decoding fails
#[wasm_bindgen(js_name = "decode")]
pub fn decode(qr_text: &str) -> Result<JsValue, JsError> {
    decode_with_options(qr_text, None)
}

/// Decode a Claim 169 QR code with options.
///
/// @param qrText - The QR code text content (Base45 encoded)
/// @param options - Optional decode options
/// @returns The decoded result as a JavaScript object
/// @throws Error if decoding fails
#[wasm_bindgen(js_name = "decodeWithOptions")]
pub fn decode_with_options(
    qr_text: &str,
    options: Option<WasmDecodeOptions>,
) -> Result<JsValue, JsError> {
    let opts = options.unwrap_or_else(WasmDecodeOptions::new);
    let decode_options = DecodeOptions {
        max_decompressed_bytes: opts.max_decompressed_bytes,
        skip_biometrics: opts.skip_biometrics,
        validate_timestamps: opts.validate_timestamps,
        allow_unverified: true,
        clock_skew_tolerance_seconds: opts.clock_skew_tolerance_seconds,
    };

    let result = core_decode(qr_text, decode_options).map_err(|e| JsError::new(&e.to_string()))?;

    let js_result = JsDecodeResult {
        claim169: JsClaim169::from(&result.claim169),
        cwt_meta: JsCwtMeta::from(&result.cwt_meta),
        verification_status: format!("{}", result.verification_status),
    };

    serde_wasm_bindgen::to_value(&js_result).map_err(|e| JsError::new(&e.to_string()))
}

/// Get the library version
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Check if the WASM module is loaded correctly
#[wasm_bindgen(js_name = "isLoaded")]
pub fn is_loaded() -> bool {
    true
}
