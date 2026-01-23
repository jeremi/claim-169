# Versioning

This project ships multiple artifacts (Rust crate, Python package, TypeScript package, and docs). To avoid confusion:

## What the docs reflect

- The docs in this repository track the **current branch** (typically `main`).
- Released packages may lag behind `main`.

## How to pin versions

### Rust

Pin a crate version in `Cargo.toml`:

```toml
[dependencies]
claim169-core = "0.1.0-alpha"
```

### Python

Pin via pip:

```bash
pip install "claim169==0.1.0-alpha"
```

### TypeScript

Pin via npm:

```bash
npm install "claim169@0.1.0-alpha"
```

## When to update docs

- If you change any exported API, update `docs/en/api/*` and add/adjust tests that lock the API surface (Python stubs, TypeScript exports).
- If you change the wire format or parsing behavior, update `docs/en/specification.md` and refresh `test-vectors/`.

