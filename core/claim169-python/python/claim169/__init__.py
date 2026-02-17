"""MOSIP Claim 169 QR Code encoder/decoder library."""

from enum import IntEnum
from typing import Optional

import re
from datetime import datetime


class Gender(IntEnum):
    """Gender codes per Claim 169 specification (1-indexed)."""

    MALE = 1
    FEMALE = 2
    OTHER = 3


class MaritalStatus(IntEnum):
    """Marital status codes per Claim 169 specification (1-indexed)."""

    UNMARRIED = 1
    MARRIED = 2
    DIVORCED = 3


class PhotoFormat(IntEnum):
    """Photo format codes per Claim 169 specification (1-indexed)."""

    JPEG = 1
    JPEG2000 = 2
    AVIF = 3
    WEBP = 4

from .claim169 import (
    # Exceptions
    Claim169Exception,
    Base45DecodeError,
    DecompressError,
    CoseParseError,
    CwtParseError,
    Claim169NotFoundError,
    SignatureError,
    DecryptionError,
    EncryptionError,
    # Data classes (decode output)
    Biometric,
    CwtMeta,
    Claim169,
    DecodeResult,
    # Data classes (encode input)
    Claim169Input,
    CwtMetaInput,
    # Custom crypto provider wrappers
    SignatureVerifier,
    Decryptor,
    Signer,
    Encryptor,
    # Inspect
    InspectResult,
    inspect,
    # Decode functions
    decode_unverified,
    decode_with_ed25519,
    decode_with_ecdsa_p256,
    decode_with_ed25519_pem,
    decode_with_ecdsa_p256_pem,
    decode_with_verifier,
    decode_encrypted_aes,
    decode_encrypted_aes256,
    decode_encrypted_aes128,
    decode_with_decryptor,
    # Encode functions
    encode_with_ed25519,
    encode_with_ecdsa_p256,
    encode_signed_encrypted,
    encode_signed_encrypted_aes128,
    encode_unsigned,
    encode_with_signer,
    encode_with_signer_and_encryptor,
    encode_with_encryptor,
    generate_nonce,
    # Utilities
    version,
)

# Backward-compatible aliases for renamed crypto wrapper classes
PySignatureVerifier = SignatureVerifier
PyDecryptor = Decryptor
PySigner = Signer
PyEncryptor = Encryptor


def _validate_date_format(date: str | None, field_name: str) -> None:
    """Validate date format is ISO 8601 and represents a valid date.

    Accepts both extended format (YYYY-MM-DD) and basic format (YYYYMMDD).
    """
    if date is None:
        return

    # Accept both ISO 8601 extended (YYYY-MM-DD) and basic (YYYYMMDD) formats
    if re.match(r"^\d{4}-\d{2}-\d{2}$", date):
        date_format = "%Y-%m-%d"
    elif re.match(r"^\d{8}$", date):
        date_format = "%Y%m%d"
    else:
        raise Claim169Exception(
            f"Invalid {field_name} format: '{date}'. Expected YYYY-MM-DD or YYYYMMDD."
        )

    # Validate the date values are actually valid
    try:
        datetime.strptime(date, date_format)
    except ValueError:
        raise Claim169Exception(
            f"Invalid {field_name} value: '{date}' is not a valid calendar date."
        )

