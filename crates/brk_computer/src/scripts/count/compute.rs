use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::StoredU64;
use vecdb::{Exit, TypedVecIterator};

use super::Vecs;
use crate::{indexes, utils::OptionExt, ComputeIndexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_p2a_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.address.height_to_first_p2aaddressindex,
                    &indexer.vecs.address.p2aaddressindex_to_p2abytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2ms_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.output.height_to_first_p2msoutputindex,
                    &indexer.vecs.output.p2msoutputindex_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2pk33_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.address.height_to_first_p2pk33addressindex,
                    &indexer.vecs.address.p2pk33addressindex_to_p2pk33bytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2pk65_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.address.height_to_first_p2pk65addressindex,
                    &indexer.vecs.address.p2pk65addressindex_to_p2pk65bytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2pkh_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.address.height_to_first_p2pkhaddressindex,
                    &indexer.vecs.address.p2pkhaddressindex_to_p2pkhbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2sh_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.address.height_to_first_p2shaddressindex,
                    &indexer.vecs.address.p2shaddressindex_to_p2shbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2tr_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.address.height_to_first_p2traddressindex,
                    &indexer.vecs.address.p2traddressindex_to_p2trbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2wpkh_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.address.height_to_first_p2wpkhaddressindex,
                    &indexer.vecs.address.p2wpkhaddressindex_to_p2wpkhbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_p2wsh_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.address.height_to_first_p2wshaddressindex,
                    &indexer.vecs.address.p2wshaddressindex_to_p2wshbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_opreturn_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.output.height_to_first_opreturnindex,
                    &indexer.vecs.output.opreturnindex_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_unknownoutput_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.output.height_to_first_unknownoutputindex,
                    &indexer.vecs.output.unknownoutputindex_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_emptyoutput_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.output.height_to_first_emptyoutputindex,
                    &indexer.vecs.output.emptyoutputindex_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        // Compute segwit_count = p2wpkh + p2wsh + p2tr
        let mut p2wsh_iter = self.indexes_to_p2wsh_count.height.u().into_iter();
        let mut p2tr_iter = self.indexes_to_p2tr_count.height.u().into_iter();

        self.indexes_to_segwit_count
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_p2wpkh_count.height.u(),
                    |(h, p2wpkh, ..)| {
                        let sum = *p2wpkh + *p2wsh_iter.get_unwrap(h) + *p2tr_iter.get_unwrap(h);
                        (h, StoredU64::from(sum))
                    },
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
