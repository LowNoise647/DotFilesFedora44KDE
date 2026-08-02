#!/usr/bin/env bash
# config.sh - Despliegue de todos los archivos de configuracion, temas,
# wallpapers, cursores y logos del repositorio a las rutas reales de $HOME.
#
# Politica de despliegue:
#   * home/   -> dotfiles estables que las apps no reescriben  => ENLACE SIMBOLICO
#   * config/ -> ~/.config  (KDE/Plasma las reescriben a menudo) => COPIA
#   * local/  -> ~/.local/share (temas, widgets, esquemas)      => COPIA
#   * assets/ -> wallpapers, iconos, logos fastfetch            => COPIA

set -euo pipefail

install_home_dotfiles() {
    step "Dotfiles de $HOME"

    local dotfile
    for dotfile in .bashrc .bash_profile .profile .bash_logout .gtkrc-2.0; do
        local src="${REPO_DIR}/home/${dotfile}"
        [[ -f "${src}" ]] || continue
        deploy_link "${src}" "${HOME}/${dotfile}"
    done

    # Cursores personalizados (copia; son grandes y solo lectura)
    if [[ -d "${REPO_DIR}/home/.icons" ]]; then
        for d in "${REPO_DIR}"/home/.icons/*; do
            [[ -d "${d}" ]] || continue
            deploy_copy "${d}" "${HOME}/.icons/$(basename "${d}")"
        done
    fi
}

install_config() {
    step "Configuraciones (-> ~/.config)"
    local f
    for f in "${REPO_DIR}"/config/*; do
        [[ -e "${f}" ]] || continue
        # kwinoutputconfig.json es especifico del hardware (monitores) y se
        # aplica de forma condicional en install/kde.sh (--with-display-config).
        [[ "$(basename "${f}")" == "kwinoutputconfig.json" ]] && continue
        deploy_copy "${f}" "${HOME}/.config/$(basename "${f}")"
    done
}

install_local_share() {
    step "Temas y datos de aplicaciones (-> ~/.local/share)"
    local f
    for f in "${REPO_DIR}"/local/share/*; do
        [[ -e "${f}" ]] || continue
        deploy_copy "${f}" "${HOME}/.local/share/$(basename "${f}")"
    done
}

install_wallpapers() {
    step "Wallpapers y recursos graficos"

    local xdg_pictures="${HOME}/Imágenes"
    if [[ -f "${HOME}/.config/user-dirs.dirs" ]]; then
        # shellcheck disable=SC1091
        local pics
        pics="$(grep '^XDG_PICTURES_DIR' "${HOME}/.config/user-dirs.dirs" | cut -d'"' -f2)"
        [[ -n "${pics}" ]] && xdg_pictures="${pics/#\$\{HOME\}/$HOME}"
        xdg_pictures="${xdg_pictures//\$HOME/$HOME}"
    fi

    local walls_dir="${xdg_pictures}/WallPapers"
    local icons_dir="${xdg_pictures}/Icons"
    ensure_dir "${walls_dir}" "${icons_dir}"

    # Wallpapers propios del usuario
    local w
    for w in "${REPO_DIR}"/assets/wallpapers/*; do
        [[ -f "${w}" ]] || continue
        local name="$(basename "${w}")"
        if [[ ! -f "${walls_dir}/${name}" ]]; then
            deploy_copy "${w}" "${walls_dir}/${name}"
        else
            ok "wallpaper ya presente: ${name}"
        fi
    done

    # Iconos personalizados (p. ej. IconoSpider.png usado por el lanzador)
    local ic
    for ic in "${REPO_DIR}"/assets/icons/*; do
        [[ -f "${ic}" ]] || continue
        local iname="$(basename "${ic}")"
        if [[ ! -f "${icons_dir}/${iname}" ]]; then
            deploy_copy "${ic}" "${icons_dir}/${iname}"
        else
            ok "icono ya presente: ${iname}"
        fi
    done

    # Logos de fastfetch (se instalan junto a su config en ~/.config/fastfetch)
    ensure_dir "${HOME}/.config/fastfetch/logos"
    local l
    for l in "${REPO_DIR}"/assets/fastfetch/logos/*; do
        [[ -f "${l}" ]] || continue
        local lname="$(basename "${l}")"
        if [[ ! -f "${HOME}/.config/fastfetch/logos/${lname}" ]]; then
            deploy_copy "${l}" "${HOME}/.config/fastfetch/logos/${lname}"
        fi
    done
}

refresh_caches() {
    step "Actualizando caches"

    if is_installed gtk-update-icon-cache; then
        for d in "${HOME}"/.local/share/icons/* "${HOME}"/.icons/*; do
            [[ -d "${d}" ]] && gtk-update-icon-cache -f -q "${d}" 2>/dev/null || true
        done
    fi

    if is_installed fc-cache; then
        fc-cache -f -q >/dev/null 2>&1 || true
        ok "cache de fuentes actualizada (fc-cache)"
    fi

    if is_installed update-desktop-database; then
        update-desktop-database -q "${HOME}/.local/share/applications" 2>/dev/null || true
    fi

    if is_installed kbuildsycoca6; then
        kbuildsycoca6 --noincremental >/dev/null 2>&1 || true
        ok "base de datos de servicios de Plasma regenerada"
    fi
}
