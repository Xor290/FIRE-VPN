use crate::app::VpnApp;
use crate::ui::theme;
use egui::{Rounding, Stroke, Vec2};

pub fn render(ui: &mut egui::Ui, app: &mut VpnApp) {
    theme::draw_top_accent(ui);

    // ── Header bar (mobile-style) ──────────────────────────────────────────
    ui.horizontal(|ui| {
        ui.add_space(4.0);

        // Logo
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

    ui.add_space(16.0);

    // ── Connection status hero (with pulsing animation) ────────────────────
    ui.vertical_centered(|ui| {
        ui.add_space(8.0);

        // Pulsing circle indicator (animated like mobile)
        let time = ui.input(|i| i.time);
        let pulse = ((time * 2.6).sin() as f32 + 1.0) / 2.0; // 0..1 oscillation

        let (rect, _) = ui.allocate_exact_size(Vec2::splat(80.0), egui::Sense::hover());
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let center = rect.center();

            // Animated outer ring (expands and fades like mobile pulseRing)
            let ring_scale = 1.0 + pulse * 0.4; // 1.0 to 1.4
            let ring_alpha = ((1.0 - pulse) * 0.6 * 255.0) as u8; // fades out
            let ring_radius = 30.0 * ring_scale;
            painter.circle_stroke(
                center,
                ring_radius,
                Stroke::new(
                    2.0,
                    egui::Color32::from_rgba_unmultiplied(63, 185, 80, ring_alpha),
                ),
            );

            // Static inner glow
            painter.circle_filled(center, 24.0, theme::SUCCESS_DIM);

            // Core dot
            painter.circle_filled(center, 10.0, theme::SUCCESS);
        }

        // Request repaint for animation
        ui.ctx().request_repaint();

        ui.add_space(12.0);

        ui.label(
            egui::RichText::new("CONNECTE")
                .size(20.0)
                .color(theme::SUCCESS)
                .strong(),
        );

        // Show connected server name with flag
        if let Some(idx) = app.get_selected_server() {
            let servers = app.get_servers();
            if let Some(server) = servers.get(idx) {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    if let Some(tex) = app.flag_store.get(&server.country) {
                        let size = egui::vec2(22.0, 14.0);
                        ui.add(egui::Image::new(tex).fit_to_exact_size(size).rounding(2.0));
                    }
                    ui.label(
                        egui::RichText::new(&server.name)
                            .size(14.0)
                            .color(theme::TEXT_SECONDARY),
                    );
                });
            }
        }
    });

    ui.add_space(20.0);

    // ── Connection details card ────────────────────────────────────────────
    if let Some(session) = app.get_session() {
        if let Some(config) = session.current_config() {
            let address = config.address.clone();
            let dns = config.dns.clone();

            theme::card_frame(ui, |ui| {
                ui.set_min_width(ui.available_width());

                ui.label(
                    egui::RichText::new("DETAILS DE CONNEXION")
                        .size(11.0)
                        .color(theme::TEXT_SECONDARY)
                        .strong(),
                );
                ui.add_space(12.0);

                theme::info_row(ui, "IP locale", &address);
                theme::info_row(ui, "DNS", &dns);
            });
        }
    }

    ui.add_space(16.0);

    // ── Server switch section ──────────────────────────────────────────────
    theme::section_heading(ui, "SERVEURS");

    let servers: Vec<_> = app
        .get_servers()
        .iter()
        .map(|s| (s.id, s.country.clone(), s.name.clone()))
        .collect();
    let selected = app.get_selected_server();

    let bottom_space = 80.0;
    let scroll_height = (ui.available_height() - bottom_space).max(60.0);

    egui::ScrollArea::vertical()
        .max_height(scroll_height)
        .show(ui, |ui| {
            for (idx, (_id, country, name)) in servers.iter().enumerate() {
                let is_current = selected == Some(idx);

                let fill = if is_current {
                    theme::BG_CARD_HOVER
                } else {
                    theme::BG_CARD
                };
                let border = if is_current {
                    Stroke::new(1.0, theme::SUCCESS)
                } else {
                    Stroke::new(1.0, theme::BORDER)
                };

                egui::Frame::none()
                    .fill(fill)
                    .rounding(Rounding::same(12.0))
                    .stroke(border)
                    .inner_margin(egui::Margin::symmetric(14.0, 10.0))
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        ui.horizontal(|ui| {
                            if let Some(tex) = app.flag_store.get(country) {
                                let size = egui::vec2(22.0, 14.0);
                                ui.add(egui::Image::new(tex).fit_to_exact_size(size).rounding(2.0));
                            }

                            ui.add_space(4.0);

                            let label = if is_current {
                                egui::RichText::new(name)
                                    .size(13.0)
                                    .color(theme::SUCCESS)
                                    .strong()
                            } else {
                                egui::RichText::new(name)
                                    .size(13.0)
                                    .color(theme::TEXT_PRIMARY)
                            };
                            ui.label(label);

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if is_current {
                                        theme::status_pill(ui, "ACTIF", theme::SUCCESS);
                                    } else if ui
                                        .add(
                                            egui::Label::new(
                                                egui::RichText::new("Changer")
                                                    .size(13.0)
                                                    .color(theme::ACCENT),
                                            )
                                            .sense(egui::Sense::click()),
                                        )
                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .clicked()
                                    {
                                        app.handle_switch_server(idx);
                                    }
                                },
                            );
                        });
                    });

                ui.add_space(8.0);
            }
        });

    // ── Footer with separator ──────────────────────────────────────────────
    ui.add_space(4.0);
    let separator_rect = ui.available_rect_before_wrap();
    let line = egui::Rect::from_min_size(
        separator_rect.min,
        egui::Vec2::new(separator_rect.width(), 1.0),
    );
    ui.painter()
        .rect_filled(line, Rounding::ZERO, theme::BORDER);
    ui.add_space(12.0);

    if theme::danger_button(ui, "Se deconnecter") {
        app.handle_disconnect();
    }
}
