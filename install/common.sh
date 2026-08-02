#!/usr/bin/env bash
# common.sh - Funciones compartidas por el instalador.
# Este archivo NO debe ejecutarse directamente.

set -euo pipefail

# ---------------------------------------------------------------------------
# Colores (solo si la salida es un terminal y NO_COLOR no esta definido)
# ---------------------------------------------------------------------------
if [[ -t 1 ]] && [[ "${NO_COLOR:-}" != "1" ]]; then
    C_RESET=$'\e[0m'
    C_RED=$'\e[31m'
    C_GREEN=$'\e[32m'
    C_YELLOW=$'\e[33m'
    C_BLUE=$'\e[34m'
    C_CYAN=$'\e[36m'
    C_BOLD=$'\e[1m'
    C_DIM=$'\e[2m'
else
    C_RESET= C_RED= C_GREEN= C_YELLOW= C_BLUE= C_CYAN= C_BOLD= C_DIM=
fi

# ---------------------------------------------------------------------------
# Rutas
# ---------------------------------------------------------------------------
COMMON_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "${COMMON_DIR}/.." && pwd)"

# HOME de referencia con el que se genero este repositorio. El instalador
# reemplaza esta ruta por el $HOME real del sistema de destino en cada copia.
REF_HOME="/home/lownoise"

# ---------------------------------------------------------------------------
# Logging
# ---------------------------------------------------------------------------
log()  { printf '%s[dotfiles]%s %s%s%s\n' "$C_CYAN" "$C_RESET" "$C_BOLD" "$*" "$C_RESET"; }
ok()   { printf '%s[ ok ]%s   %s\n' "$C_GREEN" "$C_RESET" "$*"; }
warn() { printf '%s[warn]%s   %s\n' "$C_YELLOW" "$C_RESET" "$*"; }
err()  { printf '%s[err ]%s   %s\n' "$C_RED" "$C_RESET" "$*" >&2; }
die()  { err "$*"; exit 1; }
step() { printf '\n%s==>%s %s\n' "$C_BLUE" "$C_RESET" "$*"; }

# ---------------------------------------------------------------------------
# Utilidades
# ---------------------------------------------------------------------------
ensure_dir() { mkdir -p "$1"; }

is_installed() { command -v "$1" >/dev/null 2>&1; }

# Pregunta si/no. Devuelve 0 para "si", 1 para "no".
ask_yes_no() {
    local prompt="$1" default="${2:-}"
    local hint=""
    case "$default" in
        y) hint="[S/n]";;
        n) hint="[s/N]";;
        *) hint="[s/n]";;
    esac
    local ans
    while true; do
        read -r -p "$(printf '%s%s%s %s ' "$C_BOLD" "$prompt" "$C_RESET" "$hint")" ans || return 1
        case "${ans,,}" in
            "" ) [[ "$default" == "y" ]] && return 0 || { [[ "$default" == "n" ]] && return 1 || continue; };;
            s|y|si|yes|1) return 0;;
            n|no|0) return 1;;
            *) continue;;
        esac
    done
}

# ---------------------------------------------------------------------------
# Backups
# ---------------------------------------------------------------------------
BACKUP_ROOT=""

init_backup() {
    BACKUP_ROOT="${HOME}/.dotfiles-backup-$(date +%Y%m%d-%H%M%S)"
    ensure_dir "${BACKUP_ROOT}"
    log "Backup de la configuracion previa en: ${C_BOLD}${BACKUP_ROOT}${C_RESET}"
    log "Si todo funciona, puedes borrarlo con: rm -rf ${BACKUP_ROOT}"
}

# Copia a un backup lo que vaya a ser sobrescrito (antes de tocarlo).
backup_path() {
    local src="$1"
    [[ -e "${src}" ]] || return 0
    [[ -n "${BACKUP_ROOT}" ]] || return 0
    local rel="${src#${HOME}/}"
    local dst="${BACKUP_ROOT}/${rel}"
    ensure_dir "$(dirname "${dst}")"
    # cp -al usa hardlinks: instantaneo y sin duplicar espacio. Como deploy_copy
    # sobrescribe archivos (nuevos inodes), el backup conserva el contenido
    # original intacto.
    if [[ -d "${src}" ]]; then
        cp -al "${src}" "${dst}"
    else
        cp -al "${src}" "${dst}"
    fi
}

# Elimina con backup previo una ruta gestionada por el repositorio. Lo usa la
# "instalacion limpia": antes de desplegar se purga lo que una ejecucion
# anterior hubiera dejado, para no arrastrar restos de un intento fallido.
purge_path() {
    local dst="$1"
    [[ -e "${dst}" ]] || return 0
    backup_path "${dst}"
    rm -rf "${dst}"
}

# ---------------------------------------------------------------------------
# Despliegue de archivos
# ---------------------------------------------------------------------------

# Copia recursiva con backup previo y sustitucion de la ruta HOME de referencia.
#   deploy_copy <origen-en-repo> <destino-absoluto> [permisos-octal]
deploy_copy() {
    local src="$1" dst="$2" mode="${3:-}"
    [[ -e "${src}" ]] || die "Origen inexistente: ${src}"
    ensure_dir "$(dirname "${dst}")"
    backup_path "${dst}"

    if [[ -d "${src}" ]]; then
        # Fusion: sobrescribe el contenido del repo sin borrar lo que el usuario
        # ya tenga en el destino (p. ej. iconos Tela instalados previamente).
        mkdir -p "${dst}"
        cp -a "${src}/." "${dst}/"
        # Sustituir rutas HOME en archivos de texto dentro del arbol copiado
        find "${dst}" -type f \( \
            -name '*.rc' -o -name '*.conf' -o -name '*.json' -o -name '*.jsonc' \
            -o -name '*.ini' -o -name '*.list' -o -name '*.desktop' -o -name '*.css' \
            -o -name '*.colors' -o -name '*.colorscheme' -o -name '*.profile' \
            -o -name '*.txt' -o -name '*.qml' \) \
            -exec sed -i "s|${REF_HOME}|${HOME}|g" {} + 2>/dev/null || true
    else
        cp -a "${src}" "${dst}"
        sed -i "s|${REF_HOME}|${HOME}|g" "${dst}" 2>/dev/null || true
        if [[ -n "${mode}" ]]; then
            chmod "${mode}" "${dst}"
        fi
    fi
    ok "copiado: ${dst}"
}

# Enlace simbolico con backup previo. Usado SOLO para dotfiles estables de $HOME
# (p. ej. .bashrc) que las aplicaciones no reescriben.
deploy_link() {
    local src="$1" dst="$2"
    [[ -e "${src}" ]] || die "Origen inexistente: ${src}"
    ensure_dir "$(dirname "${dst}")"
    backup_path "${dst}"
    rm -rf "${dst}"
    ln -s "${src}" "${dst}"
    ok "enlazado: ${dst} -> ${src}"
}

# ---------------------------------------------------------------------------
# Utilidades de sistema
# ---------------------------------------------------------------------------
needs_root() {
    if [[ "${EUID}" -eq 0 ]]; then
        return 0
    fi
    if command -v sudo >/dev/null 2>&1; then
        return 0
    fi
    return 1
}

run_root() {
    if [[ "${EUID}" -eq 0 ]]; then
        "$@"
    else
        sudo "$@"
    fi
}
