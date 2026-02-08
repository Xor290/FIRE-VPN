mod client;

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
pub(crate) struct ApiSuccess<T> {
    pub data: T,
}

#[derive(Deserialize)]
pub(crate) struct ApiErrorResp {
    pub error: String,
}

pub struct ApiClient {
    pub(crate) base_url: String,
    pub(crate) token: String,
    pub(crate) client: reqwest::blocking::Client,
}
