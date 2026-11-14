use crate::{
    error::AppError,
    models::orden::OrdenConsolidada,
    services::couchdb_client::CouchDbClient,
};
use axum::{extract::State, Json};
use chrono::Utc;
use serde_json::{json, Value};
use std::time::Instant;
use uuid::Uuid;

pub async fn crear_orden_consolidada(
    State(db): State<CouchDbClient>,
    Json(mut payload): Json<OrdenConsolidada>,
) -> Result<Json<Value>, AppError> {
    let start = Instant::now();

    if payload.items.is_empty() {
        return Err(AppError::ValidationError(
            "La orden consolidada debe contener al menos un ítem".to_string(),
        ));
    }

    let total_calculado: f64 = payload.items.iter().map(|i| i.subtotal).sum();

    payload.id = Some(format!("ord_{}", Uuid::new_v4()));
    payload.doc_type = Some("orden_consolidada".to_string()); // Envuelto en Some()
    payload.total = total_calculado;
    payload.estado = "CONFIRMADA_LOCAL".to_string();
    payload.fecha_creacion = Utc::now().to_rfc3339();

    let res = db.insert_doc(&payload).await?;
    let latency_ms = start.elapsed().as_secs_f64() * 1000.0;

    Ok(Json(json!({
        "status": "ok",
        "latency_ms": latency_ms,
        "orden_id": payload.id,
        "total": total_calculado,
        "couchdb_response": res
    })))
}