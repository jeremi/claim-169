"""Tests for custom crypto provider functions (signer, encryptor callbacks)."""

import secrets

import pytest

import claim169


class TestEncodeWithSigner:
    """Tests for encode_with_signer function."""

    def test_encode_with_custom_signer_eddsa(self):
        """Test encoding with a custom EdDSA signer callback."""
        # Generate a test Ed25519 keypair using software crypto
        from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey

        private_key = Ed25519PrivateKey.generate()
        public_key_bytes = private_key.public_key().public_bytes_raw()

        def my_signer(algorithm, key_id, data):
            assert algorithm == "EdDSA"
            return private_key.sign(data)

        claim = claim169.Claim169Input(id="custom-signer-test", full_name="Test User")
        meta = claim169.CwtMetaInput(issuer="test")

        # Encode with custom signer
        qr_text = claim169.encode_with_signer(claim, meta, my_signer, "EdDSA")
        assert qr_text

        # Verify we can decode it
        result = claim169.decode_with_ed25519(
            qr_text, public_key_bytes, validate_timestamps=False
        )
        assert result.claim169.id == "custom-signer-test"
        assert result.verification_status == "verified"

    def test_encode_with_custom_signer_es256(self):
        """Test encoding with a custom ES256 signer callback."""
        from cryptography.hazmat.backends import default_backend
        from cryptography.hazmat.primitives.asymmetric import ec
        from cryptography.hazmat.primitives.serialization import Encoding, PublicFormat

        private_key = ec.generate_private_key(ec.SECP256R1(), default_backend())
        public_key_bytes = private_key.public_key().public_bytes(
            Encoding.X962, PublicFormat.CompressedPoint
        )

        def my_signer(algorithm, key_id, data):
            from cryptography.hazmat.primitives import hashes

            assert algorithm == "ES256"
            sig = private_key.sign(data, ec.ECDSA(hashes.SHA256()))
            # Convert DER to raw r||s format (64 bytes)
            from cryptography.hazmat.primitives.asymmetric.utils import (
                decode_dss_signature,
            )

            r, s = decode_dss_signature(sig)
            return r.to_bytes(32, "big") + s.to_bytes(32, "big")

        claim = claim169.Claim169Input(id="es256-signer", full_name="ES256 User")
        meta = claim169.CwtMetaInput(issuer="test")

        qr_text = claim169.encode_with_signer(claim, meta, my_signer, "ES256")
        assert qr_text

        result = claim169.decode_with_ecdsa_p256(
            qr_text, public_key_bytes, validate_timestamps=False
        )
        assert result.claim169.id == "es256-signer"
        assert result.verification_status == "verified"

    def test_encode_with_signer_with_key_id(self):
        """Test that key_id is passed to the signer callback."""
        from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey

        private_key = Ed25519PrivateKey.generate()
        public_key_bytes = private_key.public_key().public_bytes_raw()
        expected_key_id = b"my-key-123"
        received_key_id = None

        def my_signer(algorithm, key_id, data):
            nonlocal received_key_id
            received_key_id = key_id
            return private_key.sign(data)

        claim = claim169.Claim169Input(id="key-id-test", full_name="Test")
        meta = claim169.CwtMetaInput()

        qr_text = claim169.encode_with_signer(
            claim, meta, my_signer, "EdDSA", key_id=expected_key_id
        )
        assert qr_text
        assert received_key_id == expected_key_id

        result = claim169.decode_with_ed25519(
            qr_text, public_key_bytes, validate_timestamps=False
        )
        assert result.claim169.id == "key-id-test"

    def test_encode_with_signer_skip_biometrics(self):
        """Test skip_biometrics parameter with custom signer."""
        from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey

        private_key = Ed25519PrivateKey.generate()

        def my_signer(algorithm, key_id, data):
            return private_key.sign(data)

        claim = claim169.Claim169Input(id="skip-bio", full_name="Test")
        meta = claim169.CwtMetaInput()

        qr_with_bio = claim169.encode_with_signer(
            claim, meta, my_signer, "EdDSA", skip_biometrics=False
        )
        qr_without_bio = claim169.encode_with_signer(
            claim, meta, my_signer, "EdDSA", skip_biometrics=True
        )

        # Both should produce valid output
        assert qr_with_bio
        assert qr_without_bio

    def test_encode_with_signer_invalid_algorithm(self):
        """Test that invalid algorithm is rejected."""

        def my_signer(algorithm, key_id, data):
            return b"fake"

        claim = claim169.Claim169Input(id="test", full_name="Test")
        meta = claim169.CwtMetaInput()

        with pytest.raises(ValueError, match="Unsupported algorithm"):
            claim169.encode_with_signer(claim, meta, my_signer, "RSA256")

    def test_encode_with_signer_callback_error(self):
        """Test that callback errors are propagated."""

        def failing_signer(algorithm, key_id, data):
            raise RuntimeError("Crypto provider unavailable")

        claim = claim169.Claim169Input(id="test", full_name="Test")
        meta = claim169.CwtMetaInput()

        with pytest.raises(claim169.Claim169Exception):
            claim169.encode_with_signer(claim, meta, failing_signer, "EdDSA")


