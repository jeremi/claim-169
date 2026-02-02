# Quick Start

This guide covers the essential operations: encoding credentials and decoding QR codes using Java.

## Decoding a QR Code

The most common operation is decoding a QR code that was scanned from an identity credential.

!!! warning "Do not trim Base45"
    The Base45 alphabet includes a literal space character (`" "`). Preserve the scanned QR text exactly as-is (no `.trim()`, whitespace normalization, etc.), or you can corrupt valid credentials.

### With Ed25519 Verification

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.DecodeResultData;
import fr.acn.claim169.DecoderConfigurer;

// QR code content (Base45 encoded string from scanner)
String qrData = "NCFOXN...";

// Issuer's Ed25519 public key (32 bytes)
byte[] publicKey = hexToByteArray("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a");

// Decode and verify
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);
});

// Access identity data
System.out.println("ID: " + result.getClaim169().getId());
System.out.println("Name: " + result.getClaim169().getFullName());
System.out.println("Date of Birth: " + result.getClaim169().getDateOfBirth());

// Check verification status
String status = Claim169.verificationStatus(result);
System.out.println("Verification status: " + status);

// Access CWT metadata
System.out.println("Issuer: " + result.getCwtMeta().getIssuer());
System.out.println("Expires: " + result.getCwtMeta().getExpiresAt());
```

### Decode Without Verification

For testing and development only. Never use in production.

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.DecodeResultData;
import fr.acn.claim169.DecoderConfigurer;

// WARNING: INSECURE - skips signature verification
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.allowUnverified();
});

System.out.println("ID: " + result.getClaim169().getId());
System.out.println("Status: " + Claim169.verificationStatus(result));  // "skipped"
```

## Encoding a Credential

Create a signed credential that can be encoded in a QR code.

### Basic Encoding with Ed25519

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.Claim169Data;
import fr.acn.claim169.Claim169DataConfigurer;
import fr.acn.claim169.CwtMetaData;
import fr.acn.claim169.CwtMetaDataConfigurer;
import fr.acn.claim169.EncoderConfigurer;
import fr.acn.claim169.Gender;

// Create identity data using configurer
Claim169Data data = Claim169.claim169((Claim169DataConfigurer) builder -> {
    builder.setId("MOSIP-2024-001");
    builder.setFullName("Jane Doe");
    builder.setDateOfBirth("1990-05-15");
    builder.setGenderEnum(Gender.Female);
    builder.setEmail("jane.doe@example.org");
    builder.setNationality("US");
});

// Create CWT metadata using configurer
CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) builder -> {
    builder.setIssuer("https://id.example.org");
    builder.setExpiresAt(1900000000L);
    builder.setIssuedAt(1700000000L);
});

// Ed25519 private key (32 bytes) - keep this secret!
byte[] privateKey = hexToByteArray("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");

// Encode the credential
String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    builder.signWithEd25519(privateKey);
});

System.out.println("QR Code content (" + qrData.length() + " chars):");
System.out.println(qrData);
```

### Roundtrip Example

Encode a credential and immediately decode it to verify:

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.Claim169Data;
import fr.acn.claim169.Claim169DataConfigurer;
import fr.acn.claim169.CwtMetaData;
import fr.acn.claim169.CwtMetaDataConfigurer;
import fr.acn.claim169.DecodeResultData;
import fr.acn.claim169.DecoderConfigurer;
import fr.acn.claim169.EncoderConfigurer;

// Keys
byte[] privateKey = hexToByteArray("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");
byte[] publicKey = hexToByteArray("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a");

// Create and encode
Claim169Data data = Claim169.claim169((Claim169DataConfigurer) builder -> {
    builder.setId("TEST-001");
    builder.setFullName("Test User");
});

CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) builder -> {
    builder.setIssuer("https://test.example.org");
    builder.setExpiresAt(1900000000L);
});

String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    builder.signWithEd25519(privateKey);
});

// Decode and verify
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);
});

assert "TEST-001".equals(result.getClaim169().getId());
assert "Test User".equals(result.getClaim169().getFullName());
System.out.println("Roundtrip successful!");
```

## Error Handling

The SDK uses a sealed class hierarchy for exceptions. In Java, use `catch` blocks with specific exception types or `instanceof` checks:

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.DecoderConfigurer;
import uniffi.claim169_jni.Claim169Exception;

try {
    DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
        builder.verifyWithEd25519(publicKey);
    });
} catch (Claim169Exception.Base45Decode e) {
    System.out.println("Invalid QR code format: " + e.getMessage());
} catch (Claim169Exception.Decompress e) {
    System.out.println("Decompression failed: " + e.getMessage());
} catch (Claim169Exception.CoseParse e) {
    System.out.println("Invalid COSE structure: " + e.getMessage());
} catch (Claim169Exception.CwtParse e) {
    System.out.println("Invalid CWT structure: " + e.getMessage());
} catch (Claim169Exception.Claim169NotFound e) {
    System.out.println("Not a Claim 169 credential: " + e.getMessage());
} catch (Claim169Exception.SignatureInvalid e) {
    System.out.println("Signature verification failed: " + e.getMessage());
} catch (Claim169Exception.DecryptionFailed e) {
    System.out.println("Decryption failed: " + e.getMessage());
} catch (Claim169Exception.Expired e) {
    System.out.println("Token expired: " + e.getMessage());
} catch (Claim169Exception.NotYetValid e) {
    System.out.println("Token not yet valid: " + e.getMessage());
} catch (Claim169Exception e) {
    System.out.println("Decode error: " + e.getMessage());
}
```

## Working with Biometrics

### Checking for Biometric Data

```java
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);
});

if (result.getClaim169().hasBiometrics()) {
    System.out.println("Credential contains biometric data");

    // Check specific biometric types
    var faces = result.getClaim169().getFace();
    if (faces != null && !faces.isEmpty()) {
        var face = faces.get(0);
        System.out.println("Face photo: " + face.getData().length + " bytes, format=" + face.getFormat());
    }

    var thumbs = result.getClaim169().getRightThumb();
    if (thumbs != null && !thumbs.isEmpty()) {
        var thumb = thumbs.get(0);
        System.out.println("Right thumb: " + thumb.getData().length + " bytes");
    }
}
```

### Skipping Biometrics (Faster Decoding)

For use cases that do not need biometric data:

```java
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);
    builder.skipBiometrics();
});

// Biometric fields will be null
assert result.getClaim169().getFace() == null;
```

## Next Steps

- [Encoding Guide](encoding.md) -- Detailed encoding with all demographics
- [Decoding Guide](decoding.md) -- Advanced decoding options
- [Encryption](encryption.md) -- Add AES-GCM encryption
- [Custom Crypto](custom-crypto.md) -- HSM/KMS integration
