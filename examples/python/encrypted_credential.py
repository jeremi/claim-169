#!/usr/bin/env python3
"""
Encrypted credential example - decode an encrypted and signed credential.

Demonstrates handling credentials that are both signed and encrypted.
"""

import claim169

# Sample encrypted credential
# This would typically come from test-vectors/valid/encrypted-signed.json
# For this example, we'll create one first

# Keys for demonstration (never use in production!)
SIGN_PRIVATE_KEY = bytes.fromhex("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60")
SIGN_PUBLIC_KEY = bytes.fromhex("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
ENCRYPT_KEY = bytes.fromhex("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f")


def create_encrypted_credential():
    """Create an encrypted and signed credential for testing."""
    claim = claim169.Claim169Input(
        id="ENCRYPTED-001",
        full_name="Encrypted Person",
    )
    claim.email = "encrypted@example.com"

    meta = claim169.CwtMetaInput(
        issuer="https://secure.example.com",
        expires_at=1900000000,
    )

    # Create signed and encrypted credential
    qr_data = claim169.encode_signed_encrypted(
        claim, meta, SIGN_PRIVATE_KEY, ENCRYPT_KEY
    )

    return qr_data


def main():
    print("=== Encrypted Credential Example ===\n")

    # Create an encrypted credential for this example
    qr_data = create_encrypted_credential()
    print(f"Encrypted credential ({len(qr_data)} chars): {qr_data[:50]}...")
    print()

    # Method 1: Decrypt and verify with separate verifier
    print("Method 1: Decrypt with AES, verify with Ed25519")
    print("-" * 50)

    try:
        result = claim169.decode_encrypted_aes(
            qr_data,
            ENCRYPT_KEY,
            verifier=lambda alg, key_id, data, sig: verify_ed25519(data, sig)
        )

        print(f"  Verification status: {result.verification_status}")
        print(f"  ID: {result.claim169.id}")
        print(f"  Name: {result.claim169.full_name}")
        print(f"  Issuer: {result.cwt_meta.issuer}")

    except claim169.DecryptionError as e:
        print(f"  Decryption failed: {e}")
    except claim169.SignatureError as e:
        print(f"  Signature verification failed: {e}")


def verify_ed25519(data: bytes, signature: bytes):
    """
    Custom verifier function for Ed25519.

    In production, this might call an HSM or external service.
    """
    from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PublicKey

    public_key = Ed25519PublicKey.from_public_bytes(SIGN_PUBLIC_KEY)
    # This will raise an exception if verification fails
    public_key.verify(signature, data)


def example_skip_verification():
    """Decrypt and skip signature verification (for testing only)."""
    print("\n\nMethod 2: Decrypt and skip verification (testing only!)")
    print("-" * 50)

    qr_data = create_encrypted_credential()

    # WARNING: Never skip verification in production!
    # This is only for testing or when verification is handled separately.
    def skip_verifier(alg, key_id, data, sig):
        pass  # Accept any signature

    try:
        result = claim169.decode_encrypted_aes(
            qr_data, ENCRYPT_KEY, verifier=skip_verifier
        )

        print(f"  Decryption successful")
        print(f"  Verification status: {result.verification_status}")
        print(f"  ID: {result.claim169.id}")

    except claim169.DecryptionError as e:
        print(f"  Decryption failed: {e}")


def example_wrong_key():
    """Demonstrate decryption failure with wrong key."""
    print("\n\nWrong Decryption Key Example")
    print("-" * 50)

    qr_data = create_encrypted_credential()
    wrong_key = bytes(32)  # All zeros

    try:
        result = claim169.decode_encrypted_aes(qr_data, wrong_key, allow_unverified=True)
        print(f"  Unexpected success: {result}")
    except claim169.DecryptionError as e:
        print(f"  Expected error - decryption failed: {e}")


if __name__ == "__main__":
    main()
    example_skip_verification()
    example_wrong_key()
