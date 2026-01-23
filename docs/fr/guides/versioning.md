# Versioning

Ce projet publie plusieurs artefacts (crate Rust, package Python, package TypeScript, et documentation).

## Ce que reflète la documentation

- La doc de ce dépôt suit la **branche courante** (typiquement `main`).
- Les versions publiées peuvent être en retard par rapport à `main`.

## Épingler des versions

### Rust

```toml
[dependencies]
claim169-core = "0.1.0-alpha.2"
```

### Python

```bash
pip install "claim169==0.1.0-alpha.2"
```

### TypeScript

```bash
npm install "claim169@0.1.0-alpha.2"
```

## Quand mettre à jour la doc

- Toute modification d’API exportée doit être reflétée dans `docs/en/api/*` et protégée par des tests (stubs Python, exports TypeScript).
- Toute modification du format ou du comportement de parsing doit être reflétée dans `docs/en/specification.md` et dans les `test-vectors/`.

