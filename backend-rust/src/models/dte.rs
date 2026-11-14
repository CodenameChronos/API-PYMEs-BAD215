use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransaccionDte {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "_rev", skip_serializing_if = "Option::is_none")]
    pub rev: Option<String>,
    pub doc_type: String, // "transaccion_dte"
    pub orden_id: String,
    pub codigo_generacion: String,
    pub sello_recepcion: String,
    pub tipo_dte: String, // "01" Factura, "03" Crédito Fiscal
    pub monto_gravado: f64,
    pub iva: f64,
    pub monto_total: f64,
    pub estado_hacienda: String, // "PROCESADO_SANDBOX"
    pub fecha_emision: String,
}