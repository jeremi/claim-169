# Examples

This directory contains runnable examples demonstrating how to use the claim169 library.

## Python Examples

Located in `python/`:

| Example | Description |
|---------|-------------|
| `basic_decode.py` | Decode a QR code without verification (testing only) |
| `verified_decode.py` | Decode with Ed25519 signature verification |
| `encode_credential.py` | Create a signed credential |
| `encrypted_credential.py` | Decode an encrypted and signed credential |

### Running Python Examples

```bash
cd examples/python

# Install the library (from repo root)
cd ../../core/claim169-python && maturin develop && cd ../../examples/python

# Install example dependencies
pip install -r requirements.txt

# Run an example
python basic_decode.py
python verified_decode.py
python encode_credential.py
python encrypted_credential.py
```

## TypeScript Examples

Located in `typescript/`:

| Example | Description |
|---------|-------------|
| `basic-decode.test.ts` | Decode a QR code without verification (testing only) |
| `verified-decode.test.ts` | Decode with Ed25519 signature verification |
| `encode-credential.test.ts` | Create a signed credential |
| `encrypted-credential.test.ts` | Decode an encrypted and signed credential |

### Running TypeScript Examples

The TypeScript examples use vitest to handle WASM loading correctly.

```bash
cd examples/typescript

# Build the main library first (from repo root)
cd ../../sdks/typescript && npm run build:ts && cd ../../examples/typescript

# Install dependencies
npm install

# Run all examples
npm test

# Run individual examples
npm run basic
npm run verified
npm run encode
npm run encrypted
```

## Test Vectors

The examples use test vectors from `test-vectors/` directory. These are pre-generated QR code payloads with known keys for testing.

## Security Note

The private keys in these examples are for **demonstration purposes only**. In production:

- Never hardcode private keys in source code
- Use a secure key management system (HSM/KMS)
- Rotate keys regularly
- Follow your organization's security policies
