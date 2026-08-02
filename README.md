# Dotfiles — Fedora KDE Plasma

Repositorio de configuración **reproducible** del escritorio **Fedora 44 KDE
Plasma (Plasma 6, Wayland)**. No es un backup: es una infraestructura que
reconstruye el entorno visual y funcional del sistema original en una
instalación limpia con un solo comando.

> **Stack detectado y reproducido**
> Fedora 44 · KDE Plasma 6.7.3 (Wayland) · plasmalogin · bash · dnf + flatpak
> · Tema global **Nordic** · Esquema de color **Nordic** · Iconos
> **Tela-circle-black-dark** · Cursores **Wii Pointer** · Decoración
> **Nordic (Aurorae)** · Splash **spider-man_splash_animated** · Konsole +
> Fastfetch + Cava + Tmux · Panel flotante con **Kurve**, **Panel Colorizer** y
> **PlasMusic Toolbar**

---

## Requisitos

| Requisito | Valor |
|---|---|
| Distribución | **Fedora 44+ (KDE Plasma Edition)** — Fedora 43/45 también deberían funcionar |
| Sesión | Plasma (Wayland o X11) |
| Usuario | Normal (nunca root). La instalación pedirá sudo solo para paquetes |
| Red | Necesaria para `dnf`, `flatpak` y la descarga de iconos Tela |
| Disco | ~1 GB libre (temas e iconos) |
| Shell | `bash` (el prompt y fastfetch se configuran en `~/.bashrc`) |

---

## Instalación

### En una instalación limpia (bootstrap)

```bash
# 1) Descargar y ejecutar el bootstrap (instala git si falta, clona y ejecuta install.sh)
bash <(curl -fsSL https://raw.githubusercontent.com/LowNoise647/DotFilesFedora44KDE/main/bootstrap.sh)

# o bien, con tu propio repositorio
DOTFILES_REPO="git@github.com:LowNoise647/DotFilesFedora44KDE.git" \
  bash <(curl -fsSL https://raw.githubusercontent.com/LowNoise647/DotFilesFedora44KDE/main/bootstrap.sh)
```

### Desde el repositorio clonado

```bash
git clone https://github.com/LowNoise647/DotFilesFedora44KDE.git ~/dotfiles
cd ~/dotfiles
./install.sh
```

### Opciones del instalador

```text
-y, --yes                 No hacer preguntas (asume "si")
--with-packages           Instalar paquetes dnf y Flatpaks (requiere sudo)
--no-packages             No instalar paquetes
--with-themes             Descargar iconos Tela-circle desde GitHub (~150 MB)
--no-themes               No descargar temas de terceros
--with-display-config     Aplicar kwinoutputconfig.json (solo si los monitores coinciden)
--no-restart              No reiniciar plasmashell/KWin al final
-h, --help                Ayuda
```

El instalador es **idempotente** y hace una **instalación limpia**: puede
ejecutarse varias veces, y cada vez que se lanza **elimina (con backup previo
en `~/.dotfiles-backup-<fecha>`) todo lo que el repositorio gestiona** antes de
volver a desplegarlo. Así, si un intento anterior falló a medias, la reejecución
no arrastra restos ni conflictos de la instalación previa. El borrado solo toca
las rutas gestionadas por este repositorio (configuración de `~/.config`,
temas de `~/.local/share`, cursores, wallpapers), nunca datos de otras apps,
otros temas de iconos o plasmoids ajenos.

### Qué ocurre al finalizar

1. Se copian dotfiles, configuraciones de `~/.config` y temas de
   `~/.local/share` (con backup previo).
2. Se refrescan caches (iconos, fuentes, servicios de Plasma).
3. Se aplican esquema de color, tema de escritorio y wallpaper mediante las
   herramientas oficiales de Plasma (`plasma-apply-*`).
4. Se reinicia `plasmashell` y se reconfigura `kwin`.
5. **Cierra la sesión y vuelve a entrar** para ver splash, decoración de
   ventanas y configuración de pantallas al 100%.

---

## Estructura del repositorio

```text
dotfiles/
├── install.sh            # Instalador principal (idempotente)
├── bootstrap.sh          # Punto de entrada para instalaciones limpias
├── save.sh               # Re-exporta la config VIVA del sistema al repo
├── README.md             # Este archivo
├── LICENSE               # MIT + avisos de licencias de terceros
├── install/              # Modulos del instalador
│   ├── common.sh         # Helpers (logging, backups, copias, enlaces)
│   ├── detect.sh         # Deteccion de distro/escritorio/usuario
│   ├── packages.sh       # Paquetes dnf + Flatpaks
│   ├── themes.sh         # Descarga de temas de terceros (Tela)
│   ├── config.sh         # Despliegue de archivos y caches
│   └── kde.sh            # Aplicacion via herramientas oficiales de Plasma
├── packages/
│   ├── dnf.txt           # Paquetes dnf imprescindibles
│   └── flatpak.txt       # Aplicaciones Flatpak (Discord, Spotify)
├── config/               # -> ~/.config  (KDE, GTK, fastfetch, cava, ...)
├── local/share/          # -> ~/.local/share (temas, esquemas, widgets, kwin)
├── home/                 # -> $HOME (dotfiles estables, cursores)
├── assets/
│   ├── wallpapers/       # Fondos de pantalla del usuario
│   ├── icons/            # Iconos personalizados (IconoSpider.png, ...)
│   └── fastfetch/logos/  # Logos ASCII de fastfetch
├── scripts/              # Utilidades auxiliares
└── docs/
    ├── AUDIT.md          # Inventario completo del sistema auditado
    ├── MANUAL_STEPS.md   # Pasos manuales (no automatizables)
    ├── LIMITATIONS.md    # Limitaciones tecnicas conocidas
    └── FAQ.md            # Preguntas frecuentes
```

