//! Renderizado de las pantallas de la TUI.

use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};

use crate::animations;
use crate::runner::{LogKind, PHASES};
use crate::theme::*;
use crate::{App, InstallStatus, Screen};

const BORDERS: Borders = Borders::ALL;

fn border_style(c: Color) -> Style {
    Style::default().fg(c)
}

fn bold(c: Color) -> Style {
    Style::default().fg(c).add_modifier(Modifier::BOLD)
}

pub fn draw(f: &mut Frame, app: &App) {
    let area = f.area();
    match app.screen {
        Screen::Welcome => welcome(f, area, app),
        Screen::Setup => setup(f, area, app),
        Screen::Auth => auth(f, area, app),
        Screen::Install => install(f, area, app),
        Screen::Finished => finished(f, area, app),
    }
}

// ---------------------------------------------------------------------------
// Barra superior
// ---------------------------------------------------------------------------
fn title_bar(f: &mut Frame, area: Rect, title: &str, right: &str, accent: Color, t: f64) {
    let mut buf = f.buffer_mut();
    fill(&mut buf, area, ' ', accent, BG);
    let left = format!(" {} {} ", spinner(t), title);
    let avail = area.width.saturating_sub(2) as usize;
    let right_cap = (avail / 2).min(right.chars().count());
    let right_vis: String = right.chars().take(right_cap).collect();
    let left_cap = avail.saturating_sub(right_cap + 1);
    let left_vis: String = left.chars().take(left_cap).collect();
    set_string(&mut buf, area, 1, 0, &left_vis, TEXT, BG);
    let rx = area.width as i32 - right_vis.chars().count() as i32 - 1;
    set_string(&mut buf, area, rx, 0, &right_vis, TEXT, BG);
    for x in 0..area.width {
        let cell = &mut buf[(area.x + x, area.y)];
        cell.set_bg(BG);
    }
}

// ---------------------------------------------------------------------------
// Pantalla de bienvenida
// ---------------------------------------------------------------------------
fn welcome(f: &mut Frame, area: Rect, app: &App) {
    // Fondo: animación de Spider-Man a pantalla completa.
    animations::render(f.buffer_mut(), area, 0, app.now());

    let mw: u16 = 28;
    let mh: u16 = 10;
    let mx = area.width.saturating_sub(mw) / 2;
    let my = area.height.saturating_sub(mh).saturating_sub(1);
    let mrect = Rect::new(mx, my, mw, mh);
    f.render_widget(Clear, mrect);

    let block = Block::default()
        .borders(BORDERS)
        .border_type(BorderType::Rounded)
        .border_style(border_style(BORDER_BRIGHT))
        .title(Span::styled(" MENÚ ", bold(RED)))
        .title_alignment(Alignment::Center);
    let inner = block.inner(mrect);
    f.render_widget(block, mrect);

    let mut buf = f.buffer_mut();
    for (i, label) in ["INSTALAR", "SALIR"].iter().enumerate() {
        let y = inner.y as i32 + 1 + i as i32;
        let selected = app.menu_idx == i;
        let text = format!("{} {}", if selected { "▶" } else { " " }, label);
        if selected {
            set_string(&mut buf, inner, 1, y - inner.y as i32, &text, BLACK, RED);
        } else {
            set_string(&mut buf, inner, 1, y - inner.y as i32, &text, TEXT, BG);
        }
    }
    set_string(
        &mut buf,
        mrect,
        0,
        mh as i32 - 1,
        "  ↑↓ mover · Enter elegir · q salir  ",
        TEXT_DIM,
        BG,
    );
}

