# Encryption

This guide covers encrypting Claim 169 credentials to protect sensitive identity data.

## Overview

Encryption adds a layer of privacy by wrapping the signed COSE structure in a COSE_Encrypt0 envelope. Only parties with the symmetric key can decrypt and read the credential.

```
Signed Credential (COSE_Sign1) → COSE_Encrypt0 → zlib → Base45 → QR
```

## Supported Algorithms

| Algorithm | Key Size | Description |
|-----------|----------|-------------|
| AES-256-GCM | 32 bytes | Recommended for most use cases |
| AES-128-GCM | 16 bytes | Smaller key, still secure |

Both algorithms use Galois/Counter Mode (GCM) which provides authenticated encryption with associated data (AEAD).

## Encrypting Credentials

### AES-256-GCM (Recommended)

=== "Rust"

    ```rust
    use claim169_core::Encoder;

    let signing_key: [u8; 32] = /* Ed25519 private key */;
    let encryption_key: [u8; 32] = /* AES-256 key (32 bytes) */;

    let qr_data = Encoder::new(claim, meta)
        .sign_with_ed25519(&signing_key)?
        .encrypt_with_aes256(&encryption_key)?
        .encode()?;
    ```

=== "Python"

    ```python
    from claim169 import Claim169Input, CwtMetaInput, encode_signed_encrypted

    signing_key = bytes.fromhex("9d61b19d...")      # 32 bytes
    encryption_key = bytes.fromhex("10111213...")   # 32 bytes

    qr_data = encode_signed_encrypted(claim, meta, signing_key, encryption_key)
    ```

=== "TypeScript"

    ```typescript
    import { Encoder, hexToBytes } from 'claim169';

    const signingKey = hexToBytes("9d61b19d...");      // 32 bytes
    const encryptionKey = hexToBytes("10111213...");   // 32 bytes

    const qrData = new Encoder(claim, meta)
      .signWithEd25519(signingKey)
      .encryptWithAes256(encryptionKey)
      .encode();
    ```

### AES-128-GCM

=== "Rust"

    ```rust
    let encryption_key: [u8; 16] = /* AES-128 key (16 bytes) */;

    let qr_data = Encoder::new(claim, meta)
        .sign_with_ed25519(&signing_key)?
        .encrypt_with_aes128(&encryption_key)?
        .encode()?;
    ```

=== "Python"

    ```python
    # Note: the Python bindings currently expose AES-256-GCM encryption via
    # encode_signed_encrypted(). AES-128-GCM encoding is not exposed yet.
    ```

=== "TypeScript"

    ```typescript
    const encryptionKey = hexToBytes("1011121314151617...");  // 16 bytes

    const qrData = new Encoder(claim, meta)
      .signWithEd25519(signingKey)
      .encryptWithAes128(encryptionKey)
      .encode();
    ```

### Custom Encryptor (HSM/KMS)

For production environments where encryption keys are managed externally, use a custom encryptor callback to integrate with Hardware Security Modules (HSM) or cloud Key Management Services (AWS KMS, Google Cloud KMS, Azure Key Vault).

The callback receives:

- `algorithm`: The COSE algorithm name (e.g., `"A256GCM"`, `"A128GCM"`)
- `key_id`: Optional key identifier bytes (for COSE header)
- `plaintext`: The data to encrypt
- `aad`: Additional authenticated data (AAD) for AEAD

The callback must return the ciphertext with the authentication tag appended.

=== "Rust"

    ```rust
    use claim169_core::{Encoder, Encryptor};

    struct HsmEncryptor {
        hsm_client: MyHsmClient,
        key_id: String,
    }

    impl Encryptor for HsmEncryptor {
        fn encrypt(
            &self,
            algorithm: &str,
            _key_id: Option<&[u8]>,
            plaintext: &[u8],
            aad: &[u8],
        ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
            // Call your HSM to encrypt the data
            // Returns ciphertext || auth_tag
            let ciphertext = self.hsm_client.encrypt(&self.key_id, plaintext, aad)?;
            Ok(ciphertext)
        }
    }

    let encryptor = HsmEncryptor {
        hsm_client: my_hsm,
        key_id: "my-encryption-key".to_string(),
    };

    let qr_data = Encoder::new(claim, meta)
        .sign_with_ed25519(&signing_key)?
        .encrypt_with(&encryptor, "A256GCM")?
        .encode()?;
    ```

=== "Python"

    ```python
    from claim169 import encode_with_signer_and_encryptor

    def my_encryptor(algorithm: str, key_id: bytes | None, plaintext: bytes, aad: bytes) -> bytes:
        """
        Custom encryptor callback for HSM/KMS integration.

        Args:
            algorithm: COSE algorithm name ("A256GCM", "A128GCM", etc.)
            key_id: Optional key identifier for COSE header
            plaintext: The data to encrypt
            aad: Additional authenticated data for AEAD

        Returns:
            Ciphertext with authentication tag appended
        """
        # Example: AWS KMS (envelope encryption pattern)
        # data_key = kms_client.generate_data_key(KeyId='alias/my-key')
        # ciphertext = aes_gcm_encrypt(data_key['Plaintext'], plaintext, aad)
        # return ciphertext

        # Example: PKCS#11 HSM
        return my_hsm.encrypt(key_id, plaintext, aad)

    def my_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
        return my_hsm.sign(key_id, data)

    qr_data = encode_with_signer_and_encryptor(
        claim, meta,
        my_signer, "EdDSA",
        my_encryptor, "A256GCM"
    )
    ```

