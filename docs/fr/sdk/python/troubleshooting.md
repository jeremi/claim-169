# Dépannage

Erreurs fréquentes et solutions pour le SDK Python claim169.

## Erreurs d’import

### Module introuvable

```
ModuleNotFoundError: No module named 'claim169'
```

**Solution :**

1. Vérifier l’installation :
   ```bash
   pip show claim169
   ```

2. Vérifier que vous utilisez le bon interpréteur Python :
   ```bash
   which python
   python -c "import sys; print(sys.executable)"
   ```

3. Réinstaller :
   ```bash
   pip uninstall claim169
   pip install claim169
   ```

### Erreur de bibliothèque à l’import

```
ImportError: libssl.so.1.1: cannot open shared object file
```

**Solution :** installer les bibliothèques OpenSSL :

```bash
# Ubuntu/Debian
sudo apt-get install libssl-dev

# CentOS/RHEL
sudo yum install openssl-devel

# macOS
brew install openssl
```

---

## Erreurs de décodage

### Base45DecodeError

```python
claim169.Base45DecodeError: Invalid Base45 character at position 15
```

**Causes :**
- Le QR code n’a pas été scanné entièrement
- Le contenu du QR code a été tronqué
- L’entrée n’est pas un QR code Claim 169

**Solutions :**

1. Vérifier que le QR code a été scanné complètement
2. Vérifier les espaces avant/après :
   ```python
   qr_data = qr_data.strip()
   result = claim169.decode_with_ed25519(qr_data, public_key)
   ```
3. Vérifier que le QR code est bien un identifiant Claim 169

### DecompressError

```python
claim169.DecompressError: zlib decompression failed
```

**Causes :**
- Données QR code corrompues
- Données modifiées après encodage
- Identifiant Claim 169 invalide

**Solutions :**

1. Re-scanner le QR code
2. Vérifier une corruption lors de la transmission
3. Vérifier la source du QR code

### DecompressError : dépassement de limite

```python
claim169.DecompressError: decompressed size 150000 exceeds limit 65536
```

**Cause :** l’identifiant se décompresse au-delà de la limite (64KB par défaut).

**Solution :** augmenter la limite si vous faites confiance à la source :

```python
result = claim169.decode_with_ed25519(
    qr_data,
    public_key,
    max_decompressed_bytes=200000  # Limite 200KB
)
```

### CoseParseError

```python
claim169.CoseParseError: Invalid COSE structure
```

**Causes :**
- Ce n’est pas un identifiant encodé COSE
- Structure COSE corrompue
- Mauvais type de QR code

**Solution :** vérifier que le QR code est un identifiant Claim 169, pas un autre format (p. ex. EU DCC, SMART Health Card).

### Claim169NotFoundError

```python
claim169.Claim169NotFoundError: Claim 169 not found in CWT
```

**Cause :** la structure COSE/CWT est valide, mais ne contient pas de charge utile Claim 169.

**Solution :** ce QR code utilise un autre format de claim. Vérifier que vous scannez bien un identifiant MOSIP Claim 169.

### SignatureError

```python
claim169.SignatureError: Signature verification failed
```

**Causes :**
- Mauvaise clé publique
- Identifiant altéré
- Incohérence clé/algorithme

**Solutions :**

1. Vérifier que vous utilisez la bonne clé publique pour cet émetteur
2. Vérifier le format de clé (Ed25519 vs ECDSA P-256)
3. Vérifier la longueur de clé :
   - Ed25519 : 32 octets
   - ECDSA P-256 : 33 octets (compressée) ou 65 octets (non compressée)

```python
print(f"Key length: {len(public_key)} bytes")
# Ed25519 doit être 32
# ECDSA P-256 compressée doit être 33
# ECDSA P-256 non compressée doit être 65
```

### DecryptionError

```python
claim169.DecryptionError: Decryption failed
```

**Causes :**
- Mauvaise clé de déchiffrement
- Mauvaise taille de clé pour l’algorithme
- Ciphertext corrompu

**Solutions :**

1. Vérifier la taille de clé :
   ```python
   print(f"Key length: {len(encrypt_key)} bytes")
   # AES-256 : 32 octets
   # AES-128 : 16 octets
   ```

2. Utiliser la bonne fonction selon la taille :
   ```python
   # Pour des clés 32 octets
   result = claim169.decode_encrypted_aes256(qr_data, key_32, allow_unverified=True)

   # Pour des clés 16 octets
   result = claim169.decode_encrypted_aes128(qr_data, key_16, allow_unverified=True)
   ```

### Erreurs de validation d’horodatages

```python
claim169.Claim169Exception: Token expired at 1700000000
```

**Cause :** l’identifiant a expiré.

**Solutions :**

1. Vérifier s’il doit être rejeté (il est expiré)
2. Pour les tests, désactiver la validation des horodatages :
   ```python
   result = claim169.decode_with_ed25519(
       qr_data,
       public_key,
       validate_timestamps=False
   )
   ```

```python
claim169.Claim169Exception: Token not valid until 1800000000
```

