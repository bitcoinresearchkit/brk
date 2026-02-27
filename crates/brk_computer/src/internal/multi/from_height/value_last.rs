use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{
    indexes, prices,
    internal::{ByUnit, SatsToDollars},
};

#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct ValueFromHeightLast<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    pub base: ByUnit<M>,
}

const VERSION: Version = Version::TWO;

impl ValueFromHeightLast {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;
        Ok(Self {
            base: ByUnit::forced_import(db, name, v, indexes)?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.base.usd.compute_binary::<Sats, Dollars, SatsToDollars>(
            max_from,
            &self.base.sats.height,
            &prices.price.usd,
            exit,
        )?;
        Ok(())
    }
}
