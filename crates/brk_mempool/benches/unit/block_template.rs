use std::{hint::black_box, time::Instant};

use bitcoin::{Txid as BitcoinTxid, hashes::Hash};
use brk_types::FeeRate;
use rustc_hash::FxHashSet;
use serde_json::to_vec;

use super::*;
use crate::{
    state::TxEntry,
    test_support::{fake_entry_info, fake_tx, p2wpkh_script},
};

// Previous implementation retained only as a same-input performance baseline.
fn two_tables(resolved: ResolvedBlockTemplateDiff) -> Result<BlockTemplateDiff> {
    let ResolvedBlockTemplateDiff {
        since,
        past,
        source,
    } = resolved;
    let hash = source.hash()?;
    let prior_index: FxHashMap<Txid, u32> = past
        .iter()
        .enumerate()
        .map(|(idx, tx)| (tx.txid, idx as u32))
        .collect();
    let snap = &source.snapshot;
    let mut order = Vec::with_capacity(snap.blocks.first().map_or(0, Vec::len));
    let mut current: FxHashSet<Txid> = FxHashSet::default();
    for tx in snap.template_transactions().iter() {
        let txid = tx.txid;
        current.insert(txid);
        match prior_index.get(&txid) {
            Some(&idx) if Arc::ptr_eq(tx, &past[idx as usize]) => {
                order.push(BlockTemplateDiffEntry::Retained(idx))
            }
            _ => order.push(BlockTemplateDiffEntry::New(tx.as_ref().clone())),
        }
    }
    let removed = past
        .iter()
        .filter(|tx| !current.contains(&tx.txid))
        .map(|tx| tx.txid)
        .collect();
    Ok(BlockTemplateDiff {
        hash,
        since,
        order,
        removed,
    })
}

#[test]
#[ignore = "same-input template diff comparison; excludes snapshot construction and HTTP"]
fn benchmark_block_template_diff() {
    for count in [100usize, 2_000, 8_000] {
        let mempool = Mempool::for_test();
        let mut ids = Vec::with_capacity(count * 2);
        {
            let mut state = mempool.test_state_lock().write();
            for index in 0..count * 2 {
                let mut bytes = [0; 32];
                bytes[..8].copy_from_slice(&(index as u64 + 1).to_le_bytes());
                let mut tx = fake_tx(0, &[], &[(p2wpkh_script(1), 1_234)]);
                tx.txid = Txid::from(BitcoinTxid::from_byte_array(bytes));
                ids.push(tx.txid);
                let info = fake_entry_info(tx.txid, 100, 100);
                state.txs.insert(tx, TxEntry::new(&info, 100, false));
            }
        }
        for (name, selected) in [
            ("unchanged", ids[..count].to_vec()),
            ("reordered", ids[..count].iter().rev().copied().collect()),
            ("half-new", ids[count / 2..count + count / 2].to_vec()),
            ("all-new", ids[count..].to_vec()),
            ("empty", Vec::new()),
        ] {
            mempool.test_tick(&ids[..count], FeeRate::new(1.0));
            let since = mempool.next_block_hash().unwrap();
            mempool.test_tick(&selected, FeeRate::new(1.0));
            let resolved = mempool.resolve_block_template_diff(since).unwrap();
            let capture = || ResolvedBlockTemplateDiff {
                since: resolved.since,
                past: resolved.past.clone(),
                source: resolved.source.clone(),
            };
            assert_eq!(
                to_vec(&two_tables(capture()).unwrap()).unwrap(),
                to_vec(&capture().build().unwrap()).unwrap(),
                "{count} {name}"
            );
            let mut samples = [Vec::new(), Vec::new()];
            for round in 0..13 {
                for variant in [round % 2, 1 - round % 2] {
                    let started = Instant::now();
                    for _ in 0..20 {
                        black_box(if variant == 0 {
                            two_tables(black_box(capture())).unwrap()
                        } else {
                            black_box(capture()).build().unwrap()
                        });
                    }
                    if round >= 2 {
                        samples[variant].push(started.elapsed().as_nanos() / 20);
                    }
                }
            }
            for sample in &mut samples {
                sample.sort_unstable();
            }
            eprintln!(
                "diff {count} {name}: two-tables={}ns one-table={}ns ratio={:.3}",
                samples[0][5],
                samples[1][5],
                samples[1][5] as f64 / samples[0][5] as f64,
            );
        }
    }
}