// ---------------------------------------------------------------------------
// Pantalla de configuración (animación + opciones)
// ---------------------------------------------------------------------------
fn setup(f: &mut Frame, area: Rect, app: &App) {
    let t = app.now();
    let meta = &animations::metas()[app.anim_idx];

    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Percentage(50),
        Constraint::Length(2),
        Constraint::Min(8),
        Constraint::Length(1),
    ])
    .split(area);

    title_bar(f, chunks[0], "CONFIGURACIÓN", "· elige tu animación ·", RED, t);

    // Panel de animación.
    let block = Block::default()
        .borders(BORDERS)
        .border_type(BorderType::Rounded)
        .border_style(border_style(meta.accent))
        .title(Span::styled(format!(" {} ", meta.name), bold(meta.fg)))
        .title_alignment(Alignment::Center);
    let inner = block.inner(chunks[1]);
    f.render_widget(block, chunks[1]);
    animations::render(f.buffer_mut(), inner, app.anim_idx, t);

    // Carrusel de animaciones.
    let mut buf = f.buffer_mut();
    let carousel = format!("◀   {}   ·   {}   ▶", meta.name, meta.tagline);
    let clen = carousel.chars().count() as i32;
    let cx = (area.width as i32 - clen) / 2;
    set_string(&mut buf, area, cx.max(0), chunks[2].y as i32 - area.y as i32, &carousel, meta.fg, BG);
    set_string(
        &mut buf,
        area,
        (cx - 1).max(0),
        chunks[2].y as i32 - area.y as i32,
        "←",
        BORDER_BRIGHT,
        BG,
    );
    set_string(
        &mut buf,
        area,
        cx + clen + 1,
        chunks[2].y as i32 - area.y as i32,
        "→",
        BORDER_BRIGHT,
        BG,
    );

    // Opciones.
    let oblock = Block::default()
        .borders(BORDERS)
        .border_type(BorderType::Rounded)
        .border_style(border_style(BORDER))
        .title(Span::styled(" OPCIONES ", bold(TEXT_DIM)));
    let oinner = oblock.inner(chunks[3]);
    f.render_widget(oblock, chunks[3]);

    let items = [
        ("Paquetes dnf + Flatpak (requiere sudo)", app.options.packages),
        ("Iconos Tela (descarga ~150 MB)", app.options.themes),
        ("Configuración de pantallas (hardware)", app.options.display),
        ("Reiniciar Plasma al terminar", app.options.restart),
    ];

    let mut lines: Vec<Line> = Vec::new();
    for (i, (label, on)) in items.iter().enumerate() {
        let focused = app.option_focus == i;
        let mark = if *on { "✔" } else { " " };
        let fg = if *on { GREEN } else { TEXT_DIM };
        lines.push(Line::from(vec![
            Span::styled(
                if focused { "▶ " } else { "  " },
                if focused { bold(YELLOW) } else { Style::default().fg(TEXT_DIM) },
            ),
            Span::styled("[", Style::default().fg(BORDER)),
            Span::styled(mark, if *on { bold(GREEN) } else { Style::default().fg(TEXT_DIM) }),
            Span::styled("]", Style::default().fg(BORDER)),
            Span::styled(" ", Style::default()),
            Span::styled(*label, if focused { bold(TEXT) } else { Style::default().fg(fg) }),
            Span::styled(
                format!(
                    "  {}",
                    if *on { "activo" } else { "desactivado" }
                ),
                Style::default().fg(if *on { GREEN } else { RED }).add_modifier(Modifier::DIM),
            ),
        ]));
    }

    let start_focused = app.option_focus == 4;
    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled(
        format!("   {}  INICIAR INSTALACIÓN", if start_focused { "▶" } else { " " }),
        if start_focused {
            Style::default().fg(BLACK).bg(RED).add_modifier(Modifier::BOLD)
        } else {
            bold(GREEN)
        },
    )]));

    let p = Paragraph::new(lines).wrap(ratatui::widgets::Wrap { trim: false });
    f.render_widget(p, oinner);

    // Pista inferior.
    let hint = "[Enter] iniciar · [← →] animación · [↑ ↓] opciones · [Espacio] alternar · [q] salir";
    let mut buf = f.buffer_mut();
    let hlen = hint.chars().count() as i32;
    set_string(
        &mut buf,
        area,
        (area.width as i32 - hlen) / 2,
        chunks[4].y as i32 - area.y as i32,
        hint,
        TEXT_DIM,
        BG,
    );
}

// ---------------------------------------------------------------------------
// Pantalla de autenticación sudo
// ---------------------------------------------------------------------------
fn auth(f: &mut Frame, area: Rect, app: &App) {
    let t = app.now();
    animations::render(f.buffer_mut(), area, app.anim_idx, t);

    let mw: u16 = 52;
    let mh: u16 = 9;
    let mx = area.width.saturating_sub(mw) / 2;
    let my = area.height.saturating_sub(mh) / 2;
    let mrect = Rect::new(mx, my, mw, mh);
    f.render_widget(Clear, mrect);

    let block = Block::default()
        .borders(BORDERS)
        .border_type(BorderType::Rounded)
        .border_style(border_style(YELLOW))
        .title(Span::styled(" AUTENTICACIÓN SUDO ", bold(YELLOW)))
        .title_alignment(Alignment::Center);
    let inner = block.inner(mrect);
    f.render_widget(block, mrect);

    let mut buf = f.buffer_mut();
    set_string(&mut buf, inner, 1, 0, "Se necesita sudo para instalar paquetes.", TEXT, BG);
    set_string(
        &mut buf,
        inner,
        1,
        1,
        "Introduce la contraseña de tu usuario:",
        TEXT_DIM,
        BG,
    );
    let masked: String = "•".repeat(app.sudo.password.len());
    let pass_line = format!("  Contraseña: {} {}", masked, if (t * 2.0).fract() < 0.5 { "_" } else { " " });
    set_string(&mut buf, inner, 1, 3, &pass_line, CYAN, BG);
    if !app.sudo.error.is_empty() {
        let e = format!("  {}", app.sudo.error);
        set_string(&mut buf, inner, 1, 5, &e, RED, BG);
        set_string(&mut buf, inner, 1, 6, "  [Enter] reintentar · [Esc] volver a la configuración", TEXT_DIM, BG);
    } else {
        set_string(&mut buf, inner, 1, 6, "  [Enter] confirmar · [Esc] volver a la configuración", TEXT_DIM, BG);
    }
}

