//! Lanzamiento de install.sh en segundo plano, parsing de su salida y
//! calculo de fases/progreso.

use std::io::{BufRead, BufReader};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{self, Receiver};

// ---------------------------------------------------------------------------
// Fases de la instalacion: (matcher en minusculas, etiqueta mostrada)
// ---------------------------------------------------------------------------
pub const PHASES: &[(&str, &str)] = &[
    ("sistema detectado", "Detección del sistema"),
    ("instalando paquetes dnf", "Paquetes del sistema (dnf)"),
    ("anadiendo remoto flathub", "Añadiendo remoto Flathub"),
    ("instalando aplicaciones flatpak", "Aplicaciones Flatpak"),
    ("tela-circle", "Iconos Tela (descarga)"),
    ("instalacion limpia", "Instalación limpia (backup)"),
    ("dotfiles de", "Dotfiles de $HOME"),
    ("configuraciones", "Configuraciones (→ ~/.config)"),
    ("temas y datos de aplicaciones", "Temas y datos (→ ~/.local/share)"),
    ("wallpapers y recursos", "Wallpapers y recursos gráficos"),
    ("actualizando caches", "Actualizando caches"),
    ("aplicando ajustes kde", "Aplicando ajustes KDE"),
    ("pantallas", "Configuración de pantallas"),
    ("recargando la sesion plasma", "Recargando la sesión Plasma"),
    ("instalacion completada", "Instalación completada"),
];

// ---------------------------------------------------------------------------
// Linea de log
// ---------------------------------------------------------------------------
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum LogKind {
    Step,
    Ok,
    Warn,
    Err,
    Info,
}

pub struct LogLine {
    pub kind: LogKind,
    pub text: String,
}

// ---------------------------------------------------------------------------
// Manejador del proceso de instalacion
// ---------------------------------------------------------------------------
pub struct InstallHandle {
    child: Child,
    rx: Receiver<String>,
}

impl InstallHandle {
    pub fn try_recv(&mut self) -> Vec<String> {
        let mut out = Vec::new();
        while let Ok(line) = self.rx.try_recv() {
            out.push(line);
        }
        out
    }

    pub fn try_wait(&mut self) -> Option<std::process::ExitStatus> {
        match self.child.try_wait() {
            Ok(Some(status)) => Some(status),
            Ok(None) => None,
            Err(_) => None,
        }
    }

    pub fn kill(&mut self) {
        // El instalador se lanza en su propio grupo de procesos para poder
        // terminar tambien los subprocesos (dnf, flatpak, git, ...).
        let pgid = format!("-{}", self.child.id());
        let _ = Command::new("kill").args(["-TERM", &pgid]).status();
        let _ = self.child.wait();
    }
}

// ---------------------------------------------------------------------------
// Localizacion del repositorio
// ---------------------------------------------------------------------------
pub fn find_repo_dir() -> Result<PathBuf, String> {
    if let Ok(d) = std::env::var("DOTFILES_DIR") {
        if !d.is_empty() && PathBuf::from(&d).join("install.sh").exists() {
            return Ok(PathBuf::from(d));
        }
    }
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    let mut candidates = vec![cwd.clone()];
    let mut parent = cwd.parent();
    while let Some(p) = parent {
        candidates.push(p.to_path_buf());
        parent = p.parent();
    }
    for dir in candidates {
        if dir.join("install.sh").is_file() {
            return Ok(dir);
        }
    }
    Err("No se encontró install.sh (ejecuta la TUI desde el repositorio ~/dotfiles/tui o define DOTFILES_DIR)".into())
}

