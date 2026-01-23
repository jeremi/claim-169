"""Type stubs for claim169 Python bindings."""

from typing import Callable, Optional

class Claim169Exception(Exception):
    """Base exception for Claim 169 errors."""
    ...

class Base45DecodeError(Claim169Exception):
    """Base45 decoding failed."""
    ...

class DecompressError(Claim169Exception):
    """Zlib decompression failed."""
    ...

class CoseParseError(Claim169Exception):
    """COSE parsing failed."""
    ...

class CwtParseError(Claim169Exception):
    """CWT parsing failed."""
    ...

class Claim169NotFoundError(Claim169Exception):
    """Claim 169 not found in CWT."""
    ...

class SignatureError(Claim169Exception):
    """Signature verification failed."""
    ...

class DecryptionError(Claim169Exception):
    """Decryption failed."""
    ...

class Biometric:
    """Biometric data extracted from claim 169."""
    data: bytes
    format: Optional[int]
    sub_format: Optional[int]
    issuer: Optional[str]

class CwtMeta:
    """CWT metadata (issuer, subject, timestamps)."""
    issuer: Optional[str]
    subject: Optional[str]
    expires_at: Optional[int]
    not_before: Optional[int]
    issued_at: Optional[int]

    def is_valid_now(self) -> bool:
        """Check if token is currently valid."""
        ...

    def is_expired(self) -> bool:
        """Check if token is expired."""
        ...

class Claim169:
    """Decoded Claim 169 identity data."""
    id: Optional[str]
    version: Optional[str]
    language: Optional[str]
    full_name: Optional[str]
    first_name: Optional[str]
    middle_name: Optional[str]
    last_name: Optional[str]
    date_of_birth: Optional[str]
    gender: Optional[int]
    address: Optional[str]
    email: Optional[str]
    phone: Optional[str]
    nationality: Optional[str]
    marital_status: Optional[int]
    guardian: Optional[str]
    photo: Optional[bytes]
    photo_format: Optional[int]
    best_quality_fingers: Optional[bytes]
    secondary_full_name: Optional[str]
    secondary_language: Optional[str]
    location_code: Optional[str]
    legal_status: Optional[str]
    country_of_issuance: Optional[str]

    # Biometrics
    right_thumb: Optional[list[Biometric]]
    right_pointer_finger: Optional[list[Biometric]]
    right_middle_finger: Optional[list[Biometric]]
    right_ring_finger: Optional[list[Biometric]]
    right_little_finger: Optional[list[Biometric]]
    left_thumb: Optional[list[Biometric]]
    left_pointer_finger: Optional[list[Biometric]]
    left_middle_finger: Optional[list[Biometric]]
    left_ring_finger: Optional[list[Biometric]]
    left_little_finger: Optional[list[Biometric]]
    right_iris: Optional[list[Biometric]]
    left_iris: Optional[list[Biometric]]
    face: Optional[list[Biometric]]
    right_palm: Optional[list[Biometric]]
    left_palm: Optional[list[Biometric]]
    voice: Optional[list[Biometric]]

    def has_biometrics(self) -> bool:
        """Check if claim has biometric data."""
        ...

    def to_dict(self) -> dict:
        """Convert to dictionary."""
        ...

class DecodeResult:
    """Result of decoding a Claim 169 QR code."""
    claim169: Claim169
    cwt_meta: CwtMeta
    verification_status: str

    def is_verified(self) -> bool:
        """Check if signature was verified."""
        ...

# Signature verifier callback type
VerifierCallback = Callable[
    [str, Optional[bytes], bytes, bytes],  # (algorithm, key_id, data, signature)
    None  # Returns None on success, raises exception on failure
]

# Decryptor callback type
DecryptorCallback = Callable[
    [str, Optional[bytes], bytes, bytes, bytes],  # (algorithm, key_id, nonce, aad, ciphertext)
    bytes  # Returns decrypted plaintext
]

def decode_unverified(
    qr_text: str,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0,
) -> DecodeResult:
    """
    Decode a Claim 169 QR code without signature verification (INSECURE).

    Args:
        qr_text: The QR code text content (Base45 encoded)
        skip_biometrics: If True, skip decoding biometric data
        max_decompressed_bytes: Maximum decompressed size
        validate_timestamps: If True, validate exp/nbf timestamps (default: True)
        clock_skew_tolerance_seconds: Tolerance for timestamp validation (default: 0)

    Returns:
        DecodeResult containing the decoded claim and CWT metadata

    Raises:
        Base45DecodeError: If Base45 decoding fails
        DecompressError: If zlib decompression fails
        CoseParseError: If COSE parsing fails
        CwtParseError: If CWT parsing fails
        Claim169NotFoundError: If claim 169 is not present
    """
    ...

def decode(
    qr_text: str,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0,
) -> DecodeResult:
    """
    Convenience alias for `decode_unverified()` (INSECURE).

    Use `decode_with_ed25519()` / `decode_with_ecdsa_p256()` in production.
    """
    ...

def decode_with_ed25519(qr_text: str, public_key: bytes) -> DecodeResult:
    """
    Decode with Ed25519 signature verification.

    Args:
        qr_text: The QR code text content
        public_key: Ed25519 public key bytes (32 bytes)
    """
    ...

def decode_with_ecdsa_p256(qr_text: str, public_key: bytes) -> DecodeResult:
    """
    Decode with ECDSA P-256 signature verification.

    Args:
        qr_text: The QR code text content
        public_key: SEC1 encoded P-256 public key bytes (33 or 65 bytes)
    """
    ...

