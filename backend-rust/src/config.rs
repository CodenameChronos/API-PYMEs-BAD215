use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub server_port: u16,
    pub couchdb_url: String,
    pub couchdb_user: String,
    pub couchdb_pass: String,
    pub couchdb_db_name: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("SERVER_PORT debe ser un número válido"),
            couchdb_url: env::var("COUCHDB_URL")
                .unwrap_or_else(|_| "http://couchdb-local:5984".to_string()),
            couchdb_user: env::var("COUCHDB_USER").unwrap_or_else(|_| "admin".to_string()),
            couchdb_pass: env::var("COUCHDB_PASS").unwrap_or_else(|_| "password".to_string()),
            couchdb_db_name: env::var("COUCHDB_DB_NAME")
                .unwrap_or_else(|_| "marketplace_pymes".to_string()),
        }
    }
}