=== "TypeScript"

    ```typescript
    import { Encoder, EncryptorCallback, SignerCallback } from 'claim169';

    const myEncryptor: EncryptorCallback = async (
      algorithm: string,
      keyId: Uint8Array | null,
      plaintext: Uint8Array,
      aad: Uint8Array
    ): Promise<Uint8Array> => {
      // Example: Google Cloud KMS
      // const [encryptResponse] = await kmsClient.encrypt({
      //   name: keyName,
      //   plaintext: plaintext,
      //   additionalAuthenticatedData: aad,
      // });
      // return new Uint8Array(encryptResponse.ciphertext);

      // Your HSM encryption
      return myHsm.encrypt(keyId, plaintext, aad);
    };

    const qrData = new Encoder(claim, meta)
      .signWith(mySigner, "EdDSA")
      .encryptWith(myEncryptor, "A256GCM")
      .encode();
    ```

## Decrypting Credentials

When decrypting, you must specify the decryption method before verification:

=== "Rust"

    ```rust
    use claim169_core::Decoder;

    let encryption_key: [u8; 32] = /* AES-256 key */;
    let public_key: [u8; 32] = /* Ed25519 public key */;

    let result = Decoder::new(qr_data)
        .decrypt_with_aes256(&encryption_key)?  // Decrypt first
        .verify_with_ed25519(&public_key)?       // Then verify
        .decode()?;
    ```

=== "Python"

    ```python
    from claim169 import decode_encrypted_aes

    encryption_key = bytes.fromhex("10111213...")

    # Testing only: decrypt without nested signature verification
    result = decode_encrypted_aes(qr_data, encryption_key, allow_unverified=True)

    # Production: provide a verifier callback (HSM/KMS) to verify the nested COSE_Sign1
    # def my_verifier(algorithm, key_id, data, signature):
    #     hsm.verify(key_id, data, signature)
    # result = decode_encrypted_aes(qr_data, encryption_key, verifier=my_verifier)
    ```

=== "TypeScript"

    ```typescript
    import { Decoder, hexToBytes } from 'claim169';

    const encryptionKey = hexToBytes("10111213...");
    const publicKey = hexToBytes("d75a9801...");

    const result = new Decoder(qrData)
      .decryptWithAes256(encryptionKey)
      .verifyWithEd25519(publicKey)
      .decode();
    ```

### Custom Decryptor (HSM/KMS)

For production environments where decryption keys are managed externally, use a custom decryptor callback to integrate with Hardware Security Modules (HSM) or cloud Key Management Services.

The callback receives:

- `algorithm`: The COSE algorithm name (e.g., `"A256GCM"`, `"A128GCM"`)
- `key_id`: Optional key identifier bytes (from COSE header, if present)
- `ciphertext`: The encrypted data (ciphertext with auth tag)
- `aad`: Additional authenticated data (AAD) for AEAD

The callback must return the decrypted plaintext. Throw an exception if decryption fails (e.g., authentication tag mismatch).

=== "Rust"

    ```rust
    use claim169_core::{Decoder, Decryptor};

    struct HsmDecryptor {
        hsm_client: MyHsmClient,
    }

    impl Decryptor for HsmDecryptor {
        fn decrypt(
            &self,
            algorithm: &str,
            key_id: Option<&[u8]>,
            ciphertext: &[u8],
            aad: &[u8],
        ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
            // Use key_id to locate the correct decryption key in your HSM
            let key_name = key_id
                .map(|id| String::from_utf8_lossy(id).to_string())
                .unwrap_or_else(|| "default-encryption-key".to_string());

            // Call your HSM to decrypt the data
            let plaintext = self.hsm_client.decrypt(&key_name, ciphertext, aad)?;
            Ok(plaintext)
        }
    }

    let decryptor = HsmDecryptor { hsm_client: my_hsm };

    let result = Decoder::new(qr_data)
        .decrypt_with(&decryptor)?
        .verify_with_ed25519(&public_key)?
        .decode()?;
    ```

