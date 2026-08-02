#!/usr/bin/env bash
# packages.sh - Instalacion de paquetes (dnf) y aplicaciones Flatpak.
# Fuente de verdad: packages/dnf.txt y packages/flatpak.txt

set -euo pipefail

# Solo se instalan los paquetes de dnf imprescindibles para reproducir el
# entorno. Los paquetes que definen la "cara" del escritorio (plasma-desktop,
# kwin, konsole, dolphin, etc.) ya vienen con la edicion "KDE Plasma" de
# Fedora y NO se listan aqui para no duplicar ni romper dependencias.
DNF_FILE="${REPO_DIR}/packages/dnf.txt"
FLATPAK_FILE="${REPO_DIR}/packages/flatpak.txt"

install_dnf_packages() {
    [[ -f "${DNF_FILE}" ]] || { warn "No existe ${DNF_FILE}"; return 0; }

    local wanted=() missing=()
    mapfile -t wanted < <(grep -vE '^\s*(#|$)' "${DNF_FILE}")
    [[ ${#wanted[@]} -eq 0 ]] && return 0

    for pkg in "${wanted[@]}"; do
        if rpm -q "${pkg}" >/dev/null 2>&1; then
            ok "paquete ya instalado: ${pkg}"
        else
            missing+=("${pkg}")
        fi
    done

    if [[ ${#missing[@]} -eq 0 ]]; then
        log "Todos los paquetes dnf ya estan instalados."
        return 0
    fi

    if ! needs_root; then
        warn "No hay permisos sudo para instalar paquetes. Se omiten: ${missing[*]}"
        return 0
    fi

    step "Instalando paquetes dnf (${#missing[@]})"
    run_root dnf install -y --refresh "${missing[@]}"
}

install_flatpaks() {
    command -v flatpak >/dev/null 2>&1 || { warn "flatpak no esta instalado; se omiten aplicaciones Flatpak."; return 0; }
    [[ -f "${FLATPAK_FILE}" ]] || { warn "No existe ${FLATPAK_FILE}"; return 0; }

    local remote_ok=0
    if flatpak remotes 2>/dev/null | grep -q '^flathub'; then
        remote_ok=1
    fi

    if [[ "${remote_ok}" -eq 0 ]]; then
        if needs_root; then
            step "Anadiendo remoto Flathub"
            run_root flatpak remote-add --if-not-exists flathub \
                https://flathub.org/repo/flathub.flatpakrepo
        else
            warn "Sin sudo no se puede anadir Flathub; se omiten aplicaciones Flatpak."
            return 0
        fi
    fi

    local apps=()
    mapfile -t apps < <(grep -vE '^\s*(#|$)' "${FLATPAK_FILE}")

    local need=()
    for app in "${apps[@]}"; do
        if flatpak info "${app}" >/dev/null 2>&1; then
            ok "flatpak ya instalado: ${app}"
        else
            need+=("${app}")
        fi
    done

    [[ ${#need[@]} -eq 0 ]] && { log "Todas las aplicaciones Flatpak ya estan instaladas."; return 0; }

    step "Instalando aplicaciones Flatpak (${#need[@]})"
    flatpak install -y --noninteractive flathub "${need[@]}"
}
