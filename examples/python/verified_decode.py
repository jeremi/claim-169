#!/usr/bin/env python3
"""
Verified decoding example - decode with Ed25519 signature verification.

This is the recommended approach for production use.
"""

import claim169

# Sample QR code data with Ed25519 signature
# This is from test-vectors/valid/ed25519-signed.json
QR_DATA = "6BF590B20FFWJWG.FKJ05H7B0XKA8FA9DIWENPEJ/5P$DPQE88EB$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL580 B9%W5*B9C+9%R8646%86HKESED1/DRTC5UA QE$345$CVQEX.DX88WBK0NG8PB4 O/38TL6XDALLKLPQATHO.3ZPJMUAVQFSB1:+B*21V FWMC6SU439YU774475LJ2U5T02$VBSIMLQ3:6J.E1-1STM$4"

# Ed25519 public key (32 bytes) - from test vector
PUBLIC_KEY = bytes.fromhex("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")


def main():
    print("=== Verified Decode Example ===\n")

    try:
        # Decode with Ed25519 signature verification
        result = claim169.decode_with_ed25519(QR_DATA, PUBLIC_KEY)

        # Check if verification succeeded
        if result.is_verified():
            print("Signature verified successfully!")
        else:
            print(f"Verification status: {result.verification_status}")

        # Access identity data
        print("\nIdentity Data:")
        print(f"  ID: {result.claim169.id}")
        print(f"  Full Name: {result.claim169.full_name}")

        # Access CWT metadata
        print("\nCWT Metadata:")
        print(f"  Issuer: {result.cwt_meta.issuer}")
        print(f"  Expires At: {result.cwt_meta.expires_at}")
        print(f"  Issued At: {result.cwt_meta.issued_at}")

        # Check expiration
        if result.cwt_meta.is_expired():
            print("\nWARNING: Credential has expired!")
        else:
            print("\nCredential is valid (not expired)")

    except claim169.SignatureError as e:
        print(f"Signature verification failed: {e}")
    except claim169.Claim169Exception as e:
        print(f"Decoding failed: {e}")


def example_with_wrong_key():
    """Demonstrate what happens with wrong key."""
    print("\n=== Wrong Key Example ===\n")

    # Wrong public key
    wrong_key = bytes(32)  # All zeros

    try:
        result = claim169.decode_with_ed25519(QR_DATA, wrong_key)
        print(f"Verification status: {result.verification_status}")
    except claim169.SignatureError as e:
        print(f"Expected error - signature verification failed: {e}")


if __name__ == "__main__":
    main()
    example_with_wrong_key()
