# Encodage des identifiants

Ce guide couvre la création d'identifiants Claim 169 avec toutes les options disponibles.

## Champs d'identité

La spécification Claim 169 définit les champs d'identité suivants :

| Champ | Clé CBOR | Type | Description |
|-------|----------|------|-------------|
| `id` | 1 | string | Identifiant unique |
| `version` | 2 | string | Version de la spécification |
| `language` | 3 | string | Langue principale (ISO 639-3) |
| `fullName` | 4 | string | Nom complet |
| `firstName` | 5 | string | Prénom |
| `middleName` | 6 | string | Deuxième prénom |
| `lastName` | 7 | string | Nom de famille |
| `dateOfBirth` | 8 | string | Date de naissance (AAAAMMJJ) |
| `gender` | 9 | integer | 1=Masculin, 2=Féminin, 3=Autre |
| `address` | 10 | string | Adresse complète |
| `email` | 11 | string | Adresse email |
| `phone` | 12 | string | Numéro de téléphone |
| `nationality` | 13 | string | Code pays |
| `maritalStatus` | 14 | integer | 1=Célibataire, 2=Marié(e), 3=Divorcé(e) |

Pour plus de détails, consultez la [documentation en anglais](../../en/guides/encoding.md).
