# Playground interactif

Essayez d’encoder et de décoder des identifiants Claim 169 directement dans votre navigateur.

<div class="playground-link" markdown>
[Ouvrir le Playground](https://jeremi.github.io/claim-169/){ .md-button .md-button--primary }
</div>

![Capture d’écran du Playground](../assets/img/playground.png)

## Fonctionnalités

Le playground utilise une interface unifiée à deux panneaux inspirée de [jwt.io](https://jwt.io), avec une synchronisation bidirectionnelle en temps réel entre les panneaux.

### Panneau gauche — Identité & paramètres

- **Champs d’identité** — Renseignez les données démographiques (nom, date de naissance, email, adresse, etc.)
- **Paramètres de l’identifiant** — Configuration groupée pour :
    - **Paramètres du jeton** — Émetteur, sujet et horodatages (repliables)
    - **Cryptographie** — Options de signature et de chiffrement
- **Clés auto-générées** — Nouvelles clés cryptographiques générées lors d’un changement de méthode
- **Charger des exemples** — Données de test pré-remplies et exemples de QR codes

### Panneau droit — QR code & vérification

- **Affichage du QR code** — QR code mis à jour en direct lorsque vous modifiez les champs
- **Badge de vérification** — Indique l’état de la signature (vérifiée, non vérifiée, invalide)
- **Données Base45** — Données encodées brutes avec un bouton de copie
- **Scanner QR** — Utilisez votre caméra pour scanner des QR codes existants
- **Détails du pipeline** — Vue extensible des étapes d’encodage

## Synchronisation en direct

Les changements se propagent automatiquement dans les deux sens :

- **Modifier les champs d’identité** → le QR code se régénère instantanément
- **Coller/scanner les données QR** → les champs d’identité se remplissent automatiquement

Aucun bouton « Generate » ou « Decode » n’est nécessaire.

## Démarrage rapide

### Créer un identifiant

1. Ouvrez le [Playground](https://jeremi.github.io/claim-169/)
2. Sélectionnez **Load example → Demo Identity** pour pré-remplir des données de test
3. Modifiez les champs d’identité si besoin
4. Le QR code se met à jour automatiquement
5. Téléchargez le PNG ou copiez les données Base45

### Vérifier un identifiant

1. Cliquez sur **Scan** pour scanner un QR code, ou collez les données Base45
2. Les champs d’identité se remplissent automatiquement
3. Pour vérifier la signature :
    - Collez la clé publique de l’émetteur dans le champ **Public Key**
    - Sélectionnez l’algorithme correct (Ed25519 ou ECDSA P-256)
4. Le badge de vérification affiche le résultat

### Gestion des clés

- **Bouton Generate** — Crée de nouvelles clés pour l'algorithme sélectionné
- **Clé publique** — Dérivée automatiquement à l'encodage, modifiable pour la vérification
- Les clés sont générées par session pour la sécurité (ne réutilisez jamais les clés du playground)

### Formats de clé supportés

Le playground détecte automatiquement et supporte plusieurs formats de clé :

**Clés publiques (pour la vérification) :**

- **Hex** — Octets bruts en chaîne hexadécimale (p. ex. `d75a980182b10ab7...`)
- **PEM** — Clés SPKI avec l'en-tête `-----BEGIN PUBLIC KEY-----`

**Clés de chiffrement (AES) :**

- **Hex** — Octets bruts en chaîne hexadécimale (p. ex. `0123456789abcdef...`)
- **Base64** — Encodage Base64 standard ou URL-safe

Un badge de format s'affiche à côté du champ de clé indiquant le format détecté (Hex, PEM, ou Base64).

## Technologie

Le playground s’exécute entièrement dans votre navigateur grâce à :

- **WebAssembly** — SDK claim169 compilé en WASM
- **React** — Framework UI moderne
- **Web Crypto API** — Génération de clés (Ed25519, ECDSA P-256, AES)
- **html5-qrcode** — Scan QR via la caméra

Aucune donnée n’est envoyée à un serveur.

## Code source

Le code source du playground est disponible ici :
[github.com/jeremi/claim-169/tree/main/playground](https://github.com/jeremi/claim-169/tree/main/playground)
