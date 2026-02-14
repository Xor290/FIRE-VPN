use super::{
    ApiClient, ApiError, ApiErrorResp, ApiSuccess, ConnectionInfo, PeerStatus, ProfileUpdateResp,
    Server,
};

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

    pub fn update_profile(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<crate::auth::UserInfo, ApiError> {
        let resp = self
            .client
            .put(format!("{}/profile/update", self.base_url))
            .bearer_auth(&self.token)
            .json(&serde_json::json!({
                "username": username,
                "email": email,
                "password": password,
            }))
            .send()?;

        if !resp.status().is_success() {
            return Err(self.parse_error(resp));
        }

        let body: ProfileUpdateResp = resp.json()?;
        Ok(body.user)
    }

    pub fn delete_account(&self) -> Result<(), ApiError> {
        let resp = self
            .client
            .delete(format!("{}/profile/delete", self.base_url))
            .bearer_auth(&self.token)
            .send()?;

        if !resp.status().is_success() {
            return Err(self.parse_error(resp));
        }

        Ok(())
    }

    fn parse_error(&self, resp: reqwest::blocking::Response) -> ApiError {
        match resp.json::<ApiErrorResp>() {
            Ok(e) => ApiError::Api(e.error),
            Err(_) => ApiError::Api("unknown error".into()),
        }
    }
}
