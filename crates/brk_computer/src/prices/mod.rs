pub(crate) mod by_unit;
mod compute;
pub(crate) mod ohlcs;

use std::path::Path;

use brk_traversable::Traversable;
use brk_types::Version;
use vecdb::{Database, ReadableCloneableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{
        CentsUnsignedToDollars, CentsUnsignedToSats, PerBlock, Resolutions,
        EagerIndexes, LazyEagerIndexes, LazyPerBlock, OhlcCentsToDollars, OhlcCentsToSats,
        db_utils::{finalize_db, open_db},
    },
};

use by_unit::{OhlcByUnit, PriceByUnit, SplitByUnit, SplitCloseByUnit, SplitIndexesByUnit};
use ohlcs::{LazyOhlcVecs, OhlcVecs};

pub const DB_NAME: &str = "prices";

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub(crate) db: Database,

    pub split: SplitByUnit<M>,
    pub ohlc: OhlcByUnit<M>,
    pub spot: PriceByUnit<M>,
}

impl Vecs {
    pub(crate) fn forced_import(
        parent: &Path,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> brk_error::Result<Self> {
        let db = open_db(parent, DB_NAME, 1_000_000)?;
        let this = Self::forced_import_inner(&db, version, indexes)?;
        finalize_db(&this.db, &this)?;
        Ok(this)
    }

    fn forced_import_inner(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> brk_error::Result<Self> {
        let version = version + Version::new(11);

        let price_cents = PerBlock::forced_import(db, "price_cents", version, indexes)?;

        let open_cents = EagerIndexes::forced_import(db, "price_open_cents", version)?;
        let high_cents = EagerIndexes::forced_import(db, "price_high_cents", version)?;
        let low_cents = EagerIndexes::forced_import(db, "price_low_cents", version)?;

        let close_cents = Resolutions::forced_import(
            "price_close_cents",
            price_cents.height.read_only_boxed_clone(),
            version,
            indexes,
        );

        let ohlc_cents = OhlcVecs::forced_import(db, "price_ohlc_cents", version)?;

        let price_usd = LazyPerBlock::from_computed::<CentsUnsignedToDollars>(
            "price",
            version,
            price_cents.height.read_only_boxed_clone(),
            &price_cents,
        );

        let open_usd = LazyEagerIndexes::from_eager_indexes::<CentsUnsignedToDollars>(
            "price_open",
            version,
            &open_cents,
        );
        let high_usd = LazyEagerIndexes::from_eager_indexes::<CentsUnsignedToDollars>(
            "price_high",
            version,
            &high_cents,
        );
        let low_usd = LazyEagerIndexes::from_eager_indexes::<CentsUnsignedToDollars>(
            "price_low",
            version,
            &low_cents,
        );

        let close_usd = Resolutions::forced_import(
            "price_close",
            price_usd.height.read_only_boxed_clone(),
            version,
            indexes,
        );

        let ohlc_usd = LazyOhlcVecs::from_eager_ohlc_indexes::<OhlcCentsToDollars>(
            "price_ohlc",
            version,
            &ohlc_cents,
        );

        let price_sats = LazyPerBlock::from_computed::<CentsUnsignedToSats>(
            "price_sats",
            version,
            price_cents.height.read_only_boxed_clone(),
            &price_cents,
        );

        let open_sats = LazyEagerIndexes::from_eager_indexes::<CentsUnsignedToSats>(
            "price_open_sats",
            version,
            &open_cents,
        );
        // Sats are inversely related to cents (sats = 10B/cents), so high↔low are swapped
        let high_sats = LazyEagerIndexes::from_eager_indexes::<CentsUnsignedToSats>(
            "price_high_sats",
            version,
            &low_cents,
        );
        let low_sats = LazyEagerIndexes::from_eager_indexes::<CentsUnsignedToSats>(
            "price_low_sats",
            version,
            &high_cents,
        );

        let close_sats = Resolutions::forced_import(
            "price_close_sats",
            price_sats.height.read_only_boxed_clone(),
            version,
            indexes,
        );

        // OhlcCentsToSats handles the high↔low swap internally
        let ohlc_sats = LazyOhlcVecs::from_eager_ohlc_indexes::<OhlcCentsToSats>(
            "price_ohlc_sats",
            version,
            &ohlc_cents,
        );

        let split = SplitByUnit {
            open: SplitIndexesByUnit {
                cents: open_cents,
                usd: open_usd,
                sats: open_sats,
            },
            high: SplitIndexesByUnit {
                cents: high_cents,
                usd: high_usd,
                sats: high_sats,
            },
            low: SplitIndexesByUnit {
                cents: low_cents,
                usd: low_usd,
                sats: low_sats,
            },
            close: SplitCloseByUnit {
                cents: close_cents,
                usd: close_usd,
                sats: close_sats,
            },
        };

        let ohlc = OhlcByUnit {
            cents: ohlc_cents,
            usd: ohlc_usd,
            sats: ohlc_sats,
        };

        let spot = PriceByUnit {
            cents: price_cents,
            usd: price_usd,
            sats: price_sats,
        };

        Ok(Self {
            db: db.clone(),
            split,
            ohlc,
            spot,
        })
    }

    pub(crate) fn db(&self) -> &Database {
        &self.db
    }
}
