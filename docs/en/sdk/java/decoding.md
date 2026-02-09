# Decoding Credentials

This guide covers decoding and verifying identity credentials from QR codes using Java.

## Overview

Decoding follows these steps:

1. Receive Base45-encoded string from QR scanner
2. Choose verification method (Ed25519, ECDSA P-256, or custom)
3. Call `Claim169.decode()` with a `DecoderConfigurer` lambda
4. Access the decoded claim and metadata

## Decoding with Ed25519 Verification

The most common case using Ed25519 signatures:

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.DecodeResultData;
import fr.acn.claim169.DecoderConfigurer;

String qrData = "NCFOXN...";  // From QR scanner
byte[] publicKey = hexToByteArray("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a");

DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);
});

// Access identity data
System.out.println("ID: " + result.getClaim169().getId());
System.out.println("Name: " + result.getClaim169().getFullName());
System.out.println("DOB: " + result.getClaim169().getDateOfBirth());
System.out.println("Gender: " + result.getClaim169().getGender());

// Verification status
System.out.println("Status: " + Claim169.verificationStatus(result));
```

## Decoding with ECDSA P-256 Verification

For credentials signed with ECDSA P-256:

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.DecodeResultData;
import fr.acn.claim169.DecoderConfigurer;

String qrData = "NCFOXN...";
// SEC1 encoded P-256 public key (33 bytes compressed, or 65 bytes uncompressed)
byte[] publicKey = hexToByteArray("04...");

DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEcdsaP256(publicKey);
});

System.out.println("ID: " + result.getClaim169().getId());
System.out.println("Status: " + Claim169.verificationStatus(result));
```

## Decoding with PEM Public Keys

If you have public keys in PEM format (e.g., from OpenSSL or X.509 certificates), use the PEM decode methods.

### Ed25519 with PEM

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.DecodeResultData;
import fr.acn.claim169.DecoderConfigurer;

String qrData = "NCFOXN...";
String pemKey = "-----BEGIN PUBLIC KEY-----\n"
    + "MCowBQYDK2VwAyEA11qYAYKxCrfVS/7TyWQHOg7hcvPapjJa8CCWX4cBURo=\n"
    + "-----END PUBLIC KEY-----";

DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519Pem(pemKey);
});

System.out.println("Status: " + Claim169.verificationStatus(result));
```

### ECDSA P-256 with PEM

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.DecodeResultData;
import fr.acn.claim169.DecoderConfigurer;

String qrData = "NCFOXN...";
String pemKey = "-----BEGIN PUBLIC KEY-----\n"
    + "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE...\n"
    + "-----END PUBLIC KEY-----";

DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEcdsaP256Pem(pemKey);
});

System.out.println("Status: " + Claim169.verificationStatus(result));
```

## Decoding with Custom Verifier

For HSM, KMS, or custom crypto providers, implement the `SignatureVerifier` interface using an anonymous class:

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.DecodeResultData;
import fr.acn.claim169.DecoderConfigurer;
import fr.acn.claim169.SignatureVerifier;

SignatureVerifier customVerifier = new SignatureVerifier() {
    @Override
    public void verify(String algorithm, byte[] keyId, byte[] data, byte[] signature) {
        // Your crypto provider logic here
        // Throw an exception if verification fails
        yourCryptoProvider.verify(data, signature);
    }
};

DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWith(customVerifier);
});

System.out.println("Status: " + Claim169.verificationStatus(result));
```

## Decoding Without Verification

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
System.out.println("Status: " + Claim169.verificationStatus(result));  // VerificationStatus.Skipped
```

## Handling Timestamps

### Timestamp Validation

By default, the decoder validates timestamps (exp, nbf):

```java
// This will throw Claim169Exception.Expired or Claim169Exception.NotYetValid
// if the token is expired or not yet valid
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);
});
```

### Disabling Timestamp Validation

```java
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);
    builder.withoutTimestampValidation();
});
```

