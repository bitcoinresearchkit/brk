//! One intentional cross-route publication/reorganization sequence.
//! Setup, raw integrity, date ranges, and benchmarks have independent fixtures.

use std::{
    net::Ipv4Addr,
    sync::{Arc, atomic::Ordering},
    time::Duration,
};

use bitcoin::{
    BlockHash as BitcoinBlockHash, Txid as BitcoinTxid,
    consensus::{encode::serialize_hex, serialize},
    hashes::Hash,
};
use bitview_plugin::UpdateContext;
use bitview_plugin_indexer::HasIndexer;
#[cfg(feature = "series")]
use bitview_query::Output;
use bitview_runtime::update;
#[cfg(feature = "series")]
use bitview_types::{DataRangeFormat, SeriesName, SeriesSelection};
use brk_error::Error as QueryError;
use brk_exit::Exit;
#[cfg(feature = "series")]
use brk_types::Index;
use brk_types::{Addr, BlockHashPrefix, PoolSlug, Txid, TxidPrefix, Vout};
#[cfg(feature = "price")]
use brk_types::{Cents, Date, Day1};
#[cfg(feature = "series")]
use serde_json::json;
use serde_json::{Value, from_slice, from_str, to_string, to_value, to_vec};
use tokio::{spawn, sync::oneshot, task::spawn_blocking, time::timeout};
#[cfg(feature = "price")]
use vecdb::ReadableOptionVec;
use vecdb::ReadableVec;

use super::{
    addr_publication::AddrPublication,
    chain_fixture::{default_first, run_genesis},
    mining, server_routes,
    server_routes::exchange_with_etag,
};
#[cfg(feature = "price")]
use super::{historical_price::HistoricalPriceChecks, oracle};
use crate::{Server, ServerConfig};

#[test]
fn address_history_preserves_exclusive_cursors_and_published_bounds() {
    run_genesis(default_first(), |mut fixture| async move {
        fixture.publish(4, 2);
        let addr = Addr::try_from(&fixture.chain[1].txdata[0].output[0].script_pubkey).unwrap();
        fixture.query.sync(|q| {
            let (output_type, type_index) = q.resolve_addr(&addr).unwrap();
            let stores = q.indexer().stores();
            let indices: Vec<_> = stores
                .addr_tx_indexes_before(
                    output_type,
                    type_index,
                    q.indexer().safe_lengths().tx_index,
                )
                .unwrap()
                .rev()
                .collect();
            assert!(indices.len() >= 2);
            // Compare the old filter contract with the bounded range, including
            // an empty publication and a cursor above the published boundary.
            for published in 0..=4u32 {
                for cursor in 0..=4u32 {
                    let expected: Vec<_> = indices
                        .iter()
                        .copied()
                        .filter(|index| *index < published.into() && *index < cursor.into())
                        .collect();
                    let actual: Vec<_> = stores
                        .addr_tx_indexes_before(
                            output_type,
                            type_index,
                            published.min(cursor).into(),
                        )
                        .unwrap()
                        .rev()
                        .collect();
                    assert_eq!(actual, expected);
                }
            }
            let txids = q.addr_txids(addr.clone(), None, usize::MAX).unwrap();
            assert_eq!(txids.len(), indices.len());
            for limit in 0..=txids.len() + 1 {
                assert_eq!(
                    q.addr_txids(addr.clone(), None, limit).unwrap(),
                    txids.iter().copied().take(limit).collect::<Vec<_>>()
                );
                for (position, cursor) in txids.iter().enumerate() {
                    assert_eq!(
                        q.addr_txids(addr.clone(), Some(*cursor), limit).unwrap(),
                        txids
                            .iter()
                            .copied()
                            .skip(position + 1)
                            .take(limit)
                            .collect::<Vec<_>>()
                    );
                }
            }
            for (position, cursor) in txids.iter().enumerate() {
                let page = q
                    .resolve_addr_chain_txs(&addr, Some(*cursor), usize::MAX)
                    .unwrap();
                if let Some(index) = indices.get(position + 1) {
                    let (_, height) = q.txid_and_height_by_index(*index).unwrap();
                    assert_eq!(
                        page.activity_anchor(),
                        q.resolve_block_hash(height).unwrap()
                    );
                } else {
                    assert_eq!(page.activity_anchor(), q.tip_blockhash());
                }
            }
            let unknown = "00".repeat(32).parse().unwrap();
            assert!(matches!(
                q.addr_txids(addr.clone(), Some(unknown), 0),
                Err(QueryError::UnknownTxid)
            ));
            assert!(matches!(
                q.resolve_addr_chain_txs(&addr, Some(unknown), usize::MAX),
                Err(QueryError::UnknownTxid)
            ));
        });
        let txids = fixture
            .query
            .sync(|q| q.addr_txids(addr.clone(), None, usize::MAX).unwrap());
        for (position, cursor) in txids.iter().enumerate() {
            let path = format!("/api/address/{addr}/txs/chain/{cursor}");
            let response = exchange_with_etag(fixture.address, "GET", &path, "\"old\"").await;
            assert!(response.starts_with("HTTP/1.1 200"), "{response}");
            let body: Vec<Value> = from_str(response.split_once("\r\n\r\n").unwrap().1).unwrap();
            let actual: Vec<_> = body.iter().map(|tx| tx["txid"].as_str().unwrap()).collect();
            let expected: Vec<_> = txids[position + 1..]
                .iter()
                .map(ToString::to_string)
                .collect();
            assert_eq!(actual, expected);
            let response = exchange_with_etag(fixture.address, "GET", &path, "*").await;
            assert!(response.starts_with("HTTP/1.1 304"), "{response}");
        }
        let path = format!("/api/address/{addr}/txs/chain/{}", "00".repeat(32));
        let response = exchange_with_etag(fixture.address, "GET", &path, "*").await;
        assert!(response.starts_with("HTTP/1.1 404"), "{response}");
    });
}

