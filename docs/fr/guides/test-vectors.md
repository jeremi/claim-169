# Vecteurs de test et conformité

Ce dépôt inclut des vecteurs de test JSON dans `test-vectors/` pour :

- valider votre intégration,
- comparer le comportement entre SDKs,
- reproduire des cas limites (expiré, champs inconnus, entrées malformées).

## Structure

- `test-vectors/valid/` — doit se décoder avec succès
- `test-vectors/edge/` — doit se décoder, mais peut dépendre de votre politique (validation des timestamps, etc.)
- `test-vectors/invalid/` — doit être rejeté

Chaque vecteur contient au minimum :

- `qr_data` (texte Base45)
- des clés optionnelles pour les tests (`public_key_hex`, `private_key_hex`, clés de chiffrement)
- `expected_claim169` et `expected_cwt_meta` pour vérification

!!! warning "Ne pas utiliser les clés des vecteurs en production"
    Les clés des vecteurs de test sont publiques et ne doivent jamais être utilisées pour des identifiants réels.

## Exemple : décoder un vecteur signé

Avec `test-vectors/valid/ed25519-signed.json` :

=== "Python"

    ```python
    import json
    import claim169

    v = json.load(open("test-vectors/valid/ed25519-signed.json"))
    public_key = bytes.fromhex(v["signing_key"]["public_key_hex"])

    result = claim169.decode_with_ed25519(v["qr_data"], public_key)
    print(result.claim169.full_name)
    ```

=== "TypeScript"

    ```ts
    import fs from "fs";
    import { Decoder, hexToBytes } from "claim169";

    const v = JSON.parse(fs.readFileSync("test-vectors/valid/ed25519-signed.json", "utf8"));
    const publicKey = hexToBytes(v.signing_key.public_key_hex);

    const result = new Decoder(v.qr_data).verifyWithEd25519(publicKey).decode();
    console.log(result.claim169.fullName);
    ```

## Script de conformité inter-langages

Un script de comparaison Python/TypeScript est disponible :

```bash
./scripts/conformance-test.sh
```

Notes :

- Le script désactive la validation des timestamps pour correspondre aux valeurs par défaut TypeScript/WASM.
- Il faut disposer des dépendances Python et TypeScript dans votre environnement.

