use crate::app::VpnApp;
use crate::ui::theme;
use egui::{Rounding, Stroke, Vec2};

pub fn render(ui: &mut egui::Ui, app: &mut VpnApp) {
    theme::draw_top_accent(ui);

    // ── Header ─────────────────────────────────────────────────────────────
    let is_editing = app.is_profile_editing();

    ui.horizontal(|ui| {
        if ui
            .add(
                egui::Label::new(
                    egui::RichText::new("← Retour")
                        .size(13.0)
                        .color(theme::ACCENT),
                )
                .sense(egui::Sense::click()),
            )
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .clicked()
        {
            if is_editing {
                app.cancel_profile_edit();
            } else {
                app.hide_profile();
            }
        }
    });

    ui.add_space(4.0);

    // ── Centered column ────────────────────────────────────────────────────
    let content_width = 260.0_f32.min(ui.available_width() - 16.0);

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.allocate_ui(Vec2::new(content_width, 0.0), |ui| {
                ui.add_space(12.0);

                // ── Avatar + identity ──────────────────────────────────
                ui.vertical_centered(|ui| {
                    let initial = app
                        .get_session()
                        .map(|s| s.user().username.chars().next().unwrap_or('?'))
                        .unwrap_or('?');

                    theme::draw_avatar(ui, initial, 52.0);
                    ui.add_space(6.0);

                    if let Some(session) = app.get_session() {
                        ui.label(
                            egui::RichText::new(&session.user().username)
                                .size(15.0)
                                .color(theme::TEXT_PRIMARY)
                                .strong(),
                        );
                        ui.label(
                            egui::RichText::new(&session.user().email)
                                .size(11.0)
                                .color(theme::TEXT_SECONDARY),
                        );
                    }
                });

                ui.add_space(18.0);

                // ── Success message ────────────────────────────────────
                if let Some(msg) = app.get_profile_success() {
                    let msg = msg.to_string();
                    success_banner(ui, &msg);
                    ui.add_space(8.0);
                }

                if is_editing {
                    render_edit_mode(ui, app, content_width);
                } else {
                    render_view_mode(ui, app, content_width);
                }

                ui.add_space(16.0);
            });
        });
    });
}

// ── View mode ──────────────────────────────────────────────────────────────────

fn render_view_mode(ui: &mut egui::Ui, app: &mut VpnApp, content_width: f32) {
    // Account info
    section_label(ui, "COMPTE");

    compact_card(ui, |ui| {
        if let Some(session) = app.get_session() {
            row(ui, "ID", &format!("#{}", session.user().id), true);
            row(ui, "Utilisateur", &session.user().username, true);
            row(ui, "Email", &session.user().email, false);
        }
    });

    ui.add_space(10.0);

    // VPN status
    section_label(ui, "VPN");

    let is_connected = app.is_vpn_connected();
    let server_name = if is_connected {
        app.get_selected_server()
            .and_then(|idx| app.get_servers().get(idx))
            .map(|s| s.name.clone())
    } else {
        None
    };

    compact_card(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Statut")
                    .size(12.0)
                    .color(theme::TEXT_MUTED),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let (label, color) = if is_connected {
                    ("Connecte", theme::SUCCESS)
                } else {
                    ("Deconnecte", theme::TEXT_MUTED)
                };
                let (dot, _) = ui.allocate_exact_size(Vec2::splat(7.0), egui::Sense::hover());
                ui.painter().circle_filled(dot.center(), 3.0, color);
                ui.label(egui::RichText::new(label).size(12.0).color(color));
            });
        });

        if let Some(name) = &server_name {
            separator(ui);
            row(ui, "Serveur", name, false);
        }
    });

    ui.add_space(24.0);

    // Actions
    let btn_width = 200.0_f32.min(content_width);
    ui.vertical_centered(|ui| {
        ui.allocate_ui(Vec2::new(btn_width, 0.0), |ui| {
            if outline_button(ui, "Modifier le profil", theme::ACCENT) {
                app.start_profile_edit();
            }
        });

        ui.add_space(8.0);

        ui.allocate_ui(Vec2::new(btn_width, 0.0), |ui| {
            if outline_button(ui, "Se deconnecter", theme::TEXT_PRIMARY) {
                app.handle_logout();
            }
        });

        ui.add_space(10.0);

        if ui
            .add(
                egui::Label::new(
                    egui::RichText::new("Supprimer le compte")
                        .size(12.0)
                        .color(theme::DANGER),
                )
                .sense(egui::Sense::click()),
            )
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .clicked()
        {
            app.handle_delete_account();
        }
    });
}

// ── Edit mode ──────────────────────────────────────────────────────────────────

