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

### Signataire Personnalisé (HSM/KMS)

Pour les environnements de production, les clés privées ne doivent jamais quitter le matériel sécurisé. Utilisez un callback de signataire personnalisé pour intégrer des Modules de Sécurité Matérielle (HSM), des Services de Gestion de Clés cloud (AWS KMS, Google Cloud KMS, Azure Key Vault), des cartes à puce, des TPM ou des services de signature distants.

Le callback reçoit :

- `algorithm` : Le nom de l'algorithme COSE (ex. `"EdDSA"`, `"ES256"`)
- `key_id` : Octets d'identifiant de clé optionnels (depuis l'en-tête COSE, si présent)
- `data` : Les données à signer (COSE `Sig_structure`)

Le callback doit retourner les octets de la signature.

=== "Rust"

    ```rust
    use claim169_core::{Encoder, Signer};

    struct HsmSigner {
        hsm_client: MyHsmClient,
        key_id: String,
    }

    impl Signer for HsmSigner {
        fn sign(&self, algorithm: &str, _key_id: Option<&[u8]>, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
            // Call your HSM to sign the data
            let signature = self.hsm_client.sign(&self.key_id, data)?;
            Ok(signature)
        }
    }

    let signer = HsmSigner {
        hsm_client: my_hsm,
        key_id: "my-signing-key".to_string(),
    };

    let qr_data = Encoder::new(claim, meta)
        .sign_with(&signer, "EdDSA")?
        .encode()?;
    ```

=== "Python"

    ```python
    from claim169 import encode_with_signer

    def my_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
        """
        Custom signer callback for HSM/KMS integration.

        Args:
            algorithm: COSE algorithm name ("EdDSA", "ES256", etc.)
            key_id: Optional key identifier from COSE header
            data: The data to sign (COSE Sig_structure)

        Returns:
            Signature bytes
        """
        # Example: AWS KMS
        # response = kms_client.sign(
        #     KeyId='alias/my-signing-key',
        #     Message=data,
        #     SigningAlgorithm='ECDSA_SHA_256'
        # )
        # return response['Signature']

        # Example: PKCS#11 HSM
        return my_hsm.sign(key_id, data)

    qr_data = encode_with_signer(claim, meta, my_signer, "EdDSA")
    ```

=== "TypeScript"

    ```typescript
    import { Encoder, SignerCallback } from 'claim169';

    const mySigner: SignerCallback = async (
      algorithm: string,
      keyId: Uint8Array | null,
      data: Uint8Array
    ): Promise<Uint8Array> => {
      // Example: Google Cloud KMS
      // const [signResponse] = await kmsClient.asymmetricSign({
      //   name: keyVersionName,
      //   data: data,
      // });
      // return new Uint8Array(signResponse.signature);

      // Example: Azure Key Vault
      // const result = await cryptoClient.sign("ES256", data);
      // return result.result;

      return myHsm.sign(keyId, data);
    };

    const qrData = new Encoder(claim, meta)
      .signWith(mySigner, "EdDSA")
      .encode();
    ```

!!! tip "Identifiant de clé dans l'en-tête COSE"
    Vous pouvez inclure un identifiant de clé dans l'en-tête COSE pour aider le vérificateur à localiser la bonne clé publique. Ceci est utile lors de la rotation des clés ou lorsque plusieurs émetteurs partagent une infrastructure.

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
