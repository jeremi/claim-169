# Playground interactivo

Prueba la codificación y decodificación de credenciales Claim 169 directamente en tu navegador.

<div class="playground-link" markdown>
[Abrir Playground](https://jeremi.github.io/claim-169/){ .md-button .md-button--primary }
</div>

## Funcionalidades

El playground utiliza un diseño unificado de dos paneles inspirado en [jwt.io](https://jwt.io), con sincronización bidireccional en tiempo real.

### Panel izquierdo - Identidad y configuración

- **Campos de identidad** - Completa datos demográficos (nombre, fecha de nacimiento, email, dirección, etc.)
- **Configuración del credential** - Configuración agrupada para:
    - **Configuración del token** - Emisor, sujeto y timestamps (colapsable)
    - **Criptografía** - Opciones de firma y cifrado
- **Claves auto-generadas** - Nuevas claves criptográficas generadas al cambiar de método
- **Cargar ejemplos** - Datos de prueba pre-rellenados y códigos QR de ejemplo

### Panel derecho - QR Code y verificación

- **Visualización del QR Code** - Código QR actualizado en tiempo real al editar
- **Badge de verificación** - Muestra el estado de la firma (verificado, no verificado, inválido)
- **Datos Base45** - Datos codificados con botón de copiar
- **Escáner QR** - Usa tu cámara para escanear códigos QR existentes
- **Detalles del pipeline** - Vista expandible de las etapas de codificación

## Sincronización en tiempo real

Los cambios fluyen automáticamente en ambas direcciones:

- **Editar campos de identidad** → El QR code se regenera instantáneamente
- **Pegar/escanear datos QR** → Los campos de identidad se completan automáticamente

No se necesitan botones de "Generar" o "Decodificar".

## Inicio rápido

### Crear un credential

1. Abre el [Playground](../)
2. Selecciona **Cargar ejemplo → Identidad demo** para cargar datos de prueba
3. Modifica los campos de identidad según necesites
4. El QR code se actualiza automáticamente
5. Descarga el PNG o copia los datos Base45

### Verificar un credential

1. Haz clic en **Escanear** para escanear un QR code, o pega datos Base45
2. Los campos de identidad se completan automáticamente
3. Para verificar la firma:
    - Pega la clave pública del emisor en el campo **Clave pública**
    - Selecciona el algoritmo correcto (Ed25519 o ECDSA P-256)
4. El badge de verificación muestra el resultado

### Gestión de claves

- **Botón Generar** - Crea nuevas claves para el algoritmo seleccionado
- **Clave pública** - Derivada automáticamente al codificar, editable para verificación
- Las claves se generan por sesión por seguridad (nunca reutilizar claves del playground)

## Tecnología

El playground se ejecuta completamente en tu navegador:

- **WebAssembly** - SDK claim169 compilado a WASM
- **React** - Framework UI moderno
- **Web Crypto API** - Generación de claves (Ed25519, ECDSA P-256, AES)
- **html5-qrcode** - Escaneo de QR por cámara

No se envía ningún dato a ningún servidor.

## Código fuente

El código fuente del playground está disponible en:
[github.com/jeremi/claim-169/tree/main/playground](https://github.com/jeremi/claim-169/tree/main/playground)
