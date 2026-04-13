use brk_error::{OptionData, Result};
use brk_indexer::Indexer;
use brk_types::Indexes;
use vecdb::{AnyVec, Exit, ReadableVec, VecIndex, WritableVec};

use super::{super::type_counts::compute_type_counts, Vecs};

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

        for (_, v) in self.by_type.iter_mut() {
            v.block
                .validate_and_truncate(dep_version, starting_indexes.height)?;
        }

        let skip = self.by_type.values().map(|v| v.block.len()).min().unwrap();

        let first_tx_index = &indexer.vecs.transactions.first_tx_index;
        let end = first_tx_index.len();
        if skip >= end {
            return Ok(());
        }

        for (_, v) in self.by_type.iter_mut() {
            v.block.truncate_if_needed_at(skip)?;
        }

        let fi_batch = first_tx_index.collect_range_at(skip, end);
        let txid_len = indexer.vecs.transactions.txid.len();
        let total_txout_len = indexer.vecs.outputs.output_type.len();

        let mut otype_cursor = indexer.vecs.outputs.output_type.cursor();
        let mut fo_cursor = indexer.vecs.transactions.first_txout_index.cursor();

        compute_type_counts(
            &mut self.by_type,
            &fi_batch,
            txid_len,
            false,
            starting_indexes.height,
            exit,
            |tx_pos| {
                let fo = fo_cursor.get(tx_pos).data()?.to_usize();
                let next_fo = if tx_pos + 1 < txid_len {
                    fo_cursor.get(tx_pos + 1).data()?.to_usize()
                } else {
                    total_txout_len
                };

                let mut seen: u16 = 0;
                otype_cursor.advance(fo - otype_cursor.position());
                for _ in fo..next_fo {
                    seen |= 1u16 << (otype_cursor.next().unwrap() as u8);
                }
                Ok(seen)
            },
        )
    }
}
