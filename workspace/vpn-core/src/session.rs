use thiserror::Error;

use crate::api::{ApiClient, ApiError, Server};
use crate::auth::{self, AuthError, AuthResponse, UserInfo};
use crate::wireguard::{WireGuardConfig, WireGuardError};

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("auth error: {0}")]
    Auth(#[from] AuthError),
    #[error("api error: {0}")]
    Api(#[from] ApiError),
    #[error("wireguard error: {0}")]
    WireGuard(#[from] WireGuardError),
    #[error("not connected")]
    NotConnected,
}

pub struct Session {
    token: String,
    user: UserInfo,
    client: ApiClient,
    current_server: Option<Server>,
    config: Option<WireGuardConfig>,
}

impl Session {
    /// Crée une session via login.
    pub fn login(base_url: &str, email: &str, password: &str) -> Result<Self, SessionError> {
        let auth_resp = auth::login(base_url, email, password)?;
        Ok(Self::from_auth(base_url, auth_resp))
    }

    /// Crée une session via register.
    pub fn register(
        base_url: &str,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<Self, SessionError> {
        let auth_resp = auth::register(base_url, username, email, password)?;
        Ok(Self::from_auth(base_url, auth_resp))
    }

    fn from_auth(base_url: &str, auth_resp: AuthResponse) -> Self {
        Self {
            token: auth_resp.token.clone(),
            user: auth_resp.user,
            client: ApiClient::new(base_url, &auth_resp.token),
            current_server: None,
            config: None,
        }
    }

    pub fn user(&self) -> &UserInfo {
        &self.user
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn current_server(&self) -> Option<&Server> {
        self.current_server.as_ref()
    }

    pub fn current_config(&self) -> Option<&WireGuardConfig> {
        self.config.as_ref()
    }

    pub fn is_connected(&self) -> bool {
        self.current_server.is_some()
    }

    /// Liste les serveurs VPN disponibles.
    pub fn list_servers(&self) -> Result<Vec<Server>, SessionError> {
        Ok(self.client.list_servers()?)
    }

    /// Connecte au serveur demandé, retourne la config WireGuard parsée.
    /// C'est au code appelant (desktop: wg-quick, mobile: natif) d'appliquer la config.
    pub fn connect(&mut self, server_id: u64) -> Result<&WireGuardConfig, SessionError> {
        let conn = self.client.connect(server_id)?;
        let wg_config = WireGuardConfig::parse(&conn.config)?;

        // Stocker le serveur courant
        let servers = self.client.list_servers()?;
        self.current_server = servers.into_iter().find(|s| s.id == server_id);
        self.config = Some(wg_config);

        Ok(self.config.as_ref().unwrap())
    }

    /// Déconnecte du serveur courant.
    pub fn disconnect(&mut self) -> Result<(), SessionError> {
        let server = self.current_server.as_ref().ok_or(SessionError::NotConnected)?;
        let server_id = server.id;
        self.client.disconnect(server_id)?;
        self.current_server = None;
        self.config = None;
        Ok(())
    }

    /// Switch de serveur : disconnect du courant puis connect au nouveau.
    /// Retourne la nouvelle config WireGuard.
    pub fn switch_server(&mut self, new_server_id: u64) -> Result<&WireGuardConfig, SessionError> {
        if self.is_connected() {
            self.disconnect()?;
        }
        self.connect(new_server_id)
    }
}
