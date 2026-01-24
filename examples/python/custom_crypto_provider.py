#!/usr/bin/env python3
"""
Custom crypto provider example - integrate with external cryptographic systems.

This example demonstrates how to use custom callbacks for signing, verification,
encryption, and decryption. This pattern enables integration with:

- Hardware Security Modules (HSMs): PKCS#11, AWS CloudHSM, Thales Luna
- Cloud Key Management Services: AWS KMS, Google Cloud KMS, Azure Key Vault
- Smart Cards: PIV cards, Common Access Cards (CAC), OpenPGP cards
- Trusted Platform Modules (TPMs): Hardware-bound keys for attestation
- Remote Signing Services: DocuSign, Adobe Sign, qualified signature providers

The callbacks receive algorithm info and optional key identifiers, allowing you
to route signing/encryption requests to the appropriate external provider.

CALLBACK SIGNATURES:

    Signer (for encoding):
        def signer(algorithm: str, key_id: Optional[bytes], data: bytes) -> bytes
        - algorithm: "EdDSA" or "ES256"
        - key_id: Optional key identifier (e.g., HSM slot ID, KMS key ARN)
        - data: The data to sign (COSE Sig_structure)
        - Returns: Signature bytes (64 bytes for Ed25519/ES256)

    Verifier (for decoding):
        def verifier(algorithm: str, key_id: Optional[bytes], data: bytes, signature: bytes) -> None
        - algorithm: "EdDSA" or "ES256"
        - key_id: Optional key identifier from COSE header
        - data: The signed data
        - signature: The signature to verify
        - Returns: None on success, raises exception on failure

    Encryptor (for encoding):
        def encryptor(algorithm: str, key_id: Optional[bytes], nonce: bytes, aad: bytes, plaintext: bytes) -> bytes
        - algorithm: "A256GCM" or "A128GCM"
        - key_id: Optional key identifier
        - nonce: 12-byte IV/nonce
        - aad: Additional authenticated data (COSE Enc_structure)
        - plaintext: Data to encrypt
        - Returns: Ciphertext with authentication tag appended

    Decryptor (for decoding):
        def decryptor(algorithm: str, key_id: Optional[bytes], nonce: bytes, aad: bytes, ciphertext: bytes) -> bytes
        - algorithm: "A256GCM" or "A128GCM"
        - key_id: Optional key identifier from COSE header
        - nonce: 12-byte IV/nonce
        - aad: Additional authenticated data
        - ciphertext: Data to decrypt (includes auth tag)
        - Returns: Decrypted plaintext
"""

import secrets
import time

import claim169

# For this example, we use the cryptography library to simulate what an
# HSM or KMS would do. In production, replace these implementations with
# calls to your actual crypto provider.
from cryptography.hazmat.primitives.asymmetric.ed25519 import (
    Ed25519PrivateKey,
    Ed25519PublicKey,
)
from cryptography.hazmat.primitives.ciphers.aead import AESGCM


# =============================================================================
# Simulated External Crypto Provider
# =============================================================================
#
# In production, this class would wrap calls to your HSM, KMS, or other
# external service. The keys would be stored in the hardware/service,
# never exposed to the application.


