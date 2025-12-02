use std::env::var;

use tokio::sync::OnceCell;

#[derive(Debug)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
}

impl CorsConfig {
    fn new() -> Self {
        let origins = var("CORS_ORIGINS").expect("CORS_ORIGINS must be set");
        let methods = var("CORS_METHODS").expect("CORS_METHODS must be set");
        let headers = var("CORS_HEADERS").expect("CORS_HEADERS must be set");
        CorsConfig {
            allowed_origins: origins
                .split(',')
                .map(std::string::ToString::to_string)
                .collect(),
            allowed_methods: methods
                .split(',')
                .map(std::string::ToString::to_string)
                .collect(),
            allowed_headers: headers
                .split(',')
                .map(std::string::ToString::to_string)
                .collect(),
        }
    }
}

static CORS_CONFIG: OnceCell<CorsConfig> = OnceCell::const_new();

pub async fn get_cors_config() -> &'static CorsConfig {
    CORS_CONFIG
        .get_or_init(|| async { CorsConfig::new() })
        .await
}
