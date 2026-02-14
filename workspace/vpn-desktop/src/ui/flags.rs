use egui::{Color32, ColorImage, TextureHandle, TextureOptions};
use std::collections::HashMap;

/// Width and height of generated flag textures
const FLAG_W: usize = 32;
const FLAG_H: usize = 20;

/// Stores all pre-rendered flag textures, keyed by lowercase country code.
pub struct FlagStore {
    textures: HashMap<String, TextureHandle>,
    globe: Option<TextureHandle>,
}

impl FlagStore {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            globe: None,
        }
    }

    /// Load all flag textures into the GPU. Call once during init.
    pub fn load(&mut self, ctx: &egui::Context) {
        let flags: &[(&str, FlagDef)] = &[
            ("fr", tricolor_v(&[C_BLUE, C_WHITE, C_RED])),
            ("de", tricolor_h(&[C_BLACK, C_DE_RED, C_DE_GOLD])),
            ("it", tricolor_v(&[C_IT_GREEN, C_WHITE, C_IT_RED])),
            ("us", flag_us()),
            ("gb", flag_gb()),
            ("nl", tricolor_h(&[C_NL_RED, C_WHITE, C_NL_BLUE])),
            ("jp", flag_jp()),
            ("ca", flag_ca()),
            ("au", flag_au()),
            ("ch", flag_ch()),
            ("se", flag_se()),
            ("br", flag_br()),
            ("in", flag_in()),
            ("es", tricolor_h_weights(&[C_ES_RED, C_ES_YELLOW, C_ES_RED], &[1, 2, 1])),
            ("pl", bicolor_h(&[C_WHITE, C_PL_RED])),
            ("fi", flag_fi()),
            ("no", flag_no()),
            ("sg", flag_sg()),
        ];

        for (code, def) in flags {
            let img = render_flag(def);
            let handle =
                ctx.load_texture(format!("flag_{}", code), img, TextureOptions::LINEAR);
            self.textures.insert(code.to_string(), handle);
        }

        // Globe fallback
        let globe_img = render_globe();
        self.globe = Some(ctx.load_texture("flag_globe", globe_img, TextureOptions::LINEAR));
    }

    /// Get the texture for a country name/code. Returns globe as fallback.
    pub fn get(&self, country: &str) -> Option<&TextureHandle> {
        let key = match country.to_lowercase().as_str() {
            "fr" | "france" => "fr",
            "us" | "usa" | "united states" => "us",
            "de" | "germany" | "allemagne" => "de",
            "uk" | "gb" | "united kingdom" | "royaume-uni" => "gb",
            "nl" | "netherlands" | "pays-bas" => "nl",
            "jp" | "japan" | "japon" => "jp",
            "ca" | "canada" => "ca",
            "au" | "australia" | "australie" => "au",
            "sg" | "singapore" | "singapour" => "sg",
            "ch" | "switzerland" | "suisse" => "ch",
            "se" | "sweden" => "se",
            "br" | "brazil" => "br",
            "in" | "india" => "in",
            "es" | "spain" => "es",
            "it" | "italy" => "it",
            "pl" | "poland" => "pl",
            "fi" | "finland" => "fi",
            "no" | "norway" => "no",
            _ => "",
        };
        self.textures.get(key).or(self.globe.as_ref())
    }
}

// ── Color constants ────────────────────────────────────────────────────────────

const C_WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const C_BLACK: Color32 = Color32::from_rgb(0, 0, 0);
const C_RED: Color32 = Color32::from_rgb(239, 65, 53);
const C_BLUE: Color32 = Color32::from_rgb(0, 85, 164);

// DE
const C_DE_RED: Color32 = Color32::from_rgb(221, 0, 0);
const C_DE_GOLD: Color32 = Color32::from_rgb(255, 206, 0);

// IT
const C_IT_GREEN: Color32 = Color32::from_rgb(0, 146, 70);
const C_IT_RED: Color32 = Color32::from_rgb(206, 43, 55);

// NL
const C_NL_RED: Color32 = Color32::from_rgb(174, 28, 40);
const C_NL_BLUE: Color32 = Color32::from_rgb(33, 70, 139);

// ES
const C_ES_RED: Color32 = Color32::from_rgb(198, 11, 30);
const C_ES_YELLOW: Color32 = Color32::from_rgb(255, 196, 0);

