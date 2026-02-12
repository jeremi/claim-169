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

    let scenarios = build_scenarios();

    // ── Per-scenario detailed report ──────────────────────────────────────
    println!("─────────────────────────────────────────────────────────────────────────────");
    println!(
        "{:<42} {:>6} {:>6} {:>6} {:>8} {:>6} {:>9}",
        "Scenario", "COSE", "zlib", "B45+z", "B45 raw", "Delta", "Verdict"
    );
    println!(
        "{:<42} {:>6} {:>6} {:>6} {:>8} {:>6} {:>9}",
        "", "bytes", "bytes", "chars", "chars", "chars", ""
    );
    println!("─────────────────────────────────────────────────────────────────────────────");

    let mut results: Vec<ScenarioResult> = Vec::new();

    for scenario in &scenarios {
        let result = analyze_scenario(scenario);
        print_row(scenario, &result);
        results.push(result);
    }

    println!("─────────────────────────────────────────────────────────────────────────────");
    println!();

    // ── Summary statistics ────────────────────────────────────────────────
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

struct Scenario {
    name: String,
    description: String,
    cose_bytes: Vec<u8>,
}

struct ScenarioResult {
    cose_size: usize,
    compressed_size: usize,
    base45_compressed_len: usize,
    base45_raw_len: usize,
    #[allow(dead_code)]
    compression_ratio: f64,
    delta_chars: i64,
    helps: bool,
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

    ScenarioResult {
        cose_size,
        compressed_size,
        base45_compressed_len,
        base45_raw_len,
        compression_ratio,
        delta_chars,
        helps,
    }
}

// ════════════════════════════════════════════════════════════════════════════
// Scenario builders
// ════════════════════════════════════════════════════════════════════════════

