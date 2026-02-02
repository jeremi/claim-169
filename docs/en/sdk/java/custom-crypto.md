# Custom Crypto Providers

This guide covers integrating external cryptographic providers such as Android Keystore, Hardware Security Modules (HSMs), and cloud Key Management Services (KMS) using Java.

## Overview

The claim169 Java SDK uses interface-based crypto hooks, allowing you to:

- **Sign** with keys stored in Android Keystore, HSMs, or cloud KMS
- **Verify** signatures using external providers
- **Encrypt** with externally managed keys
- **Decrypt** using hardware-backed or cloud-hosted keys

This can help meet security requirements that mandate private key material stays within secure hardware (depending on your provider and configuration).

## Crypto Interfaces

### SignatureVerifier

Implement this interface to provide custom signature verification:

```java
import fr.acn.claim169.SignatureVerifier;

SignatureVerifier myVerifier = new SignatureVerifier() {
    @Override
    public void verify(
            String algorithm,      // "EdDSA" or "ES256"
            byte[] keyId,          // Optional key identifier from COSE header
            byte[] data,           // Signed data
            byte[] signature       // Signature to verify
    ) {
        // Throw any exception if verification fails
        // Return normally if verification succeeds
    }
};
```

### Signer

Implement this interface to provide custom signing:

```java
import fr.acn.claim169.Signer;

Signer mySigner = new Signer() {
    @Override
    public byte[] sign(String algorithm, byte[] keyId, byte[] data) {
        // Return signature bytes
        return doSign(data);
    }

    @Override
    public byte[] keyId() {
        return null; // Optional key identifier
    }
};
```

### Decryptor

Implement this interface to provide custom decryption:

```java
import fr.acn.claim169.Decryptor;

Decryptor myDecryptor = new Decryptor() {
    @Override
    public byte[] decrypt(
            String algorithm,      // "A256GCM" or "A128GCM"
            byte[] keyId,          // Optional key identifier from COSE header
            byte[] nonce,          // 12-byte nonce
            byte[] aad,            // Additional authenticated data
            byte[] ciphertext      // Encrypted data with authentication tag
    ) {
        // Return decrypted plaintext
        return doDecrypt(nonce, aad, ciphertext);
    }
};
```

### Encryptor

Implement this interface to provide custom encryption:

```java
import fr.acn.claim169.Encryptor;

Encryptor myEncryptor = new Encryptor() {
    @Override
    public byte[] encrypt(
            String algorithm,      // "A256GCM" or "A128GCM"
            byte[] keyId,          // Optional key identifier
            byte[] nonce,          // 12-byte nonce
            byte[] aad,            // Additional authenticated data
            byte[] plaintext       // Data to encrypt
    ) {
        // Return ciphertext with authentication tag
        return doEncrypt(nonce, aad, plaintext);
    }
};
```

## Android Keystore Integration

Android Keystore provides hardware-backed key storage on Android devices.

### Signing with Android Keystore

