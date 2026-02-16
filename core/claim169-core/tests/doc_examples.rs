//! Tests that verify documentation code examples work correctly.
//!
//! Each test corresponds to a code example in the docs.
//! If a test here fails, the corresponding docs page has a broken example.

use claim169_core::Decoder;

/// Getting Started: Decode Your First QR Code (Rust tab)
/// Docs: docs/en/getting-started.md, docs/fr/getting-started.md
#[test]
fn test_getting_started_decode() {
    let qr_data = "6BF590B20FFWJWG.FKJ05H7B0XKA8FA9DIWENPEJ/5P$DPQE88EB$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL580 B9%W5*B9C+9%R8646%86HKESED1/DRTC5UA QE$345$CVQEX.DX88WBK0NG8PB4 O/38TL6XDALLKLPQATHO.3ZPJMUAVQFSB1:+B*21V FWMC6SU439YU774475LJ2U5T02$VBSIMLQ3:6J.E1-1STM$4";

    let public_key = hex::decode(
        "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
    )
    .unwrap();

    let result = Decoder::new(qr_data)
        .verify_with_ed25519(&public_key)
        .unwrap()
        .decode()
        .unwrap();

    assert_eq!(result.claim169.id.as_deref(), Some("ID-SIGNED-001"));
    assert_eq!(
        result.claim169.full_name.as_deref(),
        Some("Signed Test Person")
    );
    assert_eq!(
        result.cwt_meta.issuer.as_deref(),
        Some("https://mosip.example.org")
    );
}

/// Landing page: Quick Example (Rust tab)
/// Docs: docs/en/index.md, docs/fr/index.md
#[test]
fn test_landing_page_quick_example() {
    // The landing page uses the same Decoder API with placeholder values.
    // We test the API shape with real test vector data.
    let qr_data = "6BF590B20FFWJWG.FKJ05H7B0XKA8FA9DIWENPEJ/5P$DPQE88EB$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL580 B9%W5*B9C+9%R8646%86HKESED1/DRTC5UA QE$345$CVQEX.DX88WBK0NG8PB4 O/38TL6XDALLKLPQATHO.3ZPJMUAVQFSB1:+B*21V FWMC6SU439YU774475LJ2U5T02$VBSIMLQ3:6J.E1-1STM$4";
    let public_key = hex::decode(
        "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
    )
    .unwrap();

    let result = Decoder::new(qr_data)
        .verify_with_ed25519(&public_key)
        .unwrap()
        .decode()
        .unwrap();

    // Verify the attribute names used in docs exist and work
    assert!(result.claim169.full_name.is_some());
    assert!(result.claim169.id.is_some());
}
