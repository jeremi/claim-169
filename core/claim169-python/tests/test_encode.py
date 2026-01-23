"""Tests for the claim169 encode functionality."""

import pytest

import claim169


def create_claim(
    id=None,
    full_name=None,
    version=None,
    language=None,
    first_name=None,
    middle_name=None,
    last_name=None,
    date_of_birth=None,
    gender=None,
    address=None,
    email=None,
    phone=None,
    nationality=None,
    marital_status=None,
    guardian=None,
    photo=None,
    photo_format=None,
    secondary_full_name=None,
    secondary_language=None,
    location_code=None,
    legal_status=None,
    country_of_issuance=None,
):
    """Helper to create Claim169Input with all optional fields."""
    claim = claim169.Claim169Input(id=id, full_name=full_name)
    if version is not None:
        claim.version = version
    if language is not None:
        claim.language = language
    if first_name is not None:
        claim.first_name = first_name
    if middle_name is not None:
        claim.middle_name = middle_name
    if last_name is not None:
        claim.last_name = last_name
    if date_of_birth is not None:
        claim.date_of_birth = date_of_birth
    if gender is not None:
        claim.gender = gender
    if address is not None:
        claim.address = address
    if email is not None:
        claim.email = email
    if phone is not None:
        claim.phone = phone
    if nationality is not None:
        claim.nationality = nationality
    if marital_status is not None:
        claim.marital_status = marital_status
    if guardian is not None:
        claim.guardian = guardian
    if photo is not None:
        claim.photo = photo
    if photo_format is not None:
        claim.photo_format = photo_format
    if secondary_full_name is not None:
        claim.secondary_full_name = secondary_full_name
    if secondary_language is not None:
        claim.secondary_language = secondary_language
    if location_code is not None:
        claim.location_code = location_code
    if legal_status is not None:
        claim.legal_status = legal_status
    if country_of_issuance is not None:
        claim.country_of_issuance = country_of_issuance
    return claim


def create_meta(
    issuer=None,
    subject=None,
    expires_at=None,
    not_before=None,
    issued_at=None,
):
    """Helper to create CwtMetaInput with all optional fields."""
    meta = claim169.CwtMetaInput()
    if issuer is not None:
        meta.issuer = issuer
    if subject is not None:
        meta.subject = subject
    if expires_at is not None:
        meta.expires_at = expires_at
    if not_before is not None:
        meta.not_before = not_before
    if issued_at is not None:
        meta.issued_at = issued_at
    return meta


class TestGenerateNonce:
    """Tests for nonce generation."""

    def test_generate_nonce_returns_bytes(self):
        """Test that generate_nonce returns bytes or list."""
        nonce = claim169.generate_nonce()
        # PyO3 returns list, convert if needed
        if isinstance(nonce, list):
            nonce = bytes(nonce)
        assert isinstance(nonce, bytes)

    def test_generate_nonce_length(self):
        """Test that generate_nonce returns 12 bytes (AES-GCM standard)."""
        nonce = claim169.generate_nonce()
        if isinstance(nonce, list):
            nonce = bytes(nonce)
        assert len(nonce) == 12

    def test_generate_nonce_unique(self):
        """Test that consecutive nonces are unique."""
        nonces = []
        for _ in range(100):
            n = claim169.generate_nonce()
            if isinstance(n, list):
                n = bytes(n)
            nonces.append(n)
        # All nonces should be unique
        assert len(set(nonces)) == 100


