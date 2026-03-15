#![doc = include_str!("../README.md")]

use std::{
    any::Any,
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use aide::axum::ApiRouter;
use axum::{
    Extension, ServiceExt,
    body::Body,
    http::{
        Request, Response, StatusCode, Uri,
        header::{CONTENT_TYPE, VARY},
    },
    middleware::Next,
    response::{IntoResponse, Redirect},
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
pub use api::ApiRoutes;
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

        let connect_info_layer = axum::middleware::from_fn(
            async |connect_info: axum::extract::ConnectInfo<SocketAddr>,
                   mut request: Request<Body>,
                   next: Next|
                   -> Response<Body> {
                let mut addr = connect_info.0;

                // When behind a reverse proxy (e.g. cloudflared), the direct
                // connection comes from loopback but the request is external.
                // Mark it as non-loopback so it gets the stricter limit.
                if addr.ip().is_loopback() && request.headers().contains_key("CF-Connecting-IP") {
                    addr.set_ip(std::net::Ipv4Addr::UNSPECIFIED.into());
                }

                request.extensions_mut().insert(addr);
                next.run(request).await
            },
        );

        let response_time_layer = axum::middleware::from_fn(
            async |request: Request<Body>, next: Next| -> Response<Body> {
                let uri = request.uri().clone();
                let start = Instant::now();
                let mut response = next.run(request).await;
                response.extensions_mut().insert(uri);
                response.headers_mut().insert(
                    "X-Response-Time",
                    format!("{}us", start.elapsed().as_micros())
                        .parse()
                        .unwrap(),
                );
                response
            },
        );

        // Wrap non-JSON error responses in structured JSON
        let json_error_layer = axum::middleware::from_fn(
            async |request: Request<Body>, next: Next| -> Response<Body> {
                let response = next.run(request).await;
                let status = response.status();
                if status.is_success()
                    || status.is_redirection()
                    || status.is_informational()
                    || response.headers().get(CONTENT_TYPE).is_some_and(|v| {
                        let b = v.as_bytes();
                        b.starts_with(b"application/") && b.ends_with(b"json")
                    })
                {
                    return response;
                }

                let (parts, body) = response.into_parts();
                let bytes = axum::body::to_bytes(body, 4096).await.unwrap_or_default();
                let msg = String::from_utf8_lossy(&bytes);
                let (code, msg) = match parts.status {
                    StatusCode::NOT_FOUND => (
                        "not_found",
                        if msg.is_empty() {
                            "Not found".into()
                        } else {
                            msg
                        },
                    ),
                    StatusCode::METHOD_NOT_ALLOWED => (
                        "method_not_allowed",
                        "Only GET requests are supported".into(),
                    ),
                    StatusCode::GATEWAY_TIMEOUT => ("timeout", "Request timed out".into()),
                    s if s.is_client_error() => (
                        "bad_request",
                        if msg.is_empty() {
                            "Bad request".into()
                        } else {
                            msg
                        },
                    ),
                    _ => (
                        "internal_error",
                        if msg.is_empty() {
                            "Internal server error".into()
                        } else {
                            msg
                        },
                    ),
                };
                let msg = msg.into_owned();
                let mut response = Error::new(parts.status, code, msg).into_response();
                response.extensions_mut().extend(parts.extensions);
                response
            },
        );

        let trace_layer = TraceLayer::new_for_http()
            .on_request(())
            .on_response(
                |response: &Response<Body>, latency: Duration, _: &tracing::Span| {
                    let status = response.status().as_u16();
                    let unknown = Uri::from_static("/unknown");
                    let uri = response.extensions().get::<Uri>().unwrap_or(&unknown);
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
            .layer(compression_layer)
            .layer(response_time_layer)
            .layer(trace_layer)
            .layer(CatchPanicLayer::custom(|panic: Box<dyn Any + Send>| {
                let msg = panic
                    .downcast_ref::<String>()
                    .map(|s| s.as_str())
                    .or_else(|| panic.downcast_ref::<&str>().copied())
                    .unwrap_or("Unknown panic");
                Error::internal(msg).into_response()
            }))
            .layer(TimeoutLayer::with_status_code(
                StatusCode::GATEWAY_TIMEOUT,
                Duration::from_secs(5),
            ))
            .layer(json_error_layer)
            .layer(CorsLayer::permissive())
            .layer(axum::middleware::from_fn(
                async |request: Request<Body>, next: Next| -> Response<Body> {
                    let mut response = next.run(request).await;
                    // Consolidate multiple Vary headers into one
                    let vary: Vec<&str> = response
                        .headers()
                        .get_all(VARY)
                        .iter()
                        .filter_map(|v| v.to_str().ok())
                        .collect();
                    if vary.len() > 1 {
                        let merged = vary.join(", ");
                        response.headers_mut().insert(VARY, merged.parse().unwrap());
                    }
                    response
                },
            ))
            .layer(connect_info_layer);

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

        let (router, openapi) = finish_openapi(router);

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

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                generate_bindings(vecs, &openapi, &output_paths)
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

        // NormalizePath must wrap the router (not be a layer) to run before route matching
        let app = NormalizePathLayer::trim_trailing_slash().layer(router);

        serve(
            listener,
            ServiceExt::<Request<Body>>::into_make_service_with_connect_info::<SocketAddr>(app),
        )
        .await?;

        Ok(())
    }
}

/// Finalize a router and extract the OpenAPI spec.
pub fn finish_openapi<S: Clone + Send + Sync + 'static>(
    router: ApiRouter<S>,
) -> (axum::Router<S>, aide::openapi::OpenApi) {
    let mut openapi = create_openapi();
    let router = router.finish_api(&mut openapi);
    (router, openapi)
}

#[cfg(feature = "bindgen")]
pub fn generate_bindings(
    vecs: &brk_query::Vecs,
    openapi: &aide::openapi::OpenApi,
    output_paths: &brk_bindgen::ClientOutputPaths,
) -> std::io::Result<()> {
    let openapi_json = serde_json::to_string(openapi)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    brk_bindgen::generate_clients(vecs, &openapi_json, output_paths)
}