fn render_edit_mode(ui: &mut egui::Ui, app: &mut VpnApp, content_width: f32) {
    // Error message
    if let Some(err) = app.get_profile_error() {
        let err = err.to_string();
        error_banner(ui, &err);
        ui.add_space(8.0);
    }

    section_label(ui, "MODIFIER LE PROFIL");

    compact_card(ui, |ui| {
        // Username
        ui.label(
            egui::RichText::new("Nom d'utilisateur")
                .size(11.0)
                .color(theme::TEXT_SECONDARY),
        );
        ui.add_space(2.0);
        ui.add(
            egui::TextEdit::singleline(app.get_profile_username())
                .desired_width(ui.available_width())
                .margin(egui::Margin::symmetric(10.0, 8.0))
                .font(egui::FontId::new(13.0, egui::FontFamily::Proportional)),
        );

        ui.add_space(10.0);

        // Email
        ui.label(
            egui::RichText::new("Email")
                .size(11.0)
                .color(theme::TEXT_SECONDARY),
        );
        ui.add_space(2.0);
        ui.add(
            egui::TextEdit::singleline(app.get_profile_email())
                .desired_width(ui.available_width())
                .margin(egui::Margin::symmetric(10.0, 8.0))
                .font(egui::FontId::new(13.0, egui::FontFamily::Proportional)),
        );

        ui.add_space(10.0);

        // Password
        ui.label(
            egui::RichText::new("Nouveau mot de passe")
                .size(11.0)
                .color(theme::TEXT_SECONDARY),
        );
        ui.add_space(2.0);
        ui.add(
            egui::TextEdit::singleline(app.get_profile_password())
                .password(true)
                .desired_width(ui.available_width())
                .margin(egui::Margin::symmetric(10.0, 8.0))
                .font(egui::FontId::new(13.0, egui::FontFamily::Proportional))
                .hint_text("Min. 8 caracteres"),
        );
    });

    ui.add_space(16.0);

    // Buttons
    let btn_width = 200.0_f32.min(content_width);
    ui.vertical_centered(|ui| {
        ui.allocate_ui(Vec2::new(btn_width, 0.0), |ui| {
            if accent_button(ui, "Enregistrer") {
                app.handle_update_profile();
            }
        });

        ui.add_space(6.0);

        ui.allocate_ui(Vec2::new(btn_width, 0.0), |ui| {
            if outline_button(ui, "Annuler", theme::TEXT_MUTED) {
                app.cancel_profile_edit();
            }
        });
    });
}

// ── Helpers ────────────────────────────────────────────────────────────────────

fn section_label(ui: &mut egui::Ui, text: &str) {
    ui.label(
        egui::RichText::new(text)
            .size(10.0)
            .color(theme::TEXT_MUTED)
            .strong(),
    );
    ui.add_space(4.0);
}

fn compact_card(ui: &mut egui::Ui, content: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(theme::BG_CARD)
        .rounding(Rounding::same(8.0))
        .stroke(Stroke::new(1.0, theme::BORDER))
        .inner_margin(egui::Margin::symmetric(12.0, 8.0))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            content(ui);
        });
}

fn row(ui: &mut egui::Ui, label: &str, value: &str, show_separator: bool) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(label)
                .size(12.0)
                .color(theme::TEXT_MUTED),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(value)
                    .size(12.0)
                    .color(theme::TEXT_PRIMARY),
            );
        });
    });
    if show_separator {
        separator(ui);
    }
}

fn separator(ui: &mut egui::Ui) {
    ui.add_space(2.0);
    let rect = ui.available_rect_before_wrap();
    let line = egui::Rect::from_min_size(rect.min, Vec2::new(rect.width(), 1.0));
    ui.painter().rect_filled(
        line,
        Rounding::ZERO,
        egui::Color32::from_rgba_premultiplied(48, 60, 61, 25),
    );
    ui.add_space(4.0);
}

fn outline_button(ui: &mut egui::Ui, text: &str, text_color: egui::Color32) -> bool {
    let size = Vec2::new(ui.available_width(), 34.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let bg = if response.hovered() {
            theme::BG_CARD_HOVER
        } else {
            theme::BG_CARD
        };
        let painter = ui.painter();
        painter.rect_filled(rect, Rounding::same(6.0), bg);
        painter.rect_stroke(rect, Rounding::same(6.0), Stroke::new(1.0, theme::BORDER));
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::new(13.0, egui::FontFamily::Proportional),
            text_color,
        );
    }

    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
}

fn accent_button(ui: &mut egui::Ui, text: &str) -> bool {
    let size = Vec2::new(ui.available_width(), 34.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let bg = if response.hovered() {
            theme::ACCENT_HOVER
        } else {
            theme::ACCENT
        };
        let painter = ui.painter();
        painter.rect_filled(rect, Rounding::same(6.0), bg);
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::new(13.0, egui::FontFamily::Proportional),
            theme::TEXT_PRIMARY,
        );
    }

    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
}

fn error_banner(ui: &mut egui::Ui, text: &str) {
    egui::Frame::none()
        .fill(egui::Color32::from_rgba_premultiplied(248, 81, 73, 15))
        .rounding(Rounding::same(6.0))
        .stroke(Stroke::new(
            1.0,
            egui::Color32::from_rgba_premultiplied(248, 81, 73, 40),
        ))
        .inner_margin(egui::Margin::symmetric(10.0, 6.0))
        .show(ui, |ui| {
            ui.label(egui::RichText::new(text).size(12.0).color(theme::ERROR));
        });
}

fn success_banner(ui: &mut egui::Ui, text: &str) {
    egui::Frame::none()
        .fill(egui::Color32::from_rgba_premultiplied(63, 185, 80, 15))
        .rounding(Rounding::same(6.0))
        .stroke(Stroke::new(
            1.0,
            egui::Color32::from_rgba_premultiplied(63, 185, 80, 40),
        ))
        .inner_margin(egui::Margin::symmetric(10.0, 6.0))
        .show(ui, |ui| {
            ui.label(egui::RichText::new(text).size(12.0).color(theme::SUCCESS));
        });
}
