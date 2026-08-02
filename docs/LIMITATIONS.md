# Limitaciones técnicas conocidas

Documentación honesta de lo que **no** se puede reproducir automáticamente al
100%, y por qué.

## 1. Identificadores de Plasma (UUIDs)

`plasma-org.kde.plasma.desktop-appletsrc` y `kactivitymanagerdrc` contienen
identificadores (UUIDs de contenciones, applets y actividades) generados por el
sistema original. Al copiarse a otra máquina:

* Las contenciones y applets se recrean correctamente **por nombre de plugin**,
  pero los UUIDs nuevos pueden hacer que paneles en *pantallas* concretas
  (índice de monitor) se reposicionen si la topología de monitores difiere.
* **Recomendación**: configura los monitores antes (o con
  `--with-display-config` en el mismo hardware) y reinicia la sesión.

## 2. Dependencia de versiones concretas

* Los archivos de configuración de Plasma 6.7.x pueden ser incompatibles con
  Plasma 5 o con versiones futuras (Plasma 7). Los scripts usan herramientas
  `plasma-apply-*`, `kwriteconfig6`, `kbuildsycoca6`, `kquitapp6`,
  `qdbus6` — disponibles en Plasma 6.x.
* Los widgets de terceros tienen requisitos mínimos (p. ej. PlasMusic Toolbar
  requiere Plasma ≥ 6.0.4).
* Versión de Plasma del sistema de origen: **6.7.3**. Fedora 44 con el mismo
  repositorio la mantiene; en otras distros verifica la compatibilidad.

## 3. Dependencia de red

`dnf`, `flatpak` y la descarga de iconos **Tela-circle** requieren red. El
repositorio incluye los temas *Nordic* (pequeños) pero **no** los iconos Tela
(~150 MB) para mantener el repositorio git ligero. En entornos sin red, puedes
vendorizarlos manualmente:

```bash
git clone --depth 1 https://github.com/vinceliuice/Tela-circle-icon-theme.git /tmp/tela
cd /tmp/tela && bash ./install.sh -n Tela-circle black light dark
mkdir -p ~/dotfiles/local/share/icons
cp -a ~/.local/share/icons/Tela-circle-* ~/dotfiles/local/share/icons/
# y borra la linea correspondiente de .gitignore
```

## 4. Contenido con derechos de autor

Los wallpapers de `assets/wallpapers/` y el splash de Spider-Man proceden de
descargas de internet y del usuario. **Antes de publicar el repositorio
públicamente**, verifica la licencia de cada imagen o sustitúyelos por
alternativas libres (por ejemplo, los Nordic de `local/share/wallpapers/`, CC
BY-SA 4.0).

## 5. Cursores "Wii Pointer"

Es un cursor personalizado (generado con *currust*) de ~8 MB. Se incluye tal
cual. Si lo eliminas, cambia `kcminputrc → cursorTheme` a `Nordic-cursors` o
`breeze_cursors`.

## 6. GTK

El `colors.css` de GTK (generado por `kde-gtk-config` a partir del esquema
Nordic) y los assets de decoración de ventanas se copian. Si cambias el
esquema de color desde System Settings, KDE los regenerará; ejecuta
`./save.sh` para volver a capturarlos.

## 7. Datos dinámicos excluidos a propósito

No se reproducen: `~/.bash_history`, `recently-used.xbel`, restauración de
sesión (`~/.config/session`), caches (`~/.cache`), miniaturas, `klipper`
(historial del portapapeles), `baloo` (índice de archivos) y
`kactivitymanagerd-statsrc` (uso reciente).

## 8. Ventana de terminal abierta

El instalador reinicia `plasmashell` y reconfigura `kwin` si hay una sesión
Plasma activa. Para ver los cambios completos (splash, decoración) hay que
**cerrar sesión y volver a entrar**.
