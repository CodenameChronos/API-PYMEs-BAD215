use crate::{config::AppConfig, error::AppError};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

#[derive(Clone)]
pub struct CouchDbClient {
    http: Client,
    base_url: String,
    user: String,
    pass: String,
}

impl CouchDbClient {
    pub fn new(config: &AppConfig) -> Self {
        let http = Client::builder()
            .pool_max_idle_per_host(32)
            .build()
            .expect("No se pudo construir el cliente HTTP");

        let base_url = format!("{}/{}", config.couchdb_url, config.couchdb_db_name);

        Self {
            http,
            base_url,
            user: config.couchdb_user.clone(),
            pass: config.couchdb_pass.clone(),
        }
    }

    pub async fn ensure_database(&self) -> Result<(), AppError> {
        let res = self
            .http
            .put(&self.base_url)
            .basic_auth(&self.user, Some(&self.pass))
            .send()
            .await?;

        if res.status().is_success() || res.status().as_u16() == 412 {
            Ok(())
        } else {
            Err(AppError::ValidationError(
                "No se pudo inicializar la base de datos en CouchDB".to_string(),
            ))
        }
    }

    pub async fn insert_doc<T: Serialize>(&self, doc: &T) -> Result<Value, AppError> {
        let value = serde_json::to_value(doc).map_err(|e| {
            AppError::ValidationError(format!("Error serializando documento: {}", e))
        })?;

        // Si el documento tiene un _id asignado, usamos PUT /{db}/{id}
        let res = if let Some(id) = value.get("_id").and_then(|v| v.as_str()) {
            let url = format!("{}/{}", self.base_url, id);
            self.http
                .put(&url)
                .basic_auth(&self.user, Some(&self.pass))
                .json(&value)
                .send()
                .await?
        } else {
            // Si no tiene ID, dejamos que CouchDB lo genere mediante POST /{db}
            self.http
                .post(&self.base_url)
                .basic_auth(&self.user, Some(&self.pass))
                .json(&value)
                .send()
                .await?
        };

        let body: Value = res.json().await?;
        Ok(body)
    }

    pub async fn get_doc<T: DeserializeOwned>(&self, id: &str) -> Result<T, AppError> {
        let url = format!("{}/{}", self.base_url, id);
        let res = self
            .http
            .get(&url)
            .basic_auth(&self.user, Some(&self.pass))
            .send()
            .await?;

        if res.status().as_u16() == 404 {
            return Err(AppError::NotFound(format!("Documento {} no existe", id)));
        }

        let doc: T = res.json().await?;
        Ok(doc)
    }

    pub async fn find_mango<T: DeserializeOwned>(&self, selector: Value) -> Result<Vec<T>, AppError> {
        let url = format!("{}/_find", self.base_url);
        let payload = json!({
            "selector": selector,
            "limit": 100
        });

        let res = self
            .http
            .post(&url)
            .basic_auth(&self.user, Some(&self.pass))
            .json(&payload)
            .send()
            .await?;

        let json_res: Value = res.json().await?;
        let docs_array = json_res["docs"].as_array().cloned().unwrap_or_default();

        let mut results = Vec::new();
        for item in docs_array {
            if let Ok(parsed) = serde_json::from_value::<T>(item) {
                results.push(parsed);
            }
        }

        Ok(results)
    }
}