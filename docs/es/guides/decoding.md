# Decodificación y verificación

Esta guía cubre la decodificación de credenciales Claim 169 y la verificación de su autenticidad.

## Pipeline de decodificación

Al decodificar, los datos pasan por estas etapas:

```
Código QR → Base45 → zlib → COSE → CWT → Claim 169
```

Para más detalles, consulta la [documentación en inglés](../../en/guides/decoding.md).
