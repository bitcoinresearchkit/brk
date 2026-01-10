//! Value type with lazy binary height + stored derived SumCum.
//!
//! Use this when the height-level sats is a lazy binary transform (e.g., mask × source).

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Close, Dollars, Height, Sats, Version};
use schemars::JsonSchema;
use vecdb::{
    BinaryTransform, Database, Exit, IterableBoxedVec, IterableCloneableVec, LazyVecFrom2,
};

use crate::{
    ComputeIndexes, indexes,
    internal::{
        ClosePriceTimesSats, ComputedVecValue, LazyFromHeightSumCum, LazyComputedFromHeightSumCum,
        SatsToBitcoin,
    },
    price,
};

/// Value wrapper with lazy binary height + stored derived SumCum.
///
/// Sats height is a lazy binary transform (e.g., mask × source).
/// Dollars height is also lazy (price × sats).
/// Cumulative and dateindex are stored.
#[derive(Clone, Traversable)]
pub struct LazyValueFromHeightSumCum<S1T, S2T>
where
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub sats: LazyComputedFromHeightSumCum<Sats, S1T, S2T>,
    pub bitcoin: LazyFromHeightSumCum<Bitcoin, Sats>,
    pub dollars: Option<LazyComputedFromHeightSumCum<Dollars, Close<Dollars>, Sats>>,
}

const VERSION: Version = Version::ZERO;

impl<S1T, S2T> LazyValueFromHeightSumCum<S1T, S2T>
where
    S1T: ComputedVecValue + JsonSchema,
    S2T: ComputedVecValue + JsonSchema,
{
    pub fn forced_import<F>(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        source1: IterableBoxedVec<Height, S1T>,
        source2: IterableBoxedVec<Height, S2T>,
        price: Option<&price::Vecs>,
    ) -> Result<Self>
    where
        F: BinaryTransform<S1T, S2T, Sats>,
    {
        let v = version + VERSION;

        let sats_height = LazyVecFrom2::transformed::<F>(name, v, source1, source2);
        let sats = LazyComputedFromHeightSumCum::forced_import(db, name, v, indexes, sats_height)?;

        let bitcoin = LazyFromHeightSumCum::from_derived::<SatsToBitcoin>(
            &format!("{name}_btc"),
            v,
            sats.height.boxed_clone(),
            &sats.rest,
        );

        let dollars = if let Some(price) = price {
            let dollars_height = LazyVecFrom2::transformed::<ClosePriceTimesSats>(
                &format!("{name}_usd"),
                v,
                price.usd.split.close.height.boxed_clone(),
                sats.height.boxed_clone(),
            );

            Some(LazyComputedFromHeightSumCum::forced_import(
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

    /// Derive aggregates from the lazy sats height source.
    pub fn derive_from(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.sats.derive_from(indexes, starting_indexes, exit)?;

        if let Some(dollars) = self.dollars.as_mut() {
            dollars.derive_from(indexes, starting_indexes, exit)?;
        }

        Ok(())
    }
}
