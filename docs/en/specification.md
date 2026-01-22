# MOSIP Claim 169 Specification

This library implements the [MOSIP Claim 169](https://github.com/mosip/id-claim-169) QR code specification.

## Overview

Claim 169 is a compact, secure format for encoding identity credentials in QR codes. It's designed for:

- **Offline verification** - No network required to validate credentials
- **Compact size** - Fits in standard QR codes
- **Security** - Digital signatures and optional encryption
- **Interoperability** - Based on open standards (CBOR, COSE, CWT)

## Encoding Pipeline

```
Identity Data → CBOR → CWT → COSE_Sign1 → [COSE_Encrypt0] → zlib → Base45 → QR Code
```

### 1. CBOR Encoding

Identity fields are encoded as CBOR with numeric keys for compactness:

| Key | Field | Type |
|-----|-------|------|
| 1 | id | tstr |
| 2 | version | tstr |
| 3 | language | tstr |
| 4 | fullName | tstr |
| 5 | firstName | tstr |
| 6 | middleName | tstr |
| 7 | lastName | tstr |
| 8 | dateOfBirth | tstr |
| 9 | gender | int |
| 10 | address | tstr |
| 11 | email | tstr |
| 12 | phone | tstr |
| 13 | nationality | tstr |
| 14 | maritalStatus | int |
| 15 | guardian | tstr |
| 16 | photo | bstr |
| 17 | photoFormat | int |
| 18 | legalStatus | tstr |
| 19 | countryOfIssuance | tstr |
| 20 | locationCode | tstr |
| 21 | secondaryLanguage | tstr |
| 22 | secondaryFullName | tstr |
| 23 | bestQualityFingers | array |
| 50-65 | biometrics | map |

### 2. CWT Wrapping

The CBOR data is wrapped in a CWT (CBOR Web Token) with standard claims:

| Claim | Key | Description |
|-------|-----|-------------|
| iss | 1 | Issuer URI |
| sub | 2 | Subject identifier |
| exp | 4 | Expiration time |
| nbf | 5 | Not before time |
| iat | 6 | Issued at time |
| **169** | 169 | Claim 169 payload |

### 3. COSE Signing

The CWT is signed using COSE_Sign1:

```
COSE_Sign1 = [
  protected: { alg: EdDSA | ES256 },
  unprotected: {},
  payload: CWT bytes,
  signature: signature bytes
]
```

Supported algorithms:

| Algorithm | COSE alg | Description |
|-----------|----------|-------------|
| EdDSA | -8 | Ed25519 signatures |
| ES256 | -7 | ECDSA with P-256 and SHA-256 |

### 4. Optional Encryption

For privacy, the signed payload can be encrypted:

```
COSE_Encrypt0 = [
  protected: { alg: A256GCM | A128GCM },
  unprotected: { iv: random IV },
  ciphertext: encrypted COSE_Sign1
]
```

Supported algorithms:

| Algorithm | COSE alg | Key Size |
|-----------|----------|----------|
| A256GCM | 3 | 256 bits |
| A128GCM | 1 | 128 bits |

### 5. Compression

The COSE structure is compressed with zlib (DEFLATE):

- Reduces payload size by ~40-60%
- Required for fitting in QR codes

### 6. Base45 Encoding

The compressed bytes are encoded as Base45:

- Alphabet: `0-9A-Z $%*+-./:` (45 characters)
- Optimized for QR alphanumeric mode
- ~30% overhead (vs ~33% for Base64)

## Enumeration Values

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

### Photo Format

| Value | Meaning |
|-------|---------|
| 1 | JPEG |
| 2 | JPEG 2000 |
| 3 | AVIF |

### Biometric Format

| Value | Meaning |
|-------|---------|
| 1 | ISO 19794-4 (Finger) |
| 2 | ISO 19794-5 (Face) |
| 3 | ISO 19794-6 (Iris) |

## Security Considerations

### Signature Verification

Always verify signatures before trusting credential data. The library:

- Rejects weak keys (small-order Ed25519 points, identity ECDSA point)
- Validates algorithm matches expected
- Returns clear verification status

### Timestamp Validation

Validate CWT timestamps:

- `exp` (expires) - Reject expired credentials
- `nbf` (not before) - Reject credentials not yet valid
- `iat` (issued at) - Consider clock skew tolerance

### Decompression Limits

The library limits decompression to prevent zip bombs:

- Default limit: 64 KB
- Sufficient for all standard credentials
- Configurable if needed

### CBOR Depth Limits

CBOR nesting is limited to 128 levels to prevent stack overflow attacks.

## References

- [MOSIP Claim 169 Repository](https://github.com/mosip/id-claim-169)
- [RFC 8949 - CBOR](https://www.rfc-editor.org/rfc/rfc8949)
- [RFC 9052 - COSE](https://www.rfc-editor.org/rfc/rfc9052)
- [RFC 8392 - CWT](https://www.rfc-editor.org/rfc/rfc8392)
- [RFC 9285 - Base45](https://www.rfc-editor.org/rfc/rfc9285)
