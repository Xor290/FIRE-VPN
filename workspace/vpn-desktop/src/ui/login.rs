use crate::app::VpnApp;
use crate::ui::theme;

pub fn render(ui: &mut egui::Ui, app: &mut VpnApp) {
    let panel_width = ui.available_width();

    ui.vertical_centered(|ui| {
        ui.add_space(50.0);

        // Centered accent bar (like mobile)
        theme::draw_centered_accent_bar(ui, 60.0);

        ui.add_space(24.0);

        // Title with letter spacing
        ui.label(
            egui::RichText::new("SilentGhostVPN")
                .size(28.0)
                .color(theme::ACCENT)
                .strong(),
        );
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("CONNEXION SECURISEE")
                .size(11.0)
                .color(theme::TEXT_MUTED),
        );

        ui.add_space(32.0);

        // Card container
        let card_width = (panel_width - 40.0).min(320.0);
        ui.allocate_ui(egui::Vec2::new(card_width, 0.0), |ui| {
            theme::card_frame(ui, |ui| {
                ui.set_min_width(ui.available_width());

                // Error message
                if let Some(error) = app.get_error() {
                    let error = error.to_string();
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgba_premultiplied(248, 81, 73, 15))
                        .rounding(egui::Rounding::same(6.0))
                        .stroke(egui::Stroke::new(
                            1.0,
                            egui::Color32::from_rgba_premultiplied(248, 81, 73, 40),
                        ))
                        .inner_margin(egui::Margin::same(12.0))
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new(&error).size(13.0).color(theme::ERROR));
                        });
                    ui.add_space(16.0);
                }

                // Form fields
                if app.is_show_register() {
                    theme::text_field(ui, "Nom d'utilisateur", app.get_username());
                    ui.add_space(16.0);
                }

                theme::text_field(ui, "Email", app.get_email());
                ui.add_space(16.0);

                let pw_response = theme::password_field(ui, "Mot de passe", app.get_password());
                ui.add_space(20.0);

                // Submit button
                let button_text = if app.is_show_register() {
                    "Creer un compte"
                } else {
                    "Se connecter"
                };

                let clicked = theme::primary_button(ui, button_text, true);
                let enter_pressed =
                    pw_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                if clicked || enter_pressed {
                    if app.is_show_register() {
                        app.handle_register();
                    } else {
                        app.handle_login();
                    }
                }

                ui.add_space(16.0);

                // Toggle register/login (centered)
                ui.vertical_centered(|ui| {
                    let toggle_text = if app.is_show_register() {
                        "Deja un compte ? Se connecter"
                    } else {
                        "Pas de compte ? S'inscrire"
                    };

                    if ui
                        .add(
                            egui::Label::new(
                                egui::RichText::new(toggle_text)
                                    .size(13.0)
                                    .color(theme::ACCENT),
                            )
                            .sense(egui::Sense::click()),
                        )
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        app.toggle_register();
                    }
                });
            });
        });
    });
}
