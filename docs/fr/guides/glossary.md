# Glossaire

**Base45**  
Encodage texte optimisé pour le mode alphanumérique des QR codes. Claim 169 utilise Base45 après compression.

**CBOR** (Concise Binary Object Representation)  
Format binaire compact. Claim 169 encode une map CBOR avec des clés numériques (par ex. `1`, `4`, `8`) pour minimiser la taille.

**COSE** (CBOR Object Signing and Encryption)  
Standard pour signer/chiffrer des payloads CBOR.

**COSE_Sign1**  
Structure COSE utilisée pour les signatures. Claim 169 l’utilise pour signer le CWT.

**COSE_Encrypt0**  
Structure COSE utilisée pour le chiffrement authentifié (AEAD). Claim 169 peut chiffrer le payload signé pour la confidentialité.

**CWT** (CBOR Web Token)  
Conteneur de token (analogue à un JWT mais basé sur CBOR). Claim 169 stocke le payload d’identité sous la clé `169` et utilise des claims standards comme `iss`, `exp`, `nbf`, `iat`.

**Ed25519 / EdDSA**  
Algorithme de signature avec clé publique de 32 octets.

**ECDSA P-256 / ES256**  
Algorithme de signature basé sur P-256 + SHA-256. La clé publique est fournie en SEC1 (33 octets compressés ou 65 octets non compressés) ou en PEM/SPKI (Rust).

**AES-GCM**  
Chiffrement authentifié. Claim 169 supporte AES-128-GCM et AES-256-GCM. Exige un nonce/IV de 12 octets unique par chiffrement.

**Émetteur / Vérificateur**  
L’émetteur crée et signe (et optionnellement chiffre) les identifiants. Le vérificateur contrôle signature et politique (timestamps, limites de taille, etc.) au décodage.

**Vecteurs de test**  
Payloads QR connus (valides et invalides) dans `test-vectors/` pour valider le comportement et reproduire des cas limites.

