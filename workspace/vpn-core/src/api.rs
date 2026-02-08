use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("API error: {0}")]
    Api(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub id: u64,
    pub name: String,
    pub country: String,
    pub ip: String,
    pub public_key: String,
    pub listen_port: u16,
    pub subnet: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub peer_ip: String,
    pub config: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerStatus {
    pub id: u64,
    pub user_id: u64,
    pub server_id: u64,
    pub public_key: String,
    pub allowed_ip: String,
    pub server: Server,
}

#[derive(Deserialize)]
struct ApiSuccess<T> {
    data: T,
}

#[derive(Deserialize)]
struct ApiErrorResp {
    error: String,
}

pub struct ApiClient {
    base_url: String,
    token: String,
    client: reqwest::blocking::Client,
}

impl ApiClient {
    pub fn new(base_url: &str, token: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            token: token.to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn set_token(&mut self, token: &str) {
        self.token = token.to_string();
    }

    pub fn list_servers(&self) -> Result<Vec<Server>, ApiError> {
        let resp = self
            .client
            .get(format!("{}/vpn/servers", self.base_url))
            .bearer_auth(&self.token)
            .send()?;

        if !resp.status().is_success() {
            return Err(self.parse_error(resp));
        }

        let success: ApiSuccess<Vec<Server>> = resp.json()?;
        Ok(success.data)
    }

    pub fn connect(&self, server_id: u64) -> Result<ConnectionInfo, ApiError> {
        let resp = self
            .client
            .post(format!("{}/vpn/connect", self.base_url))
            .bearer_auth(&self.token)
            .json(&serde_json::json!({ "server_id": server_id }))
            .send()?;

        if !resp.status().is_success() {
            return Err(self.parse_error(resp));
        }

        let success: ApiSuccess<ConnectionInfo> = resp.json()?;
        Ok(success.data)
    }

    pub fn disconnect(&self, server_id: u64) -> Result<(), ApiError> {
        let resp = self
            .client
            .post(format!("{}/vpn/disconnect", self.base_url))
            .bearer_auth(&self.token)
            .json(&serde_json::json!({ "server_id": server_id }))
            .send()?;

        if !resp.status().is_success() {
            return Err(self.parse_error(resp));
        }

        Ok(())
    }

    pub fn status(&self) -> Result<Vec<PeerStatus>, ApiError> {
        let resp = self
            .client
            .get(format!("{}/vpn/status", self.base_url))
            .bearer_auth(&self.token)
            .send()?;

        if !resp.status().is_success() {
            return Err(self.parse_error(resp));
        }

        let success: ApiSuccess<Vec<PeerStatus>> = resp.json()?;
        Ok(success.data)
    }

    fn parse_error(&self, resp: reqwest::blocking::Response) -> ApiError {
        match resp.json::<ApiErrorResp>() {
            Ok(e) => ApiError::Api(e.error),
            Err(_) => ApiError::Api("unknown error".into()),
        }
    }
}
