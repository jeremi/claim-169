# Codificación de credenciales

Esta guía cubre la creación de credenciales Claim 169 con todas las opciones disponibles.

## Campos de identidad

La especificación Claim 169 define los siguientes campos de identidad:

| Campo | Clave CBOR | Tipo | Descripción |
|-------|------------|------|-------------|
| `id` | 1 | string | Identificador único |
| `version` | 2 | string | Versión de la especificación |
| `language` | 3 | string | Idioma principal (ISO 639-3) |
| `fullName` | 4 | string | Nombre completo |
| `firstName` | 5 | string | Nombre |
| `middleName` | 6 | string | Segundo nombre |
| `lastName` | 7 | string | Apellido |
| `dateOfBirth` | 8 | string | Fecha de nacimiento (recomendado: `AAAAMMDD`; común también: `AAAA-MM-DD`) |
| `gender` | 9 | integer | 1=Masculino, 2=Femenino, 3=Otro |
| `address` | 10 | string | Dirección completa |
| `email` | 11 | string | Correo electrónico |
| `phone` | 12 | string | Número de teléfono |
| `nationality` | 13 | string | Código de país |
| `maritalStatus` | 14 | integer | 1=Soltero/a, 2=Casado/a, 3=Divorciado/a |
| `guardian` | 15 | string | Tutor / responsable |
| `photo` | 16 | bytes | Datos de foto |
| `photoFormat` | 17 | integer | 1=JPEG, 2=JPEG2000, 3=AVIF, 4=WEBP |
| `bestQualityFingers` | 18 | array | Posiciones de dedos (0–10) |
| `secondaryFullName` | 19 | string | Nombre en idioma secundario |
| `secondaryLanguage` | 20 | string | Idioma secundario (ISO 639-3) |
| `locationCode` | 21 | string | Código de ubicación |
| `legalStatus` | 22 | string | Estatus legal |
| `countryOfIssuance` | 23 | string | País de emisión |

## Firma (recomendado)

### Ed25519

=== "Rust"

    ```rust
    let qr_data = claim169_core::Encoder::new(claim, meta)
        .sign_with_ed25519(&private_key)?
        .encode()?;
    ```

=== "Python"

    ```python
    from claim169 import encode_with_ed25519

    qr_data = encode_with_ed25519(claim, meta, private_key)
    ```

=== "TypeScript"

    ```ts
    const qrData = new Encoder(claim, meta)
      .signWithEd25519(privateKey)
      .encode();
    ```

## Cifrado (opcional)

El cifrado protege la privacidad (AES-GCM). El orden es **firmar y luego cifrar**.

=== "Rust"

    ```rust
    let qr_data = claim169_core::Encoder::new(claim, meta)
        .sign_with_ed25519(&signing_key)?
        .encrypt_with_aes256(&encryption_key)?
        .encode()?;
    ```

=== "Python"

    ```python
    from claim169 import encode_signed_encrypted

    qr_data = encode_signed_encrypted(claim, meta, signing_key, encryption_key)
    ```

=== "TypeScript"

    ```ts
    const qrData = new Encoder(claim, meta)
      .signWithEd25519(signingKey)
      .encryptWithAes256(encryptionKey)
      .encode();
    ```

Más recursos:

- [Material de claves y formatos](keys.md)
- [Seguridad y validación](security.md)
