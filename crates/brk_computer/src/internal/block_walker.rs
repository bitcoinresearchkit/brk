//! Shared per-block-per-type cursor walker used by `outputs/by_type/` and
//! `inputs/by_type/`. The walker iterates blocks and aggregates the
//! per-tx output-type counts; pushing into a particular wrapper is left
//! to the caller.

use brk_error::Result;
use brk_types::TxIndex;
use vecdb::VecIndex;

/// Aggregated per-block counters produced by [`walk_blocks`].
pub(crate) struct BlockAggregate {
    pub entries_all: u64,
    pub entries_per_type: [u64; 12],
    pub txs_all: u64,
    pub txs_per_type: [u64; 12],
}

/// Whether to include the coinbase tx (first tx in each block) in the walk.
#[derive(Clone, Copy)]
pub(crate) enum CoinbasePolicy {
    Include,
    Skip,
}

/// Walk every block in `fi_batch`, calling `scan_tx` once per tx (which
/// fills a `[u32; 12]` with the per-output-type count for that tx),
/// aggregating into a [`BlockAggregate`] and handing it to `store`.
///
/// `entries_all` and `txs_all` aggregate over the 12 output types
/// indistinguishably; downstream consumers can cap to the 11 spendable
/// types if op_return is non-applicable.
#[inline]
pub(crate) fn walk_blocks(
    fi_batch: &[TxIndex],
    txid_len: usize,
    coinbase: CoinbasePolicy,
    mut scan_tx: impl FnMut(usize, &mut [u32; 12]) -> Result<()>,
    mut store: impl FnMut(BlockAggregate) -> Result<()>,
) -> Result<()> {
    for (j, first_tx) in fi_batch.iter().enumerate() {
        let fi = first_tx.to_usize();
        let next_fi = fi_batch
            .get(j + 1)
            .map(|v| v.to_usize())
            .unwrap_or(txid_len);

        let start_tx = match coinbase {
            CoinbasePolicy::Include => fi,
            CoinbasePolicy::Skip => fi + 1,
        };

        let mut entries_per_type = [0u64; 12];
        let mut txs_per_type = [0u64; 12];
        let mut entries_all = 0u64;
        let mut txs_all = 0u64;

        for tx_pos in start_tx..next_fi {
            let mut per_tx = [0u32; 12];
            scan_tx(tx_pos, &mut per_tx)?;
            let mut tx_has_any = false;
            for (i, &n) in per_tx.iter().enumerate() {
                if n > 0 {
                    entries_per_type[i] += u64::from(n);
                    txs_per_type[i] += 1;
                    entries_all += u64::from(n);
                    tx_has_any = true;
                }
            }
            if tx_has_any {
                txs_all += 1;
            }
        }

        store(BlockAggregate {
            entries_all,
            entries_per_type,
            txs_all,
            txs_per_type,
        })?;
    }

    Ok(())
}
