use crate::test_cache::init_cache;
use std::{
    net::{Ipv4Addr, TcpListener as StdListener},
    thread,
    time::Duration,
};

use bitcoin::{
    Amount, ScriptBuf, Transaction, TxIn, TxOut, absolute::LockTime, transaction::Version,
};
use bitview_default::DefaultPlugins;
use bitview_plugin::ImportContext;
use bitview_query::AsyncQuery;
use brk_mempool::Mempool;
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use serde_json::{Value, from_slice, from_str, json, to_string as SerdeJsonToString};
use tempfile::tempdir;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    runtime::Builder,
    spawn as TokioSpawn,
    task::spawn_blocking,
    time::timeout,
};

use super::server_routes::exchange_with_etag;
use crate::{Server, ServerConfig};

fn template_transactions() -> Vec<Value> {
    let mut input = TxIn::default();
    input.previous_output.vout = 0;
    let mut parent = Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![input],
        output: vec![TxOut {
            value: Amount::from_sat(1_000),
            script_pubkey: ScriptBuf::new(),
        }],
    };
    parent.output.push(parent.output[0].clone());
    let mut child = parent.clone();
    child.input[0].previous_output.txid = parent.compute_txid();
    let mut second_input = child.input[0].clone();
    second_input.previous_output.vout = 1;
    child.input.push(second_input);
    child.output.truncate(1);
    child.output[0].value = Amount::from_sat(1_800);
    [parent, child].iter().enumerate().map(|(index, tx)| json!({
        "data": bitcoin::consensus::encode::serialize_hex(tx),
        "txid": tx.compute_txid().to_string(), "hash": tx.compute_wtxid().to_string(),
        "fee": if index == 0 { 100 } else { 200 }, "sigops": 0, "weight": tx.weight().to_wu(),
        "depends": if index == 0 { vec![] } else { vec![1, 1] },
    })).collect()
}

async fn rpc_templates(node: TcpListener, transactions: Vec<Value>) {
    // Each attempt starts with a best-block observation. The successful one
    // also finishes with it; the malformed GBT fails before any live mutation.
    for step in 0..5 {
        let mut transactions = transactions.clone();
        if step == 1 {
            transactions[0]["txid"] = json!("0".repeat(64));
        }
        let mut socket = BufReader::new(node.accept().await.unwrap().0);
        let mut line = String::new();
        let mut length = 0;
        loop {
            line.clear();
            assert_ne!(socket.read_line(&mut line).await.unwrap(), 0);
            if line == "\r\n" {
                break;
            }
            if let Some(value) = line.to_ascii_lowercase().strip_prefix("content-length: ") {
                length = value.trim().parse::<usize>().unwrap();
            }
        }
        assert!(length < 4096);
        let mut bytes = vec![0; length];
        socket.read_exact(&mut bytes).await.unwrap();
        let request: Value = from_slice(&bytes).unwrap();
        if step % 2 == 0 {
            assert_eq!(request["method"], "getbestblockhash");
            let body =
                json!({"id": request["id"], "result": "0".repeat(64), "error": null}).to_string();
            socket
                .get_mut()
                .write_all(
                    format!(
                        "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{body}",
                        body.len()
                    )
                    .as_bytes(),
                )
                .await
                .unwrap();
            continue;
        }
        let calls = request.as_array().unwrap();
        let responses: Vec<_> = calls.iter().map(|call| {
            let result = match call["method"].as_str().unwrap() {
                "getblocktemplate" => json!({
                    "version": 1, "rules": [], "vbavailable": {}, "capabilities": [], "vbrequired": 0,
                    "previousblockhash": "0".repeat(64), "transactions": transactions, "coinbaseaux": {},
                    "coinbasevalue": 0, "target": "0".repeat(64), "mintime": 0, "mutable": [],
                    "noncerange": "00000000ffffffff", "sigoplimit": 80000, "sizelimit": 4000000,
                    "weightlimit": 4000000, "curtime": 0, "bits": "1d00ffff", "height": 1
                }),
                // Exercise the normal GBT/listing race: GBT bodies are enough
                // to populate block zero without a second transaction fetch.
                "getrawmempool" => json!([]),
                "getmempoolinfo" => json!({
                    "loaded": true, "size": 2, "bytes": 170, "usage": 1000, "total_fee": 0.000003,
                    "maxmempool": 300000000, "mempoolminfee": 0.00002, "minrelaytxfee": 0.00001,
                    "incrementalrelayfee": 0.00001, "unbroadcastcount": 0, "fullrbf": true
                }),
                method => panic!("unexpected RPC: {method}"),
            };
            json!({"id": call["id"], "result": result, "error": null})
        }).collect();
        let body = SerdeJsonToString(&responses).unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{body}",
            body.len()
        );
        socket
            .get_mut()
            .write_all(response.as_bytes())
            .await
            .unwrap();
    }
}

