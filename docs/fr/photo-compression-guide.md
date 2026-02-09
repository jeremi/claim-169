# Guide de compression photo pour les QR codes Claim 169

Ce guide couvre les bonnes pratiques pour intégrer des photos d’identité dans les QR codes Claim 169. Les QR codes ont une capacité limitée (~2 331 caractères alphanumériques en version 40, niveau M), donc les photos doivent être compressées de manière agressive pour tenir aux côtés des données démographiques et des signatures cryptographiques.

## Budget d’octets

Un QR code Claim 169 typique en version 20 (~1 156 caractères Base45) dispose approximativement du budget interne suivant :

| Composant | Octets |
|-----------|--------|
| Surcharge COSE/CWT | 50-80 |
| Signature Ed25519 | 64 |
| Champs texte démographiques | 100-300 |
| **Photo** | **400-700** |
| **Total** | **~1 005** |

La spécification MOSIP vise 1 005 octets pour l’ensemble de la charge utile. Les signatures Ed25519 (64 octets) laissent nettement plus de place aux données photo que RSA-2048 (256 octets), ce qui explique en partie pourquoi la spécification privilégie EdDSA.

## Choix du format

À la résolution 32-64 px, **la surcharge du conteneur domine** la taille totale du fichier. Le choix du format devient donc critique :

| Format | Surcharge du conteneur | Code Claim 169 | Recommandation |
|--------|----------------------|----------------|----------------|
| WebP | ~30 octets | 4 | **Meilleur choix** |
| JPEG | ~155 octets | 1 | Solution de repli acceptable |
| JPEG2000 | ~200+ octets | 2 | À éviter pour les très petites images |
| AVIF | ~303 octets | 3 | À éviter pour les très petites images |

**WebP est le format optimal** pour ce cas d’usage. Son conteneur RIFF ajoute seulement ~20-30 octets de surcharge, et son codec VP8 utilise une prédiction en sous-blocs 4x4 qui s’adapte bien aux détails fins du visage. AVIF et JPEG2000 ont des algorithmes de compression supérieurs, mais leur surcharge de conteneur (303 et 200+ octets respectivement) consomme 37 à 75 % du budget octets photo avant même d’encoder un seul pixel.

JPEG reste une solution de repli acceptable pour les navigateurs sans prise en charge de l’encodage WebP (Safari < 16). Ses blocs DCT 8x8 produisent plus d’artefacts visibles à petite taille que les sous-blocs 4x4 de WebP, mais les tailles de fichier restent exploitables.

### Détection WebP côté navigateur

`canvas.toBlob("image/webp")` produit silencieusement un blob PNG sur les navigateurs qui ne prennent pas en charge l’encodage WebP. Vérifiez toujours `blob.type` après encodage :

```typescript
let blob = await canvasToBlob(canvas, "image/webp", quality)
if (blob.type !== "image/webp") {
  // Le navigateur ne supporte pas l'encodage WebP : repli sur JPEG
  blob = await canvasToBlob(canvas, "image/jpeg", quality)
}
```

Renseignez le champ `photoFormat` d’après le type MIME **réellement** produit, et non d’après le format demandé.

## Résolution

### Contexte des standards

Les standards formels de documents d’identité imposent des résolutions élevées qui ne s’appliquent pas aux photos intégrées dans un QR :

| Standard | Minimum | Objectif |
|----------|---------|----------|
| ISO/IEC 19794-5 | 90 px de distance inter-oculaire | Reconnaissance faciale automatisée |
| ICAO Doc 9303 | 300 DPI, 35x45 mm | Impression de passeport |
| ISO 18013-5 (mDL) | 192x240 px | Permis de conduire numérique |

Ces exigences visent la correspondance biométrique automatisée. Les photos intégrées au QR servent un autre objectif : **la vérification visuelle par un humain** (comparer la photo à la personne présente).

### Résolutions recommandées

| Résolution | Nombre de pixels | Taille estimée (WebP q=60) | Cas d’usage |
|-----------|------------------|----------------------------|-------------|
| 32x32 | 1 024 | 200-350 octets | Minimal, budget octets serré |
| **48x48** | **2 304** | **400-700 octets** | **Par défaut — meilleur compromis** |
| 56x56 | 3 136 | 500-900 octets | Meilleur détail si le budget le permet |
| 64x64 | 4 096 | 700-1 200 octets | Haute qualité, seulement pour un QR de grande taille |

**48x48 est la valeur par défaut recommandée.** Des travaux montrent que les humains peuvent reconnaître des visages dès 16x16 pixels (Harmon et Julesz), donc 48x48 offre une marge confortable pour la vérification visuelle tout en restant dans le budget octets.

Si le QR code a de la marge (peu de champs démographiques, version 40), 56x56 apporte un gain de détail notable pour seulement ~100-200 octets supplémentaires.

## Réglage de la qualité

Pour WebP en 48x48 :

| Qualité | Taille fichier | Qualité visuelle |
|---------|----------------|------------------|
| 40-50 % | Plus petite | Dégradation visible |
| **55-65 %** | **Bon compromis** | **Adéquate pour l’identification** |
| 65-80 % | 20-40 % plus grand | Amélioration visuelle marginale |
| 80-100 % | Rendement décroissant | Non justifié à cette résolution |

