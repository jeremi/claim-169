# Matériel de clés et formats

Cette page explique quel matériel de clé la bibliothèque attend (octets bruts vs PEM) et comment cela se mappe aux opérations MOSIP Claim 169.

## Quelles clés sont utilisées ?

- **Signature (authenticité)** : Ed25519 (COSE `EdDSA`) ou ECDSA P-256 (COSE `ES256`)
- **Chiffrement (confidentialité, optionnel)** : AES-GCM (COSE `A256GCM` ou `A128GCM`)

!!! warning "Gestion des clés en production"
    Les clés de signature et de chiffrement sont des secrets critiques. En production, conservez-les dans un HSM/KMS et utilisez les mécanismes « crypto personnalisée » (là où ils existent) plutôt que de charger des clés privées brutes en mémoire applicative.

## Formats de clés par algorithme

### Ed25519

- **Clé publique** : 32 octets
- **Clé privée** : 32 octets (seed)

Dans la crate Rust (feature `software-crypto`, activée par défaut), le décodeur supporte aussi les clés publiques **PEM/SPKI** :

```rust
use claim169_core::Decoder;

let result = Decoder::new(qr_text)
    .verify_with_ed25519_pem(ed25519_public_key_pem)?
    .decode()?;
```

### ECDSA P-256 (ES256)

- **Clé publique** : point SEC1 encodé, soit :
  - **33 octets** (compressé, commence par `0x02` ou `0x03`), ou
  - **65 octets** (non compressé, commence par `0x04`)
- **Clé privée** : scalaire de 32 octets

Rust supporte aussi les clés publiques **PEM/SPKI** :

```rust
use claim169_core::Decoder;

let result = Decoder::new(qr_text)
    .verify_with_ecdsa_p256_pem(p256_public_key_pem)?
    .decode()?;
```

### AES-GCM (A256GCM / A128GCM)

- **Clé AES-256-GCM** : 32 octets
- **Clé AES-128-GCM** : 16 octets
- **Nonce/IV** : 12 octets (aléatoire à chaque chiffrement)

En usage normal, vous n’avez **pas** besoin de fournir un nonce : l’encodeur génère un nonce aléatoire automatiquement.

!!! danger "La réutilisation de nonce casse la sécurité"
    Ne réutilisez jamais un nonce AES-GCM avec la même clé. N’utilisez des APIs « nonce explicite » que pour des tests.

## Générer des clés de développement (Rust)

Avec la feature `software-crypto` (par défaut), vous pouvez générer des clés temporaires pour vos tests locaux :

```rust
use claim169_core::{Ed25519Signer, EcdsaP256Signer};

let ed_signer = Ed25519Signer::generate();
let ed_public_key: [u8; 32] = ed_signer.public_key_bytes();

let p256_signer = EcdsaP256Signer::generate();
let p256_public_key_uncompressed: Vec<u8> = p256_signer.public_key_uncompressed(); // 65 octets
```

## Générer des clés AES (Python / TypeScript)

=== "Python"

    ```python
    import secrets

    aes256_key = secrets.token_bytes(32)
    aes128_key = secrets.token_bytes(16)
    ```

=== "TypeScript"

    ```ts
    // Navigateur
    const aes256Key = crypto.getRandomValues(new Uint8Array(32));

    // Node.js
    import { randomBytes } from "crypto";
    const aes256KeyNode = randomBytes(32);
    ```

## Intégration HSM et KMS

Pour les déploiements en production, les clés cryptographiques doivent être stockées dans des Modules de Sécurité Matérielle (HSM) ou des Services de Gestion de Clés (KMS) cloud plutôt que chargées sous forme d'octets bruts en mémoire applicative.

### Fournisseurs Cryptographiques Personnalisés

La bibliothèque supporte des callbacks de fournisseurs cryptographiques personnalisés pour toutes les opérations cryptographiques :

| Opération | Type de Callback | Cas d'utilisation |
|-----------|------------------|-------------------|
| Signature | `Signer` / `SignerCallback` | Émission de credentials avec clés protégées par HSM |
| Vérification | `SignatureVerifier` / `VerifierCallback` | Vérification de credentials avec clés publiques gérées |
| Chiffrement | `Encryptor` / `EncryptorCallback` | Protection de la confidentialité avec clés encapsulées par HSM |
| Déchiffrement | `Decryptor` / `DecryptorCallback` | Lecture de credentials avec clés protégées par HSM |

### Interface de Callback

Tous les callbacks reçoivent :

- **`algorithm`** : Nom de l'algorithme COSE (`"EdDSA"`, `"ES256"`, `"A256GCM"`, etc.)
- **`key_id`** : Octets optionnels d'identification de clé provenant de l'en-tête COSE

Les paramètres additionnels dépendent de l'opération :

