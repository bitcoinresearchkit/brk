use std::{sync::Arc, time::Instant};

use api::{structs::Routes, ApiRoutes};
use axum::{routing::get, serve, Router};
use color_eyre::owo_colors::OwoColorize;
use log::{error, info};
use reqwest::StatusCode;
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;
use website::WebsiteRoutes;

use crate::structs::Config;

pub mod api;
mod header_map;
mod website;

#[derive(Clone)]
pub struct AppState {
    routes: Arc<Routes>,
    config: Config,
}

pub async fn main(routes: Routes, config: Config) -> color_eyre::Result<()> {
    routes.generate_dts_file();

    let state = AppState {
        routes: Arc::new(routes),
        config,
    };

    let compression_layer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true);

    let router = Router::new()
        .add_api_routes()
        .add_website_routes()
        .route("/version", get(env!("CARGO_PKG_VERSION")))
        .with_state(state)
        .layer(compression_layer);

    let mut port = 3110;

    let mut listener;
    loop {
        listener = TcpListener::bind(format!("0.0.0.0:{port}")).await;
        if listener.is_ok() {
            break;
        }
        port += 1;
    }

    info!("Starting server on port {port}...");
    let listener = listener.unwrap();

    serve(listener, router).await?;

    Ok(())
}

pub fn log_result(code: StatusCode, path: &str, instant: Instant) {
    let time = format!("{}µs", instant.elapsed().as_micros());
    let time = time.bright_black();
    match code {
        StatusCode::INTERNAL_SERVER_ERROR => error!("{} {} {}", code.as_u16().red(), path, time),
        StatusCode::NOT_MODIFIED => info!("{} {} {}", code.as_u16().bright_black(), path, time),
        StatusCode::OK => info!("{} {} {}", code.as_u16().green(), path, time),
        _ => error!("{} {} {}", code.as_u16().red(), path, time),
    }
}