**60 % est recommandé.** Descendre plus bas n’économise souvent que 30-50 octets, avec une perte visuelle notable. Monter plus haut coûte 50-100+ octets pour un gain visuel minimal.

## Prétraitement

### Stratégie de recadrage

Utilisez un **recadrage carré centré en haut** :

- Les visages se trouvent généralement dans la partie supérieure de la photo
- En 48x48, même un décalage vertical d’un pixel compte
- Un recadrage carré est préférable au portrait 3:4, car les visages ont besoin d’autant de détail horizontal (oreilles, mâchoire) que vertical

### Flou gaussien

Appliquez un **léger flou gaussien (rayon 0,3 px)** avant encodage :

- Réduit le bruit capteur haute fréquence que les compresseurs gèrent mal
- Fait gagner 5-15 % de taille sans perte perceptible à 48x48
- À cette résolution, un flou 0,3 px retire le bruit sans lisser les traits du visage

```typescript
ctx.filter = "blur(0.3px)"
ctx.drawImage(source, sx, sy, size, size, 0, 0, dimension, dimension)
ctx.filter = "none"
```

### Couleur vs niveaux de gris

**Conservez les photos en couleur.** Des études montrent qu’à basse résolution, la couleur aide activement la reconnaissance visuelle humaine, car les indices de forme sont dégradés et la couleur apporte des indices discriminants supplémentaires. L’économie en octets du niveau de gris (~60-120 octets en 48x48) ne compense pas ce compromis. Si vous êtes extrêmement contraint en octets, le niveau de gris reste un dernier recours viable.

### Éviter l’égalisation d’histogramme

CLAHE ou l’égalisation d’histogramme peuvent augmenter la plage dynamique, donc l’entropie et la taille du fichier. En 48x48, le gain visuel est marginal et peut rendre la photo artificielle.

## Interaction du pipeline avec zlib

Le pipeline d’encodage Claim 169 applique la compression zlib après l’encodage photo :

```
Photo (WebP/JPEG) -> CBOR -> CWT -> COSE_Sign1 -> zlib -> Base45 -> QR
```

Les formats d’image déjà compressés (WebP, JPEG) produisent une sortie à forte entropie que zlib ne peut pratiquement pas recomprimer. En conséquence :

- **N’utilisez pas des pixels bruts / non compressés** en espérant que zlib fasse le travail. Une image RGB brute 48x48 fait 6 912 octets ; même avec zlib elle ferait ~2 300-3 500 octets. WebP à q=60 produit 400-700 octets.
- L’étape zlib bénéficie surtout aux parties texte/structure de la charge utile CBOR (champs démographiques, en-têtes COSE).
- Les octets photo traversent zlib quasiment inchangés.

## Ce que font des systèmes similaires

### QR Code Aadhaar (Inde)
- Utilise des photos JPEG, ~500-900 octets
- Signature RSA-2048 (256 octets), donc moins de place pour la photo qu’avec Ed25519
- UIDAI reconnaît que la photo est en « basse résolution » et peut ne pas suffire à reconnaître la personne

### Certificat COVID numérique de l’UE (DCC)
- Utilise le même pipeline (CBOR -> CWT -> COSE -> zlib -> Base45 -> QR)
- **N’inclut pas de photo** : les architectes l’ont écartée à cause des contraintes de taille du QR
- La vérification d’identité repose sur un document d’identité séparé

### Permis de conduire mobile (ISO 18013-5)
- **N’intègre pas la photo dans le QR code**
- Le QR code sert uniquement à l’engagement d’appareil (initialisation BLE/NFC/WiFi)
- La photo est transférée via le canal à plus grande bande passante
- Résolution photo minimale : 192x240 px (impossible à faire tenir dans un QR code)

### MOSIP Claim 169
- Vise 1 005 octets de charge utile totale
- Utilise Ed25519 (signature de 64 octets contre 256 octets en RSA pour Aadhaar)
- Le format photo est au choix de l’implémentateur ; WebP est recommandé pour l’efficacité de taille

## Référence rapide

Pour les implémentateurs qui veulent les valeurs recommandées par défaut :

```
Format:     WebP (code 4), repli JPEG (code 1)
Résolution: 48x48 pixels
Qualité:    60 %
Recadrage:  Centré en haut, carré
Flou:       Gaussien 0,3 px avant encodage
Couleur:    Conserver la couleur (ne pas convertir en niveaux de gris)
Cible:      400-700 octets
```

## Suivi de la capacité

Surveillez la longueur totale de la chaîne Base45 après encodage. Seuils de scannabilité du QR :

| Longueur Base45 | Statut | Action |
|-----------------|--------|--------|
| < 1 800 caractères | Sûr | Aucune action nécessaire |
| 1 800-2 100 caractères | Alerte | Envisager de réduire la photo ou de retirer des champs optionnels |
| > 2 100 caractères | Critique | Le QR peut ne pas être scanné de manière fiable au niveau M |

Ces seuils supposent un QR version 40, niveau de correction d’erreurs M. Les versions QR plus basses ont des limites proportionnellement plus faibles.
