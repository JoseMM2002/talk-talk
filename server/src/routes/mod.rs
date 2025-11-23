use axum::{Json, Router, routing::get};
use log::error;
use serde::Serialize;
use sqlx::query_scalar;

use crate::database::db_pool;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub message: String,
}

#[derive(Serialize)]
struct CheckHealthResponse {
    api: bool,
    pool: bool,
    db: bool,
}

async fn check_health() -> Json<ApiResponse<CheckHealthResponse>> {
    let pool = db_pool().await;

    let (db, pool) = match pool {
        Ok(pool) => match query_scalar!("SELECT 1").fetch_one(pool).await {
            Ok(_) => (true, true),
            Err(err) => {
                error!("Error querying database: {}", err);
                (true, false)
            }
        },
        Err(err) => {
            error!("Error connecting to database: {}", err);
            (false, false)
        }
    };

    let result: ApiResponse<CheckHealthResponse> = ApiResponse {
        data: CheckHealthResponse {
            api: true,
            pool,
            db,
        },
        message: "Service is healthy".to_string(),
    };
    Json(result)
}

pub fn api_routes() -> Router {
    Router::new().nest(
        "/v1",
        Router::new().route("/checkhealth", get(check_health)),
    )
}
