use brk_types::Version;
use vecdb::{ReadableCloneableVec, LazyVecFrom1};

use super::super::cents;
use super::Vecs;
use crate::{
    indexes,
    internal::{CentsUnsignedToDollars, ComputedHeightDerivedOHLC, ComputedHeightDerivedSplitOHLC},
};

impl Vecs {
    pub(crate) fn forced_import(
        version: Version,
        indexes: &indexes::Vecs,
        cents: &cents::Vecs,
    ) -> Self {
        let price = LazyVecFrom1::transformed::<CentsUnsignedToDollars>(
            "price_usd",
            version,
            cents.price.read_only_boxed_clone(),
        );

        let split = ComputedHeightDerivedSplitOHLC::forced_import(
            "price",
            version,
            indexes,
            price.read_only_boxed_clone(),
        );

        let ohlc = ComputedHeightDerivedOHLC::forced_import(
            "price_usd",
            version,
            indexes,
            price.read_only_boxed_clone(),
        );

        Self { price, split, ohlc }
    }
}
