use brk_error::{OptionData, Result};
use brk_indexer::Indexer;
use brk_types::{Indexes, StoredU64};
use vecdb::{AnyVec, Exit, ReadableVec, VecIndex, WritableVec};

use super::Vecs;
use crate::internal::{
    PerBlockFull, compute_by_addr_type_block_counts, compute_by_addr_type_tx_percents,
};

impl Vecs {
    /// Phase 1: walk inputs and populate `input_count` + `tx_count`.
    /// Independent of transactions, can run alongside other inputs work.
    pub(crate) fn compute_counts(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let dep_version = indexer.vecs.inputs.output_type.version()
            + indexer.vecs.transactions.first_tx_index.version()
            + indexer.vecs.transactions.first_txin_index.version()
            + indexer.vecs.transactions.txid.version();

        for (_, v) in self.input_count.iter_mut() {
            v.block
                .validate_and_truncate(dep_version, starting_indexes.height)?;
        }
        for (_, v) in self.tx_count.iter_mut() {
            v.block
                .validate_and_truncate(dep_version, starting_indexes.height)?;
        }

        let skip = self
            .input_count
            .values()
            .map(|v| v.block.len())
            .min()
            .unwrap()
            .min(self.tx_count.values().map(|v| v.block.len()).min().unwrap());

        let first_tx_index = &indexer.vecs.transactions.first_tx_index;
        let end = first_tx_index.len();
        if skip >= end {
            return Ok(());
        }

        for (_, v) in self.input_count.iter_mut() {
            v.block.truncate_if_needed_at(skip)?;
        }
        for (_, v) in self.tx_count.iter_mut() {
            v.block.truncate_if_needed_at(skip)?;
        }

        let fi_batch = first_tx_index.collect_range_at(skip, end);
        let txid_len = indexer.vecs.transactions.txid.len();
        let total_txin_len = indexer.vecs.inputs.output_type.len();

        let mut itype_cursor = indexer.vecs.inputs.output_type.cursor();
        let mut fi_in_cursor = indexer.vecs.transactions.first_txin_index.cursor();

        compute_by_addr_type_block_counts(
            &mut self.input_count,
            &mut self.tx_count,
            &fi_batch,
            txid_len,
            true, // skip coinbase (1 fake input)
            starting_indexes.height,
            exit,
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
        )
    }

    /// Phase 2: derive `tx_percent` from `tx_count` and the total tx count.
    /// Must run after `transactions::Vecs::compute`.
    pub(crate) fn compute_percents(
        &mut self,
        transactions_count_total: &PerBlockFull<StoredU64>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        compute_by_addr_type_tx_percents(
            &self.tx_count,
            &mut self.tx_percent,
            transactions_count_total,
            starting_indexes,
            exit,
        )
    }
}
