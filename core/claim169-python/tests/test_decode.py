"""Tests for the claim169 decode functionality."""

import pytest

import claim169


class TestModule:
    """Tests for the module itself."""

    def test_version_returns_string(self):
        """Test that version() returns a version string."""
        version = claim169.version()
        assert isinstance(version, str)
        assert version.count(".") >= 2  # Should be semver format

    def test_dunder_version_matches_version(self):
        """Test that __version__ matches version()."""
        assert claim169.__version__ == claim169.version()

    def test_decode_requires_verification_by_default(self, minimal_vector):
        """Test that decode() requires a key unless allow_unverified=True."""
        with pytest.raises(ValueError):
            claim169.decode(minimal_vector["qr_data"])

        result = claim169.decode(minimal_vector["qr_data"], allow_unverified=True)
        assert result.verification_status == "skipped"


class TestDecodeValidVectors:
    """Tests for decoding valid test vectors."""

    def test_decode_minimal(self, minimal_vector):
        """Test decoding the minimal vector."""
        result = claim169.decode_unverified(minimal_vector["qr_data"])

        expected = minimal_vector["expected_claim169"]
        assert result.claim169.id == expected.get("id")
        assert result.claim169.full_name == expected.get("fullName")
        assert result.verification_status == "skipped"

    def test_decode_demographics_full(self, demographics_full_vector):
        """Test decoding the demographics-full vector."""
        result = claim169.decode_unverified(demographics_full_vector["qr_data"])

        expected = demographics_full_vector["expected_claim169"]
        assert result.claim169.id == expected.get("id")
        assert result.claim169.full_name == expected.get("fullName")
        if "firstName" in expected:
            assert result.claim169.first_name == expected["firstName"]

        expected_meta = demographics_full_vector.get("expected_cwt_meta", {})
        if "issuer" in expected_meta:
            assert result.cwt_meta.issuer == expected_meta["issuer"]

    def test_decode_with_face(self, with_face_vector):
        """Test decoding the with-face vector."""
        result = claim169.decode_unverified(with_face_vector["qr_data"])

        expected = with_face_vector["expected_claim169"]
        assert result.claim169.id == expected.get("id")
        assert result.claim169.face is not None
        assert len(result.claim169.face) > 0

    def test_decode_with_fingerprints(self, with_fingerprints_vector):
        """Test decoding the with-fingerprints vector."""
        result = claim169.decode_unverified(with_fingerprints_vector["qr_data"])

        expected = with_fingerprints_vector["expected_claim169"]
        assert result.claim169.id == expected.get("id")

        # Check at least one fingerprint field is present
        has_fingerprint = any([
            result.claim169.right_thumb is not None,
            result.claim169.right_pointer_finger is not None,
            result.claim169.left_thumb is not None,
        ])
        assert has_fingerprint

    def test_decode_claim169_example(self, claim169_example_vector):
        """Test decoding the example from claim_169.md specification."""
        result = claim169.decode_unverified(claim169_example_vector["qr_data"])

        expected = claim169_example_vector["expected_claim169"]
        assert result.claim169.id == expected.get("id")
        assert result.claim169.full_name == expected.get("fullName")
        assert result.claim169.version == expected.get("version")
        assert result.claim169.language == expected.get("language")
        assert result.claim169.date_of_birth == expected.get("dateOfBirth")
        assert result.claim169.gender == expected.get("gender")
        assert result.claim169.address == expected.get("address")
        assert result.claim169.email == expected.get("email")
        assert result.claim169.phone == expected.get("phone")
        assert result.claim169.nationality == expected.get("nationality")
        assert result.claim169.marital_status == expected.get("maritalStatus")
        assert result.claim169.secondary_full_name == expected.get("secondaryFullName")
        assert result.claim169.secondary_language == expected.get("secondaryLanguage")
        assert result.claim169.location_code == expected.get("locationCode")
        assert result.claim169.legal_status == expected.get("legalStatus")
        assert result.claim169.country_of_issuance == expected.get("countryOfIssuance")

        # Check face biometric is present
        assert result.claim169.face is not None
        assert len(result.claim169.face) > 0
        assert result.claim169.face[0].format == expected["face"][0]["format"]
        assert result.claim169.face[0].sub_format == expected["face"][0]["subFormat"]

        # Check CWT metadata matches example
        expected_meta = claim169_example_vector.get("expected_cwt_meta", {})
        assert result.cwt_meta.issuer == expected_meta.get("issuer")
        assert result.cwt_meta.issued_at == expected_meta.get("issuedAt")
        assert result.cwt_meta.expires_at == expected_meta.get("expiresAt")
        assert result.cwt_meta.not_before == expected_meta.get("notBefore")


