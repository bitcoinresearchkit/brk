//! Value type for Last pattern from Height.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use vecdb::{Database, EagerVec, Exit, IterableCloneableVec, PcoVec};

use crate::{
    ComputeIndexes, indexes,
    internal::{ComputedBlockLast, LazyBlockLast, SatsToBitcoin},
    price,
    traits::ComputeFromBitcoin,
};

#[derive(Clone, Traversable)]
pub struct ValueBlockLast {
    pub sats: ComputedBlockLast<Sats>,
    pub bitcoin: LazyBlockLast<Bitcoin, Sats>,
    pub dollars: Option<ComputedBlockLast<Dollars>>,
}

const VERSION: Version = Version::ZERO;

impl ValueBlockLast {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        compute_dollars: bool,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = ComputedBlockLast::forced_import(db, name, v, indexes)?;

        let bitcoin = LazyBlockLast::from_computed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.height.boxed_clone(),
            &sats,
        );

        let dollars = compute_dollars
            .then(|| ComputedBlockLast::forced_import(db, &format!("{name}_usd"), v, indexes))
            .transpose()?;

        Ok(Self {
            sats,
            bitcoin,
            dollars,
        })
    }

    pub fn compute_all<F>(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<Height, Sats>>) -> Result<()>,
    {
        // Compute sats (closure receives &mut height vec)
        self.sats
            .compute_all(indexes, starting_indexes, exit, |v| compute(v))?;

        self.compute_dollars(indexes, price, starting_indexes, exit)
    }

    /// Compute derived vecs from existing height data.
    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.sats.compute_rest(indexes, starting_indexes, exit)?;
        self.compute_dollars(indexes, price, starting_indexes, exit)
    }

    fn compute_dollars(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        if let (Some(dollars), Some(price)) = (self.dollars.as_mut(), price) {
            let height_to_bitcoin = &self.bitcoin.height;
            let height_to_price_close = &price.usd.split.close.height;

            dollars.compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_from_bitcoin(
                    starting_indexes.height,
                    height_to_bitcoin,
                    height_to_price_close,
                    exit,
                )
            })?;
        }

        Ok(())
    }
}
