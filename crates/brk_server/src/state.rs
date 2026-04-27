use std::{
    future::Future,
    net::SocketAddr,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};

use axum::{
    body::{Body, Bytes},
    http::{HeaderMap, HeaderValue, Response, Uri, header},
};
use brk_query::AsyncQuery;
use brk_types::{
    Addr, BlockHash, BlockHashPrefix, Date, Height, ONE_HOUR_IN_SEC, Timestamp as BrkTimestamp,
    Txid, Version,
};
use derive_more::Deref;
use jiff::Timestamp;
use quick_cache::sync::{Cache, GuardResult};
use serde::Serialize;
use vecdb::ReadableVec;

use crate::{
    CacheParams, CacheStrategy, Error, Website,
    extended::{ContentEncoding, HeaderMapExtended, ResponseExtended},
};

#[derive(Clone, Deref)]
pub struct AppState {
    #[deref]
    pub query: AsyncQuery,
    pub data_path: PathBuf,
    pub website: Website,
    pub cache: Arc<Cache<String, Bytes>>,
    pub last_tip: Arc<AtomicU64>,
    pub started_at: Timestamp,
    pub started_instant: Instant,
    pub max_weight: usize,
    pub max_weight_localhost: usize,
}

impl AppState {
    /// Per-request series weight cap: loopback gets `max_weight_localhost`,
    /// everyone else gets `max_weight`. The `connect_info_layer` rewrites the
    /// peer to non-loopback when `CF-Connecting-IP` is present, so requests
    /// proxied through a tunnel are billed at the external rate.
    pub fn max_weight_for(&self, addr: &SocketAddr) -> usize {
        if addr.ip().is_loopback() {
            self.max_weight_localhost
        } else {
            self.max_weight
        }
    }

    /// `Immutable` if height is >6 deep, `Tip` otherwise.
    pub fn height_cache(&self, version: Version, height: Height) -> CacheStrategy {
        let is_deep = self.sync(|q| (*q.height()).saturating_sub(*height) > 6);
        if is_deep {
            CacheStrategy::Immutable(version)
        } else {
            CacheStrategy::Tip
        }
    }

    /// `Immutable` if timestamp is >6 hours old (block definitely >6 deep), `Tip` otherwise.
    pub fn timestamp_cache(&self, version: Version, timestamp: BrkTimestamp) -> CacheStrategy {
        if (*BrkTimestamp::now()).saturating_sub(*timestamp) > 6 * ONE_HOUR_IN_SEC {
            CacheStrategy::Immutable(version)
        } else {
            CacheStrategy::Tip
        }
    }

    /// `Immutable` if `date` is strictly before the indexed tip's date, `Tip` otherwise.
    /// For per-date files that keep being rewritten while the tip is still within the
    /// date's day, then settle once the tip crosses the day boundary.
    pub fn date_cache(&self, version: Version, date: Date) -> CacheStrategy {
        self.sync(|q| {
            let height = q.indexed_height();
            q.indexer()
                .vecs
                .blocks
                .timestamp
                .collect_one(height)
                .map(|ts| {
                    if date < Date::from(ts) {
                        CacheStrategy::Immutable(version)
                    } else {
                        CacheStrategy::Tip
                    }
                })
                .unwrap_or(CacheStrategy::Tip)
        })
    }

    /// Smart address caching: checks mempool activity first (unless `chain_only`), then on-chain.
    /// - Address has mempool txs → `MempoolHash(addr_specific_hash)`
    /// - No mempool, has on-chain activity → `BlockBound(last_activity_block)`
    /// - Unknown address → `Tip`
    pub fn addr_cache(&self, version: Version, addr: &Addr, chain_only: bool) -> CacheStrategy {
        self.sync(|q| {
            if !chain_only {
                let mempool_hash = q.addr_mempool_hash(addr);
                if mempool_hash != 0 {
                    return CacheStrategy::MempoolHash(mempool_hash);
                }
            }
            q.addr_last_activity_height(addr)
                .and_then(|h| {
                    let block_hash = q.block_hash_by_height(h)?;
                    Ok(CacheStrategy::BlockBound(
                        version,
                        BlockHashPrefix::from(&block_hash),
                    ))
                })
                .unwrap_or(CacheStrategy::Tip)
        })
    }

    /// `Immutable` if the block is >6 deep (status stable), `Tip` otherwise.
    /// For block status which changes when the next block arrives.
    pub fn block_status_cache(&self, version: Version, hash: &BlockHash) -> CacheStrategy {
        self.sync(|q| {
            q.height_by_hash(hash)
                .map(|h| {
                    if (*q.height()).saturating_sub(*h) > 6 {
                        CacheStrategy::Immutable(version)
                    } else {
                        CacheStrategy::Tip
                    }
                })
                .unwrap_or(CacheStrategy::Tip)
        })
    }

    /// `BlockBound` if the block exists (reorg-safe via block hash), `Tip` if not found.
    pub fn block_cache(&self, version: Version, hash: &BlockHash) -> CacheStrategy {
        self.sync(|q| {
            if q.height_by_hash(hash).is_ok() {
                CacheStrategy::BlockBound(version, BlockHashPrefix::from(hash))
            } else {
                CacheStrategy::Tip
            }
        })
    }

