use std::{future::Future, path::PathBuf, sync::Arc, time::Instant};

use axum::{
    body::{Body, Bytes},
    http::{HeaderMap, HeaderValue, Response, header},
    response::IntoResponse,
};
use bitview_query::{AsyncQuery, Query, RepresentationId};
use brk_error::{Error as BrkError, Result};
use brk_rpc::AsyncClient;
#[cfg(feature = "chain")]
use brk_types::Version;
use derive_more::Deref;
use jiff::Timestamp;
use serde::Serialize;
use serde_json::to_vec;
use tokio::sync::Semaphore;

#[cfg(feature = "chain")]
use crate::prepared_json::PreparedJson;
#[cfg(feature = "series")]
use crate::series_bodies::SeriesBodies;
use crate::{CacheParams, CacheStrategy, CdnCacheMode, Error, Website, extended::ResponseExtended};

#[derive(Clone, Deref)]
pub struct AppState {
    #[deref]
    pub query: AsyncQuery,
    pub sync_query: Arc<Semaphore>,
    pub disk_query: Arc<Semaphore>,
    #[cfg(feature = "chain")]
    pub raw_block_bodies: Arc<Semaphore>,
    #[cfg(feature = "chain")]
    pub historical_price_bodies: Arc<Semaphore>,
    #[cfg(feature = "chain")]
    pub mempool_txid_bodies: Arc<Semaphore>,
    #[cfg(feature = "chain")]
    pub broadcast_requests: Arc<Semaphore>,
    pub node: AsyncClient,
    #[cfg(feature = "series")]
    pub series_bodies: Arc<SeriesBodies>,
    #[cfg(feature = "urpd")]
    pub urpd_query: Arc<Semaphore>,
    #[cfg(feature = "urpd")]
    pub urpd_bodies: Arc<Semaphore>,
    #[cfg(feature = "chain")]
    pub mining_pools_body: Arc<PreparedJson>,
    pub data_path: PathBuf,
    pub website: Website,
    pub started_at: Timestamp,
    pub started_instant: Instant,
    pub max_weight: usize,
    pub max_utxos: usize,
    pub cdn_cache_mode: CdnCacheMode,
}

