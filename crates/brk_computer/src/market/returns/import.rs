use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::super::lookback;
use super::Vecs;
use crate::{
    internal::{
        ComputedStandardDeviationVecsFromDateIndex, ComputedVecsFromDateIndex,
        LazyVecsFrom2FromDateIndex, PercentageDiffCloseDollars, Source,
        StandardDeviationVecsOptions, VecBuilderOptions,
    },
    indexes, price,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: &price::Vecs,
        lookback: &lookback::Vecs,
    ) -> Result<Self> {
        let v0 = Version::ZERO;
        let v1 = Version::ONE;
        let last = VecBuilderOptions::default().add_last();

        // Price returns (lazy, from price.close and lookback.price_*_ago)
        let _1d_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "1d_price_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &lookback.price_1d_ago,
            );
        let _1w_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "1w_price_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &lookback.price_1w_ago,
            );
        let _1m_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "1m_price_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &lookback.price_1m_ago,
            );
        let _3m_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "3m_price_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &lookback.price_3m_ago,
            );
        let _6m_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "6m_price_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &lookback.price_6m_ago,
            );
        let _1y_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "1y_price_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &lookback.price_1y_ago,
            );
        let _2y_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "2y_price_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &lookback.price_2y_ago,
            );
        let _3y_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "3y_price_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &lookback.price_3y_ago,
            );
        let _4y_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "4y_price_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &lookback.price_4y_ago,
            );
        let _5y_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "5y_price_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &lookback.price_5y_ago,
            );
        let _6y_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "6y_price_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &lookback.price_6y_ago,
            );
        let _8y_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "8y_price_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &lookback.price_8y_ago,
            );
        let _10y_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "10y_price_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &lookback.price_10y_ago,
            );

        // CAGR (computed)
        let _2y_cagr = ComputedVecsFromDateIndex::forced_import(
            db,
            "2y_cagr",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _3y_cagr = ComputedVecsFromDateIndex::forced_import(
            db,
            "3y_cagr",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _4y_cagr = ComputedVecsFromDateIndex::forced_import(
            db,
            "4y_cagr",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _5y_cagr = ComputedVecsFromDateIndex::forced_import(
            db,
            "5y_cagr",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _6y_cagr = ComputedVecsFromDateIndex::forced_import(
            db,
            "6y_cagr",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _8y_cagr = ComputedVecsFromDateIndex::forced_import(
            db,
            "8y_cagr",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _10y_cagr = ComputedVecsFromDateIndex::forced_import(
            db,
            "10y_cagr",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;

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
            EagerVec::forced_import(db, "downside_returns", version + v0)?;
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
            _1d_price_returns,
            _1w_price_returns,
            _1m_price_returns,
            _3m_price_returns,
            _6m_price_returns,
            _1y_price_returns,
            _2y_price_returns,
            _3y_price_returns,
            _4y_price_returns,
            _5y_price_returns,
            _6y_price_returns,
            _8y_price_returns,
            _10y_price_returns,

            _2y_cagr,
            _3y_cagr,
            _4y_cagr,
            _5y_cagr,
            _6y_cagr,
            _8y_cagr,
            _10y_cagr,

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
