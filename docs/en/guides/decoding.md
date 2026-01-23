# Decoding & Verification

This guide covers decoding Claim 169 credentials and verifying their authenticity.

## Decoding Pipeline

When decoding, the data flows through these stages:

```
QR Code → Base45 → zlib → COSE → CWT → Claim 169
```

Each stage can fail independently, and the library provides specific error types for each.

## Basic Decoding

### With Verification (Production)

Always verify signatures in production:

=== "Rust"

    ```rust
    use claim169_core::Decoder;

    let result = Decoder::new(qr_data)
        .verify_with_ed25519(&public_key)?
        .decode()?;

    // Access identity data
    println!("ID: {:?}", result.claim169.id);
    println!("Name: {:?}", result.claim169.full_name);

    // Access CWT metadata
    println!("Issuer: {:?}", result.cwt_meta.issuer);
    println!("Expires: {:?}", result.cwt_meta.expires_at);
    ```

=== "Python"

    ```python
    from claim169 import decode_with_ed25519

    result = decode_with_ed25519(qr_data, public_key)

    # Access identity data
    print(f"ID: {result.claim169.id}")
    print(f"Name: {result.claim169.full_name}")

    # Access CWT metadata
    print(f"Issuer: {result.cwt_meta.issuer}")
    print(f"Expires: {result.cwt_meta.expires_at}")
    ```

=== "TypeScript"

    ```typescript
    import { Decoder } from 'claim169';

    const result = new Decoder(qrData)
      .verifyWithEd25519(publicKey)
      .decode();

    // Access identity data
    console.log(`ID: ${result.claim169.id}`);
    console.log(`Name: ${result.claim169.fullName}`);

    // Access CWT metadata
    console.log(`Issuer: ${result.cwtMeta.issuer}`);
    console.log(`Expires: ${result.cwtMeta.expiresAt}`);
    ```

### Without Verification (Testing Only)

!!! danger "Security Warning"
    Never use `allow_unverified()` in production. Unverified credentials may be forged.

=== "Rust"

    ```rust
    let result = Decoder::new(qr_data)
        .allow_unverified()
        .decode()?;
    ```

=== "Python"

    ```python
    from claim169 import decode_unverified

    result = decode_unverified(qr_data)
    ```

=== "TypeScript"

    ```typescript
    const result = new Decoder(qrData)
      .allowUnverified()
      .decode();
    ```

## Verification Methods

### Ed25519

Ed25519 uses 32-byte public keys:

```python
# Public key must be exactly 32 bytes
public_key = bytes.fromhex("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
result = decode_with_ed25519(qr_data, public_key)
```

### ECDSA P-256

ECDSA P-256 accepts compressed (33-byte) or uncompressed (65-byte) public keys:

```python
# Compressed public key (33 bytes, starts with 02 or 03)
compressed_key = bytes.fromhex("02...")

# Uncompressed public key (65 bytes, starts with 04)
uncompressed_key = bytes.fromhex("04...")

result = decode_with_ecdsa_p256(qr_data, compressed_key)
```

### Custom Verifier (HSM/KMS)

For production environments where keys are managed externally, use a custom verifier callback to integrate with Hardware Security Modules (HSM), cloud Key Management Services (AWS KMS, Google Cloud KMS, Azure Key Vault), smart cards, TPMs, or remote verification services.

The callback receives:

- `algorithm`: The COSE algorithm name (e.g., `"EdDSA"`, `"ES256"`)
- `key_id`: Optional key identifier bytes (from COSE header, if present)
- `data`: The signed data (COSE `Sig_structure`)
- `signature`: The signature bytes to verify

The callback must throw/raise an exception if verification fails. A successful return (no exception) indicates valid signature.

=== "Rust"

    ```rust
    use claim169_core::{Decoder, SignatureVerifier};

    struct HsmVerifier {
        hsm_client: MyHsmClient,
    }

    impl SignatureVerifier for HsmVerifier {
        fn verify(
            &self,
            algorithm: &str,
            key_id: Option<&[u8]>,
            data: &[u8],
            signature: &[u8],
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            // Use key_id to locate the correct public key in your HSM
            let key_name = key_id
                .map(|id| String::from_utf8_lossy(id).to_string())
                .unwrap_or_else(|| "default-key".to_string());

            // Call your HSM to verify the signature
            self.hsm_client.verify(&key_name, data, signature)?;
            Ok(())
        }
    }

    let verifier = HsmVerifier { hsm_client: my_hsm };

    let result = Decoder::new(qr_data)
        .verify_with(&verifier)?
        .decode()?;
    ```

=== "Python"

    ```python
    from claim169 import decode_with_verifier

    def my_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes):
        """
        Custom verifier callback for HSM/KMS integration.

        Args:
            algorithm: COSE algorithm name ("EdDSA", "ES256", etc.)
            key_id: Optional key identifier from COSE header
            data: The signed data (COSE Sig_structure)
            signature: The signature bytes to verify

        Raises:
            Exception: If signature verification fails
        """
        # Example: AWS KMS
        # kms_client.verify(
        #     KeyId='alias/my-verification-key',
        #     Message=data,
        #     Signature=signature,
        #     SigningAlgorithm='ECDSA_SHA_256'
        # )

        # Example: PKCS#11 HSM
        # Raises exception on failure, returns None on success
        my_hsm.verify(key_id, data, signature)

    result = decode_with_verifier(qr_data, my_verifier)
    ```