**Política de despliegue**

| Origen | Destino | Método | Motivo |
|---|---|---|---|
| `home/` (`.bashrc`, …) | `$HOME` | **Enlace simbólico** | Estables; no las reescribe ninguna app |
| `config/` | `~/.config` | **Copia** | Plasma reescribe muchos archivos con frecuencia |
| `local/share/` | `~/.local/share` | **Copia** | Temas/widgets; se regeneran en cada instalación |
| `assets/` | `~/.config`, `~/Imágenes/…` | **Copia** | Referenciados por rutas absolutas en la config |

> Los enlaces simbólicos de `$HOME` significan que el repositorio debe quedar
> en `~/dotfiles` (como hace `bootstrap.sh`). Si lo mueves, re-ejecuta
> `./install.sh`.

---

## Actualización

Cuando retoques el escritorio (paneles, widgets, colores, wallpapers, prompt…)
y quieras que el repositorio refleje el nuevo estado:

```bash
cd ~/dotfiles
./save.sh        # exporta la configuracion viva del sistema al repo
git add -A
git commit -m "update: sincronizar estado del escritorio"
git push
```

Y en la otra máquina:

```bash
cd ~/dotfiles
git pull
./install.sh
```

---

## Personalización

* **Wallpapers**: añádelos a `assets/wallpapers/` y ejecuta `./save.sh` (o
  edita `~/Imágenes/WallPapers/` y ejecuta `./save.sh`).
* **Esquema de color / tema de escritorio**: cámbialo en `Preferencias del
  sistema > Colores y Ventanas` y luego `./save.sh`.
* **Prompt de bash**: edita `home/.bashrc`.
* **Añadir/eliminar paquetes**: edita `packages/dnf.txt` y `packages/flatpak.txt`.
* **Iconos Tela**: la variante activa es `Tela-circle-black-dark`. Para usar
  otra, cámbiala en `Preferencias del sistema > Iconos` y ejecuta `./save.sh`.

---

## Solución de problemas

| Síntoma | Solución |
|---|---|
| Los paneles no aparecen tras instalar | Cierra sesión y vuelve a entrar; si sigues sin paneles, `rm -rf ~/.config/plasma-org.kde.plasma.desktop-appletsrc` y vuelve a ejecutar `./install.sh` |
| No se aplica la configuración de pantallas | Ejecuta con `--with-display-config` **solo** si tienes los mismos monitores (eDP-1 + HDMI-A-1) |
| Iconos genéricos tras instalar | Revisa que `~/.local/share/icons/Tela-circle-black-dark` existe; re-ejecuta `./install.sh --with-themes` |
| El prompt se ve distinto | Verifica que `~/.bashrc` es un enlace a `~/dotfiles/home/.bashrc` |
| Después de un cambio algo no funciona | `./install.sh` (idempotente, instalación limpia) — tu configuración previa quedó en `~/.dotfiles-backup-*` |

---

## Seguridad

**Nunca se copia** (por diseño): claves SSH/GPG, tokens, cookies, credenciales,
bases de datos de correo (Akonadi), historiales, wallets (KWallet), perfiles de
Firefox/Google, o datos de sincronización. Consulta `docs/MANUAL_STEPS.md` para
ver cómo restaurar estos elementos manualmente.

---

## Créditos

Este proyecto se apoya en el trabajo de terceros: [EliverLara/Nordic](https://github.com/EliverLara/Nordic)
(CC BY-SA 4.0), [vinceliuice/Tela-circle-icon-theme](https://github.com/vinceliuice/Tela-circle-icon-theme)
(GPLv3), [luisbocanegra/kurve](https://github.com/luisbocanegra/kurve),
[luisbocanegra/plasma-panel-colorizer](https://github.com/luisbocanegra/plasma-panel-colorizer),
[ccatterina/plasmusic-toolbar](https://github.com/ccatterina/plasmusic-toolbar),
[EliverLara/AndromedaLauncher](https://github.com/EliverLara/AndromedaLauncher)
y [Schneegans/Burn-My-Windows](https://github.com/Schneegans/Burn-My-Windows).
