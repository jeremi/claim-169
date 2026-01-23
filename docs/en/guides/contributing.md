# Contributing

This page explains how to work on the documentation (including translations) with minimal drift vs. the code.

## Local preview

From the repo root:

```bash
uv sync --dev
uv run mkdocs serve
```

## Translation workflow (EN → FR/ES)

- Treat `docs/en/` as the source of truth.
- Every page under `docs/en/` must exist under `docs/fr/` and `docs/es/` with the same relative path.
- If you intentionally want EN content to show up in FR/ES via fallback, still create the FR/ES page and leave a clear TODO so translation debt is explicit.

Check parity:

```bash
python3 scripts/check-docs-i18n.py
```

## What not to translate

- Code identifiers (`Decoder`, `DecodeOptions`, function names).
- Wire-format fields and constants (`iss`, `exp`, numeric CBOR keys like `169`).
- CLI commands and file paths.

## Navigation labels

Navigation labels are translated via `mkdocs.yml` → `plugins.i18n.languages[*].nav_translations`.

When adding a new nav entry, update:

- `nav:` (once)
- `nav_translations:` for `en`, `fr`, and `es`

## Docs CI

The docs workflow enforces:

- i18n page parity (`scripts/check-docs-i18n.py`)
- `mkdocs build --strict`
- spellcheck (`codespell`)

