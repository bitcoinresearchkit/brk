pub(crate) mod by_unit;
mod compute;
pub(crate) mod ohlcs;

use std::path::Path;

use brk_traversable::Traversable;
use brk_types::Version;
use vecdb::{
    Database, ImportableVec, LazyVecFrom1, PcoVec, ReadableCloneableVec, Rw, StorageMode,
    PAGE_SIZE,
};

use crate::{
    indexes,
    internal::{
        CentsUnsignedToDollars, CentsUnsignedToSats, ComputedHeightDerivedLast, EagerIndexes,
        LazyEagerIndexes, OhlcCentsToDollars, OhlcCentsToSats,
    },
};

use by_unit::{
    OhlcByUnit, PriceByUnit, SplitByUnit, SplitCloseByUnit, SplitIndexesByUnit,
};
use ohlcs::{LazyOhlcVecs, OhlcVecs};

pub const DB_NAME: &str = "prices";

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub(crate) db: Database,

    pub split: SplitByUnit<M>,
    pub ohlc: OhlcByUnit<M>,
    pub price: PriceByUnit<M>,
}

impl Vecs {
    pub(crate) fn forced_import(
        parent: &Path,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> brk_error::Result<Self> {
        let db = Database::open(&parent.join(DB_NAME))?;
        db.set_min_len(PAGE_SIZE * 1_000_000)?;

        let this = Self::forced_import_inner(&db, version, indexes)?;

        this.db.retain_regions(
            this.iter_any_exportable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;
        this.db.compact()?;

        Ok(this)
    }

    fn forced_import_inner(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> brk_error::Result<Self> {
        let version = version + Version::new(11);

        // ── Cents (eager, stored) ───────────────────────────────────

        let price_cents = PcoVec::forced_import(db, "price_cents", version)?;

        let open_cents = EagerIndexes::forced_import(db, "price_open_cents", version)?;
        let high_cents = EagerIndexes::forced_import(db, "price_high_cents", version)?;
        let low_cents = EagerIndexes::forced_import(db, "price_low_cents", version)?;

        let close_cents = ComputedHeightDerivedLast::forced_import(
            "price_close_cents",
            price_cents.read_only_boxed_clone(),
            version,
            indexes,
        );

        let ohlc_cents = OhlcVecs::forced_import(db, "price_ohlc_cents", version)?;

        // ── USD (lazy from cents) ───────────────────────────────────

        let price_usd = LazyVecFrom1::transformed::<CentsUnsignedToDollars>(
            "price",
            version,
            price_cents.read_only_boxed_clone(),
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

        let close_usd = ComputedHeightDerivedLast::forced_import(
            "price_close",
            price_usd.read_only_boxed_clone(),
            version,
            indexes,
        );

        let ohlc_usd = LazyOhlcVecs::from_eager_ohlc_indexes::<OhlcCentsToDollars>(
            "price_ohlc",
            version,
            &ohlc_cents,
        );

        // ── Sats (lazy from cents, high↔low swapped) ───────────────

        let price_sats = LazyVecFrom1::transformed::<CentsUnsignedToSats>(
            "price_sats",
            version,
            price_cents.read_only_boxed_clone(),
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

        let close_sats = ComputedHeightDerivedLast::forced_import(
            "price_close_sats",
            price_sats.read_only_boxed_clone(),
            version,
            indexes,
        );

        // OhlcCentsToSats handles the high↔low swap internally
        let ohlc_sats = LazyOhlcVecs::from_eager_ohlc_indexes::<OhlcCentsToSats>(
            "price_ohlc_sats",
            version,
            &ohlc_cents,
        );

        // ── Assemble pivoted structure ──────────────────────────────

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

        let price = PriceByUnit {
            cents: price_cents,
            usd: price_usd,
            sats: price_sats,
        };

        Ok(Self {
            db: db.clone(),
            split,
            ohlc,
            price,
        })
    }

    pub(crate) fn db(&self) -> &Database {
        &self.db
    }
}
