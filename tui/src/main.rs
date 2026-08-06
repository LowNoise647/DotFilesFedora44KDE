#![forbid(unsafe_code)]

mod animations;
mod runner;
mod theme;
mod ui;

use std::io::{self, Write};
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use runner::InstallHandle;

// ---------------------------------------------------------------------------
// Estado de la aplicacion
// ---------------------------------------------------------------------------

#[derive(PartialEq, Clone, Copy)]
enum Screen {
    Welcome,
    Setup,
    Auth,
    Install,
    Finished,
}

#[derive(PartialEq, Clone, Copy)]
enum InstallStatus {
    Ready,
    Running,
    Success,
    Failed,
}

#[derive(Clone, Copy)]
struct Options {
    packages: bool,
    themes: bool,
    display: bool,
    restart: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            packages: true,
            themes: true,
            display: false,
            restart: true,
        }
    }
}

struct SudoState {
    needed: bool,
    password: Vec<char>,
    error: String,
}

impl Default for SudoState {
    fn default() -> Self {
        Self {
            needed: false,
            password: Vec::new(),
            error: String::new(),
        }
    }
}

struct App {
    screen: Screen,
    t0: Instant,
    anim_idx: usize,
    menu_idx: usize,
    options: Options,
    option_focus: usize,
    sudo: SudoState,
    log: Vec<runner::LogLine>,
    phase_idx: usize,
    current_step: String,
    status: InstallStatus,
    handle: Option<InstallHandle>,
    cancel_prompt: bool,
    detail_line: String,
}

impl App {
    fn new() -> Self {
        Self {
            screen: Screen::Welcome,
            t0: Instant::now(),
            anim_idx: 0,
            menu_idx: 0,
            options: Options::default(),
            option_focus: 0,
            sudo: SudoState::default(),
            log: Vec::new(),
            phase_idx: 0,
            current_step: runner::PHASES[0].1.to_string(),
            status: InstallStatus::Ready,
            handle: None,
            cancel_prompt: false,
            detail_line: String::new(),
        }
    }

    fn now(&self) -> f64 {
        self.t0.elapsed().as_secs_f64()
    }

    fn reset_install(&mut self) {
        self.log.clear();
        self.phase_idx = 0;
        self.current_step = runner::PHASES[0].1.to_string();
        self.status = InstallStatus::Ready;
        self.handle = None;
        self.cancel_prompt = false;
        self.detail_line.clear();
    }

