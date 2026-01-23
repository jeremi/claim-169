# Vectores de prueba y conformidad

Este repositorio incluye vectores de prueba JSON en `test-vectors/` para:

- validar tu integración,
- comparar comportamiento entre SDKs,
- reproducir casos límite (expirado, campos desconocidos, entradas malformadas).

## Estructura

- `test-vectors/valid/` — debería decodificar correctamente
- `test-vectors/edge/` — debería decodificar, pero puede depender de tu política (validación de timestamps, etc.)
- `test-vectors/invalid/` — debería ser rechazado

Cada vector contiene como mínimo:

- `qr_data` (texto Base45)
- claves opcionales para pruebas (`public_key_hex`, `private_key_hex`, claves de cifrado)
- `expected_claim169` y `expected_cwt_meta` para comprobaciones rápidas

!!! warning "No usar claves de vectores en producción"
    Las claves de los vectores de prueba son públicas y nunca deben usarse para credenciales reales.

## Ejemplo: decodificar un vector firmado

Con `test-vectors/valid/ed25519-signed.json`:

=== "Python"

    ```python
    import json
    import claim169

    v = json.load(open("test-vectors/valid/ed25519-signed.json"))
    public_key = bytes.fromhex(v["signing_key"]["public_key_hex"])

    result = claim169.decode_with_ed25519(v["qr_data"], public_key)
    print(result.claim169.full_name)
    ```

=== "TypeScript"

    ```ts
    import fs from "fs";
    import { Decoder, hexToBytes } from "claim169";

    const v = JSON.parse(fs.readFileSync("test-vectors/valid/ed25519-signed.json", "utf8"));
    const publicKey = hexToBytes(v.signing_key.public_key_hex);

    const result = new Decoder(v.qr_data).verifyWithEd25519(publicKey).decode();
    console.log(result.claim169.fullName);
    ```

## Script de conformidad entre lenguajes

Existe un script para comparar resultados Python/TypeScript:

```bash
./scripts/conformance-test.sh
```

Notas:

- El script desactiva validación de timestamps para coincidir con los valores por defecto de TypeScript/WASM.
- Necesitas dependencias de Python y TypeScript disponibles en tu entorno.

