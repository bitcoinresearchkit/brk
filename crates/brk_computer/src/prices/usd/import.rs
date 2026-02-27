use brk_types::Version;
use vecdb::{LazyVecFrom1, ReadableCloneableVec};

use super::super::cents;
use super::Vecs;
use crate::prices::{ohlcs::LazyOhlcVecs, split::SplitOhlc};
use crate::{
    indexes,
    internal::{
        CentsUnsignedToDollars, ComputedHeightDerivedLast, LazyEagerIndexes, OhlcCentsToDollars,
    },
};

impl Vecs {
    pub(crate) fn forced_import(
        version: Version,
        indexes: &indexes::Vecs,
        cents: &cents::Vecs,
    ) -> Self {
        let price = LazyVecFrom1::transformed::<CentsUnsignedToDollars>(
            "price",
            version,
            cents.price.read_only_boxed_clone(),
        );

        // Dollars are monotonically increasing from cents, so open→open, high→high, low→low
        let open = LazyEagerIndexes::from_eager_indexes::<CentsUnsignedToDollars>(
            "price_open",
            version,
            &cents.split.open,
        );
        let high = LazyEagerIndexes::from_eager_indexes::<CentsUnsignedToDollars>(
            "price_high",
            version,
            &cents.split.high,
        );
        let low = LazyEagerIndexes::from_eager_indexes::<CentsUnsignedToDollars>(
            "price_low",
            version,
            &cents.split.low,
        );

        let close = ComputedHeightDerivedLast::forced_import(
            "price_close",
            price.read_only_boxed_clone(),
            version,
            indexes,
        );

        let split = SplitOhlc {
            open,
            high,
            low,
            close,
        };

        let ohlc = LazyOhlcVecs::from_eager_ohlc_indexes::<OhlcCentsToDollars>(
            "price_ohlc",
            version,
            &cents.ohlc,
        );

        Self { split, ohlc, price }
    }
}
