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
  "description": "Basic credential with Ed25519 signature",
  "qr_data": "NCFKXE...",
  "expected": {
    "claim169": {
      "id": "12345",
      "fullName": "John Doe"
    },
    "cwtMeta": {
      "issuer": "https://example.org",
      "expiresAt": 1735689600
    },
    "verificationStatus": "Verified"
  },
  "keys": {
    "ed25519_public": "b4f3..."
  }
}
```

## Vecteurs valides

### Identifiants de base

| Vecteur | Description |
|--------|-------------|
| `basic_ed25519.json` | Identifiant minimal avec signature Ed25519 |
| `basic_ecdsa.json` | Identifiant minimal avec signature ECDSA P-256 |
| `full_demographics.json` | Tous les champs démographiques remplis |
| `with_photo.json` | Identifiant avec photo embarquée |
| `with_biometrics.json` | Identifiant avec données biométriques |

### Identifiants chiffrés

| Vecteur | Description |
|--------|-------------|
| `encrypted_aes256.json` | Identifiant chiffré AES-256-GCM |
| `encrypted_aes128.json` | Identifiant chiffré AES-128-GCM |
| `signed_then_encrypted.json` | Signé puis chiffré (COSE_Encrypt0 enveloppant COSE_Sign1) |

### Variantes d’horodatages

| Vecteur | Description |
|--------|-------------|
| `with_expiry.json` | Identifiant avec expiration |
| `with_nbf.json` | Identifiant avec « not-before » |
| `with_all_timestamps.json` | `exp`, `nbf` et `iat` définis |

## Vecteurs invalides

### Échecs de décodage

| Vecteur | Erreur attendue |
|--------|------------------|
| `invalid_base45.json` | `Base45Decode` |
| `truncated_data.json` | `Decompress` |
| `invalid_cbor.json` | `CborParse` |
| `invalid_cose.json` | `CoseParse` |
| `missing_claim169.json` | `Claim169NotFound` |

### Échecs de signature

| Vecteur | Erreur attendue |
|--------|------------------|
| `wrong_signature.json` | `SignatureInvalid` |
| `wrong_key.json` | `SignatureInvalid` |
| `tampered_payload.json` | `SignatureInvalid` |

### Échecs d’horodatages

| Vecteur | Erreur attendue |
|--------|------------------|
| `expired.json` | `Expired` |
| `not_yet_valid.json` | `NotYetValid` |

## Cas limites

| Vecteur | Description |
|--------|-------------|
| `empty_fields.json` | Tous les champs optionnels absents |
| `unicode_names.json` | Noms UTF-8 dans plusieurs écritures |
| `max_photo_size.json` | Photo volumineuse proche de la limite |
| `unknown_fields.json` | Contient des clés CBOR inconnues |
| `zero_timestamps.json` | Horodatages à 0 |
| `max_timestamps.json` | Horodatages proches du maximum i64 |

## Utiliser les vecteurs de test

### Rust

```rust
use std::fs;
use serde_json::Value;
use claim169_core::Decoder;

#[test]
fn test_basic_ed25519() {
    let json: Value = serde_json::from_str(
        &fs::read_to_string("test-vectors/valid/basic_ed25519.json").unwrap()
    ).unwrap();

    let qr_data = json["qr_data"].as_str().unwrap();
    let public_key = hex::decode(json["keys"]["ed25519_public"].as_str().unwrap()).unwrap();

    let result = Decoder::new(qr_data)
        .verify_with_ed25519(&public_key)
        .unwrap()
        .decode()
        .unwrap();

    assert_eq!(
        result.claim169.id.as_deref(),
        json["expected"]["claim169"]["id"].as_str()
    );
}
```

### Python

```python
import json
from claim169 import Decoder

def test_basic_ed25519():
    with open("test-vectors/valid/basic_ed25519.json") as f:
        vector = json.load(f)

    qr_data = vector["qr_data"]
    public_key = bytes.fromhex(vector["keys"]["ed25519_public"])

    result = Decoder(qr_data).verify_with_ed25519(public_key).decode()

    assert result.claim169.id == vector["expected"]["claim169"]["id"]
```

### TypeScript

```typescript
import { readFileSync } from 'fs';
import { Decoder } from 'claim169';

test('basic_ed25519', () => {
  const vector = JSON.parse(
    readFileSync('test-vectors/valid/basic_ed25519.json', 'utf-8')
  );

  const publicKey = Buffer.from(vector.keys.ed25519_public, 'hex');

  const result = new Decoder(vector.qr_data)
    .verifyWithEd25519(new Uint8Array(publicKey))
    .decode();

  expect(result.claim169.id).toBe(vector.expected.claim169.id);
});
```

## Tests de conformité

Pour valider une implémentation Claim 169 :

1. **Passer tous les vecteurs valides** — décoder avec succès et obtenir les sorties attendues
2. **Échouer sur tous les vecteurs invalides** — retourner le type d’erreur attendu
3. **Gérer tous les cas limites** — traiter des entrées inhabituelles mais valides

Une implémentation conforme doit passer tous les vecteurs de test.
