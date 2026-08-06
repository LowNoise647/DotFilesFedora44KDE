//! Paleta, fuente de píxeles y utilidades de dibujo compartidas.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;

// ---------------------------------------------------------------------------
// Paleta
// ---------------------------------------------------------------------------
pub const BG: Color = Color::Rgb(9, 7, 18);
pub const BORDER: Color = Color::Rgb(80, 68, 130);
pub const BORDER_BRIGHT: Color = Color::Rgb(140, 120, 210);
pub const TEXT: Color = Color::Rgb(225, 222, 238);
pub const TEXT_DIM: Color = Color::Rgb(140, 134, 168);
pub const RED: Color = Color::Rgb(229, 46, 58);
pub const BLUE: Color = Color::Rgb(48, 120, 250);
pub const CYAN: Color = Color::Rgb(70, 222, 250);
pub const MAGENTA: Color = Color::Rgb(255, 70, 170);
pub const PURPLE: Color = Color::Rgb(170, 90, 255);
pub const GREEN: Color = Color::Rgb(70, 240, 150);
pub const GREEN_MATRIX: Color = Color::Rgb(20, 220, 120);
pub const YELLOW: Color = Color::Rgb(252, 200, 70);
pub const WHITE: Color = Color::Rgb(238, 238, 248);
pub const WEB: Color = Color::Rgb(226, 224, 232);
pub const BLACK: Color = Color::Rgb(8, 8, 14);

// ---------------------------------------------------------------------------
// Utilidades de color
// ---------------------------------------------------------------------------
pub fn fract(v: f64) -> f64 {
    v - v.floor()
}

