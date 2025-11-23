use std::env::var;

use tokio::sync::OnceCell;

#[derive(Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    fn new() -> Self {
        AppConfig {
            host: var("APP_HOST").expect("APP_HOST must be set"),
            port: var("APP_PORT")
                .expect("APP_PORT must be set")
                .parse()
                .expect("APP_PORT must be a valid u16"),
        }
    }
}

static APP_CONFIG: OnceCell<AppConfig> = OnceCell::const_new();

pub async fn get_app_config() -> &'static AppConfig {
    APP_CONFIG.get_or_init(|| async { AppConfig::new() }).await
}
