use anyhow::Result;
use chrono::{Duration, Utc};
use ethers::types::{Address, Signature};
use ethers::utils::hash_message;
use jsonwebtoken::{encode, Header};
use std::str::FromStr;
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

use crate::config::{CONFIG, generate_nonce, verify_nonce};
use crate::models::{Claims, User};

// Db memory sederhana untuk menyimpan user
static USERS_DB: Lazy<Mutex<HashMap<String, User>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

pub fn get_nonce_for_wallet(wallet_address: &str) -> String {
    // Generate nonce baru untuk wallet address
    generate_nonce(wallet_address)
}

/// Verifikasi tanda tangan Ethereum
/// 
/// Fungsi ini melakukan verifikasi tanda tangan cryptocurrency dengan:
/// 1. Membuat pesan yang sama persis dengan yang ditandatangani oleh pengguna
/// 2. Memulihkan (recover) alamat wallet dari tanda tangan dan pesan tersebut
/// 3. Membandingkan alamat yang dipulihkan dengan alamat yang diklaim
/// 
/// Ini adalah implementasi standar dari Ethereum Personal Sign yang digunakan
/// oleh sebagian besar aplikasi Web3 untuk autentikasi.
pub fn verify_signature(wallet_address: &str, signature: &str, nonce: &str) -> Result<bool> {
    // Validasi nonce
    if !verify_nonce(wallet_address, nonce) {
        return Ok(false);
    }
    
    // Buat pesan yang ditandatangani (sama dengan yang dibuat di frontend)
    let signing_message = format!("Silakan tanda tangani pesan ini untuk autentikasi: {}", nonce);
    
    // Hash pesan menggunakan format Ethereum Signed Message
    let message_hash = hash_message(signing_message);
    
    // Konversi alamat wallet yang diklaim
    let claimed_address = Address::from_str(wallet_address)?;
    
    // Parse signature string 
    let sig = if signature.starts_with("0x") {
        Signature::from_str(&signature[2..])?
    } else {
        Signature::from_str(signature)?
    };
    
    // Recover address dari tanda tangan
    let recovered_address = sig.recover(message_hash)?;
    
    // Periksa apakah address yang di-recover sama dengan address yang diklaim
    Ok(recovered_address == claimed_address)
}

pub fn generate_jwt(wallet_address: &str) -> Result<String> {
    // Buat atau update user
    let wallet_address = wallet_address.to_lowercase();
    let mut users = USERS_DB.lock().unwrap();
    
    // Jika user sudah ada, update timestamp login terakhir
    // Jika belum ada, buat user baru
    let _user_data = if let Some(existing_user) = users.get_mut(&wallet_address) {
        existing_user.update_login();
        existing_user.clone()
    } else {
        let new_user = User::new(&wallet_address);
        users.insert(wallet_address.clone(), new_user.clone());
        new_user
    };
    
    // Buat claims untuk token
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(CONFIG.jwt.expiration))
        .expect("valid timestamp")
        .timestamp() as usize;
    
    let claims = Claims {
        wallet_address,
        exp: expiration,
    };
    
    // Generate token
    let token = encode(&Header::default(), &claims, &CONFIG.jwt.encoding_key)?;
    
    Ok(token)
}

pub fn get_user(wallet_address: &str) -> Option<User> {
    let users = USERS_DB.lock().unwrap();
    users.get(&wallet_address.to_lowercase()).cloned()
} 