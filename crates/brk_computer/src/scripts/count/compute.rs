use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::StoredU64;
use vecdb::{Exit, TypedVecIterator};

use super::Vecs;
use crate::{ComputeIndexes, indexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.p2a.compute_all(indexes, starting_indexes, exit, |v| {
            v.compute_count_from_indexes(
                starting_indexes.height,
                &indexer.vecs.addresses.first_p2aaddressindex,
                &indexer.vecs.addresses.p2abytes,
                exit,
            )?;
            Ok(())
        })?;

        self.p2ms
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.scripts.first_p2msoutputindex,
                    &indexer.vecs.scripts.p2ms_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        self.p2pk33
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2pk33addressindex,
                    &indexer.vecs.addresses.p2pk33bytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.p2pk65
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2pk65addressindex,
                    &indexer.vecs.addresses.p2pk65bytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.p2pkh
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2pkhaddressindex,
                    &indexer.vecs.addresses.p2pkhbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.p2sh
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2shaddressindex,
                    &indexer.vecs.addresses.p2shbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.p2tr
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2traddressindex,
                    &indexer.vecs.addresses.p2trbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.p2wpkh
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2wpkhaddressindex,
                    &indexer.vecs.addresses.p2wpkhbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.p2wsh
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2wshaddressindex,
                    &indexer.vecs.addresses.p2wshbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.opreturn
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.scripts.first_opreturnindex,
                    &indexer.vecs.scripts.opreturn_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        self.unknownoutput
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.scripts.first_unknownoutputindex,
                    &indexer.vecs.scripts.unknown_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        self.emptyoutput
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.scripts.first_emptyoutputindex,
                    &indexer.vecs.scripts.empty_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        // Compute segwit = p2wpkh + p2wsh + p2tr
        let mut p2wsh_iter = self.p2wsh.height.into_iter();
        let mut p2tr_iter = self.p2tr.height.into_iter();

        self.segwit
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.p2wpkh.height,
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
