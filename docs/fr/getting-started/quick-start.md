# Démarrage rapide

Ce guide vous accompagne dans l'encodage et le décodage de votre premier identifiant Claim 169.

## Décoder un code QR

L'opération la plus courante est le décodage et la vérification d'un identifiant QR.

### Sans vérification (tests uniquement)

Pour les tests, vous pouvez décoder sans vérifier la signature :

=== "Rust"

    ```rust
    use claim169_core::Decoder;

    let qr_data = "6BF590B20F..."; // Données QR encodées en Base45

    let result = Decoder::new(qr_data)
        .allow_unverified()
        .decode()?;

    println!("ID : {:?}", result.claim169.id);
    println!("Nom : {:?}", result.claim169.full_name);
    println!("Émetteur : {:?}", result.cwt_meta.issuer);
    ```

=== "Python"

    ```python
    from claim169 import decode_unverified

    qr_data = "6BF590B20F..."  # Données QR encodées en Base45

    result = decode_unverified(qr_data)

    print(f"ID : {result.claim169.id}")
    print(f"Nom : {result.claim169.full_name}")
    print(f"Émetteur : {result.cwt_meta.issuer}")
    ```

=== "TypeScript"

    ```typescript
    import { Decoder } from 'claim169';

    const qrData = "6BF590B20F..."; // Données QR encodées en Base45

    const result = new Decoder(qrData)
      .allowUnverified()
      .decode();

    console.log(`ID : ${result.claim169.id}`);
    console.log(`Nom : ${result.claim169.fullName}`);
    console.log(`Émetteur : ${result.cwtMeta.issuer}`);
    ```

### Avec vérification de signature

En production, vérifiez toujours la signature :

=== "Rust"

    ```rust
    use claim169_core::Decoder;

    let qr_data = "6BF590B20F...";
    let public_key: [u8; 32] = /* clé publique Ed25519 */;

    let result = Decoder::new(qr_data)
        .verify_with_ed25519(&public_key)?
        .decode()?;

    // Signature vérifiée - l'identifiant est authentique
    println!("Vérifié ! Nom : {:?}", result.claim169.full_name);
    ```

=== "Python"

    ```python
    from claim169 import decode_with_ed25519

    qr_data = "6BF590B20F..."
    public_key = bytes.fromhex("d75a980182b10ab7...")  # 32 octets

    result = decode_with_ed25519(qr_data, public_key)

    # Signature vérifiée - l'identifiant est authentique
    print(f"Vérifié ! Nom : {result.claim169.full_name}")
    ```

=== "TypeScript"

    ```typescript
    import { Decoder, hexToBytes } from 'claim169';

    const qrData = "6BF590B20F...";
    const publicKey = hexToBytes("d75a980182b10ab7..."); // 32 octets

    const result = new Decoder(qrData)
      .verifyWithEd25519(publicKey)
      .decode();

    // Signature vérifiée - l'identifiant est authentique
    console.log(`Vérifié ! Nom : ${result.claim169.fullName}`);
    ```

## Encoder un identifiant

Créez un nouvel identifiant QR avec des données d'identité.

=== "Rust"

    ```rust
    use claim169_core::{Encoder, Claim169, CwtMeta};

    let claim = Claim169::new()
        .with_id("USER-12345")
        .with_full_name("Jean Dupont")
        .with_date_of_birth("19900115");

    let meta = CwtMeta::new()
        .with_issuer("https://example.com")
        .with_expires_at(1800000000);

    let private_key: [u8; 32] = /* clé privée Ed25519 */;

    let qr_data = Encoder::new(claim, meta)
        .sign_with_ed25519(&private_key)?
        .encode()?;

    println!("Données QR : {}", qr_data);
    ```

=== "Python"

    ```python
    from claim169 import Claim169Input, CwtMetaInput, encode_with_ed25519

    claim = Claim169Input(
        id="USER-12345",
        full_name="Jean Dupont",
        date_of_birth="19900115"
    )

    meta = CwtMetaInput(
        issuer="https://example.com",
        expires_at=1800000000
    )

    private_key = bytes.fromhex("9d61b19deffd5a60...")  # 32 octets

    qr_data = encode_with_ed25519(claim, meta, private_key)

    print(f"Données QR : {qr_data}")
    ```

=== "TypeScript"

    ```typescript
    import { Encoder, hexToBytes } from 'claim169';

    const claim = {
      id: "USER-12345",
      fullName: "Jean Dupont",
      dateOfBirth: "19900115"
    };

    const meta = {
      issuer: "https://example.com",
      expiresAt: 1800000000
    };

    const privateKey = hexToBytes("9d61b19deffd5a60..."); // 32 octets

    const qrData = new Encoder(claim, meta)
      .signWithEd25519(privateKey)
      .encode();

    console.log(`Données QR : ${qrData}`);
    ```

## Prochaines étapes

- [Guide d'encodage](../guides/encoding.md) - Découvrez tous les champs disponibles
- [Guide de décodage](../guides/decoding.md) - Gérez les cas limites et les erreurs
- [Guide de chiffrement](../guides/encryption.md) - Protégez les données sensibles
- [Playground](../playground.md) - Essayez de manière interactive dans votre navigateur
