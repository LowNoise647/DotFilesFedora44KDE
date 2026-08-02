#!/usr/bin/env bash
# save.sh - Re-exporta la configuracion VIVA del sistema hacia el repositorio.
#
# Uso normal (tras haber retocado el escritorio, los paneles o las applets):
#   ./save.sh
#
# Esto mantiene el repositorio sincronizado con lo que realmente usas, de modo
# que una instalacion futura reproduzca el estado actual. Hace lo contrario
# que install.sh.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="${SCRIPT_DIR}"
REF_HOME="/home/lownoise"

# shellcheck source=install/common.sh
source "${SCRIPT_DIR}/install/common.sh"

# Lista de archivos de ~/.config gestionados por el repo
CONFIG_FILES=(
    kdeglobals kwinrc kwinrulesrc kglobalshortcutsrc kwinoutputconfig.json
    kscreenlockerrc dolphinrc konsolerc ksplashrc plasmarc plasma-localerc
    kcminputrc kactivitymanagerdrc auroraerc plasmashellrc
    plasma-org.kde.plasma.desktop-appletsrc gtkrc gtkrc-2.0
    Trolltech.conf QtProject.conf ktimezonedrc powermanagementprofilesrc
    baloofilerc kded5rc discoverrc mimeapps.list user-dirs.dirs user-dirs.locale
    kdedefaults gtk-3.0 gtk-4.0 xsettingsd fastfetch cava fontconfig
)

CONFIG_DIRS=(
    color-schemes konsole wallpapers
    plasma/look-and-feel plasma/desktoptheme plasma/plasmoids
    aurorae/themes kwin/effects icons
)

normalize() { sed -i "s|${HOME}|${REF_HOME}|g" "$1" 2>/dev/null || true; }

save_home_dotfiles() {
    step "Guardando dotfiles de $HOME"
    for dotfile in .bashrc .bash_profile .profile .bash_logout .gtkrc-2.0; do
        local live="${HOME}/${dotfile}"
        local dst="${REPO_DIR}/home/${dotfile}"
        [[ -e "${live}" ]] || continue
        # Si el dotfile es un enlace al propio repositorio, ya esta sincronizado
        if [[ -L "${live}" ]]; then
            local target
            target="$(readlink -f "${live}")"
            if [[ "${target}" == "${dst}" ]]; then
                ok "home/${dotfile} (ya enlazado al repo)"
                continue
            fi
        fi
        cp -a "${live}" "${dst}"
        ok "home/${dotfile}"
    done
    for d in "${HOME}"/.icons/*; do
        [[ -d "${d}" ]] || continue
        rm -rf "${REPO_DIR}/home/.icons/$(basename "${d}")"
        cp -a "${d}" "${REPO_DIR}/home/.icons/$(basename "${d}")"
        ok "home/.icons/$(basename "${d}")"
    done
}

save_config() {
    step "Guardando ~/.config"
    for f in "${CONFIG_FILES[@]}"; do
        local src="${HOME}/.config/${f}"
        [[ -e "${src}" ]] || { warn "no existe: ${f}"; continue; }
        local dst="${REPO_DIR}/config/${f}"
        rm -rf "${dst}"
        cp -a "${src}" "${dst}"
        normalize "${dst}"
        ok "config/${f}"
    done
    # Normalizar la ruta de los logos de fastfetch en la config guardada
    if [[ -f "${REPO_DIR}/config/fastfetch/config.jsonc" ]]; then
        sed -i 's|"~/.config/fastfetch/logos/\*.txt"|"~/.config/fastfetch/logos/*.txt"|' \
            "${REPO_DIR}/config/fastfetch/config.jsonc"
    fi
}

save_local_share() {
    step "Guardando ~/.local/share"
    for f in "${CONFIG_DIRS[@]}"; do
        local src="${HOME}/.local/share/${f}"
        [[ -e "${src}" ]] || { warn "no existe: ${f}"; continue; }
        local dst="${REPO_DIR}/local/share/${f}"
        rm -rf "${dst}"
        cp -a "${src}" "${dst}"
        ok "local/share/${f}"
    done
    # Los iconos Tela (~150 MB, descargados en tiempo de instalacion) y el
    # hicolor de JetBrains (herramientas de desarrollo) NO se versionan.
    for excluded in hicolor Tela-circle-standard Tela-circle-black \
                    Tela-circle-black-dark Tela-circle-black-light; do
        rm -rf "${REPO_DIR}/local/share/icons/${excluded}"
    done
}

save_assets() {
    step "Guardando wallpapers, iconos y logos"

    local pics="${HOME}/Imágenes"
    if [[ -f "${HOME}/.config/user-dirs.dirs" ]]; then
        pics="$(grep '^XDG_PICTURES_DIR' "${HOME}/.config/user-dirs.dirs" | cut -d'"' -f2)"
        pics="${pics//\$\{HOME\}/$HOME}"; pics="${pics//\$HOME/$HOME}"
    fi

    if [[ -d "${pics}/WallPapers" ]]; then
        rm -rf "${REPO_DIR}"/assets/wallpapers/*
        cp -a "${pics}"/WallPapers/. "${REPO_DIR}/assets/wallpapers/"
        ok "assets/wallpapers"
    fi
    if [[ -d "${pics}/Icons" ]]; then
        rm -rf "${REPO_DIR}"/assets/icons/*
        cp -a "${pics}"/Icons/. "${REPO_DIR}/assets/icons/"
        ok "assets/icons"
    fi
    if [[ -d "${HOME}/.config/fastfetch/logos" ]]; then
        rm -rf "${REPO_DIR}"/assets/fastfetch/logos/*
        cp -a "${HOME}/.config/fastfetch/logos/." "${REPO_DIR}/assets/fastfetch/logos/"
        ok "assets/fastfetch/logos"
    fi
}

main() {
    step "Re-exportando configuracion viva -> repositorio"
    save_home_dotfiles
    save_config
    save_local_share
    save_assets
    echo
    ok "Repositorio sincronizado. Revisa los cambios con:  git status"
}

main "$@"
