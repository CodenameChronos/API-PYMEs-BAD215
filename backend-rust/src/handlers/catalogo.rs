use crate::{
    error::AppError,
    models::producto::CatalogoItem,
    services::couchdb_client::CouchDbClient,
};
use axum::{extract::State, Json};
use serde_json::{json, Value};
use std::time::Instant;
use uuid::Uuid;

pub async fn crear_producto(
    State(db): State<CouchDbClient>,
    Json(mut payload): Json<CatalogoItem>,
) -> Result<Json<Value>, AppError> {
    let start = Instant::now();

    if payload.precio_unitario <= 0.0 {
        return Err(AppError::ValidationError(
            "El precio unitario debe ser positivo".to_string(),
        ));
    }

    payload.id = Some(format!("prod_{}", Uuid::new_v4()));
    payload.doc_type = Some("catalogo_item".to_string()); // <-- Envuelto en Some()

    let res = db.insert_doc(&payload).await?;
    let latency_ms = start.elapsed().as_secs_f64() * 1000.0;

    Ok(Json(json!({
        "status": "ok",
        "latency_ms": latency_ms,
        "couchdb_response": res
    })))
}

pub async fn listar_catalogo(State(db): State<CouchDbClient>) -> Result<Json<Value>, AppError> {
    let start = Instant::now();
    let selector = json!({ "doc_type": "catalogo_item" });
    let items: Vec<CatalogoItem> = db.find_mango(selector).await?;
    let latency_ms = start.elapsed().as_secs_f64() * 1000.0;

    Ok(Json(json!({
        "status": "ok",
        "latency_ms": latency_ms,
        "count": items.len(),
        "data": items
    })))
}