// ---------------------------------------------------------------------------
// Pantalla de instalación
// ---------------------------------------------------------------------------
fn install(f: &mut Frame, area: Rect, app: &App) {
    let t = app.now();
    let meta = &animations::metas()[app.anim_idx];
    let pct = (app.progress() * 100.0) as u16;

    let right = format!("{} · [←→] animación · {}% · {:.0}s", meta.name, pct, t);
    title_bar(f, area, "INSTALADOR · SPIDER", &right, meta.fg, t);

    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Percentage(50),
        Constraint::Length(4),
        Constraint::Min(6),
    ])
    .split(area);

    // Animación.
    let block = Block::default()
        .borders(BORDERS)
        .border_type(BorderType::Rounded)
        .border_style(border_style(meta.accent))
        .title(Span::styled(format!(" {} ", meta.name), bold(meta.fg)))
        .title_alignment(Alignment::Center);
    let inner = block.inner(chunks[1]);
    f.render_widget(block, chunks[1]);
    animations::render(f.buffer_mut(), inner, app.anim_idx, t);

    // Barra de progreso + paso actual.
    let step_line = match app.status {
        InstallStatus::Running => format!("  {}  {}", spinner(t), app.current_step),
        InstallStatus::Success => format!("  ✔  {}", app.current_step),
        InstallStatus::Failed => {
            if app.detail_line.is_empty() {
                format!("  ✘  {}", app.current_step)
            } else {
                format!("  ✘  {}", app.detail_line)
            }
        }
        InstallStatus::Ready => format!("  {}  {}", spinner(t), app.current_step),
    };
    let step_fg = match app.status {
        InstallStatus::Success => GREEN,
        InstallStatus::Failed => RED,
        _ => YELLOW,
    };
    let pct = (app.progress() * 100.0) as u16;

    let pblock = Block::default()
        .borders(BORDERS)
        .border_type(BorderType::Rounded)
        .border_style(border_style(BORDER))
        .title(Span::styled(" PROGRESO ", bold(BLUE)));
    let pinner = pblock.inner(chunks[2]);
    f.render_widget(pblock, chunks[2]);
    let mut buf = f.buffer_mut();
    draw_bar(&mut buf, pinner, app.progress().clamp(0.0, 1.0));
    set_string(&mut buf, pinner, 0, 1, &step_line, step_fg, BG);
    let pct_txt = format!("  {} %  ", pct);
    set_string(&mut buf, pinner, pinner.width as i32 - pct_txt.chars().count() as i32, 1, &pct_txt, TEXT, BG);

    // Columnas: fases realizadas + log.
    let cols = Layout::horizontal([Constraint::Percentage(44), Constraint::Percentage(56)]).split(chunks[3]);

    let pblock = Block::default()
        .borders(BORDERS)
        .border_type(BorderType::Rounded)
        .border_style(border_style(BORDER))
        .title(Span::styled(" PASOS ", bold(TEXT_DIM)));
    let pinner = pblock.inner(cols[0]);
    f.render_widget(pblock, cols[0]);
    draw_phases(f, pinner, app);

    let lblock = Block::default()
        .borders(BORDERS)
        .border_type(BorderType::Rounded)
        .border_style(border_style(BORDER))
        .title(Span::styled(" SALIDA ", bold(TEXT_DIM)));
    let linner = lblock.inner(cols[1]);
    f.render_widget(lblock, cols[1]);
    draw_log(f, linner, app);

    if app.cancel_prompt {
        draw_cancel_prompt(f, area);
    }
}