def decode(
    qr_text: str,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0,
    verify_with_ed25519: Optional[bytes] = None,
    verify_with_ecdsa_p256: Optional[bytes] = None,
    verify_with_ed25519_pem: Optional[str] = None,
    verify_with_ecdsa_p256_pem: Optional[str] = None,
    allow_unverified: bool = False,
) -> DecodeResult:
    """
    Decode a Claim 169 QR code.

    Security:
    - By default, requires signature verification via one of:
      - `verify_with_ed25519` (32 bytes)
      - `verify_with_ecdsa_p256` (33 or 65 bytes SEC1)
      - `verify_with_ed25519_pem` (PEM string)
      - `verify_with_ecdsa_p256_pem` (PEM string)
    - To explicitly decode without verification (testing only), set
      `allow_unverified=True`.
    """

    # Count how many verification methods are provided
    verifiers = [
        verify_with_ed25519 is not None,
        verify_with_ecdsa_p256 is not None,
        verify_with_ed25519_pem is not None,
        verify_with_ecdsa_p256_pem is not None,
    ]
    if sum(verifiers) > 1:
        raise ValueError("Provide only one verification key")

    result: DecodeResult

    if verify_with_ed25519 is not None:
        result = decode_with_ed25519(
            qr_text,
            verify_with_ed25519,
            skip_biometrics=skip_biometrics,
            max_decompressed_bytes=max_decompressed_bytes,
            validate_timestamps=validate_timestamps,
            clock_skew_tolerance_seconds=clock_skew_tolerance_seconds,
        )
    elif verify_with_ecdsa_p256 is not None:
        result = decode_with_ecdsa_p256(
            qr_text,
            verify_with_ecdsa_p256,
            skip_biometrics=skip_biometrics,
            max_decompressed_bytes=max_decompressed_bytes,
            validate_timestamps=validate_timestamps,
            clock_skew_tolerance_seconds=clock_skew_tolerance_seconds,
        )
    elif verify_with_ed25519_pem is not None:
        result = decode_with_ed25519_pem(
            qr_text,
            verify_with_ed25519_pem,
            skip_biometrics=skip_biometrics,
            max_decompressed_bytes=max_decompressed_bytes,
            validate_timestamps=validate_timestamps,
            clock_skew_tolerance_seconds=clock_skew_tolerance_seconds,
        )
    elif verify_with_ecdsa_p256_pem is not None:
        result = decode_with_ecdsa_p256_pem(
            qr_text,
            verify_with_ecdsa_p256_pem,
            skip_biometrics=skip_biometrics,
            max_decompressed_bytes=max_decompressed_bytes,
            validate_timestamps=validate_timestamps,
            clock_skew_tolerance_seconds=clock_skew_tolerance_seconds,
        )
    elif allow_unverified:
        result = decode_unverified(
            qr_text,
            skip_biometrics=skip_biometrics,
            max_decompressed_bytes=max_decompressed_bytes,
            validate_timestamps=validate_timestamps,
            clock_skew_tolerance_seconds=clock_skew_tolerance_seconds,
        )
    else:
        raise ValueError(
            "decode() requires a verification key (verify_with_ed25519 / verify_with_ecdsa_p256 / "
            "verify_with_ed25519_pem / verify_with_ecdsa_p256_pem) or allow_unverified=True"
        )

    # Validate date format (YYYY-MM-DD)
    _validate_date_format(result.claim169.date_of_birth, "date_of_birth")

    return result


