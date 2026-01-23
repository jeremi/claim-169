# Sécurité et validations

Cette page résume les paramètres de sécurité par défaut et les options disponibles selon votre contexte.

## Vérifier toujours en production

Les données d’un QR Claim 169 ne sont fiables qu’après validation.

- **Production** : vérifier les signatures (`Ed25519` ou `ECDSA P-256`)
- **Tests uniquement** : décoder sans vérification, mais considérer le résultat comme non fiable

!!! danger "Décodage non vérifié = non sécurisé"
    Sans vérification de signature, un QR code peut être falsifié. N’utilisez le décodage non vérifié que pour les vecteurs de test, le debug, ou si la vérification est faite ailleurs.

## Validation des timestamps (exp/nbf)

Les timestamps CWT permettent de rejeter les identifiants expirés ou pas encore valides :

- `exp` (expiration)
- `nbf` (not before)

### Les valeurs par défaut diffèrent selon le SDK

- **Rust** : validation des timestamps **activée par défaut**
- **Python** : validation des timestamps **activée par défaut**
- **TypeScript/WASM** : validation des timestamps **désactivée par défaut** (le WASM n’a pas accès de manière fiable à l’heure système)

## Limites de décompression

Le payload est compressé avec zlib. Pour éviter des attaques de type « zip bomb », le décodage impose une taille maximale après décompression.

- Limite par défaut : **64 KB** (`65536` octets)

Augmenter cette limite doit être fait avec précaution, surtout si la source n’est pas pleinement de confiance.

## Parsing des biométries

Les biométries peuvent être volumineuses. Si vous n’avez besoin que des données démographiques (nom, date de naissance, etc.), vous pouvez ignorer le décodage des biométries.

=== "Rust"

    ```rust
    let result = claim169_core::Decoder::new(qr_text)
        .skip_biometrics()
        .allow_unverified()
        .decode()?;
    ```

=== "Python"

    ```python
    import claim169

    result = claim169.decode_unverified(qr_text, skip_biometrics=True)
    ```

=== "TypeScript"

    ```ts
    import { Decoder } from "claim169";

    const result = new Decoder(qrText)
      .skipBiometrics()
      .allowUnverified()
      .decode();
    ```

## Ordre du chiffrement (signer puis chiffrer)

Lors de l’encodage chiffré :

1. **Signer** le CWT (`COSE_Sign1`)
2. **Chiffrer** le payload signé (`COSE_Encrypt0`)

Lors du décodage d’un payload chiffré :

1. **Déchiffrer**
2. **Vérifier**

## Recommandations

### Vérificateur (production)

- Exiger la vérification de signature
- Valider les timestamps (ou définir une politique explicite si l’horloge est peu fiable)
- Conserver des limites de décompression

### Émetteur (production)

- Toujours signer
- Chiffrer uniquement si la distribution de clé est maîtrisée
- Ne jamais réutiliser les nonces AES-GCM avec la même clé

