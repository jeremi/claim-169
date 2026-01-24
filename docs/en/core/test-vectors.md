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
  "description": "Basic credential with Ed25519 signature",
  "qr_data": "NCFKXE...",
  "expected": {
    "claim169": {
      "id": "12345",
      "fullName": "John Doe"
    },
    "cwtMeta": {
      "issuer": "https://example.org",
      "expiresAt": 1735689600
    },
    "verificationStatus": "Verified"
  },
  "keys": {
    "ed25519_public": "b4f3..."
  }
}
```

## Valid Vectors

### Basic Credentials

| Vector | Description |
|--------|-------------|
| `basic_ed25519.json` | Minimal credential with Ed25519 signature |
| `basic_ecdsa.json` | Minimal credential with ECDSA P-256 signature |
| `full_demographics.json` | All demographic fields populated |
| `with_photo.json` | Credential with embedded photo |
| `with_biometrics.json` | Credential with biometric data |

### Encrypted Credentials

| Vector | Description |
|--------|-------------|
| `encrypted_aes256.json` | AES-256-GCM encrypted credential |
| `encrypted_aes128.json` | AES-128-GCM encrypted credential |
| `signed_then_encrypted.json` | Signed, then encrypted (COSE_Encrypt0 wrapping COSE_Sign1) |

### Timestamp Variations

| Vector | Description |
|--------|-------------|
| `with_expiry.json` | Credential with expiration time |
| `with_nbf.json` | Credential with not-before time |
| `with_all_timestamps.json` | exp, nbf, and iat set |

## Invalid Vectors

### Decode Failures

| Vector | Expected Error |
|--------|----------------|
| `invalid_base45.json` | `Base45Decode` |
| `truncated_data.json` | `Decompress` |
| `invalid_cbor.json` | `CborParse` |
| `invalid_cose.json` | `CoseParse` |
| `missing_claim169.json` | `Claim169NotFound` |

### Signature Failures

| Vector | Expected Error |
|--------|----------------|
| `wrong_signature.json` | `SignatureInvalid` |
| `wrong_key.json` | `SignatureInvalid` |
| `tampered_payload.json` | `SignatureInvalid` |

### Timestamp Failures

| Vector | Expected Error |
|--------|----------------|
| `expired.json` | `Expired` |
| `not_yet_valid.json` | `NotYetValid` |

## Edge Cases

| Vector | Description |
|--------|-------------|
| `empty_fields.json` | All optional fields absent |
| `unicode_names.json` | UTF-8 names in various scripts |
| `max_photo_size.json` | Large photo near size limit |
| `unknown_fields.json` | Contains unknown CBOR keys |
| `zero_timestamps.json` | Timestamps set to 0 |
| `max_timestamps.json` | Timestamps near i64 maximum |

## Using Test Vectors

### Rust

```rust
use std::fs;
use serde_json::Value;
use claim169_core::Decoder;

#[test]
fn test_basic_ed25519() {
    let json: Value = serde_json::from_str(
        &fs::read_to_string("test-vectors/valid/basic_ed25519.json").unwrap()
    ).unwrap();

    let qr_data = json["qr_data"].as_str().unwrap();
    let public_key = hex::decode(json["keys"]["ed25519_public"].as_str().unwrap()).unwrap();

    let result = Decoder::new(qr_data)
        .verify_with_ed25519(&public_key)
        .unwrap()
        .decode()
        .unwrap();

    assert_eq!(
        result.claim169.id.as_deref(),
        json["expected"]["claim169"]["id"].as_str()
    );
}
```

### Python

```python
import json
from claim169 import Decoder

def test_basic_ed25519():
    with open("test-vectors/valid/basic_ed25519.json") as f:
        vector = json.load(f)

    qr_data = vector["qr_data"]
    public_key = bytes.fromhex(vector["keys"]["ed25519_public"])

    result = Decoder(qr_data).verify_with_ed25519(public_key).decode()

    assert result.claim169.id == vector["expected"]["claim169"]["id"]
```

### TypeScript

```typescript
import { readFileSync } from 'fs';
import { Decoder } from 'claim169';

test('basic_ed25519', () => {
  const vector = JSON.parse(
    readFileSync('test-vectors/valid/basic_ed25519.json', 'utf-8')
  );

  const publicKey = Buffer.from(vector.keys.ed25519_public, 'hex');

  const result = new Decoder(vector.qr_data)
    .verifyWithEd25519(new Uint8Array(publicKey))
    .decode();

  expect(result.claim169.id).toBe(vector.expected.claim169.id);
});
```

## Conformance Testing

To validate a Claim 169 implementation:

1. **Pass all valid vectors** — Decode successfully with correct output
2. **Fail all invalid vectors** — Return the expected error type
3. **Handle all edge cases** — Process unusual but valid inputs

A conformant implementation must pass all test vectors.
