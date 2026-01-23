# Decodificación y verificación

Esta guía cubre la decodificación de credenciales Claim 169 y la verificación de su autenticidad.

## Pipeline de decodificación

Al decodificar, los datos pasan por estas etapas:

```
Código QR → Base45 → zlib → COSE → CWT → Claim 169
```

## Decodificación básica

### Con verificación (producción)

Verifica siempre las firmas en producción:

=== "Python"

    ```python
    from claim169 import decode_with_ed25519

    result = decode_with_ed25519(qr_data, public_key)
    print(result.claim169.full_name)
    ```

=== "Rust"

    ```rust
    use claim169_core::Decoder;

    let result = Decoder::new(qr_text)
        .verify_with_ed25519(&public_key)?
        .decode()?;
    ```

=== "TypeScript"

    ```ts
    import { Decoder } from "claim169";

    const result = new Decoder(qrText)
      .verifyWithEd25519(publicKey)
      .decode();
    ```

### Sin verificación (solo pruebas)

!!! danger "Advertencia"
    Sin verificación de firma, un QR puede ser falsificado.

```python
import claim169
result = claim169.decode_unverified(qr_data)
```

## Credenciales cifradas

Para payloads cifrados, hay que **descifrar antes de verificar**.

```python
from claim169 import decode_encrypted_aes

# Solo pruebas: descifra sin verificar la firma interna
result = decode_encrypted_aes(qr_data, encryption_key)
```

Ver también:

- [Cifrado](encryption.md)
- [Material de claves y formatos](keys.md)
- [Seguridad y validación](security.md)