fn draw_bar(buf: &mut Buffer, inner: Rect, frac: f64) {
    const BLOCKS: [char; 8] = ['▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];
    let w = inner.width;
    if w == 0 {
        return;
    }
    let frac = frac.clamp(0.0, 1.0);
    let total = (w as f64 * frac * 8.0).round() as u16;
    let full = total / 8;
    let rem = total % 8;
    let bar_bg = Color::Rgb(60, 18, 26);
    for x in 0..full.min(w) {
        put(buf, inner, x as i32, 0, '█', RED, bar_bg);
    }
    if full < w && rem > 0 {
        put(buf, inner, full as i32, 0, BLOCKS[(rem as usize).saturating_sub(1)], RED, bar_bg);
    }
    for x in (full + u16::from(rem > 0)).min(w)..w {
        put(buf, inner, x as i32, 0, '░', Color::Rgb(140, 60, 75), Color::Rgb(28, 10, 16));
    }
}

fn draw_phases(f: &mut Frame, inner: Rect, app: &App) {
    let rows = inner.height as usize;
    let total = PHASES.len();
    let current = app.phase_idx.min(total - 1);
    let half = rows / 2;
    let start = current.saturating_sub(half);
    let end = (start + rows).min(total);

    let mut lines: Vec<Line> = Vec::new();
    for i in start..end {
        let (_, label) = PHASES[i];
        let (mark, fg, bold_): (&str, Color, bool) = if i < current {
            ("✓", GREEN, false)
        } else if i == current {
            if app.status == InstallStatus::Running {
                ("▶", YELLOW, true)
            } else if app.status == InstallStatus::Success {
                ("✓", GREEN, true)
            } else {
                ("✘", RED, true)
            }
        } else {
            ("·", TEXT_DIM, false)
        };
        let mut st = Style::default().fg(fg);
        if bold_ {
            st = st.add_modifier(Modifier::BOLD);
        }
        lines.push(Line::from(vec![
            Span::styled(format!("{} ", mark), st),
            Span::styled(label.to_string(), st),
        ]));
    }
    f.render_widget(Paragraph::new(lines), inner);
}

fn draw_log(f: &mut Frame, inner: Rect, app: &App) {
    let rows = inner.height as usize;
    let mut lines: Vec<Line> = Vec::new();
    for entry in app.log.iter().rev().take(rows).rev() {
        let (fg, bold_) = match entry.kind {
            LogKind::Step => (CYAN, true),
            LogKind::Ok => (GREEN, false),
            LogKind::Warn => (YELLOW, false),
            LogKind::Err => (RED, true),
            LogKind::Info => (TEXT_DIM, false),
        };
        let mut st = Style::default().fg(fg);
        if bold_ {
            st = st.add_modifier(Modifier::BOLD);
        }
        let line = Line::from(vec![Span::styled(entry.text.clone(), st)]);
        lines.push(line);
    }
    f.render_widget(Paragraph::new(lines), inner);
}

fn draw_cancel_prompt(f: &mut Frame, area: Rect) {
    let mw: u16 = 44;
    let mh: u16 = 6;
    let mx = area.width.saturating_sub(mw) / 2;
    let my = area.height.saturating_sub(mh) / 2;
    let mrect = Rect::new(mx, my, mw, mh);
    f.render_widget(Clear, mrect);
    let block = Block::default()
        .borders(BORDERS)
        .border_type(BorderType::Rounded)
        .border_style(border_style(RED))
        .title(Span::styled(" CANCELAR INSTALACIÓN ", bold(RED)));
    let inner = block.inner(mrect);
    f.render_widget(block, mrect);
    let mut buf = f.buffer_mut();
    set_string(&mut buf, inner, 2, 1, "¿Seguro que quieres cancelar la instalación?", TEXT, BG);
    set_string(
        &mut buf,
        inner,
        2,
        2,
        "La próxima ejecución limpiará cualquier resto.",
        TEXT_DIM,
        BG,
    );
    set_string(
        &mut buf,
        inner,
        2,
        4,
        "[Enter] sí, cancelar · [Esc] no, continuar",
        YELLOW,
        BG,
    );
}

// ---------------------------------------------------------------------------
// Pantalla final
// ---------------------------------------------------------------------------
fn finished(f: &mut Frame, area: Rect, app: &App) {
    // Fondo: la animación elegida sigue corriendo.
    animations::render(f.buffer_mut(), area, app.anim_idx, app.now());

    let ok = app.status == InstallStatus::Success;
    let mw: u16 = 62;
    let mh: u16 = 14;
    let mx = area.width.saturating_sub(mw) / 2;
    let my = area.height.saturating_sub(mh) / 2;
    let mrect = Rect::new(mx, my, mw, mh);
    f.render_widget(Clear, mrect);

    let (accent, title) = if ok { (GREEN, "INSTALACIÓN COMPLETADA") } else { (RED, "ERROR EN LA INSTALACIÓN") };
    let block = Block::default()
        .borders(BORDERS)
        .border_type(BorderType::Rounded)
        .border_style(border_style(accent))
        .title(Span::styled(format!(" {} ", title), bold(accent)))
        .title_alignment(Alignment::Center);
    let inner = block.inner(mrect);
    f.render_widget(block, mrect);

    let mut buf = f.buffer_mut();
    if ok {
        set_string(&mut buf, inner, 2, 1, "✔  Tu entorno Spiderman se ha desplegado correctamente.", GREEN, BG);
        set_string(&mut buf, inner, 2, 3, "1. Cierra la sesión para ver el tema completo", TEXT, BG);
        set_string(&mut buf, inner, 2, 4, "   (splash, decoraciones y configuración de pantallas).", TEXT_DIM, BG);
        set_string(&mut buf, inner, 2, 5, "2. Aplicaciones manuales: JetBrains Toolbox, IDE y claves.", TEXT, BG);
        set_string(&mut buf, inner, 2, 6, "3. Para actualizar desde el sistema:  ./save.sh", TEXT, BG);
        set_string(&mut buf, inner, 2, 8, "Backup de la configuración anterior:", TEXT_DIM, BG);
        set_string(&mut buf, inner, 2, 9, "   ~/.dotfiles-backup-<fecha>  (borrable si todo va bien)", TEXT_DIM, BG);
        set_string(&mut buf, inner, 2, 11, "Presiona Enter para salir.", YELLOW, BG);
    } else {
        set_string(
            &mut buf,
            inner,
            2,
            1,
            &format!("La instalación falló con código de salida distinto de 0."),
            RED,
            BG,
        );
        set_string(
            &mut buf,
            inner,
            2,
            3,
            "Revisa la salida anterior o ejecuta ./install.sh en una terminal",
            TEXT,
            BG,
        );
        set_string(&mut buf, inner, 2, 4, "para ver el error completo.", TEXT, BG);
        set_string(&mut buf, inner, 2, 6, "La instalación es idempotente: puedes reintentarla sin riesgo.", TEXT_DIM, BG);
        set_string(&mut buf, inner, 2, 8, "[r] reintentar · [Enter] volver al inicio · [q] salir", YELLOW, BG);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::App;
    use ratatui::backend::TestBackend;

    fn snapshot(app: &App, cols: u16, rows: u16) -> String {
        let backend = TestBackend::new(cols, rows);
        let mut term = ratatui::Terminal::new(backend).unwrap();
        term.draw(|f| draw(f, app)).unwrap();
        let buf = term.backend().buffer();
        let mut out = String::new();
        for y in 0..buf.area.height {
            for x in 0..buf.area.width {
                let cell = &buf[(x, y)];
                out.push(cell.symbol().chars().next().unwrap_or(' '));
            }
            out.push('\n');
        }
        out
    }

    fn app() -> App {
        App::new()
    }

    #[test]
    fn welcome_renders() {
        let s = snapshot(&app(), 110, 40);
        assert!(s.contains("D O T F I L E S"));
        assert!(s.contains("MENÚ"));
        assert!(s.contains("INSTALAR"));
        assert!(s.contains("SALIR"));
    }

    #[test]
    fn setup_renders() {
        let mut a = app();
        a.screen = Screen::Setup;
        let s = snapshot(&a, 110, 40);
        std::fs::write("/tmp/opencode/snap_setup.txt", &s).unwrap();
        assert!(s.contains("OPCIONES"));
        assert!(s.contains("INICIAR INSTALACIÓN"));
        assert!(s.contains("Paquetes"));
    }

    #[test]
    fn install_renders() {
        let mut a = app();
        a.screen = Screen::Install;
        a.status = InstallStatus::Running;
        a.phase_idx = 4;
        a.current_step = "Descargando iconos Tela".to_string();
        let s = snapshot(&a, 110, 40);
        std::fs::write("/tmp/opencode/snap_install.txt", &s).unwrap();
        assert!(s.contains("PROGRESO"));
        assert!(s.contains("PASOS"));
        assert!(s.contains("SALIDA"));
        assert!(s.contains("Detección del sistema"));
    }

    #[test]
    fn finished_renders() {
        let mut a = app();
        a.screen = Screen::Finished;
        a.status = InstallStatus::Success;
        a.phase_idx = crate::runner::PHASES.len() - 1;
        let s = snapshot(&a, 110, 40);
        std::fs::write("/tmp/opencode/snap_finished.txt", &s).unwrap();
        assert!(s.contains("INSTALACIÓN COMPLETADA"));
    }
}
