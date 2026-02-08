mod app;
mod ui;
mod vpn;

use app::VpnApp;

fn main() -> eframe::Result<()> {
    // Force X11 on WSL2 — Wayland via WSLg often causes "Broken pipe" errors
    if std::path::Path::new("/proc/sys/fs/binfmt_misc/WSLInterop").exists()
        || std::env::var("WSL_DISTRO_NAME").is_ok()
    {
        std::env::set_var("WAYLAND_DISPLAY", "");
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 600.0])
            .with_min_inner_size([350.0, 500.0])
            .with_icon(load_icon()),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "FIRE VPN",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(VpnApp::new(cc)))
        }),
    )
}

fn load_icon() -> egui::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let size = 32;
        let mut rgba = vec![0u8; size * size * 4];
        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - size as f32 / 2.0;
                let dy = y as f32 - size as f32 / 2.0;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < size as f32 / 2.0 {
                    let idx = (y * size + x) * 4;
                    rgba[idx] = 30;
                    rgba[idx + 1] = 144;
                    rgba[idx + 2] = 255;
                    rgba[idx + 3] = 255;
                }
            }
        }
        (rgba, size as u32, size as u32)
    };

    egui::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
