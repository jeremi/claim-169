"""MOSIP Claim 169 QR Code encoder/decoder library."""

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
    # Data classes (decode output)
    Biometric,
    CwtMeta,
    Claim169,
    DecodeResult,
    # Data classes (encode input)
    Claim169Input,
    CwtMetaInput,
    # Decode functions
    decode_unverified,
    decode_with_ed25519,
    decode_with_ecdsa_p256,
    decode_with_verifier,
    decode_encrypted_aes,
    decode_with_decryptor,
    # Encode functions
    encode_with_ed25519,
    encode_with_ecdsa_p256,
    encode_signed_encrypted,
    encode_unsigned,
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
) -> DecodeResult:
    """
    Decode a Claim 169 QR code without signature verification (INSECURE).

    This is a convenience alias for `decode_unverified()`.
    Use `decode_with_ed25519()` / `decode_with_ecdsa_p256()` in production.
    """

    return decode_unverified(
        qr_text,
        skip_biometrics=skip_biometrics,
        max_decompressed_bytes=max_decompressed_bytes,
        validate_timestamps=validate_timestamps,
        clock_skew_tolerance_seconds=clock_skew_tolerance_seconds,
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
    # Data classes (decode output)
    "Biometric",
    "CwtMeta",
    "Claim169",
    "DecodeResult",
    # Data classes (encode input)
    "Claim169Input",
    "CwtMetaInput",
    # Decode functions
    "decode_unverified",
    "decode",
    "decode_with_ed25519",
    "decode_with_ecdsa_p256",
    "decode_with_verifier",
    "decode_encrypted_aes",
    "decode_with_decryptor",
    # Encode functions
    "encode_with_ed25519",
    "encode_with_ecdsa_p256",
    "encode_signed_encrypted",
    "encode_unsigned",
    "generate_nonce",
    # Utilities
    "version",
    "__version__",
]
