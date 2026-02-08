use super::*;

pub fn register(
    base_url: &str,
    username: &str,
    email: &str,
    password: &str,
) -> Result<AuthResponse, AuthError> {
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(format!("{}/auth/register", base_url))
        .json(&serde_json::json!({
            "username": username,
            "email": email,
            "password": password,
        }))
        .send()?;

    if !resp.status().is_success() {
        let err: ApiError = resp.json().unwrap_or(ApiError {
            error: "unknown error".into(),
        });
        return Err(AuthError::Api(err.error));
    }

    let success: ApiSuccess = resp.json()?;
    Ok(AuthResponse {
        token: success.data.token,
        user: success.data.user,
    })
}

pub fn login(base_url: &str, email: &str, password: &str) -> Result<AuthResponse, AuthError> {
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(format!("{}/auth/login", base_url))
        .json(&serde_json::json!({
            "email": email,
            "password": password,
        }))
        .send()?;

    if !resp.status().is_success() {
        let err: ApiError = resp.json().unwrap_or(ApiError {
            error: "unknown error".into(),
        });
        return Err(AuthError::Api(err.error));
    }

    let success: ApiSuccess = resp.json()?;
    Ok(AuthResponse {
        token: success.data.token,
        user: success.data.user,
    })
}
