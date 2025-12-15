#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/server.rs")]
#![doc = "```"]

use std::{ops::Deref, path::PathBuf, sync::Arc, time::Duration};

use aide::axum::ApiRouter;
use api::ApiRoutes;
use axum::{
    Extension,
    body::{Body, Bytes},
    http::{Request, Response, StatusCode, Uri},
    middleware::Next,
    response::Redirect,
    routing::get,
    serve,
};
use brk_error::Result;
use brk_logger::OwoColorize;
use brk_mcp::route::MCPRoutes;
use brk_query::AsyncQuery;
use files::FilesRoutes;
use log::{error, info};
use quick_cache::sync::Cache;
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing::Span;

mod api;
pub mod cache;
mod extended;
mod files;

use api::*;
pub use cache::{CacheParams, CacheStrategy};
use extended::*;

#[derive(Clone)]
pub struct AppState {
    query: AsyncQuery,
    path: Option<PathBuf>,
    cache: Arc<Cache<String, Bytes>>,
}

impl Deref for AppState {
    type Target = AsyncQuery;
    fn deref(&self) -> &Self::Target {
        &self.query
    }
}

impl AppState {
    /// JSON response with caching
    pub async fn cached_json<T, F>(
        &self,
        headers: &axum::http::HeaderMap,
        strategy: CacheStrategy,
        f: F,
    ) -> axum::http::Response<axum::body::Body>
    where
        T: serde::Serialize + Send + 'static,
        F: FnOnce(&brk_query::Query) -> brk_error::Result<T> + Send + 'static,
    {
        let params = CacheParams::resolve(&strategy, || self.sync(|q| q.height().into()));
        if params.matches_etag(headers) {
            return ResponseExtended::new_not_modified();
        }
        match self.run(f).await {
            Ok(value) => ResponseExtended::new_json_cached(&value, &params),
            Err(e) => ResultExtended::<T>::to_json_response(Err(e), params.etag_str()),
        }
    }

    /// Text response with caching
    pub async fn cached_text<T, F>(
        &self,
        headers: &axum::http::HeaderMap,
        strategy: CacheStrategy,
        f: F,
    ) -> axum::http::Response<axum::body::Body>
    where
        T: AsRef<str> + Send + 'static,
        F: FnOnce(&brk_query::Query) -> brk_error::Result<T> + Send + 'static,
    {
        let params = CacheParams::resolve(&strategy, || self.sync(|q| q.height().into()));
        if params.matches_etag(headers) {
            return ResponseExtended::new_not_modified();
        }
        match self.run(f).await {
            Ok(value) => ResponseExtended::new_text_cached(value.as_ref(), &params),
            Err(e) => ResultExtended::<T>::to_text_response(Err(e), params.etag_str()),
        }
    }

    /// Binary response with caching
    pub async fn cached_bytes<T, F>(
        &self,
        headers: &axum::http::HeaderMap,
        strategy: CacheStrategy,
        f: F,
    ) -> axum::http::Response<axum::body::Body>
    where
        T: Into<Vec<u8>> + Send + 'static,
        F: FnOnce(&brk_query::Query) -> brk_error::Result<T> + Send + 'static,
    {
        let params = CacheParams::resolve(&strategy, || self.sync(|q| q.height().into()));
        if params.matches_etag(headers) {
            return ResponseExtended::new_not_modified();
        }
        match self.run(f).await {
            Ok(value) => ResponseExtended::new_bytes_cached(value.into(), &params),
            Err(e) => ResultExtended::<T>::to_bytes_response(Err(e), params.etag_str()),
        }
    }
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Server(AppState);

impl Server {
    pub fn new(query: &AsyncQuery, files_path: Option<PathBuf>) -> Self {
        Self(AppState {
            query: query.clone(),
            path: files_path,
            cache: Arc::new(Cache::new(5_000)),
        })
    }

    pub async fn serve(self, mcp: bool) -> Result<()> {
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
                        StatusCode::OK => {
                            info!("{} {} {:?}", status.as_u16().green(), uri, latency)
                        }
                        StatusCode::NOT_MODIFIED
                        | StatusCode::TEMPORARY_REDIRECT
                        | StatusCode::PERMANENT_REDIRECT => {
                            info!("{} {} {:?}", status.as_u16().bright_black(), uri, latency)
                        }
                        _ => error!("{} {} {:?}", status.as_u16().red(), uri, latency),
                    }
                },
            )
            .on_body_chunk(())
            .on_failure(())
            .on_eos(());

        let router = ApiRouter::new()
            .add_api_routes()
            .add_mcp_routes(&state.query, mcp)
            .add_files_routes(state.path.as_ref())
            .route(
                "/discord",
                get(Redirect::temporary("https://discord.gg/WACpShCB7M")),
            )
            .route("/crate", get(Redirect::temporary("https://crates.io/crates/brk")))
            .route(
                "/status",
                get(Redirect::temporary("https://status.bitview.space")),
            )
            .route("/github", get(Redirect::temporary("https://github.com/bitcoinresearchkit/brk")))
            .route("/changelog", get(Redirect::temporary("https://github.com/bitcoinresearchkit/brk/blob/main/docs/CHANGELOG.md")))
            .route(
                "/install",
                get(Redirect::temporary("https://github.com/bitcoinresearchkit/brk/blob/main/crates/brk_cli/README.md#brk_cli")),
            )
            .route(
                "/service",
                get(Redirect::temporary("https://github.com/bitcoinresearchkit/brk?tab=readme-ov-file#professional-hosting")),
            )
            .route("/nostr", get(Redirect::temporary("https://primal.net/p/npub1jagmm3x39lmwfnrtvxcs9ac7g300y3dusv9lgzhk2e4x5frpxlrqa73v44")))
            .with_state(state)
            .layer(compression_layer)
            .layer(response_uri_layer)
            .layer(trace_layer);

        const BASE_PORT: u16 = 3110;
        const MAX_PORT: u16 = BASE_PORT + 100;

        let mut port = BASE_PORT;
        let listener = loop {
            match TcpListener::bind(format!("0.0.0.0:{port}")).await {
                Ok(l) => break l,
                Err(_) if port < MAX_PORT => port += 1,
                Err(e) => return Err(e.into()),
            }
        };

        info!("Starting server on port {port}...");

        let mut openapi = create_openapi();
        serve(
            listener,
            router
                .finish_api(&mut openapi)
                .layer(Extension(Arc::new(openapi)))
                .into_make_service(),
        )
        .await?;

        Ok(())
    }
}