class TestClaim169Input:
    """Tests for Claim169Input construction."""

    def test_create_minimal(self):
        """Test creating a minimal Claim169Input."""
        claim = claim169.Claim169Input(
            id="TEST-001",
            full_name="Test Person"
        )
        assert claim.id == "TEST-001"
        assert claim.full_name == "Test Person"

    def test_create_with_all_demographics(self):
        """Test creating Claim169Input with all demographic fields."""
        claim = create_claim(
            id="TEST-002",
            version="1.0.0",
            language="en",
            full_name="Test Person",
            first_name="Test",
            middle_name="Middle",
            last_name="Person",
            date_of_birth="1990-01-15",
            gender=1,  # Male
            address="123 Test Street",
            email="test@example.com",
            phone="+1234567890",
            nationality="US",
            marital_status=1,  # Single
            guardian="Guardian Name",
            secondary_full_name="Alternative Name",
            secondary_language="es",
            location_code="US-CA",
            legal_status="citizen",
            country_of_issuance="US"
        )
        assert claim.id == "TEST-002"
        assert claim.full_name == "Test Person"
        assert claim.gender == 1


class TestCwtMetaInput:
    """Tests for CwtMetaInput construction."""

    def test_create_minimal(self):
        """Test creating a minimal CwtMetaInput."""
        meta = claim169.CwtMetaInput()
        # Should not raise

    def test_create_with_all_fields(self):
        """Test creating CwtMetaInput with all fields."""
        meta = create_meta(
            issuer="https://example.org",
            subject="user123",
            expires_at=1800000000,
            not_before=1700000000,
            issued_at=1700000000
        )
        assert meta.issuer == "https://example.org"
        assert meta.expires_at == 1800000000


class TestEncodeWithEd25519:
    """Tests for Ed25519 encoding."""

    def test_encode_minimal(self, ed25519_signed_vector):
        """Test encoding a minimal claim with Ed25519."""
        private_key = bytes.fromhex(
            ed25519_signed_vector["signing_key"]["private_key_hex"]
        )

        claim = create_claim(
            id="ENCODE-TEST-001",
            full_name="Encode Test Person"
        )
        meta = create_meta(
            issuer="https://test.example.org",
            issued_at=1700000000,
            expires_at=1900000000  # Far future to avoid expiration
        )

        qr_data = claim169.encode_with_ed25519(claim, meta, private_key)

        assert isinstance(qr_data, str)
        assert len(qr_data) > 0

    def test_encode_decode_roundtrip(self, ed25519_signed_vector):
        """Test that encode->decode roundtrip preserves data."""
        private_key = bytes.fromhex(
            ed25519_signed_vector["signing_key"]["private_key_hex"]
        )
        public_key = bytes.fromhex(
            ed25519_signed_vector["signing_key"]["public_key_hex"]
        )

        original_id = "ROUNDTRIP-001"
        original_name = "Roundtrip Test Person"
        original_email = "roundtrip@test.org"

        claim = create_claim(
            id=original_id,
            full_name=original_name,
            email=original_email
        )
        meta = create_meta(
            issuer="https://roundtrip.example.org",
            issued_at=1700000000,
            expires_at=1900000000
        )

        qr_data = claim169.encode_with_ed25519(claim, meta, private_key)
        result = claim169.decode_with_ed25519(qr_data, public_key)

        assert result.claim169.id == original_id
        assert result.claim169.full_name == original_name
        assert result.claim169.email == original_email
        assert result.cwt_meta.issuer == "https://roundtrip.example.org"
        assert result.verification_status == "verified"

    def test_invalid_key_length(self):
        """Test that invalid key length raises error."""
        claim = claim169.Claim169Input(id="TEST", full_name="Test")
        meta = claim169.CwtMetaInput()

        with pytest.raises((ValueError, claim169.Claim169Exception)):
            claim169.encode_with_ed25519(claim, meta, bytes(16))  # Too short

    def test_encode_with_all_demographics(self, ed25519_signed_vector):
        """Test encoding with all demographic fields."""
        private_key = bytes.fromhex(
            ed25519_signed_vector["signing_key"]["private_key_hex"]
        )
        public_key = bytes.fromhex(
            ed25519_signed_vector["signing_key"]["public_key_hex"]
        )

        claim = create_claim(
            id="FULL-DEMO-001",
            version="1.0.0",
            language="en",
            full_name="Full Demographics Person",
            first_name="Full",
            middle_name="Demo",
            last_name="Person",
            date_of_birth="1985-06-15",
            gender=2,  # Female
            address="456 Demo Avenue, Test City",
            email="full@demo.org",
            phone="+1987654321",
            nationality="CA",
            marital_status=2,  # Married
            secondary_full_name="Nom Complet",
            secondary_language="fr",
            location_code="CA-QC",
            legal_status="permanent_resident",
            country_of_issuance="CA"
        )
        meta = create_meta(
            issuer="https://demographics.example.org",
            subject="demo-subject",
            issued_at=1700000000,
            expires_at=1900000000,
            not_before=1700000000
        )

        qr_data = claim169.encode_with_ed25519(claim, meta, private_key)
        result = claim169.decode_with_ed25519(qr_data, public_key)

        assert result.claim169.id == "FULL-DEMO-001"
        assert result.claim169.version == "1.0.0"
        assert result.claim169.language == "en"
        assert result.claim169.full_name == "Full Demographics Person"
        assert result.claim169.first_name == "Full"
        assert result.claim169.middle_name == "Demo"
        assert result.claim169.last_name == "Person"
        assert result.claim169.date_of_birth == "1985-06-15"
        assert result.claim169.gender == 2
        assert result.claim169.address == "456 Demo Avenue, Test City"
        assert result.claim169.email == "full@demo.org"
        assert result.claim169.phone == "+1987654321"
        assert result.claim169.nationality == "CA"
        assert result.claim169.marital_status == 2
        assert result.claim169.secondary_full_name == "Nom Complet"
        assert result.claim169.secondary_language == "fr"
        assert result.claim169.location_code == "CA-QC"
        assert result.claim169.legal_status == "permanent_resident"
        assert result.claim169.country_of_issuance == "CA"
        assert result.verification_status == "verified"


