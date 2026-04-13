use brk_cohort::ByAddrType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Height, Indexes, OutputType, StoredU64, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, Exit, Rw, StorageMode};

use crate::{
    distribution::addr::WithAddrTypes,
    indexes,
    internal::{
        PerBlockCumulativeRolling, PercentCumulativeRolling, RatioU64Bp16, WindowStartVec, Windows,
    },
    scripts,
};

use super::state::AddrTypeToReusedAddrUseCount;

/// Per-block reused-address-use metrics. A "use" is a single output going
/// to an address (not deduplicated): an address receiving N outputs in one
/// block contributes N. The count only includes uses going to addresses
/// that were *already* reused at the moment of the use, so the use that
/// makes an address reused is not itself counted.
///
/// The denominator for the percent (total address-output count) lives in
/// `scripts::count` and is reused here rather than duplicated.
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
        scripts_count: &scripts::CountVecs,
        exit: &Exit,
    ) -> Result<()> {
        self.reused_addr_use_count
            .compute_rest(starting_indexes.height, exit)?;

        compute_one_percent(
            &mut self.reused_addr_use_percent.all,
            &self.reused_addr_use_count.all,
            &scripts_count.addr_output_count,
            starting_indexes.height,
            exit,
        )?;
        for otype in OutputType::ADDR_TYPES {
            compute_one_percent(
                self.reused_addr_use_percent
                    .by_addr_type
                    .get_mut_unwrap(otype),
                self.reused_addr_use_count.by_addr_type.get_unwrap(otype),
                denom_for_type(scripts_count, otype),
                starting_indexes.height,
                exit,
            )?;
        }
        Ok(())
    }
}

#[inline]
fn compute_one_percent(
    percent: &mut PercentCumulativeRolling<BasisPoints16>,
    reused: &PerBlockCumulativeRolling<StoredU64, StoredU64>,
    denom: &PerBlockCumulativeRolling<StoredU64, StoredU64>,
    starting_height: Height,
    exit: &Exit,
) -> Result<()> {
    percent.compute_binary::<StoredU64, StoredU64, RatioU64Bp16, _, _, _, _>(
        starting_height,
        &reused.cumulative.height,
        &denom.cumulative.height,
        reused.sum.as_array().map(|w| &w.height),
        denom.sum.as_array().map(|w| &w.height),
        exit,
    )
}

#[inline]
fn denom_for_type(
    scripts_count: &scripts::CountVecs,
    otype: OutputType,
) -> &PerBlockCumulativeRolling<StoredU64, StoredU64> {
    match otype {
        OutputType::P2PK33 => &scripts_count.p2pk33,
        OutputType::P2PK65 => &scripts_count.p2pk65,
        OutputType::P2PKH => &scripts_count.p2pkh,
        OutputType::P2SH => &scripts_count.p2sh,
        OutputType::P2WPKH => &scripts_count.p2wpkh,
        OutputType::P2WSH => &scripts_count.p2wsh,
        OutputType::P2TR => &scripts_count.p2tr,
        OutputType::P2A => &scripts_count.p2a,
        _ => unreachable!("OutputType::ADDR_TYPES contains only address types"),
    }
}
