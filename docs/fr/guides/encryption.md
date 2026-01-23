# Chiffrement

Ce guide couvre le chiffrement des identifiants Claim 169 pour protéger les données d'identité sensibles.

## Aperçu

Le chiffrement ajoute une couche de confidentialité en enveloppant la structure COSE signée dans une enveloppe COSE_Encrypt0.

```
Identifiant signé (COSE_Sign1) → COSE_Encrypt0 → zlib → Base45 → QR
```

## Algorithmes supportés

| Algorithme | Taille de clé | Description |
|-----------|---------------|-------------|
| AES-256-GCM | 32 octets | Recommandé pour la plupart des cas |
| AES-128-GCM | 16 octets | Clé plus petite, toujours sécurisé |

## Chiffrer (AES-256-GCM)

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

!!! note "Python et AES-128"
    Les bindings Python n'exposent pas encore l'encodage AES-128-GCM. Rust et TypeScript le supportent via `encrypt_with_aes128` / `encryptWithAes128`.

## Chiffreur Personnalisé

Pour les environnements de production où les clés de chiffrement sont gérées de manière externe, utilisez un callback de chiffreur personnalisé pour vous intégrer aux Modules de Sécurité Matérielle (HSM) ou aux services de gestion de clés cloud (AWS KMS, Google Cloud KMS, Azure Key Vault).

Le callback reçoit :

- `algorithm` : Le nom de l'algorithme COSE (par ex., `"A256GCM"`, `"A128GCM"`)
- `key_id` : Octets d'identifiant de clé optionnel (pour l'en-tête COSE)
- `plaintext` : Les données à chiffrer
- `aad` : Données authentifiées additionnelles (AAD) pour AEAD

Le callback doit retourner le texte chiffré avec le tag d'authentification ajouté.

=== "Rust"

    ```rust
    use claim169_core::{Encoder, Encryptor};

    struct HsmEncryptor {
        hsm_client: MyHsmClient,
        key_id: String,
    }

    impl Encryptor for HsmEncryptor {
        fn encrypt(
            &self,
            algorithm: &str,
            _key_id: Option<&[u8]>,
            plaintext: &[u8],
            aad: &[u8],
        ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
            // Call your HSM to encrypt the data
            // Returns ciphertext || auth_tag
            let ciphertext = self.hsm_client.encrypt(&self.key_id, plaintext, aad)?;
            Ok(ciphertext)
        }
    }

    let encryptor = HsmEncryptor {
        hsm_client: my_hsm,
        key_id: "my-encryption-key".to_string(),
    };

    let qr_data = Encoder::new(claim, meta)
        .sign_with_ed25519(&signing_key)?
        .encrypt_with(&encryptor, "A256GCM")?
        .encode()?;
    ```

=== "Python"

    ```python
    from claim169 import encode_with_signer_and_encryptor

    def my_encryptor(algorithm: str, key_id: bytes | None, plaintext: bytes, aad: bytes) -> bytes:
        """
        Custom encryptor callback for HSM/KMS integration.

        Args:
            algorithm: COSE algorithm name ("A256GCM", "A128GCM", etc.)
            key_id: Optional key identifier for COSE header
            plaintext: The data to encrypt
            aad: Additional authenticated data for AEAD

        Returns:
            Ciphertext with authentication tag appended
        """
        # Example: AWS KMS (envelope encryption pattern)
        # data_key = kms_client.generate_data_key(KeyId='alias/my-key')
        # ciphertext = aes_gcm_encrypt(data_key['Plaintext'], plaintext, aad)
        # return ciphertext

        # Example: PKCS#11 HSM
        return my_hsm.encrypt(key_id, plaintext, aad)

    def my_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
        return my_hsm.sign(key_id, data)

    qr_data = encode_with_signer_and_encryptor(
        claim, meta,
        my_signer, "EdDSA",
        my_encryptor, "A256GCM"
    )
    ```

=== "TypeScript"

    ```typescript
    import { Encoder, EncryptorCallback, SignerCallback } from 'claim169';

    const myEncryptor: EncryptorCallback = async (
      algorithm: string,
      keyId: Uint8Array | null,
      plaintext: Uint8Array,
      aad: Uint8Array
    ): Promise<Uint8Array> => {
      // Example: Google Cloud KMS
      // const [encryptResponse] = await kmsClient.encrypt({
      //   name: keyName,
      //   plaintext: plaintext,
      //   additionalAuthenticatedData: aad,
      // });
      // return new Uint8Array(encryptResponse.ciphertext);

      // Your HSM encryption
      return myHsm.encrypt(keyId, plaintext, aad);
    };

    const qrData = new Encoder(claim, meta)
      .signWith(mySigner, "EdDSA")
      .encryptWith(myEncryptor, "A256GCM")
      .encode();
    ```

## Déchiffrer

=== "Rust"

    ```rust
    let result = claim169_core::Decoder::new(qr_text)
        .decrypt_with_aes256(&encryption_key)?
        .verify_with_ed25519(&public_key)?
        .decode()?;
    ```

