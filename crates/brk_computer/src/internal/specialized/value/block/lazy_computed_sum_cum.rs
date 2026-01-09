//! Value type with stored height + lazy dollars for SumCum pattern.
//!
//! Use this when:
//! - Sats height is stored (primary source of truth)
//! - Sats indexes are derived from height
//! - Bitcoin is lazy (transform from sats)
//! - Dollars height is lazy (price × sats), with stored indexes

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Close, Dollars, Sats, Version};
use vecdb::{Database, Exit, IterableCloneableVec, LazyVecFrom2};

use crate::{
    ComputeIndexes, indexes,
    internal::{
        ClosePriceTimesSats, ComputedBlockSumCum, LazyBlockSumCum, BlockSumCumLazyHeight,
        SatsToBitcoin,
    },
    price,
};

/// Value wrapper with stored sats height + lazy dollars.
///
/// Sats height is stored (computed directly or from stateful loop).
/// Dollars height is lazy (price × sats).
/// Cumulative and dateindex aggregates are stored for both.
#[derive(Clone, Traversable)]
pub struct LazyComputedValueBlockSumCum {
    pub sats: ComputedBlockSumCum<Sats>,
    pub bitcoin: LazyBlockSumCum<Bitcoin, Sats>,
    pub dollars: Option<BlockSumCumLazyHeight<Dollars, Close<Dollars>, Sats>>,
}

const VERSION: Version = Version::ZERO;

impl LazyComputedValueBlockSumCum {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let v = version + VERSION;

        let sats = ComputedBlockSumCum::forced_import(db, name, v, indexes)?;

        let bitcoin = LazyBlockSumCum::from_computed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.height.boxed_clone(),
            &sats,
        );

        let dollars = if let Some(price) = price {
            let dollars_height = LazyVecFrom2::transformed::<ClosePriceTimesSats>(
                &format!("{name}_usd"),
                v,
                price.usd.split.close.height.boxed_clone(),
                sats.height.boxed_clone(),
            );

            Some(BlockSumCumLazyHeight::forced_import(
                db,
                &format!("{name}_usd"),
                v,
                indexes,
                dollars_height,
            )?)
        } else {
            None
        };

        Ok(Self {
            sats,
            bitcoin,
            dollars,
        })
    }

    /// Compute rest (derived indexes) from already-computed height.
    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.sats.compute_rest(indexes, starting_indexes, exit)?;

        if let Some(dollars) = self.dollars.as_mut() {
            dollars.derive_from(indexes, starting_indexes, exit)?;
        }

        Ok(())
    }
}
