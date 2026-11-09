# Icono de Most Anything

Dibujo propio de Grupo MOST. No queda nada del icono de la obra original:
su licencia de marca obliga a retirarlo en toda versión modificada, y en el
caso de Most Anything además no era código abierto.

**El dibujo.** Un azulejo de 512 a sangre con esquinas redondeadas (112 de
radio), degradado de `#10B981` a `#047857` en diagonal, y encima el motivo en
blanco: 3 trazo(s).
Dice de un vistazo qué hace el programa.

**Cómo se generan.** `tools/iconos.mjs` de la suite, que rasteriza con el
motor de Most Studio: el mismo barrido de líneas que usa el texto de área,
así que el icono es literalmente lo que Most Studio dibujaría. De ahí salen:

| Archivo | Para qué |
|---|---|
| `most-anything.svg` | La pieza canónica, 512 a sangre |
| `most-anything-small.svg` | La misma, para tamaños pequeños |
| `hicolor/<n>x<n>/apps/com.grupomostmexico.mostanything.png` | Tema de iconos de Linux |
| `most-anything-1024.png`, `-512`, `-256`, `-64` | Los que el código incrusta por nombre |
| `most-anything-macos-512.png` | Icono de ventana y Dock en macOS |
| `most-anything.icns` | Paquete de macOS |
| `most-anything.ico` | Ejecutable e instalador de Windows |

Si cambia el dibujo, se regenera todo de una vez. No se editan a mano: el
código los incrusta por nombre con `include_bytes!` y un archivo que falte
no es un aviso, es un error de compilación.

**Licencia.** Los dibujos son propiedad de Grupo MOST. `LICENSE.txt` es el
de la obra original y se conserva por lo que cubre de ella.
