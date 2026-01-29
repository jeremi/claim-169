# API Reference

Complete API documentation for the claim169 Python SDK, auto-generated from source code.

## Module Functions

::: claim169.version

::: claim169.generate_nonce

---

## Decode Functions

::: claim169.decode_unverified

::: claim169.decode_with_ed25519

::: claim169.decode_with_ecdsa_p256

::: claim169.decode_with_ed25519_pem

::: claim169.decode_with_ecdsa_p256_pem

::: claim169.decode_with_verifier

::: claim169.decode_encrypted_aes

::: claim169.decode_encrypted_aes256

::: claim169.decode_encrypted_aes128

::: claim169.decode_with_decryptor

---

## Encode Functions

::: claim169.encode_with_ed25519

::: claim169.encode_with_ecdsa_p256

::: claim169.encode_signed_encrypted

::: claim169.encode_signed_encrypted_aes128

::: claim169.encode_unsigned

::: claim169.encode_with_signer

::: claim169.encode_with_signer_and_encryptor

::: claim169.encode_with_encryptor

---

## Data Classes

::: claim169.DecodeResult

::: claim169.Claim169

::: claim169.Claim169Input

::: claim169.CwtMeta

::: claim169.CwtMetaInput

::: claim169.Biometric

::: claim169.CertificateHash

::: claim169.X509Headers

---

## Crypto Hook Classes

::: claim169.PySignatureVerifier

::: claim169.PyDecryptor

::: claim169.PySigner

::: claim169.PyEncryptor

---

## Exceptions

::: claim169.Claim169Exception

::: claim169.Base45DecodeError

::: claim169.DecompressError

::: claim169.CoseParseError

::: claim169.CwtParseError

::: claim169.Claim169NotFoundError

::: claim169.SignatureError

::: claim169.DecryptionError

::: claim169.EncryptionError

---

## Constants

### Gender Values

| Value | Meaning |
|-------|---------|
| 1 | Male |
| 2 | Female |
| 3 | Other |

### Marital Status Values

| Value | Meaning |
|-------|---------|
| 1 | Unmarried |
| 2 | Married |
| 3 | Divorced |

### Photo Format Values

| Value | Meaning |
|-------|---------|
| 1 | JPEG |
| 2 | JPEG2000 |
| 3 | AVIF |
| 4 | WebP |

### Algorithm Names

**Signing:**
- `"EdDSA"` — Ed25519
- `"ES256"` — ECDSA P-256

**Encryption:**
- `"A256GCM"` — AES-256-GCM
- `"A128GCM"` — AES-128-GCM
