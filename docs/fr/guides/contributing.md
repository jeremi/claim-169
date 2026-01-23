# Contribuer

Cette page décrit comment contribuer à la documentation (y compris les traductions) tout en minimisant la dérive vis-à-vis du code.

## Prévisualiser en local

Depuis la racine du dépôt :

```bash
uv sync --dev
uv run mkdocs serve
```

## Workflow de traduction (EN → FR/ES)

- Considérer `docs/en/` comme la référence.
- Chaque page sous `docs/en/` doit exister sous `docs/fr/` et `docs/es/` au même chemin relatif.
- Si vous souhaitez volontairement utiliser le fallback EN en FR/ES, créez quand même la page FR/ES et laissez un TODO explicite pour suivre la dette de traduction.

Vérifier la parité :

```bash
python3 scripts/check-docs-i18n.py
```

## À ne pas traduire

- Identifiants de code (`Decoder`, `DecodeOptions`, noms de fonctions).
- Champs/constantes du format (`iss`, `exp`, clés CBOR numériques comme `169`).
- Commandes CLI et chemins de fichiers.

## Libellés de navigation

Les libellés de navigation sont traduits via `mkdocs.yml` → `plugins.i18n.languages[*].nav_translations`.

Lorsqu’on ajoute une entrée dans `nav`, mettre à jour :

- `nav:` (une seule fois)
- `nav_translations:` pour `en`, `fr` et `es`

## CI docs

Le workflow documentation vérifie :

- la parité i18n (`scripts/check-docs-i18n.py`)
- `mkdocs build --strict`
- l’orthographe (`codespell`)

