use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Indexes;
use vecdb::Exit;

use super::Vecs;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.p2a.compute(starting_indexes.height, exit, |v| {
            Ok(v.compute_count_from_indexes(
                starting_indexes.height,
                &indexer.vecs.addrs.p2a.first_index,
                &indexer.vecs.addrs.p2a.bytes,
                exit,
            )?)
        })?;

        self.p2ms.compute(starting_indexes.height, exit, |v| {
            Ok(v.compute_count_from_indexes(
                starting_indexes.height,
                &indexer.vecs.scripts.p2ms.first_index,
                &indexer.vecs.scripts.p2ms.to_tx_index,
                exit,
            )?)
        })?;

        self.p2pk33.compute(starting_indexes.height, exit, |v| {
            Ok(v.compute_count_from_indexes(
                starting_indexes.height,
                &indexer.vecs.addrs.p2pk33.first_index,
                &indexer.vecs.addrs.p2pk33.bytes,
                exit,
            )?)
        })?;

        self.p2pk65.compute(starting_indexes.height, exit, |v| {
            Ok(v.compute_count_from_indexes(
                starting_indexes.height,
                &indexer.vecs.addrs.p2pk65.first_index,
                &indexer.vecs.addrs.p2pk65.bytes,
                exit,
            )?)
        })?;

        self.p2pkh.compute(starting_indexes.height, exit, |v| {
            Ok(v.compute_count_from_indexes(
                starting_indexes.height,
                &indexer.vecs.addrs.p2pkh.first_index,
                &indexer.vecs.addrs.p2pkh.bytes,
                exit,
            )?)
        })?;

        self.p2sh.compute(starting_indexes.height, exit, |v| {
            Ok(v.compute_count_from_indexes(
                starting_indexes.height,
                &indexer.vecs.addrs.p2sh.first_index,
                &indexer.vecs.addrs.p2sh.bytes,
                exit,
            )?)
        })?;

        self.p2tr.compute(starting_indexes.height, exit, |v| {
            Ok(v.compute_count_from_indexes(
                starting_indexes.height,
                &indexer.vecs.addrs.p2tr.first_index,
                &indexer.vecs.addrs.p2tr.bytes,
                exit,
            )?)
        })?;

        self.p2wpkh.compute(starting_indexes.height, exit, |v| {
            Ok(v.compute_count_from_indexes(
                starting_indexes.height,
                &indexer.vecs.addrs.p2wpkh.first_index,
                &indexer.vecs.addrs.p2wpkh.bytes,
                exit,
            )?)
        })?;

        self.p2wsh.compute(starting_indexes.height, exit, |v| {
            Ok(v.compute_count_from_indexes(
                starting_indexes.height,
                &indexer.vecs.addrs.p2wsh.first_index,
                &indexer.vecs.addrs.p2wsh.bytes,
                exit,
            )?)
        })?;

        self.op_return.compute(starting_indexes.height, exit, |v| {
            Ok(v.compute_count_from_indexes(
                starting_indexes.height,
                &indexer.vecs.scripts.op_return.first_index,
                &indexer.vecs.scripts.op_return.to_tx_index,
                exit,
            )?)
        })?;

        self.unknown_output
            .compute(starting_indexes.height, exit, |v| {
                Ok(v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.scripts.unknown.first_index,
                    &indexer.vecs.scripts.unknown.to_tx_index,
                    exit,
                )?)
            })?;

        self.empty_output
            .compute(starting_indexes.height, exit, |v| {
                Ok(v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.scripts.empty.first_index,
                    &indexer.vecs.scripts.empty.to_tx_index,
                    exit,
                )?)
            })?;

        Ok(())
    }
}