class TestEncodeWithSignerAndEncryptor:
    """Tests for encode_with_signer_and_encryptor function."""

    def test_encode_with_custom_signer_and_encryptor(self):
        """Test encoding with both custom signer and encryptor."""
        from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
        from cryptography.hazmat.primitives.ciphers.aead import AESGCM

        # Generate keys
        sign_private = Ed25519PrivateKey.generate()
        sign_public_bytes = sign_private.public_key().public_bytes_raw()
        aes_key = secrets.token_bytes(32)

        def my_signer(algorithm, key_id, data):
            return sign_private.sign(data)

        def my_encryptor(algorithm, key_id, nonce, aad, plaintext):
            assert algorithm == "A256GCM"
            aesgcm = AESGCM(aes_key)
            return aesgcm.encrypt(nonce, plaintext, aad)

        claim = claim169.Claim169Input(id="custom-both", full_name="Custom Crypto")
        meta = claim169.CwtMetaInput(issuer="test")

        qr_text = claim169.encode_with_signer_and_encryptor(
            claim, meta, my_signer, "EdDSA", my_encryptor, "A256GCM"
        )
        assert qr_text

        # Decode with the keys
        def my_verifier(algorithm, key_id, data, signature):
            sign_private.public_key().verify(signature, data)

        def my_decryptor(algorithm, key_id, nonce, aad, ciphertext):
            aesgcm = AESGCM(aes_key)
            return aesgcm.decrypt(nonce, ciphertext, aad)

        result = claim169.decode_with_decryptor(
            qr_text, my_decryptor, verifier=my_verifier
        )
        assert result.claim169.id == "custom-both"
        assert result.verification_status == "verified"

    def test_encode_with_aes128_encryptor(self):
        """Test encoding with AES-128 custom encryptor."""
        from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
        from cryptography.hazmat.primitives.ciphers.aead import AESGCM

        sign_private = Ed25519PrivateKey.generate()
        aes_key = secrets.token_bytes(16)  # AES-128

        def my_signer(algorithm, key_id, data):
            return sign_private.sign(data)

        def my_encryptor(algorithm, key_id, nonce, aad, plaintext):
            assert algorithm == "A128GCM"
            aesgcm = AESGCM(aes_key)
            return aesgcm.encrypt(nonce, plaintext, aad)

        claim = claim169.Claim169Input(id="aes128-test", full_name="AES128 User")
        meta = claim169.CwtMetaInput()

        qr_text = claim169.encode_with_signer_and_encryptor(
            claim, meta, my_signer, "EdDSA", my_encryptor, "A128GCM"
        )
        assert qr_text

        # Decode
        result = claim169.decode_encrypted_aes128(qr_text, aes_key, allow_unverified=True)
        assert result.claim169.id == "aes128-test"


class TestEncodeWithEncryptor:
    """Tests for encode_with_encryptor function (software signing + custom encryption)."""

    def test_encode_with_custom_encryptor(self):
        """Test encoding with software signing and custom encryptor."""
        from cryptography.hazmat.primitives.ciphers.aead import AESGCM

        sign_key = secrets.token_bytes(32)
        aes_key = secrets.token_bytes(32)

        def my_encryptor(algorithm, key_id, nonce, aad, plaintext):
            aesgcm = AESGCM(aes_key)
            return aesgcm.encrypt(nonce, plaintext, aad)

        claim = claim169.Claim169Input(id="custom-enc", full_name="Test")
        meta = claim169.CwtMetaInput()

        qr_text = claim169.encode_with_encryptor(
            claim, meta, sign_key, my_encryptor, "A256GCM"
        )
        assert qr_text

        # Decode with software decryption
        result = claim169.decode_encrypted_aes(qr_text, aes_key, allow_unverified=True)
        assert result.claim169.id == "custom-enc"


