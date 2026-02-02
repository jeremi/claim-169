# Encryption

This guide covers encrypting credentials with AES-GCM for additional privacy protection, using Java.

## When to Use Encryption

Encrypt credentials when:

- QR codes may be photographed by third parties
- Credentials contain sensitive biometric data
- Privacy regulations require data protection
- Credentials are shared across trust boundaries

## Encryption Overview

The library supports **sign-then-encrypt**: credentials are first signed, then the signed payload is encrypted.

```
Identity Data -> Sign -> Encrypt -> Compress -> Base45 -> QR Code
```

Decryption reverses the process:

```
QR Code -> Base45 -> Decompress -> Decrypt -> Verify -> Identity Data
```

## Supported Algorithms

| Algorithm | Key Size | Nonce Size | Use Case |
|-----------|----------|------------|----------|
| AES-256-GCM | 32 bytes | 12 bytes | High security (recommended) |
| AES-128-GCM | 16 bytes | 12 bytes | Standard security |

## Encoding with Encryption

### Sign + Encrypt with AES-256-GCM

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.Claim169Data;
import fr.acn.claim169.Claim169DataConfigurer;
import fr.acn.claim169.CwtMetaData;
import fr.acn.claim169.CwtMetaDataConfigurer;
import fr.acn.claim169.EncoderConfigurer;

// Identity data
Claim169Data data = Claim169.claim169((Claim169DataConfigurer) builder -> {
    builder.setId("ENC-001");
    builder.setFullName("Jane Doe");
});

CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) builder -> {
    builder.setIssuer("https://id.example.org");
    builder.setExpiresAt(1900000000L);
});

// Keys
byte[] signKey = hexToByteArray("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60");
    // Ed25519 private key (32 bytes)

byte[] encryptKey = hexToByteArray("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
    // AES-256 key (32 bytes)

// Encode with signing and encryption
String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    builder.signWithEd25519(signKey);
    builder.encryptWithAes256(encryptKey);
});

System.out.println("Encrypted credential: " + qrData.length() + " characters");
```

### Sign + Encrypt with AES-128-GCM

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.Claim169DataConfigurer;
import fr.acn.claim169.CwtMetaDataConfigurer;
import fr.acn.claim169.EncoderConfigurer;

Claim169Data data = Claim169.claim169((Claim169DataConfigurer) builder -> {
    builder.setId("ENC128-001");
    builder.setFullName("Jane Doe");
});

CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) builder -> {
    builder.setIssuer("https://id.example.org");
    builder.setExpiresAt(1900000000L);
});

byte[] signKey = new byte[32];    // Ed25519 private key
byte[] encryptKey = new byte[16]; // AES-128 key (16 bytes)

String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    builder.signWithEd25519(signKey);
    builder.encryptWithAes128(encryptKey);
});
```

## Decoding Encrypted Credentials

### Decrypt AES-256-GCM with Verification

For encrypted credentials, you typically need both:

1. Decryption key (symmetric AES key)
2. Verification key (signing public key) or verification callback

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.DecodeResultData;
import fr.acn.claim169.DecoderConfigurer;

// Keys
byte[] encryptKey = hexToByteArray("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
byte[] publicKey = hexToByteArray("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a");

// Decode, decrypt, and verify
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.decryptWithAes256(encryptKey);
    builder.verifyWithEd25519(publicKey);
});

System.out.println("ID: " + result.getClaim169().getId());
System.out.println("Status: " + Claim169.verificationStatus(result));
```

### Decrypt AES-128-GCM with Verification

```java
byte[] encryptKey128 = new byte[16];  // AES-128 key

DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.decryptWithAes128(encryptKey128);
    builder.verifyWithEd25519(publicKey);
});
```

### Decrypt Without Signature Verification

For testing only:

```java
// WARNING: INSECURE - skips signature verification
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.decryptWithAes256(encryptKey);
    builder.allowUnverified();
});

System.out.println("Status: " + Claim169.verificationStatus(result));  // "skipped"
```

## Custom Decryption

For HSM or KMS decryption, provide a `Decryptor` implementation using an anonymous class:

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.Decryptor;
import fr.acn.claim169.DecoderConfigurer;

Decryptor customDecryptor = new Decryptor() {
    @Override
    public byte[] decrypt(String algorithm, byte[] keyId, byte[] nonce, byte[] aad, byte[] ciphertext) {
        // Your HSM/KMS decryption logic
        return yourHsm.decrypt(algorithm, nonce, aad, ciphertext);
    }
};

DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.decryptWith(customDecryptor);
    builder.verifyWithEd25519(publicKey);
});
```

