use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    grouped::{
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

        let price_1d_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_1d_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_1w_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_1w_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_1m_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_1m_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_3m_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_3m_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_6m_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_6m_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_1y_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_1y_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_2y_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_2y_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_3y_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_3y_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_4y_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_4y_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_5y_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_5y_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_6y_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_6y_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_8y_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_8y_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;
        let price_10y_ago = ComputedVecsFromDateIndex::forced_import(
            db,
            "price_10y_ago",
            Source::Compute,
            version + v0,
            indexes,
            last,
        )?;

        let _1d_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "1d_price_returns",
                version + v0,
                &price.timeindexes_to_price_close,
                &price_1d_ago,
            );
        let _1w_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "1w_price_returns",
                version + v0,
                &price.timeindexes_to_price_close,
                &price_1w_ago,
            );
        let _1m_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "1m_price_returns",
                version + v0,
                &price.timeindexes_to_price_close,
                &price_1m_ago,
            );
        let _3m_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "3m_price_returns",
                version + v0,
                &price.timeindexes_to_price_close,
                &price_3m_ago,
            );
        let _6m_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "6m_price_returns",
                version + v0,
                &price.timeindexes_to_price_close,
                &price_6m_ago,
            );
        let _1y_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "1y_price_returns",
                version + v0,
                &price.timeindexes_to_price_close,
                &price_1y_ago,
            );
        let _2y_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "2y_price_returns",
                version + v0,
                &price.timeindexes_to_price_close,
                &price_2y_ago,
            );
        let _3y_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "3y_price_returns",
                version + v0,
                &price.timeindexes_to_price_close,
                &price_3y_ago,
            );
        let _4y_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "4y_price_returns",
                version + v0,
                &price.timeindexes_to_price_close,
                &price_4y_ago,
            );
        let _5y_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "5y_price_returns",
                version + v0,
                &price.timeindexes_to_price_close,
                &price_5y_ago,
            );
        let _6y_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "6y_price_returns",
                version + v0,
                &price.timeindexes_to_price_close,
                &price_6y_ago,
            );
        let _8y_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "8y_price_returns",
                version + v0,
                &price.timeindexes_to_price_close,
                &price_8y_ago,
            );
        let _10y_price_returns =
            LazyVecsFrom2FromDateIndex::from_computed::<PercentageDiffCloseDollars>(
                "10y_price_returns",
                version + v0,
                &price.timeindexes_to_price_close,
                &price_10y_ago,
            );

        Ok(Self {
            price_1d_ago,
            price_1w_ago,
            price_1m_ago,
            price_3m_ago,
            price_6m_ago,
            price_1y_ago,
            price_2y_ago,
            price_3y_ago,
            price_4y_ago,
            price_5y_ago,
            price_6y_ago,
            price_8y_ago,
            price_10y_ago,

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

            _2y_cagr: ComputedVecsFromDateIndex::forced_import(
                db,
                "2y_cagr",
                Source::Compute,
                version + v0,
                indexes,
                last,
            )?,
            _3y_cagr: ComputedVecsFromDateIndex::forced_import(
                db,
                "3y_cagr",
                Source::Compute,
                version + v0,
                indexes,
                last,
            )?,
            _4y_cagr: ComputedVecsFromDateIndex::forced_import(
                db,
                "4y_cagr",
                Source::Compute,
                version + v0,
                indexes,
                last,
            )?,
            _5y_cagr: ComputedVecsFromDateIndex::forced_import(
                db,
                "5y_cagr",
                Source::Compute,
                version + v0,
                indexes,
                last,
            )?,
            _6y_cagr: ComputedVecsFromDateIndex::forced_import(
                db,
                "6y_cagr",
                Source::Compute,
                version + v0,
                indexes,
                last,
            )?,
            _8y_cagr: ComputedVecsFromDateIndex::forced_import(
                db,
                "8y_cagr",
                Source::Compute,
                version + v0,
                indexes,
                last,
            )?,
            _10y_cagr: ComputedVecsFromDateIndex::forced_import(
                db,
                "10y_cagr",
                Source::Compute,
                version + v0,
                indexes,
                last,
            )?,
        })
    }
}
