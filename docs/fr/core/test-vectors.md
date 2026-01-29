# Vecteurs de test

Vecteurs de test pour valider des implémentations Claim 169.

## Vue d’ensemble

Les vecteurs de test sont des fichiers JSON contenant des entrées connues et des sorties attendues. Utilisez-les pour vérifier que votre implémentation gère correctement tous les cas.

## Emplacement

Les vecteurs de test se trouvent dans le répertoire `test-vectors/` :

```
test-vectors/
├── valid/           # Identifiants valides qui doivent décoder
├── invalid/         # Entrées invalides qui doivent échouer
└── edge/            # Cas limites (edge cases) et conditions aux bornes
```

## Générer des vecteurs de test

Générez des vecteurs à jour avec l’outil fourni :

```bash
cargo run -p generate-vectors
```

Cela crée des vecteurs avec des timestamps actuels et de nouvelles clés.

## Format des vecteurs

Chaque vecteur est un fichier JSON :

```json
{
  "name": "ed25519-signed",
  "description": "COSE_Sign1 with Ed25519 signature",
  "category": "valid",
  "qr_data": "NCFKXE...",
  "signing_key": {
    "algorithm": "EdDSA",
    "public_key_hex": "d75a9801..."
  },
  "expected_claim169": {
    "id": "ID-12345-ABCDE",
    "fullName": "Signed Test Person"
  },
  "expected_cwt_meta": {
    "issuer": "https://mosip.example.org",
    "expiresAt": 1800000000,
    "issuedAt": 1700000000
  }
}
```

Les vecteurs `invalid` / `edge` incluent un champ `expected_error` (par exemple `Base45Decode`, `Decompress`, `CoseParse`, `Claim169NotFound`).

## Vecteurs valides

### Identifiants de base

| Vecteur | Description |
|--------|-------------|
| `minimal.json` | Claim minimal avec ID et nom complet uniquement |
| `ed25519-signed.json` | COSE_Sign1 avec signature Ed25519 |
| `ecdsa-p256-signed.json` | COSE_Sign1 avec signature ECDSA P-256 |
| `demographics-full.json` | Tous les champs démographiques remplis |
| `with-face.json` | Identifiant avec biométrie visage |
| `with-fingerprints.json` | Identifiant avec biométrie empreintes digitales |
| `with-all-biometrics.json` | Identifiant avec tous les champs biométriques |
| `claim169-example.json` | Exemple de payload avec champs usuels |

### Identifiants chiffrés

| Vecteur | Description |
|--------|-------------|
| `encrypted-aes256.json` | COSE_Encrypt0 avec AES-256-GCM |
| `encrypted-signed.json` | COSE_Encrypt0 contenant un COSE_Sign1 signé |

## Vecteurs invalides

### Échecs de décodage

| Vecteur | Erreur attendue |
|--------|------------------|
| `bad-base45.json` | `Base45Decode` |
| `bad-zlib.json` | `Decompress` |
| `not-cose.json` | `CoseParse` |
| `missing-169.json` | `Claim169NotFound` |

## Cas limites

| Vecteur | Description |
|--------|-------------|
| `unknown_fields.json` | Contient des clés CBOR inconnues |
| `expired.json` | Jeton avec `exp` dans le passé |
| `not-yet-valid.json` | Jeton avec `nbf` dans le futur |

## Utiliser les vecteurs de test

### Rust

```rust
use std::fs;
use serde_json::Value;
use claim169_core::Decoder;

#[test]
fn test_basic_ed25519() {
    let json: Value = serde_json::from_str(
        &fs::read_to_string("test-vectors/valid/ed25519-signed.json").unwrap()
    ).unwrap();

    let qr_data = json["qr_data"].as_str().unwrap();
    let public_key = hex::decode(json["signing_key"]["public_key_hex"].as_str().unwrap()).unwrap();

    let result = Decoder::new(qr_data)
        .verify_with_ed25519(&public_key)
        .unwrap()
        .decode()
        .unwrap();

    assert_eq!(
        result.claim169.id.as_deref(),
        json["expected_claim169"]["id"].as_str()
    );
}
```

### Python

```python
import json
from claim169 import Decoder

def test_basic_ed25519():
    with open("test-vectors/valid/ed25519-signed.json") as f:
        vector = json.load(f)

    qr_data = vector["qr_data"]
    public_key = bytes.fromhex(vector["signing_key"]["public_key_hex"])

    result = Decoder(qr_data).verify_with_ed25519(public_key).decode()

    assert result.claim169.id == vector["expected_claim169"]["id"]
```

### TypeScript

```typescript
import { readFileSync } from 'fs';
import { Decoder } from 'claim169';

test('basic_ed25519', () => {
  const vector = JSON.parse(
    readFileSync('test-vectors/valid/ed25519-signed.json', 'utf-8')
  );

  const publicKey = Buffer.from(vector.signing_key.public_key_hex, 'hex');

  const result = new Decoder(vector.qr_data)
    .verifyWithEd25519(new Uint8Array(publicKey))
    .decode();

  expect(result.claim169.id).toBe(vector.expected_claim169.id);
});
```

## Tests de conformité

Pour valider une implémentation Claim 169 :

1. **Passer tous les vecteurs valides** — décoder avec succès et obtenir les sorties attendues
2. **Échouer sur tous les vecteurs invalides** — retourner le type d’erreur attendu
3. **Gérer tous les cas limites** — traiter des entrées inhabituelles mais valides

Une implémentation conforme doit passer tous les vecteurs de test.
