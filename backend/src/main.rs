mod config;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

use std::net::SocketAddr;
use axum::{
    routing::{get, post},
    Router,
    middleware as axum_middleware,
    http::{Method, HeaderName},
};
use tower_http::cors::{
    CorsLayer,
    AllowOrigin,
};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

use crate::config::CONFIG;
use crate::middleware::auth_middleware;
use crate::routes::auth::{
    get_nonce,
    get_profile,
    verify_auth,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let allowed_headers = vec![
        HeaderName::from_static("content-type"),
        HeaderName::from_static("authorization"),
        HeaderName::from_static("x-requested-with"),
    ];

    let app = Router::new()
        .route("/api/auth/nonce", post(get_nonce))
        .route("/api/auth/verify", post(verify_auth))
        .route("/api/profile", get(get_profile))
        .route(
            "/api/profile",
            get(get_profile)
                .route_layer(axum_middleware::from_fn(auth_middleware)),
        )
        
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::predicate(|_, _| true))
                .allow_methods([Method::GET, Method::POST])
                .allow_headers(allowed_headers)
                .allow_credentials(true),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.port));
    tracing::info!("Server berjalan di http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
