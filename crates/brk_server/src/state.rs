use std::{ops::Deref, path::PathBuf, sync::Arc};

use axum::{
    body::{Body, Bytes},
    http::{HeaderMap, Response},
};
use brk_query::AsyncQuery;
use quick_cache::sync::Cache;
use serde::Serialize;

use crate::{
    CacheParams, CacheStrategy,
    extended::{ResponseExtended, ResultExtended},
};

#[derive(Clone)]
pub struct AppState {
    pub query: AsyncQuery,
    pub path: Option<PathBuf>,
    pub cache: Arc<Cache<String, Bytes>>,
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
        headers: &HeaderMap,
        strategy: CacheStrategy,
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
        match self.run(f).await {
            Ok(value) => ResponseExtended::new_json_cached(&value, &params),
            Err(e) => ResultExtended::<T>::to_json_response(Err(e), params.etag_str()),
        }
    }

    /// Text response with caching
    pub async fn cached_text<T, F>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
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
        match self.run(f).await {
            Ok(value) => ResponseExtended::new_text_cached(value.as_ref(), &params),
            Err(e) => ResultExtended::<T>::to_text_response(Err(e), params.etag_str()),
        }
    }

    /// Binary response with caching
    pub async fn cached_bytes<T, F>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
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
        match self.run(f).await {
            Ok(value) => ResponseExtended::new_bytes_cached(value.into(), &params),
            Err(e) => ResultExtended::<T>::to_bytes_response(Err(e), params.etag_str()),
        }
    }
}
