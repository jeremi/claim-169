# Spécification MOSIP Claim 169

Cette bibliothèque implémente la spécification QR code [MOSIP Claim 169](https://github.com/mosip/id-claim-169) pour encoder et décoder des identifiants d’identité vérifiables hors ligne.

## Vue d’ensemble

Claim 169 est conçu pour :

- **Vérification hors ligne** — aucun réseau requis pour valider des identifiants
- **Taille compacte** — optimisée pour la capacité des QR codes
- **Sécurité** — signatures pour l’authenticité, chiffrement optionnel pour la confidentialité
- **Interopérabilité** — basée sur les standards CBOR, COSE et CWT

## Pipeline d’encodage

```
Identity Data → CBOR → CWT → COSE_Sign1 → [COSE_Encrypt0] → zlib → Base45 → QR Code
```

## 1) Charge utile Claim 169 (Map CBOR)

Claim 169 utilise CBOR avec des clés numériques pour être compact. La charge utile est une map CBOR (clé → valeur).

### Données démographiques & champs principaux (clés 1–23)

| Clé | Champ | Type | Description |
|-----|-------|------|-------------|
| 1 | id | tstr | Identifiant unique |
| 2 | version | tstr | Version des données d’identité |
| 3 | language | tstr | Code langue (ISO 639-3) |
| 4 | fullName | tstr | Nom complet |
| 5 | firstName | tstr | Prénom |
| 6 | middleName | tstr | Deuxième prénom |
| 7 | lastName | tstr | Nom de famille |
| 8 | dateOfBirth | tstr | Date de naissance (`YYYYMMDD` ou `YYYY-MM-DD`) |
| 9 | gender | int | Code genre (voir énumérations) |
| 10 | address | tstr | Adresse avec séparateurs `\n` |
| 11 | email | tstr | Adresse email |
| 12 | phone | tstr | Numéro de téléphone (format E.123) |
| 13 | nationality | tstr | Nationalité (ISO 3166-1/2) |
| 14 | maritalStatus | int | Code état civil |
| 15 | guardian | tstr | Nom/id du tuteur/responsable |
| 16 | photo | bstr | Données photo binaires |
| 17 | photoFormat | int | Code format photo |
| 18 | bestQualityFingers | array | Positions des doigts « best quality » (0-10) |
| 19 | secondaryFullName | tstr | Nom complet en langue secondaire |
| 20 | secondaryLanguage | tstr | Code langue secondaire (ISO 639-3) |
| 21 | locationCode | tstr | Localisation / code géographique |
| 22 | legalStatus | tstr | Statut légal de l’identité |
| 23 | countryOfIssuance | tstr | Pays d’émission |

### Biométrie (clés 50–65)

Chaque champ biométrique est un tableau d’entrées biométriques.

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

### Structure d’une entrée biométrique

Une entrée biométrique est une map CBOR :

| Clé | Champ | Type | Description |
|-----|-------|------|-------------|
| 0 | data | bstr | Données biométriques brutes |
| 1 | format | int | Code format biométrique |
| 2 | subFormat | int | Code sous-format biométrique |
| 3 | issuer | tstr | Émetteur biométrique |

## 2) Enveloppe CWT (CBOR Web Token)

La map CBOR Claim 169 est stockée dans un CWT avec des claims standards :

| Claim | Clé | Description |
|-------|-----|-------------|
| iss | 1 | Émetteur |
| sub | 2 | Sujet |
| exp | 4 | Expiration (secondes Unix) |
| nbf | 5 | Pas avant (secondes Unix) |
| iat | 6 | Émis à (secondes Unix) |
| **169** | 169 | Charge utile Claim 169 |

## 3) Signature COSE (COSE_Sign1)

Le CWT est signé via COSE_Sign1. Algorithmes de signature supportés :

| Algorithme | COSE alg | Description |
|-----------|----------|-------------|
| EdDSA | -8 | Ed25519 |
| ES256 | -7 | ECDSA P-256 + SHA-256 |

## 4) Chiffrement optionnel (COSE_Encrypt0)

Pour la confidentialité, la charge utile signée peut être chiffrée dans une enveloppe COSE_Encrypt0 :

| Algorithme | COSE alg | Taille de clé |
|-----------|----------|--------------|
| A256GCM | 3 | 32 octets |
| A128GCM | 1 | 16 octets |

Le nonce/IV fait 12 octets et doit être unique pour chaque chiffrement.

## 5) Compression (zlib)

Les octets COSE sont compressés avec zlib (DEFLATE) afin de tenir confortablement dans des QR codes.

Le décodeur détecte automatiquement le format de compression en inspectant les premiers octets (`0x78` indique zlib). Cela permet une compatibilité ascendante si d'autres formats de compression sont utilisés à l'avenir.

!!! note "Compression non standard"
    Cette bibliothèque prend également en charge l'encodage sans compression ou avec Brotli (opt-in via la feature `compression-brotli`). Ce sont des extensions non standard. La spécification impose zlib, et d'autres implémentations Claim 169 peuvent ne pas supporter les formats alternatifs.

## 6) Encodage Base45

Les octets compressés sont encodés en Base45 pour le mode alphanumérique des QR codes.

## Énumérations

### Genre

| Valeur | Signification |
|-------|---------------|
| 1 | Masculin |
| 2 | Féminin |
| 3 | Autre |

### État civil

| Valeur | Signification |
|-------|---------------|
| 1 | Célibataire |
| 2 | Marié(e) |
| 3 | Divorcé(e) |

### Format photo (clé 17)

| Valeur | Signification |
|-------|---------------|
| 1 | JPEG |
| 2 | JPEG 2000 |
| 3 | AVIF |
| 4 | WEBP |

### Format biométrique (clé 1 dans l’entrée biométrique)

| Valeur | Signification |
|-------|---------------|
| 0 | Image |
| 1 | Modèle (template) |
| 2 | Son |
| 3 | BioHash |

### Sous-format biométrique (clé 2 dans l’entrée biométrique)

La signification de `subFormat` dépend de `format` :

- **Image** : `0=PNG, 1=JPEG, 2=JPEG2000, 3=AVIF, 4=WEBP, 5=TIFF, 6=WSQ`
- **Modèle** : `0=ANSI378, 1=ISO19794-2, 2=NIST`
- **Son** : `0=WAV, 1=MP3`

## Notes de sécurité & robustesse

- **La vérification est requise par défaut** (il faut explicitement s’en écarter)
- **La validation des horodatages** (`exp`/`nbf`) est activée par défaut
- **La limite de décompression** vaut 64KB par défaut et est configurable
- **La profondeur d’imbrication CBOR** est limitée à 128 niveaux
- **Les champs inconnus** sont conservés pour la compatibilité ascendante

## Références

- [Dépôt MOSIP Claim 169](https://github.com/mosip/id-claim-169)
- [RFC 8949 — CBOR](https://www.rfc-editor.org/rfc/rfc8949)
- [RFC 9052 — COSE](https://www.rfc-editor.org/rfc/rfc9052)
- [RFC 8392 — CWT](https://www.rfc-editor.org/rfc/rfc8392)
- [RFC 9285 — Base45](https://www.rfc-editor.org/rfc/rfc9285)
