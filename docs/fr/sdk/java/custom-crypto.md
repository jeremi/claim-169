# Crypto personnalisée

Utilisez la crypto personnalisée si vous devez déléguer la signature/le déchiffrement à un HSM/KMS (ou autre fournisseur).

## Cas d'usage

- Clés privées non exportables (HSM)
- KMS cloud (AWS KMS / Azure Key Vault / GCP KMS)
- Séparation stricte des responsabilités (signer vs vérifier)

## Points d'intégration

Le SDK expose des interfaces de rappel (callbacks) à implémenter comme classes anonymes Java :

- `SignatureVerifier` / `Signer`
- `Encryptor` / `Decryptor`

```java
import fr.acn.claim169.SignatureVerifier;
import fr.acn.claim169.VerificationResult;

SignatureVerifier verifier = new SignatureVerifier() {
    @Override
    public VerificationResult verify(String algorithm, byte[] keyId, byte[] data, byte[] signature) {
        boolean valid = yourHsm.verify(data, signature);
        return valid
            ? VerificationResult.Valid.INSTANCE
            : new VerificationResult.Invalid("HSM rejected signature");
    }
};

DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) b -> {
    b.verifyWith(verifier);
});
```

Vous configurez ensuite l'encodeur/décodeur via `signWith(...)`, `verifyWith(...)`, `encryptWith(...)`, `decryptWith(...)`.

!!! note "Exemples complets"
    Pour des exemples détaillés (Android Keystore, AWS KMS, Azure Key Vault, GCP KMS, HSM/PKCS#11), basculez sur la version anglaise via le sélecteur de langue (English).
