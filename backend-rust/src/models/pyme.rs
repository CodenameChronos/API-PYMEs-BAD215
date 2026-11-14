use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PymeProveedor {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "_rev", skip_serializing_if = "Option::is_none")]
    pub rev: Option<String>,
    pub doc_type: Option<String>, // Cambiado a Option para evitar errores si el cliente no lo envía
    pub nombre_comercial: String,
    pub nit: String,
    pub nrc: String,
    pub sector: String,
    pub perfil_verificado: bool,
    pub calificacion: f32,
}