# Key Material & Formats

This page explains what key material the library expects (raw bytes vs PEM), and how it maps to MOSIP Claim 169 operations.

## What keys are used?

- **Signing (authenticity)**: Ed25519 (COSE `EdDSA`) or ECDSA P-256 (COSE `ES256`)
- **Encryption (privacy, optional)**: AES-GCM (COSE `A256GCM` or `A128GCM`)

!!! warning "Production key management"
    Treat signing keys and encryption keys as high-value secrets. For production use, keep them in an HSM/KMS and use the library’s “custom crypto” hooks (where available) rather than loading raw private key bytes into application memory.

## Key formats by algorithm

### Ed25519

- **Public key**: 32 bytes
- **Private key**: 32 bytes (seed)

In the Rust crate (default `software-crypto` feature), the decoder also supports verifying keys in **PEM/SPKI** form:

```rust
use claim169_core::Decoder;

let result = Decoder::new(qr_text)
    .verify_with_ed25519_pem(ed25519_public_key_pem)?
    .decode()?;
```

### ECDSA P-256 (ES256)

- **Public key**: SEC1-encoded point, either:
  - **33 bytes** (compressed, starts with `0x02` or `0x03`), or
  - **65 bytes** (uncompressed, starts with `0x04`)
- **Private key**: 32-byte scalar

Rust also supports verifying keys in **PEM/SPKI** form:

```rust
use claim169_core::Decoder;

let result = Decoder::new(qr_text)
    .verify_with_ecdsa_p256_pem(p256_public_key_pem)?
    .decode()?;
```

### AES-GCM (A256GCM / A128GCM)

- **AES-256-GCM key**: 32 bytes
- **AES-128-GCM key**: 16 bytes
- **Nonce/IV**: 12 bytes (random per encryption)

In normal usage you do **not** need to supply a nonce: the encoder generates a random nonce automatically.

!!! danger "Nonce reuse breaks security"
    Never reuse an AES-GCM nonce with the same key. Only use explicit-nonce APIs for testing.

## Generating development keys (Rust)

If you are using the default `software-crypto` feature, you can generate temporary keys for local testing:

```rust
use claim169_core::{Ed25519Signer, EcdsaP256Signer};

let ed_signer = Ed25519Signer::generate();
let ed_public_key: [u8; 32] = ed_signer.public_key_bytes();

let p256_signer = EcdsaP256Signer::generate();
let p256_public_key_uncompressed: Vec<u8> = p256_signer.public_key_uncompressed(); // 65 bytes
```

## Generating AES keys (Python / TypeScript)

=== "Python"

    ```python
    import secrets

    aes256_key = secrets.token_bytes(32)
    aes128_key = secrets.token_bytes(16)
    ```

=== "TypeScript"

    ```ts
    // Browser
    const aes256Key = crypto.getRandomValues(new Uint8Array(32));

    // Node.js
    import { randomBytes } from "crypto";
    const aes256KeyNode = randomBytes(32);
    ```

## HSM and KMS Integration

For production deployments, cryptographic keys should be stored in Hardware Security Modules (HSM) or cloud Key Management Services (KMS) rather than loaded as raw bytes into application memory.

### Custom Crypto Providers

The library supports custom crypto provider callbacks for all cryptographic operations:

| Operation | Callback Type | Use Case |
|-----------|---------------|----------|
| Signing | `Signer` / `SignerCallback` | Credential issuance with HSM-protected keys |
| Verification | `SignatureVerifier` / `VerifierCallback` | Credential verification with managed public keys |
| Encryption | `Encryptor` / `EncryptorCallback` | Privacy protection with HSM-wrapped keys |
| Decryption | `Decryptor` / `DecryptorCallback` | Credential reading with HSM-protected keys |

### Callback Interface

All callbacks receive:

- **`algorithm`**: COSE algorithm name (`"EdDSA"`, `"ES256"`, `"A256GCM"`, etc.)
- **`key_id`**: Optional key identifier bytes from the COSE header

Additional parameters depend on the operation:

| Operation | Additional Parameters | Return Value |
|-----------|----------------------|--------------|
| Sign | `data` (bytes to sign) | Signature bytes |
| Verify | `data`, `signature` | None (throw on failure) |
| Encrypt | `plaintext`, `aad` | Ciphertext with auth tag |
| Decrypt | `ciphertext`, `aad` | Plaintext (throw on failure) |

### Supported KMS Providers

The custom crypto interface works with any key management system:

| Provider | Signing | Encryption | Notes |
|----------|---------|------------|-------|
| AWS KMS | ES256, EdDSA | AES-GCM | Use `kms:Sign`, `kms:Verify`, `kms:Encrypt`, `kms:Decrypt` |
| Google Cloud KMS | ES256, EdDSA | AES-GCM | Asymmetric signing + symmetric encryption |
| Azure Key Vault | ES256, EdDSA | AES-GCM | Use Cryptography client |
| HashiCorp Vault | ES256, EdDSA | AES-GCM | Transit secrets engine |
| PKCS#11 HSM | All algorithms | All algorithms | Hardware-backed keys |
| TPM 2.0 | ES256, EdDSA | AES-GCM | Platform-bound keys |
| Smart Cards | ES256 | N/A | PIV/CAC cards |