```java
import android.security.keystore.KeyGenParameterSpec;
import android.security.keystore.KeyProperties;
import fr.acn.claim169.Claim169;
import fr.acn.claim169.Signer;
import fr.acn.claim169.CoseAlgorithm;
import fr.acn.claim169.EncoderConfigurer;

import java.security.KeyPairGenerator;
import java.security.KeyStore;
import java.security.Signature;
import java.security.spec.ECGenParameterSpec;

// Generate an ECDSA P-256 key pair in the Keystore
public static void generateKeystoreKey(String alias) throws Exception {
    KeyPairGenerator keyPairGenerator = KeyPairGenerator.getInstance(
        KeyProperties.KEY_ALGORITHM_EC,
        "AndroidKeyStore"
    );
    keyPairGenerator.initialize(
        new KeyGenParameterSpec.Builder(alias,
                KeyProperties.PURPOSE_SIGN | KeyProperties.PURPOSE_VERIFY)
            .setDigests(KeyProperties.DIGEST_SHA256)
            .setAlgorithmParameterSpec(new ECGenParameterSpec("secp256r1"))
            .build()
    );
    keyPairGenerator.generateKeyPair();
}

// Signer using Android Keystore
Signer androidSigner = new Signer() {
    private final String keyAlias = "claim169-signing-key";
    private final KeyStore keyStore;

    {
        try {
            keyStore = KeyStore.getInstance("AndroidKeyStore");
            keyStore.load(null);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public byte[] sign(String algorithm, byte[] keyId, byte[] data) {
        try {
            java.security.PrivateKey privateKey =
                (java.security.PrivateKey) keyStore.getKey(keyAlias, null);
            Signature sig = Signature.getInstance("SHA256withECDSA");
            sig.initSign(privateKey);
            sig.update(data);
            byte[] derSignature = sig.sign();

            // Convert DER to raw r||s format (64 bytes)
            return derToRawEcdsa(derSignature);
        } catch (Exception e) {
            throw new RuntimeException("Signing failed", e);
        }
    }

    @Override
    public byte[] keyId() {
        return keyAlias.getBytes();
    }
};

// Usage
generateKeystoreKey("claim169-signing-key");

String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    builder.signWith(androidSigner, CoseAlgorithm.ES256);
});
```

### Verifying with Android Keystore

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.SignatureVerifier;
import fr.acn.claim169.DecoderConfigurer;

import java.security.KeyStore;
import java.security.Signature;

SignatureVerifier androidVerifier = new SignatureVerifier() {
    private final String keyAlias = "claim169-signing-key";
    private final KeyStore keyStore;

    {
        try {
            keyStore = KeyStore.getInstance("AndroidKeyStore");
            keyStore.load(null);
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public void verify(String algorithm, byte[] keyId, byte[] data, byte[] signature) {
        try {
            java.security.PublicKey publicKey = keyStore.getCertificate(keyAlias).getPublicKey();
            Signature sig = Signature.getInstance("SHA256withECDSA");
            sig.initVerify(publicKey);
            sig.update(data);

            // Convert raw r||s to DER format
            byte[] derSignature = rawToDerEcdsa(signature);

            if (!sig.verify(derSignature)) {
                throw new SecurityException("Signature verification failed");
            }
        } catch (SecurityException e) {
            throw e;
        } catch (Exception e) {
            throw new RuntimeException("Verification error", e);
        }
    }
};

// Usage
DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) builder -> {
    builder.verifyWith(androidVerifier);
});
```

### DER/Raw ECDSA Conversion Utilities

```java
import java.math.BigInteger;
import java.util.Arrays;

public static byte[] derToRawEcdsa(byte[] derSignature) {
    // Parse DER sequence: 0x30 <len> 0x02 <len> <r> 0x02 <len> <s>
    int offset = 2; // Skip SEQUENCE tag and length
    if (derSignature[0] != 0x30) {
        throw new IllegalArgumentException("Not a DER sequence");
    }

    // Read r
    if (derSignature[offset] != 0x02) {
        throw new IllegalArgumentException("Expected INTEGER tag");
    }
    offset++;
    int rLen = derSignature[offset] & 0xFF;
    offset++;
    BigInteger r = new BigInteger(1, Arrays.copyOfRange(derSignature, offset, offset + rLen));
    offset += rLen;

    // Read s
    if (derSignature[offset] != 0x02) {
        throw new IllegalArgumentException("Expected INTEGER tag");
    }
    offset++;
    int sLen = derSignature[offset] & 0xFF;
    offset++;
    BigInteger s = new BigInteger(1, Arrays.copyOfRange(derSignature, offset, offset + sLen));

    // Convert to fixed-size 32-byte arrays
    byte[] rBytes = toFixedLengthBytes(r, 32);
    byte[] sBytes = toFixedLengthBytes(s, 32);

    byte[] result = new byte[64];
    System.arraycopy(rBytes, 0, result, 0, 32);
    System.arraycopy(sBytes, 0, result, 32, 32);
    return result;
}

