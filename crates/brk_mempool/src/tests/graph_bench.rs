use std::time::Instant;

use bitcoin::hashes::Hash;
use brk_types::{Sats, Timestamp, Txid, TxidPrefix, VSize};
use smallvec::SmallVec;

use crate::{TxEntry, steps::rebuilder::graph::Graph};

fn synthetic_mempool(n: usize) -> Vec<Option<TxEntry>> {
    let make_txid = |i: usize| -> Txid {
        let mut bytes = [0u8; 32];
        bytes[0..8].copy_from_slice(&(i as u64).to_ne_bytes());
        bytes[8..16].copy_from_slice(&((i as u64).wrapping_mul(2_654_435_761)).to_ne_bytes());
        Txid::from(bitcoin::Txid::from_slice(&bytes).unwrap())
    };

    let mut entries: Vec<Option<TxEntry>> = Vec::with_capacity(n);
    let mut txids: Vec<Txid> = Vec::with_capacity(n);
    for i in 0..n {
        let txid = make_txid(i);
        txids.push(txid.clone());

        let depends: SmallVec<[TxidPrefix; 2]> = match i % 100 {
            0..=94 => SmallVec::new(),
            95..=98 if i > 0 => {
                let p = (i.wrapping_mul(7919)) % i;
                std::iter::once(TxidPrefix::from(&txids[p])).collect()
            }
            _ if i > 1 => {
                let p1 = (i.wrapping_mul(7919)) % i;
                let p2 = (i.wrapping_mul(6151)) % i;
                [TxidPrefix::from(&txids[p1]), TxidPrefix::from(&txids[p2])]
                    .into_iter()
                    .collect()
            }
            _ => SmallVec::new(),
        };

        entries.push(Some(TxEntry {
            txid,
            fee: Sats::from((i as u64).wrapping_mul(137) % 10_000 + 1),
            vsize: VSize::from(250u64),
            size: 250,
            depends,
            first_seen: Timestamp::now(),
            rbf: false,
        }));
    }
    entries
}

#[test]
#[ignore = "perf benchmark; run with --ignored --nocapture"]
fn perf_build_graph() {
    let sizes = [1_000usize, 10_000, 50_000, 100_000, 300_000];
    eprintln!();
    eprintln!("Graph::build perf (release, single call):");
    eprintln!("  n          build");
    eprintln!("  ------------------------");
    for &n in &sizes {
        let entries = synthetic_mempool(n);
        let _ = Graph::build(&entries);

        let t = Instant::now();
        let g = Graph::build(&entries);
        let dt = t.elapsed();
        let ns = dt.as_nanos();
        let pretty = if ns >= 1_000_000 {
            format!("{:.2} ms", ns as f64 / 1_000_000.0)
        } else {
            format!("{:.2} µs", ns as f64 / 1_000.0)
        };
        eprintln!("  {:<10} {:<10} ({} nodes)", n, pretty, g.len());
    }
    eprintln!();
}
