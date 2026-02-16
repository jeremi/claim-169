# Getting Started

Claim 169 QR codes are **Base45-encoded** strings that carry signed (and optionally encrypted) identity data.

!!! warning "Do not trim Base45"
    The Base45 alphabet includes a literal space character (`" "`). Preserve scanned QR text exactly as-is (no trimming or whitespace normalization), or you can corrupt valid credentials.

## Decode Your First QR Code

Here is a real Ed25519-signed Claim 169 payload from the project's test vectors. Follow along in your language of choice:

**QR data** (Base45):

```text
6BF590B20FFWJWG.FKJ05H7B0XKA8FA9DIWENPEJ/5P$DPQE88EB$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL580 B9%W5*B9C+9%R8646%86HKESED1/DRTC5UA QE$345$CVQEX.DX88WBK0NG8PB4 O/38TL6XDALLKLPQATHO.3ZPJMUAVQFSB1:+B*21V FWMC6SU439YU774475LJ2U5T02$VBSIMLQ3:6J.E1-1STM$4
```

**Public key** (Ed25519, hex):

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

**Expected output:**

```text
ID: ID-SIGNED-001
Name: Signed Test Person
Issuer: https://mosip.example.org
Verified: verified
```

## Choose Your Path

- **Verifier (read)**: decode a scanned QR code and verify it with the issuer's public key.
- **Issuer (write)**: build a Claim 169 payload, sign it with the issuer's private key, and encode it for a QR code.

### Verifier (Read)

You need:

- The scanned QR text (Base45)
- The issuer public key (and the right algorithm: Ed25519 or ECDSA P-256)

Start here:

- [Python](../sdk/python/quick-start.md)
- [Rust](../sdk/rust/quick-start.md)
- [TypeScript](../sdk/typescript/quick-start.md)
- [Kotlin](../sdk/kotlin/quick-start.md)
- [Java](../sdk/java/quick-start.md)

### Issuer (Write)

You need:

- An issuer **private key** (Ed25519 recommended)
- CWT metadata (at least `issuer`, and usually `issuedAt`/`expiresAt`)
- A minimal Claim 169 payload (often `id` + `fullName`)

Start here:

- [Python](../sdk/python/encoding.md)
- [Rust](../sdk/rust/encoding.md)
- [TypeScript](../sdk/typescript/encoding.md)
- [Kotlin](../sdk/kotlin/encoding.md)
- [Java](../sdk/java/encoding.md)

## Known-Good Inputs (Test Vectors)

If you want a ready-made QR payload to decode (or to validate another implementation), use the repo's [test vectors](https://github.com/jeremi/claim-169/tree/main/test-vectors/valid):

- `test-vectors/valid/ed25519-signed.json` (signed)
- `test-vectors/valid/ecdsa-p256-signed.json` (signed)
- `test-vectors/valid/encrypted-signed.json` (encrypted + signed)

## Next Steps

- [Specification](../core/specification.md) — wire format, CBOR keys, field tables
- [Security](../core/security.md) — threat model, secure defaults, validation
- [Glossary](../core/glossary.md) — CBOR, COSE, CWT terminology
- [Interactive Playground](../playground.md) — try encoding and decoding in your browser
