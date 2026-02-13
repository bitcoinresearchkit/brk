use std::{future::Future, path::PathBuf, sync::Arc, time::{Duration, Instant}};

use derive_more::Deref;

use axum::{
    body::{Body, Bytes},
    http::{HeaderMap, Response, Uri},
};
use brk_query::AsyncQuery;
use brk_rpc::Client;
use jiff::Timestamp;
use quick_cache::sync::{Cache, GuardResult};
use serde::Serialize;

use crate::{
    CacheParams, CacheStrategy, Website,
    extended::{HeaderMapExtended, ResponseExtended, ResultExtended},
};

#[derive(Clone, Deref)]
pub struct AppState {
    #[deref]
    pub query: AsyncQuery,
    pub data_path: PathBuf,
    pub website: Website,
    pub cache: Arc<Cache<String, Bytes>>,
    pub client: Client,
    pub started_at: Timestamp,
    pub started_instant: Instant,
}

impl AppState {
    pub fn mempool_cache(&self) -> CacheStrategy {
        let hash = self.sync(|q| q.mempool().map(|m| m.next_block_hash()).unwrap_or(0));
        CacheStrategy::MempoolHash(hash)
    }

    /// JSON response with HTTP + server-side caching
    pub async fn cached_json<T, F>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        uri: &Uri,
        f: F,
    ) -> Response<Body>
    where
        T: Serialize + Send + 'static,
        F: FnOnce(&brk_query::Query) -> brk_error::Result<T> + Send + 'static,
    {
        let params = CacheParams::resolve(&strategy, || self.sync(|q| q.height().into()));
        if params.matches_etag(headers) {
            return ResponseExtended::new_not_modified();
        }

        let full_key = format!("{}-{}", uri, params.etag_str());
        let result = self
            .get_or_insert(&full_key, async move {
                let value = self.run(f).await?;
                Ok(serde_json::to_vec(&value).unwrap().into())
            })
            .await;

        match result {
            Ok(bytes) => {
                let mut response = Response::new(Body::from(bytes));
                let h = response.headers_mut();
                h.insert_content_type_application_json();
                h.insert_cache_control(&params.cache_control);
                if let Some(etag) = &params.etag {
                    h.insert_etag(etag);
                }
                response
            }
            Err(e) => ResultExtended::<T>::to_json_response(Err(e), params.etag_str()),
        }
    }

    /// Text response with HTTP + server-side caching
    pub async fn cached_text<T, F>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        uri: &Uri,
        f: F,
    ) -> Response<Body>
    where
        T: AsRef<str> + Send + 'static,
        F: FnOnce(&brk_query::Query) -> brk_error::Result<T> + Send + 'static,
    {
        let params = CacheParams::resolve(&strategy, || self.sync(|q| q.height().into()));
        if params.matches_etag(headers) {
            return ResponseExtended::new_not_modified();
        }

        let full_key = format!("{}-{}", uri, params.etag_str());
        let result = self
            .get_or_insert(&full_key, async move {
                let value = self.run(f).await?;
                Ok(Bytes::from(value.as_ref().to_owned()))
            })
            .await;

        match result {
            Ok(bytes) => {
                let mut response = Response::new(Body::from(bytes));
                let h = response.headers_mut();
                h.insert_content_type_text_plain();
                h.insert_cache_control(&params.cache_control);
                if let Some(etag) = &params.etag {
                    h.insert_etag(etag);
                }
                response
            }
            Err(e) => ResultExtended::<T>::to_text_response(Err(e), params.etag_str()),
        }
    }

    /// Binary response with HTTP + server-side caching
    pub async fn cached_bytes<T, F>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        uri: &Uri,
        f: F,
    ) -> Response<Body>
    where
        T: Into<Vec<u8>> + Send + 'static,
        F: FnOnce(&brk_query::Query) -> brk_error::Result<T> + Send + 'static,
    {
        let params = CacheParams::resolve(&strategy, || self.sync(|q| q.height().into()));
        if params.matches_etag(headers) {
            return ResponseExtended::new_not_modified();
        }

        let full_key = format!("{}-{}", uri, params.etag_str());
        let result = self
            .get_or_insert(&full_key, async move {
                let value = self.run(f).await?;
                Ok(Bytes::from(value.into()))
            })
            .await;

        match result {
            Ok(bytes) => {
                let mut response = Response::new(Body::from(bytes));
                let h = response.headers_mut();
                h.insert_content_type_octet_stream();
                h.insert_cache_control(&params.cache_control);
                if let Some(etag) = &params.etag {
                    h.insert_etag(etag);
                }
                response
            }
            Err(e) => ResultExtended::<T>::to_bytes_response(Err(e), params.etag_str()),
        }
    }

    /// Check server-side cache, compute on miss
    pub async fn get_or_insert(
        &self,
        cache_key: &str,
        compute: impl Future<Output = brk_error::Result<Bytes>>,
    ) -> brk_error::Result<Bytes> {
        let guard_res = self
            .cache
            .get_value_or_guard(cache_key, Some(Duration::from_millis(50)));

        if let GuardResult::Value(bytes) = guard_res {
            return Ok(bytes);
        }

        let bytes = compute.await?;

        if let GuardResult::Guard(g) = guard_res {
            let _ = g.insert(bytes.clone());
        }

        Ok(bytes)
    }
}
