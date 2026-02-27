use brk_types::Version;
use vecdb::{LazyVecFrom1, ReadableCloneableVec};

use super::super::cents;
use super::Vecs;
use crate::prices::{ohlcs::LazyOhlcVecs, split::SplitOhlc};
use crate::{
    indexes,
    internal::{CentsUnsignedToSats, ComputedHeightDerivedLast, LazyEagerIndexes, OhlcCentsToSats},
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

        // Sats are inversely related to cents (sats = 10B/cents), so high↔low are swapped
        let open = LazyEagerIndexes::from_eager_indexes::<CentsUnsignedToSats>(
            "price_open_sats",
            version,
            &cents.split.open,
        );
        let high = LazyEagerIndexes::from_eager_indexes::<CentsUnsignedToSats>(
            "price_high_sats",
            version,
            &cents.split.low,
        );
        let low = LazyEagerIndexes::from_eager_indexes::<CentsUnsignedToSats>(
            "price_low_sats",
            version,
            &cents.split.high,
        );

        let close = ComputedHeightDerivedLast::forced_import(
            "price_close_sats",
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

        // OhlcCentsToSats handles the high↔low swap internally
        let ohlc = LazyOhlcVecs::from_eager_ohlc_indexes::<OhlcCentsToSats>(
            "price_ohlc_sats",
            version,
            &cents.ohlc,
        );

        Self { split, ohlc, price }
    }
}
