# Test Vectors & Conformance

This repository includes a set of JSON test vectors under `test-vectors/` to help you:

- validate your integration,
- compare behavior across SDKs, and
- reproduce edge cases (expired credentials, unknown fields, malformed inputs).

## Directory layout

- `test-vectors/valid/` — expected to decode successfully
- `test-vectors/edge/` — expected to decode, but may require policy decisions (e.g. timestamp validation)
- `test-vectors/invalid/` — expected to be rejected

Each vector contains at least:

- `qr_data` (Base45 text)
- optional key material (`public_key_hex`, `private_key_hex`, encryption keys) for testing
- `expected_claim169` and `expected_cwt_meta` for spot checks

!!! warning "Do not use vector keys in production"
    Test vector keys are public and must never be used for real credentials.

## Example: decode a signed vector

Using `test-vectors/valid/ed25519-signed.json`:

=== "Rust"

    ```rust
    use claim169_core::Decoder;

    let qr_text = include_str!("../../../../test-vectors/valid/ed25519-signed.json");
    // Parse JSON in your app, then:
    // let public_key = hex::decode(public_key_hex).unwrap();

    let result = Decoder::new(qr_data)
        .verify_with_ed25519(&public_key)?
        .decode()?;
    ```

=== "Python"

    ```python
    import json
    import claim169

    v = json.load(open("test-vectors/valid/ed25519-signed.json"))
    public_key = bytes.fromhex(v["signing_key"]["public_key_hex"])

    result = claim169.decode_with_ed25519(v["qr_data"], public_key)
    print(result.claim169.full_name)
    ```

=== "TypeScript"

    ```ts
    import fs from "fs";
    import { Decoder, hexToBytes } from "claim169";

    const v = JSON.parse(fs.readFileSync("test-vectors/valid/ed25519-signed.json", "utf8"));
    const publicKey = hexToBytes(v.signing_key.public_key_hex);

    const result = new Decoder(v.qr_data).verifyWithEd25519(publicKey).decode();
    console.log(result.claim169.fullName);
    ```

## Cross-language conformance script

The repo includes a helper script that compares Python and TypeScript decode results across vectors:

```bash
./scripts/conformance-test.sh
```

Notes:

- The script disables timestamp validation to match TypeScript/WASM defaults.
- You need the Python package and TypeScript SDK dependencies available in your environment.

