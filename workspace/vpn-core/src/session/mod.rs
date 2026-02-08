use thiserror::Error;

pub mod manager;
use crate::api::{ApiClient, ApiError, Server};
use crate::auth::{AuthError, UserInfo};
use crate::wireguard::{WireGuardConfig, WireGuardError};

pub struct Session {
    token: String,
    user: UserInfo,
    client: ApiClient,
    current_server: Option<Server>,
    config: Option<WireGuardConfig>,
}

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