### Clock Skew Tolerance

For distributed systems with clock differences:

```java
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);
    builder.clockSkewTolerance(60);  // Allow 60 seconds of drift
});
```

## Decompression Limits

Protect against decompression bombs by controlling the maximum decompressed size:

```java
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);
    builder.maxDecompressedBytes(32_768);  // 32 KB limit (default is 65536)
});
```

## Skipping Biometrics

For faster decoding when biometrics are not needed:

```java
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);
    builder.skipBiometrics();
});

// Biometric fields will be null
assert result.getClaim169().getFace() == null;
```

## Closeable Decode (Memory Zeroization)

For applications handling sensitive biometric data, use `decodeCloseable()` to ensure
byte arrays are zeroized when you are done. Java's try-with-resources calls `close()` automatically:

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.CloseableDecodeResult;
import fr.acn.claim169.DecoderConfigurer;

try (CloseableDecodeResult result = Claim169.decodeCloseable(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);
})) {
    String name = result.getData().getClaim169().getFullName();
    byte[] photo = result.getData().getClaim169().getPhoto();
    // ... process credential
}
// All biometric and photo byte arrays are now zeroed
```

The `CloseableDecodeResult` implements `Closeable`, so the try-with-resources
block automatically calls `close()` which zeroizes photo, bestQualityFingers, and all
biometric data byte arrays.

## Accessing Decoded Data

### DecodeResultData

The decode function returns a `DecodeResultData` object:

```java
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWithEd25519(publicKey);
});

// The decoded identity claim
var claim = result.getClaim169();

// CWT metadata (issuer, timestamps)
var meta = result.getCwtMeta();

// Raw verification status string from decoded payload
String status = result.getVerificationStatus();

// Type-safe verification status enum
var statusEnum = Claim169.verificationStatus(result);
```

### Claim169Data Fields

```java
var claim = result.getClaim169();

// Demographics
claim.getId();                    // String (nullable)
claim.getVersion();               // String (nullable)
claim.getLanguage();              // String (nullable)
claim.getFullName();              // String (nullable)
claim.getFirstName();             // String (nullable)
claim.getMiddleName();            // String (nullable)
claim.getLastName();              // String (nullable)
claim.getDateOfBirth();           // String (nullable)
claim.getGender();                // Long (nullable, 1=Male, 2=Female, 3=Other)
// Use Gender.fromValue(claim.getGender()) for type-safe enum
claim.getAddress();               // String (nullable)
claim.getEmail();                 // String (nullable)
claim.getPhone();                 // String (nullable)
claim.getNationality();           // String (nullable)
claim.getMaritalStatus();         // Long (nullable, 1=Unmarried, 2=Married, 3=Divorced)
// Use MaritalStatus.fromValue(claim.getMaritalStatus()) for type-safe enum
claim.getGuardian();              // String (nullable)
claim.getPhoto();                 // byte[] (nullable)
claim.getPhotoFormat();           // Long (nullable, 1=JPEG, 2=JPEG2000, 3=AVIF, 4=WebP)
// Use PhotoFormat.fromValue(claim.getPhotoFormat()) for type-safe enum
claim.getSecondaryFullName();     // String (nullable)
claim.getSecondaryLanguage();     // String (nullable)
claim.getLocationCode();          // String (nullable)
claim.getLegalStatus();           // String (nullable)
claim.getCountryOfIssuance();     // String (nullable)

// Biometrics (each is List<BiometricData>, nullable)
claim.getRightThumb();
claim.getRightPointerFinger();
claim.getRightMiddleFinger();
claim.getRightRingFinger();
claim.getRightLittleFinger();
claim.getLeftThumb();
claim.getLeftPointerFinger();
claim.getLeftMiddleFinger();
claim.getLeftRingFinger();
claim.getLeftLittleFinger();
claim.getRightIris();
claim.getLeftIris();
claim.getFace();
claim.getRightPalm();
claim.getLeftPalm();
claim.getVoice();

