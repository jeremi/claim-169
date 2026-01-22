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

Pour plus de détails, consultez la [documentation en anglais](../../en/guides/encryption.md).
