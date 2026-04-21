use std::{
    future::Future,
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
}

impl AppState {
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
            if q.mempool().is_some_and(|m| m.get_txs().contains(txid)) {
                let hash = q.mempool().map(|m| m.next_block_hash()).unwrap_or(0);
                return CacheStrategy::MempoolHash(hash);
            } else if let Ok((_, height)) = q.resolve_tx(txid)
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

    /// Cached + pre-compressed response. Compression runs on the blocking thread.
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
        let encoding = ContentEncoding::negotiate(headers);
        let tip = self.sync(|q| q.tip_hash_prefix());
        if self.last_tip.swap(*tip, Ordering::Relaxed) != *tip {
            self.cache.clear();
        }
        let params = CacheParams::resolve(&strategy, || tip);
        if params.matches_etag(headers) {
            return ResponseExtended::new_not_modified_with(&params);
        }

        let full_key = format!("{}-{}-{}", uri, params.etag_str(), encoding.as_str());
        let result = self
            .get_or_insert(
                &full_key,
                async move { self.run(move |q| f(q, encoding)).await },
            )
            .await;

        match result {
            Ok(bytes) => {
                let mut response = Response::new(Body::from(bytes));
                let h = response.headers_mut();
                h.insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
                h.insert_cache_control(params.cache_control);
                h.insert_content_encoding(encoding);
                if let Some(etag) = &params.etag {
                    h.insert_etag(etag);
                }
                response
            }
            Err(e) => Error::from(e).into_response_with_etag(params.etag_str()),
        }
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
