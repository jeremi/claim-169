# Encoding Credentials

This guide covers how to create Claim 169 credentials with all available options.

## Identity Fields

The Claim 169 specification defines the following identity fields:

| Field | CBOR Key | Type | Description |
|-------|----------|------|-------------|
| `id` | 1 | string | Unique identifier |
| `version` | 2 | string | Specification version |
| `language` | 3 | string | Primary language (ISO 639-3) |
| `fullName` | 4 | string | Complete name |
| `firstName` | 5 | string | Given name |
| `middleName` | 6 | string | Middle name(s) |
| `lastName` | 7 | string | Family name |
| `dateOfBirth` | 8 | string | Birth date (recommended: `YYYYMMDD`; commonly also `YYYY-MM-DD`) |
| `gender` | 9 | integer | 1=Male, 2=Female, 3=Other |
| `address` | 10 | string | Full address |
| `email` | 11 | string | Email address |
| `phone` | 12 | string | Phone number |
| `nationality` | 13 | string | Country code |
| `maritalStatus` | 14 | integer | 1=Unmarried, 2=Married, 3=Divorced |
| `guardian` | 15 | string | Guardian ID |
| `photo` | 16 | bytes | Photo data |
| `photoFormat` | 17 | integer | 1=JPEG, 2=JPEG2000, 3=AVIF, 4=WEBP |
| `bestQualityFingers` | 18 | array | Best quality finger positions (0-10) |
| `secondaryFullName` | 19 | string | Name in secondary language |
| `secondaryLanguage` | 20 | string | Secondary language |
| `locationCode` | 21 | string | Location code |
| `legalStatus` | 22 | string | Legal status |
| `countryOfIssuance` | 23 | string | Issuing country |

## Basic Example

Create a credential with common fields:

=== "Rust"

    ```rust
    use claim169_core::{Encoder, Claim169, CwtMeta, Gender, MaritalStatus};

    let claim = Claim169::new()
        .with_id("ID-12345-ABCDE")
        .with_full_name("Jane Marie Smith")
        .with_first_name("Jane")
        .with_middle_name("Marie")
        .with_last_name("Smith")
        .with_date_of_birth("19900515")
        .with_gender(Gender::Female)
        .with_email("jane.smith@example.com")
        .with_phone("+1 555 123 4567")
        .with_address("123 Main St\nNew York, NY 10001")
        .with_nationality("USA")
        .with_marital_status(MaritalStatus::Married);

    let meta = CwtMeta::new()
        .with_issuer("https://identity.example.org")
        .with_subject("ID-12345-ABCDE")
        .with_issued_at(1700000000)
        .with_expires_at(1800000000);
    ```

=== "Python"

    ```python
    from claim169 import Claim169Input, CwtMetaInput

    claim = Claim169Input(
        id="ID-12345-ABCDE",
        full_name="Jane Marie Smith",
        first_name="Jane",
        middle_name="Marie",
        last_name="Smith",
        date_of_birth="19900515",
        gender=2,  # Female
        email="jane.smith@example.com",
        phone="+1 555 123 4567",
        address="123 Main St\nNew York, NY 10001",
        nationality="USA",
        marital_status=2  # Married
    )

    meta = CwtMetaInput(
        issuer="https://identity.example.org",
        subject="ID-12345-ABCDE",
        issued_at=1700000000,
        expires_at=1800000000
    )
    ```

=== "TypeScript"

    ```typescript
    import { Claim169Input, CwtMetaInput } from 'claim169';

    const claim: Claim169Input = {
      id: "ID-12345-ABCDE",
      fullName: "Jane Marie Smith",
      firstName: "Jane",
      middleName: "Marie",
      lastName: "Smith",
      dateOfBirth: "19900515",
      gender: 2,  // Female
      email: "jane.smith@example.com",
      phone: "+1 555 123 4567",
      address: "123 Main St\nNew York, NY 10001",
      nationality: "USA",
      maritalStatus: 2  // Married
    };

    const meta: CwtMetaInput = {
      issuer: "https://identity.example.org",
      subject: "ID-12345-ABCDE",
      issuedAt: 1700000000,
      expiresAt: 1800000000
    };
    ```

## CWT Metadata

The CWT (CBOR Web Token) metadata contains standard claims:

| Field | CWT Claim | Description |
|-------|-----------|-------------|
| `issuer` | 1 (iss) | Credential issuer URI |
| `subject` | 2 (sub) | Subject identifier |
| `expiresAt` | 4 (exp) | Expiration timestamp (Unix) |
| `notBefore` | 5 (nbf) | Not valid before timestamp |
| `issuedAt` | 6 (iat) | Issuance timestamp |

!!! warning "Timestamp Validation"
    When decoding, timestamps are validated against the current time. Use `notBefore` and `expiresAt` to control the credential's validity window.

## Signing Algorithms

### Ed25519 (Recommended)

Ed25519 provides fast, secure signatures with small key sizes:

=== "Rust"

    ```rust
    let private_key: [u8; 32] = /* your Ed25519 private key */;

    let qr_data = Encoder::new(claim, meta)
        .sign_with_ed25519(&private_key)?
        .encode()?;
    ```

=== "Python"

    ```python
    private_key = bytes.fromhex("9d61b19deffd5a60...")  # 32 bytes

    qr_data = encode_with_ed25519(claim, meta, private_key)
    ```

=== "TypeScript"

    ```typescript
    const privateKey = hexToBytes("9d61b19deffd5a60..."); // 32 bytes

    const qrData = new Encoder(claim, meta)
      .signWithEd25519(privateKey)
      .encode();
    ```

