# Playground interactif

Essayez l'encodage et le décodage des identifiants Claim 169 directement dans votre navigateur.

<div class="playground-link" markdown>
[Ouvrir le Playground](https://jeremi.github.io/
claim-169/){ .md-button .md-button--primary }
</div>

## Fonctionnalités

### Onglet Encoder

Créez de nouveaux identifiants avec :

- **Formulaire d'identité** - Remplissez les champs démographiques (nom, date de naissance, email, etc.)
- **Métadonnées CWT** - Définissez l'émetteur, le sujet et les horodatages
- **Signature** - Signez avec Ed25519 ou ECDSA P-256
- **Chiffrement** - Chiffrez optionnellement avec AES-128 ou AES-256
- **Génération QR** - Générez des codes QR scannables
- **Données d'exemple** - Chargez des données de test pré-remplies

### Onglet Décoder

Vérifiez les identifiants existants :

- **Coller les données QR** - Entrez les données QR encodées en Base45
- **Scanner un code QR** - Utilisez votre caméra pour scanner les codes QR
- **Vérification** - Vérifiez les signatures Ed25519 ou ECDSA P-256
- **Déchiffrement** - Déchiffrez les identifiants chiffrés AES

## Démarrage rapide

### Encoder un identifiant

1. Ouvrez le [Playground](../)
2. Cliquez sur **Load Sample** pour charger des données de test
3. Modifiez les champs d’identité si nécessaire
4. Cliquez sur **Generate QR Code**
5. Scannez le QR ou copiez le texte Base45

### Vérifier un identifiant

1. Passez à l’onglet **Decode**
2. Sélectionnez un exemple dans la liste, ou collez vos propres données QR
3. Entrez la clé publique (affichée lors de l’encodage)
4. Cliquez sur **Decode**
5. Consultez les données d’identité vérifiées

## Technologie

Le playground s’exécute entièrement dans votre navigateur :

- **WebAssembly** — SDK claim169 compilé en WASM
- **React** — Framework UI
- **html5-qrcode** — Scan QR via caméra

Aucune donnée n’est envoyée à un serveur.

## Code source

Le code source du playground est disponible sur :
[github.com/jeremi/claim-169/tree/main/playground](https://github.com/jeremi/claim-169/tree/main/playground)
