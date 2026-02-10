# Dépannage

Erreurs fréquentes et solutions lors de l’utilisation du crate `claim169-core`.

## Erreurs de configuration

### "verification required but no verifier provided"

**Erreur :**
```
Claim169Error::DecodingConfig("verification required but no verifier provided - use allow_unverified() to skip")
```

**Cause :** vous avez appelé `decode()` sans fournir de vérificateur, ni autoriser un décodage non vérifié.

**Solution :** soit fournir un vérificateur, soit autoriser explicitement le mode non vérifié :

```rust
// Option 1 : fournir un vérificateur
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .decode()?;

// Option 2 : autoriser non vérifié (tests uniquement)
let result = Decoder::new(qr_content)
    .allow_unverified()
    .decode()?;
```

### "either call sign_with_*() or allow_unsigned()"

**Erreur :**
```
Claim169Error::EncodingConfig("either call sign_with_*() or allow_unsigned() before encode()")
```

**Cause :** vous avez appelé `encode()` sans fournir de signataire, ni autoriser l’encodage non signé.

**Solution :** soit signer l’identifiant, soit autoriser explicitement le mode non signé :

```rust
// Option 1 : signer l’identifiant
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&private_key)?
    .encode()?;

// Option 2 : autoriser non signé (tests uniquement)
let qr_data = Encoder::new(claim169, cwt_meta)
    .allow_unsigned()
    .encode()?;
```

## Erreurs de clé

### "invalid key format"

**Erreur :**
```
Claim169Error::Crypto("invalid key format: ...")
```

**Causes :**
- Mauvaise taille de clé (p. ex. 31 octets au lieu de 32 pour Ed25519)
- Mauvais format (p. ex. DER au lieu d’octets bruts)
- Données de clé corrompues

**Solutions :**

```rust
// Les clés Ed25519 doivent faire exactement 32 octets
let ed25519_private_key: [u8; 32] = /* ... */;
let ed25519_public_key: [u8; 32] = /* ... */;

// Les clés privées ECDSA P-256 font 32 octets
let p256_private_key: [u8; 32] = /* ... */;

// Les clés publiques ECDSA P-256 font 33 octets (compressée) ou 65 octets (non compressée)
let p256_public_key_compressed: [u8; 33] = /* ... */;
let p256_public_key_uncompressed: [u8; 65] = /* ... */;

// Les clés AES-256 doivent faire 32 octets
let aes256_key: [u8; 32] = /* ... */;

// Les clés AES-128 doivent faire 16 octets
let aes128_key: [u8; 16] = /* ... */;
```

### "key not found"

**Erreur :**
```
CryptoError::KeyNotFound
```

**Cause :** l’identifiant spécifie un key ID que votre verifier/decryptor ne trouve pas.

**Solution :** inspecter le key ID dans l’en-tête COSE et s’assurer que votre résolution de clé fonctionne :

```rust
impl SignatureVerifier for MyVerifier {
    fn verify(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
        signature: &[u8],
    ) -> CryptoResult<()> {
        // Logger le key ID pour debug
        if let Some(kid) = key_id {
            println!("Looking for key: {:?}", String::from_utf8_lossy(kid));
        }
        // ... suite de la vérification
    }
}
```

## Erreurs de signature

### "signature verification failed"

**Erreur :**
```
Claim169Error::SignatureInvalid("signature verification failed")
```

**Causes :**
- Mauvaise clé publique
- Identifiant modifié après signature
- Incohérence d’algorithme

**Debug :**

```rust
// Vérifier que vous utilisez la bonne paire de clés
let signer = Ed25519Signer::generate();
let public_key = signer.public_key_bytes();

// Encoder
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with(signer, iana::Algorithm::EdDSA)
    .encode()?;

// Décoder avec la MÊME clé publique
let result = Decoder::new(&qr_data)
    .verify_with_ed25519(&public_key)?  // Utiliser la clé correspondante
    .decode()?;
```

### "unsupported algorithm"

**Erreur :**
```
Claim169Error::UnsupportedAlgorithm("algorithm_name")
```

**Cause :** l’identifiant utilise un algorithme que votre vérificateur ne supporte pas.

**Algorithmes supportés :**
- EdDSA (Ed25519)
- ES256 (ECDSA P-256)
- A256GCM (AES-256-GCM)
- A128GCM (AES-128-GCM)

## Erreurs de déchiffrement

### "decryption failed"

**Erreur :**
```
Claim169Error::DecryptionFailed("...")
```

**Causes :**
- Mauvaise clé de déchiffrement
- Données corrompues ou altérées
- Incohérence d’algorithme

**Debug :**

```rust
// S’assurer que la même clé est utilisée pour chiffrer et déchiffrer
let aes_key: [u8; 32] = /* ... */;

// Encodage
let qr_data = Encoder::new(claim169, cwt_meta)
    .sign_with_ed25519(&signing_key)?
    .encrypt_with_aes256(&aes_key)?  // Cette clé
    .encode()?;

// Décodage
let result = Decoder::new(&qr_data)
    .decrypt_with_aes256(&aes_key)?  // Doit correspondre !
    .verify_with_ed25519(&public_key)?
    .decode()?;
```

## Erreurs de format

### "invalid Base45 encoding"

**Erreur :**
```
Claim169Error::Base45Decode("...")
```

