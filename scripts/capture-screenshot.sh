#!/usr/bin/env bash
# capture-screenshot.sh - Captura del escritorio completo (para documentar en
# el README). Requiere una sesion Plasma activa y Spectacle.
#
# Uso:
#   ./scripts/capture-screenshot.sh [archivo-salida.png]

set -euo pipefail

OUT="${1:-${HOME}/Imágenes/escritorio-$(date +%Y%m%d-%H%M%S).png}"

if ! command -v spectacle >/dev/null 2>&1; then
    echo "[capture] spectacle no está instalado." >&2
    exit 1
fi

# -f: pantalla completa, -b: sin sonido, -c: cerrar tras capturar
spectacle -f -b -n -o "${OUT}" >/dev/null 2>&1 || \
    spectacle -f -b -o "${OUT}"

echo "[capture] Guardada en: ${OUT}"
