# Spécification MOSIP Claim 169

Cette bibliothèque implémente la spécification [MOSIP Claim 169](https://github.com/mosip/id-claim-169) pour les codes QR.

## Aperçu

Claim 169 est un format compact et sécurisé pour encoder les identifiants dans les codes QR. Il est conçu pour :

- **Vérification hors ligne** - Aucun réseau requis pour valider les identifiants
- **Taille compacte** - S'adapte aux codes QR standards
- **Sécurité** - Signatures numériques et chiffrement optionnel
- **Interopérabilité** - Basé sur des standards ouverts (CBOR, COSE, CWT)

## Pipeline d'encodage

```
Données d'identité → CBOR → CWT → COSE_Sign1 → [COSE_Encrypt0] → zlib → Base45 → Code QR
```

Pour la spécification complète, consultez la [documentation en anglais](../en/specification.md).