class TestEncodeWithEcdsaP256:
    """Tests for ECDSA P-256 encoding."""

    def test_encode_decode_roundtrip(self):
        """Test ECDSA P-256 encode->decode roundtrip."""
        # Generate a test key pair using cryptography library
        from cryptography.hazmat.primitives.asymmetric import ec
        from cryptography.hazmat.backends import default_backend

        private_key_obj = ec.generate_private_key(ec.SECP256R1(), default_backend())
        private_key_bytes = private_key_obj.private_numbers().private_value.to_bytes(32, 'big')
        public_key_obj = private_key_obj.public_key()
        public_numbers = public_key_obj.public_numbers()
        # Uncompressed SEC1 format: 0x04 || x || y
        public_key_bytes = b'\x04' + public_numbers.x.to_bytes(32, 'big') + public_numbers.y.to_bytes(32, 'big')

        original_id = "ECDSA-ROUNDTRIP-001"
        original_name = "ECDSA Roundtrip Person"

        claim = create_claim(
            id=original_id,
            full_name=original_name
        )
        meta = create_meta(
            issuer="https://ecdsa.example.org",
            issued_at=1700000000,
            expires_at=1900000000
        )

        qr_data = claim169.encode_with_ecdsa_p256(claim, meta, private_key_bytes)
        result = claim169.decode_with_ecdsa_p256(qr_data, public_key_bytes)

        assert result.claim169.id == original_id
        assert result.claim169.full_name == original_name
        assert result.verification_status == "verified"

    def test_invalid_key_length(self):
        """Test that invalid key length raises error."""
        claim = claim169.Claim169Input(id="TEST", full_name="Test")
        meta = claim169.CwtMetaInput()

        with pytest.raises((ValueError, claim169.Claim169Exception)):
            claim169.encode_with_ecdsa_p256(claim, meta, bytes(16))  # Too short


