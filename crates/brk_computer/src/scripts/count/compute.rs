use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::StoredU64;
use vecdb::Exit;

use super::Vecs;
use crate::ComputeIndexes;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.p2a.compute(starting_indexes, exit,|v| {
            v.compute_count_from_indexes(
                starting_indexes.height,
                &indexer.vecs.addresses.first_p2aaddressindex,
                &indexer.vecs.addresses.p2abytes,
                exit,
            )?;
            Ok(())
        })?;

        self.p2ms
            .compute(starting_indexes, exit,|v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.scripts.first_p2msoutputindex,
                    &indexer.vecs.scripts.p2ms_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        self.p2pk33
            .compute(starting_indexes, exit,|v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2pk33addressindex,
                    &indexer.vecs.addresses.p2pk33bytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.p2pk65
            .compute(starting_indexes, exit,|v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2pk65addressindex,
                    &indexer.vecs.addresses.p2pk65bytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.p2pkh
            .compute(starting_indexes, exit,|v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2pkhaddressindex,
                    &indexer.vecs.addresses.p2pkhbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.p2sh
            .compute(starting_indexes, exit,|v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2shaddressindex,
                    &indexer.vecs.addresses.p2shbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.p2tr
            .compute(starting_indexes, exit,|v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2traddressindex,
                    &indexer.vecs.addresses.p2trbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.p2wpkh
            .compute(starting_indexes, exit,|v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2wpkhaddressindex,
                    &indexer.vecs.addresses.p2wpkhbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.p2wsh
            .compute(starting_indexes, exit,|v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2wshaddressindex,
                    &indexer.vecs.addresses.p2wshbytes,
                    exit,
                )?;
                Ok(())
            })?;

        self.opreturn
            .compute(starting_indexes, exit,|v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.scripts.first_opreturnindex,
                    &indexer.vecs.scripts.opreturn_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        self.unknownoutput
            .compute(starting_indexes, exit,|v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.scripts.first_unknownoutputindex,
                    &indexer.vecs.scripts.unknown_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        self.emptyoutput
            .compute(starting_indexes, exit,|v| {
                v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.scripts.first_emptyoutputindex,
                    &indexer.vecs.scripts.empty_to_txindex,
                    exit,
                )?;
                Ok(())
            })?;

        // Compute segwit = p2wpkh + p2wsh + p2tr
        self.segwit
            .compute(starting_indexes, exit,|v| {
                v.compute_transform3(
                    starting_indexes.height,
                    &self.p2wpkh.height,
                    &self.p2wsh.height,
                    &self.p2tr.height,
                    |(h, p2wpkh, p2wsh, p2tr, ..)| {
                        (h, StoredU64::from(*p2wpkh + *p2wsh + *p2tr))
                    },
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
