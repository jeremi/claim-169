"""MOSIP Claim 169 QR Code encoder/decoder library."""

from typing import Optional

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

def decode(
    qr_text: str,
    skip_biometrics: bool = False,
    max_decompressed_bytes: int = 65536,
    validate_timestamps: bool = True,
    clock_skew_tolerance_seconds: int = 0,
    verify_with_ed25519: Optional[bytes] = None,
    verify_with_ecdsa_p256: Optional[bytes] = None,
    allow_unverified: bool = False,
) -> DecodeResult:
    """
    Decode a Claim 169 QR code.

    Security:
    - By default, requires signature verification via `verify_with_ed25519` or
      `verify_with_ecdsa_p256`.
    - To explicitly decode without verification (testing only), set
      `allow_unverified=True`.
    """

    if verify_with_ed25519 is not None and verify_with_ecdsa_p256 is not None:
        raise ValueError("Provide only one of verify_with_ed25519 or verify_with_ecdsa_p256")

    if verify_with_ed25519 is not None:
        return decode_with_ed25519(
            qr_text,
            verify_with_ed25519,
            skip_biometrics=skip_biometrics,
            max_decompressed_bytes=max_decompressed_bytes,
            validate_timestamps=validate_timestamps,
            clock_skew_tolerance_seconds=clock_skew_tolerance_seconds,
        )

    if verify_with_ecdsa_p256 is not None:
        return decode_with_ecdsa_p256(
            qr_text,
            verify_with_ecdsa_p256,
            skip_biometrics=skip_biometrics,
            max_decompressed_bytes=max_decompressed_bytes,
            validate_timestamps=validate_timestamps,
            clock_skew_tolerance_seconds=clock_skew_tolerance_seconds,
        )

    if allow_unverified:
        return decode_unverified(
            qr_text,
            skip_biometrics=skip_biometrics,
            max_decompressed_bytes=max_decompressed_bytes,
            validate_timestamps=validate_timestamps,
            clock_skew_tolerance_seconds=clock_skew_tolerance_seconds,
        )

    raise ValueError(
        "decode() requires a verification key (verify_with_ed25519 / verify_with_ecdsa_p256) "
        "or allow_unverified=True"
    )


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
