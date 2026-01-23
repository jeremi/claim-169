# Décodage et vérification

Ce guide couvre le décodage des identifiants Claim 169 et la vérification de leur authenticité.

## Pipeline de décodage

Lors du décodage, les données passent par ces étapes :

```
Code QR → Base45 → zlib → COSE → CWT → Claim 169
```

## Décodage basique

### Avec vérification (Production)

Vérifiez toujours les signatures en production :

=== "Python"

    ```python
    from claim169 import decode_with_ed25519

    result = decode_with_ed25519(qr_data, public_key)

    # Accéder aux données d'identité
    print(f"ID : {result.claim169.id}")
    print(f"Nom : {result.claim169.full_name}")

    # Accéder aux métadonnées CWT
    print(f"Émetteur : {result.cwt_meta.issuer}")
    print(f"Expire : {result.cwt_meta.expires_at}")
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

### Sans vérification (tests uniquement)

!!! danger "Avertissement"
    Sans vérification de signature, un QR code peut être falsifié.

=== "Python"

    ```python
    import claim169

    result = claim169.decode_unverified(qr_data)
    ```

## Identifiants chiffrés

Pour les payloads chiffrés, il faut **déchiffrer avant de vérifier**.

=== "Python"

    ```python
    from claim169 import decode_encrypted_aes

    # Test uniquement : déchiffre sans vérifier la signature interne
    result = decode_encrypted_aes(qr_data, encryption_key)
    ```

Voir aussi :

- [Chiffrement](encryption.md)
- [Matériel de clés et formats](keys.md)
- [Sécurité et validations](security.md)
