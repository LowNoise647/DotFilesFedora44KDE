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

# Directorio de imagenes del usuario segun ~/.config/user-dirs.dirs (con
# respaldo a "${HOME}/Imágenes"). Misma logica para la limpieza y el despliegue.
pictures_dir() {
    local dir="${HOME}/Imágenes"
    if [[ -f "${HOME}/.config/user-dirs.dirs" ]]; then
        local pics
        pics="$(grep '^XDG_PICTURES_DIR' "${HOME}/.config/user-dirs.dirs" | cut -d'"' -f2)"
        [[ -n "${pics}" ]] && dir="${pics/#\$\{HOME\}/$HOME}"
        dir="${dir//\$HOME/$HOME}"
    fi
    printf '%s' "${dir}"
}

# ---------------------------------------------------------------------------
# Instalacion limpia
# ---------------------------------------------------------------------------

# Purgar lo desplegado por una ejecucion anterior (con backup en BACKUP_ROOT),
# respetando la profundidad real de cada arbol para no borrar datos que este
# repositorio no gestiona (p. ej. otros temas de iconos o plasmoids ajenos).
clean_managed_paths() {
    step "Instalacion limpia (se elimina lo desplegado anteriormente, con backup)"

    local pics="$(pictures_dir)"

    # Dotfiles de $HOME (enlaces simbolicos) y cursores de home/.icons
    local dotfile d
    for dotfile in .bashrc .bash_profile .profile .bash_logout .gtkrc-2.0; do
        [[ -f "${REPO_DIR}/home/${dotfile}" ]] && purge_path "${HOME}/${dotfile}"
    done
    for d in "${REPO_DIR}"/home/.icons/*; do
        [[ -d "${d}" ]] && purge_path "${HOME}/.icons/$(basename "${d}")"
    done

    # ~/.config: misma lista que gestiona install_config (kwinoutputconfig.json
    # es condicional y se aplica aparte en install/kde.sh).
    local f
    for f in "${REPO_DIR}"/config/*; do
        [[ -e "${f}" ]] || continue
        [[ "$(basename "${f}")" == "kwinoutputconfig.json" ]] && continue
        purge_path "${HOME}/.config/$(basename "${f}")"
    done

    # ~/.local/share: solo las unidades que aporta el repositorio, nunca los
    # contenedores enteros (plasma/, icons/, color-schemes/, ...).
    local l base child sub area
    for l in "${REPO_DIR}"/local/share/*; do
        [[ -e "${l}" ]] || continue
        base="$(basename "${l}")"
        case "${base}" in
            plasma)
                for area in desktoptheme look-and-feel plasmoids; do
                    for sub in "${REPO_DIR}/local/share/plasma/${area}"/*; do
                        [[ -e "${sub}" ]] && purge_path "${HOME}/.local/share/plasma/${area}/$(basename "${sub}")"
                    done
                done
                ;;
            kwin)
                for sub in "${REPO_DIR}"/local/share/kwin/effects/*; do
                    [[ -e "${sub}" ]] && purge_path "${HOME}/.local/share/kwin/effects/$(basename "${sub}")"
                done
                ;;
            aurorae)
                for sub in "${REPO_DIR}"/local/share/aurorae/themes/*; do
                    [[ -e "${sub}" ]] && purge_path "${HOME}/.local/share/aurorae/themes/$(basename "${sub}")"
                done
                ;;
            *)
                for child in "${l}"/*; do
                    [[ -e "${child}" ]] && purge_path "${HOME}/.local/share/${base}/$(basename "${child}")"
                done
                ;;
        esac
    done

    # Wallpapers e iconos del usuario gestionados por assets/
    local p
    for p in "${REPO_DIR}"/assets/wallpapers/*; do
        [[ -f "${p}" ]] && purge_path "${pics}/WallPapers/$(basename "${p}")"
    done
    for p in "${REPO_DIR}"/assets/icons/*; do
        [[ -f "${p}" ]] && purge_path "${pics}/Icons/$(basename "${p}")"
    done
}

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

    local walls_dir="$(pictures_dir)/WallPapers"
    local icons_dir="$(pictures_dir)/Icons"
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
