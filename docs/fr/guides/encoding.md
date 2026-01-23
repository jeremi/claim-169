# Encodage des identifiants

Ce guide couvre la création d'identifiants Claim 169 avec toutes les options disponibles.

## Champs d'identité

La spécification Claim 169 définit les champs d'identité suivants :

| Champ | Clé CBOR | Type | Description |
|-------|----------|------|-------------|
| `id` | 1 | string | Identifiant unique |
| `version` | 2 | string | Version de la spécification |
| `language` | 3 | string | Langue principale (ISO 639-3) |
| `fullName` | 4 | string | Nom complet |
| `firstName` | 5 | string | Prénom |
| `middleName` | 6 | string | Deuxième prénom |
| `lastName` | 7 | string | Nom de famille |
| `dateOfBirth` | 8 | string | Date de naissance (recommandé : `AAAAMMJJ` ; courant aussi : `AAAA-MM-JJ`) |
| `gender` | 9 | integer | 1=Masculin, 2=Féminin, 3=Autre |
| `address` | 10 | string | Adresse complète |
| `email` | 11 | string | Adresse email |
| `phone` | 12 | string | Numéro de téléphone |
| `nationality` | 13 | string | Code pays |
| `maritalStatus` | 14 | integer | 1=Célibataire, 2=Marié(e), 3=Divorcé(e) |
| `guardian` | 15 | string | Tuteur / responsable |
| `photo` | 16 | bytes | Données photo |
| `photoFormat` | 17 | integer | 1=JPEG, 2=JPEG2000, 3=AVIF, 4=WEBP |
| `bestQualityFingers` | 18 | array | Positions des doigts (0–10) |
| `secondaryFullName` | 19 | string | Nom dans la langue secondaire |
| `secondaryLanguage` | 20 | string | Langue secondaire (ISO 639-3) |
| `locationCode` | 21 | string | Code de localisation |
| `legalStatus` | 22 | string | Statut légal |
| `countryOfIssuance` | 23 | string | Pays d’émission |

## Signature (recommandé)

### Ed25519

=== "Rust"

    ```rust
    let qr_data = claim169_core::Encoder::new(claim, meta)
        .sign_with_ed25519(&private_key)?
        .encode()?;
    ```

=== "Python"

    ```python
    from claim169 import encode_with_ed25519

    qr_data = encode_with_ed25519(claim, meta, private_key)
    ```

=== "TypeScript"

    ```ts
    const qrData = new Encoder(claim, meta)
      .signWithEd25519(privateKey)
      .encode();
    ```

## Chiffrement (optionnel)

Le chiffrement protège la confidentialité (AES-GCM). L’ordre est **signer puis chiffrer**.

=== "Rust"

    ```rust
    let qr_data = claim169_core::Encoder::new(claim, meta)
        .sign_with_ed25519(&signing_key)?
        .encrypt_with_aes256(&encryption_key)?
        .encode()?;
    ```

=== "Python"

    ```python
    from claim169 import encode_signed_encrypted

    qr_data = encode_signed_encrypted(claim, meta, signing_key, encryption_key)
    ```

=== "TypeScript"

    ```ts
    const qrData = new Encoder(claim, meta)
      .signWithEd25519(signingKey)
      .encryptWithAes256(encryptionKey)
      .encode();
    ```

Pour plus de détails, consultez aussi :

- [Matériel de clés et formats](keys.md)
- [Sécurité et validations](security.md)
