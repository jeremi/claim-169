# Spécification MOSIP Claim 169

Cette bibliothèque implémente la spécification [MOSIP Claim 169](https://github.com/mosip/id-claim-169) pour encoder et décoder des identifiants d’identité hors-ligne dans des QR codes.

## Pipeline d’encodage

```
Données d’identité → CBOR → CWT → COSE_Sign1 → [COSE_Encrypt0] → zlib → Base45 → QR Code
```

## 1) Payload Claim 169 (map CBOR)

Claim 169 utilise des clés numériques CBOR pour être compact.

### Champs principaux (clés 1–23)

| Clé | Champ | Type |
|-----|-------|------|
| 1 | id | tstr |
| 2 | version | tstr |
| 3 | language | tstr |
| 4 | fullName | tstr |
| 5 | firstName | tstr |
| 6 | middleName | tstr |
| 7 | lastName | tstr |
| 8 | dateOfBirth | tstr (recommandé : `AAAAMMJJ` ; courant aussi : `AAAA-MM-JJ`) |
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

### Biométries (clés 50–65)

| Clé | Champ |
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

## 2) CWT (CBOR Web Token)

Le payload Claim 169 est stocké dans un CWT sous la clé `169`, avec des claims standards (`iss`, `sub`, `exp`, `nbf`, `iat`).

## Énumérations (utilisées par cette bibliothèque)

### Photo format (clé 17)

| Valeur | Signification |
|--------|---------------|
| 1 | JPEG |
| 2 | JPEG 2000 |
| 3 | AVIF |
| 4 | WEBP |

### Biometric format (entrée biométrique clé 1)

| Valeur | Signification |
|--------|---------------|
| 0 | Image |
| 1 | Template |
| 2 | Son |
| 3 | BioHash |

## Références

- [MOSIP Claim 169 Repository](https://github.com/mosip/id-claim-169)
- [RFC 8949 — CBOR](https://www.rfc-editor.org/rfc/rfc8949)
- [RFC 9052 — COSE](https://www.rfc-editor.org/rfc/rfc9052)
- [RFC 8392 — CWT](https://www.rfc-editor.org/rfc/rfc8392)
- [RFC 9285 — Base45](https://www.rfc-editor.org/rfc/rfc9285)
