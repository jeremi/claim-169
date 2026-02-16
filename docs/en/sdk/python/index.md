# Python SDK

<div class="badges" markdown>
[![PyPI](https://img.shields.io/pypi/v/claim169)](https://pypi.org/project/claim169/)
[![Python](https://img.shields.io/pypi/pyversions/claim169)](https://pypi.org/project/claim169/)
[![License](https://img.shields.io/pypi/l/claim169)](https://github.com/jeremi/claim-169/blob/main/LICENSE)
</div>

The Python SDK provides native bindings for encoding and decoding MOSIP Claim 169 QR codes. Built with PyO3 for performance, it delivers the full power of the Rust core library to Python applications.

## Why Python?

- **Native Performance** — Rust-powered core with zero-copy parsing where possible
- **Type Hints** — Full type annotations for IDE support and static analysis
- **Pythonic API** — Familiar patterns with builder-style configuration
- **HSM/KMS Ready** — Callback hooks for external crypto providers
- **Cross-Platform** — Pre-built wheels for Linux, macOS, and Windows

## Installation

```bash
pip install claim169
```

Or with uv:

```bash
uv add claim169
```

## Quick Start

```python
import claim169

# Decode a QR code with Ed25519 verification
qr_data = "..."  # Base45 string from QR scanner
public_key = bytes.fromhex("...")  # Issuer's 32-byte Ed25519 public key

result = claim169.decode_with_ed25519(qr_data, public_key)

print(f"ID: {result.claim169.id}")
print(f"Name: {result.claim169.full_name}")
print(f"Verified: {result.is_verified()}")
```

## Documentation

<div class="doc-grid" markdown>

<div class="doc-card" markdown>
### [Installation](installation.md)
Python version requirements, pip/uv installation, platform support.
</div>

<div class="doc-card" markdown>
### [Quick Start](quick-start.md)
Simple encode/decode examples to get you started.
</div>

<div class="doc-card" markdown>
### [Encoding](encoding.md)
Create signed credentials with Ed25519 or ECDSA P-256.
</div>

<div class="doc-card" markdown>
### [Decoding](decoding.md)
Verify and extract identity data from QR codes.
</div>

<div class="doc-card" markdown>
### [Encryption](encryption.md)
AES-256-GCM and AES-128-GCM encryption examples.
</div>

<div class="doc-card" markdown>
### [Custom Crypto](custom-crypto.md)
HSM and cloud KMS integration with AWS, Azure, Google Cloud.
</div>

<div class="doc-card" markdown>
### [API Reference](api.md)
Complete API documentation for all functions and classes.
</div>

<div class="doc-card" markdown>
### [Troubleshooting](troubleshooting.md)
Common errors and solutions.
</div>

</div>

## Features

### Decoding

| Function | Description |
|----------|-------------|
| `decode_with_ed25519()` | Decode with Ed25519 signature verification |
| `decode_with_ecdsa_p256()` | Decode with ECDSA P-256 signature verification |
| `decode_with_verifier()` | Decode with custom verifier callback (HSM/KMS) |
| `decode_encrypted_aes256()` | Decrypt AES-256-GCM and decode |
| `decode_encrypted_aes128()` | Decrypt AES-128-GCM and decode |
| `decode_with_decryptor()` | Decrypt with custom decryptor callback |
| `decode_unverified()` | Decode without verification (testing only) |

### Encoding

| Function | Description |
|----------|-------------|
| `encode()` | Unified encode function with keyword arguments |
| `encode_with_ed25519()` | Encode with Ed25519 signature |
| `encode_with_ecdsa_p256()` | Encode with ECDSA P-256 signature |
| `encode_with_signer()` | Encode with custom signer callback (HSM/KMS) |
| `encode_signed_encrypted()` | Sign with Ed25519 and encrypt with AES-256 |
| `encode_signed_encrypted_aes128()` | Sign with Ed25519 and encrypt with AES-128 |
| `encode_with_signer_and_encryptor()` | Custom signer and encryptor callbacks |
| `encode_unsigned()` | Encode without signature (testing only) |

## Requirements

- Python 3.8 or later
- No additional dependencies for basic usage
- `cryptography` package for custom crypto providers

## License

MIT License. See [LICENSE](https://github.com/jeremi/claim-169/blob/main/LICENSE) for details.
