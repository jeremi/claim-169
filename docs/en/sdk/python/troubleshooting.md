# Troubleshooting

Common errors and solutions for the claim169 Python SDK.

## Import Errors

### Module Not Found

```
ModuleNotFoundError: No module named 'claim169'
```

**Solution:**

1. Verify installation:
   ```bash
   pip show claim169
   ```

2. Check you're using the correct Python interpreter:
   ```bash
   which python
   python -c "import sys; print(sys.executable)"
   ```

3. Reinstall:
   ```bash
   pip uninstall claim169
   pip install claim169
   ```

### Import Library Error

```
ImportError: libssl.so.1.1: cannot open shared object file
```

**Solution:** Install OpenSSL libraries:

```bash
# Ubuntu/Debian
sudo apt-get install libssl-dev

# CentOS/RHEL
sudo yum install openssl-devel

# macOS
brew install openssl
```

---

## Decoding Errors

### Base45DecodeError

```python
claim169.Base45DecodeError: Invalid Base45 character at position 15
```

**Causes:**
- QR code was not fully scanned
- QR code content was truncated
- Input is not a Claim 169 QR code

**Solutions:**

1. Verify the QR code was fully scanned
2. Check for leading/trailing whitespace:
   ```python
   qr_data = qr_data.strip()
   result = claim169.decode_with_ed25519(qr_data, public_key)
   ```
3. Verify the QR code is actually a Claim 169 credential

### DecompressError

```python
claim169.DecompressError: zlib decompression failed
```

**Causes:**
- Corrupted QR code data
- Data was modified after encoding
- Not a valid Claim 169 credential

**Solutions:**

1. Re-scan the QR code
2. Check for data corruption during transmission
3. Verify the QR code source

### DecompressError: Size Limit Exceeded

```python
claim169.DecompressError: decompressed size 150000 exceeds limit 65536
```

**Cause:** Credential decompresses to larger than the limit (default 64KB).

**Solution:** Increase the limit if you trust the source:

```python
result = claim169.decode_with_ed25519(
    qr_data,
    public_key,
    max_decompressed_bytes=200000  # 200KB limit
)
```

### CoseParseError

```python
claim169.CoseParseError: Invalid COSE structure
```

**Causes:**
- Not a COSE-encoded credential
- Corrupted COSE structure
- Wrong type of QR code

**Solution:** Verify the QR code is a Claim 169 credential, not another type (e.g., EU DCC, SMART Health Card).

### Claim169NotFoundError

```python
claim169.Claim169NotFoundError: Claim 169 not found in CWT
```

**Cause:** The COSE/CWT structure is valid but doesn't contain a Claim 169 payload.

**Solution:** This QR code uses a different claim format. Verify you're scanning a MOSIP Claim 169 credential.

### SignatureError

```python
claim169.SignatureError: Signature verification failed
```

**Causes:**
- Wrong public key
- Credential was tampered with
- Key/algorithm mismatch

**Solutions:**

1. Verify you're using the correct public key for this issuer
2. Check the key format (Ed25519 vs ECDSA P-256)
3. Verify the key is the correct length:
   - Ed25519: 32 bytes
   - ECDSA P-256: 33 bytes (compressed) or 65 bytes (uncompressed)

```python
print(f"Key length: {len(public_key)} bytes")
# Ed25519 should be 32
# ECDSA P-256 compressed should be 33
# ECDSA P-256 uncompressed should be 65
```

### DecryptionError

```python
claim169.DecryptionError: Decryption failed
```

**Causes:**
- Wrong decryption key
- Wrong key size for algorithm
- Corrupted ciphertext

**Solutions:**

1. Verify key size matches algorithm:
   ```python
   print(f"Key length: {len(encrypt_key)} bytes")
   # AES-256: 32 bytes
   # AES-128: 16 bytes
   ```

2. Use the correct function for your key size:
   ```python
   # For 32-byte keys
   result = claim169.decode_encrypted_aes256(qr_data, key_32, allow_unverified=True)

   # For 16-byte keys
   result = claim169.decode_encrypted_aes128(qr_data, key_16, allow_unverified=True)
   ```

### Timestamp Validation Errors

```python
claim169.Claim169Exception: Token expired at 1700000000
```

**Cause:** The credential has expired.

**Solutions:**

