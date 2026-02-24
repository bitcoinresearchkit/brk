use brk_types::Version;
use vecdb::{LazyVecFrom1, ReadableCloneableVec};

use super::super::cents;
use super::Vecs;
use crate::{
    indexes,
    internal::{CentsUnsignedToSats, ComputedHeightDerivedLast, LazyEagerIndexes},
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
        let open =
            LazyEagerIndexes::from_eager_indexes::<CentsUnsignedToSats>("price_sats_open", version, &cents.open);
        let high =
            LazyEagerIndexes::from_eager_indexes::<CentsUnsignedToSats>("price_sats_high", version, &cents.low);
        let low =
            LazyEagerIndexes::from_eager_indexes::<CentsUnsignedToSats>("price_sats_low", version, &cents.high);

        let close = ComputedHeightDerivedLast::forced_import(
            "price_sats_close",
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
