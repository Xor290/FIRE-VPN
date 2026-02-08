use crate::app::VpnApp;
use crate::ui::theme;

pub fn render(ui: &mut egui::Ui, app: &mut VpnApp) {
    let panel_width = ui.available_width();

    ui.vertical_centered(|ui| {
        ui.add_space(50.0);

        // Logo / Shield icon
        ui.label(
            egui::RichText::new("FIRE VPN")
                .size(28.0)
                .color(theme::ACCENT)
                .strong(),
        );
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("Connexion securisee")
                .size(12.0)
                .color(theme::TEXT_MUTED),
        );

        ui.add_space(30.0);

        // Card container
        let card_width = (panel_width - 60.0).min(320.0);
        ui.allocate_ui(egui::Vec2::new(card_width, 0.0), |ui| {
            theme::card_frame(ui, |ui| {
                ui.set_min_width(ui.available_width());

                // Error message
                if let Some(error) = app.get_error() {
                    let error = error.to_string();
                    egui::Frame::none()
                        .fill(theme::ERROR_DIM)
                        .rounding(egui::Rounding::same(8.0))
                        .inner_margin(egui::Margin::same(12.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(&error).size(12.0).color(theme::ERROR),
                                );
                            });
                        });
                    ui.add_space(12.0);
                }

                // Form fields
                if app.is_show_register() {
                    theme::text_field(ui, "NOM D'UTILISATEUR", app.get_username());
                    ui.add_space(8.0);
                }

                theme::text_field(ui, "EMAIL", app.get_email());
                ui.add_space(8.0);

                let pw_response = theme::password_field(ui, "MOT DE PASSE", app.get_password());
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
            });
        });

        ui.add_space(16.0);

        // Toggle register/login
        let toggle_text = if app.is_show_register() {
            "Deja un compte ? Se connecter"
        } else {
            "Pas de compte ? S'inscrire"
        };

        if ui
            .add(
                egui::Label::new(
                    egui::RichText::new(toggle_text)
                        .size(12.0)
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
}
