# Référence API Python

## Installation

```bash
pip install claim169
```

## Référence rapide

### Fonctions de décodage

```python
from claim169 import (
    decode_unverified,
    decode_with_ed25519,
    decode_with_ecdsa_p256,
    Decoder,
)
```

### Fonctions d'encodage

```python
from claim169 import (
    encode_with_ed25519,
    encode_with_ecdsa_p256,
    Encoder,
    Claim169Input,
    CwtMetaInput,
)
```

Pour la documentation complète, consultez la [référence API en anglais](../../en/api/python.md).
