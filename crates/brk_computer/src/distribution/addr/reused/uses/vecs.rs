use brk_cohort::ByAddrType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Indexes, OutputType, StoredU64, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, Exit, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        PerBlockCumulativeRolling, PercentCumulativeRolling, WindowStartVec, Windows,
        WithAddrTypes,
    },
    outputs,
};

use super::state::AddrTypeToReusedAddrUseCount;

/// Per-block reused-address-use metrics. A "use" is a single output going
/// to an address (not deduplicated): an address receiving N outputs in one
/// block contributes N. The count only includes uses going to addresses
/// that were *already* reused at the moment of the use, so the use that
/// makes an address reused is not itself counted.
///
/// The denominator for the percent (per-type and aggregate address-output
/// counts) is read from `outputs::ByTypeVecs::output_count` rather than
/// duplicated here.
#[derive(Traversable)]
pub struct ReusedAddrUsesVecs<M: StorageMode = Rw> {
    pub reused_addr_use_count:
        WithAddrTypes<PerBlockCumulativeRolling<StoredU64, StoredU64, M>>,
    pub reused_addr_use_percent: WithAddrTypes<PercentCumulativeRolling<BasisPoints16, M>>,
}

impl ReusedAddrUsesVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        let reused_addr_use_count =
            WithAddrTypes::<PerBlockCumulativeRolling<StoredU64, StoredU64>>::forced_import(
                db,
                "reused_addr_use_count",
                version,
                indexes,
                cached_starts,
            )?;
        let percent_name = "reused_addr_use_percent";
        let reused_addr_use_percent = WithAddrTypes {
            all: PercentCumulativeRolling::forced_import(db, percent_name, version, indexes)?,
            by_addr_type: ByAddrType::new_with_name(|type_name| {
                PercentCumulativeRolling::forced_import(
                    db,
                    &format!("{type_name}_{percent_name}"),
                    version,
                    indexes,
                )
            })?,
        };
        Ok(Self {
            reused_addr_use_count,
            reused_addr_use_percent,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.reused_addr_use_count.min_stateful_len()
    }

    pub(crate) fn par_iter_height_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.reused_addr_use_count.par_iter_height_mut()
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
        self.reused_addr_use_count.reset_height()
    }

    #[inline(always)]
    pub(crate) fn push_height(&mut self, reused: &AddrTypeToReusedAddrUseCount) {
        self.reused_addr_use_count
            .push_height(reused.sum(), reused.values().copied());
    }

    pub(crate) fn compute_rest(
        &mut self,
        starting_indexes: &Indexes,
        outputs_by_type: &outputs::ByTypeVecs,
        exit: &Exit,
    ) -> Result<()> {
        self.reused_addr_use_count
            .compute_rest(starting_indexes.height, exit)?;

        self.reused_addr_use_percent.all.compute_count_ratio(
            &self.reused_addr_use_count.all,
            &outputs_by_type.output_count.all,
            starting_indexes.height,
            exit,
        )?;
        for otype in OutputType::ADDR_TYPES {
            self.reused_addr_use_percent
                .by_addr_type
                .get_mut_unwrap(otype)
                .compute_count_ratio(
                    self.reused_addr_use_count.by_addr_type.get_unwrap(otype),
                    outputs_by_type.output_count.by_type.get(otype),
                    starting_indexes.height,
                    exit,
                )?;
        }
        Ok(())
    }
}
