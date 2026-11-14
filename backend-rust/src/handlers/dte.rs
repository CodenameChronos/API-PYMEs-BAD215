use crate::{
    error::AppError,
    models::orden::OrdenConsolidada,
    services::{couchdb_client::CouchDbClient, dte_service::DteService},
};
use axum::{extract::State, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use std::time::Instant;

#[derive(Deserialize)]
pub struct EmitirDteRequest {
    pub orden_id: String,
    pub tipo_dte: String, // "01" o "03"
}

pub async fn generar_dte_orden(
    State(db): State<CouchDbClient>,
    Json(req): Json<EmitirDteRequest>,
) -> Result<Json<Value>, AppError> {
    let start = Instant::now();

    let orden: OrdenConsolidada = db.get_doc(&req.orden_id).await?;
    let dte_doc = DteService::emitir_dte_simulado(&req.orden_id, orden.total, &req.tipo_dte)?;

    let res = db.insert_doc(&dte_doc).await?;
    let latency_ms = start.elapsed().as_secs_f64() * 1000.0;

    Ok(Json(json!({
        "status": "ok",
        "latency_ms": latency_ms,
        "dte": dte_doc,
        "couchdb_response": res
    })))
}