use brk_error::{OptionData, Result};
use brk_indexer::Indexer;
use brk_types::{Indexes, OutputType, StoredU64};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableVec, VecIndex, WritableVec};

use super::{Vecs, WithOutputTypes};
use crate::internal::{CoinbasePolicy, PerBlockCumulativeRolling, walk_blocks};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let dep_version = indexer.vecs.outputs.output_type.version()
            + indexer.vecs.transactions.first_tx_index.version()
            + indexer.vecs.transactions.first_txout_index.version()
            + indexer.vecs.transactions.txid.version();

        self.output_count
            .validate_and_truncate(dep_version, starting_indexes.height)?;
        self.spendable_output_count
            .block
            .validate_and_truncate(dep_version, starting_indexes.height)?;
        self.tx_count
            .validate_and_truncate(dep_version, starting_indexes.height)?;

        let skip = self
            .output_count
            .min_stateful_len()
            .min(self.spendable_output_count.block.len())
            .min(self.tx_count.min_stateful_len());

        let first_tx_index = &indexer.vecs.transactions.first_tx_index;
        let end = first_tx_index.len();
        if skip < end {
            self.output_count.truncate_if_needed_at(skip)?;
            self.spendable_output_count
                .block
                .truncate_if_needed_at(skip)?;
            self.tx_count.truncate_if_needed_at(skip)?;

            let fi_batch = first_tx_index.collect_range_at(skip, end);
            let txid_len = indexer.vecs.transactions.txid.len();
            let total_txout_len = indexer.vecs.outputs.output_type.len();

            let mut otype_cursor = indexer.vecs.outputs.output_type.cursor();
            let mut fo_cursor = indexer.vecs.transactions.first_txout_index.cursor();

            walk_blocks(
                &fi_batch,
                txid_len,
                CoinbasePolicy::Include,
                |tx_pos, per_tx| {
                    let fo = fo_cursor.get(tx_pos).data()?.to_usize();
                    let next_fo = if tx_pos + 1 < txid_len {
                        fo_cursor.get(tx_pos + 1).data()?.to_usize()
                    } else {
                        total_txout_len
                    };

                    otype_cursor.advance(fo - otype_cursor.position());
                    for _ in fo..next_fo {
                        let otype = otype_cursor.next().unwrap();
                        per_tx[otype as usize] += 1;
                    }
                    Ok(())
                },
                |agg| {
                    push_block(&mut self.output_count, agg.entries_all, &agg.entries_per_type);
                    push_block(&mut self.tx_count, agg.txs_all, &agg.txs_per_type);
                    let spendable_total = agg.entries_all
                        - agg.entries_per_type[OutputType::OpReturn as usize];
                    self.spendable_output_count
                        .block
                        .push(StoredU64::from(spendable_total));

                    if self.output_count.all.block.batch_limit_reached() {
                        let _lock = exit.lock();
                        self.output_count.write()?;
                        self.spendable_output_count.block.write()?;
                        self.tx_count.write()?;
                    }
                    Ok(())
                },
            )?;

            {
                let _lock = exit.lock();
                self.output_count.write()?;
                self.spendable_output_count.block.write()?;
                self.tx_count.write()?;
            }

            self.output_count
                .compute_rest(starting_indexes.height, exit)?;
            self.spendable_output_count
                .compute_rest(starting_indexes.height, exit)?;
            self.tx_count
                .compute_rest(starting_indexes.height, exit)?;
        }

        for (otype, source) in self.output_count.by_type.iter_typed() {
            self.output_share.get_mut(otype).compute_count_ratio(
                source,
                &self.output_count.all,
                starting_indexes.height,
                exit,
            )?;
        }

        for (otype, source) in self.tx_count.by_type.iter_typed() {
            self.tx_share.get_mut(otype).compute_count_ratio(
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
    metric: &mut WithOutputTypes<PerBlockCumulativeRolling<StoredU64, StoredU64>>,
    total: u64,
    per_type: &[u64; 12],
) {
    metric.all.block.push(StoredU64::from(total));
    for (otype, vec) in metric.by_type.iter_typed_mut() {
        vec.block.push(StoredU64::from(per_type[otype as usize]));
    }
}
