# SDK Python

<div class="badges" markdown>
[![PyPI](https://img.shields.io/pypi/v/claim169)](https://pypi.org/project/claim169/)
[![Python](https://img.shields.io/pypi/pyversions/claim169)](https://pypi.org/project/claim169/)
[![License](https://img.shields.io/pypi/l/claim169)](https://github.com/jeremi/claim-169/blob/main/LICENSE)
</div>

Le SDK Python fournit des bindings natifs pour encoder et décoder des QR codes MOSIP Claim 169. Construit avec PyO3 pour les performances, il expose toute la puissance de la bibliothèque cœur Rust aux applications Python.

## Pourquoi Python ?

- **Performances natives** — cœur en Rust avec parsing « zero-copy » quand c’est possible
- **Hints de types** — annotations complètes pour IDE et analyse statique
- **API pythonique** — patterns familiers avec configuration de type builder
- **Compatible HSM/KMS** — hooks de callbacks pour des fournisseurs crypto externes
- **Multi-plateforme** — wheels précompilés pour Linux, macOS et Windows

## Installation

```bash
pip install claim169
```

Ou avec uv :

```bash
uv add claim169
```

## Démarrage rapide

```python
import claim169

# Décoder un QR code avec vérification Ed25519
qr_data = "..."  # Chaîne Base45 du scanner QR
public_key = bytes.fromhex("...")  # Clé publique Ed25519 (32 octets) de l’émetteur

result = claim169.decode_with_ed25519(qr_data, public_key)

print(f"ID: {result.claim169.id}")
print(f"Name: {result.claim169.full_name}")
print(f"Verified: {result.is_verified()}")
```

## Documentation

<div class="doc-grid" markdown>

<div class="doc-card" markdown>
### [Installation](installation.md)
Versions Python supportées, installation pip/uv, compatibilité plateformes.
</div>

<div class="doc-card" markdown>
### [Démarrage rapide](quick-start.md)
Exemples simples d’encodage/décodage.
</div>

<div class="doc-card" markdown>
### [Encodage](encoding.md)
Créer des identifiants signés avec Ed25519 ou ECDSA P-256.
</div>

<div class="doc-card" markdown>
### [Décodage](decoding.md)
Vérifier et extraire les données d’identité depuis des QR codes.
</div>

<div class="doc-card" markdown>
### [Chiffrement](encryption.md)
Exemples de chiffrement AES-256-GCM et AES-128-GCM.
</div>

<div class="doc-card" markdown>
### [Crypto personnalisée](custom-crypto.md)
Intégration HSM et KMS cloud (AWS, Azure, Google Cloud).
</div>

<div class="doc-card" markdown>
### [Référence API](api.md)
Documentation complète des fonctions et classes.
</div>

<div class="doc-card" markdown>
### [Dépannage](troubleshooting.md)
Erreurs fréquentes et solutions.
</div>

</div>

## Fonctionnalités

### Décodage

| Fonction | Description |
|----------|-------------|
| `decode_with_ed25519()` | Décoder avec vérification de signature Ed25519 |
| `decode_with_ecdsa_p256()` | Décoder avec vérification de signature ECDSA P-256 |
| `decode_with_verifier()` | Décoder via callback de vérification (HSM/KMS) |
| `decode_encrypted_aes256()` | Déchiffrer AES-256-GCM puis décoder |
| `decode_encrypted_aes128()` | Déchiffrer AES-128-GCM puis décoder |
| `decode_with_decryptor()` | Déchiffrer via callback de déchiffrement personnalisé |
| `decode_unverified()` | Décoder sans vérification (tests uniquement) |

### Encodage

| Fonction | Description |
|----------|-------------|
| `encode()` | Fonction d'encodage unifiée avec arguments nommés |
| `encode_with_ed25519()` | Encoder avec signature Ed25519 |
| `encode_with_ecdsa_p256()` | Encoder avec signature ECDSA P-256 |
| `encode_with_signer()` | Encoder via callback de signature (HSM/KMS) |
| `encode_signed_encrypted()` | Signer (Ed25519) + chiffrer (AES-256) |
| `encode_signed_encrypted_aes128()` | Signer (Ed25519) + chiffrer (AES-128) |
| `encode_with_signer_and_encryptor()` | Callbacks personnalisés (signature + chiffrement) |
| `encode_unsigned()` | Encoder sans signature (tests uniquement) |

## Prérequis

- Python 3.8 ou plus
- Aucune dépendance additionnelle pour l’usage basique
- Package `cryptography` pour les fournisseurs crypto personnalisés

## Licence

Licence MIT. Voir [LICENSE](https://github.com/jeremi/claim-169/blob/main/LICENSE) pour les détails.
