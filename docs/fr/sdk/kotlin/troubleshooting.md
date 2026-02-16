# Dépannage

## Erreurs courantes

### Signature invalide

- Verifiez que vous utilisez la bonne cle publique (et le bon algorithme : Ed25519 vs ECDSA P-256).
- Si `verificationStatus == VerificationStatus.Skipped`, la verification a ete ignoree (tests uniquement).

### Expire / NotYetValid

L'implementation cote JVM valide les timestamps (`exp` / `nbf`) et peut lever une erreur si :

- le jeton est expire
- le jeton n'est pas encore valide

### Base45 corrompu

Ne tronquez pas et ne normalisez pas la chaine Base45. L'alphabet Base45 inclut un caractere espace (`" "`).

## Chargement de la lib native (JNA)

Si la lib native ne se charge pas, verifiez :

- `java.library.path`
- `jna.library.path`

!!! note "Details"
    Pour des messages d'erreurs typiques et les configurations par OS/Android, basculez sur la version anglaise via le selecteur de langue (English).
