# Cifrado

Esta guía cubre el cifrado de credenciales Claim 169 para proteger datos de identidad sensibles.

## Algoritmos soportados

| Algoritmo | Tamaño de clave | Descripción |
|-----------|-----------------|-------------|
| AES-256-GCM | 32 bytes | Recomendado para la mayoría de casos |
| AES-128-GCM | 16 bytes | Clave más pequeña, aún seguro |

## Cifrar (AES-256-GCM)

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

!!! note "Python y AES-128"
    Los bindings de Python todavía no exponen codificación AES-128-GCM. Rust y TypeScript lo soportan via `encrypt_with_aes128` / `encryptWithAes128`.

## Descifrar

=== "Rust"

    ```rust
    let result = claim169_core::Decoder::new(qr_text)
        .decrypt_with_aes256(&encryption_key)?
        .verify_with_ed25519(&public_key)?
        .decode()?;
    ```

=== "Python"

    ```python
    from claim169 import decode_encrypted_aes

    # Solo pruebas: descifra sin verificar la firma interna
    result = decode_encrypted_aes(qr_text, encryption_key)
    ```

=== "TypeScript"

    ```ts
    const result = new Decoder(qrText)
      .decryptWithAes256(encryptionKey)
      .verifyWithEd25519(publicKey)
      .decode();
    ```

## Generar claves AES

```python
import secrets
aes256_key = secrets.token_bytes(32)
```

Ver también:

- [Material de claves y formatos](keys.md)
- [Seguridad y validación](security.md)
