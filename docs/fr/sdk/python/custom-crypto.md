# Fournisseurs crypto personnalisés

Ce guide couvre l’intégration de fournisseurs cryptographiques externes (Hardware Security Modules / HSM, et services cloud de gestion de clés / KMS).

## Vue d’ensemble

La bibliothèque claim169 fournit des hooks crypto basés sur des callbacks, qui permettent :

- **Signer** avec des clés stockées dans un HSM ou un KMS cloud
- **Vérifier** des signatures via des fournisseurs externes
- **Chiffrer** avec des clés gérées en dehors de l’application
- **Déchiffrer** via un HSM ou un KMS

Cela peut aider à respecter des exigences de sécurité imposant que les clés privées restent dans du matériel sécurisé (selon votre fournisseur et votre configuration).

## Interfaces de callbacks

### Callback de signature (signer)

```python
def signer_callback(
    algorithm: str,      # "EdDSA" ou "ES256"
    key_id: bytes | None,  # Identifiant de clé optionnel
    data: bytes          # Données à signer
) -> bytes:              # Octets de signature
    ...
```

### Callback de vérification (verifier)

```python
def verifier_callback(
    algorithm: str,      # "EdDSA" ou "ES256"
    key_id: bytes | None,  # Identifiant de clé optionnel
    data: bytes,         # Données signées
    signature: bytes     # Signature à vérifier
) -> None:               # Lever une exception si invalide
    ...
```

### Callback de chiffrement (encryptor)

```python
def encryptor_callback(
    algorithm: str,      # "A256GCM" ou "A128GCM"
    key_id: bytes | None,  # Identifiant de clé optionnel
    nonce: bytes,        # Nonce de 12 octets
    aad: bytes,          # Additional authenticated data
    plaintext: bytes     # Données à chiffrer
) -> bytes:              # Ciphertext avec tag d’authentification
    ...
```

### Callback de déchiffrement (decryptor)

```python
def decryptor_callback(
    algorithm: str,      # "A256GCM" ou "A128GCM"
    key_id: bytes | None,  # Identifiant de clé optionnel
    nonce: bytes,        # Nonce de 12 octets
    aad: bytes,          # Additional authenticated data
    ciphertext: bytes    # Données à déchiffrer
) -> bytes:              # Texte clair déchiffré
    ...
```

## Intégration AWS KMS

### Setup

```bash
pip install boto3
```

### Signer avec AWS KMS

```python
import boto3
import claim169

kms_client = boto3.client('kms', region_name='us-east-1')
KEY_ID = 'arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012'

def aws_kms_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
    """Signer via AWS KMS."""
    # AWS KMS supporte ECDSA_SHA_256 pour ES256
    # Pour EdDSA, il faut une autre approche
    if algorithm != "ES256":
        raise ValueError(f"AWS KMS signer only supports ES256, got {algorithm}")

    response = kms_client.sign(
        KeyId=KEY_ID,
        Message=data,
        MessageType='RAW',
        SigningAlgorithm='ECDSA_SHA_256'
    )

    # AWS KMS retourne une signature DER ; la convertir en raw r||s
    der_sig = response['Signature']
    return der_to_raw_ecdsa(der_sig)


def der_to_raw_ecdsa(der_sig: bytes) -> bytes:
    """Convertir une signature DER vers le format raw r||s (64 octets)."""
    from cryptography.hazmat.primitives.asymmetric.utils import decode_dss_signature
    r, s = decode_dss_signature(der_sig)
    return r.to_bytes(32, 'big') + s.to_bytes(32, 'big')


# Encoder un identifiant avec signature AWS KMS
claim = claim169.Claim169Input(id="AWS-KMS-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(issuer="https://id.example.org")

qr_data = claim169.encode_with_signer(
    claim,
    meta,
    aws_kms_signer,
    "ES256",
    key_id=KEY_ID.encode()
)
```

### Vérifier avec AWS KMS

