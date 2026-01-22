# Playground interactif

Essayez l'encodage et le décodage des identifiants Claim 169 directement dans votre navigateur.

<div class="playground-link" markdown>
[:material-open-in-new: Ouvrir le Playground](https://jeremi.github.io/claim-169/){ .md-button .md-button--primary }
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

## Code source

Le code source du playground est disponible sur :
[github.com/jeremi/claim-169/tree/main/playground](https://github.com/jeremi/claim-169/tree/main/playground)