#[test]
fn reorganization_preserves_publication_and_validator_contracts() {
    run_genesis(default_first(), |mut fixture| async move {
        let query = fixture.query.clone();
        let address = fixture.address;
        let inspection_state = fixture.state.clone();
        let directory = &fixture.directory;
        let chain = fixture.chain.clone();
        let tip = fixture.tip.clone();
        let active = fixture.active.clone();
        let plugins = &mut fixture.plugins;
        #[cfg(feature = "series")]
        let version_etag = {
            let version = query
                .sync(|q| q.find_version(&"timestamp".into(), Index::Height))
                .unwrap()
                .unwrap();
            let expected = format!("W/\"sv1-{version}\"");
            for prefix in ["series"] {
                let response = exchange_with_etag(
                    address,
                    "GET",
                    &format!("/api/{prefix}/timestamp/height/version"),
                    "\"old-deploy-tag\"",
                )
                .await;
                assert!(response.starts_with("HTTP/1.1 200"), "{response}");
                assert_eq!(
                    response.split_once("\r\n\r\n").unwrap().1,
                    to_string(&version).unwrap()
                );
                assert_eq!(
                    response
                        .lines()
                        .find_map(|line| line.strip_prefix("etag: "))
                        .unwrap(),
                    expected
                );
            }
            expected
        };
        tip.store(1, Ordering::SeqCst);
        #[cfg(feature = "series")]
        let initial_len_etag = {
            let response = exchange_with_etag(
                address,
                "GET",
                "/api/series/timestamp/height/len",
                "\"old\"",
            )
            .await;
            assert!(response.starts_with("HTTP/1.1 200"), "{response}");
            assert_eq!(response.split_once("\r\n\r\n").unwrap().1, "1");
            response
                .lines()
                .find_map(|line| line.strip_prefix("etag: "))
                .unwrap()
                .to_owned()
        };
        update(plugins, UpdateContext::new(&Exit::default())).unwrap();
        #[cfg(feature = "price")]
        query.sync(|query| {
            // A successful publication must expose historical seeds through
            // the existing read-only query, not just the compute-side buffer.
            assert_eq!(
                query
                    .plugins()
                    .price
                    .spot
                    .cents
                    .height
                    .collect_range_at(0, 2),
                vec![Cents::ZERO; 2]
            );
            let day = Day1::try_from(Date::new(2009, 1, 3)).unwrap();
            assert_eq!(
                query
                    .plugins()
                    .price
                    .split
                    .close
                    .cents
                    .day1
                    .collect_one_flat(day),
                Some(Cents::ZERO)
            );
        });
        server_routes::check_recent_blocks(&inspection_state, address).await;
        #[cfg(feature = "price")]
        oracle::check(&inspection_state, address).await;
        #[cfg(feature = "price")]
        let mut historical_prices =
            HistoricalPriceChecks::before(&inspection_state, address, chain[1].header.time).await;
        mining::check_pool_blocks(&inspection_state, address).await;
        let mining_validators = mining::check_statistics(address).await;
        let parent_txid = Txid::from(chain[1].txdata[0].compute_txid());
        let mut address_publication =
            AddrPublication::start(plugins, directory.path(), &chain[1]).await;
        let utxo_addr = Addr::try_from(&chain[1].txdata[0].output[0].script_pubkey).unwrap();
        let utxo_path = format!("/api/address/{utxo_addr}/utxo");
        let utxo_response = exchange_with_etag(address, "GET", &utxo_path, "\"old\"").await;
        assert!(utxo_response.starts_with("HTTP/1.1 200"), "{utxo_response}");
        let utxo_body: Value = from_str(utxo_response.split_once("\r\n\r\n").unwrap().1).unwrap();
        assert!(!utxo_body.as_array().unwrap().is_empty());
        let native_utxos = query
            .sync(|q| q.addr_utxos(utxo_addr.clone(), 1000))
            .unwrap();
        assert_eq!(utxo_body, to_value(&native_utxos).unwrap());
        assert!(
            query
                .sync(|q| q.resolve_addr_utxos(&utxo_addr, 0))
                .is_err_and(|e| matches!(e, QueryError::TooManyUtxos))
        );
        let utxo_etag = utxo_response
            .lines()
            .find_map(|line| line.strip_prefix("etag: "))
            .unwrap()
            .to_owned();
        let limited = Server::bind(
            &query,
            ServerConfig {
                bind: Ipv4Addr::LOCALHOST.into(),
                port: 0.into(),
                max_utxos: 0,
                data_path: directory.path().to_owned(),
                ..ServerConfig::default()
            },
        )
        .await
        .unwrap();
        let limited_address = limited.listener.local_addr().unwrap();
        let limited_serving = spawn(limited.serve());
        for method in ["GET", "HEAD"] {
            for tag in [utxo_etag.as_str(), "*"] {
                let response = exchange_with_etag(address, method, &utxo_path, tag).await;
                assert!(response.starts_with("HTTP/1.1 304"), "{response}");
                assert!(response.ends_with("\r\n\r\n"));
                let response = exchange_with_etag(limited_address, method, &utxo_path, tag).await;
                assert!(response.starts_with("HTTP/1.1 400"), "{response}");
                assert!(!response.contains("\r\netag:"));
            }
        }
        limited_serving.abort();
        server_routes::check_tip_cached_routes(address).await;
        let response = exchange_with_etag(address, "GET", "/api/server/sync", "\"old\"").await;
        assert!(response.starts_with("HTTP/1.1 200"), "{response}");
        let before: Value = from_str(response.split_once("\r\n\r\n").unwrap().1).unwrap();
        assert_eq!(before["indexed_height"], 1);
        assert_eq!(before["last_indexed_at_unix"], chain[1].header.time);
        #[cfg(feature = "series")]
        let resolved_before_reorg = query
            .run(|query| {
                let params = SeriesSelection::from((
                    Index::Height,
                    SeriesName::from("timestamp"),
                    DataRangeFormat::default().set_start(1).set_end(2),
                ));
                query.resolve(params, usize::MAX)
            })
            .await
            .unwrap();
        #[cfg(feature = "series")]
        assert!(
            resolved_before_reorg.stable_count.is_some(),
            "guarding an append-style series must not mark all of its history mutable"
        );
        #[cfg(feature = "series")]
        let data_etag = {
            let response = exchange_with_etag(
                address,
                "GET",
                "/api/series/timestamp/height?start=1&end=2",
                "\"old\"",
            )
            .await;
            assert!(response.starts_with("HTTP/1.1 200"), "{response}");
            response
                .lines()
                .find_map(|line| line.strip_prefix("etag: "))
                .unwrap()
                .to_owned()
        };
        #[cfg(feature = "series")]
        let mut bulk_before = Vec::new();
        #[cfg(feature = "series")]
        for names in ["timestamp,timestamp", "timestamp,timestamp_monotonic"] {
            for format in ["json", "csv"] {
                let path = format!(
                    "/api/series/bulk?series={names}&index=height&start=1&end=2&format={format}"
                );
                let response = exchange_with_etag(address, "GET", &path, "\"old\"").await;
                assert!(response.starts_with("HTTP/1.1 200"), "{response}");
                let etag = response
                    .lines()
                    .find_map(|line| line.strip_prefix("etag: "))
                    .unwrap()
                    .to_owned();
                bulk_before.push((names, format, path, etag));
            }
        }
        #[cfg(feature = "series")]
        let latest_etag = {
            let response = exchange_with_etag(
                address,
                "GET",
                "/api/series/timestamp/height/latest",
                "\"old\"",
            )
            .await;
            assert!(response.starts_with("HTTP/1.1 200"), "{response}");
            let value: Value = from_str(response.split_once("\r\n\r\n").unwrap().1).unwrap();
            assert_eq!(value, chain[1].header.time);
            response
                .lines()
                .find_map(|line| line.strip_prefix("etag: "))
                .unwrap()
                .to_owned()
        };
        #[cfg(feature = "series")]
        let len_etag = {
            let response = exchange_with_etag(
                address,
                "GET",
                "/api/series/timestamp/height/len",
                &initial_len_etag,
            )
            .await;
            assert!(response.starts_with("HTTP/1.1 200"), "{response}");
            assert_eq!(response.split_once("\r\n\r\n").unwrap().1, "2");
            response
                .lines()
                .find_map(|line| line.strip_prefix("etag: "))
                .unwrap()
                .to_owned()
        };
        let etag = response
            .lines()
            .find_map(|line| line.strip_prefix("etag: "))
            .unwrap()
            .to_owned();
        let old_hash = chain[1].block_hash().into();
        let real_txid = chain[1].txdata[0].compute_txid();
        let mut transaction_validators = Vec::new();
        for suffix in ["", "/hex", "/merkleblock-proof", "/merkle-proof", "/status"] {
            let path = format!("/api/tx/{real_txid}{suffix}");
            let response = exchange_with_etag(address, "GET", &path, "\"old\"").await;
            assert!(response.starts_with("HTTP/1.1 200"), "{response}");
            assert!(!response.contains("immutable"));
            let etag = response
                .lines()
                .find_map(|line| line.strip_prefix("etag: "))
                .unwrap()
                .to_owned();
            for method in ["GET", "HEAD"] {
                let response = exchange_with_etag(address, method, &path, &etag).await;
                assert!(response.starts_with("HTTP/1.1 304"), "{response}");
            }
            transaction_validators.push((path, etag));
        }
        let mut collision = real_txid.to_byte_array();
        collision[31] ^= 1;
        let collision: Txid = BitcoinTxid::from_byte_array(collision).into();
        assert_eq!(
            TxidPrefix::from(collision),
            TxidPrefix::from(Txid::from(real_txid))
        );
        query
            .run(move |q| {
                assert!(matches!(
                    q.resolve_transaction(&collision),
                    Err(QueryError::UnknownTxid)
                ));
                assert!(matches!(
                    q.resolve_raw_transaction(&collision)
                        .and_then(|resolved| q.transaction_raw_resolved(resolved)),
                    Err(QueryError::UnknownTxid)
                ));
                assert!(matches!(
                    q.resolve_raw_transaction(&collision)
                        .and_then(|resolved| q.transaction_hex_resolved(resolved)),
                    Err(QueryError::UnknownTxid)
                ));
                assert!(matches!(
                    q.outspends(&collision),
                    Err(QueryError::UnknownTxid)
                ));
                assert!(matches!(
                    q.resolve_confirmed_tx(&collision)
                        .and_then(|resolved| q.merkle_proof_resolved(resolved)),
                    Err(QueryError::UnknownTxid)
                ));
                Ok(())
            })
            .await
            .unwrap();
        let mut block_tx_validators = Vec::new();
        for suffix in ["txids", "txid/0", "txs", "txs/0"] {
            let path = format!("/api/block/{old_hash}/{suffix}");
            let response = exchange_with_etag(address, "GET", &path, "\"old\"").await;
            assert!(response.starts_with("HTTP/1.1 200"), "{response}");
            assert!(!response.contains("immutable"));
            let etag = response
                .lines()
                .find_map(|line| line.strip_prefix("etag: "))
                .unwrap()
                .to_owned();
            for method in ["GET", "HEAD"] {
                let response = exchange_with_etag(address, method, &path, &etag).await;
                assert!(response.starts_with("HTTP/1.1 304"), "{response}");
            }
            block_tx_validators.push((path, etag));
        }
        for suffix in ["txid/4294967295", "txs/4294967295"] {
            for method in ["GET", "HEAD"] {
                let path = format!("/api/block/{old_hash}/{suffix}");
                let response = exchange_with_etag(address, method, &path, "*").await;
                assert!(
                    response.starts_with("HTTP/1.1 404"),
                    "invalid offset must not validate: {response}"
                );
            }
        }
        let mut tip_validators = Vec::new();
        for (kind, expected) in [
            ("height", "1".to_owned()),
            ("hash", chain[1].block_hash().to_string()),
        ] {
            let path = format!("/api/blocks/tip/{kind}");
            let response = exchange_with_etag(address, "GET", &path, "\"old\"").await;
            assert!(response.starts_with("HTTP/1.1 200"), "{response}");
            assert_eq!(response.split_once("\r\n\r\n").unwrap().1, expected);
            let etag = response
                .lines()
                .find_map(|line| line.strip_prefix("etag: "))
                .unwrap()
                .to_owned();
            for method in ["GET", "HEAD"] {
                let response = exchange_with_etag(address, method, &path, &etag).await;
                assert!(response.starts_with("HTTP/1.1 304"), "{response}");
                assert!(response.ends_with("\r\n\r\n"));
            }
            tip_validators.push((kind, path, etag));
        }
        let status_path = format!("/api/block/{}/status", chain[0].block_hash());
        let status = exchange_with_etag(address, "GET", &status_path, "\"old\"").await;
        assert!(status.starts_with("HTTP/1.1 200"), "{status}");
        assert!(
            !status.contains("immutable"),
            "block status remains mutable at every depth"
        );
        let status_body: Value = from_str(status.split_once("\r\n\r\n").unwrap().1).unwrap();
        assert_eq!(status_body["next_best"], chain[1].block_hash().to_string());
        let status_etag = status
            .lines()
            .find_map(|line| line.strip_prefix("etag: "))
            .unwrap()
            .to_owned();
        for method in ["GET", "HEAD"] {
            let response = exchange_with_etag(address, method, &status_path, &status_etag).await;
            assert!(response.starts_with("HTTP/1.1 304"), "{response}");
            assert!(response.ends_with("\r\n\r\n"));
            let response =
                exchange_with_etag(address, method, &format!("{status_path}?unexpected=1"), "*")
                    .await;
            assert!(response.starts_with("HTTP/1.1 400"), "{response}");
        }
        let timestamp_path = "/api/v1/mining/blocks/timestamp/4294967295";
        let timestamp_response =
            exchange_with_etag(address, "GET", timestamp_path, "\"old\"").await;
        assert!(
            timestamp_response.starts_with("HTTP/1.1 200"),
            "{timestamp_response}"
        );
        let timestamp_etag = timestamp_response
            .lines()
            .find_map(|line| line.strip_prefix("etag: "))
            .unwrap()
            .to_owned();
        let height_response =
            exchange_with_etag(address, "GET", "/api/block-height/1", "\"old\"").await;
        assert!(
            height_response.starts_with("HTTP/1.1 200"),
            "{height_response}"
        );
        let height_etag = height_response
            .lines()
            .find_map(|line| line.strip_prefix("etag: "))
            .unwrap()
            .to_owned();
        let base_path = format!("/api/block/{old_hash}");
        let base = exchange_with_etag(address, "GET", &base_path, "\"old\"").await;
        assert!(base.starts_with("HTTP/1.1 200"), "{base}");
        let base_etag = base
            .lines()
            .find_map(|line| line.strip_prefix("etag: "))
            .unwrap()
            .to_owned();
        let header_path = format!("{base_path}/header");
        let header = exchange_with_etag(address, "GET", &header_path, "\"old\"").await;
        assert!(header.starts_with("HTTP/1.1 200"), "{header}");
        let header_etag = header
            .lines()
            .find_map(|line| line.strip_prefix("etag: "))
            .unwrap()
            .to_owned();
        let raw_path = format!("{base_path}/raw");
        let raw = exchange_with_etag(address, "HEAD", &raw_path, "\"old\"").await;
        assert!(raw.starts_with("HTTP/1.1 200"), "{raw}");
        let raw_etag = raw
            .lines()
            .find_map(|line| line.strip_prefix("etag: "))
            .unwrap()
            .to_owned();
        let single_path = format!("/api/v1/block/{old_hash}");
        let single = exchange_with_etag(address, "GET", &single_path, "\"old\"").await;
        assert!(single.starts_with("HTTP/1.1 200"), "{single}");
        let single_etag = single
            .lines()
            .find_map(|line| line.strip_prefix("etag: "))
            .unwrap()
            .to_owned();
        let v1_response = exchange_with_etag(address, "GET", "/api/v1/blocks", "\"old\"").await;
        assert!(v1_response.starts_with("HTTP/1.1 200"), "{v1_response}");
        let v1_etag = v1_response
            .lines()
            .find_map(|line| line.strip_prefix("etag: "))
            .unwrap()
            .to_owned();
        let historical_v1 = exchange_with_etag(address, "GET", "/api/v1/blocks/0", "\"old\"").await;
        let historical_v1_etag = historical_v1
            .lines()
            .find_map(|line| line.strip_prefix("etag: "))
            .unwrap()
            .to_owned();
        let height_v1 = exchange_with_etag(address, "GET", "/api/v1/blocks/1", "\"old\"").await;
        let height_v1_etag = height_v1
            .lines()
            .find_map(|line| line.strip_prefix("etag: "))
            .unwrap()
            .to_owned();
        let mut wrong_hash = chain[1].block_hash().to_byte_array();
        wrong_hash[31] ^= 1;
        let wrong_hash = BitcoinBlockHash::from_byte_array(wrong_hash).into();
        assert_eq!(
            BlockHashPrefix::from(old_hash),
            BlockHashPrefix::from(wrong_hash)
        );
        query
            .sync(|q| {
                let resolved = q.resolve_block_snapshot(&old_hash)?;
                assert!(
                    matches!(
                        q.resolve_block_snapshot(&wrong_hash),
                        Err(QueryError::NotFound(_))
                    ),
                    "a matching prefix is not an exact block hash"
                );
                assert!(
                    q.try_resolve_block_snapshot(&wrong_hash, resolved.last_height().unwrap())?
                        .is_none(),
                    "a height hint must match the full hash"
                );
                assert_eq!(resolved.build(q)?.pop().unwrap().id, old_hash);
                Ok::<_, QueryError>(())
            })
            .unwrap();
        let recent_v1 = query
            .run(|q| {
                let prices = &q.plugins().price.spot.cents.height;
                let timestamps = &q.indexer().vecs().blocks.timestamp;
                timestamps.invalidate();
                prices.invalidate();
                assert!(q.try_resolve_blocks_v1(None, 15)?.is_none());
                let bounded = q.resolve_blocks_v1(None, 15)?;
                assert!(
                    prices.cached_snapshot().is_none(),
                    "bounded resolution must not retain history"
                );
                let captured = bounded.prices().iter().rev().copied().collect::<Vec<_>>();
                let rows = bounded.build(q)?;
                assert!(
                    timestamps.cached_snapshot().is_none(),
                    "V1 body must not fill timestamp history"
                );
                assert_eq!(
                    rows.iter().map(|row| row.extras.price).collect::<Vec<_>>(),
                    captured
                );
                assert!(
                    prices.cached_snapshot().is_none(),
                    "building captured rows must not read the cached price source"
                );
                assert_eq!(q.resolve_blocks(None, 10)?.build(q)?.len(), rows.len());
                assert!(
                    timestamps.cached_snapshot().is_none(),
                    "block bodies must not fill timestamp history"
                );
                let warm = timestamps.snapshot();
                assert_eq!(
                    to_vec(
                        &q.resolve_blocks_v1(None, 15)
                            .and_then(|resolved| resolved.build(q))?
                    )
                    .unwrap(),
                    to_vec(&rows).unwrap()
                );
                assert!(
                    Arc::ptr_eq(&warm, &timestamps.cached_snapshot().unwrap()),
                    "body reads must reuse an existing timestamp snapshot"
                );
                drop(prices.snapshot());
                q.try_resolve_blocks_v1(None, 15)?
                    .ok_or(QueryError::Internal("expected warm V1 snapshot"))
            })
            .await
            .unwrap();
        let header_snapshot = query.sync(|q| q.resolve_block_snapshot(&old_hash)).unwrap();
        let pool_snapshot = query
            .sync(|q| q.resolve_pool_blocks(PoolSlug::Unknown, None, 100))
            .unwrap();
        let pool_path = "/api/v1/mining/pool/unknown/blocks";
        let pool_response = exchange_with_etag(address, "GET", pool_path, "\"old\"").await;
        assert!(pool_response.starts_with("HTTP/1.1 200"), "{pool_response}");
        let pool_etag = pool_response
            .lines()
            .find_map(|line| line.strip_prefix("etag: "))
            .unwrap()
            .to_owned();
        let raw_snapshot = query.sync(|q| q.resolve_block_snapshot(&old_hash)).unwrap();
        let utxo_snapshot = query
            .sync(|q| q.resolve_addr_utxos(&utxo_addr, 1000))
            .unwrap();
        let gate = plugins.indexer().publication().clone();
        let mut closing = spawn_blocking(move || gate.begin_update());
        #[cfg(feature = "series")]
        {
            assert!(
                timeout(Duration::from_millis(100), &mut closing)
                    .await
                    .is_err(),
                "publication must wait for the resolved read"
            );
            let output = query
                .run(move |query| query.format(resolved_before_reorg))
                .await
                .unwrap();
            let Output::Json(bytes) = output.output else {
                panic!("expected JSON");
            };
            let body: Value = from_slice(&bytes).unwrap();
            assert_eq!(body["data"][0], chain[1].header.time);
        }
        assert!(
            timeout(Duration::from_millis(50), &mut closing)
                .await
                .is_err(),
            "V1 snapshot must retain its publication guard"
        );
        let old_v1 = query.run(move |q| recent_v1.build(q)).await.unwrap();
        assert_eq!(
            old_v1[0].info.id.to_string(),
            chain[1].block_hash().to_string()
        );
        assert_eq!(
            *old_v1[0].info.median_time,
            chain[0].header.time.max(chain[1].header.time)
        );
        assert!(
            timeout(Duration::from_millis(50), &mut closing)
                .await
                .is_err(),
            "header snapshot must retain publication exclusion"
        );
        let old_header = query
            .run(move |q| header_snapshot.anchor_header_hex(q))
            .await
            .unwrap();
        assert_eq!(old_header, serialize_hex(&chain[1].header));
        assert!(
            timeout(Duration::from_millis(50), &mut closing)
                .await
                .is_err(),
            "raw snapshot must retain publication exclusion"
        );
        let old_raw = query
            .run(move |q| raw_snapshot.anchor_raw(q))
            .await
            .unwrap();
        assert_eq!(old_raw, serialize(&chain[1]));
        assert!(
            timeout(Duration::from_millis(50), &mut closing)
                .await
                .is_err(),
            "pool snapshot must retain publication exclusion"
        );
        let old_pool = query
            .run(move |q| q.pool_blocks_resolved(pool_snapshot))
            .await
            .unwrap();
        assert_eq!(
            old_pool[0].info.id.to_string(),
            chain[1].block_hash().to_string()
        );
        // UTXO bodies use the captured outpoints and immutable prefix only.
        let old_utxos = query
            .run(move |q| q.addr_utxos_resolved(utxo_snapshot, 1000))
            .await
            .unwrap()
            .0;
        assert_eq!(to_value(old_utxos).unwrap(), utxo_body);
        // Mixed pages retain only prefix pins; they do not delay append publication.
        address_publication.consume_snapshot().await;
        timeout(Duration::from_secs(1), closing)
            .await
            .unwrap()
            .unwrap();
        #[cfg(feature = "price")]
        historical_prices.during_reorg(address).await;
        active.store(2, Ordering::SeqCst);
        let prevout_query = query.clone();
        let pending_prevouts = move || {
            spawn(async move {
                prevout_query
                    .run(move |q| {
                        Ok(q.indexer_prevout_resolver()(&[(
                            parent_txid,
                            Vout::from(0u16),
                        )]))
                    })
                    .await
            })
        };
        let mut pending_utxos = Vec::new();
        for method in ["GET", "HEAD"] {
            let path = utxo_path.clone();
            let tag = utxo_etag.clone();
            let mut task =
                spawn(async move { exchange_with_etag(address, method, &path, &tag).await });
            assert!(
                timeout(Duration::from_millis(10), &mut task).await.is_err(),
                "UTXO revalidation must wait for publication"
            );
            pending_utxos.push(task);
        }
        let mut pending_mining = Vec::new();
        for (path, tag) in mining_validators {
            for method in ["GET", "HEAD"] {
                let tag = tag.clone();
                let old_tag = tag.clone();
                let task = move || {
                    spawn(async move { exchange_with_etag(address, method, path, &tag).await })
                };
                pending_mining.push((path, method, old_tag, task));
            }
        }
        let mut pending_transactions = Vec::new();
        for (path, etag) in transaction_validators {
            pending_transactions.push(move || {
                spawn(async move { exchange_with_etag(address, "GET", &path, &etag).await })
            });
        }
        // Immutable prefix reads may complete before rollback starts. Keep
        // these old tokens/validators, then exercise them after the reorg;
        // safe_prefix separately checks reads during ordinary append.
        let mut after_reorg_block_txs = Vec::new();
        for (path, etag) in block_tx_validators {
            after_reorg_block_txs
                .push(async move { exchange_with_etag(address, "GET", &path, &etag).await });
        }
        let after_reorg_status_path = status_path.clone();
        let mut after_reorg_tips = Vec::new();
        for (kind, path, etag) in tip_validators {
            after_reorg_tips.push((kind, async move {
                exchange_with_etag(address, "GET", &path, &etag).await
            }));
        }
        let after_reorg_status = async move {
            exchange_with_etag(address, "GET", &after_reorg_status_path, &status_etag).await
        };
        let after_reorg_raw = {
            let query = query.clone();
            async move {
                query
                    .run(move |q| q.resolve_block_snapshot(&old_hash)?.anchor_raw(q))
                    .await
            }
        };
        let mut after_reorg_raw_http = Vec::new();
        for method in ["GET", "HEAD"] {
            let path = raw_path.clone();
            let tag = raw_etag.clone();
            after_reorg_raw_http
                .push(async move { exchange_with_etag(address, method, &path, &tag).await });
        }
        let mut pending_timestamp_http = Vec::new();
        let mut pending_pool_http = Vec::new();
        for method in ["GET", "HEAD"] {
            let tag = pool_etag.clone();
            let mut task =
                spawn(async move { exchange_with_etag(address, method, pool_path, &tag).await });
            assert!(
                timeout(Duration::from_millis(50), &mut task).await.is_err(),
                "pool validation must wait for publication"
            );
            pending_pool_http.push((method, task));
        }
        for method in ["GET", "HEAD"] {
            let tag = timestamp_etag.clone();
            let task = move || {
                spawn(
                    async move { exchange_with_etag(address, method, timestamp_path, &tag).await },
                )
            };
            pending_timestamp_http.push((method, task));
        }
        assert_eq!(
            query
                .sync(|q| q.try_resolve_block_hash(1u32.into()))
                .unwrap(),
            Some(old_hash),
            "the old immutable prefix remains available before rollback"
        );
        let mut after_reorg_height_http = Vec::new();
        for method in ["GET", "HEAD"] {
            let tag = height_etag.clone();
            after_reorg_height_http.push((method, async move {
                exchange_with_etag(address, method, "/api/block-height/1", &tag).await
            }));
        }
        let after_reorg_headers = {
            let query = query.clone();
            async move {
                query
                    .run(move |q| q.resolve_block_snapshot(&old_hash)?.anchor_header_hex(q))
                    .await
            }
        };
        let mut after_reorg_header_http = Vec::new();
        for method in ["GET", "HEAD"] {
            let path = header_path.clone();
            let tag = header_etag.clone();
            after_reorg_header_http
                .push(async move { exchange_with_etag(address, method, &path, &tag).await });
        }
        let mut after_reorg_base = Vec::new();
        for mode in 0..2 {
            let query = query.clone();
            after_reorg_base.push((mode, async move {
                query
                    .run(move |q| match mode {
                        1 => q
                            .resolve_blocks(Some(1u32.into()), 1)
                            .and_then(|resolved| resolved.build(q))
                            .map(|mut rows| rows.pop().unwrap()),
                        _ => q
                            .resolve_block_snapshot(&old_hash)?
                            .build(q)
                            .map(|mut rows| rows.pop().unwrap()),
                    })
                    .await
            }));
        }
        let after_reorg_base_http =
            async move { exchange_with_etag(address, "GET", &base_path, &base_etag).await };
        let mut pending_v1 = Vec::new();
        for mode in 0..3 {
            let query = query.clone();
            let (started, ready) = oneshot::channel();
            let mut task = spawn(async move {
                query
                    .run(move |q| {
                        let _ = started.send(());
                        match mode {
                            0 => q
                                .resolve_blocks_v1(None, 15)
                                .and_then(|resolved| resolved.build(q)),
                            1 => q
                                .resolve_blocks_v1(Some(1u32.into()), 1)
                                .and_then(|resolved| resolved.build(q)),
                            _ => q.resolve_block_v1(&old_hash)?.build(q),
                        }
                    })
                    .await
            });
            ready.await.unwrap();
            assert!(
                timeout(Duration::from_millis(50), &mut task).await.is_err(),
                "V1 body reads must wait for publication"
            );
            pending_v1.push((mode, task));
        }
        let blocks_etag = format!("W/\"blocks2-{}\"", chain[1].block_hash());
        let mut pending_v1_http =
            spawn(
                async move { exchange_with_etag(address, "GET", "/api/v1/blocks", &v1_etag).await },
            );
        assert!(
            timeout(Duration::from_millis(100), &mut pending_v1_http)
                .await
                .is_err(),
            "V1 validation must wait for publication"
        );
        let mut pending_single_v1 =
            spawn(
                async move { exchange_with_etag(address, "GET", &single_path, &single_etag).await },
            );
        assert!(
            timeout(Duration::from_millis(50), &mut pending_single_v1)
                .await
                .is_err(),
            "single-block validation must wait for publication"
        );
        let mut pending_height_v1 = spawn(async move {
            exchange_with_etag(address, "GET", "/api/v1/blocks/1", &height_v1_etag).await
        });
        assert!(
            timeout(Duration::from_millis(50), &mut pending_height_v1)
                .await
                .is_err(),
            "height V1 validation must wait for publication"
        );
        let height_blocks_etag = blocks_etag.clone();
        let after_reorg_height_blocks = async move {
            exchange_with_etag(address, "GET", "/api/blocks/1", &height_blocks_etag).await
        };
        let after_reorg_blocks =
            async move { exchange_with_etag(address, "GET", "/api/blocks", &blocks_etag).await };
        // Publication waits retain worker admission. Check the sync endpoint
        // after publishing, once the queued mutable readers can finish.
        let after_reorg_sync =
            async move { exchange_with_etag(address, "GET", "/api/server/sync", &etag).await };
        #[cfg(feature = "series")]
        let pending_data = move || {
            spawn(async move {
                exchange_with_etag(
                    address,
                    "GET",
                    "/api/series/timestamp/height?start=1&end=2",
                    &data_etag,
                )
                .await
            })
        };
        #[cfg(feature = "series")]
        let pending_latest = move || {
            spawn(async move {
                exchange_with_etag(
                    address,
                    "GET",
                    "/api/series/timestamp/height/latest",
                    &latest_etag,
                )
                .await
            })
        };
        #[cfg(feature = "series")]
        let pending_len = move || {
            spawn(async move {
                exchange_with_etag(
                    address,
                    "GET",
                    "/api/series/timestamp/height/len",
                    &len_etag,
                )
                .await
            })
        };
        #[cfg(feature = "series")]
        let mut pending_bulk = Vec::new();
        #[cfg(feature = "series")]
        for (names, format, path, etag) in bulk_before {
            let request_path = path.clone();
            let request = move || {
                spawn(async move { exchange_with_etag(address, "GET", &request_path, &etag).await })
            };
            pending_bulk.push((names, format, path, request));
        }
        update(plugins, UpdateContext::new(&Exit::default())).unwrap();
        let status = timeout(Duration::from_secs(5), after_reorg_status)
            .await
            .unwrap();
        #[cfg(feature = "price")]
        historical_prices.after(&inspection_state, address).await;
        assert!(
            timeout(Duration::from_secs(5), pending_prevouts())
                .await
                .unwrap()
                .unwrap()
                .unwrap()
                .is_empty(),
            "displaced parent must not resolve against a reused index"
        );
        address_publication.after_reorg(&chain[2]).await;
        let current_utxos = query.sync(|q| q.addr_utxos(utxo_addr.clone(), 1000));
        for request in pending_utxos {
            let response = timeout(Duration::from_secs(5), request)
                .await
                .unwrap()
                .unwrap();
            let expected_status = if current_utxos.is_ok() {
                "HTTP/1.1 200"
            } else {
                "HTTP/1.1 404"
            };
            assert!(
                response.starts_with(expected_status),
                "reorg must invalidate UTXO validator: {response}"
            );
        }
        if let Ok(current_utxos) = current_utxos {
            let response = exchange_with_etag(address, "GET", &utxo_path, &utxo_etag).await;
            let body: Value = from_str(response.split_once("\r\n\r\n").unwrap().1).unwrap();
            assert_eq!(body, to_value(current_utxos).unwrap());
        }
        for request in pending_transactions {
            let response = timeout(Duration::from_secs(5), request())
                .await
                .unwrap()
                .unwrap();
            assert!(
                response.starts_with("HTTP/1.1 404"),
                "displaced transactions must not validate: {response}"
            );
        }
        for request in after_reorg_block_txs {
            let response = timeout(Duration::from_secs(5), request).await.unwrap();
            assert!(
                response.starts_with("HTTP/1.1 404"),
                "displaced block transactions must not validate: {response}"
            );
        }
        for (kind, request) in after_reorg_tips {
            let response = timeout(Duration::from_secs(5), request).await.unwrap();
            if kind == "height" {
                assert!(
                    response.starts_with("HTTP/1.1 304"),
                    "unchanged height: {response}"
                );
            } else {
                assert!(
                    response.starts_with("HTTP/1.1 200"),
                    "changed hash: {response}"
                );
                assert_eq!(
                    response.split_once("\r\n\r\n").unwrap().1,
                    chain[2].block_hash().to_string()
                );
            }
        }
        assert!(
            status.starts_with("HTTP/1.1 200"),
            "same-height reorg must invalidate changed next_best: {status}"
        );
        let status_body: Value = from_str(status.split_once("\r\n\r\n").unwrap().1).unwrap();
        assert_eq!(status_body["next_best"], chain[2].block_hash().to_string());
        let base = timeout(Duration::from_secs(5), after_reorg_base_http)
            .await
            .unwrap();
        assert!(matches!(
            timeout(Duration::from_secs(5), after_reorg_raw)
                .await
                .unwrap(),
            Err(QueryError::NotFound(_))
        ));
        for task in after_reorg_raw_http {
            let response = timeout(Duration::from_secs(5), task).await.unwrap();
            assert!(
                response.starts_with("HTTP/1.1 404"),
                "displaced raw block must not validate or return replacement bytes: {response}"
            );
            assert!(!response.contains("\r\netag:"));
        }
        for (method, task) in pending_timestamp_http {
            let response = timeout(Duration::from_secs(5), task())
                .await
                .unwrap()
                .unwrap();
            assert!(
                response.starts_with("HTTP/1.1 200"),
                "timestamp cannot validate a displaced selection: {response}"
            );
            let tag = response
                .lines()
                .find_map(|line| line.strip_prefix("etag: "))
                .unwrap();
            assert_ne!(tag, timestamp_etag);
            let body = response.split_once("\r\n\r\n").unwrap().1;
            if method == "HEAD" {
                assert!(body.is_empty());
            } else {
                assert_eq!(
                    from_str::<Value>(body).unwrap()["hash"],
                    chain[2].block_hash().to_string()
                );
            }
        }
        for (method, task) in pending_pool_http {
            let response = timeout(Duration::from_secs(5), task)
                .await
                .unwrap()
                .unwrap();
            assert!(
                response.starts_with("HTTP/1.1 200"),
                "pool cannot validate displaced blocks: {response}"
            );
            assert_ne!(
                response
                    .lines()
                    .find_map(|line| line.strip_prefix("etag: "))
                    .unwrap(),
                pool_etag
            );
            let body = response.split_once("\r\n\r\n").unwrap().1;
            if method == "HEAD" {
                assert!(body.is_empty());
            } else {
                assert_eq!(
                    from_str::<Value>(body).unwrap()[0]["id"],
                    chain[2].block_hash().to_string()
                );
            }
        }
        for (path, method, old_tag, task) in pending_mining {
            let response = timeout(Duration::from_secs(5), task())
                .await
                .unwrap()
                .unwrap();
            let fresh = exchange_with_etag(address, "GET", path, "\"old\"").await;
            assert!(fresh.starts_with("HTTP/1.1 200"), "{path}: {fresh}");
            let tag = fresh
                .lines()
                .find_map(|line| line.strip_prefix("etag: "))
                .unwrap();
            let unchanged = tag == old_tag;
            let status = if unchanged {
                "HTTP/1.1 304"
            } else {
                "HTTP/1.1 200"
            };
            assert!(response.starts_with(status), "{path}: {response}");
            assert_eq!(
                response
                    .lines()
                    .find_map(|line| line.strip_prefix("etag: "))
                    .unwrap(),
                tag
            );
            let expected = if unchanged || method == "HEAD" {
                ""
            } else {
                fresh.split_once("\r\n\r\n").unwrap().1
            };
            assert_eq!(response.split_once("\r\n\r\n").unwrap().1, expected);
        }
        for (method, task) in after_reorg_height_http {
            let response = timeout(Duration::from_secs(5), task).await.unwrap();
            assert!(
                response.starts_with("HTTP/1.1 200"),
                "replacement height cannot validate old hash: {response}"
            );
            let tag = response
                .lines()
                .find_map(|line| line.strip_prefix("etag: "))
                .unwrap();
            assert_ne!(tag, height_etag);
            let expected = chain[2].block_hash().to_string();
            assert_eq!(
                response.split_once("\r\n\r\n").unwrap().1,
                if method == "HEAD" { "" } else { &expected }
            );
        }
        for task in after_reorg_header_http {
            let response = timeout(Duration::from_secs(5), task).await.unwrap();
            assert!(
                response.starts_with("HTTP/1.1 404"),
                "displaced header cannot validate or return replacement bytes: {response}"
            );
            assert!(!response.contains("\r\netag:"));
        }
        assert!(matches!(
            timeout(Duration::from_secs(5), after_reorg_headers)
                .await
                .unwrap(),
            Err(QueryError::NotFound(_))
        ));
        assert!(
            base.starts_with("HTTP/1.1 404"),
            "displaced base hash cannot return 304 or replacement rows: {base}"
        );
        assert!(!base.contains("\r\netag:"));
        for (mode, task) in after_reorg_base {
            let result = timeout(Duration::from_secs(5), task).await.unwrap();
            if mode == 1 {
                let row = result.unwrap();
                assert_eq!(row.id.to_string(), chain[2].block_hash().to_string());
                assert_eq!(*row.timestamp, chain[2].header.time);
            } else {
                assert!(
                    matches!(result, Err(QueryError::NotFound(_))),
                    "the displaced base hash must fail"
                );
            }
        }
        let single = timeout(Duration::from_secs(5), pending_single_v1)
            .await
            .unwrap()
            .unwrap();
        assert!(
            single.starts_with("HTTP/1.1 404"),
            "displaced hash cannot return 304 or replacement rows: {single}"
        );
        assert!(!single.contains("\r\netag:"));
        let v1_response = timeout(Duration::from_secs(5), pending_v1_http)
            .await
            .unwrap()
            .unwrap();
        assert!(v1_response.starts_with("HTTP/1.1 200"), "{v1_response}");
        let v1_rows: Value = from_str(v1_response.split_once("\r\n\r\n").unwrap().1).unwrap();
        assert_eq!(v1_rows[0]["id"], chain[2].block_hash().to_string());
        assert_eq!(v1_rows[0]["timestamp"], chain[2].header.time);
        assert_eq!(
            v1_rows[0]["mediantime"],
            chain[0].header.time.max(chain[2].header.time)
        );
        let height_v1 = timeout(Duration::from_secs(5), pending_height_v1)
            .await
            .unwrap()
            .unwrap();
        assert!(height_v1.starts_with("HTTP/1.1 200"), "{height_v1}");
        let height_rows: Value = from_str(height_v1.split_once("\r\n\r\n").unwrap().1).unwrap();
        assert_eq!(height_rows, v1_rows);
        let historical_v1 =
            exchange_with_etag(address, "GET", "/api/v1/blocks/0", &historical_v1_etag).await;
        assert!(
            historical_v1.starts_with("HTTP/1.1 304"),
            "unchanged historical range must retain its identity: {historical_v1}"
        );
        for (mode, task) in pending_v1 {
            let result = timeout(Duration::from_secs(5), task)
                .await
                .unwrap()
                .unwrap();
            if mode == 2 {
                assert!(
                    matches!(result, Err(QueryError::NotFound(_))),
                    "the displaced hash must not return replacement rows"
                );
            } else {
                let rows = result.unwrap();
                assert_eq!(
                    rows[0].info.id.to_string(),
                    chain[2].block_hash().to_string()
                );
                assert_eq!(*rows[0].info.timestamp, chain[2].header.time);
                assert_eq!(rows[0].extras.coinbase_raw, "0102");
            }
        }
        let blocks_response = timeout(Duration::from_secs(5), after_reorg_blocks)
            .await
            .unwrap();
        assert!(
            blocks_response.starts_with("HTTP/1.1 200"),
            "{blocks_response}"
        );
        let blocks: Value = from_str(blocks_response.split_once("\r\n\r\n").unwrap().1).unwrap();
        let height_blocks = timeout(Duration::from_secs(5), after_reorg_height_blocks)
            .await
            .unwrap();
        assert!(height_blocks.starts_with("HTTP/1.1 200"), "{height_blocks}");
        let height_rows: Value = from_str(height_blocks.split_once("\r\n\r\n").unwrap().1).unwrap();
        assert_eq!(height_rows, blocks);
        assert_eq!(blocks[0]["id"], chain[2].block_hash().to_string());
        assert_eq!(blocks[0]["timestamp"], chain[2].header.time);
        assert_eq!(
            blocks[0]["mediantime"],
            chain[0].header.time.max(chain[2].header.time)
        );
        assert_eq!(
            blocks_response
                .lines()
                .find_map(|line| line.strip_prefix("etag: "))
                .unwrap(),
            format!("W/\"blocks2-{}\"", chain[2].block_hash())
        );
        let response = timeout(Duration::from_secs(5), after_reorg_sync)
            .await
            .unwrap();
        assert!(response.starts_with("HTTP/1.1 200"), "{response}");
        let after: Value = from_str(response.split_once("\r\n\r\n").unwrap().1).unwrap();
        assert_eq!(after["indexed_height"], before["indexed_height"]);
        assert_eq!(after["tip_height"], before["tip_height"]);
        assert_eq!(after["last_indexed_at_unix"], chain[2].header.time);
        #[cfg(feature = "series")]
        for (names, format, path, request) in pending_bulk {
            let response = timeout(Duration::from_secs(5), request())
                .await
                .unwrap()
                .unwrap();
            assert!(response.starts_with("HTTP/1.1 200"), "{response}");
            let body = response.split_once("\r\n\r\n").unwrap().1;
            if format == "json" {
                let body: Value = from_str(body).unwrap();
                assert_eq!(body.as_array().unwrap().len(), 2);
                for series in body.as_array().unwrap() {
                    assert_eq!(series["data"], json!([chain[2].header.time]));
                }
            } else {
                assert_eq!(body, format!("{names}\n{0},{0}\n", chain[2].header.time));
            }
            let etag = response
                .lines()
                .find_map(|line| line.strip_prefix("etag: "))
                .unwrap();
            for method in ["GET", "HEAD"] {
                let response = exchange_with_etag(address, method, &path, etag).await;
                assert!(response.starts_with("HTTP/1.1 304"), "{response}");
                assert!(response.ends_with("\r\n\r\n"));
            }
        }
        #[cfg(feature = "series")]
        {
            let latest = timeout(Duration::from_secs(5), pending_latest())
                .await
                .unwrap()
                .unwrap();
            let length = timeout(Duration::from_secs(5), pending_len())
                .await
                .unwrap()
                .unwrap();
            assert!(length.starts_with("HTTP/1.1 304"), "{length}");
            assert!(length.ends_with("\r\n\r\n"));
            let len_etag = length
                .lines()
                .find_map(|line| line.strip_prefix("etag: "))
                .unwrap();
            for prefix in ["series"] {
                for method in ["GET", "HEAD"] {
                    let response = exchange_with_etag(
                        address,
                        method,
                        &format!("/api/{prefix}/timestamp/height/version"),
                        &version_etag,
                    )
                    .await;
                    assert!(response.starts_with("HTTP/1.1 304"), "{response}");
                    assert!(response.ends_with("\r\n\r\n"));
                    let response = exchange_with_etag(
                        address,
                        method,
                        &format!("/api/{prefix}/timestamp/height/len"),
                        len_etag,
                    )
                    .await;
                    assert!(response.starts_with("HTTP/1.1 304"), "{response}");
                    assert!(response.ends_with("\r\n\r\n"));
                }
            }
            assert!(latest.starts_with("HTTP/1.1 200"), "{latest}");
            let value: Value = from_str(latest.split_once("\r\n\r\n").unwrap().1).unwrap();
            assert_eq!(value, chain[2].header.time);
            let latest_etag = latest
                .lines()
                .find_map(|line| line.strip_prefix("etag: "))
                .unwrap();
            for prefix in ["series"] {
                for method in ["GET", "HEAD"] {
                    let response = exchange_with_etag(
                        address,
                        method,
                        &format!("/api/{prefix}/timestamp/height/latest"),
                        latest_etag,
                    )
                    .await;
                    assert!(response.starts_with("HTTP/1.1 304"), "{response}");
                    assert!(response.ends_with("\r\n\r\n"));
                }
            }
            let response = timeout(Duration::from_secs(5), pending_data())
                .await
                .unwrap()
                .unwrap();
            assert!(response.starts_with("HTTP/1.1 200"), "{response}");
            let body: Value = from_str(response.split_once("\r\n\r\n").unwrap().1).unwrap();
            assert_eq!(body["data"][0], chain[2].header.time);
            let etag = response
                .lines()
                .find_map(|line| line.strip_prefix("etag: "))
                .unwrap();
            for method in ["GET", "HEAD"] {
                let response = exchange_with_etag(
                    address,
                    method,
                    "/api/series/timestamp/height?start=1&end=2",
                    etag,
                )
                .await;
                assert!(response.starts_with("HTTP/1.1 304"), "{response}");
                assert!(response.ends_with("\r\n\r\n"));
            }
        }
        let etag = response
            .lines()
            .find_map(|line| line.strip_prefix("etag: "))
            .unwrap();
        let response = exchange_with_etag(address, "GET", "/api/server/sync", etag).await;
        assert!(response.starts_with("HTTP/1.1 304"), "{response}");
    });
}
