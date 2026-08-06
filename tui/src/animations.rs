//! Animaciones de la TUI. Cada una es una funcion pura que pinta celdas
//! dentro de un area en funcion del tiempo `t` (segundos).

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;

use crate::theme::*;

pub const COUNT: usize = 5;

pub struct AnimMeta {
    pub name: &'static str,
    pub tagline: &'static str,
    pub fg: Color,
    pub accent: Color,
}

pub fn metas() -> &'static [AnimMeta; COUNT] {
    const M: [AnimMeta; COUNT] = [
        AnimMeta {
            name: "Spider-Man",
            tagline: "Tu instalación, tu responsabilidad",
            fg: RED,
            accent: BLUE,
        },
        AnimMeta {
            name: "Matrix",
            tagline: "Sigue al conejo blanco",
            fg: GREEN_MATRIX,
            accent: WHITE,
        },
        AnimMeta {
            name: "Synthwave",
            tagline: "Noche ochentera de neón",
            fg: MAGENTA,
            accent: CYAN,
        },
        AnimMeta {
            name: "Neón Cibernético",
            tagline: "Protocolo de despliegue activo",
            fg: CYAN,
            accent: MAGENTA,
        },
        AnimMeta {
            name: "Aurora Boreal",
            tagline: "Luz del norte sobre tu escritorio",
            fg: Color::Rgb(120, 255, 190),
            accent: PURPLE,
        },
    ];
    &M
}

pub fn render(buf: &mut Buffer, area: Rect, idx: usize, t: f64) {
    if area.width < 4 || area.height < 4 {
        return;
    }
    match idx {
        0 => spider(buf, area, t),
        1 => matrix(buf, area, t),
        2 => synthwave(buf, area, t),
        3 => cyber(buf, area, t),
        _ => aurora(buf, area, t),
    }
}

// ---------------------------------------------------------------------------
// Spider-Man
// ---------------------------------------------------------------------------
const RAYS: &[f64] = &[0.0, 15.0, 30.0, 45.0, 60.0, 75.0, 90.0];

fn draw_web(
    buf: &mut Buffer,
    area: Rect,
    cx: i32,
    cy: i32,
    sx: i32,
    sy: i32,
    max_r: f64,
    t: f64,
    bright: f64,
) {
    let r_max = max_r * (0.35 + 0.65 * fract(t * 0.22));
    let box_ = r_max.ceil() as i32;
    let bg = BG;
    for y in -box_..=box_ {
        for x in -box_..=box_ {
            let u = sx as f64 * (x as f64);
            let v = sy as f64 * (y as f64);
            if u < 0.0 || v < 0.0 {
                continue;
            }
            let r = (u * u + v * v).sqrt();
            if r > r_max || r < 0.5 {
                continue;
            }
            let ang = v.atan2(u).to_degrees();
            let mut on_ray = false;
            for a in RAYS {
                if (ang - a).abs() < 1.7 {
                    on_ray = true;
                }
            }
            let mut on_arc = false;
            let mut arc_i = 0;
            for j in 1..=7 {
                let arc = 1.5 + j as f64 * 3.4;
                if arc <= r_max && (r - arc).abs() < 0.9 {
                    on_arc = true;
                    arc_i = j;
                }
            }
            if on_arc {
                let f = (arc_i as f64) / 7.0;
                let c = lerp_color(WEB, RED, f * 0.7);
                put(buf, area, cx + x, cy + y, '·', lerp_color(BG, c, bright), bg);
            } else if on_ray {
                let c = lerp_color(WEB, BLUE, 0.25);
                put(buf, area, cx + x, cy + y, '·', lerp_color(BG, c, bright), bg);
            }
        }
    }
    // Telaraña disparada (web shot) a lo largo de la diagonal.
    let shoot = fract(t * 0.8);
    let sr = r_max * (0.2 + 0.8 * shoot);
    let u = sr * 0.7071;
    for k in -2..=2 {
        let du = u + k as f64;
        put(buf, area, cx + sx * du as i32, cy + sy * du as i32, '·', WHITE, bg);
    }
}

fn draw_spider(buf: &mut Buffer, area: Rect, x: i32, y: i32) {
    let bg = BG;
    put(buf, area, x - 1, y - 1, '\\', WEB, bg);
    put(buf, area, x + 1, y - 1, '/', WEB, bg);
    put(buf, area, x - 1, y, '(', RED, bg);
    put(buf, area, x, y, 'o', RED, bg);
    put(buf, area, x + 1, y, ')', RED, bg);
    put(buf, area, x - 1, y + 1, '/', WEB, bg);
    put(buf, area, x + 1, y + 1, '\\', WEB, bg);
    put(buf, area, x, y - 2, 'o', RED, bg);
}

