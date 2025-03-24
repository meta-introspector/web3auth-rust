use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    Extension,
};
use crate::models::{
    AuthResponse, ErrorResponse, NonceRequest, NonceResponse, VerifySignatureRequest,
};
use tracing::error;
use crate::services::auth::{generate_jwt, get_nonce_for_wallet, get_user, verify_signature};

pub async fn get_nonce(
    Json(payload): Json<NonceRequest>,
) -> impl IntoResponse {
    let wallet_address = payload.wallet_address;
    
    if wallet_address.len() != 42 || !wallet_address.starts_with("0x") {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Format alamat Ethereum tidak valid".to_string(),
            }),
        ).into_response();
    }
    
    let nonce = get_nonce_for_wallet(&wallet_address);
    
    let message = format!("Silakan tanda tangani pesan ini untuk autentikasi: {}", nonce);
    
    (
        StatusCode::OK,
        Json(NonceResponse {
            nonce,
            message,
        }),
    ).into_response()
}

pub async fn verify_auth(
    Json(payload): Json<VerifySignatureRequest>,
) -> impl IntoResponse {
    let wallet_address = payload.wallet_address;
    let signature = payload.signature;
    let nonce = payload.nonce;
    
    match verify_signature(&wallet_address, &signature, &nonce) {
        Ok(true) => {
            match generate_jwt(&wallet_address) {
                Ok(token) => {
                    let user = get_user(&wallet_address).expect("User harus ada setelah generate JWT");
                    
                    (
                        StatusCode::OK,
                        Json(AuthResponse {
                            token,
                            user,
                        }),
                    ).into_response()
                },
                Err(e) => {
                    error!("Gagal membuat JWT: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: "Gagal membuat token".to_string(),
                        }),
                    ).into_response()
                }
            }
        },
        Ok(false) => {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Tanda tangan tidak valid atau nonce kedaluwarsa".to_string(),
                }),
            ).into_response()
        },
        Err(e) => {
            error!("Error verifikasi tanda tangan: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Gagal memverifikasi tanda tangan".to_string(),
                }),
            ).into_response()
        }
    }
}

pub async fn get_profile(
    Extension(wallet_address): Extension<String>,
) -> impl IntoResponse {
    match get_user(&wallet_address) {
        Some(user) => (StatusCode::OK, Json(user)).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "User tidak ditemukan".to_string(),
            }),
        ).into_response(),
    }
} 