fn build_scenarios() -> Vec<Scenario> {
    let signer = Ed25519Signer::generate();

    vec![
        // ── Unsigned scenarios (isolate compression effect) ──────────
        build_unsigned_scenario(
            "1. Minimal (ID + name)",
            "Unsigned, just id and full_name",
            &create_claim169_map(vec![
                (1, Value::Text("ID-12345-ABCDE".to_string())),
                (4, Value::Text("John Doe".to_string())),
            ]),
            &minimal_cwt(),
        ),
        build_unsigned_scenario(
            "2. Basic demographics",
            "ID, name, DOB, gender, address",
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
        ),
        build_unsigned_scenario(
            "3. Full demographics (all 23 keys)",
            "Every demographic field populated",
            &create_full_demographics(),
            &full_cwt(),
        ),
        build_unsigned_scenario(
            "4. Demographics + photo 200B",
            "Small WebP photo (200 bytes random)",
            &create_demographics_with_photo(200),
            &full_cwt(),
        ),
        build_unsigned_scenario(
            "5. Demographics + photo 400B",
            "Medium-small WebP photo",
            &create_demographics_with_photo(400),
            &full_cwt(),
        ),
        build_unsigned_scenario(
            "6. Demographics + photo 600B",
            "Medium WebP photo",
            &create_demographics_with_photo(600),
            &full_cwt(),
        ),
        build_unsigned_scenario(
            "7. Demographics + photo 800B",
            "Large WebP photo (near QR limit)",
            &create_demographics_with_photo(800),
            &full_cwt(),
        ),
        build_unsigned_scenario(
            "8. Demographics + face biometric 500B",
            "Face biometric as CBOR biometric entry",
            &create_demographics_with_face_biometric(500),
            &full_cwt(),
        ),
        build_unsigned_scenario(
            "9. Demographics + 2 fingerprint templates",
            "Two 64B fingerprint templates",
            &create_demographics_with_fingerprints(2, 64),
            &full_cwt(),
        ),
        build_unsigned_scenario(
            "10. Demographics + 4 fingerprint templates",
            "Four 64B fingerprint templates",
            &create_demographics_with_fingerprints(4, 64),
            &full_cwt(),
        ),
        build_unsigned_scenario(
            "11. Full credential (demographics+photo+face)",
            "All demographics + 400B photo + 300B face bio",
            &create_full_credential(),
            &full_cwt(),
        ),
        // ── Signed scenarios (realistic production payloads) ────────
        build_signed_scenario(
            "12. Signed: minimal",
            "Ed25519 signed, ID + name only",
            &create_claim169_map(vec![
                (1, Value::Text("ID-12345-ABCDE".to_string())),
                (4, Value::Text("John Doe".to_string())),
            ]),
            &full_cwt(),
            &signer,
        ),
        build_signed_scenario(
            "13. Signed: full demographics",
            "Ed25519 signed, all 23 fields",
            &create_full_demographics(),
            &full_cwt(),
            &signer,
        ),
        build_signed_scenario(
            "14. Signed: demo + photo 400B",
            "Ed25519 signed + 400B photo",
            &create_demographics_with_photo(400),
            &full_cwt(),
            &signer,
        ),
        build_signed_scenario(
            "15. Signed: demo + photo 600B",
            "Ed25519 signed + 600B photo",
            &create_demographics_with_photo(600),
            &full_cwt(),
            &signer,
        ),
        build_signed_scenario(
            "16. Signed: full credential",
            "Ed25519 signed, all fields + photo + bio",
            &create_full_credential(),
            &full_cwt(),
            &signer,
        ),
        // ── Edge cases: highly compressible data ────────────────────
        build_unsigned_scenario(
            "17. Repetitive photo (all zeros, 400B)",
            "Photo data that compresses extremely well",
            &create_demographics_with_compressible_photo(400),
            &full_cwt(),
        ),
        build_unsigned_scenario(
            "18. Repetitive photo (all zeros, 800B)",
            "Larger compressible photo data",
            &create_demographics_with_compressible_photo(800),
            &full_cwt(),
        ),
        // ── Edge case: very small payloads ──────────────────────────
        build_unsigned_scenario(
            "19. Absolute minimal (id only)",
            "Single short field",
            &create_claim169_map(vec![(1, Value::Text("X".to_string()))]),
            &CwtMeta::new().with_issuer("i"),
        ),
        build_unsigned_scenario(
            "20. Bilingual name + secondary lang",
            "Demographics with Arabic secondary name",
            &create_claim169_map(vec![
                (1, Value::Text("3918592438".to_string())),
                (4, Value::Text("Janardhan BS".to_string())),
                (8, Value::Text("19840118".to_string())),
                (9, Value::Integer(1.into())),
                (19, Value::Text("جاناردان بنغالور سرينيفاس".to_string())),
                (20, Value::Text("AR".to_string())),
            ]),
            &full_cwt(),
        ),
    ]
}

fn build_unsigned_scenario(
    name: &str,
    description: &str,
    claim_169_cbor: &Value,
    cwt_meta: &CwtMeta,
) -> Scenario {
    let cwt_bytes = cwt_encode(cwt_meta, claim_169_cbor);
    let cose_bytes = build_unsigned_cose(&cwt_bytes);
    Scenario {
        name: name.to_string(),
        description: description.to_string(),
        cose_bytes,
    }
}

