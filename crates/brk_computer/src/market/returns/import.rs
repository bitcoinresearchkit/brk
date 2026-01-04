use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::super::lookback::{self, LOOKBACK_PERIOD_NAMES};
use super::Vecs;
use crate::{
    indexes,
    internal::{
        ComputedStandardDeviationVecsFromDateIndex, ComputedVecsFromDateIndex,
        LazyVecsFrom2FromDateIndex, PercentageDiffCloseDollars, Source,
        StandardDeviationVecsOptions, VecBuilderOptions,
    },
    market::dca::ByDcaCagr,
    price,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: &price::Vecs,
        lookback: &lookback::Vecs,
    ) -> Result<Self> {
        let v1 = Version::ONE;
        let last = VecBuilderOptions::default().add_last();

        // Price returns (lazy, from price.close and lookback.price_ago)
        let price_returns =
            LOOKBACK_PERIOD_NAMES
                .zip_ref(&lookback.price_ago)
                .map(|(name, price_ago)| {
                    LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                        &format!("{name}_price_returns"),
                        version,
                        &price.usd.timeindexes_to_price_close,
                        price_ago,
                    )
                });

        // CAGR (computed, 2y+ only)
        let cagr = ByDcaCagr::try_new(|name, _days| {
            ComputedVecsFromDateIndex::forced_import(
                db,
                &format!("{name}_cagr"),
                Source::Compute,
                version,
                indexes,
                last,
            )
        })?;

        // Returns standard deviation (computed from 1d returns)
        let indexes_to_1d_returns_1w_sd =
            ComputedStandardDeviationVecsFromDateIndex::forced_import(
                db,
                "1d_returns_1w_sd",
                7,
                Source::Compute,
                version + v1,
                indexes,
                StandardDeviationVecsOptions::default(),
                None,
            )?;
        let indexes_to_1d_returns_1m_sd =
            ComputedStandardDeviationVecsFromDateIndex::forced_import(
                db,
                "1d_returns_1m_sd",
                30,
                Source::Compute,
                version + v1,
                indexes,
                StandardDeviationVecsOptions::default(),
                None,
            )?;
        let indexes_to_1d_returns_1y_sd =
            ComputedStandardDeviationVecsFromDateIndex::forced_import(
                db,
                "1d_returns_1y_sd",
                365,
                Source::Compute,
                version + v1,
                indexes,
                StandardDeviationVecsOptions::default(),
                None,
            )?;

        // Downside returns and deviation (for Sortino ratio)
        let dateindex_to_downside_returns =
            EagerVec::forced_import(db, "downside_returns", version)?;
        let indexes_to_downside_1w_sd = ComputedStandardDeviationVecsFromDateIndex::forced_import(
            db,
            "downside_1w_sd",
            7,
            Source::Compute,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;
        let indexes_to_downside_1m_sd = ComputedStandardDeviationVecsFromDateIndex::forced_import(
            db,
            "downside_1m_sd",
            30,
            Source::Compute,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;
        let indexes_to_downside_1y_sd = ComputedStandardDeviationVecsFromDateIndex::forced_import(
            db,
            "downside_1y_sd",
            365,
            Source::Compute,
            version + v1,
            indexes,
            StandardDeviationVecsOptions::default(),
            None,
        )?;

        Ok(Self {
            price_returns,
            cagr,

            indexes_to_1d_returns_1w_sd,
            indexes_to_1d_returns_1m_sd,
            indexes_to_1d_returns_1y_sd,

            dateindex_to_downside_returns,
            indexes_to_downside_1w_sd,
            indexes_to_downside_1m_sd,
            indexes_to_downside_1y_sd,
        })
    }
}
