use std::time::Duration;

use sqlx::Pool;
use sqlx::pool::PoolOptions;
use sqlx::postgres::Postgres;
use tokio::sync::OnceCell;

use crate::config::get_postgres_config;

type Database = Postgres;
type DatabasePool = Pool<Database>;
type DatabaseOptions = PoolOptions<Database>;

static DB_POOL: OnceCell<DatabasePool> = OnceCell::const_new();

async fn get_database_pool() -> Result<DatabasePool, sqlx::Error> {
    let uri = get_postgres_config().await;
    DatabaseOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect(uri.uri.as_str())
        .await
}

/// # Errors
/// Errors if the database pool cannot be created.
pub async fn db_pool() -> Result<&'static DatabasePool, sqlx::Error> {
    DB_POOL
        .get_or_try_init(|| async { get_database_pool().await })
        .await
}
