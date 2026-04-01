use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{FeeRate, Indexes, OutPoint, Sats, TxInIndex, VSize};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableVec, VecIndex, WritableVec, unlikely};

use super::super::size;
use super::Vecs;
use crate::{indexes, inputs};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        spent: &inputs::SpentVecs,
        size_vecs: &size::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.input_value.compute_sum_from_indexes(
            starting_indexes.tx_index,
            &indexer.vecs.transactions.first_txin_index,
            &indexes.tx_index.input_count,
            &spent.value,
            exit,
        )?;
        self.output_value.compute_sum_from_indexes(
            starting_indexes.tx_index,
            &indexer.vecs.transactions.first_txout_index,
            &indexes.tx_index.output_count,
            &indexer.vecs.outputs.value,
            exit,
        )?;

        self.compute_fees(indexer, indexes, size_vecs, starting_indexes, exit)?;

        let (r1, (r2, r3)) = rayon::join(
            || {
                self.fee
                    .derive_from_with_skip(indexer, indexes, starting_indexes, exit, 1)
            },
            || {
                rayon::join(
                    || {
                        self.fee_rate.derive_from_with_skip(
                            indexer,
                            indexes,
                            starting_indexes,
                            exit,
                            1,
                        )
                    },
                    || {
                        self.effective_fee_rate.derive_from_with_skip(
                            indexer,
                            indexes,
                            starting_indexes,
                            exit,
                            1,
                        )
                    },
                )
            },
        );
        r1?;
        r2?;
        r3?;

        Ok(())
    }

    fn compute_fees(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        size_vecs: &size::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let dep_version = self.input_value.version()
            + self.output_value.version()
            + size_vecs.vsize.tx_index.version();

        self.fee
            .tx_index
            .validate_computed_version_or_reset(dep_version)?;
        self.fee_rate
            .tx_index
            .validate_computed_version_or_reset(dep_version)?;
        self.effective_fee_rate
            .tx_index
            .validate_computed_version_or_reset(dep_version)?;

        let target = self
            .input_value
            .len()
            .min(self.output_value.len())
            .min(size_vecs.vsize.tx_index.len());
        let min = self
            .fee
            .tx_index
            .len()
            .min(self.fee_rate.tx_index.len())
            .min(self.effective_fee_rate.tx_index.len())
            .min(starting_indexes.tx_index.to_usize());

        if min >= target {
            return Ok(());
        }

        self.fee
            .tx_index
            .truncate_if_needed(starting_indexes.tx_index)?;
        self.fee_rate
            .tx_index
            .truncate_if_needed(starting_indexes.tx_index)?;
        self.effective_fee_rate
            .tx_index
            .truncate_if_needed(starting_indexes.tx_index)?;

        let start_tx = self.fee.tx_index.len();
        let max_height = indexer.vecs.transactions.first_tx_index.len();

        let start_height = if start_tx == 0 {
            0
        } else {
            indexer
                .vecs
                .transactions
                .height
                .collect_one_at(start_tx)
                .unwrap()
                .to_usize()
        };

        for h in start_height..max_height {
            let first_tx: usize = indexer
                .vecs
                .transactions
                .first_tx_index
                .collect_one_at(h)
                .unwrap()
                .to_usize();
            let n = *indexes.height.tx_index_count.collect_one_at(h).unwrap() as usize;

            if first_tx + n > target {
                break;
            }

            // Batch read all per-tx data for this block
            let input_values = self.input_value.collect_range_at(first_tx, first_tx + n);
            let output_values = self.output_value.collect_range_at(first_tx, first_tx + n);
            let vsizes: Vec<VSize> = size_vecs
                .vsize
                .tx_index
                .collect_range_at(first_tx, first_tx + n);
            let txin_starts: Vec<TxInIndex> = indexer
                .vecs
                .transactions
                .first_txin_index
                .collect_range_at(first_tx, first_tx + n);
            let input_begin = txin_starts[0].to_usize();
            let input_end = if h + 1 < max_height {
                indexer
                    .vecs
                    .inputs
                    .first_txin_index
                    .collect_one_at(h + 1)
                    .unwrap()
                    .to_usize()
            } else {
                indexer.vecs.inputs.outpoint.len()
            };
            let outpoints: Vec<OutPoint> = indexer
                .vecs
                .inputs
                .outpoint
                .collect_range_at(input_begin, input_end);

            // Compute fee + fee_rate per tx
            let mut fees = Vec::with_capacity(n);
            for j in 0..n {
                let fee = if unlikely(input_values[j].is_max()) {
                    Sats::ZERO
                } else {
                    input_values[j] - output_values[j]
                };
                self.fee.tx_index.push(fee);
                self.fee_rate.tx_index.push(FeeRate::from((fee, vsizes[j])));
                fees.push(fee);
            }

            // Effective fee rate via same-block CPFP clustering
            let effective = cluster_fee_rates(
                &txin_starts,
                &outpoints,
                input_begin,
                first_tx,
                &fees,
                &vsizes,
            );
            for rate in effective {
                self.effective_fee_rate.tx_index.push(rate);
            }

            if h % 1_000 == 0 {
                let _lock = exit.lock();
                self.fee.tx_index.write()?;
                self.fee_rate.tx_index.write()?;
                self.effective_fee_rate.tx_index.write()?;
            }
        }

        let _lock = exit.lock();
        self.fee.tx_index.write()?;
        self.fee_rate.tx_index.write()?;
        self.effective_fee_rate.tx_index.write()?;

        Ok(())
    }
}

/// Clusters same-block parent-child txs and computes effective fee rate per cluster.
fn cluster_fee_rates(
    txin_starts: &[TxInIndex],
    outpoints: &[OutPoint],
    outpoint_base: usize,
    first_tx: usize,
    fees: &[Sats],
    vsizes: &[VSize],
) -> Vec<FeeRate> {
    let n = fees.len();
    let mut parent: Vec<usize> = (0..n).collect();

    for j in 1..n {
        let start = txin_starts[j].to_usize() - outpoint_base;
        let end = if j + 1 < txin_starts.len() {
            txin_starts[j + 1].to_usize() - outpoint_base
        } else {
            outpoints.len()
        };

        for op in &outpoints[start..end] {
            if op.is_coinbase() {
                continue;
            }
            let parent_tx = op.tx_index().to_usize();
            if parent_tx >= first_tx && parent_tx < first_tx + n {
                union(&mut parent, j, parent_tx - first_tx);
            }
        }
    }

    let mut cluster_fee = vec![Sats::ZERO; n];
    let mut cluster_vsize = vec![VSize::from(0u64); n];
    for j in 0..n {
        let root = find(&mut parent, j);
        cluster_fee[root] += fees[j];
        cluster_vsize[root] += vsizes[j];
    }

    (0..n)
        .map(|j| {
            let root = find(&mut parent, j);
            FeeRate::from((cluster_fee[root], cluster_vsize[root]))
        })
        .collect()
}

fn find(parent: &mut [usize], mut i: usize) -> usize {
    while parent[i] != i {
        parent[i] = parent[parent[i]];
        i = parent[i];
    }
    i
}

fn union(parent: &mut [usize], a: usize, b: usize) {
    let ra = find(parent, a);
    let rb = find(parent, b);
    if ra != rb {
        parent[ra] = rb;
    }
}