=== "TypeScript"

    ```typescript
    import { Decoder, VerifierCallback } from 'claim169';

    const myVerifier: VerifierCallback = async (
      algorithm: string,
      keyId: Uint8Array | null,
      data: Uint8Array,
      signature: Uint8Array
    ): Promise<void> => {
      // Example: Google Cloud KMS
      // const [verifyResponse] = await kmsClient.asymmetricVerify({
      //   name: keyVersionName,
      //   data: data,
      //   signature: signature,
      // });
      // if (!verifyResponse.success) {
      //   throw new Error('Signature verification failed');
      // }

      // Example: Azure Key Vault
      // const result = await cryptoClient.verify("ES256", data, signature);
      // if (!result.result) {
      //   throw new Error('Signature verification failed');
      // }

      // Your HSM verification - throw on failure
      myHsm.verify(keyId, data, signature);
    };

    const result = new Decoder(qrData)
      .verifyWith(myVerifier)
      .decode();
    ```

!!! tip "Key Lookup"
    Use the `key_id` parameter to look up the correct public key in your key management system. This enables key rotation and multi-issuer scenarios where different credentials may be signed with different keys.

## Handling Encrypted Credentials

Encrypted credentials must be decrypted before verification:

=== "Rust"

    ```rust
    let result = Decoder::new(qr_data)
        .decrypt_with_aes256(&encryption_key)?  // First decrypt
        .verify_with_ed25519(&public_key)?       // Then verify
        .decode()?;
    ```

=== "Python"

    ```python
    from claim169 import decode_encrypted_aes

    # Testing only: decrypt but do not verify the nested signature
    result = decode_encrypted_aes(qr_data, encryption_key, allow_unverified=True)

    # Production: provide a verifier callback (e.g. HSM) to verify the nested COSE_Sign1
    # def my_verifier(algorithm, key_id, data, signature):
    #     hsm.verify(key_id, data, signature)
    # result = decode_encrypted_aes(qr_data, encryption_key, verifier=my_verifier)
    ```

=== "TypeScript"

    ```typescript
    const result = new Decoder(qrData)
      .decryptWithAes256(encryptionKey)  // First decrypt
      .verifyWithEd25519(publicKey)      // Then verify
      .decode();
    ```

!!! note "Order Matters"
    Decryption must be configured before verification. The encrypted payload contains the signed COSE structure.

## Checking Expiration

Credentials may have expiration times set via CWT claims:

=== "Rust"

    ```rust
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    if let Some(exp) = result.cwt_meta.expires_at {
        if exp < now {
            println!("Credential has expired!");
        }
    }

    if let Some(nbf) = result.cwt_meta.not_before {
        if nbf > now {
            println!("Credential is not yet valid!");
        }
    }
    ```

=== "Python"

    ```python
    import time

    now = int(time.time())

    if result.cwt_meta.expires_at and result.cwt_meta.expires_at < now:
        print("Credential has expired!")

    if result.cwt_meta.not_before and result.cwt_meta.not_before > now:
        print("Credential is not yet valid!")
    ```

=== "TypeScript"

    ```typescript
    const now = Math.floor(Date.now() / 1000);

    if (result.cwtMeta.expiresAt && result.cwtMeta.expiresAt < now) {
      console.log("Credential has expired!");
    }

    if (result.cwtMeta.notBefore && result.cwtMeta.notBefore > now) {
      console.log("Credential is not yet valid!");
    }
    ```

## Error Handling

The decoder can fail at various stages. Handle errors appropriately:

=== "Rust"

    ```rust
    use claim169_core::{Claim169Error, Decoder};

    match Decoder::new(qr_data).allow_unverified().decode() {
        Ok(result) => {
            println!("Decoded: {:?}", result.claim169.full_name);
        }
        Err(Claim169Error::Base45Decode(msg)) => {
            println!("Invalid Base45 encoding: {}", msg);
        }
        Err(Claim169Error::DecompressLimitExceeded { max_bytes }) => {
            println!("Decompression limit exceeded: max {} bytes", max_bytes);
        }
        Err(Claim169Error::Decompress(msg)) => {
            println!("Decompression failed: {}", msg);
        }
        Err(Claim169Error::CoseParse(msg)) => {
            println!("Invalid COSE structure: {}", msg);
        }
        Err(Claim169Error::SignatureInvalid(msg)) => {
            println!("Signature verification failed: {}", msg);
        }
        Err(e) => {
            println!("Decoding failed: {}", e);
        }
    }
    ```

=== "Python"

    ```python
    import claim169

    try:
        result = claim169.decode_unverified(qr_data)
    except claim169.Claim169Exception as e:
        print(f"Decoding failed: {e}")
    ```

=== "TypeScript"

    ```typescript
    import { Claim169Error, Decoder } from 'claim169';

    try {
      const result = new Decoder(qrData)
        .allowUnverified()
        .decode();
    } catch (error) {
      if (error instanceof Claim169Error) {
        console.error(`Decoding failed: ${error.message}`);
      } else {
        console.error(`Decoding failed: ${String(error)}`);
      }
    }
    ```

## Security Considerations

### Decompression Limits

The library protects against zip bomb attacks with a default decompression limit of 64KB. This is sufficient for most credentials.

### CBOR Depth Limits

Nested CBOR structures are limited to 128 levels to prevent stack overflow attacks.

### Weak Key Rejection

The library rejects weak cryptographic keys:

- **Ed25519**: Small-order points (including all-zeros)
- **ECDSA**: Identity point (point at infinity)

### Timestamp Validation

When decoding, consider validating:

1. `expires_at` - Credential should not be expired
2. `not_before` - Credential should be within validity period
3. `issued_at` - Should not be in the future (with clock skew tolerance)