class TestEncodeSignedEncrypted:
    """Tests for signed and encrypted encoding."""

    def test_encode_decode_roundtrip(self, encrypted_signed_vector):
        """Test signed+encrypted encode->decode roundtrip."""
        sign_private_key = bytes.fromhex(
            encrypted_signed_vector["signing_key"]["private_key_hex"]
        )
        sign_public_key = bytes.fromhex(
            encrypted_signed_vector["signing_key"]["public_key_hex"]
        )
        encrypt_key = bytes.fromhex(
            encrypted_signed_vector["encryption_key"]["symmetric_key_hex"]
        )

        original_id = "ENC-SIGN-ROUNDTRIP-001"
        original_name = "Encrypted Signed Roundtrip"

        claim = create_claim(
            id=original_id,
            full_name=original_name
        )
        meta = create_meta(
            issuer="https://encrypted.example.org",
            issued_at=1700000000,
            expires_at=1900000000
        )

        qr_data = claim169.encode_signed_encrypted(
            claim, meta, sign_private_key, encrypt_key
        )

        from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PublicKey

        public_key = Ed25519PublicKey.from_public_bytes(sign_public_key)

        def verify_callback(algorithm, key_id, data, signature):
            public_key.verify(bytes(signature), bytes(data))

        result = claim169.decode_encrypted_aes(
            qr_data,
            encrypt_key,
            verifier=verify_callback
        )

        assert result.claim169.id == original_id
        assert result.claim169.full_name == original_name
        assert result.verification_status == "verified"

    def test_decryption_with_wrong_key_fails(self, encrypted_signed_vector):
        """Test that decryption with wrong key fails."""
        sign_private_key = bytes.fromhex(
            encrypted_signed_vector["signing_key"]["private_key_hex"]
        )
        encrypt_key = bytes.fromhex(
            encrypted_signed_vector["encryption_key"]["symmetric_key_hex"]
        )
        wrong_decrypt_key = bytes(32)  # All zeros

        claim = claim169.Claim169Input(id="TEST", full_name="Test")
        meta = claim169.CwtMetaInput()

        qr_data = claim169.encode_signed_encrypted(
            claim, meta, sign_private_key, encrypt_key
        )

        with pytest.raises(claim169.Claim169Exception):
            claim169.decode_encrypted_aes(qr_data, wrong_decrypt_key, allow_unverified=True)

    def test_invalid_sign_key_length(self, encrypted_signed_vector):
        """Test that invalid signing key length raises error."""
        encrypt_key = bytes.fromhex(
            encrypted_signed_vector["encryption_key"]["symmetric_key_hex"]
        )
        claim = claim169.Claim169Input(id="TEST", full_name="Test")
        meta = claim169.CwtMetaInput()

        with pytest.raises((ValueError, claim169.Claim169Exception)):
            claim169.encode_signed_encrypted(claim, meta, bytes(16), encrypt_key)

    def test_invalid_encrypt_key_length(self, encrypted_signed_vector):
        """Test that invalid encryption key length raises error."""
        sign_private_key = bytes.fromhex(
            encrypted_signed_vector["signing_key"]["private_key_hex"]
        )
        claim = claim169.Claim169Input(id="TEST", full_name="Test")
        meta = claim169.CwtMetaInput()

        with pytest.raises((ValueError, claim169.Claim169Exception)):
            claim169.encode_signed_encrypted(claim, meta, sign_private_key, bytes(16))


class TestEncodeUnsigned:
    """Tests for unsigned encoding (testing only)."""

    def test_encode_decode_roundtrip(self):
        """Test unsigned encode->decode roundtrip."""
        original_id = "UNSIGNED-001"
        original_name = "Unsigned Test Person"

        claim = create_claim(
            id=original_id,
            full_name=original_name
        )
        meta = create_meta(
            issuer="https://unsigned.example.org",
            issued_at=1700000000,
            expires_at=1900000000
        )

        qr_data = claim169.encode_unsigned(claim, meta)
        result = claim169.decode_unverified(qr_data)

        assert result.claim169.id == original_id
        assert result.claim169.full_name == original_name
        assert result.verification_status == "skipped"

    def test_produces_smaller_output(self, ed25519_signed_vector):
        """Test that unsigned encoding produces smaller output than signed."""
        private_key = bytes.fromhex(
            ed25519_signed_vector["signing_key"]["private_key_hex"]
        )

        claim = create_claim(
            id="SIZE-TEST",
            full_name="Size Test Person"
        )
        meta = create_meta(
            issuer="https://size.example.org",
            expires_at=1900000000
        )

        signed_qr = claim169.encode_with_ed25519(claim, meta, private_key)
        unsigned_qr = claim169.encode_unsigned(claim, meta)

        # Unsigned should be smaller (no signature overhead)
        assert len(unsigned_qr) < len(signed_qr)