### ECDSA P-256

ECDSA P-256 (ES256) is widely supported in existing PKI systems:

=== "Rust"

    ```rust
    let private_key: [u8; 32] = /* your ECDSA P-256 private key */;

    let qr_data = Encoder::new(claim, meta)
        .sign_with_ecdsa_p256(&private_key)?
        .encode()?;
    ```

=== "Python"

    ```python
    private_key = bytes.fromhex("...")  # 32 bytes

    qr_data = encode_with_ecdsa_p256(claim, meta, private_key)
    ```

=== "TypeScript"

    ```typescript
    const privateKey = hexToBytes("..."); // 32 bytes

    const qrData = new Encoder(claim, meta)
      .signWithEcdsaP256(privateKey)
      .encode();
    ```

### Custom Signer (HSM/KMS)

For production environments, private keys should never leave secure hardware. Use a custom signer callback to integrate with Hardware Security Modules (HSM), cloud Key Management Services (AWS KMS, Google Cloud KMS, Azure Key Vault), smart cards, TPMs, or remote signing services.

The callback receives:

- `algorithm`: The COSE algorithm name (e.g., `"EdDSA"`, `"ES256"`)
- `key_id`: Optional key identifier bytes (from COSE header, if present)
- `data`: The data to sign (COSE `Sig_structure`)

The callback must return the signature bytes.

=== "Rust"

    ```rust
    use claim169_core::{Encoder, Signer};

    struct HsmSigner {
        hsm_client: MyHsmClient,
        key_id: String,
    }

    impl Signer for HsmSigner {
        fn sign(&self, algorithm: &str, _key_id: Option<&[u8]>, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
            // Call your HSM to sign the data
            let signature = self.hsm_client.sign(&self.key_id, data)?;
            Ok(signature)
        }
    }

    let signer = HsmSigner {
        hsm_client: my_hsm,
        key_id: "my-signing-key".to_string(),
    };

    let qr_data = Encoder::new(claim, meta)
        .sign_with(&signer, "EdDSA")?
        .encode()?;
    ```

=== "Python"

    ```python
    from claim169 import encode_with_signer

    def my_signer(algorithm: str, key_id: bytes | None, data: bytes) -> bytes:
        """
        Custom signer callback for HSM/KMS integration.

        Args:
            algorithm: COSE algorithm name ("EdDSA", "ES256", etc.)
            key_id: Optional key identifier from COSE header
            data: The data to sign (COSE Sig_structure)

        Returns:
            Signature bytes
        """
        # Example: AWS KMS
        # response = kms_client.sign(
        #     KeyId='alias/my-signing-key',
        #     Message=data,
        #     SigningAlgorithm='ECDSA_SHA_256'
        # )
        # return response['Signature']

        # Example: PKCS#11 HSM
        return my_hsm.sign(key_id, data)

    qr_data = encode_with_signer(claim, meta, my_signer, "EdDSA")
    ```

=== "TypeScript"

    ```typescript
    import { Encoder, SignerCallback } from 'claim169';

    const mySigner: SignerCallback = async (
      algorithm: string,
      keyId: Uint8Array | null,
      data: Uint8Array
    ): Promise<Uint8Array> => {
      // Example: Google Cloud KMS
      // const [signResponse] = await kmsClient.asymmetricSign({
      //   name: keyVersionName,
      //   data: data,
      // });
      // return new Uint8Array(signResponse.signature);

      // Example: Azure Key Vault
      // const result = await cryptoClient.sign("ES256", data);
      // return result.result;

      return myHsm.sign(keyId, data);
    };

    const qrData = new Encoder(claim, meta)
      .signWith(mySigner, "EdDSA")
      .encode();
    ```

!!! tip "Key ID in COSE Header"
    You can include a key identifier in the COSE header to help the verifier locate the correct public key. This is useful when rotating keys or when multiple issuers share infrastructure.

## Adding Encryption

Encrypt the credential for privacy:

=== "Rust"

    ```rust
    let encryption_key: [u8; 32] = /* AES-256 key */;

    let qr_data = Encoder::new(claim, meta)
        .sign_with_ed25519(&signing_key)?
        .encrypt_with_aes256(&encryption_key)?
        .encode()?;
    ```

=== "Python"

    ```python
    from claim169 import encode_signed_encrypted

    encryption_key = bytes.fromhex("...")  # 32 bytes for AES-256

    # Python currently provides a convenience function for Ed25519 + AES-256-GCM:
    qr_data = encode_signed_encrypted(claim, meta, signing_key, encryption_key)
    ```

=== "TypeScript"

    ```typescript
    const encryptionKey = hexToBytes("..."); // 32 bytes for AES-256

    const qrData = new Encoder(claim, meta)
      .signWithEd25519(signingKey)
      .encryptWithAes256(encryptionKey)
      .encode();
    ```

See the [Encryption Guide](encryption.md) for more details.

## QR Code Generation

The encoded Base45 string can be rendered as a QR code using any QR library:

=== "Python"

    ```python
    import qrcode

    qr_data = encode_with_ed25519(claim, meta, private_key)

    img = qrcode.make(qr_data)
    img.save("credential.png")
    ```

=== "TypeScript"

    ```typescript
    import QRCode from 'qrcode';

    const qrData = new Encoder(claim, meta)
      .signWithEd25519(privateKey)
      .encode();

    await QRCode.toFile('credential.png', qrData);
    ```

!!! tip "Error Correction"
    Use QR error correction level "M" (Medium, ~15%) or "Q" (Quartile, ~25%) for credentials that may be printed and scanned repeatedly.
