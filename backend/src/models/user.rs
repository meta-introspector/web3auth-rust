use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub wallet_address: String,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(wallet_address: &str) -> Self {
        let now = Utc::now();
        Self {
            wallet_address: wallet_address.to_lowercase(),
            last_login: Some(now),
            created_at: now,
        }
    }

    pub fn update_login(&mut self) {
        self.last_login = Some(Utc::now());
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub wallet_address: String,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct NonceRequest {
    pub wallet_address: String,
}

#[derive(Debug, Serialize)]
pub struct NonceResponse {
    pub nonce: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifySignatureRequest {
    pub wallet_address: String,
    pub signature: String,
    pub nonce: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
} 