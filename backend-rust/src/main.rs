mod config;
mod error;
mod handlers;
mod models;
mod services;

use axum::{
    routing::{get, post},
    Router,
};
use config::AppConfig;
use services::couchdb_client::CouchDbClient;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::from_env();
    let db_client = CouchDbClient::new(&config);

    // Intenta crear la base de datos local en CouchDB si no existe
    if let Err(e) = db_client.ensure_database().await {
        tracing::warn!("Advertencia inicializando CouchDB: {}", e);
    }

    let app = Router::new()
        .route("/health", get(|| async { "OK - Backend Rust Marketplace" }))
        .route(
            "/api/pymes",
            post(handlers::pymes::registrar_pyme).get(handlers::pymes::listar_pymes),
        )
        .route(
            "/api/catalogo",
            post(handlers::catalogo::crear_producto).get(handlers::catalogo::listar_catalogo),
        )
        .route(
            "/api/ordenes",
            post(handlers::ordenes::crear_orden_consolidada),
        )
        .route("/api/dte/emitir", post(handlers::dte::generar_dte_orden))
        .with_state(db_client);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("Servidor Rust escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}