```python
def aws_kms_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes) -> None:
    """Vérifier une signature via AWS KMS."""
    # Convertir la signature raw r||s vers DER
    raw_sig = signature
    r = int.from_bytes(raw_sig[:32], 'big')
    s = int.from_bytes(raw_sig[32:], 'big')

    from cryptography.hazmat.primitives.asymmetric.utils import encode_dss_signature
    der_sig = encode_dss_signature(r, s)

    response = kms_client.verify(
        KeyId=KEY_ID,
        Message=data,
        MessageType='RAW',
        Signature=der_sig,
        SigningAlgorithm='ECDSA_SHA_256'
    )

    if not response['SignatureValid']:
        raise ValueError("Signature verification failed")


result = claim169.decode_with_verifier(qr_data, aws_kms_verifier)
```

### Chiffrement avec AWS KMS

```python
def aws_kms_encryptor(algorithm: str, key_id: bytes | None, nonce: bytes, aad: bytes, plaintext: bytes) -> bytes:
    """Chiffrer en utilisant une data key AWS KMS."""
    # Générer une data key pour du chiffrement enveloppe
    response = kms_client.generate_data_key(
        KeyId=KEY_ID,
        KeySpec='AES_256'
    )

    # Chiffrer localement avec la clé en clair
    from cryptography.hazmat.primitives.ciphers.aead import AESGCM
    aesgcm = AESGCM(response['Plaintext'])
    ciphertext = aesgcm.encrypt(nonce, plaintext, aad)

    # En pratique, stocker response['CiphertextBlob'] avec les données chiffrées
    return ciphertext
```

## Intégration Azure Key Vault

### Setup

```bash
pip install azure-identity azure-keyvault-keys azure-keyvault-secrets
```

### Signer avec Azure Key Vault

```python
from azure.identity import DefaultAzureCredential
from azure.keyvault.keys import KeyClient
from azure.keyvault.keys.crypto import CryptographyClient, SignatureAlgorithm

credential = DefaultAzureCredential()
vault_url = "https://my-vault.vault.azure.net/"
key_name = "my-signing-key"

key_client = KeyClient(vault_url=vault_url, credential=credential)
key = key_client.get_key(key_name)
crypto_client = CryptographyClient(key, credential=credential)


def azure_kv_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
    """Signer via Azure Key Vault."""
    import hashlib

    if algorithm == "ES256":
        # Azure exige des données pré-hachées pour ECDSA
        digest = hashlib.sha256(data).digest()
        result = crypto_client.sign(SignatureAlgorithm.es256, digest)
        return result.signature
    else:
        raise ValueError(f"Unsupported algorithm: {algorithm}")


# Encoder avec signature Azure Key Vault
claim = claim169.Claim169Input(id="AZURE-KV-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(issuer="https://id.example.org")

qr_data = claim169.encode_with_signer(
    claim,
    meta,
    azure_kv_signer,
    "ES256"
)
```

### Vérifier avec Azure Key Vault

```python
def azure_kv_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes) -> None:
    """Vérifier via Azure Key Vault."""
    import hashlib

    digest = hashlib.sha256(data).digest()
    result = crypto_client.verify(SignatureAlgorithm.es256, digest, signature)

    if not result.is_valid:
        raise ValueError("Signature verification failed")


result = claim169.decode_with_verifier(qr_data, azure_kv_verifier)
```

## Intégration Google Cloud KMS

### Setup

```bash
pip install google-cloud-kms
```

### Signer avec Google Cloud KMS

```python
from google.cloud import kms

client = kms.KeyManagementServiceClient()
key_name = client.crypto_key_version_path(
    'my-project',
    'us-east1',
    'my-keyring',
    'my-key',
    '1'
)


def gcp_kms_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
    """Signer via Google Cloud KMS."""
    import hashlib
    from google.cloud.kms import CryptoKeyVersion

    if algorithm != "ES256":
        raise ValueError(f"GCP KMS signer configured for ES256, got {algorithm}")

    # GCP exige un digest SHA256 pour EC_SIGN_P256_SHA256
    digest = {'sha256': hashlib.sha256(data).digest()}

    response = client.asymmetric_sign(
        request={'name': key_name, 'digest': digest}
    )

    # GCP retourne DER ; convertir en raw
    return der_to_raw_ecdsa(response.signature)


def der_to_raw_ecdsa(der_sig: bytes) -> bytes:
    """Convertir une signature DER vers le format raw r||s."""
    from cryptography.hazmat.primitives.asymmetric.utils import decode_dss_signature
    r, s = decode_dss_signature(der_sig)
    return r.to_bytes(32, 'big') + s.to_bytes(32, 'big')


# Encoder avec signature GCP KMS
claim = claim169.Claim169Input(id="GCP-KMS-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(issuer="https://id.example.org")

qr_data = claim169.encode_with_signer(
    claim,
    meta,
    gcp_kms_signer,
    "ES256"
)
```

