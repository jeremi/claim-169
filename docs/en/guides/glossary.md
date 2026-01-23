# Glossary

**Base45**  
Text encoding optimized for QR alphanumeric mode. Claim 169 uses Base45 after compression so QR payloads stay compact and scanner-friendly.

**CBOR** (Concise Binary Object Representation)  
Binary data format used for compact encoding. Claim 169 uses CBOR maps with numeric keys (e.g. `1`, `4`, `8`) to minimize size.

**COSE** (CBOR Object Signing and Encryption)  
Standard for signing/encrypting CBOR payloads.

**COSE_Sign1**  
COSE structure used for signatures. Claim 169 uses it to sign the embedded CWT.

**COSE_Encrypt0**  
COSE structure used for authenticated encryption (AEAD). Claim 169 optionally encrypts the signed payload for privacy.

**CWT** (CBOR Web Token)  
Token container (similar to JWT but CBOR-based). Claim 169 stores the identity payload under CWT claim key `169`, and uses standard claims like `iss`, `exp`, `nbf`, `iat`.

**Ed25519 / EdDSA**  
Signature algorithm with 32-byte public keys. Used for compact signatures.

**ECDSA P-256 / ES256**  
Signature algorithm based on P-256 and SHA-256. Public key is provided as SEC1 (33-byte compressed or 65-byte uncompressed) or PEM/SPKI (Rust).

**AES-GCM**  
Authenticated encryption algorithm. Claim 169 supports AES-128-GCM and AES-256-GCM. Requires a unique 12-byte nonce/IV per encryption.

**Issuer / Verifier**  
Issuer creates and signs (and optionally encrypts) credentials. Verifier checks signatures and policy (timestamps, size limits, etc.) when decoding.

**Test vectors**  
Known-good and known-bad QR payloads stored under `test-vectors/` to validate behavior across SDKs and reproduce edge cases.

