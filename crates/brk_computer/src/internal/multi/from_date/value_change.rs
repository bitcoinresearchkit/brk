//! Change values from DateIndex - stores signed sats (changes can be negative).

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, Sats, SatsSigned, Version};
use vecdb::{CollectableVec, Database, EagerVec, Exit, ImportableVec, IterableCloneableVec, PcoVec};

use crate::{ComputeIndexes, indexes, price};

use super::LazyValueChangeDateDerived;

const VERSION: Version = Version::ZERO;

/// Change values indexed by date - uses signed sats since changes can be negative.
#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct ValueChangeFromDate {
    #[traversable(rename = "sats")]
    pub sats: EagerVec<PcoVec<DateIndex, SatsSigned>>,
    #[traversable(flatten)]
    pub rest: LazyValueChangeDateDerived,
}

impl ValueChangeFromDate {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        compute_dollars: bool,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let sats = EagerVec::forced_import(db, name, version + VERSION)?;

        let rest = LazyValueChangeDateDerived::from_source(
            db,
            name,
            sats.boxed_clone(),
            version + VERSION,
            compute_dollars,
            indexes,
        )?;

        Ok(Self { sats, rest })
    }

    /// Compute N-day change from unsigned sats source and optional dollars source.
    pub fn compute_change(
        &mut self,
        starting_dateindex: DateIndex,
        sats_source: &impl CollectableVec<DateIndex, Sats>,
        dollars_source: Option<&impl CollectableVec<DateIndex, Dollars>>,
        period: usize,
        exit: &Exit,
    ) -> Result<()> {
        self.sats
            .compute_change(starting_dateindex, sats_source, period, exit)?;

        if let (Some(dollars), Some(source)) = (self.rest.dollars.as_mut(), dollars_source) {
            dollars
                .dateindex
                .compute_change(starting_dateindex, source, period, exit)?;
        }

        Ok(())
    }

    /// Compute dollars from price after sats change is computed.
    pub fn compute_dollars_from_price(
        &mut self,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.rest
            .compute_dollars_from_price(price, starting_indexes, exit)
    }
}
