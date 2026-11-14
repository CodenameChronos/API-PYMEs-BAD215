use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CatalogoItem {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "_rev", skip_serializing_if = "Option::is_none")]
    pub rev: Option<String>,
    pub doc_type: Option<String>, // Cambiado a Option
    pub pyme_id: String,
    pub nombre: String,
    pub descripcion: String,
    pub precio_unitario: f64,
    pub stock_disponible: u32,
    pub unidad_medida: String,
}