def encode(
    claim169_data: Claim169Input,
    cwt_meta: CwtMetaInput,
    sign_with_ed25519: Optional[bytes] = None,
    sign_with_ecdsa_p256: Optional[bytes] = None,
    encrypt_with_aes256: Optional[bytes] = None,
    encrypt_with_aes128: Optional[bytes] = None,
    allow_unsigned: bool = False,
    skip_biometrics: bool = False,
) -> str:
    """
    Encode a Claim 169 credential into a Base45-encoded QR code string.

    Security:
    - By default, requires a signing key via one of:
      - ``sign_with_ed25519`` (32-byte private key)
      - ``sign_with_ecdsa_p256`` (32-byte private key)
    - Optionally encrypt with:
      - ``encrypt_with_aes256`` (32-byte AES key)
      - ``encrypt_with_aes128`` (16-byte AES key)
    - To encode without signing (testing only), set ``allow_unsigned=True``.

    Args:
        claim169_data: Identity data to encode.
        cwt_meta: CWT metadata (issuer, expiration, etc.).
        sign_with_ed25519: Ed25519 private key bytes (32 bytes).
        sign_with_ecdsa_p256: ECDSA P-256 private key bytes (32 bytes).
        encrypt_with_aes256: AES-256 key bytes (32 bytes) for encryption.
        encrypt_with_aes128: AES-128 key bytes (16 bytes) for encryption.
        allow_unsigned: If True, encode without signing (INSECURE).
        skip_biometrics: If True, exclude biometric data to reduce QR size.

    Returns:
        Base45-encoded string suitable for QR code generation.

    Raises:
        ValueError: If key combination is invalid.
        Claim169Exception: If encoding fails.
    """
    # Validate signing options
    signers = [
        sign_with_ed25519 is not None,
        sign_with_ecdsa_p256 is not None,
    ]
    if sum(signers) > 1:
        raise ValueError("Provide only one signing key")

    encryptions = [
        encrypt_with_aes256 is not None,
        encrypt_with_aes128 is not None,
    ]
    if sum(encryptions) > 1:
        raise ValueError("Provide only one encryption key")

    has_signer = any(signers)
    has_encryption = any(encryptions)

    if not has_signer and not allow_unsigned:
        raise ValueError(
            "encode() requires a signing key (sign_with_ed25519 / sign_with_ecdsa_p256) "
            "or allow_unsigned=True"
        )

    if has_encryption and not has_signer:
        raise ValueError("Encryption requires a signing key")

    # Dispatch to the appropriate encode function
    if allow_unsigned and not has_signer:
        return encode_unsigned(claim169_data, cwt_meta, skip_biometrics=skip_biometrics)

    if sign_with_ed25519 is not None:
        if encrypt_with_aes256 is not None:
            return encode_signed_encrypted(
                claim169_data, cwt_meta, sign_with_ed25519, encrypt_with_aes256,
                skip_biometrics=skip_biometrics,
            )
        elif encrypt_with_aes128 is not None:
            return encode_signed_encrypted_aes128(
                claim169_data, cwt_meta, sign_with_ed25519, encrypt_with_aes128,
                skip_biometrics=skip_biometrics,
            )
        else:
            return encode_with_ed25519(
                claim169_data, cwt_meta, sign_with_ed25519,
                skip_biometrics=skip_biometrics,
            )

    if sign_with_ecdsa_p256 is not None:
        if has_encryption:
            raise ValueError(
                "Built-in encryption is only supported with Ed25519 signing. "
                "Use encode_with_signer_and_encryptor() for ECDSA + encryption."
            )
        return encode_with_ecdsa_p256(
            claim169_data, cwt_meta, sign_with_ecdsa_p256,
            skip_biometrics=skip_biometrics,
        )

    raise ValueError("Invalid encode() parameter combination")


__version__ = version()

__all__ = [
    # Enums
    "Gender",
    "MaritalStatus",
    "PhotoFormat",
    # Exceptions
    "Claim169Exception",
    "Base45DecodeError",
    "DecompressError",
    "CoseParseError",
    "CwtParseError",
    "Claim169NotFoundError",
    "SignatureError",
    "DecryptionError",
    "EncryptionError",
    # Data classes (decode output)
    "Biometric",
    "CwtMeta",
    "Claim169",
    "DecodeResult",
    # Data classes (encode input)
    "Claim169Input",
    "CwtMetaInput",
    # Custom crypto provider wrappers
    "SignatureVerifier",
    "Decryptor",
    "Signer",
    "Encryptor",
    # Backward-compatible aliases
    "PySignatureVerifier",
    "PyDecryptor",
    "PySigner",
    "PyEncryptor",
    # Inspect
    "InspectResult",
    "inspect",
    # Decode functions
    "decode_unverified",
    "decode",
    "decode_with_ed25519",
    "decode_with_ecdsa_p256",
    "decode_with_ed25519_pem",
    "decode_with_ecdsa_p256_pem",
    "decode_with_verifier",
    "decode_encrypted_aes",
    "decode_encrypted_aes256",
    "decode_encrypted_aes128",
    "decode_with_decryptor",
    # Encode functions
    "encode",
    "encode_with_ed25519",
    "encode_with_ecdsa_p256",
    "encode_signed_encrypted",
    "encode_signed_encrypted_aes128",
    "encode_unsigned",
    "encode_with_signer",
    "encode_with_signer_and_encryptor",
    "encode_with_encryptor",
    "generate_nonce",
    # Utilities
    "version",
    "__version__",
]
