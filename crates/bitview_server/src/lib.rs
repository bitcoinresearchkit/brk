#![doc = include_str!("../README.md")]

use std::{
    any::Any,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};

use aide::{axum::ApiRouter, openapi::OpenApi};
use axum::{
    Extension, Router, ServiceExt,
    body::{Body, to_bytes},
    http::{
        Method, Request, Response, StatusCode,
        header::{ALLOW, CONTENT_TYPE, ETAG},
    },
    middleware::{Next, from_fn},
    response::{IntoResponse, Redirect},
    routing::get,
    serve,
};
use bitview_query::AsyncQuery;
use brk_error::Result;
use jiff::Timestamp;
use tokio::{net::TcpListener, sync::Semaphore};
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::{
        CompressionLayer, CompressionLevel,
        predicate::{NotForContentType, Predicate},
    },
    cors::CorsLayer,
    normalize_path::NormalizePathLayer,
    timeout::TimeoutLayer,
};
use tower_layer::Layer;
use tracing::{error, info};

mod api;
mod cache;
mod config;
mod error;
mod error_body;
mod etag;
mod extended;
mod params;
#[cfg(any(feature = "series", feature = "chain"))]
mod prepared_json;
#[cfg(any(feature = "chain", feature = "urpd", feature = "series"))]
mod raw_body;
mod read_availability;
mod response_size_above;
#[cfg(feature = "series")]
mod series_bodies;
mod state;
#[cfg(feature = "urpd")]
mod urpd_input;

pub use api::ApiRoutes;
use api::*;
pub use bitview_website::Website;
pub use brk_types::Port;
pub use cache::CdnCacheMode;
use cache::{CacheParams, CacheStrategy};
pub use config::{DEFAULT_BIND, DEFAULT_MAX_UTXOS, DEFAULT_MAX_WEIGHT, ServerConfig};
use error::Error;
#[cfg(any(feature = "chain", feature = "urpd", feature = "series"))]
use raw_body::RawBodyPermit;
use response_size_above::ResponseSizeAbove;
#[cfg(feature = "series")]
use series_bodies::SeriesBodies;
use state::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Cap for buffering an upstream error body before re-wrapping it as JSON.
/// Larger bodies are truncated; the bound only affects the message we surface.
const MAX_ERROR_BODY_BYTES: usize = 4096;

/// Per-request timeout. Hits return 504 Gateway Timeout.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

/// Avoid spending compression work on responses too small to benefit materially.
const MIN_COMPRESSED_RESPONSE_BYTES: u64 = 1024;

fn compression_layer() -> CompressionLayer<impl Predicate> {
    CompressionLayer::new()
        .br(true)
        .gzip(true)
        .zstd(true)
        .quality(CompressionLevel::Fastest)
        .compress_when(
            ResponseSizeAbove(MIN_COMPRESSED_RESPONSE_BYTES)
                .and(NotForContentType::GRPC)
                .and(NotForContentType::IMAGES)
                .and(NotForContentType::SSE),
        )
}

/// Matches `application/json` and `application/...+json`, ignoring parameters
/// like `; charset=utf-8`. Used to skip JSON-error rewriting for already-JSON bodies.
fn is_json_content_type(s: &str) -> bool {
    let mime = s.split(';').next().unwrap_or("").trim();
    mime == "application/json" || (mime.starts_with("application/") && mime.ends_with("+json"))
}

pub struct Server {
    state: AppState,
    listener: TcpListener,
}

impl Server {
    /// Binds the HTTP listener so startup failures are reported before the
    /// caller launches the long-running server task.
    pub async fn bind(query: &AsyncQuery, config: ServerConfig) -> Result<Self> {
        let address = SocketAddr::new(config.bind, config.port.into());
        let listener = TcpListener::bind(address).await?;

        config.website.log();

        #[cfg(feature = "series")]
        let series_bodies = query.run(|query| Ok(SeriesBodies::new(query))).await?;
        #[cfg(feature = "chain")]
        let mining_pools_body = Arc::new(prepared_json::PreparedJson::new(
            query.sync(|query| query.all_pools()),
        ));

        Ok(Self {
            state: AppState {
                query: query.clone(),
                sync_query: Arc::new(Semaphore::new(1)),
                disk_query: Arc::new(Semaphore::new(1)),
                #[cfg(feature = "chain")]
                raw_block_bodies: Arc::new(Semaphore::new(RawBodyPermit::CAPACITY)),
                #[cfg(feature = "chain")]
                historical_price_bodies: Arc::new(Semaphore::new(2)),
                #[cfg(feature = "chain")]
                mempool_txid_bodies: Arc::new(Semaphore::new(2)),
                #[cfg(feature = "chain")]
                broadcast_requests: Arc::new(Semaphore::new(
                    api::broadcast::BroadcastPermit::CAPACITY,
                )),
                node: query.run(|query| query.client().asynchronous()).await?,
                #[cfg(feature = "series")]
                series_bodies,
                #[cfg(feature = "urpd")]
                urpd_query: Arc::new(Semaphore::new(2)),
                #[cfg(feature = "urpd")]
                urpd_bodies: Arc::new(Semaphore::new(2)),
                #[cfg(feature = "chain")]
                mining_pools_body,
                data_path: config.data_path,
                website: config.website,
                started_at: Timestamp::now(),
                started_instant: Instant::now(),
                max_weight: config.max_weight,
                max_utxos: config.max_utxos,
                cdn_cache_mode: config.cdn_cache_mode,
            },
            listener,
        })
    }

