use crate::ui::flags::FlagStore;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use vpn_core::api::Server;
use vpn_core::session::Session;

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Login,
    ServerList,
    Connected,
}

#[derive(Default, Serialize, Deserialize)]
struct AppConfig {
    api_url: String,
    saved_token: Option<String>,
    saved_email: Option<String>,
    last_server_id: Option<u64>,
}

pub struct VpnApp {
    state: AppState,
    config: AppConfig,
    config_path: PathBuf,
    session: Option<Session>,
    email: String,
    password: String,
    username: String,
    show_register: bool,
    error_message: Option<String>,
    servers: Vec<Server>,
    selected_server: Option<usize>,
    is_connecting: bool,
    connection_status: String,
    showing_profile: bool,
    pub flag_store: FlagStore,
    // Profile editing
    profile_editing: bool,
    profile_username: String,
    profile_email: String,
    profile_password: String,
    profile_error: Option<String>,
    profile_success: Option<String>,
}

impl VpnApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure dark visuals matching mobile palette
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(0, 0, 0);
        visuals.window_fill = egui::Color32::from_rgb(0, 0, 0);
        visuals.extreme_bg_color = egui::Color32::from_rgb(0, 0, 0);
        visuals.faint_bg_color = egui::Color32::from_rgb(26, 26, 26);
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(0, 0, 0);
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(26, 26, 26);
        visuals.widgets.inactive.bg_stroke =
            egui::Stroke::new(1.0, egui::Color32::from_rgb(48, 60, 61));
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(26, 26, 26);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(26, 26, 26);
        visuals.selection.bg_fill = egui::Color32::from_rgb(75, 107, 251);
        cc.egui_ctx.set_visuals(visuals);

        let mut flag_store = FlagStore::new();
        flag_store.load(&cc.egui_ctx);

        let config_path = Self::get_config_path();
        let config = Self::load_config(&config_path);

        Self {
            state: AppState::Login,
            config_path,
            session: None,
            email: config.saved_email.clone().unwrap_or_default(),
            password: String::new(),
            username: String::new(),
            show_register: false,
            error_message: None,
            servers: Vec::new(),
            selected_server: None,
            is_connecting: false,
            connection_status: "Déconnecté".to_string(),
            showing_profile: false,
            flag_store,
            profile_editing: false,
            profile_username: String::new(),
            profile_email: String::new(),
            profile_password: String::new(),
            profile_error: None,
            profile_success: None,
            config,
        }
    }

    fn get_config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("vpn-client");
        std::fs::create_dir_all(&path).ok();
        path.push("config.json");
        path
    }

    fn load_config(path: &PathBuf) -> AppConfig {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_else(|| AppConfig {
                api_url: "http://localhost:8080".to_string(),
                ..Default::default()
            })
    }

    fn save_config(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.config) {
            std::fs::write(&self.config_path, json).ok();
        }
    }

    pub fn handle_login(&mut self) {
        self.error_message = None;

        let email = self.email.clone();
        let password = self.password.clone();
        let api_url = self.config.api_url.clone();

        match Session::login(&api_url, &email, &password) {
            Ok(session) => {
                self.config.saved_email = Some(email);
                self.config.saved_token = Some(session.token().to_string());
                self.save_config();

                self.session = Some(session);
                self.password.clear();
                self.load_servers();
                self.state = AppState::ServerList;
            }
            Err(e) => {
                self.error_message = Some(format!("Erreur de connexion: {}", e));
            }
        }
    }

    pub fn handle_register(&mut self) {
        self.error_message = None;

        let username = self.username.clone();
        let email = self.email.clone();
        let password = self.password.clone();
        let api_url = self.config.api_url.clone();

        match Session::register(&api_url, &username, &email, &password) {
            Ok(session) => {
                self.config.saved_email = Some(email);
                self.config.saved_token = Some(session.token().to_string());
                self.save_config();

                self.session = Some(session);
                self.password.clear();
                self.username.clear();
                self.show_register = false;
                self.load_servers();
                self.state = AppState::ServerList;
            }
            Err(e) => {
                self.error_message = Some(format!("Erreur d'inscription: {}", e));
            }
        }
    }

    fn load_servers(&mut self) {
        if let Some(session) = &self.session {
            match session.list_servers() {
                Ok(servers) => {
                    self.servers = servers;
                    if let Some(last_id) = self.config.last_server_id {
                        self.selected_server = self.servers.iter().position(|s| s.id == last_id);
                    }
                }
                Err(e) => {
                    self.error_message = Some(format!("Erreur de chargement des serveurs: {}", e));
                }
            }
        }
    }

    pub fn handle_connect(&mut self) {
        let idx = match self.selected_server {
            Some(i) => i,
            None => return,
        };

        let server_id = self.servers[idx].id;
        let server_name = self.servers[idx].name.clone();

        self.is_connecting = true;
        self.connection_status = format!("Connexion à {}...", server_name);

        let wg_config = match &mut self.session {
            Some(s) => match s.connect(server_id) {
                Ok(cfg) => cfg.clone(),
                Err(e) => {
                    self.error_message = Some(format!("Erreur de connexion: {}", e));
                    self.is_connecting = false;
                    self.connection_status = "Erreur".to_string();
                    return;
                }
            },
            None => return,
        };

        self.config.last_server_id = Some(server_id);
        self.save_config();

        match crate::vpn::tunnel::apply_config(&wg_config) {
            Ok(_) => {
                self.state = AppState::Connected;
                self.connection_status = format!("Connecté à {}", server_name);
                self.is_connecting = false;
            }
            Err(e) => {
                self.error_message = Some(format!("Erreur tunnel: {}", e));
                self.is_connecting = false;
                self.connection_status = "Erreur de connexion".to_string();
            }
        }
    }

    pub fn handle_disconnect(&mut self) {
        if let Some(session) = &mut self.session {
            match session.disconnect() {
                Ok(_) => {
                    if let Err(e) = crate::vpn::tunnel::stop_tunnel() {
                        eprintln!("Erreur lors de l'arrêt du tunnel: {}", e);
                    }

                    self.state = AppState::ServerList;
                    self.connection_status = "Déconnecté".to_string();
                }
                Err(e) => {
                    self.error_message = Some(format!("Erreur de déconnexion: {}", e));
                }
            }
        }
    }

    pub fn handle_switch_server(&mut self, new_idx: usize) {
        let server_id = self.servers[new_idx].id;
        let server_name = self.servers[new_idx].name.clone();

        self.connection_status = format!("Changement vers {}...", server_name);

        let session = match &mut self.session {
            Some(s) => s,
            None => return,
        };

        match session.switch_server(server_id) {
            Ok(wg_config) => {
                let _ = crate::vpn::tunnel::stop_tunnel();

                match crate::vpn::tunnel::apply_config(wg_config) {
                    Ok(_) => {
                        self.config.last_server_id = Some(server_id);
                        self.save_config();
                        self.selected_server = Some(new_idx);
                        self.connection_status = format!("Connecté à {}", server_name);
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Erreur de switch: {}", e));
                    }
                }
            }
            Err(e) => {
                self.error_message = Some(format!("Erreur de switch: {}", e));
            }
        }
    }

    pub fn handle_logout(&mut self) {
        if self.state == AppState::Connected {
            self.handle_disconnect();
        }

        self.session = None;
        self.servers.clear();
        self.selected_server = None;
        self.showing_profile = false;
        self.state = AppState::Login;
        self.email.clear();
        self.password.clear();

        if let Some(saved_email) = &self.config.saved_email {
            self.email = saved_email.clone();
        }
    }

    pub fn handle_delete_account(&mut self) {
        if let Some(session) = &mut self.session {
            if let Err(e) = session.delete_account() {
                self.profile_error = Some(format!("Erreur: {}", e));
                return;
            }
        }
        self.handle_logout();
    }

    pub fn handle_update_profile(&mut self) {
        self.profile_error = None;
        self.profile_success = None;

        let username = self.profile_username.trim().to_string();
        let email = self.profile_email.trim().to_string();
        let password = self.profile_password.clone();

        if username.len() < 3 {
            self.profile_error = Some("Le nom doit contenir au moins 3 caracteres.".into());
            return;
        }
        if !email.contains('@') {
            self.profile_error = Some("Email invalide.".into());
            return;
        }
        if password.len() < 8 {
            self.profile_error =
                Some("Le mot de passe doit contenir au moins 8 caracteres.".into());
            return;
        }

        match &mut self.session {
            Some(session) => match session.update_profile(&username, &email, &password) {
                Ok(()) => {
                    self.profile_password.clear();
                    self.profile_editing = false;
                    self.profile_success = Some("Profil mis a jour.".into());
                }
                Err(e) => {
                    self.profile_error = Some(format!("Erreur: {}", e));
                }
            },
            None => {
                self.profile_error = Some("Session expirée.".into());
            }
        }
    }

    pub fn show_profile(&mut self) {
        self.showing_profile = true;
        self.profile_editing = false;
        self.profile_error = None;
        self.profile_success = None;
    }

    pub fn hide_profile(&mut self) {
        self.showing_profile = false;
        self.profile_editing = false;
        self.profile_error = None;
        self.profile_success = None;
    }

    pub fn start_profile_edit(&mut self) {
        if let Some(session) = &self.session {
            self.profile_username = session.user().username.clone();
            self.profile_email = session.user().email.clone();
        }
        self.profile_password.clear();
        self.profile_error = None;
        self.profile_success = None;
        self.profile_editing = true;
    }

    pub fn cancel_profile_edit(&mut self) {
        self.profile_editing = false;
        self.profile_error = None;
        self.profile_password.clear();
    }

    pub fn is_profile_editing(&self) -> bool {
        self.profile_editing
    }

    pub fn get_profile_username(&mut self) -> &mut String {
        &mut self.profile_username
    }

    pub fn get_profile_email(&mut self) -> &mut String {
        &mut self.profile_email
    }

    pub fn get_profile_password(&mut self) -> &mut String {
        &mut self.profile_password
    }

    pub fn get_profile_error(&self) -> Option<&str> {
        self.profile_error.as_deref()
    }

    pub fn get_profile_success(&self) -> Option<&str> {
        self.profile_success.as_deref()
    }

    pub fn is_vpn_connected(&self) -> bool {
        self.state == AppState::Connected
    }

    pub fn get_email(&mut self) -> &mut String {
        &mut self.email
    }

    pub fn get_password(&mut self) -> &mut String {
        &mut self.password
    }

    pub fn get_username(&mut self) -> &mut String {
        &mut self.username
    }

    pub fn is_show_register(&self) -> bool {
        self.show_register
    }

    pub fn toggle_register(&mut self) {
        self.show_register = !self.show_register;
        self.error_message = None;
    }

    pub fn get_servers(&self) -> &[Server] {
        &self.servers
    }

    pub fn get_selected_server(&self) -> Option<usize> {
        self.selected_server
    }

    pub fn set_selected_server(&mut self, idx: usize) {
        self.selected_server = Some(idx);
    }

    pub fn get_connection_status(&self) -> &str {
        &self.connection_status
    }

    pub fn is_connecting(&self) -> bool {
        self.is_connecting
    }

    pub fn get_error(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    pub fn get_session(&self) -> Option<&Session> {
        self.session.as_ref()
    }
}

impl eframe::App for VpnApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(0, 0, 0))
                    .inner_margin(egui::Margin::symmetric(20.0, 12.0)),
            )
            .show(ctx, |ui| {
                // Profile overlay takes precedence when active
                if self.showing_profile {
                    crate::ui::profile::render(ui, self);
                    return;
                }

                match self.state {
                    AppState::Login => {
                        crate::ui::login::render(ui, self);
                    }
                    AppState::ServerList => {
                        crate::ui::servers::render(ui, self);
                    }
                    AppState::Connected => {
                        crate::ui::connection::render(ui, self);
                    }
                }
            });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if self.state == AppState::Connected {
            if let Some(session) = &mut self.session {
                let _ = session.disconnect();
                let _ = crate::vpn::tunnel::stop_tunnel();
            }
        }
    }
}
