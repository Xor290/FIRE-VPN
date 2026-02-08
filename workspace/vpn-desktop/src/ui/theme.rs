use egui::{Color32, FontFamily, FontId, Margin, Pos2, Rect, Rounding, Stroke, Vec2};

// ── Color palette ──────────────────────────────────────────────────────────────
pub const BG_DARK: Color32 = Color32::from_rgb(13, 17, 23);
pub const BG_CARD: Color32 = Color32::from_rgb(22, 27, 34);
pub const BG_CARD_HOVER: Color32 = Color32::from_rgb(30, 37, 48);

pub const ACCENT: Color32 = Color32::from_rgb(88, 166, 255);
pub const ACCENT_HOVER: Color32 = Color32::from_rgb(110, 180, 255);
pub const ACCENT_DIM: Color32 = Color32::from_rgb(56, 110, 180);
pub const ACCENT_GLOW: Color32 = Color32::from_rgb(88, 166, 255);

pub const SUCCESS: Color32 = Color32::from_rgb(63, 185, 80);
pub const SUCCESS_DIM: Color32 = Color32::from_rgb(35, 90, 45);
pub const ERROR: Color32 = Color32::from_rgb(248, 81, 73);
pub const ERROR_DIM: Color32 = Color32::from_rgb(100, 30, 30);

pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(230, 237, 243);
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(139, 148, 158);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(110, 118, 129);

pub const BORDER: Color32 = Color32::from_rgb(48, 54, 61);

pub const DISCONNECT_RED: Color32 = Color32::from_rgb(218, 54, 51);
pub const DISCONNECT_RED_HOVER: Color32 = Color32::from_rgb(240, 70, 67);

pub fn draw_top_accent(ui: &mut egui::Ui) {
    let rect = ui.available_rect_before_wrap();
    let bar = Rect::from_min_size(rect.min, Vec2::new(rect.width(), 3.0));
    let painter = ui.painter();

    // Simple gradient effect with 3 segments
    let third = bar.width() / 3.0;
    let colors = [
        Color32::from_rgb(88, 166, 255),
        Color32::from_rgb(130, 80, 255),
        Color32::from_rgb(88, 166, 255),
    ];
    for (i, color) in colors.iter().enumerate() {
        let segment = Rect::from_min_size(
            Pos2::new(bar.min.x + third * i as f32, bar.min.y),
            Vec2::new(third, 3.0),
        );
        painter.rect_filled(segment, Rounding::ZERO, *color);
    }
    ui.add_space(8.0);
}

pub fn card_frame(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(BG_CARD)
        .rounding(Rounding::same(12.0))
        .stroke(Stroke::new(1.0, BORDER))
        .inner_margin(Margin::same(20.0))
        .show(ui, |ui| {
            add_contents(ui);
        });
}

pub fn primary_button(ui: &mut egui::Ui, text: &str, enabled: bool) -> bool {
    let size = Vec2::new(ui.available_width(), 44.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let (bg, text_color) = if !enabled {
            (ACCENT_DIM, TEXT_MUTED)
        } else if response.hovered() {
            (ACCENT_HOVER, BG_DARK)
        } else {
            (ACCENT, BG_DARK)
        };

        painter.rect_filled(rect, Rounding::same(10.0), bg);

        if enabled && response.hovered() {
            painter.rect_stroke(rect, Rounding::same(10.0), Stroke::new(1.0, ACCENT_GLOW));
        }

        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            FontId::new(14.0, FontFamily::Proportional),
            text_color,
        );
    }

    enabled && response.clicked()
}

pub fn danger_button(ui: &mut egui::Ui, text: &str) -> bool {
    let size = Vec2::new(ui.available_width(), 44.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let bg = if response.hovered() {
            DISCONNECT_RED_HOVER
        } else {
            DISCONNECT_RED
        };

        painter.rect_filled(rect, Rounding::same(10.0), bg);
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            FontId::new(14.0, FontFamily::Proportional),
            TEXT_PRIMARY,
        );
    }

    response.clicked()
}

/// Styled text input field with label
pub fn text_field(ui: &mut egui::Ui, label: &str, value: &mut String) -> egui::Response {
    ui.label(egui::RichText::new(label).size(12.0).color(TEXT_SECONDARY));
    ui.add_space(4.0);
    let response = ui.add(
        egui::TextEdit::singleline(value)
            .desired_width(ui.available_width())
            .margin(Margin::symmetric(12.0, 10.0))
            .font(FontId::new(14.0, FontFamily::Proportional)),
    );
    response
}

/// Password input field with label
pub fn password_field(ui: &mut egui::Ui, label: &str, value: &mut String) -> egui::Response {
    ui.label(egui::RichText::new(label).size(12.0).color(TEXT_SECONDARY));
    ui.add_space(4.0);
    let response = ui.add(
        egui::TextEdit::singleline(value)
            .password(true)
            .desired_width(ui.available_width())
            .margin(Margin::symmetric(12.0, 10.0))
            .font(FontId::new(14.0, FontFamily::Proportional)),
    );
    response
}

/// Section heading
pub fn section_heading(ui: &mut egui::Ui, text: &str) {
    ui.label(
        egui::RichText::new(text)
            .size(11.0)
            .color(TEXT_MUTED)
            .strong(),
    );
    ui.add_space(6.0);
}

/// Status pill (small colored badge)
pub fn status_pill(ui: &mut egui::Ui, text: &str, color: Color32) {
    let galley = ui.painter().layout_no_wrap(
        text.to_string(),
        FontId::new(11.0, FontFamily::Proportional),
        color,
    );
    let desired_size = galley.size() + Vec2::new(16.0, 6.0);
    let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

    if ui.is_rect_visible(rect) {
        let bg = Color32::from_rgba_premultiplied(color.r() / 4, color.g() / 4, color.b() / 4, 80);
        ui.painter().rect_filled(rect, Rounding::same(10.0), bg);
        ui.painter().rect_stroke(
            rect,
            Rounding::same(10.0),
            Stroke::new(
                1.0,
                Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 60),
            ),
        );
        ui.painter().galley(
            rect.center() - galley.size() / 2.0,
            galley,
            Color32::PLACEHOLDER,
        );
    }
}

/// Info row (label: value) for connection details
pub fn info_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).size(12.0).color(TEXT_MUTED));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(value)
                    .size(12.0)
                    .color(TEXT_PRIMARY)
                    .family(FontFamily::Monospace),
            );
        });
    });
}

/// Country flag emoji from country code
pub fn country_flag(country: &str) -> &str {
    match country.to_uppercase().as_str() {
        "FR" | "FRANCE" => "FR",
        "US" | "USA" | "UNITED STATES" => "US",
        "DE" | "GERMANY" | "ALLEMAGNE" => "DE",
        "UK" | "GB" | "UNITED KINGDOM" | "ROYAUME-UNI" => "GB",
        "NL" | "NETHERLANDS" | "PAYS-BAS" => "NL",
        "JP" | "JAPAN" | "JAPON" => "JP",
        "CA" | "CANADA" => "CA",
        "AU" | "AUSTRALIA" | "AUSTRALIE" => "AU",
        "SG" | "SINGAPORE" | "SINGAPOUR" => "SG",
        "CH" | "SWITZERLAND" | "SUISSE" => "CH",
        _ => "--",
    }
}
