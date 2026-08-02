#!/usr/bin/env bash
# bootstrap.sh - Punto de entrada para una instalacion limpia.
#
# Hace todo lo que install.sh hace, pero previamente garantiza que existe
# `git` y que el repositorio esta clonado en ~/dotfiles.
#
# Uso:
#   # 1) Descargar y ejecutar sin clonar:
#   bash <(curl -fsSL https://raw.githubusercontent.com/<USER>/dotfiles/main/bootstrap.sh)
#
#   # 2) O bien, si ya clonaste el repo:
#   ./bootstrap.sh
#
#   # 3) Con un usuario/organizacion de GitHub distinto:
#   DOTFILES_REPO="git@github.com:<TU_USUARIO>/dotfiles.git" ./bootstrap.sh

set -euo pipefail

# Repositorio por defecto (cambiar <USER> al publicar). Se puede sobreescribir
# con la variable de entorno DOTFILES_REPO.
DOTFILES_REPO="${DOTFILES_REPO:-https://github.com/<USER>/dotfiles.git}"
DOTFILES_DIR="${DOTFILES_DIR:-${HOME}/dotfiles}"

log()  { printf '\033[36m[bootstrap]\033[0m %s\n' "$*"; }
err()  { printf '\033[31m[bootstrap] ERROR:\033[0m %s\n' "$*" >&2; }
die()  { err "$*"; exit 1; }

main() {
    if [[ "${EUID}" -eq 0 ]]; then
        die "No ejecutes bootstrap.sh como root."
    fi

    # --- git ---
    if ! command -v git >/dev/null 2>&1; then
        log "git no esta instalado. Intentando instalarlo..."
        if command -v dnf >/dev/null 2>&1; then
            if command -v sudo >/dev/null 2>&1; then
                sudo dnf install -y git
            elif [[ "${EUID}" -eq 0 ]]; then
                dnf install -y git
            else
                die "Instala git primero:  sudo dnf install -y git"
            fi
        else
            die "No se pudo instalar git automaticamente. Instalalo manualmente."
        fi
    fi

    # --- clonar o actualizar ---
    if [[ -d "${DOTFILES_DIR}" ]]; then
        log "El repositorio ya existe en ${DOTFILES_DIR}"
        if git -C "${DOTFILES_DIR}" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
            log "Actualizando..."
            git -C "${DOTFILES_DIR}" pull --ff-only || warn
        fi
    else
        log "Clonando ${DOTFILES_REPO} -> ${DOTFILES_DIR}"
        git clone "${DOTFILES_REPO}" "${DOTFILES_DIR}"
    fi

    # --- instalar ---
    cd "${DOTFILES_DIR}"
    log "Ejecutando install.sh"
    bash ./install.sh "$@"
}

main "$@"
