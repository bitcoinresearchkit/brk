use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::StoredF32;
use brk_types::StoredU64;
use vecdb::Exit;

use super::Vecs;
use crate::{ComputeIndexes, blocks, outputs};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        count_vecs: &blocks::CountVecs,
        outputs_count: &outputs::CountVecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let window_starts = count_vecs.window_starts();

        self.p2a
            .compute(starting_indexes.height, &window_starts, exit, |v| {
                Ok(v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2aaddressindex,
                    &indexer.vecs.addresses.p2abytes,
                    exit,
                )?)
            })?;

        self.p2ms
            .compute(starting_indexes.height, &window_starts, exit, |v| {
                Ok(v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.scripts.first_p2msoutputindex,
                    &indexer.vecs.scripts.p2ms_to_txindex,
                    exit,
                )?)
            })?;

        self.p2pk33
            .compute(starting_indexes.height, &window_starts, exit, |v| {
                Ok(v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2pk33addressindex,
                    &indexer.vecs.addresses.p2pk33bytes,
                    exit,
                )?)
            })?;

        self.p2pk65
            .compute(starting_indexes.height, &window_starts, exit, |v| {
                Ok(v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2pk65addressindex,
                    &indexer.vecs.addresses.p2pk65bytes,
                    exit,
                )?)
            })?;

        self.p2pkh
            .compute(starting_indexes.height, &window_starts, exit, |v| {
                Ok(v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2pkhaddressindex,
                    &indexer.vecs.addresses.p2pkhbytes,
                    exit,
                )?)
            })?;

        self.p2sh
            .compute(starting_indexes.height, &window_starts, exit, |v| {
                Ok(v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2shaddressindex,
                    &indexer.vecs.addresses.p2shbytes,
                    exit,
                )?)
            })?;

        self.p2tr
            .compute(starting_indexes.height, &window_starts, exit, |v| {
                Ok(v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2traddressindex,
                    &indexer.vecs.addresses.p2trbytes,
                    exit,
                )?)
            })?;

        self.p2wpkh
            .compute(starting_indexes.height, &window_starts, exit, |v| {
                Ok(v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2wpkhaddressindex,
                    &indexer.vecs.addresses.p2wpkhbytes,
                    exit,
                )?)
            })?;

        self.p2wsh
            .compute(starting_indexes.height, &window_starts, exit, |v| {
                Ok(v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.addresses.first_p2wshaddressindex,
                    &indexer.vecs.addresses.p2wshbytes,
                    exit,
                )?)
            })?;

        self.opreturn
            .compute(starting_indexes.height, &window_starts, exit, |v| {
                Ok(v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.scripts.first_opreturnindex,
                    &indexer.vecs.scripts.opreturn_to_txindex,
                    exit,
                )?)
            })?;

        self.unknownoutput
            .compute(starting_indexes.height, &window_starts, exit, |v| {
                Ok(v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.scripts.first_unknownoutputindex,
                    &indexer.vecs.scripts.unknown_to_txindex,
                    exit,
                )?)
            })?;

        self.emptyoutput
            .compute(starting_indexes.height, &window_starts, exit, |v| {
                Ok(v.compute_count_from_indexes(
                    starting_indexes.height,
                    &indexer.vecs.scripts.first_emptyoutputindex,
                    &indexer.vecs.scripts.empty_to_txindex,
                    exit,
                )?)
            })?;

        // Compute segwit = p2wpkh + p2wsh + p2tr
        self.segwit
            .compute(starting_indexes.height, &window_starts, exit, |v| {
                Ok(v.compute_transform3(
                    starting_indexes.height,
                    &self.p2wpkh.height,
                    &self.p2wsh.height,
                    &self.p2tr.height,
                    |(h, p2wpkh, p2wsh, p2tr, ..)| (h, StoredU64::from(*p2wpkh + *p2wsh + *p2tr)),
                    exit,
                )?)
            })?;

        // Adoption ratios: per-block ratio of script type / total outputs
        self.taproot_adoption.height.compute_transform2(
            starting_indexes.height,
            &self.p2tr.height,
            &outputs_count.total_count.full.sum_cumulative.sum.0,
            |(h, p2tr, total, ..)| {
                let ratio = if *total > 0 {
                    StoredF32::from(*p2tr as f64 / *total as f64)
                } else {
                    StoredF32::from(0.0)
                };
                (h, ratio)
            },
            exit,
        )?;

        self.segwit_adoption.height.compute_transform2(
            starting_indexes.height,
            &self.segwit.height,
            &outputs_count.total_count.full.sum_cumulative.sum.0,
            |(h, segwit, total, ..)| {
                let ratio = if *total > 0 {
                    StoredF32::from(*segwit as f64 / *total as f64)
                } else {
                    StoredF32::from(0.0)
                };
                (h, ratio)
            },
            exit,
        )?;

        Ok(())
    }
}
