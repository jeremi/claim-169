//! Compression impact analysis for MOSIP Claim 169 QR codes.
//!
//! This tool analyzes whether zlib compression helps or hurts QR code data size
//! across various realistic credential scenarios. The encoding pipeline is:
//!
//! ```text
//! Claim169 → CBOR → CWT → COSE_Sign1 → [zlib] → Base45 → QR Code
//! ```
//!
//! We compare sizes with and without the zlib step to determine when compression
//! is beneficial.

use std::io::Write as IoWrite;
use std::path::Path;

use ciborium::Value;
use claim169_core::crypto::software::Ed25519Signer;
use claim169_core::crypto::traits::Signer;
use claim169_core::model::CwtMeta;
use claim169_core::pipeline::{base45_encode, cwt::encode as cwt_encode};
use coset::{iana, CoseSign1Builder, HeaderBuilder, TaggedCborSerializable};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use rand::RngCore;

fn main() {
    println!("=============================================================================");
    println!("  Claim 169 QR Code — Compression Impact Analysis");
    println!("=============================================================================");
    println!();
    println!("Pipeline: Claim169 → CBOR → CWT → COSE_Sign1 → [zlib] → Base45 → QR");
    println!();
    println!("All scenarios are Ed25519-signed (production-realistic).");
    println!();

    let scenarios = build_scenarios();

    // ── Per-scenario detailed report ──────────────────────────────────────
    println!("─────────────────────────────────────────────────────────────────────────────────");
    println!(
        "{:<46} {:>6} {:>6} {:>6} {:>8} {:>6} {:>9}",
        "Scenario", "COSE", "zlib", "B45+z", "B45 raw", "Delta", "Verdict"
    );
    println!(
        "{:<46} {:>6} {:>6} {:>6} {:>8} {:>6} {:>9}",
        "", "bytes", "bytes", "chars", "chars", "chars", ""
    );
    println!("─────────────────────────────────────────────────────────────────────────────────");

    let mut results: Vec<ScenarioResult> = Vec::new();

    for scenario in &scenarios {
        let result = analyze_scenario(scenario);
        print_row(scenario, &result);
        results.push(result);
    }

    println!("─────────────────────────────────────────────────────────────────────────────────");
    println!();

    // ── Byte entropy analysis ─────────────────────────────────────────────
    print_entropy_analysis(&scenarios, &results);

    // ── Summary statistics ────────────────────────────────────────────────
    println!();
    print_summary(&scenarios, &results);

    // ── Compression level comparison ─────────────────────────────────────
    println!();
    print_compression_levels(&scenarios);

    // ── QR version impact ────────────────────────────────────────────────
    println!();
    print_qr_version_analysis(&scenarios, &results);

    // ── Conclusions ──────────────────────────────────────────────────────
    println!();
    print_conclusions(&scenarios, &results);
}

// ════════════════════════════════════════════════════════════════════════════
// Data types
// ════════════════════════════════════════════════════════════════════════════

#[allow(dead_code)]
struct Scenario {
    name: String,
    description: String,
    category: ScenarioCategory,
    cose_bytes: Vec<u8>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ScenarioCategory {
    TextOnly,
    WithBinary,
    Compressible,
    RealImage,
}

impl ScenarioCategory {
    fn label(self) -> &'static str {
        match self {
            Self::TextOnly => "text-only",
            Self::WithBinary => "random-binary",
            Self::Compressible => "compressible",
            Self::RealImage => "real-image",
        }
    }
}

struct ScenarioResult {
    cose_size: usize,
    compressed_size: usize,
    base45_compressed_len: usize,
    base45_raw_len: usize,
    compression_ratio: f64,
    delta_chars: i64,
    helps: bool,
    entropy: f64,
}

// ════════════════════════════════════════════════════════════════════════════
// Analysis
// ════════════════════════════════════════════════════════════════════════════

fn analyze_scenario(scenario: &Scenario) -> ScenarioResult {
    let cose_bytes = &scenario.cose_bytes;
    let cose_size = cose_bytes.len();

    // Path A: with zlib compression (current pipeline)
    let compressed = zlib_compress(cose_bytes, Compression::default());
    let compressed_size = compressed.len();
    let base45_compressed = base45_encode(&compressed);
    let base45_compressed_len = base45_compressed.len();

    // Path B: without zlib compression (direct Base45)
    let base45_raw = base45_encode(cose_bytes);
    let base45_raw_len = base45_raw.len();

    let compression_ratio = compressed_size as f64 / cose_size as f64;
    let delta_chars = base45_compressed_len as i64 - base45_raw_len as i64;
    let helps = base45_compressed_len < base45_raw_len;
    let entropy = shannon_entropy(cose_bytes);

    ScenarioResult {
        cose_size,
        compressed_size,
        base45_compressed_len,
        base45_raw_len,
        compression_ratio,
        delta_chars,
        helps,
        entropy,
    }
}

/// Calculate Shannon entropy of a byte sequence (0.0 = all same, 8.0 = perfectly random).
fn shannon_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mut counts = [0u64; 256];
    for &b in data {
        counts[b as usize] += 1;
    }
    let len = data.len() as f64;
    let mut entropy = 0.0f64;
    for &c in &counts {
        if c > 0 {
            let p = c as f64 / len;
            entropy -= p * p.log2();
        }
    }
    entropy
}

// ════════════════════════════════════════════════════════════════════════════
// Scenario builders
// ════════════════════════════════════════════════════════════════════════════

