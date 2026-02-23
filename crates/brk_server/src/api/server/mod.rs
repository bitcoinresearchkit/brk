use std::{borrow::Cow, fs, path};

use aide::axum::{ApiRouter, routing::get_with};
use axum::{extract::State, http::{HeaderMap, Uri}};
use brk_types::{DiskUsage, Health, Height, SyncStatus};
use vecdb::ReadableVec;

use crate::{CacheStrategy, extended::TransformResponseExtended};

use super::AppState;

pub trait ServerRoutes {
    fn add_server_routes(self) -> Self;
}

impl ServerRoutes for ApiRouter<AppState> {
    fn add_server_routes(self) -> Self {
        self.api_route(
            "/api/server/sync",
            get_with(
                async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                    let tip_height = state.client.get_last_height();

                    state
                        .cached_json(&headers, CacheStrategy::Height, &uri, move |q| {
                            let indexed_height = q.height();
                            let tip_height = tip_height?;
                            let blocks_behind = Height::from(tip_height.saturating_sub(*indexed_height));
                            let last_indexed_at_unix = q
                                .indexer()
                                .vecs
                                .blocks
                                .timestamp
                                .collect_one(indexed_height).unwrap();

                            Ok(SyncStatus {
                                indexed_height,
                                tip_height,
                                blocks_behind,
                                last_indexed_at: last_indexed_at_unix.to_iso8601(),
                                last_indexed_at_unix,
                            })
                        })
                        .await
                },
                |op| {
                    op.id("get_sync_status")
                        .server_tag()
                        .summary("Sync status")
                        .description(
                            "Returns the sync status of the indexer, including indexed height, \
                            tip height, blocks behind, and last indexed timestamp.",
                        )
                        .ok_response::<SyncStatus>()
                        .not_modified()
                },
            ),
        )
        .api_route(
            "/api/server/disk",
            get_with(
                async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                    let brk_path = state.data_path.clone();
                    state
                        .cached_json(&headers, CacheStrategy::Height, &uri, move |q| {
                            let brk_bytes = dir_size(&brk_path)?;
                            let bitcoin_bytes = dir_size(q.blocks_dir())?;
                            Ok(DiskUsage::new(brk_bytes, bitcoin_bytes))
                        })
                        .await
                },
                |op| {
                    op.id("get_disk_usage")
                        .server_tag()
                        .summary("Disk usage")
                        .description(
                            "Returns the disk space used by BRK and Bitcoin data.",
                        )
                        .ok_response::<DiskUsage>()
                        .not_modified()
                },
            ),
        )
        .api_route(
            "/health",
            get_with(
                async |State(state): State<AppState>| -> axum::Json<Health> {
                    let uptime = state.started_instant.elapsed();
                    axum::Json(Health {
                        status: Cow::Borrowed("healthy"),
                        service: Cow::Borrowed("brk"),
                        timestamp: jiff::Timestamp::now().to_string(),
                        started_at: state.started_at.to_string(),
                        uptime_seconds: uptime.as_secs(),
                    })
                },
                |op| {
                    op.id("get_health")
                        .server_tag()
                        .summary("Health check")
                        .description("Returns the health status of the API server, including uptime information.")
                        .ok_response::<Health>()
                },
            ),
        )
        .api_route(
            "/version",
            get_with(
                async |uri: Uri, headers: HeaderMap, State(state): State<AppState>| {
                    state
                        .cached_json(&headers, CacheStrategy::Static, &uri, |_| {
                            Ok(env!("CARGO_PKG_VERSION"))
                        })
                        .await
                },
                |op| {
                    op.id("get_version")
                        .server_tag()
                        .summary("API version")
                        .description("Returns the current version of the API server")
                        .ok_response::<String>()
                        .not_modified()
                },
            ),
        )
    }
}

#[cfg(unix)]
fn dir_size(path: &path::Path) -> brk_error::Result<u64> {
    use std::os::unix::fs::MetadataExt;

    let mut total = 0u64;

    if path.is_file() {
        // blocks * 512 = actual disk usage (accounts for sparse files)
        return Ok(fs::metadata(path)?.blocks() * 512);
    }

    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            total += dir_size(&path)?;
        } else {
            total += fs::metadata(&path)?.blocks() * 512;
        }
    }

    Ok(total)
}

#[cfg(not(unix))]
fn dir_size(path: &path::Path) -> brk_error::Result<u64> {
    let mut total = 0u64;

    if path.is_file() {
        return Ok(fs::metadata(path)?.len());
    }

    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            total += dir_size(&path)?;
        } else {
            total += fs::metadata(&path)?.len();
        }
    }

    Ok(total)
}
