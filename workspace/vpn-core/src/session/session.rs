use crate::api::{ApiClient, Server};
use crate::auth::{self, AuthResponse, UserInfo};
use crate::wireguard::WireGuardConfig;

use super::Session;
use super::SessionError;

impl Session {
    pub fn login(base_url: &str, email: &str, password: &str) -> Result<Self, SessionError> {
        let auth_resp = auth::login(base_url, email, password)?;
        Ok(Self::from_auth(base_url, auth_resp))
    }

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

    pub fn list_servers(&self) -> Result<Vec<Server>, SessionError> {
        Ok(self.client.list_servers()?)
    }

    pub fn connect(&mut self, server_id: u64) -> Result<&WireGuardConfig, SessionError> {
        let conn = self.client.connect(server_id)?;
        let wg_config = WireGuardConfig::parse(&conn.config)?;

        let servers = self.client.list_servers()?;
        self.current_server = servers.into_iter().find(|s| s.id == server_id);
        self.config = Some(wg_config);

        Ok(self.config.as_ref().unwrap())
    }

    pub fn disconnect(&mut self) -> Result<(), SessionError> {
        let server = self
            .current_server
            .as_ref()
            .ok_or(SessionError::NotConnected)?;
        let server_id = server.id;
        self.client.disconnect(server_id)?;
        self.current_server = None;
        self.config = None;
        Ok(())
    }

    pub fn switch_server(&mut self, new_server_id: u64) -> Result<&WireGuardConfig, SessionError> {
        if self.is_connected() {
            self.disconnect()?;
        }
        self.connect(new_server_id)
    }
}
