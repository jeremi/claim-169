# Versiones

Este proyecto publica varios artefactos (crate Rust, paquete Python, paquete TypeScript y documentación).

## Qué refleja la documentación

- La documentación de este repositorio sigue la **rama actual** (normalmente `main`).
- Los paquetes publicados pueden ir por detrás de `main`.

## Fijar versiones

### Rust

```toml
[dependencies]
claim169-core = "0.1"
```

### Python

```bash
pip install "claim169==0.1.0-alpha"
```

### TypeScript

```bash
npm install "claim169@0.1.0-alpha"
```

## Cuándo actualizar la documentación

- Si cambias una API exportada, actualiza `docs/en/api/*` y añade/ajusta tests que fijen la superficie de API (stubs Python, exports TypeScript).
- Si cambias el formato o el comportamiento de parsing, actualiza `docs/en/specification.md` y regenera `test-vectors/`.

