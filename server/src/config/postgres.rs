use tokio::sync::OnceCell;

#[derive(Debug)]
pub struct PostgresConfig {
    pub uri: String,
}

impl PostgresConfig {
    fn new() -> Self {
        PostgresConfig {
            uri: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }
}

static POSTGRES_CONFIG: OnceCell<PostgresConfig> = OnceCell::const_new();

pub async fn get_postgres_config() -> &'static PostgresConfig {
    POSTGRES_CONFIG
        .get_or_init(|| async { PostgresConfig::new() })
        .await
}
