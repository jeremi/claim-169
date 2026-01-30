# Glossary

Terms and concepts used in the Claim 169 specification and documentation.

## Standards & Protocols

### CBOR
**Concise Binary Object Representation** — A binary data format designed for small code size and message size. Similar to JSON but more compact. Defined in [RFC 8949](https://www.rfc-editor.org/rfc/rfc8949).

### COSE
**CBOR Object Signing and Encryption** — A framework for signing and encrypting CBOR data. Provides `COSE_Sign1` for signatures and `COSE_Encrypt0` for encryption. Defined in [RFC 9052](https://www.rfc-editor.org/rfc/rfc9052).

### CWT
**CBOR Web Token** — A compact token format for claims, similar to JWT but using CBOR. Carries identity claims and metadata. Defined in [RFC 8392](https://www.rfc-editor.org/rfc/rfc8392).

### Base45
An encoding scheme that converts binary data to alphanumeric characters optimized for QR codes. Defined in [RFC 9285](https://www.rfc-editor.org/rfc/rfc9285).

### zlib
A compression library using the DEFLATE algorithm. Used to reduce credential size before Base45 encoding.

## Cryptographic Terms

### Ed25519
A digital signature algorithm using elliptic curve cryptography. Fast, secure, and produces 64-byte signatures with 32-byte keys.

### ECDSA P-256
**Elliptic Curve Digital Signature Algorithm** using the NIST P-256 curve. Widely supported, produces 64-byte signatures with 32-byte private keys.

### AES-GCM
**Advanced Encryption Standard - Galois/Counter Mode** — An authenticated encryption algorithm providing both confidentiality and integrity.

### AEAD
**Authenticated Encryption with Associated Data** — Encryption that provides both confidentiality and authenticity. AES-GCM is an AEAD algorithm.

### Nonce
**Number used once** — A random value that must never be reused with the same key. For AES-GCM, nonces are 12 bytes.

### Key ID (kid)
An identifier included in COSE headers to help the verifier select the correct key.

## COSE Message Types

### COSE_Sign1
A COSE message containing a single signature. Used for signed credentials.

### COSE_Encrypt0
A COSE message containing encrypted content with a single recipient. Used for encrypted credentials.

### Protected Header
COSE header parameters that are included in signature calculation. Cannot be modified without invalidating the signature.

### Unprotected Header
COSE header parameters not included in signature calculation. Can be modified without affecting verification.

## CWT Claims

### iss (Issuer)
**Claim key: 1** — Identifies who issued the credential. Typically a URL or organizational identifier.

### sub (Subject)
**Claim key: 2** — Identifies who the credential is about. May be a user ID or name.

### exp (Expiration)
**Claim key: 4** — Unix timestamp after which the credential is invalid.

### nbf (Not Before)
**Claim key: 5** — Unix timestamp before which the credential is not valid.

### iat (Issued At)
**Claim key: 6** — Unix timestamp when the credential was issued.

### Claim 169
**Claim key: 169** — The IANA-registered claim containing the identity payload.

## Identity Fields

### Demographics
Core identity information: name, date of birth, gender, address, nationality, etc. CBOR keys 1-23.

### Biometrics
Biological identity markers: fingerprints, iris scans, face images, voice samples. CBOR keys 50-65.

### Photo Format
The image format for embedded photos: JPEG, JPEG 2000, AVIF, or WEBP.

### Best Quality Fingers
An array indicating which fingers have the highest-quality biometric captures, ordered by quality.

## Security Terms

### Signature Verification
The process of checking that a credential's signature is valid using the issuer's public key.

### Timestamp Validation
Checking that a credential is within its valid time window (after `nbf`, before `exp`).

### Decompression Bomb
A malicious compressed payload that expands to an enormous size, potentially exhausting memory.

### Algorithm Confusion
An attack where an attacker tricks the verifier into using a different algorithm than intended.

### Forward Compatibility
The ability to decode credentials containing unknown fields, preserving them for future use.

## Library Terms

### Decoder
The component that converts a Base45 QR string into a decoded credential.

### Encoder
The component that converts identity data into a signed, compressed Base45 string.

### Verification Status
The result of signature verification: `Verified`, `Skipped` (verification not performed), or an error.

### Custom Crypto Provider
An implementation of cryptographic operations using external systems (HSM, KMS, etc.).