// PL
const C_PL_RED: Color32 = Color32::from_rgb(220, 20, 60);

// US
const C_US_RED: Color32 = Color32::from_rgb(191, 10, 48);
const C_US_BLUE: Color32 = Color32::from_rgb(0, 40, 104);

// GB
const C_GB_RED: Color32 = Color32::from_rgb(200, 16, 46);
const C_GB_BLUE: Color32 = Color32::from_rgb(0, 36, 125);

// JP
const C_JP_RED: Color32 = Color32::from_rgb(188, 0, 45);

// CA
const C_CA_RED: Color32 = Color32::from_rgb(255, 0, 0);

// SE
const C_SE_BLUE: Color32 = Color32::from_rgb(0, 106, 167);
const C_SE_YELLOW: Color32 = Color32::from_rgb(254, 204, 2);

// FI
const C_FI_BLUE: Color32 = Color32::from_rgb(0, 47, 108);

// NO
const C_NO_RED: Color32 = Color32::from_rgb(186, 12, 47);
const C_NO_BLUE: Color32 = Color32::from_rgb(0, 32, 91);

// BR
const C_BR_GREEN: Color32 = Color32::from_rgb(0, 155, 58);
const C_BR_YELLOW: Color32 = Color32::from_rgb(255, 223, 0);
const C_BR_BLUE: Color32 = Color32::from_rgb(0, 39, 118);

// IN
const C_IN_SAFFRON: Color32 = Color32::from_rgb(255, 153, 51);
const C_IN_GREEN: Color32 = Color32::from_rgb(19, 136, 8);
const C_IN_BLUE: Color32 = Color32::from_rgb(0, 0, 128);

// AU
const C_AU_BLUE: Color32 = Color32::from_rgb(0, 0, 139);

// SG
const C_SG_RED: Color32 = Color32::from_rgb(237, 28, 36);

// ── Flag definition type ───────────────────────────────────────────────────────

type FlagDef = Vec<Vec<Color32>>;

fn new_canvas(color: Color32) -> FlagDef {
    vec![vec![color; FLAG_W]; FLAG_H]
}

fn render_flag(def: &FlagDef) -> ColorImage {
    let mut pixels = Vec::with_capacity(FLAG_W * FLAG_H);
    for row in def {
        for &c in row {
            pixels.push(c);
        }
    }
    ColorImage {
        size: [FLAG_W, FLAG_H],
        pixels,
    }
}

// ── Simple flag builders ───────────────────────────────────────────────────────

/// Three vertical stripes (like France, Italy)
fn tricolor_v(colors: &[Color32; 3]) -> FlagDef {
    let third = FLAG_W / 3;
    let mut canvas = new_canvas(colors[2]);
    for row in canvas.iter_mut() {
        for x in 0..FLAG_W {
            row[x] = if x < third {
                colors[0]
            } else if x < third * 2 {
                colors[1]
            } else {
                colors[2]
            };
        }
    }
    canvas
}

/// Three horizontal stripes (like Germany, Netherlands)
fn tricolor_h(colors: &[Color32; 3]) -> FlagDef {
    let third = FLAG_H / 3;
    let mut canvas = new_canvas(colors[2]);
    for (y, row) in canvas.iter_mut().enumerate() {
        let c = if y < third {
            colors[0]
        } else if y < third * 2 {
            colors[1]
        } else {
            colors[2]
        };
        for px in row.iter_mut() {
            *px = c;
        }
    }
    canvas
}

/// Horizontal stripes with custom weights (like Spain: 1-2-1)
fn tricolor_h_weights(colors: &[Color32; 3], weights: &[usize; 3]) -> FlagDef {
    let total: usize = weights.iter().sum();
    let mut canvas = new_canvas(C_WHITE);
    for (y, row) in canvas.iter_mut().enumerate() {
        let pos = y * total / FLAG_H;
        let c = if pos < weights[0] {
            colors[0]
        } else if pos < weights[0] + weights[1] {
            colors[1]
        } else {
            colors[2]
        };
        for px in row.iter_mut() {
            *px = c;
        }
    }
    canvas
}

