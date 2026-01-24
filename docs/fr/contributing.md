# Contribuer

Merci de votre intérêt pour contribuer à Claim 169 !

## Bien démarrer

1. Forkez le dépôt sur GitHub
2. Clonez votre fork en local
3. Configurez l’environnement de développement

```bash
# Cloner le dépôt
git clone https://github.com/YOUR_USERNAME/claim-169.git
cd claim-169

# Tout construire
cargo build --release
cargo test --all-features
```

## Configuration du développement

### Prérequis

- **Rust** 1.75+ avec cargo
- **Python** 3.8+ avec maturin (pour les bindings Python)
- **Node.js** 18+ avec npm (pour le SDK TypeScript)
- **wasm-pack** (pour les bindings WebAssembly)

### Construire les composants

```bash
# Bibliothèque cœur Rust
cargo build --release
cargo test --all-features

# Bindings Python
cd core/claim169-python
maturin develop --release
uv run pytest tests/ -v

# SDK TypeScript/WASM
cd sdks/typescript
npm install
npm run build
npm test
```

## Format des messages de commit

Ce projet utilise **Conventional Commits** pour générer automatiquement le changelog :

```
<type>(<scope>): <description>

[corps optionnel]

[footer(s) optionnel(s)]
```

### Types

| Type | Description |
|------|-------------|
| `feat` | Nouvelle fonctionnalité |
| `fix` | Correction de bug |
| `docs` | Documentation uniquement |
| `perf` | Amélioration de performance |
| `refactor` | Refactorisation |
| `test` | Ajout/mise à jour de tests |
| `chore` | Maintenance |
| `ci` | Changements CI/CD |

### Scopes

- `core` — Bibliothèque cœur Rust
- `python` — SDK Python
- `typescript` — SDK TypeScript
- `wasm` — Bindings WASM
- `deps` — Dépendances

### Exemples

```bash
feat(core): add support for palm biometrics
fix(python): handle empty QR code input
docs: update installation instructions
chore(deps): update ed25519-dalek to 2.1
```

## Processus de pull request

1. Créez une branche de fonctionnalité depuis `main`
2. Faites vos changements avec des tests
3. Vérifiez que tous les tests passent : `cargo test --all-features`
4. Lancez le lint : `cargo clippy --all-targets --all-features`
5. Formatez le code : `cargo fmt --all`
6. Soumettez une pull request

## Style de code

- Suivre les patterns existants dans le codebase
- Écrire des tests pour toute nouvelle fonctionnalité
- Ajouter de la documentation pour les API publiques
- Garder des changements ciblés et atomiques

## Signaler des problèmes

- Utiliser l’issue tracker GitHub
- Inclure des étapes de reproduction
- Joindre les logs/erreurs pertinents
- Préciser votre environnement (OS, version du SDK)

## Licence

En contribuant, vous acceptez que vos contributions soient licenciées sous la licence du projet.