class TestDecodeInvalidVectors:
    """Tests for rejecting invalid test vectors."""

    def test_reject_bad_base45(self, bad_base45_vector):
        """Test that bad-base45 vector is rejected."""
        with pytest.raises(claim169.Base45DecodeError):
            claim169.decode_unverified(bad_base45_vector["qr_data"])

    def test_reject_bad_zlib(self, bad_zlib_vector):
        """Test that bad-zlib vector is rejected."""
        with pytest.raises(claim169.DecompressError):
            claim169.decode_unverified(bad_zlib_vector["qr_data"])

    def test_reject_not_cose(self, not_cose_vector):
        """Test that not-cose vector is rejected."""
        with pytest.raises(claim169.CoseParseError):
            claim169.decode_unverified(not_cose_vector["qr_data"])

    def test_reject_missing_169(self, missing_169_vector):
        """Test that missing-169 vector is rejected."""
        with pytest.raises(claim169.Claim169NotFoundError):
            claim169.decode_unverified(missing_169_vector["qr_data"])


class TestDecodeEdgeCases:
    """Tests for edge case vectors."""

    def test_handle_unknown_fields(self, unknown_fields_vector):
        """Test that unknown fields are handled gracefully."""
        result = claim169.decode_unverified(unknown_fields_vector["qr_data"])

        expected = unknown_fields_vector["expected_claim169"]
        assert result.claim169.id == expected.get("id")
        assert result.claim169.full_name == expected.get("fullName")

    def test_expired_token_rejected(self, expired_vector):
        """Test that expired tokens are rejected when timestamp validation is enabled."""
        # The library enforces timestamp validation and rejects expired tokens
        with pytest.raises(claim169.Claim169Exception) as exc_info:
            claim169.decode_unverified(expired_vector["qr_data"])

        # Verify the error message indicates expiration
        expected_meta = expired_vector["expected_cwt_meta"]
        assert "expired" in str(exc_info.value).lower()
        assert str(expected_meta["expiresAt"]) in str(exc_info.value)

    def test_not_yet_valid_token_rejected(self, not_yet_valid_vector):
        """Test that not-yet-valid tokens are rejected when timestamp validation is enabled."""
        # The library enforces timestamp validation and rejects tokens with nbf in the future
        with pytest.raises(claim169.Claim169Exception) as exc_info:
            claim169.decode_unverified(not_yet_valid_vector["qr_data"])

        # Verify the error message indicates the token is not yet valid
        expected_meta = not_yet_valid_vector["expected_cwt_meta"]
        assert "not valid" in str(exc_info.value).lower()
        assert str(expected_meta["notBefore"]) in str(exc_info.value)


class TestDecodeOptions:
    """Tests for decode options."""

    def test_skip_biometrics(self, with_face_vector):
        """Test that skip_biometrics option works."""
        result = claim169.decode_unverified(with_face_vector["qr_data"], skip_biometrics=True)

        expected = with_face_vector["expected_claim169"]
        assert result.claim169.id == expected.get("id")
        assert result.claim169.face is None

    def test_max_decompressed_bytes_too_small(self, minimal_vector):
        """Test that max_decompressed_bytes limit is enforced."""
        with pytest.raises(claim169.DecompressError):
            claim169.decode_unverified(minimal_vector["qr_data"], max_decompressed_bytes=10)