1. Check if the credential should be rejected (it's expired)
2. For testing, disable timestamp validation:
   ```python
   result = claim169.decode_with_ed25519(
       qr_data,
       public_key,
       validate_timestamps=False
   )
   ```

```python
claim169.Claim169Exception: Token not valid until 1800000000
```

**Cause:** The credential's `nbf` (not before) time is in the future.

**Solutions:**

1. Wait until the credential becomes valid
2. Check for clock synchronization issues
3. Add clock skew tolerance:
   ```python
   result = claim169.decode_with_ed25519(
       qr_data,
       public_key,
       clock_skew_tolerance_seconds=300  # 5 minutes
   )
   ```

---

## Encoding Errors

### Invalid Key Length

```python
ValueError: Ed25519 private key must be 32 bytes
```

**Solution:** Ensure your key is the correct length:

```python
private_key = bytes.fromhex("...")
print(f"Key length: {len(private_key)} bytes")  # Should be 32

# If using cryptography library:
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
private_key_obj = Ed25519PrivateKey.generate()
private_key = private_key_obj.private_bytes_raw()  # Exactly 32 bytes
```

### Callback Returns Wrong Type

```python
claim169.Claim169Exception: signer callback must return bytes
```

**Cause:** Your custom signer/encryptor callback returned a non-bytes type.

**Solution:** Ensure your callback returns `bytes`:

```python
def my_signer(algorithm, key_id, data):
    signature = some_signing_operation(data)
    return bytes(signature)  # Ensure it's bytes
```

### Callback Raises Exception

```python
claim169.Claim169Exception: RuntimeError: Crypto provider unavailable
```

**Cause:** Your custom callback raised an exception.

**Solution:** Handle errors in your callback:

```python
def my_signer(algorithm, key_id, data):
    try:
        return crypto_provider.sign(data)
    except ConnectionError:
        # Log the error
        raise RuntimeError("Failed to connect to crypto provider")
```

---

## Custom Crypto Errors

### AWS KMS Errors

```python
botocore.exceptions.ClientError: AccessDeniedException
```

**Solution:** Check IAM permissions for the KMS key:

```json
{
    "Effect": "Allow",
    "Action": [
        "kms:Sign",
        "kms:Verify",
        "kms:GenerateDataKey"
    ],
    "Resource": "arn:aws:kms:..."
}
```

### Azure Key Vault Errors

```python
azure.core.exceptions.ClientAuthenticationError
```

**Solution:** Verify Azure credentials:

```python
from azure.identity import DefaultAzureCredential
credential = DefaultAzureCredential()

# Test the credential
credential.get_token("https://vault.azure.net/.default")
```

### PKCS#11 HSM Errors

```python
pkcs11.exceptions.TokenNotPresent
```

**Solution:**

1. Verify the HSM is connected
2. Check the PKCS#11 library path
3. Verify token label and PIN

---

## Performance Issues

### Slow Decoding with Biometrics

**Cause:** Large biometric data takes time to parse.

**Solution:** Skip biometrics if not needed:

```python
result = claim169.decode_with_ed25519(
    qr_data,
    public_key,
    skip_biometrics=True
)
```

### Memory Usage

**Cause:** Large credentials consume memory during decoding.

**Solution:** Set appropriate size limits:

```python
result = claim169.decode_with_ed25519(
    qr_data,
    public_key,
    max_decompressed_bytes=32768  # 32KB limit
)
```

---

## Common Mistakes

### Forgetting to Convert Key from Hex

```python
# Wrong - passing hex string
public_key = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
result = claim169.decode_with_ed25519(qr_data, public_key)  # Error!

# Correct - convert to bytes
public_key = bytes.fromhex("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
result = claim169.decode_with_ed25519(qr_data, public_key)  # Works!
```

### Using Wrong Key Type

```python
# Wrong - using private key for verification
result = claim169.decode_with_ed25519(qr_data, private_key)  # May fail

# Correct - use public key for verification
result = claim169.decode_with_ed25519(qr_data, public_key)
```

### Missing Verifier for Encrypted Credentials

```python
# Wrong - no verifier provided
result = claim169.decode_encrypted_aes(qr_data, key)  # ValueError

# Correct - provide verifier or allow_unverified
result = claim169.decode_encrypted_aes(qr_data, key, verifier=my_verifier)
# or for testing:
result = claim169.decode_encrypted_aes(qr_data, key, allow_unverified=True)
```

---

## Getting Help

If you encounter an issue not covered here:

1. **Check the API reference** — [api.md](api.md)
2. **Review examples** — Check test files in the repository
3. **Open an issue** — [GitHub Issues](https://github.com/jeremi/claim-169/issues)

When reporting issues, include:

- Python version: `python --version`
- claim169 version: `python -c "import claim169; print(claim169.version())"`
- Operating system
- Minimal code to reproduce the issue
- Full error traceback
