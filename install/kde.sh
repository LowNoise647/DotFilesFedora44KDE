#!/usr/bin/env bash
# kde.sh - Aplicacion de la configuracion KDE usando las herramientas oficiales
# de Plasma (plasma-apply-*), seguido de recarga en vivo de plasmashell y KWin.
#
# Nota: no se usa `plasma-apply-lookandfeel` a proposito: ese comando restablece
# iconos, cursores y decoracion a los valores por defecto del tema global, y
# aqui esos valores son elecciones explicitas del usuario (kdeglobals,
# kcminputrc) que ya se han copiado como configuracion real.

set -euo pipefail

ACTIVE_WALLPAPER="miles-morales-spider-man_5120x2880_xtrafondos.com.jpg"
CURSOR_THEME="Wii Pointer"
ICON_THEME="Tela-circle-black-dark"
COLOR_SCHEME="Nordic"
DESKTOP_THEME="Nordic"
DECORATION_THEME="Nordic"

apply_kde_settings() {
    step "Aplicando ajustes KDE (herramientas oficiales de Plasma)"

    if is_installed plasma-apply-colorscheme; then
        plasma-apply-colorscheme "${COLOR_SCHEME}" || warn "No se pudo aplicar el esquema de color '${COLOR_SCHEME}'."
    fi

    if is_installed plasma-apply-desktoptheme; then
        plasma-apply-desktoptheme "${DESKTOP_THEME}" || warn "No se pudo aplicar el tema de escritorio '${DESKTOP_THEME}'."
    fi

    # Wallpaper activo del escritorio (se aplica a la actividad actual)
    local wall="${HOME}/Imágenes/WallPapers/${ACTIVE_WALLPAPER}"
    if is_installed plasma-apply-wallpaperimage && [[ -f "${wall}" ]]; then
        plasma-apply-wallpaperimage "${wall}" || warn "No se pudo aplicar el wallpaper del escritorio."
    fi

    if is_installed plasma-apply-cursortheme; then
        plasma-apply-cursortheme "${CURSOR_THEME}" || warn "No se pudo aplicar el cursor '${CURSOR_THEME}'."
    fi

    # Iconos, cursores y decoracion ya quedaron escritos por config.sh en
    # kdeglobals/kcminputrc/kwinrc; se refuerzan aqui por si acaso.
    if is_installed kwriteconfig6; then
        kwriteconfig6 --file kdeglobals --group Icons --key Theme "${ICON_THEME}"
        kwriteconfig6 --file kdeglobals --group General --key ColorScheme "${COLOR_SCHEME}"
        kwriteconfig6 --file kcminputrc --group Mouse --key cursorTheme "${CURSOR_THEME}"
        kwriteconfig6 --file kcminputrc --group Mouse --key cursorSize 32
    fi

    # Esquema de color de Konsole: ya fijado por konsolerc + perfil copiado.
}

apply_display_config() {
    # kwinoutputconfig.json es especifico del hardware (monitores EDID, escala,
    # resolucion). Solo se aplica si los conectores detectados coinciden.
    local want=("eDP-1" "HDMI-A-1")
    local found=0
    if command -v kscreen-doctor >/dev/null 2>&1; then
        for c in "${want[@]}"; do
            if kscreen-doctor -o 2>/dev/null | grep -q "${c}"; then
                found=1
                break
            fi
        done
    fi

    if [[ "${found}" -eq 0 ]]; then
        warn "No se detectaron los monitores de referencia (eDP-1/HDMI-A-1)."
        warn "No se aplica kwinoutputconfig.json (configuracion de pantallas especifica del hardware)."
        warn "Ajusta tus pantallas en: Preferencias del sistema > Pantallas."
        return 0
    fi

    deploy_copy "${REPO_DIR}/config/kwinoutputconfig.json" "${HOME}/.config/kwinoutputconfig.json"
    step "Aplicando configuracion de pantallas (kwinoutputconfig.json)"
    if command -v kscreen-doctor >/dev/null 2>&1; then
        kscreen-doctor --reload 2>/dev/null || true
    fi
    log "Reinicia la sesion para que KWin aplique la configuracion de pantallas."
}

restart_plasma() {
    local in_session=0
    [[ "${XDG_CURRENT_DESKTOP:-}" == *"KDE"* ]] && in_session=1

    if [[ "${in_session}" -eq 0 ]]; then
        log "No hay sesion Plasma activa; los cambios se aplicaran en el proximo inicio de sesion."
        return 0
    fi

    step "Recargando la sesion Plasma"

    if is_installed kwin_wayland && pgrep -x kwin_wayland >/dev/null 2>&1; then
        qdbus6 org.kde.KWin /KWin reconfigure >/dev/null 2>&1 || true
        ok "KWin reconfigurado"
    fi

    if pgrep -x plasmashell >/dev/null 2>&1; then
        kquitapp6 plasmashell >/dev/null 2>&1 || killall plasmashell 2>/dev/null || true
        sleep 2
        setsid plasmashell --replace >/dev/null 2>&1 &
        ok "plasmashell reiniciado"
    fi
}