class TestDecodeWithEd25519:
    """Tests for Ed25519 signature verification."""

    def test_verify_valid_signature(self, ed25519_signed_vector):
        """Test verifying a valid Ed25519 signature."""
        public_key = bytes.fromhex(
            ed25519_signed_vector["signing_key"]["public_key_hex"]
        )
        result = claim169.decode_with_ed25519(
            ed25519_signed_vector["qr_data"],
            public_key
        )

        expected = ed25519_signed_vector["expected_claim169"]
        assert result.claim169.id == expected.get("id")
        assert result.claim169.full_name == expected.get("fullName")
        assert result.verification_status == "verified"

    def test_reject_wrong_key(self, ed25519_signed_vector):
        """Test that wrong key fails verification."""
        # Use a non-zero key to avoid "weak key" rejection
        wrong_key = bytes([0x01] + [0x00] * 31)
        with pytest.raises((claim169.Claim169Exception, ValueError)):
            claim169.decode_with_ed25519(
                ed25519_signed_vector["qr_data"],
                wrong_key
            )


class TestDecodeEncrypted:
    """Tests for encrypted payload decoding."""

    def test_decrypt_requires_verifier_by_default(self, encrypted_aes256_vector):
        """Test that encrypted decode requires a verifier unless allow_unverified=True."""
        enc_key = bytes(32)  # length-valid key
        with pytest.raises(ValueError):
            claim169.decode_encrypted_aes(encrypted_aes256_vector["qr_data"], enc_key)

    def test_decrypt_signed_aes256(self, encrypted_signed_vector):
        """Test decrypting an AES-256-GCM encrypted payload containing a signed CWT."""
        enc_key = bytes.fromhex(
            encrypted_signed_vector["encryption_key"]["symmetric_key_hex"]
        )
        sign_key = bytes.fromhex(
            encrypted_signed_vector["signing_key"]["public_key_hex"]
        )

        # Provide a verifier using the signing key from the test vector
        from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PublicKey

        public_key = Ed25519PublicKey.from_public_bytes(sign_key)

        def verify_callback(algorithm, key_id, data, signature):
            public_key.verify(bytes(signature), bytes(data))

        result = claim169.decode_encrypted_aes(
            encrypted_signed_vector["qr_data"],
            enc_key,
            verifier=verify_callback
        )

        expected = encrypted_signed_vector["expected_claim169"]
        assert result.claim169.id == expected.get("id")
        assert result.claim169.full_name == expected.get("fullName")
        assert result.verification_status == "verified"

    def test_decrypt_with_wrong_key(self, encrypted_aes256_vector):
        """Test that wrong key fails decryption."""
        wrong_key = bytes(32)  # All zeros
        with pytest.raises(claim169.Claim169Exception):
            claim169.decode_encrypted_aes(
                encrypted_aes256_vector["qr_data"], wrong_key, allow_unverified=True
            )


class TestDecodeWithVerifierHook:
    """Tests for custom verifier hook."""

    def test_custom_verifier_success(self, ed25519_signed_vector):
        """Test that custom verifier can verify signature."""
        from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PublicKey

        public_key_bytes = bytes.fromhex(
            ed25519_signed_vector["signing_key"]["public_key_hex"]
        )
        public_key = Ed25519PublicKey.from_public_bytes(public_key_bytes)

        def verify_callback(algorithm, key_id, data, signature):
            # This will raise InvalidSignature if verification fails
            public_key.verify(bytes(signature), bytes(data))

        result = claim169.decode_with_verifier(
            ed25519_signed_vector["qr_data"],
            verify_callback
        )

        assert result.verification_status == "verified"

    def test_custom_verifier_failure(self, ed25519_signed_vector):
        """Test that custom verifier failure is reported."""
        def verify_callback(algorithm, key_id, data, signature):
            raise ValueError("Verification failed")

        with pytest.raises(claim169.Claim169Exception):
            claim169.decode_with_verifier(
                ed25519_signed_vector["qr_data"],
                verify_callback
            )


