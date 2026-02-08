use crate::app::VpnApp;
use crate::ui::theme;
use egui::{Rounding, Stroke};

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

            // User info
            if let Some(session) = app.get_session() {
                ui.label(
                    egui::RichText::new(&session.user().email)
                        .size(12.0)
                        .color(theme::TEXT_SECONDARY),
                );
            }
        });
    });

    ui.add_space(8.0);

    // ── Error banner ───────────────────────────────────────────────────────
    if let Some(error) = app.get_error() {
        let error = error.to_string();
        egui::Frame::none()
            .fill(theme::ERROR_DIM)
            .rounding(Rounding::same(8.0))
            .inner_margin(egui::Margin::same(10.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&error).size(12.0).color(theme::ERROR));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .add(
                                egui::Label::new(
                                    egui::RichText::new("X").size(12.0).color(theme::ERROR),
                                )
                                .sense(egui::Sense::click()),
                            )
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
        .map(|s| {
            (
                s.id,
                s.country.clone(),
                s.name.clone(),
                s.ip.clone(),
                s.listen_port,
                s.is_active,
            )
        })
        .collect();
    let selected = app.get_selected_server();

    let bottom_space = 70.0;
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
                });
            } else {
                for (idx, (_id, country, name, ip, port, is_active)) in servers.iter().enumerate() {
                    let is_selected = selected == Some(idx);
                    server_card(
                        ui,
                        idx,
                        country,
                        name,
                        ip,
                        *port,
                        *is_active,
                        is_selected,
                        app,
                    );
                    ui.add_space(4.0);
                }
            }
        });

    // ── Bottom connect button ──────────────────────────────────────────────
    ui.add_space(8.0);
    let can_connect = app.get_selected_server().is_some() && !app.is_connecting();
    let btn_text = if app.is_connecting() {
        "Connexion en cours..."
    } else {
        "Se connecter"
    };
    if theme::primary_button(ui, btn_text, can_connect) {
        app.handle_connect();
    }
}

fn server_card(
    ui: &mut egui::Ui,
    idx: usize,
    country: &str,
    name: &str,
    ip: &str,
    port: u16,
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
        .rounding(Rounding::same(10.0))
        .stroke(border)
        .inner_margin(egui::Margin::same(14.0))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());

            ui.horizontal(|ui| {
                // Country badge
                let flag = theme::country_flag(country);
                ui.label(
                    egui::RichText::new(flag)
                        .size(13.0)
                        .color(theme::TEXT_MUTED)
                        .family(egui::FontFamily::Monospace),
                );

                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(name)
                            .size(14.0)
                            .color(theme::TEXT_PRIMARY)
                            .strong(),
                    );
                    ui.label(
                        egui::RichText::new(format!("{} - {}:{}", country, ip, port))
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
