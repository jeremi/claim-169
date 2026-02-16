# Bien démarrer

Les QR codes Claim 169 sont des chaînes **Base45** contenant des données d'identité signées (et éventuellement chiffrées).

!!! warning "Ne pas modifier la chaîne Base45"
    L'alphabet Base45 inclut un caractère espace (`" "`). Conservez le texte scanné tel quel (pas de `.trim()`, ni normalisation des espaces), sinon vous risquez de corrompre des identifiants valides.

## Décoder votre premier QR code

Voici un vrai payload Claim 169 signé avec Ed25519, issu des vecteurs de test du projet. Suivez le guide dans le langage de votre choix :

**Données QR** (Base45) :

```text
6BF590B20FFWJWG.FKJ05H7B0XKA8FA9DIWENPEJ/5P$DPQE88EB$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL580 B9%W5*B9C+9%R8646%86HKESED1/DRTC5UA QE$345$CVQEX.DX88WBK0NG8PB4 O/38TL6XDALLKLPQATHO.3ZPJMUAVQFSB1:+B*21V FWMC6SU439YU774475LJ2U5T02$VBSIMLQ3:6J.E1-1STM$4
```

**Clé publique** (Ed25519, hex) :

```text
d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a
```

=== "Python"

    ```python
    from claim169 import decode

    qr_data = "6BF590B20FFWJWG.FKJ05H7B0XKA8FA9DIWENPEJ/5P$DPQE88EB$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL580 B9%W5*B9C+9%R8646%86HKESED1/DRTC5UA QE$345$CVQEX.DX88WBK0NG8PB4 O/38TL6XDALLKLPQATHO.3ZPJMUAVQFSB1:+B*21V FWMC6SU439YU774475LJ2U5T02$VBSIMLQ3:6J.E1-1STM$4"

    public_key = bytes.fromhex(
        "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
    )

    result = decode(qr_data, verify_with_ed25519=public_key)

    print(f"ID: {result.claim169.id}")
    print(f"Name: {result.claim169.full_name}")
    print(f"Issuer: {result.cwt_meta.issuer}")
    print(f"Verified: {result.verification_status}")
    ```

=== "Rust"

    ```rust
    use claim169_core::Decoder;

    let qr_data = "6BF590B20FFWJWG.FKJ05H7B0XKA8FA9DIWENPEJ/5P$DPQE88EB$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL580 B9%W5*B9C+9%R8646%86HKESED1/DRTC5UA QE$345$CVQEX.DX88WBK0NG8PB4 O/38TL6XDALLKLPQATHO.3ZPJMUAVQFSB1:+B*21V FWMC6SU439YU774475LJ2U5T02$VBSIMLQ3:6J.E1-1STM$4";

    let public_key = hex::decode(
        "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
    )?;

    let result = Decoder::new(qr_data)
        .verify_with_ed25519(&public_key)?
        .decode()?;

    println!("ID: {:?}", result.claim169.id);
    println!("Name: {:?}", result.claim169.full_name);
    println!("Issuer: {:?}", result.cwt_meta.issuer);
    println!("Verified: {}", result.verification_status);
    ```

=== "TypeScript"

    ```typescript
    import { Decoder } from 'claim169';

    const qrData = "6BF590B20FFWJWG.FKJ05H7B0XKA8FA9DIWENPEJ/5P$DPQE88EB$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL580 B9%W5*B9C+9%R8646%86HKESED1/DRTC5UA QE$345$CVQEX.DX88WBK0NG8PB4 O/38TL6XDALLKLPQATHO.3ZPJMUAVQFSB1:+B*21V FWMC6SU439YU774475LJ2U5T02$VBSIMLQ3:6J.E1-1STM$4";

    const publicKey = Uint8Array.from(
      "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
        .match(/.{2}/g)!.map(b => parseInt(b, 16))
    );

    const result = new Decoder(qrData)
      .verifyWithEd25519(publicKey)
      .decode();

    console.log(`ID: ${result.claim169.id}`);
    console.log(`Name: ${result.claim169.fullName}`);
    console.log(`Issuer: ${result.cwtMeta.issuer}`);
    console.log(`Verified: ${result.verificationStatus}`);
    ```

