use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    internal::{
        ComputedVecsFromDateIndex, LazyVecsFrom2FromDateIndex, PercentageDiffCloseDollars, Source,
        VecBuilderOptions,
    },
    indexes, price,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: &price::Vecs,
    ) -> Result<Self> {
        let v0 = Version::ZERO;
        let last = VecBuilderOptions::default().add_last();

        // DCA by period - stack
        let _1w_dca_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "1w_dca_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _1m_dca_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "1m_dca_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _3m_dca_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "3m_dca_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _6m_dca_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "6m_dca_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _1y_dca_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "1y_dca_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _2y_dca_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "2y_dca_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _3y_dca_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "3y_dca_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _4y_dca_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "4y_dca_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _5y_dca_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "5y_dca_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _6y_dca_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "6y_dca_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _8y_dca_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "8y_dca_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _10y_dca_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "10y_dca_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;

        // DCA by period - avg price
        let _1w_dca_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "1w_dca_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _1m_dca_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "1m_dca_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _3m_dca_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "3m_dca_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _6m_dca_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "6m_dca_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _1y_dca_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "1y_dca_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _2y_dca_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "2y_dca_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _3y_dca_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "3y_dca_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _4y_dca_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "4y_dca_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _5y_dca_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "5y_dca_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _6y_dca_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "6y_dca_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _8y_dca_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "8y_dca_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _10y_dca_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "10y_dca_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;

        // DCA by period - returns (lazy)
        let _1w_dca_returns = LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
            "1w_dca_returns",
            version + v0,
            &price.usd.timeindexes_to_price_close,
            &_1w_dca_avg_price,
        );
        let _1m_dca_returns = LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
            "1m_dca_returns",
            version + v0,
            &price.usd.timeindexes_to_price_close,
            &_1m_dca_avg_price,
        );
        let _3m_dca_returns = LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
            "3m_dca_returns",
            version + v0,
            &price.usd.timeindexes_to_price_close,
            &_3m_dca_avg_price,
        );
        let _6m_dca_returns = LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
            "6m_dca_returns",
            version + v0,
            &price.usd.timeindexes_to_price_close,
            &_6m_dca_avg_price,
        );
        let _1y_dca_returns = LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
            "1y_dca_returns",
            version + v0,
            &price.usd.timeindexes_to_price_close,
            &_1y_dca_avg_price,
        );
        let _2y_dca_returns = LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
            "2y_dca_returns",
            version + v0,
            &price.usd.timeindexes_to_price_close,
            &_2y_dca_avg_price,
        );
        let _3y_dca_returns = LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
            "3y_dca_returns",
            version + v0,
            &price.usd.timeindexes_to_price_close,
            &_3y_dca_avg_price,
        );
        let _4y_dca_returns = LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
            "4y_dca_returns",
            version + v0,
            &price.usd.timeindexes_to_price_close,
            &_4y_dca_avg_price,
        );
        let _5y_dca_returns = LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
            "5y_dca_returns",
            version + v0,
            &price.usd.timeindexes_to_price_close,
            &_5y_dca_avg_price,
        );
        let _6y_dca_returns = LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
            "6y_dca_returns",
            version + v0,
            &price.usd.timeindexes_to_price_close,
            &_6y_dca_avg_price,
        );
        let _8y_dca_returns = LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
            "8y_dca_returns",
            version + v0,
            &price.usd.timeindexes_to_price_close,
            &_8y_dca_avg_price,
        );
        let _10y_dca_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "10y_dca_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &_10y_dca_avg_price,
            );

        // DCA by period - CAGR
        let _2y_dca_cagr = ComputedVecsFromDateIndex::forced_import(
            db,
            "2y_dca_cagr",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _3y_dca_cagr = ComputedVecsFromDateIndex::forced_import(
            db,
            "3y_dca_cagr",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _4y_dca_cagr = ComputedVecsFromDateIndex::forced_import(
            db,
            "4y_dca_cagr",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _5y_dca_cagr = ComputedVecsFromDateIndex::forced_import(
            db,
            "5y_dca_cagr",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _6y_dca_cagr = ComputedVecsFromDateIndex::forced_import(
            db,
            "6y_dca_cagr",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _8y_dca_cagr = ComputedVecsFromDateIndex::forced_import(
            db,
            "8y_dca_cagr",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let _10y_dca_cagr = ComputedVecsFromDateIndex::forced_import(
            db,
            "10y_dca_cagr",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;

        // DCA by year class - stack
        let dca_class_2025_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2025_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2024_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2024_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2023_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2023_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2022_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2022_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2021_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2021_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2020_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2020_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2019_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2019_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2018_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2018_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2017_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2017_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2016_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2016_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2015_stack = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2015_stack",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;

        // DCA by year class - avg price
        let dca_class_2025_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2025_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2024_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2024_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2023_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2023_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2022_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2022_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2021_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2021_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2020_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2020_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2019_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2019_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2018_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2018_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2017_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2017_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2016_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2016_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let dca_class_2015_avg_price = ComputedVecsFromDateIndex::forced_import(
            db,
            "dca_class_2015_avg_price",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;

        // DCA by year class - returns (lazy)
        let dca_class_2025_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "dca_class_2025_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &dca_class_2025_avg_price,
            );
        let dca_class_2024_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "dca_class_2024_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &dca_class_2024_avg_price,
            );
        let dca_class_2023_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "dca_class_2023_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &dca_class_2023_avg_price,
            );
        let dca_class_2022_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "dca_class_2022_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &dca_class_2022_avg_price,
            );
        let dca_class_2021_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "dca_class_2021_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &dca_class_2021_avg_price,
            );
        let dca_class_2020_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "dca_class_2020_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &dca_class_2020_avg_price,
            );
        let dca_class_2019_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "dca_class_2019_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &dca_class_2019_avg_price,
            );
        let dca_class_2018_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "dca_class_2018_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &dca_class_2018_avg_price,
            );
        let dca_class_2017_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "dca_class_2017_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &dca_class_2017_avg_price,
            );
        let dca_class_2016_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "dca_class_2016_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &dca_class_2016_avg_price,
            );
        let dca_class_2015_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "dca_class_2015_returns",
                version + v0,
                &price.usd.timeindexes_to_price_close,
                &dca_class_2015_avg_price,
            );

        Ok(Self {
            _1w_dca_stack,
            _1m_dca_stack,
            _3m_dca_stack,
            _6m_dca_stack,
            _1y_dca_stack,
            _2y_dca_stack,
            _3y_dca_stack,
            _4y_dca_stack,
            _5y_dca_stack,
            _6y_dca_stack,
            _8y_dca_stack,
            _10y_dca_stack,

            _1w_dca_avg_price,
            _1m_dca_avg_price,
            _3m_dca_avg_price,
            _6m_dca_avg_price,
            _1y_dca_avg_price,
            _2y_dca_avg_price,
            _3y_dca_avg_price,
            _4y_dca_avg_price,
            _5y_dca_avg_price,
            _6y_dca_avg_price,
            _8y_dca_avg_price,
            _10y_dca_avg_price,

            _1w_dca_returns,
            _1m_dca_returns,
            _3m_dca_returns,
            _6m_dca_returns,
            _1y_dca_returns,
            _2y_dca_returns,
            _3y_dca_returns,
            _4y_dca_returns,
            _5y_dca_returns,
            _6y_dca_returns,
            _8y_dca_returns,
            _10y_dca_returns,

            _2y_dca_cagr,
            _3y_dca_cagr,
            _4y_dca_cagr,
            _5y_dca_cagr,
            _6y_dca_cagr,
            _8y_dca_cagr,
            _10y_dca_cagr,

            dca_class_2025_stack,
            dca_class_2024_stack,
            dca_class_2023_stack,
            dca_class_2022_stack,
            dca_class_2021_stack,
            dca_class_2020_stack,
            dca_class_2019_stack,
            dca_class_2018_stack,
            dca_class_2017_stack,
            dca_class_2016_stack,
            dca_class_2015_stack,

            dca_class_2025_avg_price,
            dca_class_2024_avg_price,
            dca_class_2023_avg_price,
            dca_class_2022_avg_price,
            dca_class_2021_avg_price,
            dca_class_2020_avg_price,
            dca_class_2019_avg_price,
            dca_class_2018_avg_price,
            dca_class_2017_avg_price,
            dca_class_2016_avg_price,
            dca_class_2015_avg_price,

            dca_class_2025_returns,
            dca_class_2024_returns,
            dca_class_2023_returns,
            dca_class_2022_returns,
            dca_class_2021_returns,
            dca_class_2020_returns,
            dca_class_2019_returns,
            dca_class_2018_returns,
            dca_class_2017_returns,
            dca_class_2016_returns,
            dca_class_2015_returns,
        })
    }
}