fn build_scenarios() -> Vec<Scenario> {
    let signer = Ed25519Signer::generate();

    // NOTE: ALL scenarios are Ed25519-signed to reflect production reality.
    // Categories reflect the data type, not the signing status.

    let mut scenarios = vec![
        // ═══ SECTION A: Text-only payloads (no binary data) ═══════════════
        build_scenario(
            "A1. Minimal (ID + name)",
            "just id and full_name",
            ScenarioCategory::TextOnly,
            &create_claim169_map(vec![
                (1, Value::Text("ID-12345-ABCDE".to_string())),
                (4, Value::Text("John Doe".to_string())),
            ]),
            &minimal_cwt(),
            &signer,
        ),
        build_scenario(
            "A2. Basic demographics (Latin)",
            "ID, name, DOB, gender, address",
            ScenarioCategory::TextOnly,
            &create_claim169_map(vec![
                (1, Value::Text("ID-67890-FGHIJ".to_string())),
                (4, Value::Text("Jane Marie Smith".to_string())),
                (5, Value::Text("Jane".to_string())),
                (7, Value::Text("Smith".to_string())),
                (8, Value::Text("19900515".to_string())),
                (9, Value::Integer(2.into())),
                (
                    10,
                    Value::Text("123 Main St\nNew York, NY 10001".to_string()),
                ),
            ]),
            &minimal_cwt(),
            &signer,
        ),
        build_scenario(
            "A3. Full demographics (all 23 keys)",
            "every demographic field populated",
            ScenarioCategory::TextOnly,
            &create_full_demographics_latin(),
            &full_cwt(),
            &signer,
        ),
        // ── Arabic (RTL, high-entropy UTF-8) ──────────────────────────────
        build_scenario(
            "A4. Arabic persona",
            "Arabic name + address + secondary",
            ScenarioCategory::TextOnly,
            &create_arabic_persona(),
            &full_cwt(),
            &signer,
        ),
        // ── Devanagari (Hindi) ────────────────────────────────────────────
        build_scenario(
            "A5. Hindi/Devanagari persona",
            "Hindi name + address",
            ScenarioCategory::TextOnly,
            &create_hindi_persona(),
            &full_cwt(),
            &signer,
        ),
        // ── Chinese (CJK, 3-byte UTF-8) ──────────────────────────────────
        build_scenario(
            "A6. Chinese persona",
            "Chinese name + address",
            ScenarioCategory::TextOnly,
            &create_chinese_persona(),
            &full_cwt(),
            &signer,
        ),
        // ── Cyrillic (Russian) ────────────────────────────────────────────
        build_scenario(
            "A7. Cyrillic/Russian persona",
            "Russian name + address",
            ScenarioCategory::TextOnly,
            &create_cyrillic_persona(),
            &full_cwt(),
            &signer,
        ),
        // ── Thai ──────────────────────────────────────────────────────────
        build_scenario(
            "A8. Thai persona",
            "Thai name + address",
            ScenarioCategory::TextOnly,
            &create_thai_persona(),
            &full_cwt(),
            &signer,
        ),
        // ── Bilingual (two scripts) ──────────────────────────────────────
        build_scenario(
            "A9. Bilingual Arabic+Latin",
            "Latin primary + Arabic secondary name",
            ScenarioCategory::TextOnly,
            &create_bilingual_arabic_persona(),
            &full_cwt(),
            &signer,
        ),
        build_scenario(
            "A10. Bilingual Hindi+Latin",
            "Latin primary + Hindi secondary name",
            ScenarioCategory::TextOnly,
            &create_bilingual_hindi_persona(),
            &full_cwt(),
            &signer,
        ),
        // ═══ SECTION B: Payloads with binary data (random = incompressible) ══
        build_scenario(
            "B1. Demo + random photo 200B",
            "small random photo (simulates WebP)",
            ScenarioCategory::WithBinary,
            &create_demographics_with_photo(200),
            &full_cwt(),
            &signer,
        ),
        build_scenario(
            "B2. Demo + random photo 400B",
            "medium random photo",
            ScenarioCategory::WithBinary,
            &create_demographics_with_photo(400),
            &full_cwt(),
            &signer,
        ),
        build_scenario(
            "B3. Demo + random photo 600B",
            "medium-large random photo",
            ScenarioCategory::WithBinary,
            &create_demographics_with_photo(600),
            &full_cwt(),
            &signer,
        ),
        build_scenario(
            "B4. Demo + random photo 800B",
            "large random photo (near QR limit)",
            ScenarioCategory::WithBinary,
            &create_demographics_with_photo(800),
            &full_cwt(),
            &signer,
        ),
        build_scenario(
            "B5. Demo + face biometric 500B",
            "random face biometric data",
            ScenarioCategory::WithBinary,
            &create_demographics_with_face_biometric(500),
            &full_cwt(),
            &signer,
        ),
        build_scenario(
            "B6. Demo + 2 fingerprint templates",
            "two 64B random fingerprint templates",
            ScenarioCategory::WithBinary,
            &create_demographics_with_fingerprints(2, 64),
            &full_cwt(),
            &signer,
        ),
        build_scenario(
            "B7. Demo + 4 fingerprint templates",
            "four 64B random fingerprint templates",
            ScenarioCategory::WithBinary,
            &create_demographics_with_fingerprints(4, 64),
            &full_cwt(),
            &signer,
        ),
        build_scenario(
            "B8. Full credential (demo+photo+face)",
            "all demographics + 400B photo + 300B face",
            ScenarioCategory::WithBinary,
            &create_full_credential_random(),
            &full_cwt(),
            &signer,
        ),
    ];

    // ═══ SECTION C: Real image data ═══════════════════════════════════════
    if let Some(webp_data) = load_real_image() {
        let webp_len = webp_data.len();

        // C1-C3: original WebP (with ICC profile)
        scenarios.push(build_scenario(
            &format!("C1. Real WebP photo ({}B)", webp_len),
            "actual sample_id_1.webp with ICC profile",
            ScenarioCategory::RealImage,
            &create_demographics_with_real_photo(&webp_data),
            &full_cwt(),
            &signer,
        ));

        scenarios.push(build_scenario(
            &format!("C2. Real WebP as face bio ({}B)", webp_len),
            "real WebP in biometric CBOR structure",
            ScenarioCategory::RealImage,
            &create_demographics_with_real_face_bio(&webp_data),
            &full_cwt(),
            &signer,
        ));

        scenarios.push(build_scenario(
            &format!("C3. Real full credential ({}B photo)", webp_len),
            "full demographics + real WebP photo",
            ScenarioCategory::RealImage,
            &create_full_credential_real(&webp_data),
            &full_cwt(),
            &signer,
        ));

        // C4-C6: stripped WebP (ICC profile removed, as playground does)
        let stripped = strip_webp_icc_profile(&webp_data);
        let stripped_len = stripped.len();
        let saved = webp_len - stripped_len;
        println!(
            "  [INFO] ICC strip: {}B → {}B (saved {}B, {:.0}% of original)",
            webp_len,
            stripped_len,
            saved,
            saved as f64 / webp_len as f64 * 100.0,
        );
        println!();

        scenarios.push(build_scenario(
            &format!("C4. Stripped WebP photo ({}B)", stripped_len),
            "ICC profile removed (playground pipeline)",
            ScenarioCategory::RealImage,
            &create_demographics_with_real_photo(&stripped),
            &full_cwt(),
            &signer,
        ));

        scenarios.push(build_scenario(
            &format!("C5. Stripped WebP as face bio ({}B)", stripped_len),
            "ICC-stripped WebP in biometric structure",
            ScenarioCategory::RealImage,
            &create_demographics_with_real_face_bio(&stripped),
            &full_cwt(),
            &signer,
        ));

        scenarios.push(build_scenario(
            &format!("C6. Stripped full credential ({}B photo)", stripped_len),
            "full demographics + ICC-stripped WebP",
            ScenarioCategory::RealImage,
            &create_full_credential_real(&stripped),
            &full_cwt(),
            &signer,
        ));
    } else {
        println!("  [NOTE] Real image not found, skipping section C scenarios.");
        println!("         Expected: playground/public/sample_id_pictures/sample_id_1.webp");
        println!();
    }

    // ═══ SECTION D: Compressible binary data (synthetic) ═══════════════
    scenarios.push(build_scenario(
        "D1. All-zeros photo 400B",
        "maximally compressible photo data",
        ScenarioCategory::Compressible,
        &create_demographics_with_uniform_photo(400, 0x00),
        &full_cwt(),
        &signer,
    ));
    scenarios.push(build_scenario(
        "D2. All-zeros photo 800B",
        "maximally compressible, larger",
        ScenarioCategory::Compressible,
        &create_demographics_with_uniform_photo(800, 0x00),
        &full_cwt(),
        &signer,
    ));
    scenarios.push(build_scenario(
        "D3. Patterned biometric 400B",
        "repeating 16-byte pattern (structured template)",
        ScenarioCategory::Compressible,
        &create_demographics_with_patterned_bio(400),
        &full_cwt(),
        &signer,
    ));

    // ═══ SECTION E: Edge cases ══════════════════════════════════════════
    scenarios.push(build_scenario(
        "E1. Absolute minimal (id=X)",
        "single 1-char field",
        ScenarioCategory::TextOnly,
        &create_claim169_map(vec![(1, Value::Text("X".to_string()))]),
        &CwtMeta::new().with_issuer("i"),
        &signer,
    ));
    scenarios.push(build_scenario(
        "E2. Long address (500 char Latin)",
        "test with very long text field",
        ScenarioCategory::TextOnly,
        &create_long_address_persona(),
        &full_cwt(),
        &signer,
    ));

    scenarios
}