class TestAes128Functions:
    """Tests for AES-128 specific functions."""

    def test_encode_signed_encrypted_aes128(self):
        """Test encode_signed_encrypted_aes128 function."""
        sign_key = secrets.token_bytes(32)
        encrypt_key = secrets.token_bytes(16)  # AES-128

        claim = claim169.Claim169Input(id="aes128-enc", full_name="AES128")
        meta = claim169.CwtMetaInput(issuer="test")

        qr_text = claim169.encode_signed_encrypted_aes128(
            claim, meta, sign_key, encrypt_key
        )
        assert qr_text

        result = claim169.decode_encrypted_aes128(
            qr_text, encrypt_key, allow_unverified=True
        )
        assert result.claim169.id == "aes128-enc"

    def test_decode_encrypted_aes128_key_validation(self):
        """Test that AES-128 decryption validates key length."""
        # Create an encrypted credential with AES-256 (for testing)
        sign_key = secrets.token_bytes(32)
        encrypt_key = secrets.token_bytes(32)  # AES-256

        claim = claim169.Claim169Input(id="test", full_name="Test")
        meta = claim169.CwtMetaInput()

        qr_text = claim169.encode_signed_encrypted(claim, meta, sign_key, encrypt_key)

        # Try to decrypt with wrong key size
        wrong_key = secrets.token_bytes(16)  # AES-128 key
        with pytest.raises(claim169.DecryptionError, match="does not match key type"):
            # This should fail because AES-128 key can't decrypt AES-256 credential
            claim169.decode_encrypted_aes128(qr_text, wrong_key, allow_unverified=True)

    def test_decode_encrypted_aes256_key_validation(self):
        """Test that AES-256 decryption validates key length."""
        sign_key = secrets.token_bytes(32)
        encrypt_key = secrets.token_bytes(16)  # AES-128

        claim = claim169.Claim169Input(id="test", full_name="Test")
        meta = claim169.CwtMetaInput()

        qr_text = claim169.encode_signed_encrypted_aes128(
            claim, meta, sign_key, encrypt_key
        )

        # Try to decrypt with wrong key size
        wrong_key = secrets.token_bytes(32)  # AES-256 key
        with pytest.raises(claim169.DecryptionError, match="does not match key type"):
            # This should fail because AES-256 key can't decrypt AES-128 credential
            claim169.decode_encrypted_aes256(qr_text, wrong_key, allow_unverified=True)


class TestRoundtripCustomCrypto:
    """Roundtrip tests with custom crypto providers."""

    def test_full_roundtrip_custom_signer_and_verifier(self):
        """Test encode with custom signer and decode with custom verifier."""
        from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey

        private_key = Ed25519PrivateKey.generate()

        def my_signer(algorithm, key_id, data):
            return private_key.sign(data)

        def my_verifier(algorithm, key_id, data, signature):
            private_key.public_key().verify(signature, data)

        claim = claim169.Claim169Input(id="roundtrip-test", full_name="Roundtrip User")
        claim.email = "roundtrip@test.com"
        claim.gender = 1
        meta = claim169.CwtMetaInput(issuer="https://issuer.example")

        # Encode
        qr_text = claim169.encode_with_signer(claim, meta, my_signer, "EdDSA")

        # Decode
        result = claim169.decode_with_verifier(qr_text, my_verifier)

        assert result.claim169.id == "roundtrip-test"
        assert result.claim169.full_name == "Roundtrip User"
        assert result.claim169.email == "roundtrip@test.com"
        assert result.claim169.gender == 1
        assert result.cwt_meta.issuer == "https://issuer.example"
        assert result.verification_status == "verified"

    def test_full_roundtrip_custom_crypto_all(self):
        """Test full roundtrip with custom signer, encryptor, verifier, decryptor."""
        from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
        from cryptography.hazmat.primitives.ciphers.aead import AESGCM

        private_key = Ed25519PrivateKey.generate()
        aes_key = secrets.token_bytes(32)

        def my_signer(algorithm, key_id, data):
            return private_key.sign(data)

        def my_encryptor(algorithm, key_id, nonce, aad, plaintext):
            return AESGCM(aes_key).encrypt(nonce, plaintext, aad)

        def my_verifier(algorithm, key_id, data, signature):
            private_key.public_key().verify(signature, data)

        def my_decryptor(algorithm, key_id, nonce, aad, ciphertext):
            return AESGCM(aes_key).decrypt(nonce, ciphertext, aad)

        claim = claim169.Claim169Input(id="full-custom", full_name="Full Custom Test")
        meta = claim169.CwtMetaInput(issuer="https://full-custom.example")

        # Encode with custom signer and encryptor
        qr_text = claim169.encode_with_signer_and_encryptor(
            claim, meta, my_signer, "EdDSA", my_encryptor, "A256GCM"
        )

        # Decode with custom decryptor and verifier
        result = claim169.decode_with_decryptor(
            qr_text, my_decryptor, verifier=my_verifier
        )

        assert result.claim169.id == "full-custom"
        assert result.claim169.full_name == "Full Custom Test"
        assert result.cwt_meta.issuer == "https://full-custom.example"
        assert result.verification_status == "verified"


class TestErrorHandling:
    """Test error handling in custom crypto providers."""

    def test_signer_returns_wrong_type(self):
        """Test that returning wrong type from signer raises error."""

        def bad_signer(algorithm, key_id, data):
            return "not bytes"  # Should return bytes

        claim = claim169.Claim169Input(id="test", full_name="Test")
        meta = claim169.CwtMetaInput()

        with pytest.raises(claim169.Claim169Exception):
            claim169.encode_with_signer(claim, meta, bad_signer, "EdDSA")

    def test_encryptor_returns_wrong_type(self):
        """Test that returning wrong type from encryptor raises error."""
        sign_key = secrets.token_bytes(32)

        def bad_encryptor(algorithm, key_id, nonce, aad, plaintext):
            return 12345  # Should return bytes

        claim = claim169.Claim169Input(id="test", full_name="Test")
        meta = claim169.CwtMetaInput()

        with pytest.raises(claim169.Claim169Exception):
            claim169.encode_with_encryptor(
                claim, meta, sign_key, bad_encryptor, "A256GCM"
            )
