#!/usr/bin/env bash
# themes.sh - Descarga e instalacion de temas de terceros desde sus fuentes
# oficiales. Mantiene el repositorio git ligero (no se "vendorean" ~180 MB de
# iconos) a cambio de requerir red durante la instalacion.
#
#  - Tela-circle-icon-theme (GPLv3): la variante activa es Tela-circle-black-dark
#    https://github.com/vinceliuice/Tela-circle-icon-theme

set -euo pipefail

TELA_REPO="https://github.com/vinceliuice/Tela-circle-icon-theme.git"
TELA_REF="master"

install_tela_icons() {
    local variant="${1:-black}"

    if [[ -d "${HOME}/.local/share/icons/Tela-circle-${variant}-dark" ]]; then
        ok "Tela-circle-${variant}* ya instalado en ~/.local/share/icons"
        return 0
    fi

    is_installed git || { warn "git no esta instalado; no se pueden descargar los iconos Tela."; return 0; }

    local tmp
    tmp="$(mktemp -d)"
    local cleanup=0
    trap '[[ "${cleanup}" -eq 1 ]] && rm -rf "${tmp}"' RETURN

    step "Descargando Tela-circle-icon-theme (${TELA_REF})"
    if ! git clone --quiet --depth 1 --branch "${TELA_REF}" "${TELA_REPO}" "${tmp}/tela" 2>/dev/null; then
        warn "No se pudo descargar ${TELA_REPO}; los iconos Tela se instalarán manualmente después."
        return 0
    fi

    log "Instalando variantes Tela-circle: ${variant} (light y dark)"
    (
        cd "${tmp}/tela"
        bash ./install.sh -n Tela-circle "${variant}" light dark
    )

    # Actualizar cache de iconos de las variantes instaladas
    for d in "${HOME}"/.local/share/icons/Tela-circle-${variant}-*; do
        [[ -d "${d}" ]] || continue
        gtk-update-icon-cache -f -q "${d}" 2>/dev/null || true
    done

    cleanup=1
    ok "Iconos Tela-circle instalados."
}
