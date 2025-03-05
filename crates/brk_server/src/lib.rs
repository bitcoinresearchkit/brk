#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

use std::{
    fs,
    io::Cursor,
    path::{Path, PathBuf},
    time::Instant,
};

use api::{ApiRoutes, DTS};
use axum::{Json, Router, http::StatusCode, routing::get, serve};
use brk_computer::Computer;
use brk_core::path_dot_brk;
use brk_indexer::Indexer;
use brk_query::Query;
use color_eyre::owo_colors::OwoColorize;
use files::FilesRoutes;
use log::{error, info};
pub use tokio;
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;

mod api;
mod files;
mod traits;

pub use files::Frontend;

#[derive(Clone)]
pub struct AppState {
    query: &'static Query<'static>,
    frontend: Frontend,
    websites_path: Option<PathBuf>,
}

const DEV_PATH: &str = "../..";
const DOWNLOADS: &str = "downloads";
const WEBSITES: &str = "websites";

// TODO
pub struct Server;

pub async fn main(
    indexer: Indexer,
    computer: Computer,
    frontend: Frontend,
) -> color_eyre::Result<()> {
    let indexer = Box::leak(Box::new(indexer));
    let computer = Box::leak(Box::new(computer));
    let query = Box::leak(Box::new(Query::build(indexer, computer)));

    let websites_path = if frontend.is_some() {
        let websites_dev_path = Path::new(DEV_PATH).join(WEBSITES);

        let websites_path = if fs::exists(&websites_dev_path)? {
            websites_dev_path
        } else {
            let downloads_path = path_dot_brk().join(DOWNLOADS);

            let downloaded_websites_path = downloads_path.join("brk-main").join(WEBSITES);

            if !fs::exists(&downloaded_websites_path)? {
                info!("Downloading websites from Github...");

                // TODO
                // Need to download versioned, this is only for testing !
                let url = "https://github.com/bitcoinresearchkit/brk/archive/refs/heads/main.zip";

                let response = minreq::get(url).send()?;
                let bytes = response.as_bytes();
                let cursor = Cursor::new(bytes);

                let mut zip = zip::ZipArchive::new(cursor)?;

                zip.extract(&downloads_path)?;
            }

            downloaded_websites_path
        };

        query.generate_dts_file(frontend, websites_path.as_path())?;

        Some(websites_path)
    } else {
        None
    };

    let state = AppState {
        query,
        frontend,
        websites_path,
    };

    let compression_layer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true);

    let router = Router::new()
        .add_api_routes()
        .add_website_routes(frontend)
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