public static byte[] rawToDerEcdsa(byte[] rawSignature) {
    if (rawSignature.length != 64) {
        throw new IllegalArgumentException("Raw ECDSA signature must be 64 bytes");
    }

    BigInteger r = new BigInteger(1, Arrays.copyOfRange(rawSignature, 0, 32));
    BigInteger s = new BigInteger(1, Arrays.copyOfRange(rawSignature, 32, 64));

    byte[] rBytes = r.toByteArray();
    byte[] sBytes = s.toByteArray();

    int totalLen = 2 + rBytes.length + 2 + sBytes.length;
    byte[] der = new byte[2 + totalLen];
    int offset = 0;
    der[offset++] = 0x30; // SEQUENCE
    der[offset++] = (byte) totalLen;
    der[offset++] = 0x02; // INTEGER
    der[offset++] = (byte) rBytes.length;
    System.arraycopy(rBytes, 0, der, offset, rBytes.length);
    offset += rBytes.length;
    der[offset++] = 0x02; // INTEGER
    der[offset++] = (byte) sBytes.length;
    System.arraycopy(sBytes, 0, der, offset, sBytes.length);

    return der;
}

private static byte[] toFixedLengthBytes(BigInteger value, int length) {
    byte[] bytes = value.toByteArray();
    if (bytes.length == length) {
        return bytes;
    } else if (bytes.length > length) {
        return Arrays.copyOfRange(bytes, bytes.length - length, bytes.length);
    } else {
        byte[] padded = new byte[length];
        System.arraycopy(bytes, 0, padded, length - bytes.length, bytes.length);
        return padded;
    }
}
```

## AWS KMS Integration

### Signing with AWS KMS

```java
import fr.acn.claim169.Claim169;
import fr.acn.claim169.Signer;
import fr.acn.claim169.CoseAlgorithm;
import fr.acn.claim169.EncoderConfigurer;
import software.amazon.awssdk.core.SdkBytes;
import software.amazon.awssdk.services.kms.KmsClient;
import software.amazon.awssdk.services.kms.model.MessageType;
import software.amazon.awssdk.services.kms.model.SigningAlgorithmSpec;

public class AwsKmsSigner implements Signer {
    private final KmsClient kmsClient;
    private final String keyArn;

    public AwsKmsSigner(KmsClient kmsClient, String keyArn) {
        this.kmsClient = kmsClient;
        this.keyArn = keyArn;
    }

    @Override
    public byte[] sign(String algorithm, byte[] keyId, byte[] data) {
        var response = kmsClient.sign(builder -> {
            builder.keyId(keyArn);
            builder.message(SdkBytes.fromByteArray(data));
            builder.messageType(MessageType.RAW);
            builder.signingAlgorithm(SigningAlgorithmSpec.ECDSA_SHA_256);
        });

        // AWS KMS returns DER-encoded signature, convert to raw r||s
        return derToRawEcdsa(response.signature().asByteArray());
    }

    @Override
    public byte[] keyId() {
        return keyArn.getBytes();
    }
}

// Usage
KmsClient kmsClient = KmsClient.builder().build();
Signer signer = new AwsKmsSigner(kmsClient, "arn:aws:kms:us-east-1:123456789012:key/...");

String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    builder.signWith(signer, CoseAlgorithm.ES256);
});
```

### Verifying with AWS KMS

```java
import fr.acn.claim169.SignatureVerifier;
import software.amazon.awssdk.core.SdkBytes;
import software.amazon.awssdk.services.kms.KmsClient;
import software.amazon.awssdk.services.kms.model.MessageType;
import software.amazon.awssdk.services.kms.model.SigningAlgorithmSpec;

public class AwsKmsVerifier implements SignatureVerifier {
    private final KmsClient kmsClient;
    private final String keyArn;

    public AwsKmsVerifier(KmsClient kmsClient, String keyArn) {
        this.kmsClient = kmsClient;
        this.keyArn = keyArn;
    }