### Vérifier avec Google Cloud KMS

```python
def gcp_kms_verifier(algorithm: str, key_id: bytes | None, data: bytes, signature: bytes) -> None:
    """Vérifier via Google Cloud KMS."""
    import hashlib
    from cryptography.hazmat.primitives.asymmetric.utils import encode_dss_signature

    # Convertir raw r||s vers DER pour GCP
    r = int.from_bytes(signature[:32], 'big')
    s = int.from_bytes(signature[32:], 'big')
    der_sig = encode_dss_signature(r, s)

    digest = {'sha256': hashlib.sha256(data).digest()}

    response = client.asymmetric_verify(
        request={
            'name': key_name,
            'digest': digest,
            'signature': der_sig
        }
    )

    if not response.success:
        raise ValueError("Signature verification failed")


result = claim169.decode_with_verifier(qr_data, gcp_kms_verifier)
```

## Intégration HashiCorp Vault

### Setup

```bash
pip install hvac
```

### Signer avec HashiCorp Vault Transit

```python
import hvac

client = hvac.Client(url='https://vault.example.org:8200')
client.token = 'your-vault-token'


def vault_transit_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
    """Signer via HashiCorp Vault Transit."""
    import base64

    # Vault attend une entrée en base64
    input_b64 = base64.b64encode(data).decode()

    response = client.secrets.transit.sign_data(
        name='my-signing-key',
        hash_input=input_b64,
        signature_algorithm='pkcs1v15' if algorithm == 'ES256' else 'ed25519'
    )

    # Extraire et décoder la signature
    sig_b64 = response['data']['signature'].split(':')[-1]
    return base64.b64decode(sig_b64)


# Encoder avec signature Vault
claim = claim169.Claim169Input(id="VAULT-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(issuer="https://id.example.org")

qr_data = claim169.encode_with_signer(
    claim,
    meta,
    vault_transit_signer,
    "EdDSA"
)
```

## Intégration PKCS#11 / HSM

### Setup

```bash
pip install python-pkcs11
```

### Signer avec un HSM PKCS#11

```python
import pkcs11
from pkcs11 import KeyType, Mechanism

# Charger la bibliothèque PKCS#11
lib = pkcs11.lib('/usr/lib/softhsm/libsofthsm2.so')
token = lib.get_token(token_label='MyHSM')


def pkcs11_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
    """Signer via un HSM PKCS#11."""
    with token.open(user_pin='1234') as session:
        # Trouver la clé privée
        private_key = session.get_key(
            key_type=KeyType.EC,
            object_class=pkcs11.ObjectClass.PRIVATE_KEY,
            label='my-signing-key'
        )

        # Signer les données
        if algorithm == "ES256":
            signature = private_key.sign(
                data,
                mechanism=Mechanism.ECDSA_SHA256
            )
        else:
            raise ValueError(f"Unsupported algorithm: {algorithm}")

        return signature


# Encoder avec signature HSM
claim = claim169.Claim169Input(id="HSM-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(issuer="https://id.example.org")

qr_data = claim169.encode_with_signer(
    claim,
    meta,
    pkcs11_signer,
    "ES256"
)
```

## Exemple complet : AWS KMS (signer + chiffrer)