#[test]
fn template_revalidation_skips_body_admission_but_validates_history_and_queries() {
    init_cache();
    thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            let directory = tempdir().unwrap();
            let node = StdListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
            node.set_nonblocking(true).unwrap();
            let client = Client::new(
                &format!("http://{}", node.local_addr().unwrap()),
                Auth::None,
            )
            .unwrap();
            let reader = Reader::new_without_rlimit(directory.path().join("blocks"), &client);
            let plugins =
                DefaultPlugins::import(ImportContext::new(directory.path()), &reader).unwrap();
            let mut mempool = Mempool::new(&client);
            let query = AsyncQuery::build(&plugins, Some(mempool.read_only_clone()));
            Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let node = TcpListener::from_std(node).unwrap();
                    let transactions = template_transactions();
                    let mock = TokioSpawn(rpc_templates(node, transactions.clone()));
                    let server = Server::bind(
                        &query,
                        ServerConfig {
                            bind: Ipv4Addr::LOCALHOST.into(),
                            port: 0.into(),
                            data_path: directory.path().to_owned(),
                            ..ServerConfig::default()
                        },
                    )
                    .await
                    .unwrap();
                    let address = server.listener.local_addr().unwrap();
                    let admission = server.state.sync_query.clone();
                    let serving = TokioSpawn(server.serve());
                    // Unpublished snapshots return immediately without a validator.
                    timeout(Duration::from_secs(120), async {
                        let path = "/api/v1/mempool/block-template";
                        for route in [
                            path,
                            "/api/mempool/hash",
                            "/api/v1/fees/recommended",
                            "/api/v1/fees/precise",
                            "/api/v1/fees/mempool-blocks",
                            #[cfg(feature = "price")]
                            "/api/oracle/price",
                            #[cfg(feature = "price")]
                            "/api/mempool/price",
                            #[cfg(feature = "price")]
                            "/api/oracle/histogram/payments/live",
                            #[cfg(feature = "price")]
                            "/api/oracle/histogram/outputs/live",
                        ] {
                            for method in ["GET", "HEAD"] {
                                let response =
                                    exchange_with_etag(address, method, route, "*").await;
                                assert!(
                                    response.starts_with("HTTP/1.1 503"),
                                    "{route}: {response}"
                                );
                                assert!(!response.contains("\r\netag:"));
                                assert!(response.contains("\r\ncache-control: no-store\r\n"));
                            }
                        }
                        let (mut mempool, error) = spawn_blocking(move || {
                            let error = mempool
                                .tick_with(|_| Default::default())
                                .err()
                                .expect("invalid GBT must fail");
                            (mempool, error)
                        })
                        .await
                        .unwrap();
                        assert!(
                            error.to_string().contains("identity/weight mismatch"),
                            "{error}"
                        );
                        let response = exchange_with_etag(address, "GET", path, "*").await;
                        assert!(response.starts_with("HTTP/1.1 503"), "{response}");
                        spawn_blocking(move || {
                            let cycle = mempool.tick_with(|_| Default::default()).unwrap();
                            assert_eq!(
                                cycle
                                    .snapshot
                                    .txs
                                    .iter()
                                    .map(|tx| tx.parents.len())
                                    .sum::<usize>(),
                                1
                            );
                        })
                        .await
                        .unwrap();
                        mock.await.unwrap();
                        let fees = exchange_with_etag(
                            address,
                            "GET",
                            "/api/v1/fees/recommended",
                            "\"old\"",
                        )
                        .await;
                        assert!(fees.starts_with("HTTP/1.1 200"), "{fees}");
                        let fees: Value = from_str(fees.split_once("\r\n\r\n").unwrap().1).unwrap();
                        assert_eq!(fees["minimumFee"], 2.0);
                        let response = exchange_with_etag(address, "GET", path, "\"old\"").await;
                        assert!(response.starts_with("HTTP/1.1 200"), "{response}");
                        let body: Value =
                            from_str(response.split_once("\r\n\r\n").unwrap().1).unwrap();
                        let body_transactions = body["transactions"].as_array().unwrap();
                        assert_eq!(body_transactions.len(), transactions.len());
                        for (actual, expected) in body_transactions.iter().zip(&transactions) {
                            assert_eq!(actual["txid"], expected["txid"]);
                        }
                        let tag = response
                            .lines()
                            .find_map(|line| line.strip_prefix("etag: "))
                            .unwrap();
                        let diff_path = format!("{path}/diff/{}", body["hash"].as_u64().unwrap());
                        let diff = exchange_with_etag(address, "GET", &diff_path, "\"old\"").await;
                        assert!(diff.starts_with("HTTP/1.1 200"), "{diff}");
                        let diff_body: Value =
                            from_str(diff.split_once("\r\n\r\n").unwrap().1).unwrap();
                        assert_eq!(diff_body["order"], json!([0, 1]));
                        let diff_tag = diff
                            .lines()
                            .find_map(|line| line.strip_prefix("etag: "))
                            .unwrap();
                        // Matching conditionals must not queue behind body construction.
                        let _permit = admission.acquire().await.unwrap();
                        for method in ["GET", "HEAD"] {
                            for (route, tag) in [(path, tag), (diff_path.as_str(), diff_tag)] {
                                for condition in [tag, "*"] {
                                    let response =
                                        exchange_with_etag(address, method, route, condition).await;
                                    assert!(response.starts_with("HTTP/1.1 304"), "{response}");
                                    assert!(response.ends_with("\r\n\r\n"));
                                    for (invalid, status) in [
                                        (format!("{route}?unknown=1"), "400"),
                                        (format!("{path}/diff/3735928559"), "404"),
                                    ] {
                                        let response = exchange_with_etag(
                                            address, method, &invalid, condition,
                                        )
                                        .await;
                                        assert!(
                                            response.starts_with(&format!("HTTP/1.1 {status}")),
                                            "{response}"
                                        );
                                        assert!(!response.contains("\r\netag:"));
                                    }
                                }
                            }
                        }
                    })
                    .await
                    .unwrap();
                    serving.abort();
                    let _ = serving.await;
                });
        })
        .unwrap()
        .join()
        .unwrap();
}
