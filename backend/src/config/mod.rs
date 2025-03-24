use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use std::env;
use std::sync::Mutex;
use std::collections::HashMap;
use uuid::Uuid;

pub static USER_NONCES: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

pub struct JwtConfig {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub expiration: i64,
}

impl JwtConfig {
    pub fn init() -> Self {
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "rahasia_jwt_sangat_aman_dan_panjang".to_string());
        
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            expiration: 3600, // 1 jam
        }
    }
}

pub static CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    let _ = dotenv::dotenv();
    
    AppConfig {
        port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap_or(8080),
        jwt: JwtConfig::init(),
    }
});

pub struct AppConfig {
    pub port: u16,
    pub jwt: JwtConfig,
}

pub fn generate_nonce(wallet_address: &str) -> String {
    let nonce = Uuid::new_v4().to_string();
    
    let mut nonces = USER_NONCES.lock().unwrap();
    nonces.insert(wallet_address.to_lowercase(), nonce.clone());
    
    nonce
}

pub fn verify_nonce(wallet_address: &str, nonce: &str) -> bool {
    let mut nonces = USER_NONCES.lock().unwrap();
    
    if let Some(stored_nonce) = nonces.get(&wallet_address.to_lowercase()) {
        if stored_nonce == nonce {
            nonces.remove(&wallet_address.to_lowercase());
            return true;
        }
    }
    
    false
} 