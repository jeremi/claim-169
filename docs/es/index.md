# Claim 169

Codifica y verifica credenciales de identidad digital con la especificación de código QR [MOSIP Claim 169](https://github.com/mosip/id-claim-169).

[Comenzar](getting-started/installation.md){ .md-button .md-button--primary }
[Probar el playground](playground.md){ .md-button }

## ¿Qué es Claim 169?

MOSIP Claim 169 define un formato compacto y seguro para codificar datos de identidad en códigos QR, optimizado para verificación sin conexión. Este repositorio proporciona:

- **Biblioteca Rust** (codificación, decodificación, verificación, cifrado)
- **SDK Python** (integración del lado del servidor)
- **SDK TypeScript/JavaScript** (WASM para navegador y Node.js)
- **Playground interactivo** (prueba vectores y construye payloads QR)

<div class="grid cards" markdown>

-   ### Rust Core
    Codificación/decodificación de alto rendimiento con verificación de firma y cifrado opcional.

    [API Rust](api/rust.md){ .md-button }

-   ### SDK Python
    Funciones simples para verificación, descifrado y pipelines de decodificación en servicios Python.

    [API Python](api/python.md){ .md-button }

-   ### TypeScript / JavaScript
    SDK impulsado por WebAssembly para navegador y Node.js.

    [API TypeScript](api/typescript.md){ .md-button }

-   ### Playground
    Codifica, decodifica, descifra y verifica sin instalar nada.

    [Abrir playground](playground.md){ .md-button }

</div>

## Pipeline de codificación

```text
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
    use claim169_core::Decoder;

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
    import { Decoder } from "claim169";

    const result = new Decoder(qrText)
      .verifyWithEd25519(publicKey)
      .decode();

    console.log(`Nombre: ${result.claim169.fullName}`);
    ```

## Enlaces rápidos

<div class="grid cards" markdown>

-   ### Instalación
    Instala el SDK para tu lenguaje.

    [Instalar](getting-started/installation.md){ .md-button }

-   ### Inicio rápido
    Codifica y decodifica tu primera credencial.

    [Inicio rápido](getting-started/quick-start.md){ .md-button }

-   ### Claves
    Formatos de claves y cómo proporcionarlas.

    [Claves](guides/keys.md){ .md-button }

-   ### Seguridad y validación
    Valores por defecto y opciones de política.

    [Seguridad](guides/security.md){ .md-button }

-   ### Especificación
    El formato de transmisión y las estructuras.

    [Especificación](specification.md){ .md-button }

-   ### Solución de problemas
    Errores comunes y soluciones.

    [Solución de problemas](guides/troubleshooting.md){ .md-button }

</div>

## Próximos pasos

- [Inicio rápido](getting-started/quick-start.md) — codifica y decodifica tu primera credencial
- [Material de claves y formatos](guides/keys.md) — formatos de claves y soporte PEM
- [Seguridad y validación](guides/security.md) — valores por defecto y opciones de política
- [Glosario](guides/glossary.md) — CBOR, COSE, CWT, etc.
- [Versiones](guides/versioning.md) — relación entre la documentación y las versiones
- [Solución de problemas](guides/troubleshooting.md) — errores comunes y soluciones

**¿Necesitas ayuda?** Comienza con [Solución de problemas](guides/troubleshooting.md) o consulta [Contribuir](guides/contributing.md) para mejorar la documentación.
