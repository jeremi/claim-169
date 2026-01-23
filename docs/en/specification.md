# MOSIP Claim 169 Specification

This library implements the [MOSIP Claim 169](https://github.com/mosip/id-claim-169) QR code specification for encoding and decoding offline-verifiable identity credentials.

## Overview

Claim 169 is designed for:

- **Offline verification** — no network is required to validate credentials
- **Compact size** — optimized for QR code capacity
- **Security** — signatures for authenticity, optional encryption for privacy
- **Interoperability** — based on CBOR, COSE, and CWT

## Encoding pipeline

```
Identity Data → CBOR → CWT → COSE_Sign1 → [COSE_Encrypt0] → zlib → Base45 → QR Code
```

## 1) Claim 169 payload (CBOR map)

Claim 169 uses CBOR with numeric keys for compactness. The payload is a CBOR map (key → value).

### Demographics & core fields (keys 1–23)

| Key | Field | Type |
|-----|-------|------|
| 1 | id | tstr |
| 2 | version | tstr |
| 3 | language | tstr |
| 4 | fullName | tstr |
| 5 | firstName | tstr |
| 6 | middleName | tstr |
| 7 | lastName | tstr |
| 8 | dateOfBirth | tstr (recommended: `YYYYMMDD`; commonly also `YYYY-MM-DD`) |
| 9 | gender | int |
| 10 | address | tstr |
| 11 | email | tstr |
| 12 | phone | tstr |
| 13 | nationality | tstr |
| 14 | maritalStatus | int |
| 15 | guardian | tstr |
| 16 | photo | bstr |
| 17 | photoFormat | int |
| 18 | bestQualityFingers | array |
| 19 | secondaryFullName | tstr |
| 20 | secondaryLanguage | tstr |
| 21 | locationCode | tstr |
| 22 | legalStatus | tstr |
| 23 | countryOfIssuance | tstr |

### Biometrics (keys 50–65)

Each biometric field is an array of biometric entries.

| Key | Field |
|-----|-------|
| 50 | rightThumb |
| 51 | rightPointerFinger |
| 52 | rightMiddleFinger |
| 53 | rightRingFinger |
| 54 | rightLittleFinger |
| 55 | leftThumb |
| 56 | leftPointerFinger |
| 57 | leftMiddleFinger |
| 58 | leftRingFinger |
| 59 | leftLittleFinger |
| 60 | rightIris |
| 61 | leftIris |
| 62 | face |
| 63 | rightPalm |
| 64 | leftPalm |
| 65 | voice |

### Biometric entry structure

A biometric entry is a CBOR map:

| Key | Field | Type |
|-----|-------|------|
| 0 | data | bstr |
| 1 | format | int |
| 2 | subFormat | int |
| 3 | issuer | tstr |

## 2) CWT wrapping (CBOR Web Token)

The Claim 169 CBOR map is stored inside a CWT with standard claims:

| Claim | Key | Description |
|-------|-----|-------------|
| iss | 1 | Issuer |
| sub | 2 | Subject |
| exp | 4 | Expiration time (Unix seconds) |
| nbf | 5 | Not before (Unix seconds) |
| iat | 6 | Issued at (Unix seconds) |
| **169** | 169 | Claim 169 payload |

## 3) COSE signing (COSE_Sign1)

The CWT is signed using COSE_Sign1. Supported signature algorithms:

| Algorithm | COSE alg | Description |
|-----------|----------|-------------|
| EdDSA | -8 | Ed25519 |
| ES256 | -7 | ECDSA P-256 + SHA-256 |

## 4) Optional encryption (COSE_Encrypt0)

For privacy, the signed payload may be encrypted in a COSE_Encrypt0 envelope. Supported algorithms:

| Algorithm | COSE alg | Key size |
|-----------|----------|----------|
| A256GCM | 3 | 32 bytes |
| A128GCM | 1 | 16 bytes |

The nonce/IV is 12 bytes and must be unique per encryption.

## 5) Compression (zlib)

The COSE bytes are compressed with zlib (DEFLATE) to fit comfortably in QR codes.

## 6) Base45 encoding

The compressed bytes are encoded as Base45 for QR alphanumeric mode.

## Enumerations (as used by this library)

### Gender

| Value | Meaning |
|-------|---------|
| 1 | Male |
| 2 | Female |
| 3 | Other |

### Marital status

| Value | Meaning |
|-------|---------|
| 1 | Unmarried |
| 2 | Married |
| 3 | Divorced |

### Photo format (key 17)

| Value | Meaning |
|-------|---------|
| 1 | JPEG |
| 2 | JPEG 2000 |
| 3 | AVIF |
| 4 | WEBP |

### Biometric format (biometric entry key 1)

| Value | Meaning |
|-------|---------|
| 0 | Image |
| 1 | Template |
| 2 | Sound |
| 3 | BioHash |

### Biometric subFormat (biometric entry key 2)

The meaning of `subFormat` depends on `format`:

- `Image`: `0=PNG, 1=JPEG, 2=JPEG2000, 3=AVIF, 4=WEBP, 5=TIFF, 6=WSQ`
- `Template`: `0=ANSI378, 1=ISO19794-2, 2=NIST`
- `Sound`: `0=WAV, 1=MP3`

## Security & robustness notes (library behavior)

- **Verification is required by default** (unless you explicitly opt out for testing).
- **Timestamp validation** (`exp`/`nbf`) is enabled by default in Rust and Python.
- **Decompression limit** defaults to **64KB** and is configurable.
- **CBOR nesting depth** is limited to **128 levels**.
- **Unknown fields** are preserved for forward compatibility.

## References

- [MOSIP Claim 169 Repository](https://github.com/mosip/id-claim-169)
- [RFC 8949 — CBOR](https://www.rfc-editor.org/rfc/rfc8949)
- [RFC 9052 — COSE](https://www.rfc-editor.org/rfc/rfc9052)
- [RFC 8392 — CWT](https://www.rfc-editor.org/rfc/rfc8392)
- [RFC 9285 — Base45](https://www.rfc-editor.org/rfc/rfc9285)
