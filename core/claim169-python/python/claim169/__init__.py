"""MOSIP Claim 169 QR Code encoder/decoder library."""

from typing import Optional

import re
from datetime import datetime

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
    PySignatureVerifier,
    PyDecryptor,
    PySigner,
    PyEncryptor,
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


__version__ = version()

__all__ = [
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
    "PySignatureVerifier",
    "PyDecryptor",
    "PySigner",
    "PyEncryptor",
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