/// Two horizontal stripes (like Poland)
fn bicolor_h(colors: &[Color32; 2]) -> FlagDef {
    let half = FLAG_H / 2;
    let mut canvas = new_canvas(colors[1]);
    for (y, row) in canvas.iter_mut().enumerate() {
        let c = if y < half { colors[0] } else { colors[1] };
        for px in row.iter_mut() {
            *px = c;
        }
    }
    canvas
}

// ── Complex flags ──────────────────────────────────────────────────────────────

fn flag_us() -> FlagDef {
    let mut canvas = new_canvas(C_WHITE);
    let stripe_h = FLAG_H / 13;

    // 13 stripes
    for (y, row) in canvas.iter_mut().enumerate() {
        let stripe_idx = y / stripe_h.max(1);
        let c = if stripe_idx % 2 == 0 { C_US_RED } else { C_WHITE };
        for px in row.iter_mut() {
            *px = c;
        }
    }

    // Blue canton (top-left)
    let canton_h = (FLAG_H * 7) / 13;
    let canton_w = FLAG_W * 2 / 5;
    for y in 0..canton_h {
        for x in 0..canton_w {
            canvas[y][x] = C_US_BLUE;
        }
    }

    // Stars (simplified: dots in a grid)
    let star_rows = 4;
    let star_cols = 4;
    for sr in 0..star_rows {
        for sc in 0..star_cols {
            let sx = (sc * canton_w / star_cols) + canton_w / (star_cols * 2);
            let sy = (sr * canton_h / star_rows) + canton_h / (star_rows * 2);
            if sy < FLAG_H && sx < FLAG_W {
                canvas[sy][sx] = C_WHITE;
            }
        }
    }

    canvas
}

fn flag_gb() -> FlagDef {
    let mut canvas = new_canvas(C_GB_BLUE);
    let cx = FLAG_W / 2;
    let cy = FLAG_H / 2;

    // White diagonals (St Andrew / St Patrick base)
    for y in 0..FLAG_H {
        for x in 0..FLAG_W {
            let dx = (x as f32 - cx as f32) / FLAG_W as f32;
            let dy = (y as f32 - cy as f32) / FLAG_H as f32;
            if (dx - dy).abs() < 0.08 || (dx + dy).abs() < 0.08 {
                canvas[y][x] = C_WHITE;
            }
        }
    }

    // Red diagonals (thinner)
    for y in 0..FLAG_H {
        for x in 0..FLAG_W {
            let dx = (x as f32 - cx as f32) / FLAG_W as f32;
            let dy = (y as f32 - cy as f32) / FLAG_H as f32;
            if (dx - dy).abs() < 0.035 || (dx + dy).abs() < 0.035 {
                canvas[y][x] = C_GB_RED;
            }
        }
    }

    // White cross (St George base)
    let cross_w = 3;
    let cross_h = 2;
    for y in 0..FLAG_H {
        for x in 0..FLAG_W {
            if (x as i32 - cx as i32).unsigned_abs() as usize <= cross_w
                || (y as i32 - cy as i32).unsigned_abs() as usize <= cross_h
            {
                canvas[y][x] = C_WHITE;
            }
        }
    }

    // Red cross (St George)
    let rcross_w = 2;
    let rcross_h = 1;
    for y in 0..FLAG_H {
        for x in 0..FLAG_W {
            if (x as i32 - cx as i32).unsigned_abs() as usize <= rcross_w
                || (y as i32 - cy as i32).unsigned_abs() as usize <= rcross_h
            {
                canvas[y][x] = C_GB_RED;
            }
        }
    }

    canvas
}

fn flag_jp() -> FlagDef {
    let mut canvas = new_canvas(C_WHITE);
    let cx = FLAG_W as f32 / 2.0;
    let cy = FLAG_H as f32 / 2.0;
    let r = FLAG_H as f32 * 0.35;

    for y in 0..FLAG_H {
        for x in 0..FLAG_W {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            if dx * dx + dy * dy <= r * r {
                canvas[y][x] = C_JP_RED;
            }
        }
    }
    canvas
}

