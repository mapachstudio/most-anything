# Cambios respecto a la obra original

Most Anything es una obra derivada de **Craft Anything**, del equipo ArtCraft y
su comunidad (https://github.com/storytold/craft-anything), distribuida bajo Apache-2.0.

Este archivo cumple la cláusula 4(b) de Apache-2.0, que obliga a declarar los
cambios. Los archivos que admiten comentarios lo dicen además en su cabecera.

## Qué se ha cambiado

- **La marca.** Se ha retirado la de ArtCraft —nombre, wordmark, logo y marca
  «A»— y puesto la de Grupo MOST, como exige `docs/brand/LICENSE-brand.txt`
  de la obra original para toda versión modificada.
- Nombre de producto: `Craft Anything` → `Most Anything`.
- Identificador del paquete: `ai.storyteller.craft-anything` → `com.grupomostmexico.mostanything`.
- Extensión de archivo: `.craftanything` → `.mostanything`.
- Icono de la aplicación y marca de la interfaz, sustituidos por los nuestros.
- Carpeta `docs/brand/` de la obra original, eliminada.

Lo que NO se ha tocado: los nombres internos de los crates de Rust. Son
identificadores de código, no marcas presentadas al usuario.

## Archivos modificados sin aviso en cabecera

Su formato no admite comentarios:

- `crates/ui-egui/src/fr.tsv`
- `crates/ui-egui/src/i18n/cs.tsv`
- `crates/ui-egui/src/i18n/de.tsv`
- `crates/ui-egui/src/i18n/ko.tsv`
- `crates/ui-egui/src/i18n/ru.tsv`
- `crates/ui-egui/src/i18n/zh-hans.tsv`
- `crates/ui-egui/src/i18n/zh-hant.tsv`
- `crates/ui-egui/src/ja.tsv`
- `packaging/linux/ai.storyteller.craft-anything.metainfo.xml.in`
- `packaging/macos/Info.plist.in`
- `packaging/macos/entitlements.plist`
- `packaging/windows/portable.txt`

## Atribución

> Most Anything está basado en Craft Anything, del equipo ArtCraft y su
> comunidad (https://github.com/storytold/craft-anything), bajo licencia Apache-2.0. Grupo MOST no está
> asociado con el equipo ArtCraft, que no patrocina ni avala este producto.

Total de archivos modificados: 185.