class TestEncoderEdgeCases:
    """Edge case tests for encoder."""

    def test_empty_optional_fields(self, ed25519_signed_vector):
        """Test encoding with None for all optional fields."""
        private_key = bytes.fromhex(
            ed25519_signed_vector["signing_key"]["private_key_hex"]
        )
        public_key = bytes.fromhex(
            ed25519_signed_vector["signing_key"]["public_key_hex"]
        )

        # Only required fields
        claim = claim169.Claim169Input()
        meta = create_meta(expires_at=1900000000)

        qr_data = claim169.encode_with_ed25519(claim, meta, private_key)
        result = claim169.decode_with_ed25519(qr_data, public_key)

        # All optional fields should be None
        assert result.claim169.id is None
        assert result.claim169.full_name is None
        assert result.claim169.email is None

    def test_unicode_in_fields(self, ed25519_signed_vector):
        """Test encoding with Unicode characters."""
        private_key = bytes.fromhex(
            ed25519_signed_vector["signing_key"]["private_key_hex"]
        )
        public_key = bytes.fromhex(
            ed25519_signed_vector["signing_key"]["public_key_hex"]
        )

        unicode_name = "日本語テスト"
        unicode_address = "东京都渋谷区"

        claim = create_claim(
            id="UNICODE-001",
            full_name=unicode_name,
            address=unicode_address
        )
        meta = create_meta(expires_at=1900000000)

        qr_data = claim169.encode_with_ed25519(claim, meta, private_key)
        result = claim169.decode_with_ed25519(qr_data, public_key)

        assert result.claim169.full_name == unicode_name
        assert result.claim169.address == unicode_address

    def test_large_field_values(self, ed25519_signed_vector):
        """Test encoding with large field values."""
        private_key = bytes.fromhex(
            ed25519_signed_vector["signing_key"]["private_key_hex"]
        )
        public_key = bytes.fromhex(
            ed25519_signed_vector["signing_key"]["public_key_hex"]
        )

        # Long but reasonable values
        long_name = "A" * 200
        long_address = "B" * 500

        claim = create_claim(
            id="LARGE-001",
            full_name=long_name,
            address=long_address
        )
        meta = create_meta(expires_at=1900000000)

        qr_data = claim169.encode_with_ed25519(claim, meta, private_key)
        result = claim169.decode_with_ed25519(qr_data, public_key)

        assert result.claim169.full_name == long_name
        assert result.claim169.address == long_address

    def test_special_characters_in_fields(self, ed25519_signed_vector):
        """Test encoding with special characters."""
        private_key = bytes.fromhex(
            ed25519_signed_vector["signing_key"]["private_key_hex"]
        )
        public_key = bytes.fromhex(
            ed25519_signed_vector["signing_key"]["public_key_hex"]
        )

        special_email = "test+special@example.org"
        special_phone = "+1 (234) 567-8900"

        claim = create_claim(
            id="SPECIAL-001",
            full_name="O'Brien-Smith",
            email=special_email,
            phone=special_phone
        )
        meta = create_meta(expires_at=1900000000)

        qr_data = claim169.encode_with_ed25519(claim, meta, private_key)
        result = claim169.decode_with_ed25519(qr_data, public_key)

        assert result.claim169.full_name == "O'Brien-Smith"
        assert result.claim169.email == special_email
        assert result.claim169.phone == special_phone
