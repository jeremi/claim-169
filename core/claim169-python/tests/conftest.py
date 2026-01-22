"""Pytest configuration and fixtures for claim169 tests."""

import json
from pathlib import Path

import pytest


TEST_VECTORS_PATH = Path(__file__).parent.parent.parent.parent / "test-vectors"


@pytest.fixture
def test_vectors_path() -> Path:
    """Return the path to the test vectors directory."""
    return TEST_VECTORS_PATH


def load_test_vector(category: str, name: str) -> dict:
    """Load a test vector JSON file."""
    file_path = TEST_VECTORS_PATH / category / f"{name}.json"
    with open(file_path) as f:
        return json.load(f)


@pytest.fixture
def minimal_vector() -> dict:
    """Load the minimal test vector."""
    return load_test_vector("valid", "minimal")


@pytest.fixture
def demographics_full_vector() -> dict:
    """Load the demographics-full test vector."""
    return load_test_vector("valid", "demographics-full")


@pytest.fixture
def with_face_vector() -> dict:
    """Load the with-face test vector."""
    return load_test_vector("valid", "with-face")


@pytest.fixture
def with_fingerprints_vector() -> dict:
    """Load the with-fingerprints test vector."""
    return load_test_vector("valid", "with-fingerprints")


@pytest.fixture
def ed25519_signed_vector() -> dict:
    """Load the ed25519-signed test vector."""
    return load_test_vector("valid", "ed25519-signed")


@pytest.fixture
def encrypted_aes256_vector() -> dict:
    """Load the encrypted-aes256 test vector."""
    return load_test_vector("valid", "encrypted-aes256")


@pytest.fixture
def encrypted_signed_vector() -> dict:
    """Load the encrypted-signed test vector."""
    return load_test_vector("valid", "encrypted-signed")


@pytest.fixture
def ecdsa_p256_signed_vector() -> dict:
    """Load the ecdsa-p256-signed test vector."""
    return load_test_vector("valid", "ecdsa-p256-signed")


@pytest.fixture
def with_all_biometrics_vector() -> dict:
    """Load the with-all-biometrics test vector."""
    return load_test_vector("valid", "with-all-biometrics")


@pytest.fixture
def bad_base45_vector() -> dict:
    """Load the bad-base45 test vector."""
    return load_test_vector("invalid", "bad-base45")


@pytest.fixture
def bad_zlib_vector() -> dict:
    """Load the bad-zlib test vector."""
    return load_test_vector("invalid", "bad-zlib")


@pytest.fixture
def not_cose_vector() -> dict:
    """Load the not-cose test vector."""
    return load_test_vector("invalid", "not-cose")


@pytest.fixture
def missing_169_vector() -> dict:
    """Load the missing-169 test vector."""
    return load_test_vector("invalid", "missing-169")


@pytest.fixture
def expired_vector() -> dict:
    """Load the expired test vector."""
    return load_test_vector("edge", "expired")


@pytest.fixture
def unknown_fields_vector() -> dict:
    """Load the unknown-fields test vector."""
    return load_test_vector("edge", "unknown-fields")


@pytest.fixture
def not_yet_valid_vector() -> dict:
    """Load the not-yet-valid test vector."""
    return load_test_vector("edge", "not-yet-valid")


@pytest.fixture
def claim169_example_vector() -> dict:
    """Load the claim169-example test vector (from claim_169.md)."""
    return load_test_vector("valid", "claim169-example")
