# Playground interactivo

Prueba la codificación y decodificación de credenciales Claim 169 directamente en tu navegador.

<div class="playground-link" markdown>
[:material-open-in-new: Abrir Playground](https://jeremi.github.io/claim-169/){ .md-button .md-button--primary }
</div>

## Funcionalidades

### Pestaña Encode

Crea nuevas credenciales con:

- **Formulario de identidad** - Completa campos demográficos (nombre, fecha de nacimiento, email, etc.)
- **Metadatos CWT** - Define issuer, subject y timestamps
- **Firma** - Firma con Ed25519 o ECDSA P-256
- **Cifrado** - Opcionalmente cifra con AES-128 o AES-256
- **Generación de QR** - Genera códigos QR escaneables
- **Datos de ejemplo** - Carga datos de prueba pre-rellenados con claves demo

### Pestaña Decode

Verifica credenciales existentes:

- **Pegar datos QR** - Ingresa texto Base45
- **Escanear QR** - Usa la cámara para escanear
- **Verificación** - Verifica firmas Ed25519 o ECDSA P-256
- **Descifrado** - Descifra credenciales cifradas con AES
- **Ejemplos** - Carga ejemplos pre-hechos desde los test vectors

## Inicio rápido

### Codificar una credencial

1. Abre el [Playground](https://jeremi.github.io/claim-169/)
2. Haz clic en **Load Sample** para cargar datos de prueba
3. Ajusta los campos de identidad según necesites
4. Haz clic en **Generate QR Code**
5. Escanea el QR o copia el texto Base45

### Verificar una credencial

1. Cambia a la pestaña **Decode**
2. Selecciona un ejemplo del desplegable, o pega tus propios datos QR
3. Ingresa la clave pública (se muestra al codificar)
4. Haz clic en **Decode**
5. Revisa los datos de identidad verificados

## Tecnología

El playground se ejecuta completamente en tu navegador:

- **WebAssembly** - SDK claim169 compilado a WASM
- **React** - Framework UI
- **html5-qrcode** - Escaneo de QR por cámara

No se envía ningún dato a ningún servidor.

## Código fuente

El código fuente del playground está disponible en:
[github.com/jeremi/claim-169/tree/main/playground](https://github.com/jeremi/claim-169/tree/main/playground)

