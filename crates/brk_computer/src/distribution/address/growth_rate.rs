//! Growth rate: new_addr_count / addr_count (global + per-type)

use brk_cohort::ByAddressType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredF32, StoredU64, Version};
use vecdb::{Database, EagerVec, Exit, PcoVec, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedFromHeightDistribution, WindowStarts},
};

use super::{AddrCountsVecs, NewAddrCountVecs};

/// Growth rate: new_addr_count / addr_count (global + per-type)
#[derive(Traversable)]
pub struct GrowthRateVecs<M: StorageMode = Rw> {
    pub all: ComputedFromHeightDistribution<StoredF32, M>,
    #[traversable(flatten)]
    pub by_addresstype: ByAddressType<ComputedFromHeightDistribution<StoredF32, M>>,
}

impl GrowthRateVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let all = ComputedFromHeightDistribution::forced_import(
            db,
            "growth_rate",
            version,
            indexes,
        )?;

        let by_addresstype: ByAddressType<ComputedFromHeightDistribution<StoredF32>> =
            ByAddressType::new_with_name(|name| {
                ComputedFromHeightDistribution::forced_import(
                    db,
                    &format!("{name}_growth_rate"),
                    version,
                    indexes,
                )
            })?;

        Ok(Self { all, by_addresstype })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        new_addr_count: &NewAddrCountVecs,
        addr_count: &AddrCountsVecs,
        exit: &Exit,
    ) -> Result<()> {
        self.all.compute(max_from, windows, exit, |target| {
            compute_ratio(
                target,
                max_from,
                &new_addr_count.all.height,
                &addr_count.all.count.height,
                exit,
            )
        })?;

        for ((_, growth), ((_, new), (_, addr))) in self
            .by_addresstype
            .iter_mut()
            .zip(
                new_addr_count
                    .by_addresstype
                    .iter()
                    .zip(addr_count.by_addresstype.iter()),
            )
        {
            growth.compute(max_from, windows, exit, |target| {
                compute_ratio(
                    target,
                    max_from,
                    &new.height,
                    &addr.count.height,
                    exit,
                )
            })?;
        }

        Ok(())
    }
}

fn compute_ratio(
    target: &mut EagerVec<PcoVec<Height, StoredF32>>,
    max_from: Height,
    numerator: &impl ReadableVec<Height, StoredU64>,
    denominator: &impl ReadableVec<Height, StoredU64>,
    exit: &Exit,
) -> Result<()> {
    target.compute_transform2(
        max_from,
        numerator,
        denominator,
        |(h, num, den, ..)| {
            let n = *num as f64;
            let d = *den as f64;
            let ratio = if d == 0.0 { 0.0 } else { n / d };
            (h, StoredF32::from(ratio))
        },
        exit,
    )?;
    Ok(())
}
