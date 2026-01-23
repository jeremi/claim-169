# Décodage et vérification

Ce guide couvre le décodage des identifiants Claim 169 et la vérification de leur authenticité.

## Pipeline de décodage

Lors du décodage, les données passent par ces étapes :

```
Code QR → Base45 → zlib → COSE → CWT → Claim 169
```

## Décodage basique

### Avec vérification (Production)

Vérifiez toujours les signatures en production :

=== "Python"

    ```python
    from claim169 import decode_with_ed25519

    result = decode_with_ed25519(qr_data, public_key)

    # Accéder aux données d'identité
    print(f"ID : {result.claim169.id}")
    print(f"Nom : {result.claim169.full_name}")

    # Accéder aux métadonnées CWT
    print(f"Émetteur : {result.cwt_meta.issuer}")
    print(f"Expire : {result.cwt_meta.expires_at}")
    ```

=== "Rust"

    ```rust
    use claim169_core::Decoder;

    let result = Decoder::new(qr_text)
        .verify_with_ed25519(&public_key)?
        .decode()?;
    ```

=== "TypeScript"

    ```ts
    import { Decoder } from "claim169";

    const result = new Decoder(qrText)
      .verifyWithEd25519(publicKey)
      .decode();
    ```

### Sans vérification (tests uniquement)

!!! danger "Avertissement"
    Sans vérification de signature, un QR code peut être falsifié.

=== "Python"

    ```python
    import claim169

    result = claim169.decode_unverified(qr_data)
    ```

## Identifiants chiffrés

Pour les payloads chiffrés, il faut **déchiffrer avant de vérifier**.

=== "Python"

    ```python
    from claim169 import decode_encrypted_aes

    # Test uniquement : déchiffre sans vérifier la signature interne
    result = decode_encrypted_aes(qr_data, encryption_key, allow_unverified=True)
    ```

## Méthodes de vérification

### Ed25519

Ed25519 utilise des clés publiques de 32 octets :

```python
# La clé publique doit faire exactement 32 octets
public_key = bytes.fromhex("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
result = decode_with_ed25519(qr_data, public_key)
```

### ECDSA P-256

ECDSA P-256 accepte les clés publiques compressées (33 octets) ou non compressées (65 octets) :

```python
# Clé publique compressée (33 octets, commence par 02 ou 03)
compressed_key = bytes.fromhex("02...")

# Clé publique non compressée (65 octets, commence par 04)
uncompressed_key = bytes.fromhex("04...")

result = decode_with_ecdsa_p256(qr_data, compressed_key)
```

### Vérificateur Personnalisé (HSM/KMS)

Pour les environnements de production où les clés sont gérées en externe, utilisez un callback de vérification personnalisé pour intégrer des modules de sécurité matériels (HSM), des services de gestion de clés cloud (AWS KMS, Google Cloud KMS, Azure Key Vault), des cartes à puce, des TPM ou des services de vérification distants.

Le callback reçoit :

- `algorithm` : Le nom de l'algorithme COSE (ex. `"EdDSA"`, `"ES256"`)
- `key_id` : Identifiant de clé optionnel en octets (depuis l'en-tête COSE, si présent)
- `data` : Les données signées (COSE `Sig_structure`)
- `signature` : Les octets de la signature à vérifier

Le callback doit lever une exception si la vérification échoue. Un retour réussi (sans exception) indique une signature valide.

=== "Rust"

    ```rust
    use claim169_core::{Decoder, SignatureVerifier};

    struct HsmVerifier {
        hsm_client: MyHsmClient,
    }

    impl SignatureVerifier for HsmVerifier {
        fn verify(
            &self,
            algorithm: &str,
            key_id: Option<&[u8]>,
            data: &[u8],
            signature: &[u8],
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            // Use key_id to locate the correct public key in your HSM
            let key_name = key_id
                .map(|id| String::from_utf8_lossy(id).to_string())
                .unwrap_or_else(|| "default-key".to_string());

            // Call your HSM to verify the signature
            self.hsm_client.verify(&key_name, data, signature)?;
            Ok(())
        }
    }

    let verifier = HsmVerifier { hsm_client: my_hsm };

    let result = Decoder::new(qr_data)
        .verify_with(&verifier)?
        .decode()?;
    ```

=== "Python"

    ```python
    from claim169 import decode_with_verifier

    def my_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes):
        """
        Callback de vérification personnalisé pour intégration HSM/KMS.

        Args:
            algorithm: Nom de l'algorithme COSE ("EdDSA", "ES256", etc.)
            key_id: Identifiant de clé optionnel depuis l'en-tête COSE
            data: Les données signées (COSE Sig_structure)
            signature: Les octets de la signature à vérifier

        Raises:
            Exception: Si la vérification de signature échoue
        """
        # Exemple : AWS KMS
        # kms_client.verify(
        #     KeyId='alias/my-verification-key',
        #     Message=data,
        #     Signature=signature,
        #     SigningAlgorithm='ECDSA_SHA_256'
        # )

        # Exemple : HSM PKCS#11
        # Lève une exception en cas d'échec, retourne None en cas de succès
        my_hsm.verify(key_id, data, signature)

    result = decode_with_verifier(qr_data, my_verifier)
    ```

=== "TypeScript"

    ```typescript
    import { Decoder, VerifierCallback } from 'claim169';

    const myVerifier: VerifierCallback = async (
      algorithm: string,
      keyId: Uint8Array | null,
      data: Uint8Array,
      signature: Uint8Array
    ): Promise<void> => {
      // Exemple : Google Cloud KMS
      // const [verifyResponse] = await kmsClient.asymmetricVerify({
      //   name: keyVersionName,
      //   data: data,
      //   signature: signature,
      // });
      // if (!verifyResponse.success) {
      //   throw new Error('Signature verification failed');
      // }

      // Exemple : Azure Key Vault
      // const result = await cryptoClient.verify("ES256", data, signature);
      // if (!result.result) {
      //   throw new Error('Signature verification failed');
      // }

      // Votre vérification HSM - lever une exception en cas d'échec
      myHsm.verify(keyId, data, signature);
    };

    const result = new Decoder(qrData)
      .verifyWith(myVerifier)
      .decode();
    ```

!!! tip "Recherche de clé"
    Utilisez le paramètre `key_id` pour rechercher la bonne clé publique dans votre système de gestion de clés. Cela permet la rotation des clés et les scénarios multi-émetteurs où différents identifiants peuvent être signés avec différentes clés.

Voir aussi :

- [Chiffrement](encryption.md)
- [Matériel de clés et formats](keys.md)
- [Sécurité et validations](security.md)
