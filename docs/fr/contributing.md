# Contribuer

Merci de votre intérêt pour contribuer à Claim 169 !

## Bien démarrer

1. Forkez le dépôt sur GitHub
2. Clonez votre fork en local
3. Configurez l'environnement de développement

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
- **Python** 3.8+ avec uv et maturin (pour les bindings Python)
- **Node.js** 18+ avec npm (pour le SDK TypeScript)
- **JDK** 17+ avec Gradle (pour le SDK Kotlin)
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

# SDK Kotlin/Java
cargo build -p claim169-jni
cd sdks/kotlin && ./gradlew :claim169-core:test
```

## Développement assisté par IA

Ce projet utilise intensivement des outils d'IA dans le développement. Les contributions assistées par IA sont acceptées, mais le choix des outils vous appartient.

### Responsabilité

**Vous êtes responsable de vos contributions.** Que vous utilisiez des outils d'IA ou que vous écriviez du code manuellement, vous restez pleinement responsable de l'exactitude, de la sécurité, de la qualité et de la compatibilité des licences.

### Divulgation

**Les contributions externes doivent divulguer l'utilisation d'IA.** Si vous avez utilisé des outils d'IA (Codex, Claude, Copilot chat, etc.) pour générer ou façonner substantiellement du code dans votre PR, indiquez-le dans la description de la PR. Cela aide les relecteurs à calibrer leur revue — le code généré par IA semble souvent plausible mais peut comporter des problèmes subtils, en particulier dans la logique cryptographique ou le parsing.

Format :

```
AI: Utilisé [outil] pour [ce qu'il a aidé à faire]. Validé par [comment vous avez vérifié].
```

L'autocomplétion en ligne de l'IDE, l'aide grammaticale pour la documentation et l'utilisation de l'IA pour comprendre le codebase ne nécessitent pas de divulgation.

!!! warning "Code cryptographique"
    Les outils d'IA suggèrent souvent des implémentations cryptographiques qui semblent correctes mais qui comportent des failles de sécurité subtiles (attaques par canal auxiliaire, gestion faible des clés, paramètres d'algorithme incorrects). Validez toujours par rapport aux spécifications et incluez des tests de sécurité complets.

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
- `kotlin` — SDK Kotlin/Java
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
6. Soumettez une pull request en utilisant le modèle fourni

Le modèle de PR inclut une section **AI Disclosure** — remplissez-la si vous avez utilisé des outils d'IA, ou supprimez-la si ce n'est pas le cas.

## Style de code

- Suivre les patterns existants dans le codebase
- Écrire des tests pour toute nouvelle fonctionnalité
- Ajouter de la documentation pour les API publiques
- Garder des changements ciblés et atomiques

## Sécurité

**N'ouvrez pas d'issues GitHub publiques pour les vulnérabilités de sécurité.** Utilisez le [signalement privé de vulnérabilités de GitHub](https://github.com/jeremi/claim-169/security/advisories/new). Consultez [SECURITY.md](https://github.com/jeremi/claim-169/blob/main/SECURITY.md) pour plus de détails.

## Signaler des problèmes

- Utiliser l'issue tracker GitHub
- Inclure des étapes de reproduction
- Joindre les logs/erreurs pertinents
- Préciser votre environnement (OS, version du SDK)

## Licence

En contribuant, vous acceptez que vos contributions soient licenciées sous la licence du projet.