```python
import boto3
import secrets
import claim169
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

# Configuration AWS KMS
kms_client = boto3.client('kms', region_name='us-east-1')
SIGN_KEY_ID = 'arn:aws:kms:...:key/sign-key-id'
ENCRYPT_KEY_ID = 'arn:aws:kms:...:key/encrypt-key-id'


def aws_signer(algorithm, key_id, data):
    """Signer via AWS KMS."""
    response = kms_client.sign(
        KeyId=SIGN_KEY_ID,
        Message=data,
        MessageType='RAW',
        SigningAlgorithm='ECDSA_SHA_256'
    )
    return der_to_raw_ecdsa(response['Signature'])


def aws_encryptor(algorithm, key_id, nonce, aad, plaintext):
    """Chiffrer via chiffrement enveloppe AWS KMS."""
    # Générer une data key
    response = kms_client.generate_data_key(
        KeyId=ENCRYPT_KEY_ID,
        KeySpec='AES_256'
    )

    # Chiffrer localement avec la data key
    aesgcm = AESGCM(response['Plaintext'])
    return aesgcm.encrypt(nonce, plaintext, aad)


def aws_verifier(algorithm, key_id, data, signature):
    """Vérifier via AWS KMS."""
    r = int.from_bytes(signature[:32], 'big')
    s = int.from_bytes(signature[32:], 'big')

    from cryptography.hazmat.primitives.asymmetric.utils import encode_dss_signature
    der_sig = encode_dss_signature(r, s)

    response = kms_client.verify(
        KeyId=SIGN_KEY_ID,
        Message=data,
        MessageType='RAW',
        Signature=der_sig,
        SigningAlgorithm='ECDSA_SHA_256'
    )
    if not response['SignatureValid']:
        raise ValueError("Verification failed")


def aws_decryptor(algorithm, key_id, nonce, aad, ciphertext):
    """Déchiffrer avec une data key stockée localement."""
    # En pratique, récupérer la data key chiffrée puis la déchiffrer d’abord
    # Exemple simplifié
    data_key = get_cached_data_key()  # Votre implémentation
    aesgcm = AESGCM(data_key)
    return aesgcm.decrypt(nonce, ciphertext, aad)


# Encoder
claim = claim169.Claim169Input(id="AWS-FULL-001", full_name="Jane Doe")
meta = claim169.CwtMetaInput(issuer="https://id.example.org")

qr_data = claim169.encode_with_signer_and_encryptor(
    claim,
    meta,
    aws_signer,
    "ES256",
    aws_encryptor,
    "A256GCM"
)

# Décoder
result = claim169.decode_with_decryptor(
    qr_data,
    aws_decryptor,
    verifier=aws_verifier
)

print(f"Verified: {result.is_verified()}")
```

## Gestion des erreurs

```python
import claim169

def safe_signer(algorithm, key_id, data):
    try:
        return external_crypto_provider.sign(data)
    except ConnectionError as e:
        raise RuntimeError(f"Crypto provider unavailable: {e}")
    except PermissionError as e:
        raise RuntimeError(f"Access denied to signing key: {e}")


try:
    qr_data = claim169.encode_with_signer(claim, meta, safe_signer, "EdDSA")
except claim169.Claim169Exception as e:
    print(f"Encoding failed: {e}")
```

## Bonnes pratiques

### Rotation de clés

- Mettre en place du versioning de clé dans votre KMS
- Utiliser le paramètre `key_id` pour suivre les versions
- Prévoir une rotation sans interruption

### Gestion des erreurs

- Capturer et encapsuler les exceptions spécifiques au fournisseur
- Implémenter des retries pour les erreurs transitoires
- Journaliser les opérations crypto pour l’audit

### Performance

- Mettre en cache les clients KMS et les connexions
- Envisager le pooling de connexions pour les HSM
- Utiliser des APIs async lorsqu’elles existent

### Sécurité

- Utiliser des rôles IAM / managed identities, pas des identifiants statiques
- Activer la journalisation d’audit côté KMS
- Mettre en place des contrôles d’accès appropriés

## Étapes suivantes

- [Référence API](api.md) — documentation complète des fonctions
- [Dépannage](troubleshooting.md) — erreurs fréquentes et solutions
