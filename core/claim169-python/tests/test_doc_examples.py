"""Tests that verify documentation code examples work correctly.

Each test in this file corresponds to a code example in the documentation.
If a test here fails, the corresponding docs page has a broken example.
"""


# -- Getting Started: Decode Your First QR Code (Python tab) --
# Docs: docs/en/getting-started.md, docs/fr/getting-started.md


def test_getting_started_decode():
    """Verify the getting-started decode example produces expected output."""
    import claim169

    qr_data = "6BF590B20FFWJWG.FKJ05H7B0XKA8FA9DIWENPEJ/5P$DPQE88EB$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL580 B9%W5*B9C+9%R8646%86HKESED1/DRTC5UA QE$345$CVQEX.DX88WBK0NG8PB4 O/38TL6XDALLKLPQATHO.3ZPJMUAVQFSB1:+B*21V FWMC6SU439YU774475LJ2U5T02$VBSIMLQ3:6J.E1-1STM$4"

    public_key = bytes.fromhex(
        "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
    )

    result = claim169.decode(qr_data, verify_with_ed25519=public_key)

    assert result.claim169.id == "ID-SIGNED-001"
    assert result.claim169.full_name == "Signed Test Person"
    assert result.cwt_meta.issuer == "https://mosip.example.org"
    assert result.verification_status == "verified"


# -- Landing Page: Quick Example (Python tab) --
# Docs: docs/en/index.md, docs/fr/index.md


def test_landing_page_quick_example():
    """Verify the landing page quick example imports and API shape."""
    import claim169

    # The landing page uses placeholder "..." values, so we test the
    # same API shape with real test vector data.
    qr_data = "6BF590B20FFWJWG.FKJ05H7B0XKA8FA9DIWENPEJ/5P$DPQE88EB$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL580 B9%W5*B9C+9%R8646%86HKESED1/DRTC5UA QE$345$CVQEX.DX88WBK0NG8PB4 O/38TL6XDALLKLPQATHO.3ZPJMUAVQFSB1:+B*21V FWMC6SU439YU774475LJ2U5T02$VBSIMLQ3:6J.E1-1STM$4"
    public_key = bytes.fromhex(
        "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
    )

    result = claim169.decode(qr_data, verify_with_ed25519=public_key)

    # Verify the attribute names used in docs exist and work
    assert result.claim169.full_name is not None
    assert result.claim169.id is not None
    assert result.verification_status is not None
