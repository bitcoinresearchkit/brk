use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, Height, Indexes, StoredI64, StoredU64, Version};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{
    distribution::{
        metrics::ImportConfig,
        state::{CohortState, CostBasisOps, RealizedOps},
    },
    internal::ComputedPerBlockWithDeltas,
};

/// Base output metrics: utxo_count + delta.
#[derive(Traversable)]
pub struct OutputsBase<M: StorageMode = Rw> {
    pub unspent_count: ComputedPerBlockWithDeltas<StoredU64, StoredI64, BasisPointsSigned32, M>,
}

impl OutputsBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            unspent_count: ComputedPerBlockWithDeltas::forced_import(
                cfg.db,
                &cfg.name("utxo_count"),
                cfg.version,
                Version::ONE,
                cfg.indexes,
                cfg.cached_starts,
            )?,
        })
    }

    pub(crate) fn min_len(&self) -> usize {
        self.unspent_count.height.len()
    }

    pub(crate) fn truncate_push(&mut self, height: Height, state: &CohortState<impl RealizedOps, impl CostBasisOps>) -> Result<()> {
        self.unspent_count
            .height
            .truncate_push(height, StoredU64::from(state.supply.utxo_count))?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![&mut self.unspent_count.height as &mut dyn AnyStoredVec]
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
        Ok(())
    }
}