**Causes :**
- QR code mal scanné
- Chaîne tronquée
- Caractères invalides

**Solutions :**
- Capturer le contenu complet du QR code
- Vérifier les espaces, retours à la ligne, etc.
- Vérifier que le QR code a été généré par un encodeur Claim 169

```rust
// Supprimer les espaces superflus
let qr_content = scanned_content.trim();
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .decode()?;
```

### "decompression failed"

**Erreur :**
```
Claim169Error::Decompress("...")
```

**Cause :** les données zlib sont corrompues ou invalides.

**Solution :** re-scanner ou régénérer l’identifiant.

### "decompression limit exceeded"

**Erreur :**
```
Claim169Error::DecompressLimitExceeded { max_bytes: 65536 }
```

**Cause :** la taille décompressée dépasse la limite de sécurité (64KB par défaut).

**Solutions :**

```rust
// Option 1 : augmenter la limite si vous attendez de gros identifiants
let result = Decoder::new(qr_content)
    .max_decompressed_bytes(131072)  // 128KB
    .verify_with_ed25519(&public_key)?
    .decode()?;

// Option 2 : ignorer la biométrie pour réduire la taille
let result = Decoder::new(qr_content)
    .skip_biometrics()
    .verify_with_ed25519(&public_key)?
    .decode()?;
```

### "invalid COSE structure"

**Erreur :**
```
Claim169Error::CoseParse("...")
```

**Cause :** les données CBOR ne respectent pas la structure COSE_Sign1 ou COSE_Encrypt0.

**Solution :** indique généralement un identifiant mal formé. Vérifier l’encodeur.

## Erreurs d’horodatages

### "credential expired"

**Erreur :**
```
Claim169Error::Expired(timestamp)
```

**Cause :** le claim `exp` est dans le passé.

**Solutions :**

```rust
// Option 1 : tolérance à la dérive d’horloge
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .clock_skew_tolerance(300)  // 5 minutes
    .decode()?;

// Option 2 : désactiver la validation (déconseillé)
let result = Decoder::new(qr_content)
    .verify_with_ed25519(&public_key)?
    .without_timestamp_validation()
    .decode()?;
```

### "credential not valid until"

**Erreur :**
```
Claim169Error::NotYetValid(timestamp)
```

**Cause :** le claim `nbf` est dans le futur.

**Solutions :** identiques à l’expiration — utiliser `clock_skew_tolerance()` ou `without_timestamp_validation()`.

## Erreurs de données manquantes

### "claim 169 not found"

**Erreur :**
```
Claim169Error::Claim169NotFound
```

**Cause :** la charge utile CWT ne contient pas la clé claim 169.

**Solution :** vérifier que l’identifiant a été encodé correctement avec les données Claim 169.

## Erreurs de compilation

### "cannot find function verify_with_ed25519"

**Cause :** le feature `software-crypto` n’est pas activé.

**Solution :** activer le feature dans `Cargo.toml` :

```toml
[dependencies]
claim169-core = { version = "0.1", features = ["software-crypto"] }
```

Ou utiliser les features par défaut :

```toml
[dependencies]
claim169-core = "0.2.0-alpha"
```

### "trait Signer is not implemented"

**Cause :** votre type personnalisé n’implémente pas le trait requis.

**Solution :** implémenter le trait :

```rust
use claim169_core::{Signer, CryptoResult};
use coset::iana;

impl Signer for MyCustomSigner {
    fn sign(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        // Votre implémentation
    }
}
```

### "Send + Sync not satisfied"

**Cause :** votre type crypto personnalisé n’est pas thread-safe.

**Solution :** encapsuler l’état non thread-safe dans `Arc<Mutex<T>>` :

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ThreadSafeSigner {
    inner: Arc<Mutex<UnsafeSigner>>,
}

impl Signer for ThreadSafeSigner {
    fn sign(
        &self,
        algorithm: iana::Algorithm,
        key_id: Option<&[u8]>,
        data: &[u8],
    ) -> CryptoResult<Vec<u8>> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let inner = self.inner.lock().await;
            inner.sign(algorithm, key_id, data)
        })
    }
}
```

## Problèmes de performance

### Les gros QR codes sont trop longs à encoder/décoder

**Solutions :**

1. Ignorer la biométrie si inutile :
   ```rust
   Encoder::new(claim169, cwt_meta)
       .skip_biometrics()
       .sign_with_ed25519(&key)?
       .encode()?
   ```

2. Utiliser des photos plus petites (compresser avant encodage)

3. Utiliser `without_biometrics()` sur le claim :
   ```rust
   let smaller_claim = claim169.without_biometrics();
   ```

### Utilisation mémoire élevée

**Solutions :**

1. Traiter les identifiants un par un (pas en batch)
2. Utiliser `skip_biometrics()` lorsque la biométrie est inutile
3. Fixer une limite `max_decompressed_bytes()` raisonnable

## Obtenir de l’aide

Si vous rencontrez un problème non couvert :

1. Consulter la [Référence API](./api.md) pour l’usage correct
2. Activer des logs de debug pour plus de détails
3. Ouvrir une issue GitHub avec :
   - Version Rust (`rustc --version`)
   - Version `claim169-core`
   - Code minimal de reproduction
   - Message d’erreur complet
