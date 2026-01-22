#!/usr/bin/env python3
"""
Basic decoding example - decode without signature verification.

WARNING: This is for testing/development only. In production, always
verify signatures using decode_with_ed25519() or decode_with_ecdsa_p256().
"""

import claim169

# Sample QR code data (Base45 encoded)
# This is from test-vectors/valid/minimal.json
QR_DATA = "6BFL70+30FFWJWG.FKJ0587B0XKA8FA9DIWENPEJ/5P$DPQE88EB$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL680 B94W5QF60R6KW5/G8HS83P0KI949DP34W3ER68HSLK1"


def main():
    print("=== Basic Decode Example ===\n")

    # Decode without verification (testing only!)
    result = claim169.decode_unverified(QR_DATA)

    # Access identity data
    print("Identity Data:")
    print(f"  ID: {result.claim169.id}")
    print(f"  Full Name: {result.claim169.full_name}")

    # Access CWT metadata
    print("\nCWT Metadata:")
    print(f"  Issuer: {result.cwt_meta.issuer}")
    print(f"  Expires At: {result.cwt_meta.expires_at}")

    # Check verification status
    print(f"\nVerification Status: {result.verification_status}")

    # Convert to dictionary if needed
    claim_dict = result.claim169.to_dict()
    print(f"\nAs dictionary: {claim_dict}")


if __name__ == "__main__":
    main()
