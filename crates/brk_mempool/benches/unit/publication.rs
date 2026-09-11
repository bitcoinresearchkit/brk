//! Deterministic native pipeline workload; RPC and HTTP are tested separately.
//! Run alone: cargo test -p brk_mempool --release publication_benchmark -- --ignored --nocapture --test-threads=1

use std::{
    alloc::{GlobalAlloc, Layout, System},
    hint::black_box,
    sync::{
        Barrier,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};

use bitcoin::{ScriptBuf, Txid as BitcoinTxid, WPubkeyHash, hashes::Hash};
use brk_types::{FeeRate, Sats, Transaction, TxidPrefix, Vin};

use super::*;
use crate::{
    ReadOnlyMempool, ReadOnlyState, TxRemoval,
    cycle::AddrTransitions,
    state::TxEntry,
    steps::preparer::{TxAddition, TxsPulled},
    test_support::{fake_entry_info, fake_tx},
};

struct AllocationMeter;
static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

#[global_allocator]
static ALLOCATOR: AllocationMeter = AllocationMeter;

fn allocated(bytes: usize) {
    ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
    ALLOCATED.fetch_add(bytes, Ordering::Relaxed);
    let live = LIVE.fetch_add(bytes, Ordering::Relaxed) + bytes;
    PEAK.fetch_max(live, Ordering::Relaxed);
}

unsafe impl GlobalAlloc for AllocationMeter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            allocated(layout.size());
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        LIVE.fetch_sub(layout.size(), Ordering::Relaxed);
        unsafe { System.dealloc(pointer, layout) };
    }

    unsafe fn realloc(&self, pointer: *mut u8, old: Layout, size: usize) -> *mut u8 {
        let pointer = unsafe { System.realloc(pointer, old, size) };
        if !pointer.is_null() {
            LIVE.fetch_sub(old.size(), Ordering::Relaxed);
            allocated(size);
        }
        pointer
    }
}

// These adapters isolate the ownership API for a same-fixture baseline run.
fn edit<T>(writer: &mut Mempool, f: impl FnOnce(&mut State) -> T) -> T {
    f(&mut writer.state)
}
fn rebuild(writer: &mut Mempool, gbt: &[Txid], floor: FeeRate, _changed: bool) {
    writer.rebuilder.tick(&writer.state, gbt, floor);
}
fn commit(writer: &mut Mempool, ids: &[Txid]) {
    writer.publish_observation(BlockHash::default(), writer.state.contains_all(ids));
}
fn reader(writer: &Mempool) -> ReadOnlyMempool {
    writer.read_only_clone()
}
fn capture(reader: &ReadOnlyMempool) -> Arc<ReadOnlyState> {
    reader.load()
}
fn read_queries(reader: &ReadOnlyMempool, txid: &Txid) -> bool {
    let state = reader.load();
    let tip = BlockHash::default();
    let valid = black_box(state.live_raw_histogram(&tip)).is_ok();
    let info = black_box(state.info());
    let ids = black_box(state.txids_hash());
    let transaction = black_box(state.transaction(txid, &tip));
    valid && info.is_ok() && ids.is_ok() && transaction.is_ok()
}

fn txid(index: usize) -> Txid {
    let mut bytes = [0; 32];
    bytes[..8].copy_from_slice(&(index as u64 + 1).to_le_bytes());
    BitcoinTxid::from_byte_array(bytes).into()
}

fn script(index: usize) -> ScriptBuf {
    let mut bytes = [0; 20];
    bytes[..8].copy_from_slice(&(index as u64 + 1).to_le_bytes());
    ScriptBuf::new_p2wpkh(&WPubkeyHash::from_byte_array(bytes))
}

fn transaction(index: usize, unresolved: bool) -> TxAddition {
    let prevout = (!unresolved).then(|| TxOut::from((script(index / 2), Sats::from(20_000u64))));
    let fee = 100 + (index % 1000) as u64;
    let mut tx: Transaction = fake_tx(
        0,
        &[prevout],
        &[(script(index), 12_345), (script(index / 2), 7_655 - fee)],
    );
    tx.txid = txid(index);
    tx.fee = Sats::from(fee);
    tx.input[0].txid = txid(index + 10_000_000);
    let mut info = fake_entry_info(tx.txid, fee, 200);
    if index > 0 && index % 10 == 1 {
        info.depends.push(txid(index - 1));
        tx.input[0].txid = txid(index - 1);
    }
    TxAddition::Fresh {
        tx,
        entry: TxEntry::new(&info, 200, true),
    }
}

fn apply(writer: &mut Mempool, pulled: TxsPulled) {
    let graph = writer.rebuilder.snapshot();
    applier::apply(&mut writer.state, &graph, pulled, &mut CycleDiff::default());
}

