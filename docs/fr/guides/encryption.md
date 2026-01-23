# Chiffrement

Ce guide couvre le chiffrement des identifiants Claim 169 pour protéger les données d'identité sensibles.

## Aperçu

Le chiffrement ajoute une couche de confidentialité en enveloppant la structure COSE signée dans une enveloppe COSE_Encrypt0.

```
Identifiant signé (COSE_Sign1) → COSE_Encrypt0 → zlib → Base45 → QR
```

## Algorithmes supportés

| Algorithme | Taille de clé | Description |
|-----------|---------------|-------------|
| AES-256-GCM | 32 octets | Recommandé pour la plupart des cas |
| AES-128-GCM | 16 octets | Clé plus petite, toujours sécurisé |

## Chiffrer (AES-256-GCM)

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

!!! note "Python et AES-128"
    Les bindings Python n’exposent pas encore l’encodage AES-128-GCM. Rust et TypeScript le supportent via `encrypt_with_aes128` / `encryptWithAes128`.

## Déchiffrer

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

    # Test uniquement : déchiffre sans vérifier la signature interne
    result = decode_encrypted_aes(qr_text, encryption_key, allow_unverified=True)
    ```

=== "TypeScript"

    ```ts
    const result = new Decoder(qrText)
      .decryptWithAes256(encryptionKey)
      .verifyWithEd25519(publicKey)
      .decode();
    ```

## Générer des clés AES

```python
import secrets
aes256_key = secrets.token_bytes(32)
```

Voir aussi :

- [Matériel de clés et formats](keys.md)
- [Sécurité et validations](security.md)
