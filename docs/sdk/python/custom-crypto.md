# Custom Crypto Providers

This guide covers integrating external cryptographic providers such as Hardware Security Modules (HSMs) and cloud Key Management Services (KMS).

## Overview

The claim169 library supports callback-based crypto hooks, allowing you to:

- **Sign** with keys stored in HSMs or cloud KMS
- **Verify** signatures using external providers
- **Encrypt** with externally managed keys
- **Decrypt** using HSM or KMS

This can help meet security requirements that mandate private key material stays within secure hardware (depending on your provider and configuration).

## Callback Interfaces

### Signer Callback

```python
def signer_callback(
    algorithm: str,      # "EdDSA" or "ES256"
    key_id: bytes | None,  # Optional key identifier
    data: bytes          # Data to sign
) -> bytes:              # Signature bytes
    ...
```

### Verifier Callback

```python
def verifier_callback(
    algorithm: str,      # "EdDSA" or "ES256"
    key_id: bytes | None,  # Optional key identifier
    data: bytes,         # Signed data
    signature: bytes     # Signature to verify
) -> None:               # Raise exception if invalid
    ...
```

### Encryptor Callback

```python
def encryptor_callback(
    algorithm: str,      # "A256GCM" or "A128GCM"
    key_id: bytes | None,  # Optional key identifier
    nonce: bytes,        # 12-byte nonce
    aad: bytes,          # Additional authenticated data
    plaintext: bytes     # Data to encrypt
) -> bytes:              # Ciphertext with auth tag
    ...
```

### Decryptor Callback

```python
def decryptor_callback(
    algorithm: str,      # "A256GCM" or "A128GCM"
    key_id: bytes | None,  # Optional key identifier
    nonce: bytes,        # 12-byte nonce
    aad: bytes,          # Additional authenticated data
    ciphertext: bytes    # Data to decrypt
) -> bytes:              # Decrypted plaintext
    ...
```

## AWS KMS Integration

### Setup

```bash
pip install boto3
```

### Signing with AWS KMS

```python
import boto3
import claim169

kms_client = boto3.client('kms', region_name='us-east-1')
KEY_ID = 'arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012'

def aws_kms_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
    """Sign using AWS KMS."""
    # AWS KMS supports ECDSA_SHA_256 for ES256
    # For EdDSA, you would need a different approach
    if algorithm != "ES256":
        raise ValueError(f"AWS KMS signer only supports ES256, got {algorithm}")

    response = kms_client.sign(
        KeyId=KEY_ID,
        Message=bytes(data),
        MessageType='RAW',
        SigningAlgorithm='ECDSA_SHA_256'
    )

    # AWS KMS returns DER-encoded signature, convert to raw r||s
    der_sig = response['Signature']
    return der_to_raw_ecdsa(der_sig)


def der_to_raw_ecdsa(der_sig: bytes) -> bytes:
    """Convert DER signature to raw r||s format (64 bytes)."""
    from cryptography.hazmat.primitives.asymmetric.utils import decode_dss_signature
    r, s = decode_dss_signature(der_sig)
    return r.to_bytes(32, 'big') + s.to_bytes(32, 'big')


# Encode credential with AWS KMS signing
claim = claim169.Claim169Input(id="AWS-KMS-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(issuer="https://id.example.org")

qr_data = claim169.encode_with_signer(
    claim,
    meta,
    aws_kms_signer,
    "ES256",
    key_id=KEY_ID.encode()
)
```

### Verifying with AWS KMS

```python
def aws_kms_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes) -> None:
    """Verify signature using AWS KMS."""
    # Convert raw r||s signature back to DER
    raw_sig = bytes(signature)
    r = int.from_bytes(raw_sig[:32], 'big')
    s = int.from_bytes(raw_sig[32:], 'big')

    from cryptography.hazmat.primitives.asymmetric.utils import encode_dss_signature
    der_sig = encode_dss_signature(r, s)

    response = kms_client.verify(
        KeyId=KEY_ID,
        Message=bytes(data),
        MessageType='RAW',
        Signature=der_sig,
        SigningAlgorithm='ECDSA_SHA_256'
    )

    if not response['SignatureValid']:
        raise ValueError("Signature verification failed")


result = claim169.decode_with_verifier(qr_data, aws_kms_verifier)
```

### Encryption with AWS KMS

```python
def aws_kms_encryptor(algorithm: str, key_id: bytes | None, nonce: bytes, aad: bytes, plaintext: bytes) -> bytes:
    """Encrypt using AWS KMS data key."""
    # Generate a data key for envelope encryption
    response = kms_client.generate_data_key(
        KeyId=KEY_ID,
        KeySpec='AES_256'
    )

    # Use the plaintext key for local encryption
    from cryptography.hazmat.primitives.ciphers.aead import AESGCM
    aesgcm = AESGCM(response['Plaintext'])
    ciphertext = aesgcm.encrypt(bytes(nonce), bytes(plaintext), bytes(aad))

    # In practice, you'd store response['CiphertextBlob'] alongside the encrypted data
    return ciphertext
```

## Azure Key Vault Integration

### Setup