    @Override
    public void verify(String algorithm, byte[] keyId, byte[] data, byte[] signature) {
        // Convert raw r||s back to DER for AWS KMS
        byte[] derSignature = rawToDerEcdsa(signature);

        var response = kmsClient.verify(builder -> {
            builder.keyId(keyArn);
            builder.message(SdkBytes.fromByteArray(data));
            builder.messageType(MessageType.RAW);
            builder.signature(SdkBytes.fromByteArray(derSignature));
            builder.signingAlgorithm(SigningAlgorithmSpec.ECDSA_SHA_256);
        });

        if (!response.signatureValid()) {
            throw new SecurityException("AWS KMS signature verification failed");
        }
    }
}
```

## Azure Key Vault Integration

### Signing with Azure Key Vault

```java
import com.azure.identity.DefaultAzureCredentialBuilder;
import com.azure.security.keyvault.keys.cryptography.CryptographyClient;
import com.azure.security.keyvault.keys.cryptography.CryptographyClientBuilder;
import com.azure.security.keyvault.keys.cryptography.models.SignatureAlgorithm;
import fr.acn.claim169.Signer;
import fr.acn.claim169.CoseAlgorithm;

import java.security.MessageDigest;

public class AzureKeyVaultSigner implements Signer {
    private final CryptographyClient cryptoClient;

    public AzureKeyVaultSigner(CryptographyClient cryptoClient) {
        this.cryptoClient = cryptoClient;
    }

    @Override
    public byte[] sign(String algorithm, byte[] keyId, byte[] data) {
        try {
            // Azure requires pre-hashed data for ECDSA
            byte[] digest = MessageDigest.getInstance("SHA-256").digest(data);
            var result = cryptoClient.sign(SignatureAlgorithm.ES256, digest);
            return result.getSignature();
        } catch (Exception e) {
            throw new RuntimeException("Azure Key Vault signing failed", e);
        }
    }

    @Override
    public byte[] keyId() {
        return null;
    }
}

// Usage
var credential = new DefaultAzureCredentialBuilder().build();
CryptographyClient cryptoClient = new CryptographyClientBuilder()
    .credential(credential)
    .keyIdentifier("https://my-vault.vault.azure.net/keys/my-key/version")
    .buildClient();

Signer signer = new AzureKeyVaultSigner(cryptoClient);
String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    builder.signWith(signer, CoseAlgorithm.ES256);
});
```

## Google Cloud KMS Integration

### Signing with Google Cloud KMS

```java
import com.google.cloud.kms.v1.AsymmetricSignRequest;
import com.google.cloud.kms.v1.CryptoKeyVersionName;
import com.google.cloud.kms.v1.Digest;
import com.google.cloud.kms.v1.KeyManagementServiceClient;
import com.google.protobuf.ByteString;
import fr.acn.claim169.Signer;
import fr.acn.claim169.CoseAlgorithm;

import java.security.MessageDigest;

public class GcpKmsSigner implements Signer {
    private final KeyManagementServiceClient kmsClient;
    private final CryptoKeyVersionName keyVersionName;

    public GcpKmsSigner(KeyManagementServiceClient kmsClient, CryptoKeyVersionName keyVersionName) {
        this.kmsClient = kmsClient;
        this.keyVersionName = keyVersionName;
    }

    @Override
    public byte[] sign(String algorithm, byte[] keyId, byte[] data) {
        try {
            byte[] sha256Digest = MessageDigest.getInstance("SHA-256").digest(data);
            Digest digest = Digest.newBuilder()
                .setSha256(ByteString.copyFrom(sha256Digest))
                .build();

            var response = kmsClient.asymmetricSign(
                AsymmetricSignRequest.newBuilder()
                    .setName(keyVersionName.toString())
                    .setDigest(digest)
                    .build()
            );

            // GCP returns DER, convert to raw r||s
            return derToRawEcdsa(response.getSignature().toByteArray());
        } catch (Exception e) {
            throw new RuntimeException("GCP KMS signing failed", e);
        }
    }

