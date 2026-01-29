# Test Vectors

Test vectors for validating Claim 169 implementations.

## Overview

Test vectors are JSON files containing known inputs and expected outputs. Use them to verify your implementation handles all cases correctly.

## Location

Test vectors are in the `test-vectors/` directory:

```
test-vectors/
├── valid/           # Valid credentials that should decode
├── invalid/         # Invalid inputs that should fail
└── edge/            # Edge cases and boundary conditions
```

## Generating Test Vectors

Generate fresh test vectors using the included tool:

```bash
cargo run -p generate-vectors
```

This creates vectors with current timestamps and fresh keys.

## Vector Format

Each test vector is a JSON file:

```json
{
  "name": "ed25519-signed",
  "description": "COSE_Sign1 with Ed25519 signature",
  "category": "valid",
  "qr_data": "NCFKXE...",
  "signing_key": {
    "algorithm": "EdDSA",
    "public_key_hex": "d75a9801..."
  },
  "expected_claim169": {
    "id": "ID-12345-ABCDE",
    "fullName": "Signed Test Person"
  },
  "expected_cwt_meta": {
    "issuer": "https://mosip.example.org",
    "expiresAt": 1800000000,
    "issuedAt": 1700000000
  }
}
```

Invalid/edge vectors include an `expected_error` field (for example `Base45Decode`, `Decompress`, `CoseParse`, `Claim169NotFound`).

## Valid Vectors

### Basic Credentials

| Vector | Description |
|--------|-------------|
| `minimal.json` | Minimal claim with ID and full name only |
| `ed25519-signed.json` | COSE_Sign1 with Ed25519 signature |
| `ecdsa-p256-signed.json` | COSE_Sign1 with ECDSA P-256 signature |
| `demographics-full.json` | All demographic fields populated |
| `with-face.json` | Credential with face biometric data |
| `with-fingerprints.json` | Credential with fingerprint biometric data |
| `with-all-biometrics.json` | Credential with all biometrics fields |
| `claim169-example.json` | Example payload with typical fields |

### Encrypted Credentials

| Vector | Description |
|--------|-------------|
| `encrypted-aes256.json` | COSE_Encrypt0 with AES-256-GCM |
| `encrypted-signed.json` | COSE_Encrypt0 containing signed COSE_Sign1 |

## Invalid Vectors

### Decode Failures

| Vector | Expected Error |
|--------|----------------|
| `bad-base45.json` | `Base45Decode` |
| `bad-zlib.json` | `Decompress` |
| `not-cose.json` | `CoseParse` |
| `missing-169.json` | `Claim169NotFound` |

## Edge Cases

| Vector | Description |
|--------|-------------|
| `unknown_fields.json` | Contains unknown CBOR keys |
| `expired.json` | Token with `exp` in the past |
| `not-yet-valid.json` | Token with `nbf` in the future |

## Using Test Vectors

### Rust

```rust
use std::fs;
use serde_json::Value;
use claim169_core::Decoder;

#[test]
fn test_basic_ed25519() {
    let json: Value = serde_json::from_str(
        &fs::read_to_string("test-vectors/valid/ed25519-signed.json").unwrap()
    ).unwrap();

    let qr_data = json["qr_data"].as_str().unwrap();
    let public_key = hex::decode(json["signing_key"]["public_key_hex"].as_str().unwrap()).unwrap();

    let result = Decoder::new(qr_data)
        .verify_with_ed25519(&public_key)
        .unwrap()
        .decode()
        .unwrap();

    assert_eq!(
        result.claim169.id.as_deref(),
        json["expected_claim169"]["id"].as_str()
    );
}
```

### Python

```python
import json
from claim169 import Decoder

def test_basic_ed25519():
    with open("test-vectors/valid/ed25519-signed.json") as f:
        vector = json.load(f)

    qr_data = vector["qr_data"]
    public_key = bytes.fromhex(vector["signing_key"]["public_key_hex"])

    result = Decoder(qr_data).verify_with_ed25519(public_key).decode()

    assert result.claim169.id == vector["expected_claim169"]["id"]
```

### TypeScript

```typescript
import { readFileSync } from 'fs';
import { Decoder } from 'claim169';

test('basic_ed25519', () => {
  const vector = JSON.parse(
    readFileSync('test-vectors/valid/ed25519-signed.json', 'utf-8')
  );

  const publicKey = Buffer.from(vector.signing_key.public_key_hex, 'hex');

  const result = new Decoder(vector.qr_data)
    .verifyWithEd25519(new Uint8Array(publicKey))
    .decode();

  expect(result.claim169.id).toBe(vector.expected_claim169.id);
});
```

## Conformance Testing

To validate a Claim 169 implementation:

1. **Pass all valid vectors** — Decode successfully with correct output
2. **Fail all invalid vectors** — Return the expected error type
3. **Handle all edge cases** — Process unusual but valid inputs

A conformant implementation must pass all test vectors.
