use crate::app::VpnApp;
use crate::ui::theme;
use egui::{ColorImage, Rounding, Stroke};

const BG_GHOST: &[u8] = include_bytes!("../../assets/bg-ghost.jpg");

pub fn render(ui: &mut egui::Ui, app: &mut VpnApp) {
    // ── Background ghost image with low opacity ────────────────────────────
    let panel_rect = ui.available_rect_before_wrap();

    // Charger l'image depuis les bytes
    if let Ok(image) = image::load_from_memory(BG_GHOST) {
        let size = [image.width() as usize, image.height() as usize];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

        let texture = ui
            .ctx()
            .load_texture("bg-ghost", color_image, egui::TextureOptions::LINEAR);

        let bg_image = egui::Image::new(&texture)
            .tint(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 30))
            .fit_to_exact_size(panel_rect.size());
        bg_image.paint_at(ui, panel_rect);
    }

    theme::draw_top_accent(ui);

    // ── Header bar (mobile-style with profile button) ──────────────────────
    ui.horizontal(|ui| {
        ui.add_space(4.0);

        ui.label(
            egui::RichText::new("SilentGhostVPN")
                .size(16.0)
                .color(theme::ACCENT)
                .strong(),
        );

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Logout link
            if ui
                .add(
                    egui::Label::new(
                        egui::RichText::new("Deconnexion")
                            .size(13.0)
                            .color(theme::TEXT_MUTED),
                    )
                    .sense(egui::Sense::click()),
                )
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                app.handle_logout();
            }

            ui.add_space(8.0);

            // Profile button
            if theme::small_button(ui, "Profil", theme::ACCENT, theme::TEXT_PRIMARY) {
                app.show_profile();
            }
        });
    });

    ui.add_space(12.0);

    // ── Error banner ───────────────────────────────────────────────────────
    if let Some(error) = app.get_error() {
        let error = error.to_string();
        egui::Frame::none()
            .fill(egui::Color32::from_rgba_premultiplied(248, 81, 73, 15))
            .rounding(Rounding::same(6.0))
            .stroke(Stroke::new(
                1.0,
                egui::Color32::from_rgba_premultiplied(248, 81, 73, 40),
            ))
            .inner_margin(egui::Margin::same(10.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&error).size(13.0).color(theme::ERROR));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .add(
                                egui::Label::new(
                                    egui::RichText::new("✕").size(12.0).color(theme::ERROR),
                                )
                                .sense(egui::Sense::click()),
                            )
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .clicked()
                        {
                            app.clear_error();
                        }
                    });
                });
            });
        ui.add_space(8.0);
    }

    // ── Section heading ────────────────────────────────────────────────────
    theme::section_heading(ui, "SERVEURS DISPONIBLES");

    // ── Server list ────────────────────────────────────────────────────────
    let servers: Vec<_> = app
        .get_servers()
        .iter()
        .map(|s| (s.id, s.country.clone(), s.name.clone(), s.is_active))
        .collect();
    let selected = app.get_selected_server();

    let bottom_space = 80.0;
    let scroll_height = ui.available_height() - bottom_space;

    egui::ScrollArea::vertical()
        .max_height(scroll_height)
        .show(ui, |ui| {
            if servers.is_empty() {
                ui.add_space(40.0);
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Aucun serveur disponible")
                            .size(14.0)
                            .color(theme::TEXT_MUTED),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new("Verifiez votre connexion")
                            .size(12.0)
                            .color(theme::TEXT_MUTED),
                    );
                });
            } else {
                for (idx, (_id, country, name, is_active)) in servers.iter().enumerate() {
                    let is_selected = selected == Some(idx);
                    server_card(ui, idx, country, name, *is_active, is_selected, app);
                    ui.add_space(8.0);
                }
            }
        });

    // ── Footer pinned to bottom ────────────────────────────────────────────
    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
        ui.add_space(4.0);
        let can_connect = app.get_selected_server().is_some() && !app.is_connecting();
        let btn_text = if app.is_connecting() {
            "Connexion en cours..."
        } else {
            "Se connecter"
        };
        if theme::primary_button(ui, btn_text, can_connect) {
            app.handle_connect();
        }
        ui.add_space(4.0);
        let separator_rect = ui.available_rect_before_wrap();
        let line_y = separator_rect.max.y;
        let line = egui::Rect::from_min_size(
            egui::Pos2::new(separator_rect.min.x, line_y),
            egui::Vec2::new(separator_rect.width(), 1.0),
        );
        ui.painter()
            .rect_filled(line, Rounding::ZERO, theme::BORDER);
    });
}

fn server_card(
    ui: &mut egui::Ui,
    idx: usize,
    country: &str,
    name: &str,
    is_active: bool,
    is_selected: bool,
    app: &mut VpnApp,
) {
    let fill = if is_selected {
        theme::BG_CARD_HOVER
    } else {
        theme::BG_CARD
    };
    let border = if is_selected {
        Stroke::new(1.0, theme::ACCENT)
    } else {
        Stroke::new(1.0, theme::BORDER)
    };

    let response = egui::Frame::none()
        .fill(fill)
        .rounding(Rounding::same(12.0))
        .stroke(border)
        .inner_margin(egui::Margin::same(14.0))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());

            ui.horizontal(|ui| {
                // Country flag image
                if let Some(tex) = app.flag_store.get(country) {
                    let size = egui::vec2(24.0, 15.0);
                    ui.add(egui::Image::new(tex).fit_to_exact_size(size).rounding(2.0));
                }

                ui.add_space(4.0);

                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(name)
                            .size(14.0)
                            .color(theme::TEXT_PRIMARY)
                            .strong(),
                    );
                    ui.label(
                        egui::RichText::new(country)
                            .size(11.0)
                            .color(theme::TEXT_MUTED),
                    );
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (status_text, status_color) = if is_active {
                        ("EN LIGNE", theme::SUCCESS)
                    } else {
                        ("HORS LIGNE", theme::TEXT_MUTED)
                    };
                    theme::status_pill(ui, status_text, status_color);
                });
            });
        })
        .response;

    if response
        .interact(egui::Sense::click())
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
    {
        app.set_selected_server(idx);
    }
}