fn build_scenario(
    name: &str,
    description: &str,
    category: ScenarioCategory,
    claim_169_cbor: &Value,
    cwt_meta: &CwtMeta,
    signer: &Ed25519Signer,
) -> Scenario {
    let cwt_bytes = cwt_encode(cwt_meta, claim_169_cbor);
    let cose_bytes = build_signed_cose(&cwt_bytes, signer);
    Scenario {
        name: name.to_string(),
        description: description.to_string(),
        category,
        cose_bytes,
    }
}

// ════════════════════════════════════════════════════════════════════════════
// CBOR data builders
// ════════════════════════════════════════════════════════════════════════════

fn create_claim169_map(fields: Vec<(i64, Value)>) -> Value {
    Value::Map(
        fields
            .into_iter()
            .map(|(k, v)| (Value::Integer(k.into()), v))
            .collect(),
    )
}

fn minimal_cwt() -> CwtMeta {
    CwtMeta::new()
        .with_issuer("https://mosip.example.org")
        .with_issued_at(1700000000)
        .with_expires_at(1800000000)
}

fn full_cwt() -> CwtMeta {
    CwtMeta::new()
        .with_issuer("https://mosip.example.org")
        .with_subject("ID-67890-FGHIJ")
        .with_issued_at(1700000000)
        .with_expires_at(1800000000)
        .with_not_before(1700000000)
}

// ── Latin script ──────────────────────────────────────────────────────

fn create_full_demographics_latin() -> Value {
    create_claim169_map(vec![
        (1, Value::Text("ID-67890-FGHIJ".to_string())),
        (2, Value::Text("1.0".to_string())),
        (3, Value::Text("eng".to_string())),
        (4, Value::Text("Jane Marie Smith".to_string())),
        (5, Value::Text("Jane".to_string())),
        (6, Value::Text("Marie".to_string())),
        (7, Value::Text("Smith".to_string())),
        (8, Value::Text("19900515".to_string())),
        (9, Value::Integer(2.into())),
        (
            10,
            Value::Text("123 Main St\nApt 4\nNew York, NY 10001".to_string()),
        ),
        (11, Value::Text("jane.smith@example.com".to_string())),
        (12, Value::Text("+1 555 123 4567".to_string())),
        (13, Value::Text("USA".to_string())),
        (14, Value::Integer(2.into())),
        (15, Value::Text("Guardian-ID-001".to_string())),
        (17, Value::Integer(4.into())),
        (
            18,
            Value::Array(vec![
                Value::Integer(1.into()),
                Value::Integer(2.into()),
                Value::Integer(6.into()),
            ]),
        ),
        (19, Value::Text("जेन मैरी स्मिथ".to_string())),
        (20, Value::Text("hin".to_string())),
        (21, Value::Text("US-NY-NYC".to_string())),
        (22, Value::Text("citizen".to_string())),
        (23, Value::Text("USA".to_string())),
    ])
}

// ── Arabic ────────────────────────────────────────────────────────────

fn create_arabic_persona() -> Value {
    create_claim169_map(vec![
        (1, Value::Text("SA-1234567890".to_string())),
        (3, Value::Text("ara".to_string())),
        (4, Value::Text("محمد عبدالله الرشيدي".to_string())),
        (5, Value::Text("محمد".to_string())),
        (6, Value::Text("عبدالله".to_string())),
        (7, Value::Text("الرشيدي".to_string())),
        (8, Value::Text("19850310".to_string())),
        (9, Value::Integer(1.into())),
        (
            10,
            Value::Text("شارع الملك فهد\nحي العليا\nالرياض 12211".to_string()),
        ),
        (11, Value::Text("mohammed@example.sa".to_string())),
        (12, Value::Text("+966501234567".to_string())),
        (13, Value::Text("SA".to_string())),
        (14, Value::Integer(2.into())),
        (19, Value::Text("Mohammed Abdullah Al-Rashidi".to_string())),
        (20, Value::Text("eng".to_string())),
        (21, Value::Text("SA-RY-RIY".to_string())),
        (22, Value::Text("citizen".to_string())),
        (23, Value::Text("SA".to_string())),
    ])
}

