# MOSIP Claim 169 Specification

This library implements the [MOSIP Claim 169](https://github.com/mosip/id-claim-169) QR code specification for encoding and decoding offline-verifiable identity credentials.

## Overview

Claim 169 is designed for:

- **Offline verification** — no network required to validate credentials
- **Compact size** — optimized for QR code capacity
- **Security** — signatures for authenticity, optional encryption for privacy
- **Interoperability** — based on CBOR, COSE, and CWT standards

## Encoding Pipeline

```
Identity Data → CBOR → CWT → COSE_Sign1 → [COSE_Encrypt0] → zlib → Base45 → QR Code
```

## 1) Claim 169 Payload (CBOR Map)

Claim 169 uses CBOR with numeric keys for compactness. The payload is a CBOR map (key → value).

### Demographics & Core Fields (Keys 1–23)

| Key | Field | Type | Description |
|-----|-------|------|-------------|
| 1 | id | tstr | Unique identifier |
| 2 | version | tstr | Version of the ID data |
| 3 | language | tstr | Language code (ISO 639-3) |
| 4 | fullName | tstr | Full name |
| 5 | firstName | tstr | First name |
| 6 | middleName | tstr | Middle name |
| 7 | lastName | tstr | Last name |
| 8 | dateOfBirth | tstr | Date of birth (`YYYYMMDD` or `YYYY-MM-DD`) |
| 9 | gender | int | Gender code (see enumerations) |
| 10 | address | tstr | Address with `\n` separators |
| 11 | email | tstr | Email address |
| 12 | phone | tstr | Phone number (E.123 format) |
| 13 | nationality | tstr | Nationality (ISO 3166-1/2) |
| 14 | maritalStatus | int | Marital status code |
| 15 | guardian | tstr | Guardian name/id |
| 16 | photo | bstr | Binary photo data |
| 17 | photoFormat | int | Photo format code |
| 18 | bestQualityFingers | array | Best quality finger positions (0-10) |
| 19 | secondaryFullName | tstr | Full name in secondary language |
| 20 | secondaryLanguage | tstr | Secondary language code (ISO 639-3) |
| 21 | locationCode | tstr | Geo location/code |
| 22 | legalStatus | tstr | Legal status of identity |
| 23 | countryOfIssuance | tstr | Country of issuance |

### Biometrics (Keys 50–65)

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

### Biometric Entry Structure

A biometric entry is a CBOR map:

| Key | Field | Type | Description |
|-----|-------|------|-------------|
| 0 | data | bstr | Raw biometric data |
| 1 | format | int | Biometric format code |
| 2 | subFormat | int | Biometric sub-format code |
| 3 | issuer | tstr | Biometric issuer |

## 2) CWT Wrapping (CBOR Web Token)

The Claim 169 CBOR map is stored inside a CWT with standard claims:

| Claim | Key | Description |
|-------|-----|-------------|
| iss | 1 | Issuer |
| sub | 2 | Subject |
| exp | 4 | Expiration time (Unix seconds) |
| nbf | 5 | Not before (Unix seconds) |
| iat | 6 | Issued at (Unix seconds) |
| **169** | 169 | Claim 169 payload |

## 3) COSE Signing (COSE_Sign1)

The CWT is signed using COSE_Sign1. Supported signature algorithms:

| Algorithm | COSE alg | Description |
|-----------|----------|-------------|
| EdDSA | -8 | Ed25519 |
| ES256 | -7 | ECDSA P-256 + SHA-256 |

## 4) Optional Encryption (COSE_Encrypt0)

For privacy, the signed payload may be encrypted in a COSE_Encrypt0 envelope:

| Algorithm | COSE alg | Key Size |
|-----------|----------|----------|
| A256GCM | 3 | 32 bytes |
| A128GCM | 1 | 16 bytes |

The nonce/IV is 12 bytes and must be unique per encryption.

## 5) Compression (zlib)

The COSE bytes are compressed with zlib (DEFLATE) to fit comfortably in QR codes.

## 6) Base45 Encoding

The compressed bytes are encoded as Base45 for QR alphanumeric mode.

## Enumerations

### Gender

| Value | Meaning |
|-------|---------|
| 1 | Male |
| 2 | Female |
| 3 | Other |

### Marital Status

| Value | Meaning |
|-------|---------|
| 1 | Unmarried |
| 2 | Married |
| 3 | Divorced |

### Photo Format (Key 17)

| Value | Meaning |
|-------|---------|
| 1 | JPEG |
| 2 | JPEG 2000 |
| 3 | AVIF |
| 4 | WEBP |

### Biometric Format (Key 1 in Biometric Entry)

| Value | Meaning |
|-------|---------|
| 0 | Image |
| 1 | Template |
| 2 | Sound |
| 3 | BioHash |

### Biometric SubFormat (Key 2 in Biometric Entry)

The meaning of `subFormat` depends on `format`:

- **Image**: `0=PNG, 1=JPEG, 2=JPEG2000, 3=AVIF, 4=WEBP, 5=TIFF, 6=WSQ`
- **Template**: `0=ANSI378, 1=ISO19794-2, 2=NIST`
- **Sound**: `0=WAV, 1=MP3`

## Security & Robustness Notes

- **Verification is required by default** (must explicitly opt out)
- **Timestamp validation** (`exp`/`nbf`) is enabled by default
- **Decompression limit** defaults to 64KB and is configurable
- **CBOR nesting depth** is limited to 128 levels
- **Unknown fields** are preserved for forward compatibility

## References

- [MOSIP Claim 169 Repository](https://github.com/mosip/id-claim-169)
- [RFC 8949 — CBOR](https://www.rfc-editor.org/rfc/rfc8949)
- [RFC 9052 — COSE](https://www.rfc-editor.org/rfc/rfc9052)
- [RFC 8392 — CWT](https://www.rfc-editor.org/rfc/rfc8392)
- [RFC 9285 — Base45](https://www.rfc-editor.org/rfc/rfc9285)