class TestDecodeWithDecryptorHook:
    """Tests for custom decryptor hook."""

    def test_custom_decryptor_requires_verifier_by_default(self, encrypted_aes256_vector):
        """Test that decryptor hook requires a verifier unless allow_unverified=True."""
        def decrypt_callback(algorithm, key_id, nonce, aad, ciphertext):
            return b""

        with pytest.raises(ValueError):
            claim169.decode_with_decryptor(
                encrypted_aes256_vector["qr_data"], decrypt_callback
            )

    def test_custom_decryptor_success(self, encrypted_signed_vector):
        """Test that custom decryptor can decrypt payload with nested signature."""
        from cryptography.hazmat.primitives.ciphers.aead import AESGCM
        from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PublicKey

        enc_key = bytes.fromhex(
            encrypted_signed_vector["encryption_key"]["symmetric_key_hex"]
        )
        sign_key = bytes.fromhex(
            encrypted_signed_vector["signing_key"]["public_key_hex"]
        )

        aesgcm = AESGCM(enc_key)
        public_key = Ed25519PublicKey.from_public_bytes(sign_key)

        def decrypt_callback(algorithm, key_id, nonce, aad, ciphertext):
            return aesgcm.decrypt(bytes(nonce), bytes(ciphertext), bytes(aad))

        def verify_callback(algorithm, key_id, data, signature):
            public_key.verify(bytes(signature), bytes(data))

        result = claim169.decode_with_decryptor(
            encrypted_signed_vector["qr_data"],
            decrypt_callback,
            verifier=verify_callback
        )

        expected = encrypted_signed_vector["expected_claim169"]
        assert result.claim169.id == expected.get("id")
        assert result.verification_status == "verified"

    def test_custom_decryptor_failure(self, encrypted_aes256_vector):
        """Test that custom decryptor failure is reported."""
        def decrypt_callback(algorithm, key_id, nonce, aad, ciphertext):
            raise ValueError("Decryption failed")

        with pytest.raises(claim169.Claim169Exception):
            claim169.decode_with_decryptor(
                encrypted_aes256_vector["qr_data"], decrypt_callback, allow_unverified=True
            )


class TestCwtMeta:
    """Tests for CWT metadata methods."""

    def test_cwt_meta_repr(self, minimal_vector):
        """Test CwtMeta string representation."""
        result = claim169.decode_unverified(minimal_vector["qr_data"])
        repr_str = repr(result.cwt_meta)
        assert "CwtMeta" in repr_str


class TestClaim169:
    """Tests for Claim169 methods."""

    def test_claim169_repr(self, minimal_vector):
        """Test Claim169 string representation."""
        result = claim169.decode_unverified(minimal_vector["qr_data"])
        repr_str = repr(result.claim169)
        assert "Claim169" in repr_str

    def test_has_biometrics_true(self, with_face_vector):
        """Test has_biometrics returns True when biometrics present."""
        result = claim169.decode_unverified(with_face_vector["qr_data"])
        assert result.claim169.has_biometrics() is True

    def test_has_biometrics_false(self, minimal_vector):
        """Test has_biometrics returns False when no biometrics."""
        result = claim169.decode_unverified(minimal_vector["qr_data"])
        assert result.claim169.has_biometrics() is False

    def test_to_dict(self, demographics_full_vector):
        """Test converting claim to dictionary."""
        result = claim169.decode_unverified(demographics_full_vector["qr_data"])
        claim_dict = result.claim169.to_dict()
        assert isinstance(claim_dict, dict)
        if result.claim169.id:
            assert claim_dict["id"] == result.claim169.id


class TestDecodeResult:
    """Tests for DecodeResult methods."""

    def test_decode_result_repr(self, minimal_vector):
        """Test DecodeResult string representation."""
        result = claim169.decode_unverified(minimal_vector["qr_data"])
        repr_str = repr(result)
        assert "DecodeResult" in repr_str

    def test_is_verified_false_when_skipped(self, minimal_vector):
        """Test is_verified returns False when verification skipped."""
        result = claim169.decode_unverified(minimal_vector["qr_data"])
        assert result.is_verified() is False

    def test_is_verified_true_when_verified(self, ed25519_signed_vector):
        """Test is_verified returns True when verified."""
        public_key = bytes.fromhex(
            ed25519_signed_vector["signing_key"]["public_key_hex"]
        )
        result = claim169.decode_with_ed25519(
            ed25519_signed_vector["qr_data"],
            public_key
        )
        assert result.is_verified() is True