    /// Mempool → `MempoolHash`, confirmed → `BlockBound`, unknown → `Tip`.
    pub fn tx_cache(&self, version: Version, txid: &Txid) -> CacheStrategy {
        self.sync(|q| {
            if let Some(mempool) = q.mempool()
                && mempool.txs().contains(txid)
            {
                return CacheStrategy::MempoolHash(mempool.next_block_hash());
            }
            if let Ok((_, height)) = q.resolve_tx(txid)
                && let Ok(block_hash) = q.block_hash_by_height(height)
            {
                return CacheStrategy::BlockBound(version, BlockHashPrefix::from(&block_hash));
            }
            CacheStrategy::Tip
        })
    }

    pub fn mempool_cache(&self) -> CacheStrategy {
        let hash = self.sync(|q| q.mempool().map(|m| m.next_block_hash()).unwrap_or(0));
        CacheStrategy::MempoolHash(hash)
    }

    /// Shared response pipeline: tip-clear, etag short-circuit, server-side
    /// cache lookup, body computation on a blocking thread, header assembly.
    /// Used by [`AppState::cached`] (strategy-driven) and the series endpoint
    /// (which builds [`CacheParams`] directly from query resolution).
    pub(crate) async fn cached_with_params<F>(
        &self,
        headers: &HeaderMap,
        uri: &Uri,
        params: CacheParams,
        apply_content_headers: impl FnOnce(&mut HeaderMap),
        f: F,
    ) -> Response<Body>
    where
        F: FnOnce(&brk_query::Query, ContentEncoding) -> brk_error::Result<Bytes> + Send + 'static,
    {
        let tip = self.sync(|q| q.tip_hash_prefix());
        if self.last_tip.swap(*tip, Ordering::Relaxed) != *tip {
            self.cache.clear();
        }

        if params.matches_etag(headers) {
            return ResponseExtended::new_not_modified(&params);
        }

        let encoding = ContentEncoding::negotiate(headers);
        let cache_key = format!("{}-{}-{}", uri, params.etag, encoding.as_str());
        let result = self
            .get_or_insert(&cache_key, async move {
                self.run(move |q| f(q, encoding)).await
            })
            .await;

        match result {
            Ok(bytes) => {
                let mut response = Response::new(Body::from(bytes));
                let h = response.headers_mut();
                apply_content_headers(h);
                params.apply_to(h);
                h.insert_content_encoding(encoding);
                response
            }
            Err(e) => Error::from(e).into_response_with_etag(params.etag.clone()),
        }
    }

    /// Strategy-driven cached response. Compression runs on the blocking thread.
    async fn cached<F>(
        &self,
        headers: &HeaderMap,
        strategy: CacheStrategy,
        uri: &Uri,
        content_type: &'static str,
        f: F,
    ) -> Response<Body>
    where
        F: FnOnce(&brk_query::Query, ContentEncoding) -> brk_error::Result<Bytes> + Send + 'static,
    {
        let tip = self.sync(|q| q.tip_hash_prefix());
        let params = CacheParams::resolve(&strategy, tip);
        self.cached_with_params(
            headers,
            uri,
            params,
            |h| {
                h.insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
            },
            f,
        )
        .await
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
        self.cached(headers, strategy, uri, "application/json", move |q, enc| {
            let value = f(q)?;
            Ok(enc.compress(Bytes::from(serde_json::to_vec(&value).unwrap())))
        })
        .await
    }

    /// JSON response where the strategy depends on the loaded value.
    ///
    /// Clients already holding `optimistic`'s ETag get a 304 before any work
    /// is done. Otherwise the closure runs on a blocking thread and returns
    /// both the value and the actual strategy (e.g. `Immutable` if deeply
    /// confirmed, `Tip` otherwise). Errors fall back to `Tip`. Use for
    /// resources whose freshness category depends on the data itself
    /// (outspends, threshold-based block status).
    pub async fn cached_json_optimistic<T, F>(
        &self,
        headers: &HeaderMap,
        optimistic: CacheStrategy,
        uri: &Uri,
        f: F,
    ) -> Response<Body>
    where
        T: Serialize + Send + 'static,
        F: FnOnce(&brk_query::Query) -> brk_error::Result<(T, CacheStrategy)> + Send + 'static,
    {
        let tip = self.sync(|q| q.tip_hash_prefix());
        let optimistic_params = CacheParams::resolve(&optimistic, tip);
        if optimistic_params.matches_etag(headers) {
            return ResponseExtended::new_not_modified(&optimistic_params);
        }

        let (value_result, strategy) = match self.run(f).await {
            Ok((v, s)) => (Ok(v), s),
            Err(e) => (Err(e), CacheStrategy::Tip),
        };
        let params = CacheParams::resolve(&strategy, tip);
        self.cached_with_params(
            headers,
            uri,
            params,
            |h| {
                h.insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("application/json"),
                );
            },
            move |_q, enc| {
                let value = value_result?;
                Ok(enc.compress(Bytes::from(serde_json::to_vec(&value).unwrap())))
            },
        )
        .await
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
        self.cached(headers, strategy, uri, "text/plain", move |q, enc| {
            let value = f(q)?;
            Ok(enc.compress(Bytes::from(value.as_ref().as_bytes().to_vec())))
        })
        .await
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
        self.cached(
            headers,
            strategy,
            uri,
            "application/octet-stream",
            move |q, enc| {
                let value = f(q)?;
                Ok(enc.compress(Bytes::from(value.into())))
            },
        )
        .await
    }

    /// Check server-side cache, compute on miss
    async fn get_or_insert(
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
