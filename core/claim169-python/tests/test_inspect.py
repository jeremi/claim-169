"""Tests for the claim169 inspect functionality."""

import pytest

import claim169


class TestInspectSigned:
    """Tests for inspecting signed credentials."""

    def test_inspect_returns_issuer_kid_algorithm(self, ed25519_signed_vector):
        """Test that inspect extracts issuer, kid, and algorithm from a signed credential."""
        result = claim169.inspect(ed25519_signed_vector["qr_data"])

        expected_meta = ed25519_signed_vector.get("expected_cwt_meta", {})
        assert result.issuer == expected_meta.get("issuer")
        assert result.algorithm == "EdDSA"
        assert result.cose_type == "Sign1"

    def test_inspect_cose_type_is_sign1(self, ed25519_signed_vector):
        """Test that inspect correctly identifies Sign1 type."""
        result = claim169.inspect(ed25519_signed_vector["qr_data"])

        assert result.cose_type == "Sign1"

    def test_inspect_expires_at(self, ed25519_signed_vector):
        """Test that inspect returns the expiration timestamp."""
        result = claim169.inspect(ed25519_signed_vector["qr_data"])

        expected_meta = ed25519_signed_vector.get("expected_cwt_meta", {})
        assert result.expires_at == expected_meta.get("expiresAt")

    def test_inspect_ecdsa_p256_signed(self, ecdsa_p256_signed_vector):
        """Test inspecting an ECDSA P-256 signed credential."""
        result = claim169.inspect(ecdsa_p256_signed_vector["qr_data"])

        expected_meta = ecdsa_p256_signed_vector.get("expected_cwt_meta", {})
        assert result.issuer == expected_meta.get("issuer")
        assert result.algorithm == "ES256"
        assert result.cose_type == "Sign1"


class TestInspectUnsigned:
    """Tests for inspecting unsigned credentials."""

    def test_inspect_unsigned_credential(self, minimal_vector):
        """Test that inspect works on unsigned credentials."""
        result = claim169.inspect(minimal_vector["qr_data"])

        expected_meta = minimal_vector.get("expected_cwt_meta", {})
        assert result.issuer == expected_meta.get("issuer")
        assert result.key_id is None
        assert result.cose_type == "Sign1"

    def test_inspect_returns_subject(self, demographics_full_vector):
        """Test that inspect returns subject from CWT claims."""
        result = claim169.inspect(demographics_full_vector["qr_data"])

        expected_meta = demographics_full_vector.get("expected_cwt_meta", {})
        if "subject" in expected_meta:
            assert result.subject == expected_meta["subject"]


class TestInspectEncrypted:
    """Tests for inspecting encrypted credentials."""

    def test_inspect_encrypted_returns_encrypt0_type(self, encrypted_aes256_vector):
        """Test that inspect identifies Encrypt0 type for encrypted credentials."""
        result = claim169.inspect(encrypted_aes256_vector["qr_data"])

        assert result.cose_type == "Encrypt0"

    def test_inspect_encrypted_cwt_fields_are_none(self, encrypted_aes256_vector):
        """Test that CWT-level fields are None for encrypted credentials."""
        result = claim169.inspect(encrypted_aes256_vector["qr_data"])

        # CWT fields are not accessible for encrypted payloads
        assert result.issuer is None
        assert result.subject is None
        assert result.expires_at is None

    def test_inspect_encrypted_has_algorithm(self, encrypted_aes256_vector):
        """Test that encrypted credentials have an algorithm in the header."""
        result = claim169.inspect(encrypted_aes256_vector["qr_data"])

        assert result.algorithm is not None


class TestInspectErrors:
    """Tests for inspect error handling."""

    def test_inspect_invalid_base45(self):
        """Test that inspect raises Base45DecodeError for invalid input."""
        with pytest.raises(claim169.Base45DecodeError):
            claim169.inspect("NOT_VALID_BASE45!!!")

    def test_inspect_empty_string(self):
        """Test that inspect raises an error for empty input."""
        with pytest.raises(claim169.Claim169Exception):
            claim169.inspect("")


class TestInspectRepr:
    """Tests for InspectResult representation."""

    def test_repr_contains_class_name(self, minimal_vector):
        """Test that InspectResult repr includes class name."""
        result = claim169.inspect(minimal_vector["qr_data"])
        repr_str = repr(result)
        assert "InspectResult" in repr_str

    def test_key_id_type(self, ed25519_signed_vector):
        """Test that key_id is bytes when present."""
        result = claim169.inspect(ed25519_signed_vector["qr_data"])
        if result.key_id is not None:
            assert isinstance(result.key_id, bytes)


class TestInspectRoundtrip:
    """Tests for inspecting programmatically-created credentials."""

    def test_inspect_roundtrip_with_ed25519(self, ed25519_signed_vector):
        """Test inspecting a credential created via encode_with_ed25519."""
        private_key = bytes.fromhex(
            ed25519_signed_vector["signing_key"]["private_key_hex"]
        )

        claim = claim169.Claim169Input(
            id="INSPECT-RT-001",
            full_name="Inspect Roundtrip Person"
        )
        meta = claim169.CwtMetaInput()
        meta.issuer = "https://inspect.roundtrip.org"
        meta.expires_at = 1900000000

        qr_data = claim169.encode_with_ed25519(claim, meta, private_key)
        result = claim169.inspect(qr_data)

        assert result.issuer == "https://inspect.roundtrip.org"
        assert result.algorithm == "EdDSA"
        assert result.cose_type == "Sign1"
        assert result.expires_at == 1900000000

    def test_inspect_roundtrip_unsigned(self):
        """Test inspecting an unsigned credential created via encode_unsigned."""
        claim = claim169.Claim169Input(
            id="INSPECT-UNSIGNED-001",
            full_name="Unsigned Inspect Person"
        )
        meta = claim169.CwtMetaInput()
        meta.issuer = "https://unsigned.inspect.org"
        meta.expires_at = 1900000000

        qr_data = claim169.encode_unsigned(claim, meta)
        result = claim169.inspect(qr_data)

        assert result.issuer == "https://unsigned.inspect.org"
        assert result.key_id is None
        assert result.cose_type == "Sign1"
        assert result.expires_at == 1900000000
