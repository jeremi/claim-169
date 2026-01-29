# Crypto personnalisée

Utilisez la crypto personnalisée si vous devez déléguer la signature/le déchiffrement à un HSM/KMS (ou autre fournisseur).

## Cas d’usage

- Clés privées non exportables (HSM)
- KMS cloud (AWS KMS / Azure Key Vault / GCP KMS)
- Séparation stricte des responsabilités (signer vs vérifier)

## Points d’intégration

Le SDK expose des interfaces de rappel (callbacks) pour brancher :

- `Signer` / `SignatureVerifier`
- `Encryptor` / `Decryptor`

Vous configurez ensuite l’encodeur/décodeur via `signWith(...)`, `verifyWith(...)`, `encryptWith(...)`, `decryptWith(...)`.

!!! note "Exemples complets"
    Pour des exemples détaillés (gestion de `kid`, mapping des algorithmes COSE, gestion des erreurs), basculez sur la version anglaise via le sélecteur de langue (English).
