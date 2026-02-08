use crate::app::VpnApp;
use crate::ui::theme;
use egui::{Rounding, Stroke, Vec2};

pub fn render(ui: &mut egui::Ui, app: &mut VpnApp) {
    theme::draw_top_accent(ui);

    // ── Header bar ─────────────────────────────────────────────────────────
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("FIRE VPN")
                .size(18.0)
                .color(theme::ACCENT)
                .strong(),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .add(
                    egui::Label::new(
                        egui::RichText::new("Deconnexion")
                            .size(12.0)
                            .color(theme::TEXT_MUTED),
                    )
                    .sense(egui::Sense::click()),
                )
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                app.handle_logout();
            }
        });
    });

    ui.add_space(16.0);

    // ── Connection status hero ─────────────────────────────────────────────
    ui.vertical_centered(|ui| {
        // Pulsing circle indicator
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(80.0), egui::Sense::hover());
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let center = rect.center();

            // Outer glow ring
            painter.circle_stroke(center, 38.0, Stroke::new(2.0, theme::SUCCESS_DIM));
            // Inner filled circle
            painter.circle_filled(center, 28.0, theme::SUCCESS_DIM);
            // Core dot
            painter.circle_filled(center, 12.0, theme::SUCCESS);
        }

        ui.add_space(12.0);

        ui.label(
            egui::RichText::new("CONNECTE")
                .size(20.0)
                .color(theme::SUCCESS)
                .strong(),
        );

        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(app.get_connection_status())
                .size(13.0)
                .color(theme::TEXT_SECONDARY),
        );
    });

    ui.add_space(20.0);

    if let Some(session) = app.get_session() {
        if let Some(config) = session.current_config() {
            let address = config.address.clone();
            let dns = config.dns.clone();
            let endpoint = config.endpoint.clone();

            theme::card_frame(ui, |ui| {
                ui.set_min_width(ui.available_width());

                ui.label(
                    egui::RichText::new("DETAILS DE CONNEXION")
                        .size(11.0)
                        .color(theme::TEXT_MUTED)
                        .strong(),
                );
                ui.add_space(10.0);

                theme::info_row(ui, "IP locale", &address);
                ui.add_space(4.0);
                theme::info_row(ui, "DNS", &dns);
                ui.add_space(4.0);
                theme::info_row(ui, "Endpoint", &endpoint);
            });
        }
    }

    ui.add_space(16.0);

    // ── Server switch section ──────────────────────────────────────────────
    theme::section_heading(ui, "CHANGER DE SERVEUR");

    let servers: Vec<_> = app
        .get_servers()
        .iter()
        .map(|s| (s.id, s.country.clone(), s.name.clone()))
        .collect();
    let selected = app.get_selected_server();

    let bottom_space = 70.0;
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
                    .rounding(Rounding::same(8.0))
                    .stroke(border)
                    .inner_margin(egui::Margin::symmetric(14.0, 10.0))
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        ui.horizontal(|ui| {
                            let flag = theme::country_flag(country);
                            ui.label(
                                egui::RichText::new(flag)
                                    .size(12.0)
                                    .color(theme::TEXT_MUTED)
                                    .family(egui::FontFamily::Monospace),
                            );

                            let label = if is_current {
                                egui::RichText::new(format!("{} - {}", country, name))
                                    .size(13.0)
                                    .color(theme::SUCCESS)
                                    .strong()
                            } else {
                                egui::RichText::new(format!("{} - {}", country, name))
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
                                                    .size(11.0)
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

                ui.add_space(4.0);
            }
        });

    // ── Bottom disconnect button ───────────────────────────────────────────
    ui.add_space(8.0);
    if theme::danger_button(ui, "Se deconnecter") {
        app.handle_disconnect();
    }
}
