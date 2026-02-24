use brk_types::Version;
use vecdb::{LazyVecFrom1, ReadableCloneableVec};

use super::super::cents;
use super::Vecs;
use crate::{
    indexes,
    internal::{CentsUnsignedToDollars, ComputedHeightDerivedLast, LazyEagerIndexes},
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

        // Dollars are monotonically increasing from cents, so open→open, high→high, low→low
        let open =
            LazyEagerIndexes::from_eager_indexes::<CentsUnsignedToDollars>("price_usd_open", version, &cents.open);
        let high =
            LazyEagerIndexes::from_eager_indexes::<CentsUnsignedToDollars>("price_usd_high", version, &cents.high);
        let low =
            LazyEagerIndexes::from_eager_indexes::<CentsUnsignedToDollars>("price_usd_low", version, &cents.low);

        let close = ComputedHeightDerivedLast::forced_import(
            "price_usd_close",
            price.read_only_boxed_clone(),
            version,
            indexes,
        );

        Self {
            price,
            open,
            high,
            low,
            close,
        }
    }
}
