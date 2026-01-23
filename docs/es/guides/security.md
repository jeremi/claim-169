# Seguridad y validación

Esta página resume los valores por defecto de seguridad y las opciones que puedes ajustar según tu contexto.

## Verificar siempre en producción

Los datos de un QR Claim 169 solo son confiables después de validarlos.

- **Producción**: verificar firmas (`Ed25519` o `ECDSA P-256`)
- **Solo pruebas**: decodificar sin verificación, pero tratar el resultado como no confiable

!!! danger "Decodificación sin verificación es insegura"
    Si omites la verificación de firma, un QR puede ser falsificado. Usa decodificación sin verificación solo para vectores de prueba, depuración, o si la verificación se hace en otro lugar.

## Validación de timestamps (exp/nbf)

Los timestamps CWT ayudan a rechazar credenciales expiradas o no vigentes todavía:

- `exp` (expiración)
- `nbf` (not before)

### Los valores por defecto varían por SDK

- **Rust**: validación de timestamps **activada por defecto**
- **Python**: validación de timestamps **activada por defecto**
- **TypeScript/WASM**: validación de timestamps **desactivada por defecto** (WASM no tiene acceso fiable al reloj del sistema)

## Límites de descompresión

El payload está comprimido con zlib. Para prevenir ataques tipo “zip bomb”, el decodificador impone un tamaño máximo tras descompresión.

- Límite por defecto: **64 KB** (`65536` bytes)

Si aumentas el límite, hazlo con cuidado y solo si confías en la fuente.

## Parseo de biometría

Los datos biométricos pueden ser grandes. Si solo necesitas datos demográficos (nombre, fecha de nacimiento, etc.), puedes omitir el parseo biométrico.

=== "Rust"

    ```rust
    let result = claim169_core::Decoder::new(qr_text)
        .skip_biometrics()
        .allow_unverified()
        .decode()?;
    ```

=== "Python"

    ```python
    import claim169

    result = claim169.decode_unverified(qr_text, skip_biometrics=True)
    ```

=== "TypeScript"

    ```ts
    import { Decoder } from "claim169";

    const result = new Decoder(qrText)
      .skipBiometrics()
      .allowUnverified()
      .decode();
    ```

## Orden de cifrado (firmar y luego cifrar)

Al codificar con cifrado:

1. **Firmar** el CWT (`COSE_Sign1`)
2. **Cifrar** el payload firmado (`COSE_Encrypt0`)

Al decodificar un payload cifrado:

1. **Descifrar**
2. **Verificar**

## Recomendaciones

### Verificador (producción)

- Exigir verificación de firma
- Validar timestamps (o definir una política explícita si el reloj no es fiable)
- Mantener límites de descompresión

### Emisor (producción)

- Firmar siempre
- Cifrar solo si hay un plan de distribución de claves seguro
- Nunca reutilizar nonces AES-GCM con la misma clave