    @Override
    public byte[] keyId() {
        return null;
    }
}

// Usage
KeyManagementServiceClient kmsClient = KeyManagementServiceClient.create();
CryptoKeyVersionName keyVersionName = CryptoKeyVersionName.of(
    "my-project", "us-east1", "my-keyring", "my-key", "1"
);

Signer signer = new GcpKmsSigner(kmsClient, keyVersionName);
String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    builder.signWith(signer, CoseAlgorithm.ES256);
});
```

## HSM/PKCS#11 Integration Pattern

For PKCS#11 HSMs using a JCA provider:

```java
import fr.acn.claim169.Signer;
import fr.acn.claim169.CoseAlgorithm;

import java.security.KeyStore;
import java.security.Signature;

public class Pkcs11Signer implements Signer {
    private final String keyAlias;
    private final char[] pin;
    private final KeyStore keyStore;

    public Pkcs11Signer(String keyAlias, char[] pin) throws Exception {
        this.keyAlias = keyAlias;
        this.pin = pin;
        this.keyStore = KeyStore.getInstance("PKCS11");
        this.keyStore.load(null, pin);
    }

    @Override
    public byte[] sign(String algorithm, byte[] keyId, byte[] data) {
        try {
            java.security.PrivateKey privateKey =
                (java.security.PrivateKey) keyStore.getKey(keyAlias, pin);
            Signature sig = Signature.getInstance("SHA256withECDSA");
            sig.initSign(privateKey);
            sig.update(data);
            byte[] derSignature = sig.sign();

            return derToRawEcdsa(derSignature);
        } catch (Exception e) {
            throw new RuntimeException("PKCS#11 signing failed", e);
        }
    }

    @Override
    public byte[] keyId() {
        return null;
    }
}

// Usage
Signer signer = new Pkcs11Signer("my-signing-key", "1234".toCharArray());
String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
    builder.signWith(signer, CoseAlgorithm.ES256);
});
```

## Error Handling

Wrap provider-specific exceptions so they surface as meaningful messages:

```java
import fr.acn.claim169.Signer;

public class SafeSigner implements Signer {
    private final Signer delegate;

    public SafeSigner(Signer delegate) {
        this.delegate = delegate;
    }

    @Override
    public byte[] sign(String algorithm, byte[] keyId, byte[] data) {
        try {
            return delegate.sign(algorithm, keyId, data);
        } catch (java.net.ConnectException e) {
            throw new RuntimeException("Crypto provider unavailable: " + e.getMessage(), e);
        } catch (SecurityException e) {
            throw new RuntimeException("Access denied to signing key: " + e.getMessage(), e);
        }
    }

    @Override
    public byte[] keyId() {
        return delegate.keyId();
    }
}

// Usage
Signer safeSigner = new SafeSigner(new AwsKmsSigner(kmsClient, keyArn));

try {
    String qrData = Claim169.encode(data, meta, (EncoderConfigurer) builder -> {
        builder.signWith(safeSigner, CoseAlgorithm.ES256);
    });
} catch (Claim169Exception e) {
    System.out.println("Encoding failed: " + e.getMessage());
}
```

## Best Practices

### Key Rotation

- Implement key versioning in your KMS
- Use the `keyId()` method to track key versions
- Plan for graceful key rotation

### Error Handling

- Catch and wrap provider-specific exceptions
- Implement retry logic for transient failures
- Log crypto operations for audit trails

### Performance

- Cache KMS clients and connections
- Consider connection pooling for HSMs
- Use `ExecutorService` or virtual threads for async KMS calls

### Security

- Use IAM roles/managed identities, not static credentials
- Enable audit logging on your KMS
- Implement proper key access controls
- On Android, use hardware-backed key attestation

## Next Steps

- [API Reference](api.md) -- Complete interface documentation
- [Troubleshooting](troubleshooting.md) -- Common errors and solutions
