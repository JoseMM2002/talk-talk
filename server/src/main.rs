use std::net::ToSocketAddrs;

use axum::{Router, routing::any};
use log::info;
use server::{
    config::{self, build_tls_config},
    middleware::build_cors_layer,
    routes, websocket,
};
use tower_http::trace::TraceLayer;

#[dotenvy::load]
#[tokio::main]
async fn main() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install AWS-LC-Rust as default crypto provider");
    env_logger::init();
    let app_config = config::get_app_config().await;
    let tls_config = build_tls_config().await;

    let addr = (app_config.host.as_str(), app_config.port)
        .to_socket_addrs()
        .expect("Unable to resolve address")
        .next()
        .expect("Could not get next address");

    let app = Router::new()
        .nest("/api", routes::api_routes())
        .route("/ws", any(websocket::handler))
        .layer(TraceLayer::new_for_http())
        .layer(build_cors_layer().await);

    info!(
        "Starting server on https://{}:{}",
        app_config.host, app_config.port
    );

    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
