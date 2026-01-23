# Solución de problemas

Esta página lista errores comunes y cómo resolverlos rápidamente.

## “decoding configuration error”

Si ves un error como “either provide a verifier or explicitly allow unverified decoding”, significa que intentaste decodificar sin:

- configurar verificación de firma, o
- optar explícitamente por no verificar (solo pruebas).

Soluciones:

- **Rust**: en producción, llamar `.verify_with_ed25519(...)` / `.verify_with_ecdsa_p256(...)`; en pruebas, llamar `.allow_unverified()`
- **Python**: en producción, usar `decode_with_ed25519()` / `decode_with_ecdsa_p256()`; en pruebas, usar `decode_unverified()` (o `decode(..., allow_unverified=True)`)
- **TypeScript**: en producción, llamar `.verifyWithEd25519(...)` / `.verifyWithEcdsaP256(...)`; en pruebas, llamar `.allowUnverified()`

## Fallos de verificación de firma

Causas comunes:

- algoritmo incorrecto (Ed25519 vs ES256),
- clave pública incorrecta (emisor equivocado / entorno equivocado),
- QR truncado o corrupto.

Solución:

- confirmar que el vector/clave coinciden,
- asegurar formato correcto de clave (ver guía de claves).

## “credential expired” / “not valid until …”

La validación de timestamps rechaza la credencial por `exp`/`nbf`.

Opciones:

- usar credenciales no expiradas,
- ajustar tolerancia de reloj,
- desactivar validación solo si tu modelo de amenaza lo permite.

## “decompression limit exceeded”

La librería impone un límite de tamaño tras descompresión (64KB por defecto).

Solución:

- si controlas el emisor y esperas payloads más grandes, aumentar el límite (`max_decompressed_bytes(...)` / `maxDecompressedBytes(...)`),
- si no, tratar la entrada como potencialmente maliciosa y rechazar.

## Fallos de descifrado

Causas comunes:

- clave AES incorrecta (o longitud incorrecta),
- orden incorrecto (hay que descifrar antes de verificar),
- ciphertext corrupto.

Solución:

- descifrar **antes** de verificar,
- verificar tamaños: AES-256 = 32 bytes, AES-128 = 16 bytes.