fn measured<T>(f: impl FnOnce() -> T) -> (T, [u128; 3]) {
    let bytes = ALLOCATED.load(Ordering::Relaxed);
    let allocations = ALLOCATIONS.load(Ordering::Relaxed);
    let start = Instant::now();
    let value = f();
    let nanos = start.elapsed().as_nanos();
    (
        value,
        [
            nanos,
            (ALLOCATED.load(Ordering::Relaxed) - bytes) as u128,
            (ALLOCATIONS.load(Ordering::Relaxed) - allocations) as u128,
        ],
    )
}

fn report(count: usize, workload: &str, stage: &str, samples: &mut [[u128; 3]]) {
    samples.sort_unstable_by_key(|sample| sample[0]);
    let median = samples[samples.len() / 2];
    let p95 = samples[(samples.len() * 95 / 100).min(samples.len() - 1)][0];
    let p99 = samples[(samples.len() * 99 / 100).min(samples.len() - 1)][0];
    println!(
        "sample,{count},{workload},{stage},p50_ns={},p95_ns={p95},p99_ns={p99},bytes={},allocations={}",
        median[0], median[1], median[2]
    );
}

#[test]
#[ignore = "native pipeline allocation/latency benchmark; run alone"]
fn publication_benchmark() {
    for count in [0usize, 100, 10_000, 50_000, 200_000] {
        let initial_live = LIVE.load(Ordering::Relaxed);
        PEAK.store(initial_live, Ordering::Relaxed);
        let mut writer = Mempool::for_test();
        apply(
            &mut writer,
            TxsPulled {
                live_len: count,
                added: (0..count).map(|index| transaction(index, false)).collect(),
                removed: vec![],
            },
        );
        let mut ids: Vec<_> = (0..count).map(txid).collect();
        rebuild(
            &mut writer,
            &ids[..ids.len().min(2500)],
            FeeRate::new(1.0),
            true,
        );
        let (_, freeze) = measured(|| commit(&mut writer, &ids));
        let reader = reader(&writer);
        println!(
            "initial,{count},live_bytes={},peak_bytes={},freeze_ns={},freeze_bytes={},freeze_allocations={}",
            LIVE.load(Ordering::Relaxed) - initial_live,
            PEAK.load(Ordering::Relaxed) - initial_live,
            freeze[0],
            freeze[1],
            freeze[2]
        );
        let mut retained = Vec::new();
        let mut next = count;
        for workload in [
            "noop",
            "fee",
            "churn",
            "replacement",
            "fills",
            "eviction",
            "expiry",
        ] {
            let mut stages: [Vec<[u128; 3]>; 4] = Default::default();
            for round in 0..11 {
                let amount = match workload {
                    "eviction" => ids.len().min(2500),
                    "churn" | "replacement" | "fills" => (count / 100).max(1).min(ids.len()),
                    _ => 0,
                };
                let remove: Vec<_> = ids.iter().rev().take(amount).copied().collect();
                if workload == "fills" {
                    apply(
                        &mut writer,
                        TxsPulled {
                            live_len: ids.len(),
                            removed: remove
                                .iter()
                                .map(|id| (TxidPrefix::from(id), TxRemoval::Vanished))
                                .collect(),
                            added: (next..next + amount)
                                .map(|index| transaction(index, true))
                                .collect(),
                        },
                    );
                    ids.truncate(ids.len() - amount);
                    ids.extend((next..next + amount).map(txid));
                    next += amount;
                    rebuild(
                        &mut writer,
                        &ids[..ids.len().min(2500)],
                        FeeRate::new(1.0),
                        true,
                    );
                    commit(&mut writer, &ids);
                }
                if workload == "expiry" {
                    edit(&mut writer, |state| {
                        for index in next..next + count / 100 {
                            let TxAddition::Fresh { tx, entry } = transaction(index, false) else {
                                unreachable!()
                            };
                            state
                                .graveyard
                                .bury(tx, entry, FeeRate::new(5.0), TxRemoval::Vanished);
                        }
                    });
                    next += count / 100;
                }
                let begin = Instant::now();
                let (_, mutation) = measured(|| match workload {
                    "churn" | "replacement" | "eviction" => {
                        let replacing = workload == "replacement";
                        let additions = if workload == "eviction" { 0 } else { amount };
                        apply(
                            &mut writer,
                            TxsPulled {
                                live_len: ids.len() - amount + additions,
                                removed: remove
                                    .iter()
                                    .enumerate()
                                    .map(|(index, id)| {
                                        (
                                            TxidPrefix::from(id),
                                            if replacing {
                                                TxRemoval::Replaced {
                                                    by: txid(next + index),
                                                }
                                            } else {
                                                TxRemoval::Vanished
                                            },
                                        )
                                    })
                                    .collect(),
                                added: (next..next + additions)
                                    .map(|index| transaction(index, false))
                                    .collect(),
                            },
                        );
                        ids.truncate(ids.len() - amount);
                        ids.extend((next..next + additions).map(txid));
                        next += additions;
                    }
                    "fills" => edit(&mut writer, |state| {
                        for id in ids.iter().rev().take(amount) {
                            let prevouts = state.txs.apply_fills(
                                &TxidPrefix::from(id),
                                vec![(
                                    Vin::from(0usize),
                                    TxOut::from((script(next), Sats::from(20_000u64))),
                                )],
                            );
                            for prevout in prevouts {
                                state.addrs.add_input(
                                    &mut AddrTransitions::default(),
                                    id,
                                    &prevout,
                                );
                            }
                        }
                    }),
                    "expiry" => edit(&mut writer, |state| {
                        state.graveyard.shift_oldest_back(usize::MAX);
                        state.graveyard.evict_old();
                    }),
                    _ => (),
                });
                let (_, graph) = measured(|| {
                    rebuild(
                        &mut writer,
                        &ids[..ids.len().min(2500)],
                        FeeRate::new(if workload == "fee" {
                            1.0 + (round % 2) as f64
                        } else {
                            1.0
                        }),
                        amount != 0 && workload != "fills",
                    )
                });
                let (_, publication) = measured(|| commit(&mut writer, &ids));
                let total = begin.elapsed().as_nanos();
                if round >= 2 {
                    for (stage, value) in stages.iter_mut().zip([
                        mutation,
                        graph,
                        publication,
                        [
                            total,
                            mutation[1] + graph[1] + publication[1],
                            mutation[2] + graph[2] + publication[2],
                        ],
                    ]) {
                        stage.push(value);
                    }
                }
                if round == 4 {
                    retained.push(capture(&reader));
                }
            }
            for (stage, samples) in ["mutation", "graph", "publication", "cycle"]
                .into_iter()
                .zip(&mut stages)
            {
                report(count, workload, stage, samples);
            }
        }
        let mut reads = Vec::new();
        let mut successes = 0;
        for _ in 0..101 {
            let (_, sample) = measured(|| {
                for _ in 0..100 {
                    successes += usize::from(read_queries(&reader, &txid(0)));
                }
            });
            reads.push([sample[0] / 100, sample[1] / 100, sample[2] / 100]);
        }
        report(count, "idle", "read", &mut reads);
        println!("reads,{count},success={successes},attempts=10100");
        let ready = Arc::new(Barrier::new(5));
        let stop = Arc::new(AtomicBool::new(false));
        let workers: Vec<_> = (0..4)
            .map(|_| {
                let reader = reader.clone();
                let ready = ready.clone();
                let stop = stop.clone();
                thread::spawn(move || {
                    let mut samples = Vec::new();
                    let mut failed = 0;
                    let mut success = 0;
                    ready.wait();
                    while !stop.load(Ordering::Relaxed) {
                        let start = Instant::now();
                        if read_queries(&reader, &txid(0)) {
                            success += 1;
                            if success % 100 == 0 {
                                samples.push([start.elapsed().as_nanos(), 0, 0]);
                            }
                        } else {
                            failed += 1;
                        }
                    }
                    (samples, success, failed)
                })
            })
            .collect();
        ready.wait();
        for round in 0..10 {
            apply(
                &mut writer,
                TxsPulled {
                    live_len: ids.len(),
                    added: vec![],
                    removed: vec![],
                },
            );
            rebuild(
                &mut writer,
                &ids[..ids.len().min(2500)],
                FeeRate::new(1.0 + (round % 2) as f64),
                false,
            );
            commit(&mut writer, &ids);
        }
        stop.store(true, Ordering::Relaxed);
        let mut parallel = Vec::new();
        let mut failed = 0;
        let mut success = 0;
        for worker in workers {
            let (samples, successes, failures) = worker.join().unwrap();
            success += successes;
            parallel.extend(samples);
            failed += failures;
        }
        if !parallel.is_empty() {
            report(count, "4_readers_10_updates", "read", &mut parallel);
        }
        println!("concurrent,{count},success={},failed={failed}", success);
        drop(parallel);
        drop(reads);
        let live = LIVE.load(Ordering::Relaxed) - initial_live;
        let (_, release) = measured(|| drop(retained));
        let reclaimed = live.saturating_sub(LIVE.load(Ordering::Relaxed) - initial_live);
        drop(reader);
        let (_, destruction) = measured(|| drop(writer));
        println!(
            "retention,{count},live_bytes={live},peak_bytes={},reclaimed_bytes={reclaimed},release_ns={},writer_drop_ns={}",
            PEAK.load(Ordering::Relaxed) - initial_live,
            release[0],
            destruction[0]
        );
    }
}
