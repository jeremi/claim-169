#!/usr/bin/env python3
"""
Encoding example - create a signed credential.

Demonstrates how to create a MOSIP Claim 169 QR code with identity data.
"""

import time
import claim169

# Example Ed25519 key pair (for demonstration only!)
# In production, use proper key management (HSM/KMS)
PRIVATE_KEY = bytes.fromhex("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60")
PUBLIC_KEY = bytes.fromhex("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")


def main():
    print("=== Encode Credential Example ===\n")

    # Create identity data
    claim = claim169.Claim169Input(
        id="USER-12345",
        full_name="Jane Smith",
    )
    claim.first_name = "Jane"
    claim.last_name = "Smith"
    claim.date_of_birth = "1985-03-15"
    claim.gender = 2  # Female
    claim.email = "jane.smith@example.com"
    claim.phone = "+1-555-123-4567"
    claim.address = "123 Main Street, Anytown, USA"
    claim.nationality = "US"

    # Create CWT metadata
    now = int(time.time())
    expires = now + (365 * 24 * 60 * 60)  # 1 year from now

    meta = claim169.CwtMetaInput(
        issuer="https://identity.example.com",
        expires_at=expires,
    )
    meta.issued_at = now
    meta.subject = "USER-12345"

    # Encode with Ed25519 signature
    qr_data = claim169.encode_with_ed25519(claim, meta, PRIVATE_KEY)

    print(f"Generated QR data ({len(qr_data)} characters):")
    print(f"  {qr_data[:60]}...")
    print()

    # Verify by decoding
    print("Verifying the generated credential...")
    result = claim169.decode_with_ed25519(qr_data, PUBLIC_KEY)

    if result.is_verified():
        print("Credential verified successfully!")
        print(f"\nDecoded data:")
        print(f"  ID: {result.claim169.id}")
        print(f"  Name: {result.claim169.full_name}")
        print(f"  DOB: {result.claim169.date_of_birth}")
        print(f"  Issuer: {result.cwt_meta.issuer}")


def example_ecdsa_p256():
    """Example using ECDSA P-256 instead of Ed25519."""
    print("\n=== ECDSA P-256 Example ===\n")

    # ECDSA P-256 key (32-byte scalar)
    # In production, generate with proper crypto library
    private_key = bytes.fromhex("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721")

    claim = claim169.Claim169Input(id="ECDSA-001", full_name="ECDSA Test")
    meta = claim169.CwtMetaInput(issuer="https://ecdsa.example.com")

    qr_data = claim169.encode_with_ecdsa_p256(claim, meta, private_key)
    print(f"Generated ECDSA-signed credential: {qr_data[:50]}...")


if __name__ == "__main__":
    main()
    example_ecdsa_p256()
