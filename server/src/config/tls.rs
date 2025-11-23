use axum_server::tls_rustls::RustlsConfig;
use tokio::sync::OnceCell;

#[derive(Debug)]
pub struct TlsConfig {
    pub cert: String,
    pub key: String,
}

impl TlsConfig {
    fn new() -> Self {
        TlsConfig {
            cert: std::env::var("TLS_CERT").expect("TLS_CERT must be set"),
            key: std::env::var("TLS_KEY").expect("TLS_KEY must be set"),
        }
    }
}

static TLS_CONFIG: OnceCell<TlsConfig> = OnceCell::const_new();

pub async fn get_tls_config() -> &'static TlsConfig {
    TLS_CONFIG.get_or_init(|| async { TlsConfig::new() }).await
}

pub async fn build_tls_config() -> RustlsConfig {
    let tls_config = get_tls_config().await;
    RustlsConfig::from_pem_file(&tls_config.cert, &tls_config.key)
        .await
        .expect("Failed to build RustlsConfig from PEM files")
}
