#![doc = include_str!("../README.md")]

use std::{panic, path::PathBuf, sync::Arc, time::{Duration, Instant}};

use aide::axum::ApiRouter;
use axum::{
    Extension,
    body::Body,
    http::{Request, Response, StatusCode, Uri},
    middleware::Next,
    response::Redirect,
    routing::get,
    serve,
};
use brk_error::Result;
use brk_mcp::route::mcp_router;
use brk_query::AsyncQuery;
use include_dir::{include_dir, Dir};
use quick_cache::sync::Cache;
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing::{error, info};

/// Embedded website assets
pub static EMBEDDED_WEBSITE: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../website");

/// Source for serving the website
#[derive(Debug, Clone)]
pub enum WebsiteSource {
    Disabled,
    Embedded,
    Filesystem(PathBuf),
}

impl WebsiteSource {
    pub fn is_enabled(&self) -> bool {
        !matches!(self, Self::Disabled)
    }
}

mod api;
pub mod cache;
mod extended;
mod files;
mod state;

use api::*;
pub use cache::{CacheParams, CacheStrategy};
use extended::*;
use files::FilesRoutes;
use state::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Server(AppState);

impl Server {
    pub fn new(query: &AsyncQuery, data_path: PathBuf, website: WebsiteSource) -> Self {
        Self(AppState {
            client: query.client().clone(),
            query: query.clone(),
            data_path,
            website,
            cache: Arc::new(Cache::new(5_000)),
            started_at: jiff::Timestamp::now(),
            started_instant: Instant::now(),
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
            .on_response(|response: &Response<Body>, latency: Duration, _: &tracing::Span| {
                let status = response.status().as_u16();
                let uri = response.extensions().get::<Uri>().unwrap();
                match response.status() {
                    StatusCode::OK => info!(status, %uri, ?latency),
                    StatusCode::NOT_MODIFIED
                    | StatusCode::TEMPORARY_REDIRECT
                    | StatusCode::PERMANENT_REDIRECT => info!(status, %uri, ?latency),
                    _ => error!(status, %uri, ?latency),
                }
            })
            .on_body_chunk(())
            .on_failure(())
            .on_eos(());

        let vecs = state.query.inner().vecs();
        let router = ApiRouter::new()
            .add_api_routes()
            .add_files_routes(&state.website)
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
        let router = router.finish_api(&mut openapi);

        let workspace_root: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .unwrap()
            .into();
        let output_paths = brk_bindgen::ClientOutputPaths::new()
            .rust(workspace_root.join("crates/brk_client/src/lib.rs"))
            .javascript(workspace_root.join("modules/brk-client/index.js"))
            .python(workspace_root.join("packages/brk_client/brk_client/__init__.py"));

        let openapi_json = Arc::new(serde_json::to_string(&openapi).unwrap());
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            brk_bindgen::generate_clients(vecs, &openapi_json, &output_paths)
        }));

        match result {
            Ok(Ok(())) => info!("Generated clients"),
            Ok(Err(e)) => error!("Failed to generate clients: {e}"),
            Err(_) => error!("Client generation panicked"),
        }

        let router = if mcp {
            let base_url = format!("http://127.0.0.1:{port}");
            router.merge(mcp_router(base_url, openapi_json))
        } else {
            router
        };

        serve(
            listener,
            router
                .layer(Extension(Arc::new(openapi)))
                .into_make_service(),
        )
        .await?;

        Ok(())
    }
}