## Custom Encryption

For HSM or KMS encryption during encoding:

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.Encryptor;
import fr.acn.claim169.CoseAlgorithm;
import fr.acn.claim169.EncoderConfigurer;

Encryptor customEncryptor = new Encryptor() {
    @Override
    public byte[] encrypt(String algorithm, byte[] keyId, byte[] nonce, byte[] aad, byte[] plaintext) {
        // Your HSM/KMS encryption logic
        return yourHsm.encrypt(nonce, aad, plaintext);
    }
};

String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    builder.signWithEd25519(signKey);
    builder.encryptWith(customEncryptor, CoseAlgorithm.A256GCM);
});
```

The `CoseAlgorithm` enum provides type-safe algorithm selection. You can also pass a raw string:

```java
builder.encryptWith(customEncryptor, "A256GCM");
```

## Full Custom Crypto

Use custom callbacks for both signing and encryption:

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.Signer;
import fr.acn.claim169.Encryptor;
import fr.acn.claim169.SignatureVerifier;
import fr.acn.claim169.Decryptor;
import fr.acn.claim169.CoseAlgorithm;
import fr.acn.claim169.Claim169DataConfigurer;
import fr.acn.claim169.CwtMetaDataConfigurer;
import fr.acn.claim169.EncoderConfigurer;
import fr.acn.claim169.DecoderConfigurer;

// Custom signer
Signer signer = new Signer() {
    @Override
    public byte[] sign(String algorithm, byte[] keyId, byte[] data) {
        return yourSigningProvider.sign(data);
    }

    @Override
    public byte[] keyId() {
        return null;
    }
};

// Custom encryptor
Encryptor encryptor = new Encryptor() {
    @Override
    public byte[] encrypt(String algorithm, byte[] keyId, byte[] nonce, byte[] aad, byte[] plaintext) {
        return yourEncryptionProvider.encrypt(nonce, aad, plaintext);
    }
};

// Encode with both custom callbacks
Claim169Data data = Claim169.claim169((Claim169DataConfigurer) builder -> {
    builder.setId("CUSTOM-001");
    builder.setFullName("Jane Doe");
});

CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) builder -> {
    builder.setIssuer("https://id.example.org");
});

String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    builder.signWith(signer, CoseAlgorithm.EdDSA);
    builder.encryptWith(encryptor, CoseAlgorithm.A256GCM);
});

// Custom verifier and decryptor
SignatureVerifier verifier = new SignatureVerifier() {
    @Override
    public void verify(String algorithm, byte[] keyId, byte[] data, byte[] signature) {
        yourSigningProvider.verify(data, signature);
    }
};

Decryptor decryptor = new Decryptor() {
    @Override
    public byte[] decrypt(String algorithm, byte[] keyId, byte[] nonce, byte[] aad, byte[] ciphertext) {
        return yourEncryptionProvider.decrypt(nonce, aad, ciphertext);
    }
};

// Decode with custom callbacks
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.decryptWith(decryptor);
    builder.verifyWith(verifier);
});

System.out.println("ID: " + result.getClaim169().getId());
System.out.println("Status: " + Claim169.verificationStatus(result));
```

## Error Handling

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.DecoderConfigurer;
import uniffi.claim169_jni.Claim169Exception;

try {
    DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
        builder.decryptWithAes256(encryptKey);
        builder.allowUnverified();
    });
} catch (Claim169Exception.DecryptionFailed e) {
    // Decryption failed (wrong key, corrupted data, etc.)
    System.out.println("Decryption failed: " + e.getMessage());
} catch (IllegalArgumentException e) {
    // Invalid key size
    System.out.println("Invalid key: " + e.getMessage());
} catch (Claim169Exception e) {
    // Other errors
    System.out.println("Error: " + e.getMessage());
}
```

## Security Best Practices

### Key Management

- **Never hardcode keys** in source code
- **Use secure storage** (Android Keystore, HSM, KMS, secret manager)
- **Rotate keys** periodically
- **Limit key access** to authorized systems only

### Nonce Requirements

- **Never reuse nonces** with the same key
- The library generates random nonces automatically
- For custom encryption, always use cryptographically secure random nonces

### Key Distribution

- Distribute encryption keys through secure channels
- Consider using key derivation functions for shared secrets
- Implement proper key exchange protocols for distributed systems

## Next Steps

- [Custom Crypto](custom-crypto.md) -- HSM/KMS integration examples
- [API Reference](api.md) -- Complete class documentation