// ── Hindi/Devanagari ──────────────────────────────────────────────────

fn create_hindi_persona() -> Value {
    create_claim169_map(vec![
        (1, Value::Text("IN-AADHAAR-1234".to_string())),
        (3, Value::Text("hin".to_string())),
        (4, Value::Text("राजेश कुमार शर्मा".to_string())),
        (5, Value::Text("राजेश".to_string())),
        (6, Value::Text("कुमार".to_string())),
        (7, Value::Text("शर्मा".to_string())),
        (8, Value::Text("19780622".to_string())),
        (9, Value::Integer(1.into())),
        (
            10,
            Value::Text("फ्लैट नं. 301, अशोक विहार\nसेक्टर 3\nनई दिल्ली 110052".to_string()),
        ),
        (12, Value::Text("+919876543210".to_string())),
        (13, Value::Text("IN".to_string())),
        (14, Value::Integer(2.into())),
        (19, Value::Text("Rajesh Kumar Sharma".to_string())),
        (20, Value::Text("eng".to_string())),
        (23, Value::Text("IN".to_string())),
    ])
}

// ── Chinese ───────────────────────────────────────────────────────────

fn create_chinese_persona() -> Value {
    create_claim169_map(vec![
        (1, Value::Text("CN-110101199001011234".to_string())),
        (3, Value::Text("zho".to_string())),
        (4, Value::Text("张伟".to_string())),
        (5, Value::Text("伟".to_string())),
        (7, Value::Text("张".to_string())),
        (8, Value::Text("19900101".to_string())),
        (9, Value::Integer(1.into())),
        (
            10,
            Value::Text("北京市朝阳区建国路88号\n中国建筑大厦\n100022".to_string()),
        ),
        (11, Value::Text("zhangwei@example.cn".to_string())),
        (12, Value::Text("+8613812345678".to_string())),
        (13, Value::Text("CN".to_string())),
        (14, Value::Integer(1.into())),
        (19, Value::Text("Zhang Wei".to_string())),
        (20, Value::Text("eng".to_string())),
        (23, Value::Text("CN".to_string())),
    ])
}

// ── Cyrillic ──────────────────────────────────────────────────────────

fn create_cyrillic_persona() -> Value {
    create_claim169_map(vec![
        (1, Value::Text("RU-4510-123456".to_string())),
        (3, Value::Text("rus".to_string())),
        (4, Value::Text("Иванов Алексей Петрович".to_string())),
        (5, Value::Text("Алексей".to_string())),
        (6, Value::Text("Петрович".to_string())),
        (7, Value::Text("Иванов".to_string())),
        (8, Value::Text("19820714".to_string())),
        (9, Value::Integer(1.into())),
        (
            10,
            Value::Text("ул. Тверская, д. 15, кв. 42\nМосква, 125009".to_string()),
        ),
        (11, Value::Text("ivanov@example.ru".to_string())),
        (12, Value::Text("+74951234567".to_string())),
        (13, Value::Text("RU".to_string())),
        (14, Value::Integer(2.into())),
        (19, Value::Text("Ivanov Aleksei Petrovich".to_string())),
        (20, Value::Text("eng".to_string())),
        (23, Value::Text("RU".to_string())),
    ])
}

// ── Thai ──────────────────────────────────────────────────────────────

fn create_thai_persona() -> Value {
    create_claim169_map(vec![
        (1, Value::Text("TH-1100501234567".to_string())),
        (3, Value::Text("tha".to_string())),
        (4, Value::Text("สมชาย ใจดี".to_string())),
        (5, Value::Text("สมชาย".to_string())),
        (7, Value::Text("ใจดี".to_string())),
        (8, Value::Text("19950830".to_string())),
        (9, Value::Integer(1.into())),
        (
            10,
            Value::Text("123 ถนนสุขุมวิท\nแขวงคลองเตย เขตคลองเตย\nกรุงเทพมหานคร 10110".to_string()),
        ),
        (12, Value::Text("+66812345678".to_string())),
        (13, Value::Text("TH".to_string())),
        (14, Value::Integer(1.into())),
        (19, Value::Text("Somchai Jaidee".to_string())),
        (20, Value::Text("eng".to_string())),
        (23, Value::Text("TH".to_string())),
    ])
}

// ── Bilingual personas ────────────────────────────────────────────────

fn create_bilingual_arabic_persona() -> Value {
    create_claim169_map(vec![
        (1, Value::Text("3918592438".to_string())),
        (2, Value::Text("1.0".to_string())),
        (3, Value::Text("eng".to_string())),
        (4, Value::Text("Janardhan Bangalore Srinivas".to_string())),
        (5, Value::Text("Janardhan".to_string())),
        (7, Value::Text("Srinivas".to_string())),
        (8, Value::Text("19840118".to_string())),
        (9, Value::Integer(1.into())),
        (
            10,
            Value::Text("Flat No 007, Emerald Park\nNear Metro Line\nBengaluru, KA".to_string()),
        ),
        (11, Value::Text("janardhan@example.com".to_string())),
        (12, Value::Text("+919876543210".to_string())),
        (13, Value::Text("IN".to_string())),
        (14, Value::Integer(2.into())),
        (19, Value::Text("جاناردان بنغالور سرينيفاس".to_string())),
        (20, Value::Text("AR".to_string())),
        (21, Value::Text("849VCWC8+R9".to_string())),
        (22, Value::Text("Refugee".to_string())),
        (23, Value::Text("IN".to_string())),
    ])
}

fn create_bilingual_hindi_persona() -> Value {
    create_claim169_map(vec![
        (1, Value::Text("IN-9876543210".to_string())),
        (3, Value::Text("eng".to_string())),
        (4, Value::Text("Priya Sharma".to_string())),
        (5, Value::Text("Priya".to_string())),
        (7, Value::Text("Sharma".to_string())),
        (8, Value::Text("19920415".to_string())),
        (9, Value::Integer(2.into())),
        (
            10,
            Value::Text("42 MG Road\nBangalore, KA 560001".to_string()),
        ),
        (12, Value::Text("+919012345678".to_string())),
        (13, Value::Text("IN".to_string())),
        (19, Value::Text("प्रिया शर्मा".to_string())),
        (20, Value::Text("hin".to_string())),
        (23, Value::Text("IN".to_string())),
    ])
}

