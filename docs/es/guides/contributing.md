# Contribuir

Esta página explica cómo contribuir a la documentación (incluyendo traducciones) con mínima deriva respecto al código.

## Vista previa local

Desde la raíz del repositorio:

```bash
uv sync --dev
uv run mkdocs serve
```

## Flujo de traducción (EN → FR/ES)

- Trata `docs/en/` como fuente de verdad.
- Cada página bajo `docs/en/` debe existir bajo `docs/fr/` y `docs/es/` con la misma ruta relativa.
- Si intencionalmente quieres que el contenido EN se muestre en FR/ES por fallback, crea igualmente la página FR/ES y deja un TODO claro para que la deuda de traducción sea explícita.

Verificar paridad:

```bash
python3 scripts/check-docs-i18n.py
```

## Qué no traducir

- Identificadores de código (`Decoder`, `DecodeOptions`, nombres de funciones).
- Campos/constantes del formato (`iss`, `exp`, claves CBOR numéricas como `169`).
- Comandos CLI y rutas de archivos.

## Etiquetas de navegación

Las etiquetas de navegación se traducen en `mkdocs.yml` → `plugins.i18n.languages[*].nav_translations`.

Al añadir una nueva entrada de navegación, actualizar:

- `nav:` (una vez)
- `nav_translations:` para `en`, `fr` y `es`

## CI de docs

El workflow de documentación valida:

- paridad i18n (`scripts/check-docs-i18n.py`)
- `mkdocs build --strict`
- ortografía (`codespell`)

