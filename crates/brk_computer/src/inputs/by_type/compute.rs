use brk_error::{OptionData, Result};
use brk_indexer::Indexer;
use brk_types::{Indexes, StoredU64};
use vecdb::{AnyVec, Exit, ReadableVec, VecIndex, WritableVec};

use super::{Vecs, WithInputTypes};
use crate::internal::{CoinbasePolicy, PerBlockCumulativeRolling, walk_blocks};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let dep_version = indexer.vecs.inputs.output_type.version()
            + indexer.vecs.transactions.first_tx_index.version()
            + indexer.vecs.transactions.first_txin_index.version()
            + indexer.vecs.transactions.txid.version();

        self.input_count
            .validate_and_truncate(dep_version, starting_indexes.height)?;
        self.tx_count
            .validate_and_truncate(dep_version, starting_indexes.height)?;

        let skip = self
            .input_count
            .min_stateful_len()
            .min(self.tx_count.min_stateful_len());

        let first_tx_index = &indexer.vecs.transactions.first_tx_index;
        let end = first_tx_index.len();
        if skip < end {
            self.input_count.truncate_if_needed_at(skip)?;
            self.tx_count.truncate_if_needed_at(skip)?;

            let fi_batch = first_tx_index.collect_range_at(skip, end);
            let txid_len = indexer.vecs.transactions.txid.len();
            let total_txin_len = indexer.vecs.inputs.output_type.len();

            let mut itype_cursor = indexer.vecs.inputs.output_type.cursor();
            let mut fi_in_cursor = indexer.vecs.transactions.first_txin_index.cursor();

            walk_blocks(
                &fi_batch,
                txid_len,
                CoinbasePolicy::Skip,
                |tx_pos, per_tx| {
                    let fi_in = fi_in_cursor.get(tx_pos).data()?.to_usize();
                    let next_fi_in = if tx_pos + 1 < txid_len {
                        fi_in_cursor.get(tx_pos + 1).data()?.to_usize()
                    } else {
                        total_txin_len
                    };

                    itype_cursor.advance(fi_in - itype_cursor.position());
                    for _ in fi_in..next_fi_in {
                        let otype = itype_cursor.next().unwrap();
                        per_tx[otype as usize] += 1;
                    }
                    Ok(())
                },
                |agg| {
                    push_block(&mut self.input_count, agg.entries_all, &agg.entries_per_type);
                    push_block(&mut self.tx_count, agg.txs_all, &agg.txs_per_type);

                    if self.input_count.all.block.batch_limit_reached() {
                        let _lock = exit.lock();
                        self.input_count.write()?;
                        self.tx_count.write()?;
                    }
                    Ok(())
                },
            )?;

            {
                let _lock = exit.lock();
                self.input_count.write()?;
                self.tx_count.write()?;
            }

            self.input_count
                .compute_rest(starting_indexes.height, exit)?;
            self.tx_count
                .compute_rest(starting_indexes.height, exit)?;
        }

        for (otype, source) in self.tx_count.by_type.iter_typed() {
            self.tx_percent.get_mut(otype).compute_count_ratio(
                source,
                &self.tx_count.all,
                starting_indexes.height,
                exit,
            )?;
        }
        Ok(())
    }
}

#[inline]
fn push_block(
    metric: &mut WithInputTypes<PerBlockCumulativeRolling<StoredU64, StoredU64>>,
    total: u64,
    per_type: &[u64; 12],
) {
    metric.all.block.push(StoredU64::from(total));
    for (otype, vec) in metric.by_type.iter_typed_mut() {
        vec.block.push(StoredU64::from(per_type[otype as usize]));
    }
}
