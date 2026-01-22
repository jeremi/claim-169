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

Pour plus de détails, consultez la [documentation en anglais](../../en/guides/decoding.md).