| Opération | Paramètres Additionnels | Valeur de Retour |
|-----------|-------------------------|------------------|
| Sign | `data` (octets à signer) | Octets de signature |
| Verify | `data`, `signature` | Aucun (exception en cas d'échec) |
| Encrypt | `plaintext`, `aad` | Texte chiffré avec tag d'authentification |
| Decrypt | `ciphertext`, `aad` | Texte en clair (exception en cas d'échec) |

### Fournisseurs KMS Supportés

L'interface crypto personnalisée fonctionne avec tout système de gestion de clés :

| Fournisseur | Signature | Chiffrement | Notes |
|-------------|-----------|-------------|-------|
| AWS KMS | ES256, EdDSA | AES-GCM | Utiliser `kms:Sign`, `kms:Verify`, `kms:Encrypt`, `kms:Decrypt` |
| Google Cloud KMS | ES256, EdDSA | AES-GCM | Signature asymétrique + chiffrement symétrique |
| Azure Key Vault | ES256, EdDSA | AES-GCM | Utiliser le client Cryptography |
| HashiCorp Vault | ES256, EdDSA | AES-GCM | Moteur de secrets Transit |
| HSM PKCS#11 | Tous les algorithmes | Tous les algorithmes | Clés adossées au matériel |
| TPM 2.0 | ES256, EdDSA | AES-GCM | Clés liées à la plateforme |
| Cartes à puce | ES256 | N/A | Cartes PIV/CAC |

### Exemple : AWS KMS

=== "Python"

    ```python
    import boto3
    from claim169 import encode_with_signer, decode_with_verifier

    kms = boto3.client('kms')
    KEY_ID = 'alias/claim169-signing-key'

    def aws_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
        response = kms.sign(
            KeyId=KEY_ID,
            Message=data,
            MessageType='RAW',
            SigningAlgorithm='ECDSA_SHA_256'  # For ES256
        )
        return response['Signature']

    def aws_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes):
        kms.verify(
            KeyId=KEY_ID,
            Message=data,
            MessageType='RAW',
            Signature=signature,
            SigningAlgorithm='ECDSA_SHA_256'
        )
        # Raises InvalidSignatureException on failure

    # Encode with AWS KMS
    qr_data = encode_with_signer(claim, meta, aws_signer, "ES256")

    # Decode with AWS KMS
    result = decode_with_verifier(qr_data, aws_verifier)
    ```

=== "TypeScript"

    ```typescript
    import { KMSClient, SignCommand, VerifyCommand } from '@aws-sdk/client-kms';
    import { Encoder, Decoder, SignerCallback, VerifierCallback } from 'claim169';

    const kms = new KMSClient({ region: 'us-east-1' });
    const KEY_ID = 'alias/claim169-signing-key';

    const awsSigner: SignerCallback = async (algorithm, keyId, data) => {
      const command = new SignCommand({
        KeyId: KEY_ID,
        Message: data,
        MessageType: 'RAW',
        SigningAlgorithm: 'ECDSA_SHA_256',
      });
      const response = await kms.send(command);
      return new Uint8Array(response.Signature!);
    };

    const awsVerifier: VerifierCallback = async (algorithm, keyId, data, signature) => {
      const command = new VerifyCommand({
        KeyId: KEY_ID,
        Message: data,
        MessageType: 'RAW',
        Signature: signature,
        SigningAlgorithm: 'ECDSA_SHA_256',
      });
      const response = await kms.send(command);
      if (!response.SignatureValid) {
        throw new Error('Signature verification failed');
      }
    };

    // Encode with AWS KMS
    const qrData = new Encoder(claim, meta)
      .signWith(awsSigner, "ES256")
      .encode();

    // Decode with AWS KMS
    const result = new Decoder(qrData)
      .verifyWith(awsVerifier)
      .decode();
    ```

### Exemple : Google Cloud KMS

=== "Python"

    ```python
    from google.cloud import kms

    client = kms.KeyManagementServiceClient()
    KEY_VERSION = 'projects/my-project/locations/global/keyRings/my-ring/cryptoKeys/my-key/cryptoKeyVersions/1'

    def gcp_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
        response = client.asymmetric_sign(
            request={'name': KEY_VERSION, 'data': data}
        )
        return response.signature

    def gcp_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes):
        response = client.asymmetric_verify(
            request={'name': KEY_VERSION, 'data': data, 'signature': signature}
        )
        if not response.success:
            raise ValueError('Signature verification failed')
    ```

### Exemple : Azure Key Vault

=== "Python"

    ```python
    from azure.identity import DefaultAzureCredential
    from azure.keyvault.keys.crypto import CryptographyClient, SignatureAlgorithm

    credential = DefaultAzureCredential()
    crypto_client = CryptographyClient(
        'https://my-vault.vault.azure.net/keys/my-key/version',
        credential
    )

    def azure_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
        result = crypto_client.sign(SignatureAlgorithm.es256, data)
        return result.signature

    def azure_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes):
        result = crypto_client.verify(SignatureAlgorithm.es256, data, signature)
        if not result.is_valid:
            raise ValueError('Signature verification failed')
    ```

### Rotation des Clés

Utilisez le champ `key_id` dans l'en-tête COSE pour supporter la rotation des clés :

```python
from claim169 import encode_with_signer

# Include key version in the credential
def rotating_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
    # key_id contains the key version identifier
    current_key = key_store.get_current_signing_key()
    return current_key.sign(data)

# The verifier can use key_id to look up the correct public key
def rotating_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes):
    if key_id:
        key_version = key_id.decode('utf-8')
        public_key = key_store.get_public_key(key_version)
    else:
        public_key = key_store.get_default_public_key()
    public_key.verify(data, signature)
```

!!! tip "Bonnes Pratiques"
    - Stocker les clés de signature dans un HSM avec permission `sign` uniquement (pas d'export)
    - Utiliser des clés distinctes pour la signature et le chiffrement
    - Implémenter la rotation des clés avec des périodes de validité chevauchantes
    - Journaliser toutes les opérations cryptographiques pour les pistes d'audit
    - Utiliser des alias de clés plutôt que des IDs bruts dans le code applicatif

## Vecteurs de test

Pour des exemples de clés connues (uniquement pour des tests), voir `test-vectors/valid/*.json`. Ces vecteurs incluent `public_key_hex` et (pour certains) `private_key_hex`.

