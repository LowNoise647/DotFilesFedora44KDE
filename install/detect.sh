#!/usr/bin/env bash
# detect.sh - Deteccion automatica del sistema.
# Define las variables globales del entorno de destino.

set -euo pipefail

DISTRO_ID=""
DISTRO_NAME=""
DISTRO_VERSION_ID=""
DISTRO_VERSION=""
DISTRO_LIKE=""
IS_FEDORA=0
IS_RHEL_FAMILY=0
ARCH=""
DESKTOP=""
SESSION_TYPE=""
RUNNING_USER=""
RUNNING_UID=""
RUNNING_HOME=""
HAS_DNF=0
HAS_FLATPAK=0
HAS_PLASMA=0
PLASMA_VERSION=""

detect_system() {
    RUNNING_USER="$(id -un)"
    RUNNING_UID="$(id -u)"
    RUNNING_HOME="${HOME}"

    if [[ -r /etc/os-release ]]; then
        # shellcheck disable=SC1091
        . /etc/os-release
        DISTRO_ID="${ID:-unknown}"
        DISTRO_NAME="${NAME:-unknown}"
        DISTRO_VERSION_ID="${VERSION_ID:-unknown}"
        DISTRO_VERSION="${VERSION:-${VERSION_ID:-unknown}}"
        DISTRO_LIKE="${ID_LIKE:-}"
    fi

    [[ "${DISTRO_ID}" == "fedora" ]] && IS_FEDORA=1
    [[ "${DISTRO_LIKE}" == *"fedora"* ]] && IS_FEDORA=1
    if [[ "${IS_FEDORA}" -eq 1 ]] || [[ "${DISTRO_LIKE}" == *"rhel"* ]]; then
        IS_RHEL_FAMILY=1
    fi

    ARCH="$(uname -m)"
    DESKTOP="${XDG_CURRENT_DESKTOP:-unknown}"
    SESSION_TYPE="${XDG_SESSION_TYPE:-unknown}"

    command -v dnf >/dev/null 2>&1 && HAS_DNF=1
    command -v flatpak >/dev/null 2>&1 && HAS_FLATPAK=1

    if command -v plasmashell >/dev/null 2>&1; then
        HAS_PLASMA=1
        PLASMA_VERSION="$(plasmashell --version 2>/dev/null | sed 's/plasmashell //' || true)"
    fi
}

print_system_summary() {
    step "Sistema detectado"
    printf '  Distribucion : %s\n' "${DISTRO_NAME} (${DISTRO_ID} ${DISTRO_VERSION_ID})"
    printf '  Arquitectura : %s\n' "${ARCH}"
    printf '  Escritorio   : %s (%s)\n' "${DESKTOP}" "${SESSION_TYPE}"
    if [[ "${HAS_PLASMA}" -eq 1 ]]; then
        printf '  Plasma       : %s\n' "${PLASMA_VERSION}"
    fi
    printf '  Usuario      : %s (uid %s)\n' "${RUNNING_USER}" "${RUNNING_UID}"
}
