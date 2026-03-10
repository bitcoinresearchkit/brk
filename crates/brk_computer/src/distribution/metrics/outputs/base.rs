use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, StoredU64, Version};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{distribution::state::{CohortState, RealizedOps}, internal::ComputedPerBlock};

use crate::distribution::metrics::ImportConfig;

/// Base output metrics: utxo_count only (1 stored vec).
#[derive(Traversable)]
pub struct OutputsBase<M: StorageMode = Rw> {
    pub utxo_count: ComputedPerBlock<StoredU64, M>,
}

impl OutputsBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            utxo_count: cfg.import("utxo_count", Version::ZERO)?,
        })
    }

    pub(crate) fn min_len(&self) -> usize {
        self.utxo_count.height.len()
    }

    pub(crate) fn truncate_push(&mut self, height: Height, state: &CohortState<impl RealizedOps>) -> Result<()> {
        self.utxo_count
            .height
            .truncate_push(height, StoredU64::from(state.supply.utxo_count))?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![&mut self.utxo_count.height as &mut dyn AnyStoredVec]
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.utxo_count.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.utxo_count.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }
}
