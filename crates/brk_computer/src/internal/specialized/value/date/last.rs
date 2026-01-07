//! Value type for Last pattern from DateIndex.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{DateIndex, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, EagerVec, Exit, ImportableVec, IterableCloneableVec, PcoVec};

use crate::{ComputeIndexes, indexes, price};

use super::ValueDerivedDateLast;

#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct ValueDateLast {
    #[traversable(rename = "sats")]
    pub sats_dateindex: EagerVec<PcoVec<DateIndex, Sats>>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: ValueDerivedDateLast,
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

        let rest = ValueDerivedDateLast::from_source(
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

    pub fn compute_all<F>(
        &mut self,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<DateIndex, Sats>>) -> Result<()>,
    {
        compute(&mut self.sats_dateindex)?;
        self.rest.compute_rest(price, starting_indexes, exit)?;
        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.rest.compute_rest(price, starting_indexes, exit)
    }
}
