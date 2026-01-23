# Troubleshooting

This page lists common errors and the fastest way to resolve them.

## “decoding configuration error”

If you see an error like “either provide a verifier or explicitly allow unverified decoding”, it means you tried to decode without:

- configuring signature verification, or
- explicitly opting out (testing only).

Fix:

=== "Rust"

    - Production: call `.verify_with_ed25519(...)` or `.verify_with_ecdsa_p256(...)`
    - Testing: call `.allow_unverified()`

=== "Python"

    - Production: use `decode_with_ed25519()` / `decode_with_ecdsa_p256()`
    - Testing: use `decode_unverified()` (or `decode()`, if you enabled the alias)

=== "TypeScript"

    - Production: call `.verifyWithEd25519(...)` / `.verifyWithEcdsaP256(...)`
    - Testing: call `.allowUnverified()`

## Signature verification failures

Common causes:

- wrong algorithm (Ed25519 vs ES256),
- wrong public key (wrong issuer, wrong environment),
- corrupted or truncated QR text.

Fix:

- confirm the vector/key pair match,
- ensure you pass raw key bytes in the expected format (see the Keys guide).

## “credential expired” / “not valid until …”

This is timestamp validation rejecting the credential based on `exp`/`nbf`.

Fix options:

- use current (non-expired) credentials,
- adjust clock skew tolerance,
- disable timestamp validation only if your threat model allows it.

## “decompression limit exceeded”

The decoder enforces a maximum decompressed payload size (default 64KB) to prevent zip-bomb attacks.

Fix:

- if you control the issuer and expect larger payloads, raise the limit with `max_decompressed_bytes(...)` / `maxDecompressedBytes(...)`,
- otherwise treat the input as malicious and reject.

## Decryption failures

Common causes:

- using the wrong AES key (or wrong key length),
- attempting to verify before decrypting (order matters),
- corrupted ciphertext.

Fix:

- ensure you decrypt **before** verifying,
- verify key size: AES-256 is 32 bytes, AES-128 is 16 bytes.

