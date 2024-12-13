use std::{collections::BTreeMap, path::PathBuf, sync::Arc};

use api::{structs::Routes, ApiRoutes};
use axum::{routing::get, serve, Router};
use color_eyre::owo_colors::OwoColorize;
use log::{error, info};
use reqwest::StatusCode;
use serde::Serialize;
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;
use website::WebsiteRoutes;

use crate::structs::Config;

mod api;
mod header_map;
mod website;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Grouped<T> {
    pub date: T,
    pub height: T,
    pub last: T,
}

#[derive(Clone)]
pub struct AppState {
    routes: Arc<Routes>,
}

pub async fn main(
    paths_to_type: BTreeMap<PathBuf, String>,
    config: &Config,
) -> color_eyre::Result<()> {
    let routes = Routes::build(paths_to_type, config);

    routes.generate_dts_file();

    let state = AppState {
        routes: Arc::new(routes),
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

pub fn log_result(code: StatusCode, path: &str) {
    match code {
        StatusCode::INTERNAL_SERVER_ERROR => error!("{} {}", code.as_u16().red(), path),
        StatusCode::NOT_MODIFIED => info!("{} {}", code.as_u16().bright_black(), path),
        StatusCode::OK => info!("{} {}", code.as_u16().green(), path),
        _ => error!("{} {}", code.as_u16().red(), path),
    }
}