fn spider(buf: &mut Buffer, area: Rect, t: f64) {
    let w = area.width as i32;
    let h = area.height as i32;
    if w < 20 || h < 10 {
        return;
    }

    // Fondo degradado rojo oscuro -> casi negro.
    for y in 0..h {
        let f = y as f64 / h as f64;
        let c = mix_rgb((36, 6, 12), (8, 6, 16), f);
        for x in 0..w {
            put(buf, area, x, y, ' ', Color::Rgb(8, 6, 16), c);
        }
    }

    // Telarañas.
    let mw = w as f64;
    draw_web(buf, area, 1, 1, 1, 1, mw * 0.52, t, 1.0);
    draw_web(buf, area, w - 1, 1, -1, 1, mw * 0.30, t * 0.7, 0.65);
    draw_web(buf, area, w - 1, h - 1, -1, -1, mw * 0.38, t * 0.4, 0.5);

    // Araña columpiándose.
    let anchor_x = (w as f64 * 0.62) as i32;
    let len = (h as f64 * 0.32).max(5.0);
    let ang = (t * 1.25).sin() * 0.85;
    let sx = anchor_x as f64 + ang.sin() * len;
    let sy = (len * ang.cos()).abs().min((h - 4) as f64);
    let sx_i = sx as i32;
    let sy_i = sy as i32;
    let dx = sx_i - anchor_x;
    let dy = sy_i;
    let strand = if dx.abs() > dy.abs() {
        if dx > 0 { '\\' } else { '/' }
    } else {
        '|'
    };
    line(buf, area, anchor_x, 0, sx_i, sy_i - 2, strand, WEB, BG);
    draw_spider(buf, area, sx_i, sy_i);

    // Título.
    let ty = (h as f64 * 0.5 - 8.0).max(2.0) as i32;
    let w1 = pixel_width("SPIDER");
    let w2 = pixel_width("MAN");
    let total = w1 + w2 + 2;
    let tx = ((w - total) / 2).max(1);
    pixel_text(buf, area, tx, ty, "SPIDER", RED, BG, 0);
    pixel_text(buf, area, tx + w1 + 2, ty, "MAN", BLUE, BG, 0);

    let sub = "· D O T F I L E S ·";
    set_string(buf, area, (w - sub.len() as i32) / 2, ty + 7, sub, WEB, BG);

    // Lema animado.
    let motto = "«CON GRAN PODER VIENE UNA GRAN RESPONSABILIDAD»";
    let mc = if (t * 2.0).sin() > 0.0 { RED } else { BLUE };
    let my = h - 2;
    set_string(buf, area, (w - motto.len() as i32) / 2, my, motto, mc, BG);
}

// ---------------------------------------------------------------------------
// Matrix
// ---------------------------------------------------------------------------
fn matrix(buf: &mut Buffer, area: Rect, t: f64) {
    let w = area.width as usize;
    let h = area.height as usize;
    fill(buf, area, ' ', BLACK, BLACK);

    const GLYPHS: &[char] = &[
        'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ', 'シ', 'ス', 'セ', 'ソ', 'タ',
        'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 'ハ', 'ヒ', 'フ', 'ヘ', 'ホ', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'Z', 'X',
    ];

    for col in 0..w {
        let speed = 0.10 + (fract(hash(col as u64, 2) as f64) * 0.55);
        let phase = fract(hash(col as u64, 3) as f64) * 2.0;
        let total = h as f64 + 14.0;
        let head = (t * speed + phase * 0.9).rem_euclid(total);
        let head_y = head - 7.0;
        let frame = (t * 4.0) as u64;
        for i in 0..(h + 14) {
            let yf = head_y - i as f64;
            if yf < 0.0 || yf >= h as f64 {
                continue;
            }
            let color = if i == 0 {
                WHITE
            } else if i <= 2 {
                GREEN_MATRIX
            } else if i <= 6 {
                Color::Rgb(70, 210, 130)
            } else {
                Color::Rgb(22, 95, 55)
            };
            let gi = (hash(col as u64, i as u64 * 31 + frame)) as usize % GLYPHS.len();
            put(buf, area, col as i32, yf as i32, GLYPHS[gi], color, BLACK);
        }
    }
}