    // Comprueba si sudo ya esta autenticado sin pedir contrasena.
    fn sudo_cached(&self) -> bool {
        std::process::Command::new("sudo")
            .args(["-n", "true"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    // Intenta autenticar con `sudo -S -v` usando la contrasena introducida.
    fn try_auth(&mut self) {
        let pass: String = self.sudo.password.iter().collect();
        let mut child = match std::process::Command::new("sudo")
            .args(["-S", "-v"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                self.sudo.error = format!("No se pudo lanzar sudo: {e}");
                return;
            }
        };
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(format!("{}\n", pass).as_bytes());
            let _ = stdin.flush();
        }
        let status = child.wait();
        match status {
            Ok(s) if s.success() => {
                self.sudo.password.clear();
                self.sudo.needed = false;
                self.begin_install();
            }
            Ok(_) => {
                self.sudo.error = "Contrasena incorrecta o usuario sin permisos sudo.".to_string();
            }
            Err(e) => self.sudo.error = format!("sudo fallo: {e}"),
        }
    }

    fn begin_install(&mut self) {
        match runner::spawn_installer(&self.options) {
            Ok(handle) => {
                self.reset_install();
                self.handle = Some(handle);
                self.status = InstallStatus::Running;
                self.screen = Screen::Install;
            }
            Err(e) => {
                self.status = InstallStatus::Failed;
                self.detail_line = format!("No se pudo lanzar install.sh: {e}");
                self.screen = Screen::Install;
            }
        }
    }

    // Arranca la instalacion, pidiendo sudo si hace falta.
    fn start_install(&mut self) {
        self.sudo.error.clear();
        if self.options.packages && !self.sudo_cached() {
            self.sudo.needed = true;
            self.sudo.password.clear();
            self.screen = Screen::Auth;
            return;
        }
        self.begin_install();
    }

    // Consume las lineas emitidas por install.sh.
    fn drain_output(&mut self) {
        let lines = {
            let Some(h) = self.handle.as_mut() else { return };
            h.try_recv()
        };
        for line in lines {
            let (kind, text) = runner::classify_line(&line);
            if kind == runner::LogKind::Step {
                self.advance_phase(&text);
            }
            self.log.push(runner::LogLine { kind, text });
            if self.log.len() > 400 {
                self.log.drain(..self.log.len() - 400);
            }
        }
        let exited = {
            let Some(h) = self.handle.as_mut() else { return };
            h.try_wait()
        };
        if let Some(status) = exited {
            let success = status.success();
            self.handle.take();
            if success {
                self.phase_idx = runner::PHASES.len() - 1;
                self.current_step = runner::PHASES[runner::PHASES.len() - 1].1.to_string();
                self.status = InstallStatus::Success;
                self.screen = Screen::Finished;
            } else {
                self.status = InstallStatus::Failed;
                self.screen = Screen::Finished;
            }
        }
    }

    fn advance_phase(&mut self, line: &str) {
        let lower = line.to_lowercase();
        for (i, (matcher, label)) in runner::PHASES.iter().enumerate() {
            if lower.contains(matcher) && i >= self.phase_idx {
                self.phase_idx = i;
                self.current_step = label.to_string();
                return;
            }
        }
    }

    fn progress(&self) -> f64 {
        if self.status == InstallStatus::Success {
            return 1.0;
        }
        let n = runner::PHASES.len() - 1;
        (self.phase_idx as f64) / (n as f64)
    }

    fn kill_install(&mut self) {
        if let Some(h) = self.handle.as_mut() {
            let _ = h.kill();
        }
    }
}

// ---------------------------------------------------------------------------
// Bucle principal
// ---------------------------------------------------------------------------

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    crossterm::execute!(io::stdout(), crossterm::cursor::Hide)?;

    let mut app = App::new();
    apply_args(&mut app, std::env::args().skip(1));

    let result = run_app(&mut terminal, app);

    crossterm::execute!(io::stdout(), crossterm::cursor::Show)?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    result
}

// Aplica los argumentos de linea de comandos (mismos que install.sh) para
// preconfigurar las opciones del instalador.
fn apply_args(app: &mut App, args: impl Iterator<Item = String>) {
    for arg in args {
        match arg.as_str() {
            "--with-packages" => app.options.packages = true,
            "--no-packages" => app.options.packages = false,
            "--with-themes" => app.options.themes = true,
            "--no-themes" => app.options.themes = false,
            "--with-display-config" => app.options.display = true,
            "--no-restart" => app.options.restart = false,
            "-h" | "--help" => {
                println!("Uso: spider-installer [opciones]");
                println!("  --with-packages / --no-packages   instalar paquetes dnf + Flatpak");
                println!("  --with-themes / --no-themes       descargar iconos Tela");
                println!("  --with-display-config             aplicar kwinoutputconfig.json");
                println!("  --no-restart                      no reiniciar plasmashell/KWin");
                println!("  --animation=N                     animacion inicial (0-4)");
                std::process::exit(0);
            }
            a if a.starts_with("--animation=") => {
                if let Ok(n) = a.trim_start_matches("--animation=").parse::<usize>() {
                    app.anim_idx = n.min(animations::COUNT - 1);
                }
            }
            _ => {}
        }
    }
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    mut app: App,
) -> io::Result<()> {
    let tick = Duration::from_millis(25);

    loop {
        app.drain_output();

        terminal.draw(|f| ui::draw(f, &app))?;

        if event::poll(tick)? {
            match event::read()? {
                Event::Key(key) => {
                    if handle_key(&mut app, key) {
                        return Ok(());
                    }
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }
}

fn handle_key(app: &mut App, key: event::KeyEvent) -> bool {
    if app.screen == Screen::Install && app.status == InstallStatus::Running && app.cancel_prompt {
        match key.code {
            KeyCode::Enter | KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Char('y')
            | KeyCode::Char('Y') => {
                app.kill_install();
                return true;
            }
            KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
                app.cancel_prompt = false;
            }
            _ => {}
        }
        return false;
    }

    match app.screen {
        Screen::Welcome => match key.code {
            KeyCode::Up | KeyCode::Char('k') => app.menu_idx = (app.menu_idx + 1) % 2,
            KeyCode::Down | KeyCode::Char('j') => app.menu_idx = (app.menu_idx + 1) % 2,
            KeyCode::Enter => {
                if app.menu_idx == 0 {
                    app.screen = Screen::Setup;
                } else {
                    return true;
                }
            }
            KeyCode::Char('q') | KeyCode::Esc => return true,
            _ => {}
        },
        Screen::Setup => match key.code {
            KeyCode::Left => {
                app.anim_idx = (app.anim_idx + animations::COUNT - 1) % animations::COUNT
            }
            KeyCode::Right => app.anim_idx = (app.anim_idx + 1) % animations::COUNT,
            KeyCode::Up | KeyCode::Char('k') => {
                app.option_focus = (app.option_focus + 4) % 5
            }
            KeyCode::Down | KeyCode::Char('j') => app.option_focus = (app.option_focus + 1) % 5,
            KeyCode::Char(' ') => {
                if app.option_focus < 4 {
                    toggle_option(app);
                } else {
                    app.start_install();
                }
            }
            KeyCode::Enter => app.start_install(),
            KeyCode::Char('q') | KeyCode::Esc => return true,
            _ => {}
        },
        Screen::Auth => match key.code {
            KeyCode::Char(c) => {
                if !key.modifiers.contains(KeyModifiers::CONTROL) {
                    app.sudo.password.push(c);
                }
            }
            KeyCode::Backspace => {
                app.sudo.password.pop();
            }
            KeyCode::Enter => app.try_auth(),
            KeyCode::Esc => {
                app.sudo.needed = false;
                app.sudo.password.clear();
                app.sudo.error.clear();
                app.screen = Screen::Setup;
            }
            _ => {}
        },
        Screen::Install => match key.code {
            KeyCode::Left => {
                app.anim_idx = (app.anim_idx + animations::COUNT - 1) % animations::COUNT
            }
            KeyCode::Right => app.anim_idx = (app.anim_idx + 1) % animations::COUNT,
            KeyCode::Esc => {
                if app.status == InstallStatus::Running {
                    app.cancel_prompt = true;
                } else {
                    app.screen = Screen::Welcome;
                }
            }
            KeyCode::Char('q') => {
                if app.status == InstallStatus::Running {
                    app.cancel_prompt = true;
                } else {
                    return true;
                }
            }
            _ => {}
        },
        Screen::Finished => match key.code {
            KeyCode::Enter | KeyCode::Esc | KeyCode::Char('q') => {
                if app.status == InstallStatus::Success {
                    return true;
                }
                app.screen = Screen::Welcome;
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                if app.status == InstallStatus::Failed {
                    app.screen = Screen::Setup;
                }
            }
            _ => {}
        },
    }
    false
}

fn toggle_option(app: &mut App) {
    match app.option_focus {
        0 => app.options.packages = !app.options.packages,
        1 => app.options.themes = !app.options.themes,
        2 => app.options.display = !app.options.display,
        3 => app.options.restart = !app.options.restart,
        _ => {}
    }
}
