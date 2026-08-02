#!/usr/bin/env bash
# install.sh - Instalador automatico de dotfiles (Fedora KDE Plasma).
#
# Idempotente: puede ejecutarse multiples veces sin romper la instalacion.
# Cada ejecucion hace un backup previo de lo que vaya a sobrescribir.
#
# Uso:
#   ./install.sh [opciones]
#
# Opciones:
#   -y, --yes               No hacer preguntas (asume "si").
#   --with-packages         Instalar paquetes dnf y Flatpaks.
#   --no-packages           No instalar paquetes.
#   --with-themes           Descargar temas de terceros (iconos Tela).
#   --no-themes             No descargar temas de terceros.
#   --with-display-config   Aplicar kwinoutputconfig.json (hardware especifico).
#   --no-restart            No reiniciar plasmashell/KWin al final.
#   -h, --help              Mostrar ayuda.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="${SCRIPT_DIR}"

# shellcheck source=install/common.sh
source "${SCRIPT_DIR}/install/common.sh"
# shellcheck source=install/detect.sh
source "${SCRIPT_DIR}/install/detect.sh"
# shellcheck source=install/packages.sh
source "${SCRIPT_DIR}/install/packages.sh"
# shellcheck source=install/themes.sh
source "${SCRIPT_DIR}/install/themes.sh"
# shellcheck source=install/config.sh
source "${SCRIPT_DIR}/install/config.sh"
# shellcheck source=install/kde.sh
source "${SCRIPT_DIR}/install/kde.sh"

# ---------------------------------------------------------------------------
# Argumentos
# ---------------------------------------------------------------------------
ASSUME_YES=0
DO_PACKAGES="ask"
DO_THEMES="ask"
DO_DISPLAY=0
DO_RESTART=1

usage() {
    sed -n '2,20p' "${SCRIPT_DIR}/install.sh"
    exit 0
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        -y|--yes)            ASSUME_YES=1;;
        --with-packages)     DO_PACKAGES=1;;
        --no-packages)       DO_PACKAGES=0;;
        --with-themes)       DO_THEMES=1;;
        --no-themes)         DO_THEMES=0;;
        --with-display-config) DO_DISPLAY=1;;
        --no-restart)        DO_RESTART=0;;
        -h|--help)           usage;;
        *)                   die "Opcion desconocida: $1 (usa --help)";;
    esac
    shift
done

# ---------------------------------------------------------------------------
# Preflight
# ---------------------------------------------------------------------------
main() {
    detect_system
    print_system_summary

    if [[ "${EUID}" -eq 0 ]]; then
        die "No ejecutes este instalador como root. Ejecútalo como tu usuario normal."
    fi

    if [[ "${IS_FEDORA}" -eq 0 ]] && [[ "${IS_RHEL_FAMILY}" -eq 0 ]]; then
        warn "Este repositorio esta pensado para Fedora KDE. Se continua en modo compatible."
        warn "  (se omitiran los pasos de paquetes dnf y Flatpak de Flathub)"
    fi

    if [[ "${ASSUME_YES}" -eq 0 ]]; then
        echo
        if ! ask_yes_no "Este script copiara configuraciones en tu $HOME (con backup previo). Continuar?" "y"; then
            log "Cancelado."
            exit 0
        fi
    fi

    # Paquetes
    if [[ "${DO_PACKAGES}" == "ask" ]]; then
        if [[ "${ASSUME_YES}" -eq 1 ]]; then
            DO_PACKAGES=1
        else
            DO_PACKAGES=0
            if ask_yes_no "Instalar paquetes dnf y Flatpaks (requiere sudo)?" "n"; then
                DO_PACKAGES=1
            fi
        fi
    fi
    if [[ "${DO_PACKAGES}" -eq 1 ]]; then
        [[ "${IS_FEDORA}" -eq 1 ]] && install_dnf_packages
        install_flatpaks
    fi

    # Temas de terceros (descarga)
    if [[ "${DO_THEMES}" == "ask" ]]; then
        if [[ "${ASSUME_YES}" -eq 1 ]]; then
            DO_THEMES=1
        else
            DO_THEMES=0
            if ask_yes_no "Descargar temas de iconos Tela-circle desde GitHub (~150 MB)?" "y"; then
                DO_THEMES=1
            fi
        fi
    fi
    if [[ "${DO_THEMES}" -eq 1 ]]; then
        install_tela_icons black
    fi

    # Backups + despliegue
    init_backup
    install_home_dotfiles
    install_config
    install_local_share
    install_wallpapers
    refresh_caches

    # Ajustes KDE en vivo
    apply_kde_settings
    [[ "${DO_DISPLAY}" -eq 1 ]] && apply_display_config
    [[ "${DO_RESTART}" -eq 1 ]] && restart_plasma

    # -----------------------------------------------------------------------
    echo
    ok "Instalacion completada."
    echo
    log "Resumen de lo que se copio:"
    printf '  %sBackup%s        %s\n' "$C_BOLD" "$C_RESET" "${BACKUP_ROOT}"
    printf '  %sSiguientes pasos:%s\n' "$C_BOLD" "$C_RESET"
    printf '    1. Cierra la sesion y vuelve a entrar para ver el tema completo\n'
    printf '       (splash, decoraciones y configuracion de pantallas).\n'
    printf '    2. Aplicaciones manuales: JetBrains Toolbox, IDE, claves (ver docs/MANUAL_STEPS.md).\n'
    printf '    3. Para actualizar desde el sistema:  ./save.sh\n'
}

main "$@"