// Helper method
claim.hasBiometrics();  // true if any biometric data present
```

### CwtMetaData Fields

```java
var meta = result.getCwtMeta();

meta.getIssuer();       // String (nullable) - Credential issuer
meta.getSubject();      // String (nullable) - Subject identifier
meta.getExpiresAt();    // Long (nullable) - Expiration timestamp (Unix epoch)
meta.getNotBefore();    // Long (nullable) - Not valid before timestamp
meta.getIssuedAt();     // Long (nullable) - Issuance timestamp
```

### BiometricData Fields

```java
var faces = result.getClaim169().getFace();
if (faces != null && !faces.isEmpty()) {
    var face = faces.get(0);
    face.getData();       // byte[] - Raw biometric data
    face.getFormat();     // Integer (nullable) - Format code
    face.getSubFormat();  // Integer (nullable) - Sub-format code
    face.getIssuer();     // String (nullable) - Biometric issuer
}
```

## Error Handling

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.DecoderConfigurer;
import fr.acn.claim169.Claim169Exception;

try {
    DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
        builder.verifyWithEd25519(publicKey);
    });
} catch (Claim169Exception.Base45Decode e) {
    System.out.println("QR code format error: " + e.getMessage());
} catch (Claim169Exception.Decompress e) {
    System.out.println("Decompression error: " + e.getMessage());
} catch (Claim169Exception.CoseParse e) {
    System.out.println("COSE parse error: " + e.getMessage());
} catch (Claim169Exception.CwtParse e) {
    System.out.println("CWT parse error: " + e.getMessage());
} catch (Claim169Exception.Claim169NotFound e) {
    System.out.println("Not a Claim 169 credential: " + e.getMessage());
} catch (Claim169Exception.SignatureInvalid e) {
    System.out.println("Invalid signature: " + e.getMessage());
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

## Complete Example

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.DecodeResultData;
import fr.acn.claim169.DecoderConfigurer;
import fr.acn.claim169.VerificationStatus;
import fr.acn.claim169.Claim169Exception;

import java.util.HashMap;
import java.util.Map;

public class CredentialVerifier {

    public static Map<String, Object> verifyCredential(String qrData, byte[] publicKey) {
        try {
            DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
                builder.verifyWithEd25519(publicKey);
                builder.clockSkewTolerance(60);
            });

            VerificationStatus status = Claim169.verificationStatus(result);
            if (status != VerificationStatus.Verified) {
                System.out.println("Warning: " + status);
                return null;
            }

            Map<String, Object> credential = new HashMap<>();
            credential.put("id", result.getClaim169().getId());
            credential.put("fullName", result.getClaim169().getFullName());
            credential.put("dateOfBirth", result.getClaim169().getDateOfBirth());
            credential.put("issuer", result.getCwtMeta().getIssuer());
            credential.put("expiresAt", result.getCwtMeta().getExpiresAt());
            credential.put("hasPhoto", result.getClaim169().getPhoto() != null);
            credential.put("hasBiometrics", result.getClaim169().hasBiometrics());
            return credential;

        } catch (Claim169Exception.SignatureInvalid e) {
            System.out.println("Invalid signature - credential may be tampered");
            return null;
        } catch (Claim169Exception e) {
            System.out.println("Failed to decode credential: " + e.getMessage());
            return null;
        }
    }

    public static void main(String[] args) {
        byte[] publicKey = hexToByteArray(
            "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
        );
        String qrData = "NCFOXN...";

        Map<String, Object> credential = verifyCredential(qrData, publicKey);
        if (credential != null) {
            System.out.println("Verified: " + credential.get("fullName"));
        }
    }
}
```

## Next Steps

- [Encryption](encryption.md) -- Decode encrypted credentials
- [Custom Crypto](custom-crypto.md) -- HSM/KMS integration
- [API Reference](api.md) -- Complete class documentation
