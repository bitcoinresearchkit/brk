//! Value type for derived SumCum pattern (derives from external height source).

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Close, Dollars, Height, Sats, Version};
use vecdb::{Database, Exit, IterableBoxedVec, IterableCloneableVec, IterableVec, LazyVecFrom2};

use crate::{
    ComputeIndexes, indexes,
    internal::{ClosePriceTimesSats, DerivedComputedBlockSumCum, LazyBlockSumCum, SatsToBitcoin},
    price,
};

pub type LazyDollarsHeight = LazyVecFrom2<Height, Dollars, Height, Close<Dollars>, Height, Sats>;

/// Value wrapper for derived SumCum (derives from external height source).
#[derive(Clone, Traversable)]
pub struct DerivedValueBlockSumCum {
    pub sats: DerivedComputedBlockSumCum<Sats>,
    pub bitcoin: LazyBlockSumCum<Bitcoin, Sats>,
    pub dollars_source: Option<LazyDollarsHeight>,
    pub dollars: Option<DerivedComputedBlockSumCum<Dollars>>,
}

const VERSION: Version = Version::ZERO;

impl DerivedValueBlockSumCum {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        sats_source: IterableBoxedVec<Height, Sats>,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = DerivedComputedBlockSumCum::forced_import(
            db,
            name,
            sats_source.boxed_clone(),
            v,
            indexes,
        )?;

        let bitcoin = LazyBlockSumCum::from_derived::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats_source.boxed_clone(),
            &sats,
        );

        let (dollars_source, dollars) = if let Some(price) = price {
            let dollars_source = LazyVecFrom2::transformed::<ClosePriceTimesSats>(
                &format!("{name}_usd"),
                v,
                price.usd.chainindexes_to_price_close.height.boxed_clone(),
                sats_source.boxed_clone(),
            );

            let dollars = DerivedComputedBlockSumCum::forced_import(
                db,
                &format!("{name}_usd"),
                dollars_source.boxed_clone(),
                v,
                indexes,
            )?;

            (Some(dollars_source), Some(dollars))
        } else {
            (None, None)
        };

        Ok(Self {
            sats,
            bitcoin,
            dollars_source,
            dollars,
        })
    }

    /// Derive aggregates from caller-provided sats height source.
    pub fn derive_from(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        sats_source: &impl IterableVec<Height, Sats>,
        exit: &Exit,
    ) -> Result<()> {
        self.sats
            .derive_from(indexes, starting_indexes, sats_source, exit)?;

        if let (Some(dollars), Some(dollars_source)) =
            (self.dollars.as_mut(), self.dollars_source.as_ref())
        {
            dollars.derive_from(indexes, starting_indexes, dollars_source, exit)?;
        }

        Ok(())
    }
}
