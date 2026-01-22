# Inicio rápido

Esta guía te acompaña en la codificación y decodificación de tu primera credencial Claim 169.

## Decodificar un código QR

La operación más común es decodificar y verificar una credencial QR.

### Sin verificación (solo para pruebas)

Para pruebas, puedes decodificar sin verificar la firma:

=== "Python"

    ```python
    from claim169 import decode_unverified

    qr_data = "6BF590B20F..."  # Datos QR codificados en Base45

    result = decode_unverified(qr_data)

    print(f"ID: {result.claim169.id}")
    print(f"Nombre: {result.claim169.full_name}")
    print(f"Emisor: {result.cwt_meta.issuer}")
    ```

=== "TypeScript"

    ```typescript
    import { Decoder } from 'claim169';

    const qrData = "6BF590B20F..."; // Datos QR codificados en Base45

    const result = new Decoder(qrData)
      .allowUnverified()
      .decode();

    console.log(`ID: ${result.claim169.id}`);
    console.log(`Nombre: ${result.claim169.fullName}`);
    console.log(`Emisor: ${result.cwtMeta.issuer}`);
    ```

### Con verificación de firma

En producción, siempre verifica la firma:

=== "Python"

    ```python
    from claim169 import decode_with_ed25519

    qr_data = "6BF590B20F..."
    public_key = bytes.fromhex("d75a980182b10ab7...")  # 32 bytes

    result = decode_with_ed25519(qr_data, public_key)

    # Firma verificada - la credencial es auténtica
    print(f"¡Verificado! Nombre: {result.claim169.full_name}")
    ```

=== "TypeScript"

    ```typescript
    import { Decoder, hexToBytes } from 'claim169';

    const qrData = "6BF590B20F...";
    const publicKey = hexToBytes("d75a980182b10ab7..."); // 32 bytes

    const result = new Decoder(qrData)
      .verifyWithEd25519(publicKey)
      .decode();

    // Firma verificada - la credencial es auténtica
    console.log(`¡Verificado! Nombre: ${result.claim169.fullName}`);
    ```

## Próximos pasos

- [Guía de codificación](../guides/encoding.md) - Conoce todos los campos disponibles
- [Guía de decodificación](../guides/decoding.md) - Maneja casos límite y errores
- [Guía de cifrado](../guides/encryption.md) - Protege datos sensibles
- [Playground](../playground.md) - Pruébalo interactivamente en tu navegador
