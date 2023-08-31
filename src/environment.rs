use std::env;
use std::env::VarError;
use tracing::{error, warn};
use crate::server_errors::ServerError;

pub struct Environment {
    pub database_url: String,
    pub redis_url: String,

    // Media storage (s3)
    pub s3_app_id: String,
    pub s3_app_key: String,
    pub s3_region: String,
    pub s3_endpoint: String,
    pub s3_base_storage_url: String,
    pub s3_bucket: String,

    // CORS origin
    pub origin: String,

    pub server_port: u16,

    // Loki logging server url & name of running figure-backend instance
    pub loki_host: Option<String>,
    pub loki_url: Option<String>,
}

impl Environment {
    pub fn new() -> Result<Self, ServerError> {
        Ok(
            Self {
                database_url: Self::get_var("DATABASE_URL").expect("No DATABASE_URL env found"),
                redis_url: Self::get_var("REDIS_URL").expect("No REDIS_URL env found"),
                s3_app_id: Self::get_var("S3_APP_ID").expect("No S3_APP_ID env found"),
                s3_app_key: Self::get_var("S3_APP_KEY").expect("No S3_APP_KEY env found"),
                s3_region: Self::get_var("S3_REGION").expect("No S3_REGION env found"),
                s3_endpoint: Self::get_var("S3_ENDPOINT").expect("No S3_ENDPOINT env found"),
                s3_base_storage_url: Self::get_var("S3_BASE_STORAGE_URL").expect("No S3_BASE_STORAGE_URL env found"),
                s3_bucket: Self::get_var("S3_BUCKET").expect("No S3_BUCKET env found"),
                origin: Self::get_var("ORIGIN").expect("No ORIGIN env found"),
                server_port: Self::get_var("SERVER_PORT").unwrap_or_else(|e| {
                    error!("{}", e);
                    warn!("Environment variable SERVER_PORT not found or invalid, defaulting to port 8000");
                    "8000".to_string()
                }).parse::<u16>().expect("Invalid SERVER_PORT env"),
                loki_host: Self::get_var("LOKI_HOST").ok(),
                loki_url: Self::get_var("LOKI_URL").ok(),
            }
        )
    }

    fn get_var(key: &str) -> Result<String, VarError> {
        let result = env::var(key);
        env::remove_var(key);
        result
    }
}