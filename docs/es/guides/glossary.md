# Glosario

**Base45**  
Codificación de texto optimizada para el modo alfanumérico de QR. Claim 169 usa Base45 después de la compresión.

**CBOR** (Concise Binary Object Representation)  
Formato binario compacto. Claim 169 usa mapas CBOR con claves numéricas (p. ej. `1`, `4`, `8`) para minimizar tamaño.

**COSE** (CBOR Object Signing and Encryption)  
Estándar para firmar/cifrar payloads CBOR.

**COSE_Sign1**  
Estructura COSE para firmas. Claim 169 la usa para firmar el CWT.

**COSE_Encrypt0**  
Estructura COSE para cifrado autenticado (AEAD). Claim 169 puede cifrar el payload firmado para privacidad.

**CWT** (CBOR Web Token)  
Contenedor de token (similar a JWT pero basado en CBOR). Claim 169 guarda el payload de identidad bajo la clave `169` y usa claims estándar como `iss`, `exp`, `nbf`, `iat`.

**Ed25519 / EdDSA**  
Algoritmo de firma con clave pública de 32 bytes.

**ECDSA P-256 / ES256**  
Firma basada en P-256 + SHA-256. La clave pública se pasa como SEC1 (33 bytes comprimida o 65 bytes sin comprimir) o PEM/SPKI (Rust).

**AES-GCM**  
Cifrado autenticado. Claim 169 soporta AES-128-GCM y AES-256-GCM. Requiere un nonce/IV de 12 bytes único por cifrado.

**Emisor / Verificador**  
El emisor crea y firma (y opcionalmente cifra) credenciales. El verificador valida la firma y la política (timestamps, límites de tamaño, etc.) al decodificar.

**Vectores de prueba**  
Payloads QR conocidos (válidos e inválidos) en `test-vectors/` para validar comportamiento y reproducir casos límite.