// ── Long text ─────────────────────────────────────────────────────────

fn create_long_address_persona() -> Value {
    let long_address = "Building 42, Floor 7, Suite 712, International Business Park, \
                        Phase III\nPlot No. C-14/15, Sector 62, Noida Technology Corridor\n\
                        Gautam Buddha Nagar District\nUttar Pradesh 201309\nINDIA";
    create_claim169_map(vec![
        (1, Value::Text("IN-LONG-ADDR-001".to_string())),
        (
            4,
            Value::Text("Venkatanarasimharajuvaripeta Subramaniam".to_string()),
        ),
        (8, Value::Text("19880101".to_string())),
        (9, Value::Integer(1.into())),
        (10, Value::Text(long_address.to_string())),
        (12, Value::Text("+919999999999".to_string())),
        (13, Value::Text("IN".to_string())),
    ])
}

// ── Binary data scenarios ─────────────────────────────────────────────

fn create_demographics_with_photo(photo_size: usize) -> Value {
    let photo_data = random_bytes(photo_size);
    create_claim169_map(vec![
        (1, Value::Text("ID-67890-FGHIJ".to_string())),
        (4, Value::Text("Jane Marie Smith".to_string())),
        (5, Value::Text("Jane".to_string())),
        (7, Value::Text("Smith".to_string())),
        (8, Value::Text("19900515".to_string())),
        (9, Value::Integer(2.into())),
        (
            10,
            Value::Text("123 Main St\nNew York, NY 10001".to_string()),
        ),
        (16, Value::Bytes(photo_data)),
        (17, Value::Integer(4.into())),
    ])
}

fn create_demographics_with_uniform_photo(size: usize, byte: u8) -> Value {
    let photo_data = vec![byte; size];
    create_claim169_map(vec![
        (1, Value::Text("ID-67890-FGHIJ".to_string())),
        (4, Value::Text("Jane Marie Smith".to_string())),
        (8, Value::Text("19900515".to_string())),
        (9, Value::Integer(2.into())),
        (16, Value::Bytes(photo_data)),
        (17, Value::Integer(4.into())),
    ])
}

fn create_demographics_with_patterned_bio(total_size: usize) -> Value {
    // Simulate a biometric template with repeating structure (header + blocks)
    let pattern: Vec<u8> = vec![
        0x46, 0x49, 0x52, 0x00, // "FIR\0" header
        0x01, 0x02, 0x03, 0x04, // minutia block
        0x80, 0x40, 0x20, 0x10, // quality scores
        0xFF, 0x00, 0xAA, 0x55, // flags
    ];
    let bio_data: Vec<u8> = pattern.iter().cycle().take(total_size).copied().collect();
    let face = Value::Array(vec![Value::Map(vec![
        (Value::Integer(0.into()), Value::Bytes(bio_data)),
        (Value::Integer(1.into()), Value::Integer(1.into())),
        (Value::Integer(2.into()), Value::Integer(1.into())),
    ])]);
    create_claim169_map(vec![
        (1, Value::Text("ID-67890-FGHIJ".to_string())),
        (4, Value::Text("Jane Marie Smith".to_string())),
        (8, Value::Text("19900515".to_string())),
        (9, Value::Integer(2.into())),
        (62, face),
    ])
}

fn create_demographics_with_face_biometric(bio_size: usize) -> Value {
    let bio_data = random_bytes(bio_size);
    let face = Value::Array(vec![Value::Map(vec![
        (Value::Integer(0.into()), Value::Bytes(bio_data)),
        (Value::Integer(1.into()), Value::Integer(0.into())),
        (Value::Integer(2.into()), Value::Integer(4.into())),
    ])]);
    create_claim169_map(vec![
        (1, Value::Text("ID-67890-FGHIJ".to_string())),
        (4, Value::Text("Jane Marie Smith".to_string())),
        (8, Value::Text("19900515".to_string())),
        (9, Value::Integer(2.into())),
        (
            10,
            Value::Text("123 Main St\nNew York, NY 10001".to_string()),
        ),
        (62, face),
    ])
}

fn create_demographics_with_fingerprints(count: usize, template_size: usize) -> Value {
    let mut fields: Vec<(i64, Value)> = vec![
        (1, Value::Text("ID-67890-FGHIJ".to_string())),
        (4, Value::Text("Jane Marie Smith".to_string())),
        (8, Value::Text("19900515".to_string())),
        (9, Value::Integer(2.into())),
        (
            10,
            Value::Text("123 Main St\nNew York, NY 10001".to_string()),
        ),
    ];
    for i in 0..count {
        let template_data = random_bytes(template_size);
        let bio = Value::Array(vec![Value::Map(vec![
            (Value::Integer(0.into()), Value::Bytes(template_data)),
            (Value::Integer(1.into()), Value::Integer(1.into())),
            (Value::Integer(2.into()), Value::Integer(1.into())),
        ])]);
        fields.push((50 + i as i64, bio));
    }
    create_claim169_map(fields)
}

fn create_full_credential_random() -> Value {
    let photo_data = random_bytes(400);
    let face_data = random_bytes(300);
    let face = Value::Array(vec![Value::Map(vec![
        (Value::Integer(0.into()), Value::Bytes(face_data)),
        (Value::Integer(1.into()), Value::Integer(0.into())),
        (Value::Integer(2.into()), Value::Integer(4.into())),
    ])]);

    create_claim169_map(vec![
        (1, Value::Text("ID-67890-FGHIJ".to_string())),
        (2, Value::Text("1.0".to_string())),
        (3, Value::Text("eng".to_string())),
        (4, Value::Text("Janardhan Bangalore Srinivas".to_string())),
        (5, Value::Text("Janardhan".to_string())),
        (6, Value::Text("Bangalore".to_string())),
        (7, Value::Text("Srinivas".to_string())),
        (8, Value::Text("19840118".to_string())),
        (9, Value::Integer(1.into())),
        (
            10,
            Value::Text("Flat No 007, Emerald Park\nNear Metro Line\nBengaluru, KA".to_string()),
        ),
        (11, Value::Text("janardhan@example.com".to_string())),
        (12, Value::Text("+919876543210".to_string())),
        (13, Value::Text("IN".to_string())),
        (14, Value::Integer(2.into())),
        (16, Value::Bytes(photo_data)),
        (17, Value::Integer(4.into())),
        (
            18,
            Value::Array(vec![Value::Integer(1.into()), Value::Integer(6.into())]),
        ),
        (19, Value::Text("جاناردان بنغالور سرينيفاس".to_string())),
        (20, Value::Text("AR".to_string())),
        (21, Value::Text("849VCWC8+R9".to_string())),
        (22, Value::Text("Refugee".to_string())),
        (23, Value::Text("IN".to_string())),
        (62, face),
    ])
}