// ---------------------------------------------------------------------------
// Synthwave
// ---------------------------------------------------------------------------
fn synthwave(buf: &mut Buffer, area: Rect, t: f64) {
    let w = area.width as i32;
    let h = area.height as i32;
    let cx = w as f64 / 2.0;
    let horizon = (h as f64 * 0.45) as i32;
    let bottom = (h - 1).max(1);
    let sun_cy = horizon as f64 - (h as f64 * 0.16);
    let sun_r = (h as f64 * 0.24).max(3.0);

    // Fondo vertical degradado.
    for y in 0..h {
        let f = y as f64 / h as f64;
        let c = if y <= horizon {
            mix_rgb((16, 7, 38), (44, 13, 72), f * 2.0)
        } else {
            mix_rgb((20, 6, 46), (12, 4, 34), f)
        };
        for x in 0..w {
            put(buf, area, x, y, ' ', c, c);
        }
    }

    // Estrellas.
    let n_stars = (w.max(24) as usize) * 2;
    for i in 0..n_stars {
        let hx = hash(i as u64, 5);
        let sx = (hx % w as u64) as i32;
        let sy = (hash(i as u64, 9) % (sun_cy.max(2.0) as u64)) as i32;
        if sy >= 0 && sy < sun_cy as i32 {
            let tw = 0.3 + 0.7 * (t * 2.0 + (hash(i as u64, 13) % 1000) as f64 * 0.02).sin();
            let c = lerp_color(BG, WHITE, tw.abs());
            put(buf, area, sx, sy, '.', c, BG);
        }
    }

    // Sol de neón con franjas.
    for y in 0..horizon {
        if y % 5 == 0 {
            continue;
        }
        for x in 0..w {
            let dx = x as f64 - cx;
            let dy = y as f64 - sun_cy;
            let d = (dx * dx + dy * dy).sqrt();
            if d <= sun_r {
                let f = d / sun_r;
                let c = mix_rgb((255, 240, 180), (255, 60, 140), f);
                put(buf, area, x, y, '█', c, c);
            }
        }
    }

    // Líneas horizontales de la rejilla (perspectiva).
    let mut k = 1.0;
    loop {
        let yf = horizon as f64 + k * k * 0.55 + fract(t * 1.5);
        if yf >= bottom as f64 {
            break;
        }
        let f = (yf - horizon as f64) / (bottom as f64 - horizon as f64);
        let c = lerp_color(Color::Rgb(50, 24, 96), CYAN, f);
        for x in 0..w {
            put(buf, area, x, yf as i32, '─', c, BG);
        }
        k += 1.0;
    }

    // Líneas verticales que convergen en el punto de fuga.
    for y in (horizon + 1)..h {
        let dy = (bottom - y).max(1) as f64;
        for x in 0..w {
            let dx = x as f64 - cx;
            let ang = dx.atan2(dy).abs().to_degrees();
            if ang % 8.0 < 0.7 {
                let f = dy / (h as f64);
                let c = lerp_color(Color::Rgb(30, 40, 100), Color::Rgb(120, 235, 255), (1.0 - f) * 0.9);
                put(buf, area, x, y, '│', c, BG);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Neón Cibernético
// ---------------------------------------------------------------------------
fn cyber(buf: &mut Buffer, area: Rect, t: f64) {
    let w = area.width as i32;
    let h = area.height as i32;
    let cx = w as f64 / 2.0;
    let cy = h as f64 / 2.0;
    fill(buf, area, ' ', Color::Rgb(6, 8, 18), Color::Rgb(6, 8, 18));

    // Flujo binario en los laterales.
    for &col in &[0, 1, 2, w - 1, w - 2, w - 3] {
        if col < 0 || col >= w {
            continue;
        }
        for y in 0..h {
            let cell = hash(col as u64, y as u64 + (t * 8.0) as u64 * 7);
            let ch = if cell % 10 < 5 { '1' } else { '0' };
            let c = lerp_color(Color::Rgb(10, 40, 60), Color::Rgb(60, 220, 255), fract((cell % 1000) as f64 / 1000.0) * 0.7);
            put(buf, area, col, y, ch, c, Color::Rgb(6, 8, 18));
        }
    }

    // Anillos pulsantes.
    for ring in 0..4 {
        let ph = fract(t * 0.8 + ring as f64 * 0.27);
        let r = 2.0 + ph * (h as f64 * 0.52);
        let bright = (1.0 - ph) * 0.9;
        let c = lerp_color(Color::Rgb(16, 20, 60), CYAN, bright);
        circle_outline(buf, area, cx, cy, r, '·', c, Color::Rgb(6, 8, 18));
    }

    // Rombo rotatorio.
    let a = t * 0.7;
    let r = (h as f64 * 0.30).min(w as f64 * 0.28);
    let mut prev = (0.0f64, 0.0f64);
    for i in 0..4 {
        let ang = a + i as f64 * std::f64::consts::FRAC_PI_2;
        let (x, y) = (cx + r * ang.cos(), cy + r * ang.sin());
        if i > 0 {
            line(buf, area, prev.0 as i32, prev.1 as i32, x as i32, y as i32, '·', MAGENTA, Color::Rgb(6, 8, 18));
        }
        prev = (x, y);
    }
    line(buf, area, prev.0 as i32, prev.1 as i32, cx as i32 + r as i32, cy as i32, '·', MAGENTA, Color::Rgb(6, 8, 18));

    // Núcleo pulsante.
    let pr = 2.0 + 1.0 * (t * 3.0).sin().abs();
    let pc = lerp_color(MAGENTA, WHITE, 0.5 + 0.5 * (t * 4.0).sin());
    disc(buf, area, cx, cy, pr, '█', pc, pc);
}

// ---------------------------------------------------------------------------
// Aurora Boreal
// ---------------------------------------------------------------------------
fn aurora(buf: &mut Buffer, area: Rect, t: f64) {
    let w = area.width as i32;
    let h = area.height as i32;

    // Noche profunda.
    for y in 0..h {
        let f = y as f64 / h as f64;
        let c = mix_rgb((4, 6, 16), (14, 18, 36), smooth01(f));
        for x in 0..w {
            put(buf, area, x, y, ' ', c, c);
        }
    }

    // Estrellas.
    let n = (w.max(20) as usize) * 2;
    for i in 0..n {
        let sx = (hash(i as u64, 21) % w as u64) as i32;
        let sy = (hash(i as u64, 22) % (h as u64 * 6 / 10).max(1)) as i32;
        let tw = 0.25 + 0.75 * (t * 1.5 + (hash(i as u64, 23) % 1000) as f64 * 0.01).sin().abs();
        let c = lerp_color(Color::Rgb(60, 90, 160), WHITE, tw);
        put(buf, area, sx, sy, '.', c, Color::Rgb(4, 6, 16));
    }

    // Cortinas de luz.
    const PAL: &[(u8, u8, u8)] = &[
        (80, 255, 170),
        (60, 220, 255),
        (120, 90, 255),
        (255, 90, 200),
        (180, 255, 120),
    ];
    for x in 0..w {
        let fx = x as f64;
        let wave = h as f64 * 0.52
            + (h as f64 * 0.26)
                * ((fx * 0.09 + t * 0.55).sin() + 0.6 * (fx * 0.05 - t * 0.33).sin() + 0.4 * (fx * 0.16 + t * 0.2).sin());
        let cidx = (fx * 0.08 + t * 0.22).abs() as usize % PAL.len();
        for y in 0..h {
            let fy = y as f64;
            let d = fy - wave;
            let ad = d.abs();
            let col = if ad <= 2.0 {
                let b = 1.0 - ad / 2.0;
                mix_rgb(PAL[cidx], (235, 255, 245), b)
            } else if ad <= 6.0 {
                let b = 1.0 - (ad - 2.0) / 4.0;
                mix_rgb(PAL[cidx], (14, 18, 36), b * 0.85)
            } else if ad <= 13.0 {
                let b = 1.0 - (ad - 6.0) / 7.0;
                mix_rgb(PAL[(cidx + 2) % PAL.len()], (14, 18, 36), b * 0.4)
            } else {
                continue;
            };
            put(buf, area, x, y, '█', col, col);
        }
    }

    // Silueta del horizonte.
    let base = h as f64 * 0.84;
    for x in 0..w {
        let gx = base + (h as f64 * 0.04) * ((x as f64) * 0.06 + t * 0.12).sin();
        let mut gy = gx as i32;
        if gy < h - 1 && gy > 0 {
            gy += 1;
        }
        for y in gy..h {
            put(buf, area, x, y, '█', Color::Rgb(10, 12, 26), Color::Rgb(10, 12, 26));
        }
    }
}