=== "Python"

    ```python
    from claim169 import decode_with_decryptor_and_verifier

    def my_decryptor(algorithm: str, key_id: bytes | None, ciphertext: bytes, aad: bytes) -> bytes:
        """
        Custom decryptor callback for HSM/KMS integration.

        Args:
            algorithm: COSE algorithm name ("A256GCM", "A128GCM", etc.)
            key_id: Optional key identifier from COSE header
            ciphertext: The encrypted data (ciphertext with auth tag)
            aad: Additional authenticated data for AEAD

        Returns:
            Decrypted plaintext

        Raises:
            Exception: If decryption fails (e.g., auth tag mismatch)
        """
        # Example: AWS KMS
        # response = kms_client.decrypt(
        #     KeyId='alias/my-key',
        #     CiphertextBlob=ciphertext,
        # )
        # return response['Plaintext']

        # Example: PKCS#11 HSM
        return my_hsm.decrypt(key_id, ciphertext, aad)

    def my_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes):
        my_hsm.verify(key_id, data, signature)

    result = decode_with_decryptor_and_verifier(qr_data, my_decryptor, my_verifier)
    ```

=== "TypeScript"

    ```typescript
    import { Decoder, DecryptorCallback, VerifierCallback } from 'claim169';

    const myDecryptor: DecryptorCallback = async (
      algorithm: string,
      keyId: Uint8Array | null,
      ciphertext: Uint8Array,
      aad: Uint8Array
    ): Promise<Uint8Array> => {
      // Example: Google Cloud KMS
      // const [decryptResponse] = await kmsClient.decrypt({
      //   name: keyName,
      //   ciphertext: ciphertext,
      //   additionalAuthenticatedData: aad,
      // });
      // return new Uint8Array(decryptResponse.plaintext);

      // Example: Azure Key Vault
      // const result = await cryptoClient.decrypt("A256GCM", ciphertext);
      // return result.result;

      // Your HSM decryption - throw on failure
      return myHsm.decrypt(keyId, ciphertext, aad);
    };

    const result = new Decoder(qrData)
      .decryptWith(myDecryptor)
      .verifyWith(myVerifier)
      .decode();
    ```

!!! warning "Authentication Tag Verification"
    AES-GCM includes an authentication tag that ensures data integrity. If your HSM returns an error during decryption (e.g., "authentication failed"), propagate this error rather than returning corrupted data.

## Key Management

### Generating Keys

Generate cryptographically secure random keys:

=== "Python"

    ```python
    import secrets

    # Generate AES-256 key
    aes_key = secrets.token_bytes(32)
    print(f"AES-256 key: {aes_key.hex()}")

    # Generate AES-128 key
    aes_key_128 = secrets.token_bytes(16)
    print(f"AES-128 key: {aes_key_128.hex()}")
    ```

=== "TypeScript"

    ```typescript
    // Browser
    const aesKey = crypto.getRandomValues(new Uint8Array(32));

    // Node.js
    import { randomBytes } from 'crypto';
    const aesKey = randomBytes(32);
    ```

### Key Distribution

!!! warning "Security Consideration"
    Symmetric encryption keys must be securely distributed to all parties who need to decrypt credentials. Consider:

    - **Key agreement protocols** (e.g., ECDH) for dynamic key exchange
    - **Key management systems** (KMS) for enterprise deployments
    - **Hardware security modules** (HSM) for high-security environments

## Use Cases

### Privacy-Preserving Credentials

Encrypt credentials so only authorized verifiers can read the contents:

```python
from claim169 import Claim169Input, encode_signed_encrypted, decode_encrypted_aes

# Issuer encrypts with verifier's key
qr_data = encode_signed_encrypted(claim, meta, issuer_signing_key, verifier_encryption_key)

# Only the authorized verifier can decrypt
result = decode_encrypted_aes(qr_data, verifier_encryption_key, verifier=my_verifier)
```

### Selective Disclosure

Create multiple credentials with different encryption keys for different verifiers:

```python
from claim169 import Claim169Input, encode_signed_encrypted

# Full credential for government agencies
full_qr = encode_signed_encrypted(full_claim, meta, signing_key, government_key)

# Minimal credential for age verification
minimal_claim = Claim169Input(
    id=full_claim.id,
    date_of_birth=full_claim.date_of_birth
)
age_qr = encode_signed_encrypted(minimal_claim, meta, signing_key, merchant_key)
```

## Error Handling

Decryption can fail for several reasons:

=== "Python"

    ```python
    import claim169

    def verifier(algorithm, key_id, data, signature):
        # Verify nested signature here (HSM/KMS or software verifier).
        # Raise if verification fails.
        ...

    try:
        result = claim169.decode_encrypted_aes(qr_data, encryption_key, verifier=verifier)
    except claim169.DecryptionError as e:
        print(f"Decryption failed (wrong key?): {e}")
    except claim169.SignatureError as e:
        print(f"Signature verification failed: {e}")
    except claim169.Claim169Exception as e:
        print(f"Claim 169 error: {e}")
    ```

=== "TypeScript"

    ```typescript
    try {
      const result = new Decoder(qrData)
        .decryptWithAes256(encryptionKey)
        .verifyWithEd25519(publicKey)
        .decode();
    } catch (error) {
      if (error.message.includes('decrypt')) {
        console.error('Decryption failed - wrong key?');
      } else if (error.message.includes('signature')) {
        console.error('Signature verification failed');
      } else {
        console.error(`Error: ${error.message}`);
      }
    }
    ```