fn flag_ca() -> FlagDef {
    let mut canvas = new_canvas(C_WHITE);
    let bar_w = FLAG_W / 4;

    // Red bars left and right
    for y in 0..FLAG_H {
        for x in 0..bar_w {
            canvas[y][x] = C_CA_RED;
        }
        for x in (FLAG_W - bar_w)..FLAG_W {
            canvas[y][x] = C_CA_RED;
        }
    }

    // Maple leaf (simplified diamond shape in center)
    let cx = FLAG_W / 2;
    let cy = FLAG_H / 2;
    for y in 0..FLAG_H {
        for x in 0..FLAG_W {
            let dx = (x as i32 - cx as i32).unsigned_abs() as f32;
            let dy = (y as i32 - cy as i32).unsigned_abs() as f32;
            let h = FLAG_H as f32 * 0.4;
            if dx + dy * 1.5 < h {
                canvas[y][x] = C_CA_RED;
            }
        }
    }

    canvas
}

fn flag_ch() -> FlagDef {
    let mut canvas = new_canvas(C_RED);
    let cx = FLAG_W / 2;
    let cy = FLAG_H / 2;

    // White cross
    let arm_w = 2;
    let arm_h = (FLAG_H as f32 * 0.35) as usize;
    let arm_w2 = (FLAG_W as f32 * 0.18) as usize;
    for y in 0..FLAG_H {
        for x in 0..FLAG_W {
            let dx = (x as i32 - cx as i32).unsigned_abs() as usize;
            let dy = (y as i32 - cy as i32).unsigned_abs() as usize;
            if (dx <= arm_w && dy <= arm_h) || (dy <= arm_w && dx <= arm_w2) {
                canvas[y][x] = C_WHITE;
            }
        }
    }
    canvas
}

fn flag_se() -> FlagDef {
    let mut canvas = new_canvas(C_SE_BLUE);
    let cross_x = FLAG_W * 3 / 8;
    let cross_w = 2;
    let cross_h = 2;

    for y in 0..FLAG_H {
        for x in 0..FLAG_W {
            let cy = FLAG_H / 2;
            if (x as i32 - cross_x as i32).unsigned_abs() as usize <= cross_w
                || (y as i32 - cy as i32).unsigned_abs() as usize <= cross_h
            {
                canvas[y][x] = C_SE_YELLOW;
            }
        }
    }
    canvas
}

fn flag_fi() -> FlagDef {
    let mut canvas = new_canvas(C_WHITE);
    let cross_x = FLAG_W * 3 / 8;
    let cross_w = 2;
    let cross_h = 2;

    for y in 0..FLAG_H {
        for x in 0..FLAG_W {
            let cy = FLAG_H / 2;
            if (x as i32 - cross_x as i32).unsigned_abs() as usize <= cross_w
                || (y as i32 - cy as i32).unsigned_abs() as usize <= cross_h
            {
                canvas[y][x] = C_FI_BLUE;
            }
        }
    }
    canvas
}

fn flag_no() -> FlagDef {
    let mut canvas = new_canvas(C_NO_RED);
    let cross_x = FLAG_W * 3 / 8;

    // White cross (wider)
    let white_w = 3;
    let white_h = 3;
    for y in 0..FLAG_H {
        for x in 0..FLAG_W {
            let cy = FLAG_H / 2;
            if (x as i32 - cross_x as i32).unsigned_abs() as usize <= white_w
                || (y as i32 - cy as i32).unsigned_abs() as usize <= white_h
            {
                canvas[y][x] = C_WHITE;
            }
        }
    }

    // Blue cross (thinner, on top)
    let blue_w = 1;
    let blue_h = 1;
    for y in 0..FLAG_H {
        for x in 0..FLAG_W {
            let cy = FLAG_H / 2;
            if (x as i32 - cross_x as i32).unsigned_abs() as usize <= blue_w
                || (y as i32 - cy as i32).unsigned_abs() as usize <= blue_h
            {
                canvas[y][x] = C_NO_BLUE;
            }
        }
    }
    canvas
}

fn flag_br() -> FlagDef {
    let mut canvas = new_canvas(C_BR_GREEN);
    let cx = FLAG_W as f32 / 2.0;
    let cy = FLAG_H as f32 / 2.0;

    // Yellow diamond
    for y in 0..FLAG_H {
        for x in 0..FLAG_W {
            let dx = (x as f32 - cx).abs() / (FLAG_W as f32 * 0.45);
            let dy = (y as f32 - cy).abs() / (FLAG_H as f32 * 0.42);
            if dx + dy < 1.0 {
                canvas[y][x] = C_BR_YELLOW;
            }
        }
    }

    // Blue circle
    let r = FLAG_H as f32 * 0.28;
    for y in 0..FLAG_H {
        for x in 0..FLAG_W {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            if dx * dx + dy * dy <= r * r {
                canvas[y][x] = C_BR_BLUE;
            }
        }
    }

    canvas
}

