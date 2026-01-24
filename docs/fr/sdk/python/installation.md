# Installation

## Prérequis

- **Python 3.8+** — le SDK supporte Python 3.8, 3.9, 3.10, 3.11 et 3.12
- **Aucune dépendance runtime** — la bibliothèque cœur est auto-contenue

## Installation avec pip

```bash
pip install claim169
```

## Installation avec uv

```bash
uv add claim169
```

## Installation avec Poetry

```bash
poetry add claim169
```

## Installation de développement

Pour contribuer ou tester des fournisseurs crypto personnalisés :

```bash
# Cloner le dépôt
git clone https://github.com/jeremi/claim-169.git
cd claim-169/core/claim169-python

# Installer avec les dépendances de dev
uv sync --dev

# Construire l’extension native
maturin develop

# Lancer les tests
uv run pytest tests/ -v
```

## Compatibilité plateformes

Des wheels précompilés sont disponibles pour :

| Plateforme | Architecture | Wheel |
|----------|--------------|-------|
| Linux | x86_64 | `manylinux_2_28` |
| Linux | aarch64 | `manylinux_2_28` |
| macOS | x86_64 | `macosx_10_12` |
| macOS | arm64 (Apple Silicon) | `macosx_11_0` |
| Windows | x86_64 | `win_amd64` |

Si aucun wheel n’est disponible pour votre plateforme, pip tentera de construire depuis les sources, ce qui nécessite :

- Toolchain Rust 1.70+
- maturin (`pip install maturin`)

## Vérifier l’installation

```python
import claim169

print(f"claim169 version: {claim169.version()}")
print(f"Python version: {claim169.__version__}")
```

Sortie :

```
claim169 version: 0.1.0-alpha.2
Python version: 0.1.0-alpha.2
```

## Dépendances optionnelles

Pour les fournisseurs crypto personnalisés (HSM, intégration KMS), installez `cryptography` :

```bash
pip install cryptography
```

Cela permet :

- Callbacks de vérification de signature personnalisés
- Callbacks de déchiffrement personnalisés
- Intégration AWS KMS, Azure Key Vault, Google Cloud KMS

## Mettre à jour

```bash
pip install --upgrade claim169
```

## Désinstaller

```bash
pip uninstall claim169
```

## Dépannage d’installation

### Échec de build depuis les sources

Si l’installation depuis les sources échoue :

1. Installer Rust : `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Installer maturin : `pip install maturin`
3. Tenter un build manuel : `maturin build --release`

### Erreur d’import

Si vous voyez `ImportError: No module named 'claim169'` :

1. Vérifier l’installation : `pip show claim169`
2. Vérifier votre Python path : `python -c "import sys; print(sys.path)"`
3. Vérifier que vous utilisez le bon interpréteur Python

### Incohérence de version

Si `claim169.version()` retourne une version inattendue :

```bash
pip uninstall claim169
pip cache purge
pip install claim169
```
