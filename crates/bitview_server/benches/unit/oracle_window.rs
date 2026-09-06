//! Native warmup versus owned warmed-state reuse. Synthetic indexed blocks;
//! excludes HTTP, cold device I/O, producer update cost and cache-key checks.

use std::{
    fs,
    hint::black_box,
    net::Ipv4Addr,
    thread,
    time::{Duration, Instant},
};

use bitcoin::{
    Amount, Block, Network, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness,
    absolute::LockTime, blockdata::constants::genesis_block, consensus::serialize,
    transaction::Version,
};
use bitview_plugin::ImportContext;
use bitview_plugin_indexer::Indexer;
use bitview_plugin_price::Vecs as Prices;
use brk_exit::Exit;
use brk_oracle::{Config, HistogramRaw, Oracle, cents_to_bin};
use brk_reader::Reader;
use brk_rpc::{Auth, Client};
use serde_json::Value;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    runtime::Builder,
};

fn chain() -> Vec<Block> {
    let mut chain = vec![genesis_block(Network::Bitcoin)];
    for height in 1..=40u8 {
        let mut block = chain.last().unwrap().clone();
        block.header.prev_blockhash = block.block_hash();
        block.header.time += 600;
        block.txdata.truncate(1);
        block.txdata[0].input[0].script_sig = ScriptBuf::from_bytes(vec![1, height]);
        let mut parent = OutPoint::new(block.txdata[0].compute_txid(), 0);
        let mut remaining = block.txdata[0].output[0].value.to_sat();
        for index in 0..2000u32 {
            // Vary eligible amounts and script bytes rather than measuring a
            // single repeated value or a coinbase-only window.
            let payment = 12_345 + u64::from(index) * 17;
            remaining -= payment + 1000;
            let mut script = vec![0, 20];
            script.extend_from_slice(&[height; 20]);
            let tx = Transaction {
                version: Version::TWO,
                lock_time: LockTime::ZERO,
                input: vec![TxIn {
                    previous_output: parent,
                    script_sig: ScriptBuf::new(),
                    sequence: Sequence::MAX,
                    witness: Witness::new(),
                }],
                output: vec![
                    TxOut {
                        value: Amount::from_sat(payment),
                        script_pubkey: ScriptBuf::from_bytes(script),
                    },
                    TxOut {
                        value: Amount::from_sat(remaining),
                        script_pubkey: ScriptBuf::new(),
                    },
                ],
            };
            parent = OutPoint::new(tx.compute_txid(), 1);
            block.txdata.push(tx);
        }
        block.header.merkle_root = block.compute_merkle_root().unwrap();
        assert!(block.weight() <= bitcoin::Weight::MAX_BLOCK);
        chain.push(block);
    }
    chain
}

#[test]
#[ignore = "native 12/40-block oracle replay tradeoff; synthetic 2000-transaction blocks, excludes HTTP and cold storage"]
fn benchmark_native_oracle_window() {
    thread::Builder::new().stack_size(8 * 1024 * 1024).spawn(|| {
        Builder::new_multi_thread().worker_threads(2).enable_all().build().unwrap().block_on(async {
            let directory = tempfile::tempdir().unwrap();
            let blocks_path = directory.path().join("blocks");
            fs::create_dir(&blocks_path).unwrap();
            let blocks = chain();
            let mut record = Vec::new();
            for block in &blocks {
                let bytes = serialize(block);
                record.extend_from_slice(&[0xf9, 0xbe, 0xb4, 0xd9]);
                record.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
                record.extend_from_slice(&bytes);
            }
            fs::write(blocks_path.join("blk00000.dat"), record).unwrap();
            let node = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
            let client = Client::new_with(&format!("http://{}", node.local_addr().unwrap()), Auth::None, 0, Duration::ZERO).unwrap();
            let mock = tokio::spawn(async move {
                loop {
                    let mut socket = BufReader::new(node.accept().await.unwrap().0);
                    let mut line = String::new();
                    let mut length = 0;
                    loop {
                        line.clear();
                        assert_ne!(socket.read_line(&mut line).await.unwrap(), 0);
                        if line == "\r\n" { break; }
                        if let Some(value) = line.to_ascii_lowercase().strip_prefix("content-length: ") {
                            length = value.trim().parse::<usize>().unwrap();
                        }
                    }
                    let mut body = vec![0; length];
                    socket.read_exact(&mut body).await.unwrap();
                    let request: Value = serde_json::from_slice(&body).unwrap();
                    let body = super::chain_rpc::reply(&request, 40, &blocks, 40).to_string();
                    socket.get_mut().write_all(format!("HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{body}", body.len()).as_bytes()).await.unwrap();
                }
            });
            let reader = Reader::new_without_rlimit(blocks_path, &client);
            let mut indexer = Indexer::import(ImportContext::new(directory.path()), &reader).unwrap();
            indexer.index(&Exit::default()).unwrap();
            indexer.finish_update().unwrap();
            let safe = indexer.safe_lengths();
            assert_eq!(usize::from(safe.height), 41);
            for config in [Config::default(), Config::slow()] {
                let range = (41 - config.window_size)..41;
                let rebuild = || {
                    let mut result = Ok(());
                    let oracle = Oracle::from_checkpoint(cents_to_bin(1_000_000.0), config, |oracle| {
                        result = Prices::feed_blocks_for_warmup(oracle, &indexer, range.clone(), Some(&safe));
                    });
                    result.unwrap();
                    oracle
                };
                let base = rebuild();
                assert!(base.ema().iter().any(|value| *value > 0.0));
                let histogram = HistogramRaw::zeros();
                let mut expected = base.clone();
                expected.process_histogram(&histogram);
                let expected = expected.ema().iter().copied().collect::<Vec<_>>();
                for (label, repeat) in [("replay", 30), ("clone", 300)] {
                    let mut durations = Vec::new();
                    for _ in 0..repeat {
                        let started = Instant::now();
                        let mut oracle = if label == "replay" { rebuild() } else { base.clone() };
                        oracle.process_histogram(black_box(&histogram));
                        black_box(oracle.price_cents());
                        durations.push(started.elapsed());
                        assert!(oracle.ema().iter().copied().eq(expected.iter().copied()));
                    }
                    durations.sort_unstable();
                    eprintln!("oracle {} blocks, {label}: p50={:?}, p95={:?}, samples={repeat}", config.window_size,
                        durations[repeat / 2], durations[(repeat * 95 / 100).min(repeat - 1)]);
                }
            }
            mock.abort();
            let _ = mock.await;
        });
    }).unwrap().join().unwrap();
}
