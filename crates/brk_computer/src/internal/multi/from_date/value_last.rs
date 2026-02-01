//! Value type for Last pattern from DateIndex.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{CollectableVec, Database, EagerVec, Exit, ImportableVec, IterableCloneableVec, PcoVec};

use crate::{ComputeIndexes, indexes, price};

use super::{ComputedFromDateLast, LazyValueDateDerivedLast};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ValueFromDateLast {
    #[traversable(rename = "sats")]
    pub sats_dateindex: EagerVec<PcoVec<DateIndex, Sats>>,
    #[deref]
    #[deref_mut]
    pub rest: LazyValueDateDerivedLast,
}

const VERSION: Version = Version::ZERO;

impl ValueFromDateLast {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        compute_dollars: bool,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let sats_dateindex = EagerVec::forced_import(db, name, version + VERSION)?;

        let rest = LazyValueDateDerivedLast::from_source(
            db,
            name,
            sats_dateindex.boxed_clone(),
            version + VERSION,
            compute_dollars,
            indexes,
        )?;

        Ok(Self {
            sats_dateindex,
            rest,
        })
    }

    pub fn compute_sats<F>(&mut self, mut compute: F) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<DateIndex, Sats>>) -> Result<()>,
    {
        compute(&mut self.sats_dateindex)
    }

    pub fn compute_all<F>(
        &mut self,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<DateIndex, Sats>>) -> Result<()>,
    {
        self.compute_sats(compute)?;
        self.compute_dollars_from_price(price, starting_indexes, exit)
    }

    pub fn compute_dollars<F>(&mut self, compute: F) -> Result<()>
    where
        F: FnMut(&mut ComputedFromDateLast<Dollars>) -> Result<()>,
    {
        self.rest.compute_dollars(compute)
    }

    pub fn compute_dollars_from_price(
        &mut self,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.rest
            .compute_dollars_from_price(price, starting_indexes, exit)
    }

    /// Compute both sats and dollars using provided closures.
    pub fn compute_both<S, D>(
        &mut self,
        compute_sats: S,
        compute_dollars: D,
    ) -> Result<()>
    where
        S: FnOnce(&mut EagerVec<PcoVec<DateIndex, Sats>>) -> Result<()>,
        D: FnOnce(&mut ComputedFromDateLast<Dollars>) -> Result<()>,
    {
        compute_sats(&mut self.sats_dateindex)?;
        if let Some(dollars) = self.rest.dollars.as_mut() {
            compute_dollars(dollars)?;
        }
        Ok(())
    }

    /// Compute EMA for sats and optionally dollars from source vecs.
    pub fn compute_ema(
        &mut self,
        starting_dateindex: DateIndex,
        sats_source: &impl CollectableVec<DateIndex, Sats>,
        dollars_source: Option<&impl CollectableVec<DateIndex, Dollars>>,
        period: usize,
        exit: &Exit,
    ) -> Result<()> {
        self.sats_dateindex
            .compute_ema(starting_dateindex, sats_source, period, exit)?;

        if let (Some(dollars), Some(source)) = (self.rest.dollars.as_mut(), dollars_source) {
            dollars
                .dateindex
                .compute_ema(starting_dateindex, source, period, exit)?;
        }

        Ok(())
    }

    /// Compute N-day change for sats and optionally dollars from source vecs.
    pub fn compute_change(
        &mut self,
        starting_dateindex: DateIndex,
        sats_source: &impl CollectableVec<DateIndex, Sats>,
        dollars_source: Option<&impl CollectableVec<DateIndex, Dollars>>,
        period: usize,
        exit: &Exit,
    ) -> Result<()> {
        self.sats_dateindex
            .compute_change(starting_dateindex, sats_source, period, exit)?;

        if let (Some(dollars), Some(source)) = (self.rest.dollars.as_mut(), dollars_source) {
            dollars
                .dateindex
                .compute_change(starting_dateindex, source, period, exit)?;
        }

        Ok(())
    }
}