// ── Real image scenarios ──────────────────────────────────────────────

fn load_real_image() -> Option<Vec<u8>> {
    // Try multiple paths for the real WebP sample image
    let candidates = [
        "playground/public/sample_id_pictures/sample_id_1.webp",
        "../playground/public/sample_id_pictures/sample_id_1.webp",
    ];

    for path in &candidates {
        if let Ok(data) = std::fs::read(path) {
            return Some(data);
        }
    }

    // Try relative to CARGO_MANIFEST_DIR
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir.parent()?.parent()?;
    let path = workspace_root.join("playground/public/sample_id_pictures/sample_id_1.webp");
    std::fs::read(path).ok()
}

fn create_demographics_with_real_photo(photo_data: &[u8]) -> Value {
    create_claim169_map(vec![
        (1, Value::Text("3215489387".to_string())),
        (4, Value::Text("Janardhan Bangalore Srinivas".to_string())),
        (8, Value::Text("19750404".to_string())),
        (9, Value::Integer(1.into())),
        (
            10,
            Value::Text("Flat No 007, Emerald Park\nNear Metro Line\nBengaluru, KA".to_string()),
        ),
        (12, Value::Text("+919876543210".to_string())),
        (13, Value::Text("IN".to_string())),
        (16, Value::Bytes(photo_data.to_vec())),
        (17, Value::Integer(4.into())),
        (19, Value::Text("جاناردان بنغالور سرينيفاس".to_string())),
        (20, Value::Text("AR".to_string())),
    ])
}

fn create_demographics_with_real_face_bio(photo_data: &[u8]) -> Value {
    let face = Value::Array(vec![Value::Map(vec![
        (Value::Integer(0.into()), Value::Bytes(photo_data.to_vec())),
        (Value::Integer(1.into()), Value::Integer(0.into())),
        (Value::Integer(2.into()), Value::Integer(4.into())),
        (Value::Integer(3.into()), Value::Text("MOSIP".to_string())),
    ])]);
    create_claim169_map(vec![
        (1, Value::Text("3215489387".to_string())),
        (4, Value::Text("Janardhan Bangalore Srinivas".to_string())),
        (8, Value::Text("19750404".to_string())),
        (9, Value::Integer(1.into())),
        (62, face),
    ])
}

fn create_full_credential_real(photo_data: &[u8]) -> Value {
    create_claim169_map(vec![
        (1, Value::Text("3215489387".to_string())),
        (2, Value::Text("1.0".to_string())),
        (3, Value::Text("eng".to_string())),
        (4, Value::Text("Janardhan Bangalore Srinivas".to_string())),
        (5, Value::Text("Janardhan".to_string())),
        (6, Value::Text("Bangalore".to_string())),
        (7, Value::Text("Srinivas".to_string())),
        (8, Value::Text("19750404".to_string())),
        (9, Value::Integer(1.into())),
        (
            10,
            Value::Text("Flat No 007, Emerald Park\nNear Metro Line\nBengaluru, KA".to_string()),
        ),
        (11, Value::Text("janardhan@example.com".to_string())),
        (12, Value::Text("+919876543210".to_string())),
        (13, Value::Text("IN".to_string())),
        (14, Value::Integer(2.into())),
        (16, Value::Bytes(photo_data.to_vec())),
        (17, Value::Integer(4.into())),
        (
            18,
            Value::Array(vec![Value::Integer(1.into()), Value::Integer(6.into())]),
        ),
        (19, Value::Text("جاناردان بنغالور سرينيفاس".to_string())),
        (20, Value::Text("AR".to_string())),
        (21, Value::Text("849VCWC8+R9".to_string())),
        (22, Value::Text("Refugee".to_string())),
        (23, Value::Text("IN".to_string())),
    ])
}

// ════════════════════════════════════════════════════════════════════════════
// COSE builders
// ════════════════════════════════════════════════════════════════════════════

fn build_signed_cose(payload: &[u8], signer: &Ed25519Signer) -> Vec<u8> {
    let protected = HeaderBuilder::new()
        .algorithm(iana::Algorithm::EdDSA)
        .build();

    let mut sign1 = CoseSign1Builder::new()
        .protected(protected)
        .payload(payload.to_vec())
        .build();

    let tbs_data = sign1.tbs_data(&[]);
    let signature = signer
        .sign(iana::Algorithm::EdDSA, None, &tbs_data)
        .expect("signing failed");
    sign1.signature = signature;

    sign1.to_tagged_vec().unwrap()
}

// ════════════════════════════════════════════════════════════════════════════
// Compression helpers
// ════════════════════════════════════════════════════════════════════════════

fn zlib_compress(input: &[u8], level: Compression) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), level);
    encoder
        .write_all(input)
        .expect("compression should not fail");
    encoder.finish().expect("compression should not fail")
}

fn random_bytes(n: usize) -> Vec<u8> {
    let mut buf = vec![0u8; n];
    rand::thread_rng().fill_bytes(&mut buf);
    buf
}

/// Strip the ICC color profile (ICCP chunk) and alpha (ALPH chunk) from a WebP file.
///
/// Mirrors the playground's `stripWebpIccProfile()` in `image.ts`.
/// Chrome embeds an sRGB ICC profile (~456 bytes) via the VP8X extended format.
/// This extracts only the VP8/VP8L bitstream into a minimal RIFF container.
fn strip_webp_icc_profile(data: &[u8]) -> Vec<u8> {
    if data.len() < 20 {
        return data.to_vec();
    }

    // Verify RIFF + WEBP signature
    if &data[0..4] != b"RIFF" || &data[8..12] != b"WEBP" {
        return data.to_vec();
    }

    // Check first chunk at offset 12
    if &data[12..16] != b"VP8X" {
        // Already simple format — no stripping needed
        return data.to_vec();
    }

    // Walk chunks after VP8X to find the VP8/VP8L image data chunk
    // VP8X chunk: 4 (fourCC) + 4 (size) + 10 (payload) = 18 bytes
    let mut offset = 12 + 4 + 4 + 10; // skip RIFF header (12) + VP8X chunk (18)

    while offset + 8 <= data.len() {
        let fourcc = &data[offset..offset + 4];
        let chunk_size = u32::from_le_bytes([
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]) as usize;
        let padded_size = chunk_size + (chunk_size % 2);

        if fourcc == b"VP8 " || fourcc == b"VP8L" {
            // Found the image chunk — rewrap in a minimal RIFF container
            let chunk_total = 8 + padded_size; // fourCC + size + data
            let riff_size = 4 + chunk_total; // "WEBP" + chunk

            let mut result = Vec::with_capacity(12 + chunk_total);
            result.extend_from_slice(b"RIFF");
            result.extend_from_slice(&(riff_size as u32).to_le_bytes());
            result.extend_from_slice(b"WEBP");
            result.extend_from_slice(&data[offset..offset + chunk_total]);

            return result;
        }

        offset += 8 + padded_size;
    }

    // No VP8/VP8L chunk found — return original
    data.to_vec()
}