pub fn smooth01(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn lerp_color(a: Color, b: Color, t: f64) -> Color {
    let (Color::Rgb(ar, ag, ab), Color::Rgb(br, bg, bb)) = (a, b) else {
        return a;
    };
    let t = t.clamp(0.0, 1.0);
    let f = |x: u8, y: u8| (x as f64 + (y as f64 - x as f64) * t).round() as u8;
    Color::Rgb(f(ar, br), f(ag, bg), f(ab, bb))
}

pub fn mix_rgb(a: (u8, u8, u8), b: (u8, u8, u8), t: f64) -> Color {
    let t = t.clamp(0.0, 1.0);
    let f = |x: u8, y: u8| (x as f64 + (y as f64 - x as f64) * t).round() as u8;
    Color::Rgb(f(a.0, b.0), f(a.1, b.1), f(a.2, b.2))
}

// ---------------------------------------------------------------------------
// Dibujo por celdas (coordenadas relativas al area, recortadas)
// ---------------------------------------------------------------------------
pub fn put(buf: &mut Buffer, area: Rect, x: i32, y: i32, ch: char, fg: Color, bg: Color) {
    if x < 0 || y < 0 {
        return;
    }
    let ax = area.x as i32 + x;
    let ay = area.y as i32 + y;
    if ax < 0 || ay < 0 || ax >= buf.area.width as i32 || ay >= buf.area.height as i32 {
        return;
    }
    let cell = &mut buf[(ax as u16, ay as u16)];
    cell.set_char(ch);
    cell.set_fg(fg);
    cell.set_bg(bg);
}

pub fn fill(buf: &mut Buffer, area: Rect, ch: char, fg: Color, bg: Color) {
    for y in area.y..area.y.saturating_add(area.height) {
        for x in area.x..area.x.saturating_add(area.width) {
            if x >= buf.area.width || y >= buf.area.height {
                continue;
            }
            let cell = &mut buf[(x, y)];
            cell.set_char(ch);
            cell.set_fg(fg);
            cell.set_bg(bg);
        }
    }
}

pub fn set_string(buf: &mut Buffer, area: Rect, x: i32, y: i32, s: &str, fg: Color, bg: Color) {
    let max = area.width as i32 - x;
    if max <= 0 {
        return;
    }
    for (i, ch) in s.chars().enumerate() {
        if i as i32 >= max {
            break;
        }
        put(buf, area, x + i as i32, y, ch, fg, bg);
    }
}

// Bresenham en coordenadas relativas.
pub fn line(buf: &mut Buffer, area: Rect, x0: i32, y0: i32, x1: i32, y1: i32, ch: char, fg: Color, bg: Color) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        put(buf, area, x0, y0, ch, fg, bg);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

pub fn circle_outline(buf: &mut Buffer, area: Rect, cx: f64, cy: f64, r: f64, ch: char, fg: Color, bg: Color) {
    let r0 = r.floor() as i32;
    for y in -r0..=r0 {
        for x in -r0..=r0 {
            let d = ((x as f64).powi(2) + (y as f64).powi(2)).sqrt();
            if (d - r).abs() < 0.9 {
                put(buf, area, cx as i32 + x, cy as i32 + y, ch, fg, bg);
            }
        }
    }
}

pub fn disc(buf: &mut Buffer, area: Rect, cx: f64, cy: f64, r: f64, ch: char, fg: Color, bg: Color) {
    let r0 = r.ceil() as i32;
    for y in -r0..=r0 {
        for x in -r0..=r0 {
            let d = ((x as f64).powi(2) + (y as f64).powi(2)).sqrt();
            if d <= r {
                put(buf, area, cx as i32 + x, cy as i32 + y, ch, fg, bg);
            }
        }
    }
}

// Hash determinista para animaciones.
pub fn hash(a: u64, b: u64) -> u64 {
    let mut x = a.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(b.wrapping_mul(0xC2B2_AE3D_27D4_EB4F));
    x ^= x >> 30;
    x = x.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^= x >> 31;
    x
}

// ---------------------------------------------------------------------------
// Fuente de píxeles 3x5
// ---------------------------------------------------------------------------
const FONT: &[(char, [&str; 5])] = &[
    ('A', ["111", "101", "111", "101", "101"]),
    ('B', ["110", "101", "110", "101", "110"]),
    ('C', ["111", "100", "100", "100", "111"]),
    ('D', ["110", "101", "101", "101", "110"]),
    ('E', ["111", "100", "110", "100", "111"]),
    ('F', ["111", "100", "110", "100", "100"]),
    ('G', ["111", "100", "101", "101", "111"]),
    ('H', ["101", "101", "111", "101", "101"]),
    ('I', ["111", "010", "010", "010", "111"]),
    ('J', ["001", "001", "001", "101", "111"]),
    ('K', ["101", "101", "100", "101", "101"]),
    ('L', ["100", "100", "100", "100", "111"]),
    ('M', ["101", "111", "111", "101", "101"]),
    ('N', ["101", "111", "101", "101", "101"]),
    ('O', ["111", "101", "101", "101", "111"]),
    ('P', ["111", "101", "111", "100", "100"]),
    ('Q', ["111", "101", "111", "011", "111"]),
    ('R', ["111", "101", "110", "101", "101"]),
    ('S', ["111", "100", "111", "001", "111"]),
    ('T', ["111", "010", "010", "010", "010"]),
    ('U', ["101", "101", "101", "101", "111"]),
    ('V', ["101", "101", "101", "101", "010"]),
    ('W', ["101", "101", "111", "111", "101"]),
    ('X', ["101", "101", "010", "101", "101"]),
    ('Y', ["101", "101", "010", "010", "010"]),
    ('Z', ["111", "001", "010", "100", "111"]),
    ('0', ["111", "101", "111", "101", "111"]),
    ('1', ["010", "110", "010", "010", "111"]),
    ('2', ["111", "001", "111", "100", "111"]),
    ('3', ["111", "001", "111", "001", "111"]),
    ('4', ["101", "101", "111", "001", "001"]),
    ('5', ["111", "100", "111", "001", "111"]),
    ('6', ["111", "100", "111", "101", "111"]),
    ('7', ["111", "001", "010", "010", "010"]),
    ('8', ["111", "101", "111", "101", "111"]),
    ('9', ["111", "101", "111", "001", "111"]),
    ('.', ["000", "000", "000", "000", "010"]),
    ('!', ["010", "010", "010", "000", "010"]),
    ('-', ["000", "000", "111", "000", "000"]),
    (':', ["000", "010", "000", "010", "000"]),
    ('?', ["111", "001", "010", "000", "010"]),
    ('_', ["000", "000", "000", "000", "111"]),
    ('·', ["000", "000", "000", "000", "000"]),
    (' ', ["000", "000", "000", "000", "000"]),
];

pub fn pixel_text(buf: &mut Buffer, area: Rect, x: i32, y: i32, text: &str, fg: Color, bg: Color, gap: i32) {
    let mut cx = x;
    for ch in text.chars() {
        if let Some(rows) = FONT.iter().find(|(c, _)| *c == ch) {
            for (r, row) in rows.1.iter().enumerate() {
                for (col, px) in row.chars().enumerate() {
                    if px == '1' {
                        put(buf, area, cx + col as i32, y + r as i32, '█', fg, bg);
                    }
                }
            }
        }
        cx += 3 + gap;
    }
}

pub fn pixel_width(text: &str) -> i32 {
    let mut w = 0;
    for ch in text.chars() {
        if FONT.iter().any(|(c, _)| *c == ch) {
            w += 3;
        } else {
            w += 3;
        }
        w += 1; // separación
    }
    w - 1
}

// Secuencia de animación de espera.
pub fn spinner(t: f64) -> char {
    const S: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    S[((t * 8.0) as usize) % S.len()]
}
