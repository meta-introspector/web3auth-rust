use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, Validation};

use crate::config::CONFIG;
use crate::models::Claims;

pub async fn auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header["Bearer ".len()..];

    let token_data = match decode::<Claims>(
        token,
        &CONFIG.jwt.decoding_key,
        &Validation::default(),
    ) {
        Ok(data) => data,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let mut request = request;
    request.extensions_mut().insert(token_data.claims.wallet_address);

    let response = next.run(request).await;
    Ok(response)
} 