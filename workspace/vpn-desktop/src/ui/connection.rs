use crate::app::VpnApp;
use crate::ui::continents;
use crate::ui::theme;
use egui::{Rounding, Stroke, Vec2};

// ── Globe data: lat/lon of major cities (in radians) ───────────────────────────
// Each point: (latitude_rad, longitude_rad)
const GLOBE_POINTS: [(f32, f32); 30] = [
    (0.8527, 0.0407),   // Paris
    (0.9075, -0.0014),  // London
    (0.7117, -1.2915),  // New York
    (0.6228, -1.5436),  // Miami
    (0.5934, -0.8430),  // Mexico City
    (-0.4014, -0.8010), // São Paulo
    (-0.5995, -0.6064), // Buenos Aires
    (0.9598, 0.2285),   // Berlin
    (0.7298, 0.2120),   // Rome
    (0.6980, 0.6458),   // Istanbul
    (0.9700, 0.4363),   // Moscow
    (0.4887, 0.9605),   // Dubai
    (0.3316, 1.3439),   // Mumbai
    (0.5583, 1.7994),   // Beijing
    (0.6232, 2.4390),   // Tokyo
    (0.0233, 1.8145),   // Singapore
    (-0.5881, 2.6494),  // Sydney
    (-0.7165, 3.0490),  // Auckland
    (0.8937, -2.0363),  // Vancouver
    (0.7602, -1.5255),  // Chicago
    (0.5920, -2.0580),  // Los Angeles
    (0.8227, -1.0697),  // Reykjavik
    (0.7854, 0.1571),   // Amsterdam
    (1.0531, 0.4363),   // Stockholm
    (0.1745, 0.6458),   // Nairobi
    (-0.4654, 0.4887),  // Johannesburg
    (0.5236, 0.7679),   // Cairo
    (0.2443, 1.7627),   // Bangkok
    (0.0593, 1.8064),   // Jakarta
    (0.8168, -1.8151),  // Denver
];

// Connections between cities (index pairs) forming a network mesh
const GLOBE_EDGES: [(usize, usize); 36] = [
    (0, 1),   // Paris - London
    (0, 8),   // Paris - Rome
    (0, 7),   // Paris - Berlin
    (1, 22),  // London - Amsterdam
    (1, 21),  // London - Reykjavik
    (2, 3),   // New York - Miami
    (2, 19),  // New York - Chicago
    (2, 1),   // New York - London
    (3, 4),   // Miami - Mexico City
    (4, 5),   // Mexico City - São Paulo
    (5, 6),   // São Paulo - Buenos Aires
    (7, 23),  // Berlin - Stockholm
    (7, 10),  // Berlin - Moscow
    (8, 9),   // Rome - Istanbul
    (9, 10),  // Istanbul - Moscow
    (9, 26),  // Istanbul - Cairo
    (10, 23), // Moscow - Stockholm
    (11, 9),  // Dubai - Istanbul
    (11, 12), // Dubai - Mumbai
    (11, 26), // Dubai - Cairo
    (12, 27), // Mumbai - Bangkok
    (13, 14), // Beijing - Tokyo
    (13, 27), // Beijing - Bangkok
    (14, 15), // Tokyo - Singapore
    (15, 28), // Singapore - Jakarta
    (15, 27), // Singapore - Bangkok
    (16, 17), // Sydney - Auckland
    (16, 28), // Sydney - Jakarta
    (18, 20), // Vancouver - Los Angeles
    (18, 19), // Vancouver - Chicago
    (19, 29), // Chicago - Denver
    (20, 29), // Los Angeles - Denver
    (21, 18), // Reykjavik - Vancouver
    (22, 23), // Amsterdam - Stockholm
    (24, 26), // Nairobi - Cairo
    (24, 25), // Nairobi - Johannesburg
];

