use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetalleOrden {
    pub producto_id: String,
    pub pyme_id: String,
    pub cantidad: u32,
    pub subtotal: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrdenConsolidada {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "_rev", skip_serializing_if = "Option::is_none")]
    pub rev: Option<String>,
    #[serde(default)]
    pub doc_type: Option<String>,
    pub comprador_institucional: String,
    pub items: Vec<DetalleOrden>,
    #[serde(default)] // Permite recibir 0.0 o vacío si el cliente no lo envía
    pub total: f64,
    #[serde(default)] // Permite recibir vacío si el cliente no lo envía
    pub estado: String,
    #[serde(default)]
    pub fecha_creacion: String,
    pub dte_codigo_generacion: Option<String>,
}