class SimulatedHSM:
    """
    Simulates an external crypto provider (HSM, KMS, etc.).

    In a real implementation, this would:
    - Connect to an HSM via PKCS#11
    - Use AWS KMS SDK to call kms:Sign and kms:Encrypt
    - Connect to a smart card reader
    - Use Azure Key Vault REST API

    The private key material never leaves the HSM/KMS.
    """

    def __init__(self):
        # In production, keys are generated and stored in the HSM
        # These are just for demonstration
        self._signing_key = Ed25519PrivateKey.generate()
        self._encryption_key = secrets.token_bytes(32)

        # Public key can be exported from HSM for verification
        self.public_key = self._signing_key.public_key().public_bytes_raw()

    def sign(self, algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
        """
        Sign data using the HSM.

        In production:
            # AWS KMS example:
            response = kms_client.sign(
                KeyId=key_id.decode() if key_id else 'alias/my-signing-key',
                Message=data,
                MessageType='RAW',
                SigningAlgorithm='ECDSA_SHA_256' if algorithm == 'ES256' else 'EDDSA'
            )
            return response['Signature']

            # PKCS#11 example:
            session.sign(private_key_handle, data, mechanism)
        """
        print(f"    [HSM] Signing with algorithm={algorithm}, key_id={key_id}")

        if algorithm != "EdDSA":
            raise ValueError(f"Unsupported algorithm: {algorithm}")

        signature = self._signing_key.sign(data)
        print(f"    [HSM] Produced {len(signature)}-byte signature")
        return signature

    def verify(
        self, algorithm: str, key_id: bytes | None, data: bytes, signature: bytes
    ) -> None:
        """
        Verify a signature using the HSM.

        In production:
            # AWS KMS example:
            response = kms_client.verify(
                KeyId=key_id.decode() if key_id else 'alias/my-signing-key',
                Message=data,
                Signature=signature,
                SigningAlgorithm='ECDSA_SHA_256' if algorithm == 'ES256' else 'EDDSA'
            )
            if not response['SignatureValid']:
                raise Exception("Signature invalid")

            # Alternatively, verify locally with exported public key
        """
        print(f"    [HSM] Verifying with algorithm={algorithm}, key_id={key_id}")

        if algorithm != "EdDSA":
            raise ValueError(f"Unsupported algorithm: {algorithm}")

        # Using local verification with exported public key
        public_key = Ed25519PublicKey.from_public_bytes(self.public_key)
        public_key.verify(signature, data)  # Raises on failure
        print("    [HSM] Signature verified successfully")

    def encrypt(
        self,
        algorithm: str,
        key_id: bytes | None,
        nonce: bytes,
        aad: bytes,
        plaintext: bytes,
    ) -> bytes:
        """
        Encrypt data using the HSM.

        In production:
            # AWS KMS example (for data key encryption):
            response = kms_client.encrypt(
                KeyId=key_id.decode() if key_id else 'alias/my-encryption-key',
                Plaintext=plaintext,
                EncryptionAlgorithm='AES_256_GCM',
                EncryptionContext={'aad': base64.b64encode(aad)}
            )
            return response['CiphertextBlob']
        """
        print(
            f"    [HSM] Encrypting with algorithm={algorithm}, "
            f"key_id={key_id}, nonce_len={len(nonce)}"
        )

        if algorithm not in ("A256GCM", "A128GCM"):
            raise ValueError(f"Unsupported algorithm: {algorithm}")

        aesgcm = AESGCM(self._encryption_key)
        ciphertext = aesgcm.encrypt(nonce, plaintext, aad)
        print(f"    [HSM] Produced {len(ciphertext)}-byte ciphertext")
        return ciphertext

    def decrypt(
        self,
        algorithm: str,
        key_id: bytes | None,
        nonce: bytes,
        aad: bytes,
        ciphertext: bytes,
    ) -> bytes:
        """
        Decrypt data using the HSM.

        In production:
            # AWS KMS example:
            response = kms_client.decrypt(
                KeyId=key_id.decode() if key_id else 'alias/my-encryption-key',
                CiphertextBlob=ciphertext,
                EncryptionAlgorithm='AES_256_GCM',
                EncryptionContext={'aad': base64.b64encode(aad)}
            )
            return response['Plaintext']
        """
        print(
            f"    [HSM] Decrypting with algorithm={algorithm}, "
            f"key_id={key_id}, ciphertext_len={len(ciphertext)}"
        )

        if algorithm not in ("A256GCM", "A128GCM"):
            raise ValueError(f"Unsupported algorithm: {algorithm}")

        aesgcm = AESGCM(self._encryption_key)
        plaintext = aesgcm.decrypt(nonce, ciphertext, aad)
        print(f"    [HSM] Decrypted to {len(plaintext)} bytes")
        return plaintext


# =============================================================================
# Example 1: Custom Signer Callback
# =============================================================================


def example_custom_signer():
    """Demonstrate encoding with a custom signer callback."""
    print("=" * 70)
    print("Example 1: Custom Signer Callback")
    print("=" * 70)
    print()

    # Initialize our simulated HSM
    hsm = SimulatedHSM()

    # Create identity data
    claim = claim169.Claim169Input(
        id="HSM-SIGNED-001",
        full_name="Alice HSM-Signed",
    )
    claim.email = "alice@hsm-example.com"
    claim.date_of_birth = "1990-05-15"
    claim.gender = 2  # Female

    # Create CWT metadata
    now = int(time.time())
    meta = claim169.CwtMetaInput(
        issuer="https://hsm-issuer.example.com",
        expires_at=now + (365 * 24 * 60 * 60),  # 1 year
    )
    meta.issued_at = now
    meta.subject = "HSM-SIGNED-001"

    # Define the signer callback that delegates to the HSM
    def hsm_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
        return hsm.sign(algorithm, key_id, data)

    # Encode with the custom signer
    print("Encoding credential with custom HSM signer...")
    qr_data = claim169.encode_with_signer(
        claim,
        meta,
        signer=hsm_signer,
        algorithm="EdDSA",
        key_id=b"signing-key-001",  # Optional key identifier
    )

    print(f"\nGenerated QR data ({len(qr_data)} characters):")
    print(f"  {qr_data[:60]}...")

    # Verify by decoding with the known public key
    print("\nVerifying the credential with Ed25519 public key...")
    result = claim169.decode_with_ed25519(
        qr_data, hsm.public_key, validate_timestamps=False
    )

    print(f"\nDecoded result:")
    print(f"  ID: {result.claim169.id}")
    print(f"  Name: {result.claim169.full_name}")
    print(f"  Email: {result.claim169.email}")
    print(f"  Verification: {result.verification_status}")
    print()


# =============================================================================
# Example 2: Custom Verifier Callback
# =============================================================================


def example_custom_verifier():
    """Demonstrate decoding with a custom verifier callback."""
    print("=" * 70)
    print("Example 2: Custom Verifier Callback")
    print("=" * 70)
    print()

    # Initialize our simulated HSM
    hsm = SimulatedHSM()

    # First, create a signed credential
    claim = claim169.Claim169Input(
        id="VERIFIER-TEST-001",
        full_name="Bob Verified",
    )
    meta = claim169.CwtMetaInput(issuer="https://verifier-test.example.com")

    def hsm_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
        return hsm.sign(algorithm, key_id, data)

    print("Creating a signed credential...")
    qr_data = claim169.encode_with_signer(claim, meta, hsm_signer, "EdDSA")

    # Define the verifier callback that delegates to the HSM
    def hsm_verifier(
        algorithm: str, key_id: bytes | None, data: bytes, signature: bytes
    ) -> None:
        """
        Custom verifier that uses the HSM for verification.

        Note: The callback should raise an exception if verification fails.
        Returning without exception indicates success.
        """
        hsm.verify(algorithm, key_id, data, signature)

    # Decode with the custom verifier
    print("\nDecoding with custom HSM verifier...")
    result = claim169.decode_with_verifier(qr_data, hsm_verifier)

    print(f"\nDecoded result:")
    print(f"  ID: {result.claim169.id}")
    print(f"  Name: {result.claim169.full_name}")
    print(f"  Issuer: {result.cwt_meta.issuer}")
    print(f"  Verification: {result.verification_status}")
    print()


# =============================================================================
# Example 3: Custom Encryptor and Decryptor Callbacks
# =============================================================================


def example_custom_encryption():
    """Demonstrate encryption/decryption with custom callbacks."""
    print("=" * 70)
    print("Example 3: Custom Encryptor and Decryptor Callbacks")
    print("=" * 70)
    print()

    # Initialize our simulated HSM
    hsm = SimulatedHSM()

    # Create identity data
    claim = claim169.Claim169Input(
        id="ENCRYPTED-001",
        full_name="Carol Encrypted",
    )
    claim.email = "carol@encrypted.example.com"

    meta = claim169.CwtMetaInput(issuer="https://encrypted-issuer.example.com")

    # Define callbacks
    def hsm_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
        return hsm.sign(algorithm, key_id, data)

    def hsm_encryptor(
        algorithm: str,
        key_id: bytes | None,
        nonce: bytes,
        aad: bytes,
        plaintext: bytes,
    ) -> bytes:
        """
        Custom encryptor that uses the HSM for encryption.

        The nonce is generated by the library and passed to your callback.
        You must return ciphertext with the authentication tag appended.
        """
        return hsm.encrypt(algorithm, key_id, nonce, aad, plaintext)

    # Encode with custom signer and encryptor
    print("Encoding encrypted credential with custom HSM crypto...")
    qr_data = claim169.encode_with_signer_and_encryptor(
        claim,
        meta,
        signer=hsm_signer,
        sign_algorithm="EdDSA",
        encryptor=hsm_encryptor,
        encrypt_algorithm="A256GCM",
    )

    print(f"\nGenerated encrypted QR data ({len(qr_data)} characters):")
    print(f"  {qr_data[:60]}...")

    # Now decode with custom decryptor and verifier
    def hsm_decryptor(
        algorithm: str,
        key_id: bytes | None,
        nonce: bytes,
        aad: bytes,
        ciphertext: bytes,
    ) -> bytes:
        """
        Custom decryptor that uses the HSM for decryption.

        The ciphertext includes the authentication tag.
        You must return the decrypted plaintext.
        """
        return hsm.decrypt(algorithm, key_id, nonce, aad, ciphertext)

    def hsm_verifier(
        algorithm: str, key_id: bytes | None, data: bytes, signature: bytes
    ) -> None:
        hsm.verify(algorithm, key_id, data, signature)

    print("\nDecoding encrypted credential with custom HSM crypto...")
    result = claim169.decode_with_decryptor(
        qr_data,
        decryptor=hsm_decryptor,
        verifier=hsm_verifier,
    )

    print(f"\nDecoded result:")
    print(f"  ID: {result.claim169.id}")
    print(f"  Name: {result.claim169.full_name}")
    print(f"  Email: {result.claim169.email}")
    print(f"  Issuer: {result.cwt_meta.issuer}")
    print(f"  Verification: {result.verification_status}")
    print()


# =============================================================================
# Example 4: Full Roundtrip with All Custom Callbacks
# =============================================================================


def example_full_roundtrip():
    """Demonstrate a complete encode/decode cycle with all custom callbacks."""
    print("=" * 70)
    print("Example 4: Full Roundtrip (Sign + Encrypt -> Decrypt + Verify)")
    print("=" * 70)
    print()

    # Initialize our simulated HSM
    hsm = SimulatedHSM()

    # Create comprehensive identity data
    claim = claim169.Claim169Input(
        id="FULL-ROUNDTRIP-001",
        full_name="Diana Complete",
    )
    claim.first_name = "Diana"
    claim.last_name = "Complete"
    claim.date_of_birth = "1985-12-25"
    claim.gender = 2
    claim.email = "diana@complete.example.com"
    claim.phone = "+1-555-ROUND-TRIP"
    claim.address = "123 Roundtrip Lane, Crypto City, CC 12345"
    claim.nationality = "CC"

    now = int(time.time())
    meta = claim169.CwtMetaInput(
        issuer="https://complete-issuer.example.com",
        expires_at=now + (2 * 365 * 24 * 60 * 60),  # 2 years
    )
    meta.issued_at = now
    meta.subject = "FULL-ROUNDTRIP-001"
    meta.not_before = now

    # All four callbacks
    def signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
        return hsm.sign(algorithm, key_id, data)

    def encryptor(
        algorithm: str,
        key_id: bytes | None,
        nonce: bytes,
        aad: bytes,
        plaintext: bytes,
    ) -> bytes:
        return hsm.encrypt(algorithm, key_id, nonce, aad, plaintext)

    def decryptor(
        algorithm: str,
        key_id: bytes | None,
        nonce: bytes,
        aad: bytes,
        ciphertext: bytes,
    ) -> bytes:
        return hsm.decrypt(algorithm, key_id, nonce, aad, ciphertext)

    def verifier(
        algorithm: str, key_id: bytes | None, data: bytes, signature: bytes
    ) -> None:
        hsm.verify(algorithm, key_id, data, signature)

    # Encode
    print("Step 1: Encoding with custom signer and encryptor...")
    qr_data = claim169.encode_with_signer_and_encryptor(
        claim, meta, signer, "EdDSA", encryptor, "A256GCM"
    )
    print(f"  Generated {len(qr_data)} characters of QR data")

    # Decode
    print("\nStep 2: Decoding with custom decryptor and verifier...")
    result = claim169.decode_with_decryptor(qr_data, decryptor, verifier=verifier)

    # Verify all fields match
    print("\nStep 3: Verifying roundtrip integrity...")
    assert result.claim169.id == "FULL-ROUNDTRIP-001"
    assert result.claim169.full_name == "Diana Complete"
    assert result.claim169.first_name == "Diana"
    assert result.claim169.last_name == "Complete"
    assert result.claim169.date_of_birth == "1985-12-25"
    assert result.claim169.gender == 2
    assert result.claim169.email == "diana@complete.example.com"
    assert result.claim169.phone == "+1-555-ROUND-TRIP"
    assert result.claim169.address == "123 Roundtrip Lane, Crypto City, CC 12345"
    assert result.claim169.nationality == "CC"
    assert result.cwt_meta.issuer == "https://complete-issuer.example.com"
    assert result.verification_status == "verified"
    print("  All fields verified successfully!")

    print("\nFull roundtrip completed successfully!")
    print(f"  Final verification status: {result.verification_status}")
    print()


# =============================================================================
# Example 5: Error Handling in Custom Callbacks
# =============================================================================


def example_error_handling():
    """Demonstrate error handling when custom callbacks fail."""
    print("=" * 70)
    print("Example 5: Error Handling")
    print("=" * 70)
    print()

    # Example: HSM unavailable
    def failing_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
        raise ConnectionError("HSM connection lost")

    claim = claim169.Claim169Input(id="error-test", full_name="Error Test")
    meta = claim169.CwtMetaInput()

    print("Attempting to sign with unavailable HSM...")
    try:
        claim169.encode_with_signer(claim, meta, failing_signer, "EdDSA")
    except claim169.Claim169Exception as e:
        print(f"  Expected error caught: {e}")

    # Example: Signature verification failure
    hsm = SimulatedHSM()

    def signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
        return hsm.sign(algorithm, key_id, data)

    qr_data = claim169.encode_with_signer(claim, meta, signer, "EdDSA")

    def wrong_key_verifier(
        algorithm: str, key_id: bytes | None, data: bytes, signature: bytes
    ) -> None:
        """Verifier with wrong key - will fail."""
        wrong_key = Ed25519PrivateKey.generate().public_key()
        wrong_key.verify(signature, data)

    print("\nAttempting to verify with wrong key...")
    try:
        claim169.decode_with_verifier(qr_data, wrong_key_verifier)
    except claim169.SignatureError as e:
        print(f"  Expected signature error: {e}")

    # Example: Decryption failure
    def wrong_key_decryptor(
        algorithm: str,
        key_id: bytes | None,
        nonce: bytes,
        aad: bytes,
        ciphertext: bytes,
    ) -> bytes:
        """Decryptor with wrong key - will fail."""
        wrong_key = secrets.token_bytes(32)
        return AESGCM(wrong_key).decrypt(nonce, ciphertext, aad)

    def encryptor(
        algorithm: str,
        key_id: bytes | None,
        nonce: bytes,
        aad: bytes,
        plaintext: bytes,
    ) -> bytes:
        return AESGCM(secrets.token_bytes(32)).encrypt(nonce, plaintext, aad)

    qr_encrypted = claim169.encode_with_signer_and_encryptor(
        claim, meta, signer, "EdDSA", encryptor, "A256GCM"
    )

    print("\nAttempting to decrypt with wrong key...")
    try:
        claim169.decode_with_decryptor(
            qr_encrypted, wrong_key_decryptor, allow_unverified=True
        )
    except claim169.DecryptionError as e:
        print(f"  Expected decryption error: {e}")

    print("\nError handling examples completed.")
    print()


# =============================================================================
# Main
# =============================================================================


def main():
    """Run all examples."""
    print()
    print("MOSIP Claim 169 - Custom Crypto Provider Examples")
    print("=" * 70)
    print()
    print("These examples show how to integrate with external crypto providers")
    print("like HSMs, cloud KMS services, smart cards, and TPMs.")
    print()

    example_custom_signer()
    example_custom_verifier()
    example_custom_encryption()
    example_full_roundtrip()
    example_error_handling()

    print("=" * 70)
    print("All examples completed successfully!")
    print("=" * 70)


if __name__ == "__main__":
    main()
