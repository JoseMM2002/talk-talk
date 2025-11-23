use std::str::FromStr;

use axum::http::{HeaderName, HeaderValue, Method};
use log::{debug, error};
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::config::get_cors_config;

pub async fn build_cors_layer() -> CorsLayer {
    let cors_config = get_cors_config().await;

    let mut origins: Vec<HeaderValue> = Vec::with_capacity(cors_config.allowed_origins.len());
    let mut methods: Vec<Method> = Vec::with_capacity(cors_config.allowed_methods.len());
    let mut headers: Vec<HeaderName> = Vec::with_capacity(cors_config.allowed_methods.len());

    for origin in cors_config.allowed_origins.iter() {
        match HeaderValue::from_str(origin.as_str()) {
            Ok(v) => origins.push(v),
            Err(e) => error!("Error parsing CORS origin '{}': {}", origin, e),
        }
    }

    for method in cors_config.allowed_methods.iter() {
        match Method::from_str(method.as_str()) {
            Ok(m) => methods.push(m),
            Err(e) => error!("Error parsing CORS method '{}': {}", method, e),
        }
    }

    for header in cors_config.allowed_headers.iter() {
        match HeaderName::from_str(header.as_str()) {
            Ok(v) => headers.push(v),
            Err(e) => error!("Error parsing CORS header '{}': {}", header, e),
        }
    }

    debug!("CORS origins: {:?}", origins);
    debug!("CORS methods: {:?}", methods);
    debug!("CORS headers: {:?}", headers);

    CorsLayer::new()
        .allow_methods(methods)
        .allow_origin(AllowOrigin::list(origins))
        .allow_headers(headers)
}
