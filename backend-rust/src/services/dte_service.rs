use crate::{error::AppError, models::dte::TransaccionDte};
use chrono::Utc;
use uuid::Uuid;

pub struct DteService;

impl DteService {
    /// Simula la estructuración, validación fiscal (IVA 13% El Salvador) y generación de sello DTE
    pub fn emitir_dte_simulado(
        orden_id: &str,
        monto_total: f64,
        tipo_dte: &str,
    ) -> Result<TransaccionDte, AppError> {
        if monto_total <= 0.0 {
            return Err(AppError::DteError(
                "El monto total de la transacción debe ser mayor a cero".to_string(),
            ));
        }

        if tipo_dte != "01" && tipo_dte != "03" {
            return Err(AppError::DteError(
                "Tipo de DTE inválido. Use '01' (Factura) o '03' (CCF)".to_string(),
            ));
        }

        let monto_gravado = (monto_total / 1.13 * 100.0).round() / 100.0;
        let iva = ((monto_total - monto_gravado) * 100.0).round() / 100.0;

        let codigo_generacion = Uuid::new_v4().to_string().to_uppercase();
        let sello_recepcion = format!("MH-SV-{}", Uuid::new_v4().simple());

        Ok(TransaccionDte {
            id: Some(format!("dte_{}", codigo_generacion)),
            rev: None,
            doc_type: "transaccion_dte".to_string(),
            orden_id: orden_id.to_string(),
            codigo_generacion,
            sello_recepcion,
            tipo_dte: tipo_dte.to_string(),
            monto_gravado,
            iva,
            monto_total,
            estado_hacienda: "PROCESADO_SANDBOX".to_string(),
            fecha_emision: Utc::now().to_rfc3339(),
        })
    }
}