// ════════════════════════════════════════════════════════════════════════════
// Output formatting
// ════════════════════════════════════════════════════════════════════════════

fn print_row(scenario: &Scenario, r: &ScenarioResult) {
    let verdict = if r.helps { "HELPS" } else { "HURTS" };
    let delta_str = if r.delta_chars >= 0 {
        format!("+{}", r.delta_chars)
    } else {
        format!("{}", r.delta_chars)
    };

    println!(
        "{:<46} {:>6} {:>6} {:>6} {:>8} {:>6} {:>9}",
        scenario.name,
        r.cose_size,
        r.compressed_size,
        r.base45_compressed_len,
        r.base45_raw_len,
        delta_str,
        verdict
    );
}

fn print_entropy_analysis(scenarios: &[Scenario], results: &[ScenarioResult]) {
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!("  BYTE ENTROPY ANALYSIS (Shannon entropy of COSE payload, 0=uniform, 8=random)");
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!();
    println!(
        "{:<46} {:>8} {:>10} {:>10} {:>9}",
        "Scenario", "Entropy", "Comp.ratio", "Category", "Verdict"
    );
    println!("─────────────────────────────────────────────────────────────────────────────────");

    for (i, scenario) in scenarios.iter().enumerate() {
        let r = &results[i];
        let verdict = if r.helps { "HELPS" } else { "HURTS" };
        println!(
            "{:<46} {:>8.2} {:>9.1}% {:>10} {:>9}",
            scenario.name,
            r.entropy,
            r.compression_ratio * 100.0,
            scenario.category.label(),
            verdict
        );
    }

    println!();
    println!("  Interpretation: Higher entropy → less compressible → zlib more likely to hurt.");
    println!("  Typical entropy: text ~4-5 bits, CBOR structures ~5-6, random binary ~7.5-8.");
}

fn print_summary(scenarios: &[Scenario], results: &[ScenarioResult]) {
    let total = results.len();
    let helps_count = results.iter().filter(|r| r.helps).count();
    let hurts_count = results.iter().filter(|r| !r.helps).count();

    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!("  SUMMARY");
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!();
    println!(
        "  Compression HELPS in {}/{} scenarios ({:.0}%)",
        helps_count,
        total,
        helps_count as f64 / total as f64 * 100.0
    );
    println!(
        "  Compression HURTS in {}/{} scenarios ({:.0}%)",
        hurts_count,
        total,
        hurts_count as f64 / total as f64 * 100.0
    );
    println!();

    // Per-category breakdown
    println!("  Per-category breakdown:");
    for cat in &[
        ScenarioCategory::TextOnly,
        ScenarioCategory::WithBinary,
        ScenarioCategory::RealImage,
        ScenarioCategory::Compressible,
    ] {
        let in_cat: Vec<_> = scenarios
            .iter()
            .zip(results.iter())
            .filter(|(s, _)| s.category == *cat)
            .collect();
        if in_cat.is_empty() {
            continue;
        }
        let cat_helps = in_cat.iter().filter(|(_, r)| r.helps).count();
        println!(
            "    {:<16} helps {}/{} ({:.0}%)",
            cat.label(),
            cat_helps,
            in_cat.len(),
            cat_helps as f64 / in_cat.len() as f64 * 100.0
        );
    }
    println!();

    // Biggest win and biggest loss
    let mut best_saving = 0i64;
    let mut worst_penalty = 0i64;
    let mut best_idx = 0;
    let mut worst_idx = 0;

    for (i, r) in results.iter().enumerate() {
        if r.delta_chars < best_saving {
            best_saving = r.delta_chars;
            best_idx = i;
        }
        if r.delta_chars > worst_penalty {
            worst_penalty = r.delta_chars;
            worst_idx = i;
        }
    }

    if best_saving < 0 {
        let pct = (-best_saving) as f64 / results[best_idx].base45_raw_len as f64 * 100.0;
        println!(
            "  Biggest win:  {} chars saved ({:.1}%) — {}",
            -best_saving, pct, scenarios[best_idx].name
        );
    }
    if worst_penalty > 0 {
        let pct = worst_penalty as f64 / results[worst_idx].base45_raw_len as f64 * 100.0;
        println!(
            "  Biggest loss: {} chars added ({:.1}%) — {}",
            worst_penalty, pct, scenarios[worst_idx].name
        );
    }
}

fn print_compression_levels(scenarios: &[Scenario]) {
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!("  COMPRESSION LEVEL COMPARISON (Base45 chars)");
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!();
    println!(
        "{:<46} {:>8} {:>8} {:>8} {:>8}",
        "Scenario", "No zlib", "Fast(1)", "Dflt(6)", "Best(9)"
    );
    println!("─────────────────────────────────────────────────────────────────────────────────");

    for scenario in scenarios {
        let cose = &scenario.cose_bytes;

        let raw = base45_encode(cose).len();
        let fast = base45_encode(&zlib_compress(cose, Compression::fast())).len();
        let default = base45_encode(&zlib_compress(cose, Compression::default())).len();
        let best = base45_encode(&zlib_compress(cose, Compression::best())).len();

        println!(
            "{:<46} {:>8} {:>8} {:>8} {:>8}",
            scenario.name, raw, fast, default, best
        );
    }
}