fn flag_in() -> FlagDef {
    let third = FLAG_H / 3;
    let mut canvas = new_canvas(C_IN_GREEN);

    for (y, row) in canvas.iter_mut().enumerate() {
        let c = if y < third {
            C_IN_SAFFRON
        } else if y < third * 2 {
            C_WHITE
        } else {
            C_IN_GREEN
        };
        for px in row.iter_mut() {
            *px = c;
        }
    }

    // Ashoka Chakra (blue circle in center)
    let cx = FLAG_W as f32 / 2.0;
    let cy = FLAG_H as f32 / 2.0;
    let r = (third as f32) * 0.4;
    for y in 0..FLAG_H {
        for x in 0..FLAG_W {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            if (dist - r).abs() < 1.0 {
                canvas[y][x] = C_IN_BLUE;
            }
        }
    }

    canvas
}

fn flag_au() -> FlagDef {
    let mut canvas = new_canvas(C_AU_BLUE);

    // Union Jack in top-left quarter (simplified)
    let qw = FLAG_W / 2;
    let qh = FLAG_H / 2;
    let qcx = qw / 2;
    let qcy = qh / 2;

    for y in 0..qh {
        for x in 0..qw {
            // White diagonals
            let dx = (x as f32 - qcx as f32) / qw as f32;
            let dy = (y as f32 - qcy as f32) / qh as f32;
            if (dx - dy).abs() < 0.1 || (dx + dy).abs() < 0.1 {
                canvas[y][x] = C_WHITE;
            }
        }
    }
    // Red cross in canton
    for y in 0..qh {
        for x in 0..qw {
            if (x as i32 - qcx as i32).unsigned_abs() <= 1
                || (y as i32 - qcy as i32).unsigned_abs() <= 1
            {
                canvas[y][x] = C_GB_RED;
            }
        }
    }

    // Stars (white dots for Southern Cross - simplified)
    let stars = [(24, 5), (28, 10), (24, 15), (20, 10), (16, 17)];
    for (sx, sy) in stars {
        if sx < FLAG_W && sy < FLAG_H {
            canvas[sy][sx] = C_WHITE;
            if sx + 1 < FLAG_W {
                canvas[sy][sx + 1] = C_WHITE;
            }
            if sy + 1 < FLAG_H {
                canvas[sy + 1][sx] = C_WHITE;
            }
        }
    }

    canvas
}

fn flag_sg() -> FlagDef {
    let half = FLAG_H / 2;
    let mut canvas = new_canvas(C_WHITE);

    // Top red
    for y in 0..half {
        for x in 0..FLAG_W {
            canvas[y][x] = C_SG_RED;
        }
    }

    // Crescent + stars in top-left (simplified: white circle cutout)
    let cx = 8.0_f32;
    let cy = (half as f32) / 2.0;
    let r = half as f32 * 0.35;
    for y in 0..half {
        for x in 0..FLAG_W / 2 {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            if dx * dx + dy * dy <= r * r {
                canvas[y][x] = C_WHITE;
            }
            // Cut inner circle for crescent
            let dx2 = x as f32 - (cx + 2.0);
            if dx2 * dx2 + dy * dy <= (r * 0.8) * (r * 0.8) {
                canvas[y][x] = C_SG_RED;
            }
        }
    }

    canvas
}

fn render_globe() -> ColorImage {
    let mut canvas = new_canvas(Color32::from_rgb(40, 50, 60));
    let cx = FLAG_W as f32 / 2.0;
    let cy = FLAG_H as f32 / 2.0;
    let r = FLAG_H as f32 * 0.42;

    for y in 0..FLAG_H {
        for x in 0..FLAG_W {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = dx * dx + dy * dy;
            if dist <= r * r {
                canvas[y][x] = Color32::from_rgb(70, 130, 180);
            }
        }
    }

    render_flag(&canvas)
}
