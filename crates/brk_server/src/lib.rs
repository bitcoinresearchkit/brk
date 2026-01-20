#![doc = include_str!("../README.md")]

use std::{
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use aide::axum::ApiRouter;
use axum::{
    Extension, ServiceExt,
    body::Body,
    http::{Request, Response, StatusCode, Uri},
    middleware::Next,
    response::Redirect,
    routing::get,
    serve,
};
use brk_query::AsyncQuery;
use quick_cache::sync::Cache;
use tokio::net::TcpListener;
use tower_http::{
    catch_panic::CatchPanicLayer, classify::ServerErrorsFailureClass,
    compression::CompressionLayer, cors::CorsLayer, normalize_path::NormalizePathLayer,
    timeout::TimeoutLayer, trace::TraceLayer,
};
use tower_layer::Layer;
use tracing::{error, info};

mod api;
pub mod cache;
mod error;
mod extended;
mod state;

use api::*;
pub use brk_types::Port;
pub use brk_website::Website;
pub use cache::{CacheParams, CacheStrategy};
pub use error::{Error, Result};
use state::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Server(AppState);

impl Server {
    pub fn new(query: &AsyncQuery, data_path: PathBuf, website: Website) -> Self {
        website.log();
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

    pub async fn serve(self, port: Option<Port>) -> brk_error::Result<()> {
        let state = self.0;

        #[cfg(feature = "bindgen")]
        let vecs = state.query.inner().vecs();

        let compression_layer = CompressionLayer::new().br(true).gzip(true).zstd(true);

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
                |response: &Response<Body>, latency: Duration, _: &tracing::Span| {
                    let status = response.status().as_u16();
                    let uri = response.extensions().get::<Uri>().unwrap();
                    match response.status() {
                        StatusCode::OK => info!(status, %uri, ?latency),
                        StatusCode::NOT_MODIFIED
                        | StatusCode::TEMPORARY_REDIRECT
                        | StatusCode::PERMANENT_REDIRECT => info!(status, %uri, ?latency),
                        _ => error!(status, %uri, ?latency),
                    }
                },
            )
            .on_body_chunk(())
            .on_failure(
                |error: ServerErrorsFailureClass, latency: Duration, _: &tracing::Span| {
                    error!(?error, ?latency, "request failed");
                },
            )
            .on_eos(());

        let website_router = brk_website::router(state.website.clone());
        let mut router = ApiRouter::new().add_api_routes();
        if !state.website.is_enabled() {
            router = router.route("/", get(Redirect::temporary("/api")));
        }
        let router = router
            .with_state(state)
            .merge(website_router)
            .layer(CatchPanicLayer::new())
            .layer(compression_layer)
            .layer(response_uri_layer)
            .layer(trace_layer)
            .layer(TimeoutLayer::with_status_code(
                StatusCode::GATEWAY_TIMEOUT,
                Duration::from_secs(5),
            ))
            .layer(CorsLayer::permissive());

        let (listener, port) = match port {
            Some(port) => {
                let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
                (listener, *port)
            }
            None => {
                let base_port: u16 = *Port::DEFAULT;
                let max_port: u16 = base_port + 100;
                let mut port = base_port;
                let listener = loop {
                    match TcpListener::bind(format!("0.0.0.0:{port}")).await {
                        Ok(l) => break l,
                        Err(_) if port < max_port => port += 1,
                        Err(e) => return Err(e.into()),
                    }
                };
                (listener, port)
            }
        };

        info!("Starting server on port {port}...");

        let mut openapi = create_openapi();
        let router = router.finish_api(&mut openapi);

        #[cfg(feature = "bindgen")]
        {
            let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .and_then(|p| p.parent())
                .unwrap()
                .to_path_buf();

            let output_paths = brk_bindgen::ClientOutputPaths::new()
                .rust(workspace_root.join("crates/brk_client/src/lib.rs"))
                .javascript(workspace_root.join("modules/brk-client/index.js"))
                .python(workspace_root.join("packages/brk_client/brk_client/__init__.py"));

            let openapi_json = serde_json::to_string(&openapi).unwrap();

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                brk_bindgen::generate_clients(vecs, &openapi_json, &output_paths)
            }));

            match result {
                Ok(Ok(())) => info!("Generated clients"),
                Ok(Err(e)) => error!("Failed to generate clients: {e}"),
                Err(_) => error!("Client generation panicked"),
            }
        }

        let api_json = Arc::new(ApiJson::new(&openapi));

        let router = router
            .layer(Extension(Arc::new(openapi)))
            .layer(Extension(api_json));

        let service = NormalizePathLayer::trim_trailing_slash().layer(router);

        serve(
            listener,
            ServiceExt::<Request<Body>>::into_make_service(service),
        )
        .await?;

        Ok(())
    }
}
