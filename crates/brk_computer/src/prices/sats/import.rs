use brk_types::Version;
use vecdb::{ReadableCloneableVec, LazyVecFrom1};

use super::super::cents;
use super::Vecs;
use crate::{
    indexes,
    internal::{CentsUnsignedToSats, ComputedHeightDerivedOHLC, ComputedHeightDerivedSplitOHLC},
};

impl Vecs {
    pub(crate) fn forced_import(
        version: Version,
        indexes: &indexes::Vecs,
        cents: &cents::Vecs,
    ) -> Self {
        let price = LazyVecFrom1::transformed::<CentsUnsignedToSats>(
            "price_sats",
            version,
            cents.price.read_only_boxed_clone(),
        );

        let split = ComputedHeightDerivedSplitOHLC::forced_import(
            "price_sats",
            version,
            indexes,
            price.read_only_boxed_clone(),
        );

        let ohlc = ComputedHeightDerivedOHLC::forced_import(
            "price_sats",
            version,
            indexes,
            price.read_only_boxed_clone(),
        );

        Self { price, split, ohlc }
    }
}