=== "Kotlin"

    ```kotlin
    import fr.acn.claim169.Claim169

    val qrData = "6BF590B20FFWJWG.FKJ05H7B0XKA8FA9DIWENPEJ/5P\$DPQE88EB\$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL580 B9%W5*B9C+9%R8646%86HKESED1/DRTC5UA QE\$345\$CVQEX.DX88WBK0NG8PB4 O/38TL6XDALLKLPQATHO.3ZPJMUAVQFSB1:+B*21V FWMC6SU439YU774475LJ2U5T02\$VBSIMLQ3:6J.E1-1STM\$4"

    val publicKey = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
        .chunked(2).map { it.toInt(16).toByte() }.toByteArray()

    val result = Claim169.decode(qrData) {
        verifyWithEd25519(publicKey)
    }

    println("ID: ${result.claim169.id}")
    println("Name: ${result.claim169.fullName}")
    println("Issuer: ${result.cwtMeta.issuer}")
    println("Verified: ${result.isVerified}")
    ```

=== "Java"

    ```java
    import fr.acn.claim169.Claim169;
    import fr.acn.claim169.DecodeResultData;

    String qrData = "6BF590B20FFWJWG.FKJ05H7B0XKA8FA9DIWENPEJ/5P$DPQE88EB$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL580 B9%W5*B9C+9%R8646%86HKESED1/DRTC5UA QE$345$CVQEX.DX88WBK0NG8PB4 O/38TL6XDALLKLPQATHO.3ZPJMUAVQFSB1:+B*21V FWMC6SU439YU774475LJ2U5T02$VBSIMLQ3:6J.E1-1STM$4";

    byte[] publicKey = hexToByteArray(
        "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
    );

    DecodeResultData result = Claim169.decode(qrData, builder -> {
        builder.verifyWithEd25519(publicKey);
    });

    System.out.println("ID: " + result.getClaim169().getId());
    System.out.println("Name: " + result.getClaim169().getFullName());
    System.out.println("Issuer: " + result.getCwtMeta().getIssuer());
    System.out.println("Verified: " + Claim169.verificationStatus(result));
    ```

**Sortie attendue :**

```text
ID: ID-SIGNED-001
Name: Signed Test Person
Issuer: https://mosip.example.org
Verified: verified
```

## Choisir votre parcours

- **Vérifier (lire)** : décoder un QR code scanné et le vérifier avec la clé publique de l'émetteur.
- **Émettre (écrire)** : construire un payload Claim 169, le signer avec la clé privée de l'émetteur, puis l'encoder pour un QR code.

### Vérifier (Lire)

Vous avez besoin de :

- Le texte scanné (Base45)
- La clé publique de l'émetteur (et le bon algorithme : Ed25519 ou ECDSA P-256)

Commencez ici :

- [Python](../sdk/python/quick-start.md)
- [Rust](../sdk/rust/quick-start.md)
- [TypeScript](../sdk/typescript/quick-start.md)
- [Kotlin](../sdk/kotlin/quick-start.md)
- [Java](../sdk/java/quick-start.md)

### Émettre (Écrire)

Vous avez besoin de :

- Une **clé privée** d'émetteur (Ed25519 recommandé)
- Des métadonnées CWT (au minimum `issuer`, et souvent `issuedAt`/`expiresAt`)
- Un payload Claim 169 minimal (souvent `id` + `fullName`)

Commencez ici :

- [Python](../sdk/python/encoding.md)
- [Rust](../sdk/rust/encoding.md)
- [TypeScript](../sdk/typescript/encoding.md)
- [Kotlin](../sdk/kotlin/encoding.md)
- [Java](../sdk/java/encoding.md)

## Entrées de référence (vecteurs de test)

Pour décoder un QR déjà prêt (ou valider une autre implémentation), utilisez les [vecteurs de test](https://github.com/jeremi/claim-169/tree/main/test-vectors/valid) du dépôt :

- `test-vectors/valid/ed25519-signed.json` (signé)
- `test-vectors/valid/ecdsa-p256-signed.json` (signé)
- `test-vectors/valid/encrypted-signed.json` (chiffré + signé)

## Pour aller plus loin

- [Spécification](../core/specification.md) — format sur le fil, clés CBOR, tables de champs
- [Sécurité](../core/security.md) — modèle de menaces, valeurs sûres, validation
- [Glossaire](../core/glossary.md) — terminologie CBOR, COSE, CWT
- [Playground interactif](../playground.md) — essayez l'encodage et le décodage dans votre navigateur
