#![allow(clippy::type_complexity)]

mod by_unit;
mod compute;
mod dependencies;
mod has;
mod lazy_ohlc;
mod ohlcs;

use brk_error::Result;

use bitview_compute::{
    CentsUnsignedToDollars, CentsUnsignedToSats, LazyIndexes, LazyPerBlock, OhlcCentsToDollars,
    OhlcCentsToHighCents, OhlcCentsToLowCents, OhlcCentsToOpenCents, OhlcCentsToSats, PerBlock,
    Resolutions,
};
use bitview_plugin::{ImportContext, Plugin, PluginId, PluginStorage};
use bitview_traversable::Traversable;
use brk_oracle::VERSION as ORACLE_VERSION;
use brk_types::Version;
use vecdb::{Database, Rw, StorageMode};

use by_unit::{OhlcByUnit, PriceByUnit, SplitByUnit, SplitCloseByUnit, SplitIndexesByUnit};
pub use dependencies::Dependencies;
pub use has::HasPrice;
use ohlcs::{LazyIndexesFromOhlc, LazyOhlcCentsVecs, LazyOhlcVecs};

const STORAGE: PluginStorage =
    PluginStorage::new(PluginId::new("price"), Version::new(20 + ORACLE_VERSION));
pub const ID: PluginId = STORAGE.id();

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    db: Database,

    /// Separate open, high, low, and close views of BRK's block-level BTC/USD
    /// price series for each supported time period. Heights before 340,000 use
    /// baked historical exchange prices; later heights use an on-chain oracle
    /// that estimates price from round-USD transaction-output patterns.
    pub split: SplitByUnit,
    /// Open-high-low-close (OHLC) candles formed from block-level Bitcoin spot
    /// prices within each supported time period. Heights before 340,000 use
    /// baked historical exchange prices; later heights use an on-chain oracle
    /// that estimates price from round-USD transaction-output patterns. Empty
    /// periods carry the previous close as all four candle values.
    pub ohlc: OhlcByUnit,
    /// BRK's block-level Bitcoin (BTC/USD) spot-price estimate. Heights before
    /// 340,000 use baked historical exchange prices; later heights use an on-chain oracle
    /// that estimates price from round-USD transaction-output patterns. This is
    /// a model-derived block price, not a contemporaneous exchange ticker.
    pub spot: PriceByUnit<M>,
}

impl<M: StorageMode> Plugin for Vecs<M>
where
    Self: Traversable + Send + Sync,
{
    fn storage(&self) -> PluginStorage {
        STORAGE
    }
}

impl Vecs {
    pub fn import(
        context: ImportContext<'_>,
        mappings: &bitview_plugin_mappings::Vecs,
    ) -> Result<Self> {
        let db = STORAGE.open_database(context, 100_000)?;
        let this = Self::forced_import_inner(&db, STORAGE.schema_version(), mappings)?;
        STORAGE.finalize_database(&this.db)?;
        Ok(this)
    }

    fn forced_import_inner(
        db: &Database,
        version: Version,
        mappings: &bitview_plugin_mappings::Vecs,
    ) -> Result<Self> {
        let price_cents = PerBlock::forced_import(db, "price_cents", version, mappings)?;
        let close_cents =
            Resolutions::from_source("price_close_cents", &price_cents.height, version, mappings);

        let ohlc_cents = LazyOhlcCentsVecs::new(
            "price_ohlc_cents",
            version,
            mappings,
            price_cents.height.read_only_cached_boxed_clone(),
        );

        let open_cents = LazyIndexes::from_ohlc_indexes::<OhlcCentsToOpenCents>(
            "price_open_cents",
            version,
            &ohlc_cents,
        );
        let high_cents = LazyIndexes::from_ohlc_indexes::<OhlcCentsToHighCents>(
            "price_high_cents",
            version,
            &ohlc_cents,
        );
        let low_cents = LazyIndexes::from_ohlc_indexes::<OhlcCentsToLowCents>(
            "price_low_cents",
            version,
            &ohlc_cents,
        );

        let price_usd = LazyPerBlock::from_resolutions::<CentsUnsignedToDollars>(
            "price",
            version,
            &price_cents,
        );

        let open_usd = LazyIndexes::from_lazy_indexes::<CentsUnsignedToDollars, _>(
            "price_open",
            version,
            &open_cents,
        );
        let high_usd = LazyIndexes::from_lazy_indexes::<CentsUnsignedToDollars, _>(
            "price_high",
            version,
            &high_cents,
        );
        let low_usd = LazyIndexes::from_lazy_indexes::<CentsUnsignedToDollars, _>(
            "price_low",
            version,
            &low_cents,
        );

        let close_usd =
            Resolutions::from_source("price_close", &price_usd.height, version, mappings);

        let ohlc_usd = LazyOhlcVecs::from_ohlc_indexes::<OhlcCentsToDollars>(
            "price_ohlc",
            version,
            &ohlc_cents,
        );

        let price_sats = LazyPerBlock::from_resolutions::<CentsUnsignedToSats>(
            "price_sats",
            version,
            &price_cents,
        );

        let open_sats = LazyIndexes::from_lazy_indexes::<CentsUnsignedToSats, _>(
            "price_open_sats",
            version,
            &open_cents,
        );
        // Sats are inversely related to cents (sats = 10B/cents), so high↔low are swapped
        let high_sats = LazyIndexes::from_lazy_indexes::<CentsUnsignedToSats, _>(
            "price_high_sats",
            version,
            &low_cents,
        );
        let low_sats = LazyIndexes::from_lazy_indexes::<CentsUnsignedToSats, _>(
            "price_low_sats",
            version,
            &high_cents,
        );

        let close_sats =
            Resolutions::from_source("price_close_sats", &price_sats.height, version, mappings);

        // OhlcCentsToSats handles the high↔low swap internally
        let ohlc_sats = LazyOhlcVecs::from_ohlc_indexes::<OhlcCentsToSats>(
            "price_ohlc_sats",
            version,
            &ohlc_cents,
        );

        let split = SplitByUnit {
            open: SplitIndexesByUnit {
                usd: open_usd,
                cents: open_cents,
                sats: open_sats,
            },
            high: SplitIndexesByUnit {
                usd: high_usd,
                cents: high_cents,
                sats: high_sats,
            },
            low: SplitIndexesByUnit {
                usd: low_usd,
                cents: low_cents,
                sats: low_sats,
            },
            close: SplitCloseByUnit {
                usd: close_usd,
                cents: close_cents,
                sats: close_sats,
            },
        };

        let ohlc = OhlcByUnit {
            usd: ohlc_usd,
            cents: ohlc_cents,
            sats: ohlc_sats,
        };

        let spot = PriceByUnit {
            usd: price_usd,
            cents: price_cents,
            sats: price_sats,
        };

        Ok(Self {
            db: db.clone(),
            split,
            ohlc,
            spot,
        })
    }
}