```bash
pip install azure-identity azure-keyvault-keys azure-keyvault-secrets
```

### Signing with Azure Key Vault

```python
from azure.identity import DefaultAzureCredential
from azure.keyvault.keys import KeyClient
from azure.keyvault.keys.crypto import CryptographyClient, SignatureAlgorithm

credential = DefaultAzureCredential()
vault_url = "https://my-vault.vault.azure.net/"
key_name = "my-signing-key"

key_client = KeyClient(vault_url=vault_url, credential=credential)
key = key_client.get_key(key_name)
crypto_client = CryptographyClient(key, credential=credential)


def azure_kv_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
    """Sign using Azure Key Vault."""
    import hashlib

    if algorithm == "ES256":
        # Azure requires pre-hashed data for ECDSA
        digest = hashlib.sha256(bytes(data)).digest()
        result = crypto_client.sign(SignatureAlgorithm.es256, digest)
        return result.signature
    else:
        raise ValueError(f"Unsupported algorithm: {algorithm}")


# Encode with Azure Key Vault signing
claim = claim169.Claim169Input(id="AZURE-KV-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(issuer="https://id.example.org")

qr_data = claim169.encode_with_signer(
    claim,
    meta,
    azure_kv_signer,
    "ES256"
)
```

### Verifying with Azure Key Vault

```python
def azure_kv_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes) -> None:
    """Verify using Azure Key Vault."""
    import hashlib

    digest = hashlib.sha256(bytes(data)).digest()
    result = crypto_client.verify(SignatureAlgorithm.es256, digest, bytes(signature))

    if not result.is_valid:
        raise ValueError("Signature verification failed")


result = claim169.decode_with_verifier(qr_data, azure_kv_verifier)
```

## Google Cloud KMS Integration

### Setup

```bash
pip install google-cloud-kms
```

### Signing with Google Cloud KMS

```python
from google.cloud import kms

client = kms.KeyManagementServiceClient()
key_name = client.crypto_key_version_path(
    'my-project',
    'us-east1',
    'my-keyring',
    'my-key',
    '1'
)


def gcp_kms_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
    """Sign using Google Cloud KMS."""
    import hashlib
    from google.cloud.kms import CryptoKeyVersion

    if algorithm != "ES256":
        raise ValueError(f"GCP KMS signer configured for ES256, got {algorithm}")

    # GCP requires SHA256 digest for EC_SIGN_P256_SHA256
    digest = {'sha256': hashlib.sha256(bytes(data)).digest()}

    response = client.asymmetric_sign(
        request={'name': key_name, 'digest': digest}
    )

    # GCP returns DER, convert to raw
    return der_to_raw_ecdsa(response.signature)


def der_to_raw_ecdsa(der_sig: bytes) -> bytes:
    """Convert DER signature to raw r||s format."""
    from cryptography.hazmat.primitives.asymmetric.utils import decode_dss_signature
    r, s = decode_dss_signature(der_sig)
    return r.to_bytes(32, 'big') + s.to_bytes(32, 'big')


# Encode with GCP KMS signing
claim = claim169.Claim169Input(id="GCP-KMS-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(issuer="https://id.example.org")

qr_data = claim169.encode_with_signer(
    claim,
    meta,
    gcp_kms_signer,
    "ES256"
)
```

### Verifying with Google Cloud KMS

```python
def gcp_kms_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes) -> None:
    """Verify using Google Cloud KMS."""
    import hashlib
    from cryptography.hazmat.primitives.asymmetric.utils import encode_dss_signature

    # Convert raw r||s back to DER for GCP
    raw_sig = bytes(signature)
    r = int.from_bytes(raw_sig[:32], 'big')
    s = int.from_bytes(raw_sig[32:], 'big')
    der_sig = encode_dss_signature(r, s)

    digest = {'sha256': hashlib.sha256(bytes(data)).digest()}

    response = client.asymmetric_verify(
        request={
            'name': key_name,
            'digest': digest,
            'signature': der_sig
        }
    )

    if not response.success:
        raise ValueError("Signature verification failed")


result = claim169.decode_with_verifier(qr_data, gcp_kms_verifier)
```

## HashiCorp Vault Integration

### Setup

```bash
pip install hvac
```

### Signing with HashiCorp Vault Transit

```python
import hvac

client = hvac.Client(url='https://vault.example.org:8200')
client.token = 'your-vault-token'


def vault_transit_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
    """Sign using HashiCorp Vault Transit."""
    import base64

    # Vault expects base64 input
    input_b64 = base64.b64encode(bytes(data)).decode()

    response = client.secrets.transit.sign_data(
        name='my-signing-key',
        hash_input=input_b64,
        signature_algorithm='pkcs1v15' if algorithm == 'ES256' else 'ed25519'
    )

    # Extract and decode the signature
    sig_b64 = response['data']['signature'].split(':')[-1]
    return base64.b64decode(sig_b64)


# Encode with Vault signing
claim = claim169.Claim169Input(id="VAULT-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(issuer="https://id.example.org")

qr_data = claim169.encode_with_signer(
    claim,
    meta,
    vault_transit_signer,
    "EdDSA"
)
```

## PKCS#11 / HSM Integration