// ---------------------------------------------------------------------------
// Lanzador
// ---------------------------------------------------------------------------
pub fn spawn_installer(options: &crate::Options) -> Result<InstallHandle, String> {
    let repo = find_repo_dir()?;

    let mut cmd = Command::new("bash");
    cmd.arg("install.sh").arg("--yes").current_dir(&repo);
    if options.packages {
        cmd.arg("--with-packages");
    } else {
        cmd.arg("--no-packages");
    }
    if options.themes {
        cmd.arg("--with-themes");
    } else {
        cmd.arg("--no-themes");
    }
    if options.display {
        cmd.arg("--with-display-config");
    }
    if !options.restart {
        cmd.arg("--no-restart");
    }

    cmd.stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null())
        .process_group(0);

    let mut child = cmd.spawn().map_err(|e| format!("{e}"))?;

    let stdout = child.stdout.take().ok_or("sin stdout")?;
    let stderr = child.stderr.take().ok_or("sin stderr")?;

    let (tx, rx) = mpsc::channel::<String>();
    let tx_out = tx.clone();
    std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            if tx_out.send(line).is_err() {
                break;
            }
        }
    });
    let tx_err = tx.clone();
    std::thread::spawn(move || {
        for line in BufReader::new(stderr).lines().map_while(Result::ok) {
            if tx_err.send(line).is_err() {
                break;
            }
        }
    });

    Ok(InstallHandle { child, rx })
}

// ---------------------------------------------------------------------------
// Parsing de lineas
// ---------------------------------------------------------------------------
pub fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut it = s.chars();
    while let Some(c) = it.next() {
        if c == '\u{1b}' {
            // Consumir hasta la 'm' final de una secuencia CSI/SGR.
            for nxt in it.by_ref() {
                if nxt == 'm' {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

pub fn classify_line(raw: &str) -> (LogKind, String) {
    let clean = strip_ansi(raw);
    let t = clean.trim();
    let kind = if t.starts_with("==>") {
        LogKind::Step
    } else if t.starts_with("[ ok ]") {
        LogKind::Ok
    } else if t.starts_with("[warn]") {
        LogKind::Warn
    } else if t.starts_with("[err ]") || t.starts_with("[err]") {
        LogKind::Err
    } else if t.starts_with("[dotfiles]") || t.starts_with("[bootstrap]") {
        LogKind::Info
    } else {
        LogKind::Info
    };
    (kind, clean)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn match_phase(line: &str) -> Option<usize> {
        let lower = line.to_lowercase();
        PHASES
            .iter()
            .position(|(m, _)| lower.contains(m))
    }

    #[test]
    fn strip_ansi_removes_codes() {
        assert_eq!(strip_ansi("\u{1b}[31m[ ok ]\u{1b}[0m  x"), "[ ok ]  x");
        assert_eq!(strip_ansi("hola"), "hola");
    }

    #[test]
    fn classify_kinds() {
        assert_eq!(classify_line("==> Sistema detectado").0, LogKind::Step);
        assert_eq!(classify_line("[ ok ] copiado").0, LogKind::Ok);
        assert_eq!(classify_line("[warn] sin sudo").0, LogKind::Warn);
        assert_eq!(classify_line("[err ] fallo").0, LogKind::Err);
        assert_eq!(classify_line("[dotfiles] backup").0, LogKind::Info);
        assert_eq!(classify_line("salida de dnf").0, LogKind::Info);
    }

    #[test]
    fn phases_advance_in_order() {
        let steps = [
            "==> Sistema detectado",
            "==> Instalando paquetes dnf (3)",
            "==> Anadiendo remoto Flathub",
            "==> Instalando aplicaciones Flatpak (2)",
            "==> Descargando Tela-circle-icon-theme (master)",
            "==> Instalacion limpia (se elimina lo desplegado anteriormente, con backup)",
            "==> Dotfiles de /home/lownoise",
            "==> Configuraciones (-> ~/.config)",
            "==> Temas y datos de aplicaciones (-> ~/.local/share)",
            "==> Wallpapers y recursos graficos",
            "==> Actualizando caches",
            "==> Aplicando ajustes KDE (herramientas oficiales de Plasma)",
            "==> Aplicando configuracion de pantallas (kwinoutputconfig.json)",
            "==> Recargando la sesion Plasma",
            "==> Instalacion completada.",
        ];
        let mut prev = 0usize;
        for (i, s) in steps.iter().enumerate() {
            let idx = match_phase(s).expect("fase no encontrada");
            assert!(idx >= prev, "fase fuera de orden en el paso {i}: {s}");
            assert_eq!(idx, i, "orden de fases incorrecto en: {s}");
            prev = idx;
        }
    }

    #[test]
    fn find_repo_dir_looks_upward() {
        let dir = find_repo_dir().expect("no encontro el repositorio");
        assert!(dir.join("install.sh").is_file());
    }
}

