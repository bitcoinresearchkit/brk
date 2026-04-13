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
        let dep_version = indexer.vecs.inputs.output_type.version()
            + indexer.vecs.transactions.first_tx_index.version()
            + indexer.vecs.transactions.first_txin_index.version()
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
        let total_txin_len = indexer.vecs.inputs.output_type.len();

        let mut itype_cursor = indexer.vecs.inputs.output_type.cursor();
        let mut fi_in_cursor = indexer.vecs.transactions.first_txin_index.cursor();

        compute_type_counts(
            &mut self.by_type,
            &fi_batch,
            txid_len,
            true,
            starting_indexes.height,
            exit,
            |tx_pos| {
                let fi_in = fi_in_cursor.get(tx_pos).data()?.to_usize();
                let next_fi_in = if tx_pos + 1 < txid_len {
                    fi_in_cursor.get(tx_pos + 1).data()?.to_usize()
                } else {
                    total_txin_len
                };

                let mut seen: u16 = 0;
                itype_cursor.advance(fi_in - itype_cursor.position());
                for _ in fi_in..next_fi_in {
                    seen |= 1u16 << (itype_cursor.next().unwrap() as u8);
                }
                Ok(seen)
            },
        )
    }
}
