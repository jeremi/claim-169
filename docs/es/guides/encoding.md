# Codificación de credenciales

Esta guía cubre la creación de credenciales Claim 169 con todas las opciones disponibles.

## Campos de identidad

La especificación Claim 169 define los siguientes campos de identidad:

| Campo | Clave CBOR | Tipo | Descripción |
|-------|------------|------|-------------|
| `id` | 1 | string | Identificador único |
| `version` | 2 | string | Versión de la especificación |
| `language` | 3 | string | Idioma principal (ISO 639-3) |
| `fullName` | 4 | string | Nombre completo |
| `firstName` | 5 | string | Nombre |
| `middleName` | 6 | string | Segundo nombre |
| `lastName` | 7 | string | Apellido |
| `dateOfBirth` | 8 | string | Fecha de nacimiento (AAAAMMDD) |
| `gender` | 9 | integer | 1=Masculino, 2=Femenino, 3=Otro |
| `address` | 10 | string | Dirección completa |
| `email` | 11 | string | Correo electrónico |
| `phone` | 12 | string | Número de teléfono |
| `nationality` | 13 | string | Código de país |
| `maritalStatus` | 14 | integer | 1=Soltero/a, 2=Casado/a, 3=Divorciado/a |

Para más detalles, consulta la [documentación en inglés](../../en/guides/encoding.md).