fn build_signed_scenario(
    name: &str,
    description: &str,
    claim_169_cbor: &Value,
    cwt_meta: &CwtMeta,
    signer: &Ed25519Signer,
) -> Scenario {
    let cwt_bytes = cwt_encode(cwt_meta, claim_169_cbor);
    let cose_bytes = build_signed_cose(&cwt_bytes, signer);
    Scenario {
        name: name.to_string(),
        description: description.to_string(),
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

fn create_full_demographics() -> Value {
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

fn create_demographics_with_compressible_photo(photo_size: usize) -> Value {
    let photo_data = vec![0u8; photo_size];
    create_claim169_map(vec![
        (1, Value::Text("ID-67890-FGHIJ".to_string())),
        (4, Value::Text("Jane Marie Smith".to_string())),
        (8, Value::Text("19900515".to_string())),
        (9, Value::Integer(2.into())),
        (16, Value::Bytes(photo_data)),
        (17, Value::Integer(4.into())),
    ])
}

fn create_demographics_with_face_biometric(bio_size: usize) -> Value {
    let bio_data = random_bytes(bio_size);
    let face = Value::Array(vec![Value::Map(vec![
        (Value::Integer(0.into()), Value::Bytes(bio_data)),
        (Value::Integer(1.into()), Value::Integer(0.into())), // Image format
        (Value::Integer(2.into()), Value::Integer(4.into())), // WebP sub-format
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

    // Add fingerprint biometrics starting at key 50 (right thumb)
    for i in 0..count {
        let template_data = random_bytes(template_size);
        let bio = Value::Array(vec![Value::Map(vec![
            (Value::Integer(0.into()), Value::Bytes(template_data)),
            (Value::Integer(1.into()), Value::Integer(1.into())), // Template format
            (Value::Integer(2.into()), Value::Integer(1.into())), // ISO 19794-2
        ])]);
        fields.push((50 + i as i64, bio));
    }

    create_claim169_map(fields)
}

fn create_full_credential() -> Value {
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

// ════════════════════════════════════════════════════════════════════════════
// COSE builders
// ════════════════════════════════════════════════════════════════════════════

fn build_unsigned_cose(payload: &[u8]) -> Vec<u8> {
    let sign1 = CoseSign1Builder::new()
        .protected(
            HeaderBuilder::new()
                .algorithm(iana::Algorithm::EdDSA)
                .build(),
        )
        .payload(payload.to_vec())
        .build();
    sign1.to_tagged_vec().unwrap()
}

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
        "{:<42} {:>6} {:>6} {:>6} {:>8} {:>6} {:>9}",
        scenario.name,
        r.cose_size,
        r.compressed_size,
        r.base45_compressed_len,
        r.base45_raw_len,
        delta_str,
        verdict
    );
}

fn print_summary(scenarios: &[Scenario], results: &[ScenarioResult]) {
    let helps_count = results.iter().filter(|r| r.helps).count();
    let hurts_count = results.iter().filter(|r| !r.helps).count();
    let total = results.len();

    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("  SUMMARY");
    println!("═══════════════════════════════════════════════════════════════════════════════");
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
    println!();

    // Breakdown by data characteristics
    println!("  ┌─────────────────────────────────────────────────────────────────────────┐");
    println!("  │ Key observations:                                                       │");
    println!("  │                                                                         │");

    // Check: small payloads (< 200 bytes COSE)
    let small: Vec<_> = results
        .iter()
        .enumerate()
        .filter(|(_, r)| r.cose_size < 200)
        .collect();
    if !small.is_empty() {
        let small_hurts = small.iter().filter(|(_, r)| !r.helps).count();
        println!(
            "  │ Small payloads (<200B COSE): compression hurts in {}/{} cases{} │",
            small_hurts,
            small.len(),
            " ".repeat(14 - format!("{}/{}", small_hurts, small.len()).len())
        );
    }

    // Check: medium payloads (200-500 bytes COSE)
    let medium: Vec<_> = results
        .iter()
        .enumerate()
        .filter(|(_, r)| r.cose_size >= 200 && r.cose_size < 500)
        .collect();
    if !medium.is_empty() {
        let medium_helps = medium.iter().filter(|(_, r)| r.helps).count();
        println!(
            "  │ Medium payloads (200-500B COSE): compression helps in {}/{} cases{}│",
            medium_helps,
            medium.len(),
            " ".repeat(13 - format!("{}/{}", medium_helps, medium.len()).len())
        );
    }

    // Check: large payloads (>= 500 bytes COSE)
    let large: Vec<_> = results
        .iter()
        .enumerate()
        .filter(|(_, r)| r.cose_size >= 500)
        .collect();
    if !large.is_empty() {
        let large_helps = large.iter().filter(|(_, r)| r.helps).count();
        println!(
            "  │ Large payloads (>=500B COSE): compression helps in {}/{} cases{}│",
            large_helps,
            large.len(),
            " ".repeat(15 - format!("{}/{}", large_helps, large.len()).len())
        );
    }

    // Check: random binary data (photos, biometrics)
    let has_random: Vec<_> = results
        .iter()
        .enumerate()
        .filter(|(i, _)| {
            let name = &scenarios[*i].name;
            name.contains("photo")
                || name.contains("biometric")
                || name.contains("face")
                || name.contains("finger")
                || name.contains("credential")
        })
        .filter(|(_, r)| r.cose_size >= 200)
        .collect();
    if !has_random.is_empty() {
        let random_hurts = has_random.iter().filter(|(_, r)| !r.helps).count();
        println!(
            "  │ Payloads with random binary (photos/bio): hurts in {}/{} cases{}│",
            random_hurts,
            has_random.len(),
            " ".repeat(10 - format!("{}/{}", random_hurts, has_random.len()).len())
        );
    }

    println!("  └─────────────────────────────────────────────────────────────────────────┘");
}

fn print_compression_levels(scenarios: &[Scenario]) {
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("  COMPRESSION LEVEL COMPARISON (Base45 chars)");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    println!(
        "{:<42} {:>8} {:>8} {:>8} {:>8}",
        "Scenario", "No zlib", "Fast(1)", "Dflt(6)", "Best(9)"
    );
    println!("─────────────────────────────────────────────────────────────────────────────");

    for scenario in scenarios {
        let cose = &scenario.cose_bytes;

        let raw = base45_encode(cose).len();
        let fast = base45_encode(&zlib_compress(cose, Compression::fast())).len();
        let default = base45_encode(&zlib_compress(cose, Compression::default())).len();
        let best = base45_encode(&zlib_compress(cose, Compression::best())).len();

        println!(
            "{:<42} {:>8} {:>8} {:>8} {:>8}",
            scenario.name, raw, fast, default, best
        );
    }
}

fn print_qr_version_analysis(scenarios: &[Scenario], results: &[ScenarioResult]) {
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("  QR CODE VERSION IMPACT");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    println!("  Base45 uses QR Alphanumeric mode (5.5 bits/char).");
    println!("  QR versions with error correction level L (Low):");
    println!();
    println!(
        "{:<42} {:>8} {:>8} {:>8} {:>8}",
        "Scenario", "w/ zlib", "w/o zlib", "QR+zlib", "QR raw"
    );
    println!("─────────────────────────────────────────────────────────────────────────────");

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
            "{:<42} {:>8} {:>8} {:>8} {:>8}{}",
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
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("  CONCLUSIONS");
    println!("═══════════════════════════════════════════════════════════════════════════════");
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
            (Some(_), None) => version_helps += 1, // compression made it fit
            (None, Some(_)) => version_hurts += 1, // compression made it not fit
            _ => version_same += 1,
        }
    }

    // Analyze random binary content scenarios
    let random_scenarios: Vec<_> = results
        .iter()
        .enumerate()
        .filter(|(i, _)| {
            let desc = &scenarios[*i].description;
            desc.contains("random")
                || desc.contains("WebP")
                || desc.contains("photo")
                || desc.contains("bio")
                || desc.contains("credential")
        })
        .collect();

    let random_hurts: Vec<_> = random_scenarios.iter().filter(|(_, r)| !r.helps).collect();

    let text_scenarios: Vec<_> = results
        .iter()
        .enumerate()
        .filter(|(i, _)| {
            let desc = &scenarios[*i].description;
            !desc.contains("random")
                && !desc.contains("WebP")
                && !desc.contains("photo")
                && !desc.contains("bio")
                && !desc.contains("credential")
                && !desc.contains("compressible")
                && !desc.contains("zeros")
        })
        .collect();
    let text_helps: Vec<_> = text_scenarios.iter().filter(|(_, r)| r.helps).collect();

    println!("  1. TEXT-DOMINATED PAYLOADS (demographics only, no binary):");
    if text_scenarios.is_empty() {
        println!("     No pure-text scenarios found.");
    } else {
        println!(
            "     Compression helps in {}/{} cases.",
            text_helps.len(),
            text_scenarios.len()
        );
        if text_helps.len() > text_scenarios.len() / 2 {
            println!("     → zlib is generally BENEFICIAL for text-heavy credentials.");
        } else {
            println!("     → zlib has MIXED results for text-heavy credentials.");
        }
    }
    println!();

    println!("  2. PAYLOADS WITH RANDOM BINARY DATA (photos, biometrics):");
    if random_scenarios.is_empty() {
        println!("     No random-binary scenarios found.");
    } else {
        println!(
            "     Compression hurts in {}/{} cases.",
            random_hurts.len(),
            random_scenarios.len()
        );
        if random_hurts.len() > random_scenarios.len() / 2 {
            println!("     → zlib generally INCREASES size when data contains pre-compressed");
            println!("       images (JPEG, WebP) or random biometric data.");
        } else {
            println!("     → zlib still helps in most cases even with binary data.");
        }
    }
    println!();

    println!("  3. QR VERSION IMPACT:");
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

    println!("  4. OVERALL RECOMMENDATION:");
    if help_pct >= 70.0 {
        println!(
            "     zlib compression is BENEFICIAL in the majority of scenarios ({:.0}%).",
            help_pct
        );
        println!("     Keep compression in the pipeline. The overhead on incompressible data");
        println!("     (typically +5-15 chars) is small relative to the savings on text-heavy");
        println!("     payloads.");
    } else if help_pct >= 40.0 {
        println!(
            "     zlib compression has MIXED results ({:.0}% beneficial).",
            help_pct
        );
        println!("     Consider making compression optional, or use adaptive compression:");
        println!("     compress, then compare sizes, and use whichever is smaller.");
    } else {
        println!(
            "     zlib compression HURTS more often than it helps ({:.0}% beneficial).",
            help_pct
        );
        println!("     Consider removing compression from the pipeline, or making it");
        println!("     conditional based on payload characteristics.");
    }

    // Specific recommendation about adaptive approach
    println!();
    println!("  5. ADAPTIVE COMPRESSION (try both, pick smaller):");
    let mut adaptive_wins = 0;
    let mut adaptive_total_saving = 0i64;
    for r in results {
        let best = r.base45_compressed_len.min(r.base45_raw_len);
        let worst = r.base45_compressed_len.max(r.base45_raw_len);
        if best < worst {
            adaptive_wins += 1;
        }
        // Compare to always-compress
        let current = r.base45_compressed_len;
        adaptive_total_saving += current as i64 - best as i64;
    }
    println!(
        "     Adaptive approach would improve {} of {} scenarios vs always-compress.",
        adaptive_wins - total_helps,
        total
    );
    println!(
        "     Total chars saved vs always-compress: {}",
        adaptive_total_saving
    );

    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
}

// ════════════════════════════════════════════════════════════════════════════
// QR Code version lookup
// ════════════════════════════════════════════════════════════════════════════

/// Returns the minimum QR Code version needed for the given number of
/// alphanumeric characters, using error correction level L.
///
/// QR versions 1-40, alphanumeric capacity at EC level L.
/// Returns None if the data exceeds the maximum capacity (version 40-L = 4296 chars).
fn qr_version_for_alphanumeric(chars: usize) -> Option<u8> {
    // Alphanumeric capacity for QR versions 1-40 at EC level L
    let capacities: &[usize] = &[
        25,   // V1
        47,   // V2
        77,   // V3
        114,  // V4
        154,  // V5
        195,  // V6
        224,  // V7
        279,  // V8
        335,  // V9
        395,  // V10
        468,  // V11
        535,  // V12
        619,  // V13
        667,  // V14
        758,  // V15
        854,  // V16
        938,  // V17
        1046, // V18
        1153, // V19
        1249, // V20
        1352, // V21
        1460, // V22
        1588, // V23
        1704, // V24
        1853, // V25
        1990, // V26
        2132, // V27
        2223, // V28
        2369, // V29
        2520, // V30
        2677, // V31
        2840, // V32
        3009, // V33
        3183, // V34
        3351, // V35
        3537, // V36
        3729, // V37
        3927, // V38
        4087, // V39
        4296, // V40
    ];

    for (i, &cap) in capacities.iter().enumerate() {
        if chars <= cap {
            return Some((i + 1) as u8);
        }
    }
    None
}
