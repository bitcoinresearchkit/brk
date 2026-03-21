use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPointsSigned32, Height, Indexes, StoredF32, StoredI64, StoredU32, StoredU64, Version,
};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableVec, Rw, StorageMode, WritableVec};

use crate::{
    distribution::{
        metrics::ImportConfig,
        state::{CohortState, CostBasisOps, RealizedOps},
    },
    internal::{PerBlock, PerBlockCumulativeRolling, PerBlockWithDeltas, RatioU64F32},
};

/// Base output metrics: utxo_count + delta.
#[derive(Traversable)]
pub struct OutputsBase<M: StorageMode = Rw> {
    pub unspent_count: PerBlockWithDeltas<StoredU64, StoredI64, BasisPointsSigned32, M>,
    pub spent_count: PerBlockCumulativeRolling<StoredU32, StoredU64, M>,
    pub spending_rate: PerBlock<StoredF32, M>,
}

impl OutputsBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        Ok(Self {
            unspent_count: PerBlockWithDeltas::forced_import(
                cfg.db,
                &cfg.name("utxo_count"),
                cfg.version,
                v1,
                cfg.indexes,
                cfg.cached_starts,
            )?,
            spent_count: cfg.import("spent_utxo_count", v1)?,
            spending_rate: cfg.import("spending_rate", Version::TWO)?,
        })
    }

    pub(crate) fn min_len(&self) -> usize {
        self.unspent_count
            .height
            .len()
            .min(self.spent_count.block.len())
    }

    #[inline(always)]
    pub(crate) fn push_state(&mut self, state: &CohortState<impl RealizedOps, impl CostBasisOps>) {
        self.unspent_count
            .height
            .push(StoredU64::from(state.supply.utxo_count));
        self.spent_count
            .block
            .push(StoredU32::from(state.spent_utxo_count));
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.unspent_count.height as &mut dyn AnyStoredVec,
            &mut self.spent_count.block,
        ]
    }

    pub(crate) fn compute_rest(&mut self, max_from: Height, exit: &Exit) -> Result<()> {
        self.spent_count.compute_rest(max_from, exit)
    }

    pub(crate) fn compute_part2(
        &mut self,
        max_from: Height,
        all_utxo_count: &impl ReadableVec<Height, StoredU64>,
        exit: &Exit,
    ) -> Result<()> {
        self.spending_rate
            .compute_binary::<StoredU64, StoredU64, RatioU64F32>(
                max_from,
                &self.spent_count.sum.0._1y.height,
                all_utxo_count,
                exit,
            )
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.unspent_count.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.unspent_count.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        sum_others!(self, starting_indexes, others, exit; spent_count.block);
        Ok(())
    }
}
