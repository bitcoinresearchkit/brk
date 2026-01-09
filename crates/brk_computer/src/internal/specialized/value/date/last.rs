//! Value type for Last pattern from DateIndex.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{DateIndex, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, EagerVec, Exit, ImportableVec, IterableCloneableVec, PcoVec};

use crate::{ComputeIndexes, indexes, price};

use super::ValueLazyPeriodsLast;

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ValueDateLast {
    #[traversable(rename = "sats")]
    pub sats_dateindex: EagerVec<PcoVec<DateIndex, Sats>>,
    #[deref]
    #[deref_mut]
    pub rest: ValueLazyPeriodsLast,
}

const VERSION: Version = Version::ZERO;

impl ValueDateLast {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        compute_dollars: bool,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let sats_dateindex = EagerVec::forced_import(db, name, version + VERSION)?;

        let rest = ValueLazyPeriodsLast::from_source(
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
        F: FnMut(&mut crate::internal::ComputedDateLast<brk_types::Dollars>) -> Result<()>,
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
}
