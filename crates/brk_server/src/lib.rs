#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

use std::{
    fs,
    io::Cursor,
    path::{Path, PathBuf},
    time::Duration,
};

use api::{ApiRoutes, DTS};
use axum::{
    Json, Router,
    body::Body,
    http::{Request, Response, StatusCode, Uri},
    middleware::Next,
    routing::get,
    serve,
};
use brk_computer::Computer;
use brk_core::dot_brk_path;
use brk_indexer::Indexer;
use brk_query::Query;
use color_eyre::owo_colors::OwoColorize;
use files::FilesRoutes;
use log::{error, info};
pub use tokio;
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

mod api;
mod files;
mod traits;

pub use files::Website;
use tracing::Span;

#[derive(Clone)]
pub struct AppState {
    // indexer: &'static Indexer,
    // computer: &'static Computer,
    query: &'static Query<'static>,
    website: Website,
    websites_path: Option<PathBuf>,
}

const DEV_PATH: &str = "../..";
const DOWNLOADS: &str = "downloads";
const WEBSITES: &str = "websites";
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Server(AppState);

impl Server {
    pub fn new(indexer: Indexer, computer: Computer, website: Website) -> color_eyre::Result<Self> {
        let indexer = Box::leak(Box::new(indexer));
        let computer = Box::leak(Box::new(computer));
        let query = Box::leak(Box::new(Query::build(indexer, computer)));

        let websites_path = if website.is_some() {
            let websites_dev_path = Path::new(DEV_PATH).join(WEBSITES);

            let websites_path = if fs::exists(&websites_dev_path)? {
                websites_dev_path
            } else {
                let downloads_path = dot_brk_path().join(DOWNLOADS);

                let downloaded_websites_path = downloads_path.join(VERSION).join(WEBSITES);

                if !fs::exists(&downloaded_websites_path)? {
                    info!("Downloading websites from Github...");

                    let url = format!(
                        "https://github.com/bitcoinresearchkit/brk/archive/refs/tags/{}.zip",
                        VERSION
                    );

                    let response = minreq::get(url).send()?;
                    let bytes = response.as_bytes();
                    let cursor = Cursor::new(bytes);

                    let mut zip = zip::ZipArchive::new(cursor)?;

                    zip.extract(&downloads_path)?;
                }

                downloaded_websites_path
            };

            query.generate_dts_file(website, websites_path.as_path())?;

            Some(websites_path)
        } else {
            None
        };

        Ok(Self(AppState {
            query,
            website,
            websites_path,
        }))
    }

    pub async fn serve(self) -> color_eyre::Result<()> {
        let state = self.0;

        let compression_layer = CompressionLayer::new()
            .br(true)
            .deflate(true)
            .gzip(true)
            .zstd(true);

        let response_uri_layer = axum::middleware::from_fn(
            async |request: Request<Body>, next: Next| -> Response<Body> {
                let uri = request.uri().clone();
                let mut response = next.run(request).await;
                response.extensions_mut().insert(uri);
                response
            },
        );

        let trace_layer = TraceLayer::new_for_http()
            .on_request(())
            .on_response(
                |response: &Response<Body>, latency: Duration, _span: &Span| {
                    let latency = latency.bright_black();
                    let status = response.status();
                    let uri = response.extensions().get::<Uri>().unwrap();
                    match status {
                        StatusCode::INTERNAL_SERVER_ERROR => {
                            error!("{} {} {:?}", status.as_u16().red(), uri, latency)
                        }
                        StatusCode::NOT_MODIFIED => {
                            info!("{} {} {:?}", status.as_u16().bright_black(), uri, latency)
                        }
                        StatusCode::OK => {
                            info!("{} {} {:?}", status.as_u16().green(), uri, latency)
                        }
                        _ => error!("{} {} {:?}", status.as_u16().red(), uri, latency),
                    }
                },
            )
            .on_body_chunk(())
            .on_failure(())
            .on_eos(());

        let router = Router::new()
            .add_api_routes()
            .add_website_routes(state.website)
            .route("/version", get(Json(VERSION)))
            .with_state(state)
            .layer(compression_layer)
            .layer(response_uri_layer)
            .layer(trace_layer);

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
}
