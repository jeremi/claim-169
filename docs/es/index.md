# Claim 169

Una implementación multi-lenguaje de la especificación [MOSIP Claim 169](https://github.com/mosip/id-claim-169) para codificar y verificar credenciales de identidad digital en códigos QR.

## Descripción general

MOSIP Claim 169 define un formato compacto y seguro para codificar datos de identidad en códigos QR, optimizado para verificación sin conexión. Esta biblioteca proporciona:

- **Biblioteca Rust** con codificación, decodificación, verificación de firma y cifrado
- **SDK Python** para integración del lado del servidor
- **SDK TypeScript/JavaScript** via WebAssembly para navegador y Node.js
- **Playground interactivo** para experimentar con códigos QR

## Pipeline de codificación

```
Datos de identidad → CBOR → CWT → COSE_Sign1 → [COSE_Encrypt0] → zlib → Base45 → Código QR
```

## Algoritmos soportados

| Operación | Algoritmos |
|-----------|------------|
| Firma | Ed25519, ECDSA P-256 (ES256) |
| Cifrado | AES-128-GCM, AES-256-GCM |
| Compresión | zlib (DEFLATE) |
| Codificación | Base45 |

## Ejemplo rápido

=== "Rust"

    ```rust
    use claim169_core::{Decoder, Encoder, Claim169Input, CwtMetaInput};

    // Decodificar un código QR
    let result = Decoder::new(qr_content)
        .verify_with_ed25519(&public_key)?
        .decode()?;

    println!("Nombre: {:?}", result.claim169.full_name);
    ```

=== "Python"

    ```python
    from claim169 import decode_with_ed25519

    result = decode_with_ed25519(qr_text, public_key_bytes)
    print(f"Nombre: {result.claim169.full_name}")
    ```

=== "TypeScript"

    ```typescript
    import { Decoder } from 'claim169';

    const result = new Decoder(qrText)
      .verifyWithEd25519(publicKey)
      .decode();

    console.log(`Nombre: ${result.claim169.fullName}`);
    ```

## Próximos pasos

- [Instalación](getting-started/installation.md) - Instalar el SDK para tu lenguaje
- [Inicio rápido](getting-started/quick-start.md) - Codificar y decodificar tu primera credencial
- [Playground](playground.md) - Pruébalo en tu navegador
