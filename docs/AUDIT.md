# Auditoría del sistema (inventario)

Inventario completo detectado durante la fase de análisis (Fases 1-4). Este
documento es la fuente de verdad de **qué** se reproduce y **dónde** vive cada
elemento en el repositorio.

---

## 1. Sistema

| Atributo | Valor |
|---|---|
| Sistema operativo | Fedora Linux |
| Edición | 44 (KDE Plasma Desktop Edition) |
| Kernel | 7.1.5-201.fc44.x86_64 |
| Arquitectura | x86_64 |
| Hostname | BashHunter |
| Usuario | lownoise (uid 1000) |
| Locale | es_ES.UTF-8 (teclado es, pc105) |
| Gestor de sesión | **plasmalogin** (Plasma Login Manager, Plasma 6.7) |
| Sesión | KDE / Wayland (`XDG_SESSION_TYPE=wayland`) |
| Shell | bash 5.x |
| Gestores de paquetes | dnf + rpm, flatpak (remotes: flathub, fedora) |
| Paquetes RPM instalados | 2327 |

## 2. Identidad visual (tema "Nordic")

| Componente | Valor | Ubicación en el sistema | Repo |
|---|---|---|---|
| Look & Feel global | `Nordic` | `~/.config/kdeglobals [KDE] LookAndFeelPackage` | `config/kdeglobals` |
| Esquema de color | `Nordic` | `~/.local/share/color-schemes/Nordic.colors` | `local/share/color-schemes/` |
| Tema de escritorio Plasma | `Nordic` | `~/.config/plasmarc [Theme] name` | `config/plasmarc` |
| Tema de iconos | `Tela-circle-black-dark` | `~/.config/kdeglobals [Icons] Theme` | descarga en instalación |
| Cursores | `Wii Pointer` (tamaño 32) | `~/.config/kcminputrc` | `home/.icons/` |
| Decoración de ventanas | `Nordic` (Aurorae) | `~/.config/kwinrc`, `kdedefaults` | `config/`, `local/share/aurorae/` |
| Widget style Qt | `Breeze` | `kdedefaults/kdeglobals` | `config/kdedefaults/` |
| Splash screen | `spider-man_splash_animated` | `~/.config/ksplashrc` | `local/share/plasma/look-and-feel/` |
| Tema GTK | `Breeze-Dark` + `colors.css` generado de Nordic | `~/.config/gtk-3.0`, `gtk-4.0` | `config/gtk-3.0`, `config/gtk-4.0` |
| Fuentes | Noto Sans 10 (predeterminadas; **sin Nerd Fonts**) | — | — |

Temas adicionales presentes (alternativas): esquemas `NordicDarker`,
`nordicbluish`; desktop-themes `Nordic-bluish`, `Nordic-bluish-solid`,
`Nordic-darker`, `Nordic-darker-solid`, `Nordic-Solid`; look-and-feel
`Nordic-bluish`, `Nordic-darker`; iconos `Nordic-bluish`, `Nordic-darker`,
`Nordic-green`; iconos Tela `Tela-circle-black`, `Tela-circle-black-dark`,
`Tela-circle-black-light`.

## 3. Escritorio y paneles

Configuración completa en `~/.config/plasma-org.kde.plasma.desktop-appletsrc`
(repo: `config/`), `plasmashellrc` y `kwinrc`.

* **Escritorios virtuales**: 1 (KWin).
* **Actividad**: `Por omisión` (id `7a80688c-…`).
* **4 paneles flotantes** (grosor 46, todos con Panel Colorizer + Kurve + PlasMusic):
  1. Panel superior (containment 3): Kickoff, colorizer, espaciador, Kurve ×2,
     PlasMusic, espaciador, bandeja del sistema (21 entradas), reloj, mostrar
     escritorio.
  2. Panel inferior / dock (containment 30): Kickoff + gestor de tareas.
  3. (containments 56, 58): variantes con sistema de bandeja.
* **Widgets de terceros en uso**: `luisbocanegra.panel.colorizer` (preset
  "Carbon"), `luisbocanegra.audio.visualizer` (Kurve, usa shaders de cava),
  `plasmusic-toolbar`.
* **Icono del lanzador**: `~/Imágenes/Icons/IconoSpider.png` (vendored en
  `assets/icons/`).
