# Preguntas frecuentes (FAQ)

### ¿Por qué no hay `~/.gitconfig` en el repositorio?

No existía en el sistema de origen (git 2.55.0 recién instalado). Cuando lo
configures, añádelo a `config/` (se copiará a `~/.config/git/config`).

### ¿Por qué los iconos Tela no están en el repositorio?

Son ~150 MB para las tres variantes. El instalador los descarga desde su
repositorio oficial (GPLv3) durante la instalación. Ver `docs/LIMITATIONS.md`.

### He movido el repositorio de `~/dotfiles` a otra ruta. ¿Qué pasa?

Los dotfiles de `$HOME` (`.bashrc`, etc.) son **enlaces simbólicos** al
repositorio. Si lo mueves, re-ejecuta `./install.sh` para recrearlos. El resto
(configuraciones, temas) se copian, así que siguen funcionando.

### El instalador me pide sudo. ¿Es normal?

Solo para instalar paquetes (`--with-packages`) y añadir Flathub. El resto de
la instalación funciona sin privilegios.

### ¿Cómo desinstalo/reviero?

`install.sh` crea un backup en `~/.dotfiles-backup-<fecha>` antes de tocar
cualquier archivo. Para revertir:

```bash
rm -rf ~/.config/plasma-org.kde.plasma.desktop-appletsrc   # paneles (opcional)
# restaura archivos concretos desde el backup:
cp -a ~/.dotfiles-backup-<fecha>/config/kdeglobals ~/.config/
```

### ¿Puedo usar esto en otra distro?

Los temas, scripts de bash, fastfetch y cava sí. Los paquetes `dnf` y las
herramientas `plasma-apply-*` son de Fedora/Plasma. En otras distros ejecuta
`./install.sh --no-packages` (avisa y continúa).

### ¿Por qué cambia el `prompt` respecto a antes de instalar?

El prompt original usaba rutas y usuarios hardcodeados (`/home/lownoise`). Se
mantiene el diseño idéntico (`LowNoise │ BashHunter`) pero la ruta de opencode
y el PATH de JetBrains ahora se añaden de forma condicional, por lo que si
esos programas no existen en la máquina nueva no aparecen errores.

### ¿Dónde está la configuración de la pantalla de login?

El sistema usa **plasmalogin** (Plasma Login Manager) con el tema por defecto
de Fedora; no se ha personalizado en el sistema de origen, por lo que no se
incluye ninguna configuración de login.
