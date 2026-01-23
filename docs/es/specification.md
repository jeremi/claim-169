# Especificación MOSIP Claim 169

Esta biblioteca implementa la especificación [MOSIP Claim 169](https://github.com/mosip/id-claim-169) para codificar y decodificar credenciales de identidad offline en códigos QR.

## Pipeline de codificación

```
Datos de identidad → CBOR → CWT → COSE_Sign1 → [COSE_Encrypt0] → zlib → Base45 → Código QR
```

## 1) Payload Claim 169 (map CBOR)

Claim 169 usa claves CBOR numéricas para ser compacto.

### Campos principales (claves 1–23)

| Clave | Campo | Tipo |
|------:|-------|------|
| 1 | id | tstr |
| 2 | version | tstr |
| 3 | language | tstr |
| 4 | fullName | tstr |
| 5 | firstName | tstr |
| 6 | middleName | tstr |
| 7 | lastName | tstr |
| 8 | dateOfBirth | tstr (recomendado: `AAAAMMDD`; común también: `AAAA-MM-DD`) |
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

### Biometría (claves 50–65)

| Clave | Campo |
|------:|-------|
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

El payload Claim 169 se guarda dentro de un CWT bajo la clave `169`, junto con claims estándar (`iss`, `sub`, `exp`, `nbf`, `iat`).

## Enumeraciones (usadas por esta biblioteca)

### Formato de foto (clave 17)

| Valor | Significado |
|------:|-------------|
| 1 | JPEG |
| 2 | JPEG 2000 |
| 3 | AVIF |
| 4 | WEBP |

### Biometric format (entrada biométrica clave 1)

| Valor | Significado |
|------:|-------------|
| 0 | Image |
| 1 | Template |
| 2 | Sound |
| 3 | BioHash |

## Referencias

- [MOSIP Claim 169 Repository](https://github.com/mosip/id-claim-169)
- [RFC 8949 — CBOR](https://www.rfc-editor.org/rfc/rfc8949)
- [RFC 9052 — COSE](https://www.rfc-editor.org/rfc/rfc9052)
- [RFC 8392 — CWT](https://www.rfc-editor.org/rfc/rfc8392)
- [RFC 9285 — Base45](https://www.rfc-editor.org/rfc/rfc9285)
