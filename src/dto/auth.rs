use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct AccessTokenRequest<'a> {
    pub client_id: &'a str,
    pub client_secret: &'a str,
}

impl std::fmt::Debug for AccessTokenRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessTokenRequest")
            .field("client_id", &self.client_id)
            .field("client_secret", &"[REDACTED]")
            .finish()
    }
}

#[derive(Clone, Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
}

impl std::fmt::Debug for AccessTokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessTokenResponse")
            .field("access_token", &self.access_token)
            .field("expires_in", &self.expires_in)
            .field("token_type", &self.token_type)
            .finish()
    }
}