* **Wallpapers** (`~/Imágenes/WallPapers/`):
  * Activo en escritorio: `miles-morales-spider-man_5120x2880_xtrafondos.com.jpg`
  * Otros: `1360883.jpeg`, `bsod.png` (también fondo de pantalla de bloqueo),
    `choso-red-aesthetic-3840x2160-26047.jpg`, `sushi.jpg`
  * Nordic: `Nordic-mountain-wallpaper.jpg`, `nordic-wallpaper.jpg`
    (`~/.local/share/wallpapers/`)

## 4. KWin

* `~/.config/kwinrc` + `kwinrulesrc` (sin reglas).
* Efectos activados: bouncing windows, cube, magic lamp, translucidez,
  wobbly windows; `scale`/`squash` desactivados.
* Efectos de terceros (`~/.local/share/kwin/effects/`): `bouncingWindows`,
  `kwin6_effect_pixelate` (Burn-My-Windows).
* Tiling: layout 25/50/25 con padding 4 en ambas pantallas.
* `Xwayland Scale=1.25`.
* **Pantallas** (`kwinoutputconfig.json`, específico de hardware):
  * eDP-1: 2560×1440 @165 Hz, escala 1.25, sRGB, VRR Never.
  * HDMI-A-1: 1920×1080 @165 Hz, escala 1, posición (0,0).
* Atajos globales personalizados (`kglobalshortcutsrc`): `Meta+W` vista
  general, `Meta+C` cubo, `Meta+T` editor de mosaicos, `Meta+Esc`… (archivo
  completo en el repo).

## 5. Aplicaciones de KDE

| App | Config en el sistema | Repo |
|---|---|---|
| Konsole | `~/.config/konsolerc` (perfil `LowNoiseProfile.profile`) | `config/konsolerc`, `local/share/konsole/` |
| Dolphin | `~/.config/dolphinrc` (menú oculto) | `config/dolphinrc` |
| Descubrimiento | `~/.config/discoverrc` | `config/discoverrc` |
| Bloqueo de pantalla | `~/.config/kscreenlockerrc` (timeout 10 s) | `config/kscreenlockerrc` |
| Portapapeles/barra | widgets configurados en appletsrc | `config/` |
| GTK (kde-gtk-config) | `gtk-3.0`, `gtk-4.0`, `xsettingsd` | `config/gtk-*`, `config/xsettingsd` |

## 6. Herramientas CLI y dotfiles

| Herramienta | Config | Repo |
|---|---|---|
| bash | `~/.bashrc`, `~/.bash_profile`, `~/.profile`, `~/.bash_logout` (prompt personalizado con git-prompt + colores + fastfetch al arranque) | `home/` |
| fastfetch | `~/.config/fastfetch/config.jsonc` + logos `spider1-4.txt` | `config/fastfetch`, `assets/fastfetch/logos/` |
| cava | shaders en `~/.config/cava/shaders/` (usados por Kurve) | `config/cava` |
| git | 2.55.0 (sin `~/.gitconfig`) | — |
| tmux | instalado, sin config | — |
| vim/nano | instalados, sin config | — |
| JetBrains | Toolbox + IntelliJ IDEA + Android Studio (desarrollo) | **no se reproduce** (manual) |
| opencode | `~/.opencode` | **no se reproduce** (instalación propia) |

## 7. Aplicaciones de escritorio (Flatpak)

* `com.discordapp.Discord`, `com.spotify.Client` → `packages/flatpak.txt`.
* Runtimes: `org.freedesktop.Platform`, `org.freedesktop.Platform.GL.default`,
  `org.freedesktop.Platform.VAAPI.Intel`, `codecs-extra`,
  `org.gtk.Gtk3theme.Breeze`.

## 8. Servicios de usuario y autostart

* `~/.config/autostart/`: solo JetBrains Toolbox (manual).
* `~/.config/systemd/user/`: vacío. Sin servicios de usuario personalizados.
* `~/.config/session/`: restauración de sesión de Dolphin (dinámico, no se
  reproduce).

## 9. Widgets / scripts de terceros (vendored)

`~/.local/share/plasma/plasmoids/`: AndromedaLauncher 0.6, KDE Control Station,
Kurve 3.5.1, Panel Colorizer 3.x, PlasMusic Toolbar 4.2.0 → `local/share/plasma/plasmoids/`.
