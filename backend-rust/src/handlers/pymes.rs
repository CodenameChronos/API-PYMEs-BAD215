use crate::{
    error::AppError,
    models::pyme::PymeProveedor,
    services::couchdb_client::CouchDbClient,
};
use axum::{extract::State, Json};
use serde_json::{json, Value};
use std::time::Instant;
use uuid::Uuid;

pub async fn registrar_pyme(
    State(db): State<CouchDbClient>,
    Json(mut payload): Json<PymeProveedor>,
) -> Result<Json<Value>, AppError> {
    let start = Instant::now();

    if payload.nombre_comercial.trim().is_empty() || payload.nit.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Nombre comercial y NIT son obligatorios".to_string(),
        ));
    }

    payload.id = Some(format!("pyme_{}", Uuid::new_v4()));
    payload.doc_type = Some("pyme_proveedor".to_string()); // <-- Corregido con Some()

    let res = db.insert_doc(&payload).await?;
    let latency_ms = start.elapsed().as_secs_f64() * 1000.0;

    Ok(Json(json!({
        "status": "ok",
        "latency_ms": latency_ms,
        "couchdb_response": res
    })))
}

pub async fn listar_pymes(State(db): State<CouchDbClient>) -> Result<Json<Value>, AppError> {
    let start = Instant::now();
    let selector = json!({ "doc_type": "pyme_proveedor" });
    let pymes: Vec<PymeProveedor> = db.find_mango(selector).await?;
    let latency_ms = start.elapsed().as_secs_f64() * 1000.0;

    Ok(Json(json!({
        "status": "ok",
        "latency_ms": latency_ms,
        "count": pymes.len(),
        "data": pymes
    })))
}