=== "Python"

    ```python
    from claim169 import decode_encrypted_aes

    # Test uniquement : déchiffre sans vérifier la signature interne
    result = decode_encrypted_aes(qr_text, encryption_key, allow_unverified=True)
    ```

=== "TypeScript"

    ```ts
    const result = new Decoder(qrText)
      .decryptWithAes256(encryptionKey)
      .verifyWithEd25519(publicKey)
      .decode();
    ```

## Déchiffreur Personnalisé

Pour les environnements de production où les clés de déchiffrement sont gérées de manière externe, utilisez un callback de déchiffreur personnalisé pour vous intégrer aux Modules de Sécurité Matérielle (HSM) ou aux services de gestion de clés cloud.

Le callback reçoit :

- `algorithm` : Le nom de l'algorithme COSE (par ex., `"A256GCM"`, `"A128GCM"`)
- `key_id` : Octets d'identifiant de clé optionnel (provenant de l'en-tête COSE, si présent)
- `ciphertext` : Les données chiffrées (texte chiffré avec tag d'authentification)
- `aad` : Données authentifiées additionnelles (AAD) pour AEAD

Le callback doit retourner le texte clair déchiffré. Levez une exception si le déchiffrement échoue (par ex., non-correspondance du tag d'authentification).

=== "Rust"

    ```rust
    use claim169_core::{Decoder, Decryptor};

    struct HsmDecryptor {
        hsm_client: MyHsmClient,
    }

    impl Decryptor for HsmDecryptor {
        fn decrypt(
            &self,
            algorithm: &str,
            key_id: Option<&[u8]>,
            ciphertext: &[u8],
            aad: &[u8],
        ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
            // Use key_id to locate the correct decryption key in your HSM
            let key_name = key_id
                .map(|id| String::from_utf8_lossy(id).to_string())
                .unwrap_or_else(|| "default-encryption-key".to_string());

            // Call your HSM to decrypt the data
            let plaintext = self.hsm_client.decrypt(&key_name, ciphertext, aad)?;
            Ok(plaintext)
        }
    }

    let decryptor = HsmDecryptor { hsm_client: my_hsm };

    let result = Decoder::new(qr_data)
        .decrypt_with(&decryptor)?
        .verify_with_ed25519(&public_key)?
        .decode()?;
    ```

=== "Python"

    ```python
    from claim169 import decode_with_decryptor_and_verifier

    def my_decryptor(algorithm: str, key_id: bytes | None, ciphertext: bytes, aad: bytes) -> bytes:
        """
        Custom decryptor callback for HSM/KMS integration.

        Args:
            algorithm: COSE algorithm name ("A256GCM", "A128GCM", etc.)
            key_id: Optional key identifier from COSE header
            ciphertext: The encrypted data (ciphertext with auth tag)
            aad: Additional authenticated data for AEAD

        Returns:
            Decrypted plaintext

        Raises:
            Exception: If decryption fails (e.g., auth tag mismatch)
        """
        # Example: AWS KMS
        # response = kms_client.decrypt(
        #     KeyId='alias/my-key',
        #     CiphertextBlob=ciphertext,
        # )
        # return response['Plaintext']

        # Example: PKCS#11 HSM
        return my_hsm.decrypt(key_id, ciphertext, aad)

    def my_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes):
        my_hsm.verify(key_id, data, signature)

    result = decode_with_decryptor_and_verifier(qr_data, my_decryptor, my_verifier)
    ```

=== "TypeScript"

    ```typescript
    import { Decoder, DecryptorCallback, VerifierCallback } from 'claim169';

    const myDecryptor: DecryptorCallback = async (
      algorithm: string,
      keyId: Uint8Array | null,
      ciphertext: Uint8Array,
      aad: Uint8Array
    ): Promise<Uint8Array> => {
      // Example: Google Cloud KMS
      // const [decryptResponse] = await kmsClient.decrypt({
      //   name: keyName,
      //   ciphertext: ciphertext,
      //   additionalAuthenticatedData: aad,
      // });
      // return new Uint8Array(decryptResponse.plaintext);

      // Example: Azure Key Vault
      // const result = await cryptoClient.decrypt("A256GCM", ciphertext);
      // return result.result;

      // Your HSM decryption - throw on failure
      return myHsm.decrypt(keyId, ciphertext, aad);
    };

    const result = new Decoder(qrData)
      .decryptWith(myDecryptor)
      .verifyWith(myVerifier)
      .decode();
    ```

!!! warning "Vérification du Tag d'Authentification"
    AES-GCM inclut un tag d'authentification qui garantit l'intégrité des données. Si votre HSM retourne une erreur lors du déchiffrement (par ex., "échec d'authentification"), propagez cette erreur plutôt que de retourner des données corrompues.

## Générer des clés AES

```python
import secrets
aes256_key = secrets.token_bytes(32)
```

Voir aussi :

- [Matériel de clés et formats](keys.md)
- [Sécurité et validations](security.md)