def decode_with_verifier(qr_text: str, verifier: VerifierCallback) -> DecodeResult:
    """
    Decode with a custom verifier hook (for HSM integration).

    Args:
        qr_text: The QR code text content
        verifier: Callable that verifies signatures

    Example:
        def my_hsm_verify(algorithm, key_id, data, signature):
            hsm.verify(key_id, data, signature)

        result = decode_with_verifier(qr_text, my_hsm_verify)
    """
    ...

def decode_encrypted_aes(
    qr_text: str,
    key: bytes,
    verifier: Optional[VerifierCallback] = None,
) -> DecodeResult:
    """
    Decode an encrypted Claim 169 QR code.

    Args:
        qr_text: The QR code text content
        key: AES-GCM key bytes (16 or 32 bytes)
        verifier: Optional verifier for nested signature verification
    """
    ...

def decode_with_decryptor(
    qr_text: str,
    decryptor: DecryptorCallback,
    verifier: Optional[VerifierCallback] = None,
) -> DecodeResult:
    """
    Decode encrypted with a custom decryptor hook (for HSM integration).

    Args:
        qr_text: The QR code text content
        decryptor: Callable that decrypts ciphertext
        verifier: Optional verifier for nested signature verification

    Example:
        def my_hsm_decrypt(algorithm, key_id, nonce, aad, ciphertext):
            return hsm.decrypt(key_id, nonce, aad, ciphertext)

        result = decode_with_decryptor(qr_text, my_hsm_decrypt)
    """
    ...

def version() -> str:
    """Get the library version."""
    ...

# ============================================================================
# Encoder Input Classes
# ============================================================================

class Claim169Input:
    """Input data for encoding a Claim 169 credential.

    Only `id` and `full_name` can be set in the constructor.
    Other fields must be set as attributes after construction.
    """

    # Settable attributes
    id: Optional[str]
    version: Optional[str]
    language: Optional[str]
    full_name: Optional[str]
    first_name: Optional[str]
    middle_name: Optional[str]
    last_name: Optional[str]
    date_of_birth: Optional[str]
    gender: Optional[int]
    address: Optional[str]
    email: Optional[str]
    phone: Optional[str]
    nationality: Optional[str]
    marital_status: Optional[int]
    guardian: Optional[str]
    photo: Optional[bytes]
    photo_format: Optional[int]
    secondary_full_name: Optional[str]
    secondary_language: Optional[str]
    location_code: Optional[str]
    legal_status: Optional[str]
    country_of_issuance: Optional[str]

    def __init__(
        self,
        id: Optional[str] = None,
        full_name: Optional[str] = None,
    ) -> None: ...

class CwtMetaInput:
    """Input metadata for encoding a CWT token.

    All fields can be set as attributes after construction.
    """

    # Settable attributes
    issuer: Optional[str]
    subject: Optional[str]
    expires_at: Optional[int]
    not_before: Optional[int]
    issued_at: Optional[int]

    def __init__(self) -> None: ...

# ============================================================================
# Encoder Functions
# ============================================================================

def encode_with_ed25519(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    private_key: bytes,
) -> str:
    """
    Encode a Claim 169 credential with Ed25519 signature.

    Args:
        claim169: Identity data to encode
        cwt_meta: CWT metadata (issuer, expiration, etc.)
        private_key: Ed25519 private key bytes (32 bytes)

    Returns:
        Base45-encoded string suitable for QR code generation

    Raises:
        ValueError: If private key is invalid
        Claim169Exception: If encoding fails
    """
    ...

def encode_with_ecdsa_p256(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    private_key: bytes,
) -> str:
    """
    Encode a Claim 169 credential with ECDSA P-256 signature.

    Args:
        claim169: Identity data to encode
        cwt_meta: CWT metadata (issuer, expiration, etc.)
        private_key: ECDSA P-256 private key bytes (32 bytes)

    Returns:
        Base45-encoded string suitable for QR code generation

    Raises:
        ValueError: If private key is invalid
        Claim169Exception: If encoding fails
    """
    ...

def encode_signed_encrypted(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
    sign_key: bytes,
    encrypt_key: bytes,
) -> str:
    """
    Encode a Claim 169 credential with Ed25519 signature and AES-256-GCM encryption.

    Args:
        claim169: Identity data to encode
        cwt_meta: CWT metadata (issuer, expiration, etc.)
        sign_key: Ed25519 private key bytes (32 bytes)
        encrypt_key: AES-256 key bytes (32 bytes)

    Returns:
        Base45-encoded string suitable for QR code generation

    Raises:
        ValueError: If keys are invalid
        Claim169Exception: If encoding fails
    """
    ...

def encode_unsigned(
    claim169: Claim169Input,
    cwt_meta: CwtMetaInput,
) -> str:
    """
    Encode a Claim 169 credential without signature (INSECURE - testing only).

    Args:
        claim169: Identity data to encode
        cwt_meta: CWT metadata (issuer, expiration, etc.)

    Returns:
        Base45-encoded string suitable for QR code generation

    Raises:
        Claim169Exception: If encoding fails
    """
    ...

def generate_nonce() -> bytes:
    """
    Generate a cryptographically secure random 12-byte nonce for AES-GCM encryption.

    Returns:
        12-byte nonce suitable for AES-GCM IV
    """
    ...
