use std::time::Instant;

use api::{ApiRoutes, VecIdToIndexToVec};
use axum::{http::StatusCode, routing::get, serve, Json, Router};
use color_eyre::owo_colors::OwoColorize;
use computer::Computer;
use files::FilesRoutes;
use indexer::Indexer;
use logger::{error, info};
use storable_vec::STATELESS;
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;

mod api;
mod files;
mod traits;

#[derive(Clone)]
pub struct AppState {
    vecs: &'static VecIdToIndexToVec,
    indexer: &'static Indexer<STATELESS>,
    computer: &'static Computer<STATELESS>,
}

pub const WEBSITE_DEV_PATH: &str = "../websites/kibo.money/";

pub async fn main(indexer: Indexer<STATELESS>, computer: Computer<STATELESS>) -> color_eyre::Result<()> {
    let indexer = Box::leak(Box::new(indexer));
    let computer = Box::leak(Box::new(computer));
    let vecs = Box::leak(Box::new(VecIdToIndexToVec::default()));

    indexer
        .vecs
        .as_any_json_vec_slice()
        .into_iter()
        .for_each(|vec| vecs.insert(vec));

    vecs.generate_dts_file()?;

    let state = AppState {
        vecs,
        indexer,
        computer,
    };

    let compression_layer = CompressionLayer::new().br(true).deflate(true).gzip(true).zstd(true);

    let router = Router::new()
        .add_api_routes()
        .add_website_routes()
        .route("/version", get(Json(env!("CARGO_PKG_VERSION"))))
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