    pub async fn serve(self) -> Result<()> {
        let Self { state, listener } = self;
        let address = listener.local_addr()?;

        let response_time_layer = from_fn(
            async |request: Request<Body>, next: Next| -> Response<Body> {
                let uri = request.uri().clone();
                let method = request.method().clone();
                let start = Instant::now();
                let mut response = next.run(request).await;
                let latency = start.elapsed();
                let status_code = response.status();
                let status = status_code.as_u16();

                match status_code {
                    status_code
                        if status_code.is_informational()
                            || status_code.is_success()
                            || status_code.is_redirection() =>
                    {
                        info!(%method, status, %uri, ?latency)
                    }
                    _ => error!(%method, status, %uri, ?latency),
                }

                response.headers_mut().insert(
                    "X-Response-Time",
                    format!("{}us", latency.as_micros()).parse().unwrap(),
                );
                if method == Method::POST {
                    CacheParams::apply_error_cache_control(
                        response.headers_mut(),
                        cache::ErrorCachePolicy::NoStore,
                    );
                    response.headers_mut().remove(ETAG);
                }
                #[cfg(any(feature = "chain", feature = "urpd", feature = "series"))]
                let response = RawBodyPermit::retain(response);
                response
            },
        );

        // Wrap non-JSON error responses in structured JSON
        let json_error_layer = from_fn(
            async |request: Request<Body>, next: Next| -> Response<Body> {
                let action = request.method() == Method::POST;
                let response = next.run(request).await;
                let status = response.status();
                if status.is_success()
                    || status.is_redirection()
                    || status.is_informational()
                    || response
                        .headers()
                        .get(CONTENT_TYPE)
                        .is_some_and(|v| v.to_str().is_ok_and(is_json_content_type))
                {
                    return response;
                }

                let (parts, body) = response.into_parts();
                let bytes = to_bytes(body, MAX_ERROR_BODY_BYTES)
                    .await
                    .unwrap_or_default();
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
                        "Method not allowed for this endpoint".into(),
                    ),
                    StatusCode::GATEWAY_TIMEOUT if action => (
                        "timeout",
                        "Request timed out; submission outcome may be unknown".into(),
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
                if let Some(allow) = parts.headers.get(ALLOW) {
                    response.headers_mut().insert(ALLOW, allow.clone());
                }
                response
            },
        );

        let website_router = bitview_website::router(state.website.clone());
        let mut router = ApiRouter::new()
            .add_api_routes()
            .layer(from_fn(read_availability::wait))
            .layer(TimeoutLayer::with_status_code(
                StatusCode::GATEWAY_TIMEOUT,
                REQUEST_TIMEOUT,
            ));
        if !state.website.is_enabled() {
            router = router.route("/", get(Redirect::temporary("/api")));
        }
        let router = router
            .with_state(state)
            .merge(website_router)
            .layer(json_error_layer)
            .layer(compression_layer())
            .layer(CorsLayer::permissive())
            .layer(CatchPanicLayer::custom(|panic: Box<dyn Any + Send>| {
                let msg = panic
                    .downcast_ref::<String>()
                    .map(|s| s.as_str())
                    .or_else(|| panic.downcast_ref::<&str>().copied())
                    .unwrap_or("Unknown panic");
                Error::internal(msg).into_response()
            }))
            .layer(response_time_layer);

        info!("Server listening on http://{address}");

        let (router, openapi) = finish_openapi(router);

        let router = router
            .layer(Extension(OpenApiJson::new(&openapi)))
            .layer(Extension(ApiJson::new(&openapi)));

        // NormalizePath must wrap the router (not be a layer) to run before route matching
        let app = NormalizePathLayer::trim_trailing_slash().layer(router);

        serve(
            listener,
            ServiceExt::<Request<Body>>::into_make_service(app),
        )
        .await?;

        Ok(())
    }
}

/// Finalize a router and extract the OpenAPI spec.
pub fn finish_openapi<S: Clone + Send + Sync + 'static>(
    router: ApiRouter<S>,
) -> (Router<S>, OpenApi) {
    let mut openapi = create_openapi();
    let router = router.finish_api(&mut openapi);
    (router, openapi)
}

#[cfg(test)]
#[path = "../tests/unit/mod.rs"]
mod tests;