**Cause :** le `nbf` (not before) est dans le futur.

**Solutions :**

1. Attendre que l’identifiant devienne valide
2. Vérifier la synchronisation des horloges
3. Ajouter une tolérance à la dérive :
   ```python
   result = claim169.decode_with_ed25519(
       qr_data,
       public_key,
       clock_skew_tolerance_seconds=300  # 5 minutes
   )
   ```

---

## Erreurs d’encodage

### Longueur de clé invalide

```python
ValueError: Ed25519 private key must be 32 bytes
```

**Solution :** s’assurer que la clé a la bonne longueur :

```python
private_key = bytes.fromhex("...")
print(f"Key length: {len(private_key)} bytes")  # Doit être 32

# Si vous utilisez cryptography :
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
private_key_obj = Ed25519PrivateKey.generate()
private_key = private_key_obj.private_bytes_raw()  # Exactement 32 octets
```

### Callback retourne le mauvais type

```python
claim169.Claim169Exception: signer callback must return bytes
```

**Cause :** votre callback signer/encryptor a renvoyé un type différent de `bytes`.

**Solution :** s’assurer que le callback renvoie `bytes` :

```python
def my_signer(algorithm, key_id, data):
    signature = some_signing_operation(data)
    return bytes(signature)  # S’assurer que c’est bytes
```

### Callback lève une exception

```python
claim169.Claim169Exception: RuntimeError: Crypto provider unavailable
```

**Cause :** votre callback a levé une exception.

**Solution :** gérer les erreurs dans le callback :

```python
def my_signer(algorithm, key_id, data):
    try:
        return crypto_provider.sign(data)
    except ConnectionError:
        # Logger l’erreur
        raise RuntimeError("Failed to connect to crypto provider")
```

---

## Erreurs de crypto personnalisée

### Erreurs AWS KMS

```python
botocore.exceptions.ClientError: AccessDeniedException
```

**Solution :** vérifier les permissions IAM sur la clé KMS :

```json
{
    "Effect": "Allow",
    "Action": [
        "kms:Sign",
        "kms:Verify",
        "kms:GenerateDataKey"
    ],
    "Resource": "arn:aws:kms:..."
}
```

### Erreurs Azure Key Vault

```python
azure.core.exceptions.ClientAuthenticationError
```

**Solution :** vérifier les credentials Azure :

```python
from azure.identity import DefaultAzureCredential
credential = DefaultAzureCredential()

# Tester le credential
credential.get_token("https://vault.azure.net/.default")
```

### Erreurs HSM PKCS#11

```python
pkcs11.exceptions.TokenNotPresent
```

**Solution :**

1. Vérifier que le HSM est connecté
2. Vérifier le chemin de la bibliothèque PKCS#11
3. Vérifier le label de token et le PIN

---

## Problèmes de performance

### Décodage lent avec biométrie

**Cause :** de grosses données biométriques sont coûteuses à parser.

**Solution :** ignorer la biométrie si inutile :

```python
result = claim169.decode_with_ed25519(
    qr_data,
    public_key,
    skip_biometrics=True
)
```

### Utilisation mémoire

**Cause :** de gros identifiants consomment de la mémoire au décodage.

**Solution :** fixer des limites adaptées :

```python
result = claim169.decode_with_ed25519(
    qr_data,
    public_key,
    max_decompressed_bytes=32768  # Limite 32KB
)
```

---

## Erreurs fréquentes

### Oublier de convertir la clé depuis l’hex

```python
# Faux - passer une chaîne hex
public_key = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
result = claim169.decode_with_ed25519(qr_data, public_key)  # Error!

# Correct - convertir en bytes
public_key = bytes.fromhex("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
result = claim169.decode_with_ed25519(qr_data, public_key)  # Works!
```

### Utiliser le mauvais type de clé

```python
# Faux - utiliser la clé privée pour vérifier
result = claim169.decode_with_ed25519(qr_data, private_key)  # May fail

# Correct - utiliser la clé publique pour vérifier
result = claim169.decode_with_ed25519(qr_data, public_key)
```

### Oublier un vérificateur pour des identifiants chiffrés

```python
# Faux - pas de verifier fourni
result = claim169.decode_encrypted_aes(qr_data, key)  # ValueError

# Correct - fournir un verifier ou allow_unverified
result = claim169.decode_encrypted_aes(qr_data, key, verifier=my_verifier)
# ou pour tests :
result = claim169.decode_encrypted_aes(qr_data, key, allow_unverified=True)
```

---

## Obtenir de l’aide

Si vous rencontrez un problème non couvert ici :

1. **Consulter la référence API** — [api.md](api.md)
2. **Regarder les exemples** — voir les fichiers de test du dépôt
3. **Ouvrir une issue** — [GitHub Issues](https://github.com/jeremi/claim-169/issues)

Lors d’un signalement, inclure :

- Version Python : `python --version`
- Version claim169 : `python -c "import claim169; print(claim169.version())"`
- Système d’exploitation
- Code minimal de reproduction
- Traceback complet