fn print_qr_version_analysis(scenarios: &[Scenario], results: &[ScenarioResult]) {
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!("  QR CODE VERSION IMPACT");
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!();
    println!("  Base45 uses QR Alphanumeric mode (5.5 bits/char).");
    println!("  QR versions with error correction level L (Low):");
    println!();
    println!(
        "{:<46} {:>8} {:>8} {:>8} {:>8}",
        "Scenario", "w/ zlib", "w/o zlib", "QR+zlib", "QR raw"
    );
    println!("─────────────────────────────────────────────────────────────────────────────────");

    for (i, scenario) in scenarios.iter().enumerate() {
        let r = &results[i];
        let qr_compressed = qr_version_for_alphanumeric(r.base45_compressed_len);
        let qr_raw = qr_version_for_alphanumeric(r.base45_raw_len);

        let qr_comp_str = match qr_compressed {
            Some(v) => format!("V{}", v),
            None => "TOO BIG".to_string(),
        };
        let qr_raw_str = match qr_raw {
            Some(v) => format!("V{}", v),
            None => "TOO BIG".to_string(),
        };

        let marker = if qr_compressed != qr_raw { " *" } else { "" };

        println!(
            "{:<46} {:>8} {:>8} {:>8} {:>8}{}",
            scenario.name,
            r.base45_compressed_len,
            r.base45_raw_len,
            qr_comp_str,
            qr_raw_str,
            marker
        );
    }

    println!();
    println!("  * = compression changes the required QR version");
}

fn print_conclusions(scenarios: &[Scenario], results: &[ScenarioResult]) {
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!("  CONCLUSIONS");
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!();

    // Count version changes
    let mut version_helps = 0;
    let mut version_hurts = 0;
    let mut version_same = 0;

    for r in results {
        let qr_comp = qr_version_for_alphanumeric(r.base45_compressed_len);
        let qr_raw = qr_version_for_alphanumeric(r.base45_raw_len);
        match (qr_comp, qr_raw) {
            (Some(a), Some(b)) if a < b => version_helps += 1,
            (Some(a), Some(b)) if a > b => version_hurts += 1,
            (Some(_), None) => version_helps += 1,
            (None, Some(_)) => version_hurts += 1,
            _ => version_same += 1,
        }
    }

    // Per-category analysis
    for (label, cat) in &[
        (
            "TEXT-ONLY PAYLOADS (demographics, no binary)",
            ScenarioCategory::TextOnly,
        ),
        (
            "RANDOM BINARY DATA (simulated photos/biometrics)",
            ScenarioCategory::WithBinary,
        ),
        (
            "REAL IMAGE DATA (actual WebP from repo)",
            ScenarioCategory::RealImage,
        ),
        (
            "COMPRESSIBLE BINARY (patterned/uniform data)",
            ScenarioCategory::Compressible,
        ),
    ] {
        let in_cat: Vec<_> = scenarios
            .iter()
            .zip(results.iter())
            .filter(|(s, _)| s.category == *cat)
            .collect();
        if in_cat.is_empty() {
            continue;
        }
        let helps = in_cat.iter().filter(|(_, r)| r.helps).count();
        let hurts = in_cat.len() - helps;

        println!("  {}:", label);
        println!(
            "     Compression helps in {}/{}, hurts in {}/{}.",
            helps,
            in_cat.len(),
            hurts,
            in_cat.len()
        );

        if helps > in_cat.len() / 2 {
            println!("     → zlib is generally BENEFICIAL for this category.");
        } else if helps == 0 {
            println!("     → zlib ALWAYS HURTS for this category.");
        } else {
            println!("     → zlib has MIXED results for this category.");
        }
        println!();
    }

    println!("  QR VERSION IMPACT:");
    println!(
        "     Version reduced (smaller QR):  {} scenarios",
        version_helps
    );
    println!(
        "     Version increased (larger QR):  {} scenarios",
        version_hurts
    );
    println!(
        "     Version unchanged:              {} scenarios",
        version_same
    );
    println!();

    // Overall recommendation
    let total_helps = results.iter().filter(|r| r.helps).count();
    let total = results.len();
    let help_pct = total_helps as f64 / total as f64 * 100.0;

    println!("  OVERALL RECOMMENDATION:");
    if help_pct >= 70.0 {
        println!(
            "     zlib compression is BENEFICIAL in the majority of scenarios ({:.0}%).",
            help_pct
        );
        println!("     Keep compression in the pipeline.");
    } else if help_pct >= 40.0 {
        println!(
            "     zlib compression has MIXED results ({:.0}% beneficial).",
            help_pct
        );
        println!("     Consider making compression optional or adaptive.");
    } else {
        println!(
            "     zlib compression HURTS more often than it helps ({:.0}% beneficial).",
            help_pct
        );
        println!("     Consider removing compression or making it adaptive:");
        println!("     encode both ways, emit whichever is shorter.");
    }

    // Adaptive compression analysis
    println!();
    println!("  ADAPTIVE COMPRESSION (try both, pick smaller):");
    let mut adaptive_total_saving = 0i64;
    for r in results {
        let best = r.base45_compressed_len.min(r.base45_raw_len);
        let current = r.base45_compressed_len;
        adaptive_total_saving += current as i64 - best as i64;
    }
    let hurts_count = results.iter().filter(|r| !r.helps).count();
    println!(
        "     Would improve {}/{} scenarios vs always-compress.",
        hurts_count, total
    );
    println!(
        "     Total chars saved vs always-compress: {}",
        adaptive_total_saving
    );

    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════════");
}

// ════════════════════════════════════════════════════════════════════════════
// QR Code version lookup
// ════════════════════════════════════════════════════════════════════════════

/// Returns the minimum QR Code version needed for the given number of
/// alphanumeric characters, using error correction level L.
fn qr_version_for_alphanumeric(chars: usize) -> Option<u8> {
    // Alphanumeric capacity for QR versions 1-40 at EC level L
    let capacities: &[usize] = &[
        25, 47, 77, 114, 154, 195, 224, 279, 335, 395, // V1-V10
        468, 535, 619, 667, 758, 854, 938, 1046, 1153, 1249, // V11-V20
        1352, 1460, 1588, 1704, 1853, 1990, 2132, 2223, 2369, 2520, // V21-V30
        2677, 2840, 3009, 3183, 3351, 3537, 3729, 3927, 4087, 4296, // V31-V40
    ];

    for (i, &cap) in capacities.iter().enumerate() {
        if chars <= cap {
            return Some((i + 1) as u8);
        }
    }
    None
}