/// Project a 3D point on a sphere to 2D screen coordinates.
/// Returns (x, y, z) where z is depth (for visibility culling).
fn project_sphere(
    lat: f32,
    lon: f32,
    rotation: f32,
    center_x: f32,
    center_y: f32,
    radius: f32,
) -> (f32, f32, f32) {
    let adjusted_lon = lon + rotation;
    let x = lat.cos() * adjusted_lon.sin();
    let y = lat.sin();
    let z = lat.cos() * adjusted_lon.cos();

    // Apply slight tilt for more interesting view (20 degrees)
    let tilt: f32 = 0.35;
    let y_tilted = y * tilt.cos() - z * tilt.sin();
    let z_tilted = y * tilt.sin() + z * tilt.cos();

    let screen_x = center_x + x * radius;
    let screen_y = center_y - y_tilted * radius;
    (screen_x, screen_y, z_tilted)
}

pub fn render(ui: &mut egui::Ui, app: &mut VpnApp) {
    theme::draw_top_accent(ui);

    // ── Header bar ─────────────────────────────────────────────────────────
    ui.horizontal(|ui| {
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("SilentGhostVPN")
                .size(16.0)
                .color(theme::ACCENT)
                .strong(),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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
            if theme::small_button(ui, "Profil", theme::ACCENT, theme::TEXT_PRIMARY) {
                app.show_profile();
            }
        });
    });

    ui.add_space(8.0);

    // ── Holographic Earth Globe ────────────────────────────────────────────
    ui.vertical_centered(|ui| {
        let time = ui.input(|i| i.time) as f32;
        let globe_size = 200.0;
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(globe_size), egui::Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let center = rect.center();
            let radius = globe_size * 0.42;

            // Rotation speed
            let rotation = time * 0.3;

            // ── Outer holographic glow rings ───────────────────────────
            for i in 0..3 {
                let pulse = ((time * 0.8 + i as f32 * 0.7).sin() + 1.0) / 2.0;
                let r = radius + 8.0 + i as f32 * 6.0 + pulse * 3.0;
                let alpha = (15.0 + pulse * 20.0) as u8;
                painter.circle_stroke(
                    center,
                    r,
                    Stroke::new(
                        0.5,
                        egui::Color32::from_rgba_unmultiplied(0, 200, 255, alpha),
                    ),
                );
            }

            // ── Globe outline (holographic cyan) ───────────────────────
            let outline_pulse = ((time * 1.2).sin() + 1.0) / 2.0;
            let outline_alpha = (40.0 + outline_pulse * 30.0) as u8;
            painter.circle_stroke(
                center,
                radius,
                Stroke::new(
                    1.5,
                    egui::Color32::from_rgba_unmultiplied(0, 220, 255, outline_alpha),
                ),
            );

            // ── Globe fill (very subtle dark glow) ─────────────────────
            let fill_alpha = (8.0 + outline_pulse * 6.0) as u8;
            painter.circle_filled(
                center,
                radius - 1.0,
                egui::Color32::from_rgba_unmultiplied(0, 40, 60, fill_alpha),
            );

            // ── Continent outlines (holographic landmasses) ────────────
            let continents = continents::continent_outlines();
            for polygon in &continents {
                if polygon.len() < 2 {
                    continue;
                }
                // Project all polygon vertices
                let projected_poly: Vec<(f32, f32, f32)> = polygon
                    .iter()
                    .map(|&(lat, lon)| {
                        project_sphere(lat, lon, rotation, center.x, center.y, radius)
                    })
                    .collect();

                // Draw filled continent shape using triangles (fan from centroid)
                // Only fill visible front-facing segments
                {
                    // Collect visible vertices for fill
                    let visible: Vec<egui::Pos2> = projected_poly
                        .iter()
                        .filter(|&&(_, _, z)| z > -0.05)
                        .map(|&(x, y, _)| egui::Pos2::new(x, y))
                        .collect();

                    if visible.len() >= 3 {
                        // Compute average depth for alpha
                        let avg_z: f32 = projected_poly
                            .iter()
                            .filter(|&&(_, _, z)| z > -0.05)
                            .map(|&(_, _, z)| z)
                            .sum::<f32>()
                            / visible.len() as f32;
                        let depth_factor = ((avg_z + 0.05) * 1.1).min(1.0).max(0.0);
                        let fill_a = (depth_factor * (18.0 + outline_pulse * 12.0)).min(40.0) as u8;

                        // Fill the continent shape
                        let mesh_color = egui::Color32::from_rgba_unmultiplied(0, 120, 80, fill_a);
                        let mut mesh = egui::Mesh::default();
                        let base_idx = mesh.vertices.len() as u32;
                        for &pos in &visible {
                            mesh.vertices.push(egui::epaint::Vertex {
                                pos,
                                uv: egui::epaint::WHITE_UV,
                                color: mesh_color,
                            });
                        }
                        // Fan triangulation from first vertex
                        for i in 1..(visible.len() as u32 - 1) {
                            mesh.indices.push(base_idx);
                            mesh.indices.push(base_idx + i);
                            mesh.indices.push(base_idx + i + 1);
                        }
                        painter.add(egui::Shape::mesh(mesh));
                    }
                }

                // Draw outline edges
                for i in 0..projected_poly.len() {
                    let j = (i + 1) % projected_poly.len();
                    let (ax, ay, az) = projected_poly[i];
                    let (bx, by, bz) = projected_poly[j];

                    if az > -0.05 && bz > -0.05 {
                        let min_depth = az.min(bz);
                        let depth_factor = ((min_depth + 0.05) * 1.1).min(1.0).max(0.0);
                        let alpha = (depth_factor * (35.0 + outline_pulse * 30.0)).min(80.0) as u8;

                        if alpha > 2 {
                            painter.line_segment(
                                [egui::Pos2::new(ax, ay), egui::Pos2::new(bx, by)],
                                Stroke::new(
                                    0.8,
                                    egui::Color32::from_rgba_unmultiplied(0, 220, 160, alpha),
                                ),
                            );
                        }
                    }
                }
            }

            // ── Latitude lines (holographic grid) ──────────────────────
            let lat_lines = [-60.0_f32, -30.0, 0.0, 30.0, 60.0];
            for &lat_deg in &lat_lines {
                let lat = lat_deg.to_radians();
                let line_r = radius * lat.cos();
                let line_y = center.y - radius * lat.sin() * 0.35_f32.cos();

                if line_r > 2.0 {
                    let alpha = (12.0 + outline_pulse * 8.0) as u8;
                    painter.circle_stroke(
                        egui::Pos2::new(center.x, line_y),
                        line_r,
                        Stroke::new(
                            0.4,
                            egui::Color32::from_rgba_unmultiplied(0, 180, 220, alpha),
                        ),
                    );
                }
            }

            // ── Longitude lines (meridians) ────────────────────────────
            for i in 0..12 {
                let lon_offset = (i as f32 / 12.0) * std::f32::consts::TAU + rotation;
                let segments = 40;
                let mut prev: Option<(f32, f32, f32)> = None;
                for s in 0..=segments {
                    let lat = -std::f32::consts::FRAC_PI_2
                        + (s as f32 / segments as f32) * std::f32::consts::PI;
                    let (sx, sy, sz) =
                        project_sphere(lat, lon_offset, 0.0, center.x, center.y, radius);
                    if let Some((px, py, _pz)) = prev {
                        if sz > -0.1 {
                            let depth_alpha = ((sz + 0.1) * 0.9).min(1.0);
                            let alpha = (depth_alpha * 15.0) as u8;
                            if alpha > 1 {
                                painter.line_segment(
                                    [egui::Pos2::new(px, py), egui::Pos2::new(sx, sy)],
                                    Stroke::new(
                                        0.3,
                                        egui::Color32::from_rgba_unmultiplied(0, 180, 220, alpha),
                                    ),
                                );
                            }
                        }
                    }
                    prev = Some((sx, sy, sz));
                }
            }

            // ── Project all city points ────────────────────────────────
            let projected: Vec<(f32, f32, f32)> = GLOBE_POINTS
                .iter()
                .map(|&(lat, lon)| project_sphere(lat, lon, rotation, center.x, center.y, radius))
                .collect();

            // ── Draw network edges (connections between cities) ────────
            for &(a, b) in &GLOBE_EDGES {
                let (ax, ay, az) = projected[a];
                let (bx, by, bz) = projected[b];

                // Only draw if both points are on the visible hemisphere
                if az > -0.15 && bz > -0.15 {
                    let min_depth = az.min(bz);
                    let depth_factor = ((min_depth + 0.15) * 1.2).min(1.0).max(0.0);

                    // Pulsing data flow effect along edges
                    let flow = ((time * 2.0 + (a + b) as f32 * 0.5).sin() + 1.0) / 2.0;
                    let alpha = (depth_factor * (25.0 + flow * 40.0)) as u8;

                    if alpha > 2 {
                        painter.line_segment(
                            [egui::Pos2::new(ax, ay), egui::Pos2::new(bx, by)],
                            Stroke::new(
                                0.6 + flow * 0.4,
                                egui::Color32::from_rgba_unmultiplied(0, 220, 255, alpha),
                            ),
                        );

                        // Traveling pulse dot along the edge
                        let pulse_t = ((time * 1.5 + a as f32 * 0.3) % 2.0) / 2.0;
                        let px = ax + (bx - ax) * pulse_t;
                        let py = ay + (by - ay) * pulse_t;
                        let dot_alpha =
                            (depth_factor * 80.0 * (1.0 - (pulse_t - 0.5).abs() * 2.0).max(0.0))
                                as u8;
                        if dot_alpha > 5 {
                            painter.circle_filled(
                                egui::Pos2::new(px, py),
                                1.2,
                                egui::Color32::from_rgba_unmultiplied(100, 240, 255, dot_alpha),
                            );
                        }
                    }
                }
            }

            // ── Draw city nodes ────────────────────────────────────────
            for (i, &(sx, sy, sz)) in projected.iter().enumerate() {
                if sz > -0.1 {
                    let depth_factor = ((sz + 0.1) * 1.2).min(1.0).max(0.0);

                    // Outer glow
                    let glow_pulse = ((time * 1.5 + i as f32 * 0.7).sin() + 1.0) / 2.0;
                    let glow_alpha = (depth_factor * (20.0 + glow_pulse * 30.0)) as u8;
                    let glow_r = 3.5 + glow_pulse * 2.0;
                    painter.circle_filled(
                        egui::Pos2::new(sx, sy),
                        glow_r,
                        egui::Color32::from_rgba_unmultiplied(0, 180, 255, glow_alpha),
                    );

                    // Core dot
                    let core_alpha = (depth_factor * (140.0 + glow_pulse * 80.0)).min(255.0) as u8;
                    let core_r = 1.5 + depth_factor * 0.8;
                    painter.circle_filled(
                        egui::Pos2::new(sx, sy),
                        core_r,
                        egui::Color32::from_rgba_unmultiplied(150, 240, 255, core_alpha),
                    );
                }
            }

            // ── Scanline effect (holographic sweep) ────────────────────
            let scan_angle = (time * 0.6) % std::f32::consts::TAU;
            let scan_x = center.x + scan_angle.cos() * radius;
            let scan_y = center.y + scan_angle.sin() * radius;
            let scan_alpha = 20u8;
            painter.line_segment(
                [center, egui::Pos2::new(scan_x, scan_y)],
                Stroke::new(
                    0.5,
                    egui::Color32::from_rgba_unmultiplied(0, 255, 200, scan_alpha),
                ),
            );

            // Sweep arc glow
            let sweep_segments = 20;
            for s in 0..sweep_segments {
                let t = s as f32 / sweep_segments as f32;
                let a = scan_angle - t * 0.4;
                let alpha = ((1.0 - t) * 12.0) as u8;
                let sx2 = center.x + a.cos() * radius;
                let sy2 = center.y + a.sin() * radius;
                painter.circle_filled(
                    egui::Pos2::new(sx2, sy2),
                    1.0,
                    egui::Color32::from_rgba_unmultiplied(0, 255, 200, alpha),
                );
            }
        }

        ui.ctx().request_repaint();

        ui.add_space(10.0);

        // ── Status text with holographic glow ──────────────────────────
        let text_glow = ((time * 1.6).sin() + 1.0) / 2.0;
        let g = (200.0 + text_glow * 55.0).min(255.0) as u8;
        let b = (240.0 + text_glow * 15.0).min(255.0) as u8;
        ui.label(
            egui::RichText::new("CONNECTE")
                .size(18.0)
                .color(egui::Color32::from_rgb(0, g, b))
                .strong(),
        );

        // Show connected server name with flag
        if let Some(idx) = app.get_selected_server() {
            let servers = app.get_servers();
            if let Some(server) = servers.get(idx) {
                ui.add_space(2.0);
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

    ui.add_space(12.0);

    // ── Connection details card (holographic border) ───────────────────────
    if let Some(session) = app.get_session() {
        if let Some(config) = session.current_config() {
            let address = config.address.clone();
            let dns = config.dns.clone();

            let time = ui.input(|i| i.time) as f32;
            let border_glow = ((time * 1.2).sin() + 1.0) / 2.0;
            let border_alpha = (25.0 + border_glow * 45.0) as u8;

            egui::Frame::none()
                .fill(egui::Color32::from_rgba_unmultiplied(0, 15, 25, 180))
                .rounding(Rounding::same(12.0))
                .stroke(Stroke::new(
                    1.0,
                    egui::Color32::from_rgba_unmultiplied(0, 200, 255, border_alpha),
                ))
                .inner_margin(egui::Margin::same(16.0))
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());

                    ui.label(
                        egui::RichText::new("DETAILS DE CONNEXION")
                            .size(11.0)
                            .color(egui::Color32::from_rgb(0, 180, 220))
                            .strong(),
                    );
                    ui.add_space(10.0);

                    let public_ip = app.get_public_ip();
                    theme::info_row(ui, "IP publique", public_ip.as_deref().unwrap_or("..."));
                    theme::info_row(ui, "IP locale", &address);
                    theme::info_row(ui, "DNS", &dns);
                });
        }
    }

    ui.add_space(12.0);

    // ── Server switch section ──────────────────────────────────────────────
    theme::section_heading(ui, "SERVEURS");

    let servers: Vec<_> = app
        .get_servers()
        .iter()
        .map(|s| (s.id, s.country.clone(), s.name.clone()))
        .collect();
    let selected = app.get_selected_server();

    let button_area_height = 64.0;
    let scroll_height = (ui.available_height() - button_area_height).max(60.0);

    egui::ScrollArea::vertical()
        .max_height(scroll_height)
        .show(ui, |ui| {
            for (idx, (_id, country, name)) in servers.iter().enumerate() {
                let is_current = selected == Some(idx);

                let fill = if is_current {
                    egui::Color32::from_rgba_unmultiplied(0, 30, 50, 180)
                } else {
                    theme::BG_CARD
                };
                let border = if is_current {
                    Stroke::new(1.0, egui::Color32::from_rgb(0, 200, 255))
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
                                    .color(egui::Color32::from_rgb(0, 220, 255))
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
                                        theme::status_pill(
                                            ui,
                                            "ACTIF",
                                            egui::Color32::from_rgb(0, 220, 255),
                                        );
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

    // ── Footer ─────────────────────────────────────────────────────────────
    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
        ui.add_space(4.0);
        if theme::danger_button(ui, "Se deconnecter") {
            app.handle_disconnect();
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