impl AppState {
    /// Keep blocking admission with the job, including after request cancellation.
    pub async fn run_admitted<T: Send + 'static>(
        &self,
        f: impl FnOnce(&Query) -> Result<T> + Send + 'static,
    ) -> Result<T> {
        self.run_with_admission(&self.sync_query, f).await
    }

    pub async fn run_with_admission<T: Send + 'static>(
        &self,
        admission: &Arc<Semaphore>,
        f: impl FnOnce(&Query) -> Result<T> + Send + 'static,
    ) -> Result<T> {
        let permit = admission
            .clone()
            .acquire_owned()
            .await
            .map_err(|_| BrkError::Internal("query admission closed"))?;
        self.run(move |q| {
            let _permit = permit;
            f(q)
        })
        .await
    }

    #[cfg(feature = "chain")]
    pub fn representation_strategy(version: Version, identity: RepresentationId) -> CacheStrategy {
        match identity {
            RepresentationId::Content(hash) => CacheStrategy::LiveHash(hash),
            // Anchor identity and availability are distinct: a displaced
            // confirmed transaction must be resolved again even at depth.
            RepresentationId::Block(hash) => {
                CacheStrategy::Live(format!("tx3-{version}-{hash}").into())
            }
        }
    }

    pub fn assemble_response(
        params: CacheParams,
        bytes: Bytes,
        apply_content_headers: impl FnOnce(&mut HeaderMap),
    ) -> Response<Body> {
        let mut response = Response::new(Body::from(bytes));
        let headers = response.headers_mut();
        apply_content_headers(headers);
        params.apply_to(headers);
        response
    }

    /// Shared response pipeline: ETag short-circuit, body computation on the
    /// query thread, and header assembly. Used by [`AppState::respond`]
    /// (strategy-driven) and series endpoints, which build [`CacheParams`]
    /// directly from query resolution.
    pub async fn respond_with_params<F>(
        &self,
        headers: &HeaderMap,
        params: CacheParams,
        apply_content_headers: impl FnOnce(&mut HeaderMap),
        f: F,
    ) -> Response<Body>
    where
        F: FnOnce(&Query) -> Result<Bytes> + Send + 'static,
    {
        Self::respond_with_future(headers, params, async {
            Ok((self.run_admitted(f).await?, apply_content_headers))
        })
        .await
    }

    /// Defer body work and content-header preparation until after revalidation.
    pub async fn respond_with_future<H: FnOnce(&mut HeaderMap)>(
        headers: &HeaderMap,
        params: CacheParams,
        body: impl Future<Output = Result<(Bytes, H)>>,
    ) -> Response<Body> {
        if params.matches_etag(headers) {
            return ResponseExtended::new_not_modified(&params);
        }

        match body.await {
            Ok((bytes, apply_content_headers)) => {
                Self::assemble_response(params, bytes, apply_content_headers)
            }
            Err(error) => Error::from(error).into_response(),
        }
    }

    /// Strategy-driven cached response.
    async fn respond<F>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        content_type: &'static str,
        f: F,
    ) -> Response<Body>
    where
        F: FnOnce(&Query) -> Result<Bytes> + Send + 'static,
    {
        let params = CacheParams::resolve(&strategy, self.cdn_cache_mode);
        self.respond_with_params(
            headers,
            params,
            |h| {
                h.insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
            },
            f,
        )
        .await
    }

    fn respond_immediate(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        content_type: &'static str,
        bytes: impl FnOnce() -> Bytes,
    ) -> Response<Body> {
        let params = CacheParams::resolve(&strategy, self.cdn_cache_mode);
        if params.matches_etag(headers) {
            return ResponseExtended::new_not_modified(&params);
        }

        Self::assemble_response(params, bytes(), |headers| {
            headers.insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
        })
    }

    /// Immediate JSON response whose value is only built after ETag validation.
    pub fn respond_json_immediate<T: Serialize>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        value: impl FnOnce() -> T,
    ) -> Response<Body> {
        self.respond_immediate(headers, strategy, "application/json", || {
            Bytes::from(to_vec(&value()).unwrap())
        })
    }

    /// Immediate JSON response for values already resolved during preflight.
    pub fn respond_json_value<T: Serialize>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        value: T,
    ) -> Response<Body> {
        self.respond_json_immediate(headers, strategy, || value)
    }

    /// Immediate text response for values already resolved during preflight.
    #[cfg(feature = "chain")]
    pub fn respond_text_value(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        value: String,
    ) -> Response<Body> {
        self.respond_immediate(headers, strategy, "text/plain", || Bytes::from(value))
    }

    /// Immediate JSON response whose validator is derived from its exact bytes.
    #[cfg(feature = "chain")]
    pub fn respond_json_content_value<T: Serialize>(
        &self,
        headers: &HeaderMap,
        value: T,
    ) -> Response<Body> {
        let bytes = Bytes::from(to_vec(&value).unwrap());
        self.respond_json_content_bytes(headers, bytes)
    }

    /// JSON response with HTTP cache validation.
    pub async fn respond_json<T, F>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        f: F,
    ) -> Response<Body>
    where
        T: Serialize + Send + 'static,
        F: FnOnce(&Query) -> Result<T> + Send + 'static,
    {
        self.respond(headers, strategy, "application/json", move |q| {
            let value = f(q)?;
            Ok(Bytes::from(to_vec(&value).unwrap()))
        })
        .await
    }

    /// Pre-serialized JSON response with HTTP cache validation.
    pub async fn respond_json_bytes<F>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        f: F,
    ) -> Response<Body>
    where
        F: FnOnce(&Query) -> Result<Vec<u8>> + Send + 'static,
    {
        self.respond(headers, strategy, "application/json", move |q| {
            f(q).map(Bytes::from)
        })
        .await
    }

    /// JSON response whose representation identity is produced with its bytes.
    #[cfg(feature = "chain")]
    pub async fn respond_json_bound<F>(
        &self,
        headers: &HeaderMap,
        version: Version,
        f: F,
    ) -> Response<Body>
    where
        F: FnOnce(&Query) -> Result<(Vec<u8>, RepresentationId)> + Send + 'static,
    {
        let outcome = self
            .run_admitted(move |query| {
                let initial_tip = query.tip_hash_prefix();
                let (bytes, identity) = f(query)?;
                let current_tip = query.tip_hash_prefix();
                if matches!(identity, RepresentationId::Block(_)) && initial_tip != current_tip {
                    return Err(BrkError::StateUpdating);
                }
                Ok((bytes, identity))
            })
            .await;

        match outcome {
            Ok((bytes, identity)) => self.respond_immediate(
                headers,
                Self::representation_strategy(version, identity),
                "application/json",
                || Bytes::from(bytes),
            ),
            Err(error) => Error::from(error).into_response(),
        }
    }

    /// JSON response whose validator is derived from the exact serialized value.
    #[cfg(any(
        feature = "chain",
        feature = "price",
        feature = "urpd",
        all(test, feature = "series")
    ))]
    pub async fn respond_json_content<T, F>(&self, headers: &HeaderMap, f: F) -> Response<Body>
    where
        T: Serialize + Send + 'static,
        F: FnOnce(&Query) -> Result<T> + Send + 'static,
    {
        let bytes = self
            .run_admitted(move |query| {
                let value = f(query)?;
                Ok(Bytes::from(to_vec(&value)?))
            })
            .await;
        match bytes {
            Ok(bytes) => self.respond_json_content_bytes(headers, bytes),
            Err(error) => Error::from(error).into_response(),
        }
    }

    pub fn respond_json_content_bytes(&self, headers: &HeaderMap, bytes: Bytes) -> Response<Body> {
        let hash = RepresentationId::content_hash(&bytes);
        self.respond_immediate(
            headers,
            CacheStrategy::LiveHash(hash),
            "application/json",
            || bytes,
        )
    }

    /// Text response with HTTP cache validation.
    pub async fn respond_text<F>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        f: F,
    ) -> Response<Body>
    where
        F: FnOnce(&Query) -> Result<String> + Send + 'static,
    {
        self.respond(headers, strategy, "text/plain", move |q| {
            let value = f(q)?;
            Ok(Bytes::from(value))
        })
        .await
    }

    /// Binary response with HTTP cache validation.
    pub async fn respond_bytes<T, F>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        f: F,
    ) -> Response<Body>
    where
        T: Into<Vec<u8>> + Send + 'static,
        F: FnOnce(&Query) -> Result<T> + Send + 'static,
    {
        self.respond(headers, strategy, "application/octet-stream", move |q| {
            let value = f(q)?;
            Ok(Bytes::from(value.into()))
        })
        .await
    }
}

#[cfg(test)]
#[path = "../tests/unit/state.rs"]
mod tests;