### Example: AWS KMS

=== "Python"

    ```python
    import boto3
    from claim169 import encode_with_signer, decode_with_verifier

    kms = boto3.client('kms')
    KEY_ID = 'alias/claim169-signing-key'

    def aws_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
        response = kms.sign(
            KeyId=KEY_ID,
            Message=data,
            MessageType='RAW',
            SigningAlgorithm='ECDSA_SHA_256'  # For ES256
        )
        return response['Signature']

    def aws_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes):
        kms.verify(
            KeyId=KEY_ID,
            Message=data,
            MessageType='RAW',
            Signature=signature,
            SigningAlgorithm='ECDSA_SHA_256'
        )
        # Raises InvalidSignatureException on failure

    # Encode with AWS KMS
    qr_data = encode_with_signer(claim, meta, aws_signer, "ES256")

    # Decode with AWS KMS
    result = decode_with_verifier(qr_data, aws_verifier)
    ```

=== "TypeScript"

    ```typescript
    import { KMSClient, SignCommand, VerifyCommand } from '@aws-sdk/client-kms';
    import { Encoder, Decoder, SignerCallback, VerifierCallback } from 'claim169';

    const kms = new KMSClient({ region: 'us-east-1' });
    const KEY_ID = 'alias/claim169-signing-key';

    const awsSigner: SignerCallback = async (algorithm, keyId, data) => {
      const command = new SignCommand({
        KeyId: KEY_ID,
        Message: data,
        MessageType: 'RAW',
        SigningAlgorithm: 'ECDSA_SHA_256',
      });
      const response = await kms.send(command);
      return new Uint8Array(response.Signature!);
    };

    const awsVerifier: VerifierCallback = async (algorithm, keyId, data, signature) => {
      const command = new VerifyCommand({
        KeyId: KEY_ID,
        Message: data,
        MessageType: 'RAW',
        Signature: signature,
        SigningAlgorithm: 'ECDSA_SHA_256',
      });
      const response = await kms.send(command);
      if (!response.SignatureValid) {
        throw new Error('Signature verification failed');
      }
    };

    // Encode with AWS KMS
    const qrData = new Encoder(claim, meta)
      .signWith(awsSigner, "ES256")
      .encode();

    // Decode with AWS KMS
    const result = new Decoder(qrData)
      .verifyWith(awsVerifier)
      .decode();
    ```

### Example: Google Cloud KMS

=== "Python"

    ```python
    from google.cloud import kms

    client = kms.KeyManagementServiceClient()
    KEY_VERSION = 'projects/my-project/locations/global/keyRings/my-ring/cryptoKeys/my-key/cryptoKeyVersions/1'

    def gcp_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
        response = client.asymmetric_sign(
            request={'name': KEY_VERSION, 'data': data}
        )
        return response.signature

    def gcp_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes):
        response = client.asymmetric_verify(
            request={'name': KEY_VERSION, 'data': data, 'signature': signature}
        )
        if not response.success:
            raise ValueError('Signature verification failed')
    ```

### Example: Azure Key Vault

=== "Python"

    ```python
    from azure.identity import DefaultAzureCredential
    from azure.keyvault.keys.crypto import CryptographyClient, SignatureAlgorithm

    credential = DefaultAzureCredential()
    crypto_client = CryptographyClient(
        'https://my-vault.vault.azure.net/keys/my-key/version',
        credential
    )

    def azure_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
        result = crypto_client.sign(SignatureAlgorithm.es256, data)
        return result.signature

    def azure_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes):
        result = crypto_client.verify(SignatureAlgorithm.es256, data, signature)
        if not result.is_valid:
            raise ValueError('Signature verification failed')
    ```

### Key Rotation

Use the `key_id` field in the COSE header to support key rotation:

```python
from claim169 import encode_with_signer

# Include key version in the credential
def rotating_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
    # key_id contains the key version identifier
    current_key = key_store.get_current_signing_key()
    return current_key.sign(data)

# The verifier can use key_id to look up the correct public key
def rotating_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes):
    if key_id:
        key_version = key_id.decode('utf-8')
        public_key = key_store.get_public_key(key_version)
    else:
        public_key = key_store.get_default_public_key()
    public_key.verify(data, signature)
```

!!! tip "Best Practices"
    - Store signing keys in HSM with `sign` permission only (no export)
    - Use separate keys for signing and encryption
    - Implement key rotation with overlapping validity periods
    - Log all cryptographic operations for audit trails
    - Use key aliases instead of raw key IDs in application code

## Test vectors

For known-good example keys (only for testing), see `test-vectors/valid/*.json`. These vectors include `public_key_hex` and (for some vectors) `private_key_hex`.