### Setup

```bash
pip install python-pkcs11
```

### Signing with PKCS#11 HSM

```python
import pkcs11
from pkcs11 import KeyType, Mechanism

# Load the PKCS#11 library
lib = pkcs11.lib('/usr/lib/softhsm/libsofthsm2.so')
token = lib.get_token(token_label='MyHSM')


def pkcs11_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
    """Sign using PKCS#11 HSM."""
    with token.open(user_pin='1234') as session:
        # Find the private key
        private_key = session.get_key(
            key_type=KeyType.EC,
            object_class=pkcs11.ObjectClass.PRIVATE_KEY,
            label='my-signing-key'
        )

        # Sign the data
        if algorithm == "ES256":
            signature = private_key.sign(
                bytes(data),
                mechanism=Mechanism.ECDSA_SHA256
            )
        else:
            raise ValueError(f"Unsupported algorithm: {algorithm}")

        return signature


# Encode with HSM signing
claim = claim169.Claim169Input(id="HSM-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(issuer="https://id.example.org")

qr_data = claim169.encode_with_signer(
    claim,
    meta,
    pkcs11_signer,
    "ES256"
)
```

## Complete Example: AWS KMS Sign + Encrypt

```python
import boto3
import secrets
import claim169
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

# AWS KMS setup
kms_client = boto3.client('kms', region_name='us-east-1')
SIGN_KEY_ID = 'arn:aws:kms:...:key/sign-key-id'
ENCRYPT_KEY_ID = 'arn:aws:kms:...:key/encrypt-key-id'


def aws_signer(algorithm, key_id, data):
    """Sign using AWS KMS."""
    response = kms_client.sign(
        KeyId=SIGN_KEY_ID,
        Message=bytes(data),
        MessageType='RAW',
        SigningAlgorithm='ECDSA_SHA_256'
    )
    return der_to_raw_ecdsa(response['Signature'])


def aws_encryptor(algorithm, key_id, nonce, aad, plaintext):
    """Encrypt using AWS KMS envelope encryption."""
    # Generate a data key
    response = kms_client.generate_data_key(
        KeyId=ENCRYPT_KEY_ID,
        KeySpec='AES_256'
    )

    # Encrypt locally with the data key
    aesgcm = AESGCM(response['Plaintext'])
    return aesgcm.encrypt(bytes(nonce), bytes(plaintext), bytes(aad))


def aws_verifier(algorithm, key_id, data, signature):
    """Verify using AWS KMS."""
    raw_sig = bytes(signature)
    r = int.from_bytes(raw_sig[:32], 'big')
    s = int.from_bytes(raw_sig[32:], 'big')

    from cryptography.hazmat.primitives.asymmetric.utils import encode_dss_signature
    der_sig = encode_dss_signature(r, s)

    response = kms_client.verify(
        KeyId=SIGN_KEY_ID,
        Message=bytes(data),
        MessageType='RAW',
        Signature=der_sig,
        SigningAlgorithm='ECDSA_SHA_256'
    )
    if not response['SignatureValid']:
        raise ValueError("Verification failed")


def aws_decryptor(algorithm, key_id, nonce, aad, ciphertext):
    """Decrypt using locally stored data key."""
    # In practice, retrieve the encrypted data key and decrypt it first
    # This is a simplified example
    data_key = get_cached_data_key()  # Your implementation
    aesgcm = AESGCM(data_key)
    return aesgcm.decrypt(bytes(nonce), bytes(ciphertext), bytes(aad))


# Encode
claim = claim169.Claim169Input(id="AWS-FULL-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(issuer="https://id.example.org")

qr_data = claim169.encode_with_signer_and_encryptor(
    claim,
    meta,
    aws_signer,
    "ES256",
    aws_encryptor,
    "A256GCM"
)

# Decode
result = claim169.decode_with_decryptor(
    qr_data,
    aws_decryptor,
    verifier=aws_verifier
)

print(f"Verified: {result.is_verified()}")
```

## Error Handling

```python
import claim169

def safe_signer(algorithm, key_id, data):
    try:
        return external_crypto_provider.sign(data)
    except ConnectionError as e:
        raise RuntimeError(f"Crypto provider unavailable: {e}")
    except PermissionError as e:
        raise RuntimeError(f"Access denied to signing key: {e}")


try:
    qr_data = claim169.encode_with_signer(claim, meta, safe_signer, "EdDSA")
except claim169.Claim169Exception as e:
    print(f"Encoding failed: {e}")
```

## Best Practices

### Key Rotation

- Implement key versioning in your KMS
- Use `key_id` parameter to track key versions
- Plan for graceful key rotation

### Error Handling

- Catch and wrap provider-specific exceptions
- Implement retry logic for transient failures
- Log crypto operations for audit trails

### Performance

- Cache KMS clients and connections
- Consider connection pooling for HSMs
- Use async APIs where available

### Security

- Use IAM roles/managed identities, not static credentials
- Enable audit logging on your KMS
- Implement proper key access controls

## Next Steps

- [API Reference](api.md) — Complete function documentation
- [Troubleshooting](troubleshooting.md) — Common errors and solutions
