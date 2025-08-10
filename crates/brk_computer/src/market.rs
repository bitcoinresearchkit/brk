use std::{path::Path, thread};

use brk_error::Result;
use brk_indexer::Indexer;
use brk_structs::{Date, DateIndex, Dollars, Height, Sats, StoredF32, StoredU16, Version};
use vecdb::{
    AnyCollectableVec, Computation, Database, EagerVec, Exit, Format, PAGE_SIZE, StoredIndex,
    VecIterator,
};

use crate::{
    grouped::Source,
    price,
    traits::{ComputeDCAAveragePriceViaLen, ComputeDCAStackViaLen, ComputeDrawdown},
};

use super::{
    Indexes,
    grouped::{ComputedRatioVecsFromDateIndex, ComputedVecsFromDateIndex, VecBuilderOptions},
    indexes, transactions,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct Vecs {
    db: Database,

    pub height_to_marketcap: EagerVec<Height, Dollars>,
    pub height_to_ath: EagerVec<Height, Dollars>,
    pub height_to_drawdown: EagerVec<Height, StoredF32>,
    pub indexes_to_marketcap: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_ath: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_drawdown: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_days_since_ath: ComputedVecsFromDateIndex<StoredU16>,
    pub indexes_to_max_days_between_aths: ComputedVecsFromDateIndex<StoredU16>,
    pub indexes_to_max_years_between_aths: ComputedVecsFromDateIndex<StoredF32>,

    pub indexes_to_1w_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_8d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_13d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_21d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_1m_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_34d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_55d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_89d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_144d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_200d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_1y_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_2y_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_200w_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_4y_sma: ComputedRatioVecsFromDateIndex,

    pub indexes_to_200d_sma_x2_4: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_200d_sma_x0_8: ComputedVecsFromDateIndex<Dollars>,

    pub price_1d_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_1w_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_1m_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_3m_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_6m_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_1y_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_2y_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_3y_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_4y_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_5y_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_6y_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_8y_ago: ComputedVecsFromDateIndex<Dollars>,
    pub price_10y_ago: ComputedVecsFromDateIndex<Dollars>,

    pub _1d_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _1w_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _1m_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _3m_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _6m_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _1y_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _2y_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _3y_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _4y_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _5y_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _6y_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _8y_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _10y_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _2y_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _3y_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _4y_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _5y_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _6y_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _8y_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _10y_cagr: ComputedVecsFromDateIndex<StoredF32>,

    pub _1w_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _1m_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _3m_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _6m_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _1y_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _2y_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _3y_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _4y_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _5y_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _6y_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _8y_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _10y_dca_stack: ComputedVecsFromDateIndex<Sats>,
    pub _1w_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _1m_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _3m_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _6m_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _1y_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _2y_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _3y_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _4y_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _5y_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _6y_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _8y_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _10y_dca_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub _1w_dca_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _1m_dca_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _3m_dca_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _6m_dca_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _1y_dca_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _2y_dca_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _3y_dca_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _4y_dca_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _5y_dca_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _6y_dca_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _8y_dca_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _10y_dca_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _2y_dca_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _3y_dca_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _4y_dca_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _5y_dca_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _6y_dca_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _8y_dca_cagr: ComputedVecsFromDateIndex<StoredF32>,
    pub _10y_dca_cagr: ComputedVecsFromDateIndex<StoredF32>,

    pub dca_class_2025_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2024_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2023_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2022_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2021_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2020_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2019_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2018_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2017_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2016_stack: ComputedVecsFromDateIndex<Sats>,
    pub dca_class_2015_stack: ComputedVecsFromDateIndex<Sats>,

    pub dca_class_2025_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2024_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2023_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2022_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2021_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2020_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2019_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2018_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2017_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2016_avg_price: ComputedVecsFromDateIndex<Dollars>,
    pub dca_class_2015_avg_price: ComputedVecsFromDateIndex<Dollars>,

    pub dca_class_2025_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub dca_class_2024_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub dca_class_2023_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub dca_class_2022_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub dca_class_2021_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub dca_class_2020_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub dca_class_2019_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub dca_class_2018_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub dca_class_2017_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub dca_class_2016_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub dca_class_2015_returns: ComputedVecsFromDateIndex<StoredF32>,
}

impl Vecs {
    pub fn forced_import(
        parent: &Path,
        version: Version,
        computation: Computation,
        format: Format,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let db = Database::open(&parent.join("market"))?;
        db.set_min_len(PAGE_SIZE * 1_000_000)?;

        Ok(Self {
            height_to_marketcap: EagerVec::forced_import(
                &db,
                "marketcap",
                version + VERSION + Version::ZERO,
                format,
            )?,
            height_to_ath: EagerVec::forced_import(
                &db,
                "ath",
                version + VERSION + Version::ZERO,
                format,
            )?,
            height_to_drawdown: EagerVec::forced_import(
                &db,
                "drawdown",
                version + VERSION + Version::ZERO,
                format,
            )?,
            indexes_to_marketcap: ComputedVecsFromDateIndex::forced_import(
                &db,
                "marketcap",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_ath: ComputedVecsFromDateIndex::forced_import(
                &db,
                "ath",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_drawdown: ComputedVecsFromDateIndex::forced_import(
                &db,
                "drawdown",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_days_since_ath: ComputedVecsFromDateIndex::forced_import(
                &db,
                "days_since_ath",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_max_days_between_aths: ComputedVecsFromDateIndex::forced_import(
                &db,
                "max_days_between_aths",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_max_years_between_aths: ComputedVecsFromDateIndex::forced_import(
                &db,
                "max_years_between_aths",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,

            indexes_to_1w_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "1w_sma",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                true,
            )?,
            indexes_to_8d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "8d_sma",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                true,
            )?,
            indexes_to_13d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "13d_sma",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                true,
            )?,
            indexes_to_21d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "21d_sma",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                true,
            )?,
            indexes_to_1m_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "1m_sma",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                true,
            )?,
            indexes_to_34d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "34d_sma",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                true,
            )?,
            indexes_to_55d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "55d_sma",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                true,
            )?,
            indexes_to_89d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "89d_sma",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                true,
            )?,
            indexes_to_144d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "144d_sma",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                true,
            )?,
            indexes_to_200d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "200d_sma",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                true,
            )?,
            indexes_to_1y_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "1y_sma",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                true,
            )?,
            indexes_to_2y_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "2y_sma",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                true,
            )?,
            indexes_to_200w_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "200w_sma",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                true,
            )?,
            indexes_to_4y_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "4y_sma",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                true,
            )?,

            _1d_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1d_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1w_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1w_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1m_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1m_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3m_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3m_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6m_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6m_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1y_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1y_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _2y_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "2y_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3y_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3y_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _4y_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "4y_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _5y_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "5y_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6y_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6y_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _8y_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "8y_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _10y_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "10y_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _2y_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "2y_cagr",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3y_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3y_cagr",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _4y_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "4y_cagr",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _5y_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "5y_cagr",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6y_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6y_cagr",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _8y_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "8y_cagr",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _10y_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "10y_cagr",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,

            _1w_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1w_dca_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1m_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1m_dca_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3m_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3m_dca_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6m_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6m_dca_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1y_dca_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _2y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "2y_dca_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3y_dca_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _4y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "4y_dca_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _5y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "5y_dca_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6y_dca_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _8y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "8y_dca_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _10y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "10y_dca_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _2y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "2y_dca_cagr",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3y_dca_cagr",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _4y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "4y_dca_cagr",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _5y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "5y_dca_cagr",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6y_dca_cagr",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _8y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "8y_dca_cagr",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _10y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "10y_dca_cagr",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1w_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1w_dca_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1m_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1m_dca_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3m_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3m_dca_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6m_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6m_dca_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1y_dca_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _2y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "2y_dca_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3y_dca_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _4y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "4y_dca_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _5y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "5y_dca_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6y_dca_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _8y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "8y_dca_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _10y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "10y_dca_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_1d_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_1d_ago",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_1w_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_1w_ago",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_1m_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_1m_ago",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_3m_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_3m_ago",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_6m_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_6m_ago",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_1y_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_1y_ago",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_2y_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_2y_ago",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_3y_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_3y_ago",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_4y_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_4y_ago",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_5y_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_5y_ago",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_6y_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_6y_ago",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_8y_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_8y_ago",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_10y_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_10y_ago",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1w_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1w_dca_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1m_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1m_dca_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3m_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3m_dca_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6m_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6m_dca_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1y_dca_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _2y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "2y_dca_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3y_dca_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _4y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "4y_dca_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _5y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "5y_dca_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6y_dca_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _8y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "8y_dca_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _10y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "10y_dca_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,

            dca_class_2025_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2025_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2024_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2024_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2023_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2023_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2022_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2022_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2021_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2021_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2020_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2020_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2019_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2019_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2018_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2018_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2017_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2017_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2016_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2016_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2015_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2015_stack",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,

            dca_class_2025_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2025_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2024_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2024_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2023_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2023_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2022_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2022_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2021_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2021_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2020_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2020_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2019_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2019_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2018_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2018_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2017_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2017_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2016_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2016_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2015_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2015_avg_price",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,

            dca_class_2025_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2025_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2024_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2024_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2023_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2023_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2022_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2022_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2021_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2021_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2020_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2020_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2019_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2019_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2018_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2018_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2017_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2017_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2016_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2016_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2015_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2015_returns",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,

            indexes_to_200d_sma_x2_4: ComputedVecsFromDateIndex::forced_import(
                &db,
                "200d_sma_x2_4",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_200d_sma_x0_8: ComputedVecsFromDateIndex::forced_import(
                &db,
                "200d_sma_x0_8",
                Source::Compute,
                version + VERSION + Version::ZERO,
                format,
                computation,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,

            db,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: &price::Vecs,
        transactions: &mut transactions::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(
            indexer,
            indexes,
            price,
            transactions,
            starting_indexes,
            exit,
        )?;
        self.db.flush_then_punch()?;
        Ok(())
    }

    fn compute_(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: &price::Vecs,
        transactions: &mut transactions::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_marketcap.compute_multiply(
            starting_indexes.height,
            &price.chainindexes_to_close.height,
            transactions
                .indexes_to_subsidy
                .bitcoin
                .height_extra
                .unwrap_cumulative(),
            exit,
        )?;
        self.height_to_ath.compute_max(
            starting_indexes.height,
            &price.chainindexes_to_high.height,
            exit,
        )?;
        self.height_to_drawdown.compute_drawdown(
            starting_indexes.height,
            &price.chainindexes_to_close.height,
            &self.height_to_ath,
            exit,
        )?;

        self.indexes_to_marketcap.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_multiply(
                    starting_indexes.dateindex,
                    price.timeindexes_to_close.dateindex.as_ref().unwrap(),
                    transactions
                        .indexes_to_subsidy
                        .bitcoin
                        .dateindex
                        .unwrap_cumulative(),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_ath.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_max(
                    starting_indexes.dateindex,
                    price.timeindexes_to_high.dateindex.as_ref().unwrap(),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_drawdown.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_drawdown(
                    starting_indexes.dateindex,
                    price.timeindexes_to_close.dateindex.as_ref().unwrap(),
                    self.indexes_to_ath.dateindex.as_ref().unwrap(),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_days_since_ath.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                let mut high_iter = price
                    .timeindexes_to_high
                    .dateindex
                    .as_ref()
                    .unwrap()
                    .into_iter();
                let mut prev = None;
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_ath.dateindex.as_ref().unwrap(),
                    |(i, ath, slf)| {
                        if prev.is_none() {
                            let i = i.unwrap_to_usize();
                            prev.replace(if i > 0 {
                                slf.into_iter().unwrap_get_inner_(i - 1)
                            } else {
                                StoredU16::default()
                            });
                        }
                        let days = if *high_iter.unwrap_get_inner(i) == ath {
                            StoredU16::default()
                        } else {
                            prev.unwrap() + StoredU16::new(1)
                        };
                        prev.replace(days);
                        (i, days)
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_max_days_between_aths.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                let mut prev = None;
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_days_since_ath.dateindex.as_ref().unwrap(),
                    |(i, days, slf)| {
                        if prev.is_none() {
                            let i = i.unwrap_to_usize();
                            prev.replace(if i > 0 {
                                slf.into_iter().unwrap_get_inner_(i - 1)
                            } else {
                                StoredU16::ZERO
                            });
                        }
                        let max = prev.unwrap().max(days);
                        prev.replace(max);
                        (i, max)
                    },
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_max_years_between_aths.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_max_days_between_aths
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    |(i, max, ..)| (i, StoredF32::from(*max as f64 / 365.0)),
                    exit,
                )?;
                Ok(())
            },
        )?;

        [
            (1, &mut self.price_1d_ago, &mut self._1d_returns, None),
            (7, &mut self.price_1w_ago, &mut self._1w_returns, None),
            (30, &mut self.price_1m_ago, &mut self._1m_returns, None),
            (3 * 30, &mut self.price_3m_ago, &mut self._3m_returns, None),
            (6 * 30, &mut self.price_6m_ago, &mut self._6m_returns, None),
            (365, &mut self.price_1y_ago, &mut self._1y_returns, None),
            (
                2 * 365,
                &mut self.price_2y_ago,
                &mut self._2y_returns,
                Some(&mut self._2y_cagr),
            ),
            (
                3 * 365,
                &mut self.price_3y_ago,
                &mut self._3y_returns,
                Some(&mut self._3y_cagr),
            ),
            (
                4 * 365,
                &mut self.price_4y_ago,
                &mut self._4y_returns,
                Some(&mut self._4y_cagr),
            ),
            (
                5 * 365,
                &mut self.price_5y_ago,
                &mut self._5y_returns,
                Some(&mut self._5y_cagr),
            ),
            (
                6 * 365,
                &mut self.price_6y_ago,
                &mut self._6y_returns,
                Some(&mut self._6y_cagr),
            ),
            (
                8 * 365,
                &mut self.price_8y_ago,
                &mut self._8y_returns,
                Some(&mut self._8y_cagr),
            ),
            (
                10 * 365,
                &mut self.price_10y_ago,
                &mut self._10y_returns,
                Some(&mut self._10y_cagr),
            ),
        ]
        .into_iter()
        .try_for_each(|(days, ago, returns, cagr)| -> Result<()> {
            ago.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |v, _, _, starting_indexes, exit| {
                    v.compute_previous_value(
                        starting_indexes.dateindex,
                        price.timeindexes_to_close.dateindex.as_ref().unwrap(),
                        days,
                        exit,
                    )?;
                    Ok(())
                },
            )?;

            returns.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |v, _, _, starting_indexes, exit| {
                    v.compute_percentage_change(
                        starting_indexes.dateindex,
                        price.timeindexes_to_close.dateindex.as_ref().unwrap(),
                        days,
                        exit,
                    )?;
                    Ok(())
                },
            )?;

            if let Some(cagr) = cagr {
                cagr.compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |v, _, _, starting_indexes, exit| {
                        v.compute_cagr(
                            starting_indexes.dateindex,
                            returns.dateindex.as_ref().unwrap(),
                            days,
                            exit,
                        )?;
                        Ok(())
                    },
                )?;
            }

            Ok(())
        })?;

        [
            (
                7,
                &mut self._1w_dca_stack,
                &mut self._1w_dca_avg_price,
                &mut self._1w_dca_returns,
                None,
            ),
            (
                30,
                &mut self._1m_dca_stack,
                &mut self._1m_dca_avg_price,
                &mut self._1m_dca_returns,
                None,
            ),
            (
                3 * 30,
                &mut self._3m_dca_stack,
                &mut self._3m_dca_avg_price,
                &mut self._3m_dca_returns,
                None,
            ),
            (
                6 * 30,
                &mut self._6m_dca_stack,
                &mut self._6m_dca_avg_price,
                &mut self._6m_dca_returns,
                None,
            ),
            (
                365,
                &mut self._1y_dca_stack,
                &mut self._1y_dca_avg_price,
                &mut self._1y_dca_returns,
                None,
            ),
            (
                2 * 365,
                &mut self._2y_dca_stack,
                &mut self._2y_dca_avg_price,
                &mut self._2y_dca_returns,
                Some(&mut self._2y_dca_cagr),
            ),
            (
                3 * 365,
                &mut self._3y_dca_stack,
                &mut self._3y_dca_avg_price,
                &mut self._3y_dca_returns,
                Some(&mut self._3y_dca_cagr),
            ),
            (
                4 * 365,
                &mut self._4y_dca_stack,
                &mut self._4y_dca_avg_price,
                &mut self._4y_dca_returns,
                Some(&mut self._4y_dca_cagr),
            ),
            (
                5 * 365,
                &mut self._5y_dca_stack,
                &mut self._5y_dca_avg_price,
                &mut self._5y_dca_returns,
                Some(&mut self._5y_dca_cagr),
            ),
            (
                6 * 365,
                &mut self._6y_dca_stack,
                &mut self._6y_dca_avg_price,
                &mut self._6y_dca_returns,
                Some(&mut self._6y_dca_cagr),
            ),
            (
                8 * 365,
                &mut self._8y_dca_stack,
                &mut self._8y_dca_avg_price,
                &mut self._8y_dca_returns,
                Some(&mut self._8y_dca_cagr),
            ),
            (
                10 * 365,
                &mut self._10y_dca_stack,
                &mut self._10y_dca_avg_price,
                &mut self._10y_dca_returns,
                Some(&mut self._10y_dca_cagr),
            ),
        ]
        .into_iter()
        .try_for_each(
            |(days, dca_stack, dca_avg_price, dca_returns, dca_cagr)| -> Result<()> {
                dca_stack.compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |v, _, _, starting_indexes, exit| {
                        v.compute_dca_stack_via_len(
                            starting_indexes.dateindex,
                            price.timeindexes_to_close.dateindex.as_ref().unwrap(),
                            days,
                            exit,
                        )?;
                        Ok(())
                    },
                )?;

                dca_avg_price.compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |v, _, _, starting_indexes, exit| {
                        v.compute_dca_avg_price_via_len(
                            starting_indexes.dateindex,
                            dca_stack.dateindex.as_ref().unwrap(),
                            days,
                            exit,
                        )?;
                        Ok(())
                    },
                )?;

                dca_returns.compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |v, _, _, starting_indexes, exit| {
                        v.compute_percentage_difference(
                            starting_indexes.dateindex,
                            price.timeindexes_to_close.dateindex.as_ref().unwrap(),
                            dca_avg_price.dateindex.as_ref().unwrap(),
                            exit,
                        )?;
                        Ok(())
                    },
                )?;

                if let Some(dca_cagr) = dca_cagr {
                    dca_cagr.compute_all(
                        indexer,
                        indexes,
                        starting_indexes,
                        exit,
                        |v, _, _, starting_indexes, exit| {
                            v.compute_cagr(
                                starting_indexes.dateindex,
                                dca_returns.dateindex.as_ref().unwrap(),
                                days,
                                exit,
                            )?;
                            Ok(())
                        },
                    )?;
                }

                Ok(())
            },
        )?;

        [
            (
                2015,
                &mut self.dca_class_2015_avg_price,
                &mut self.dca_class_2015_returns,
                &mut self.dca_class_2015_stack,
            ),
            (
                2016,
                &mut self.dca_class_2016_avg_price,
                &mut self.dca_class_2016_returns,
                &mut self.dca_class_2016_stack,
            ),
            (
                2017,
                &mut self.dca_class_2017_avg_price,
                &mut self.dca_class_2017_returns,
                &mut self.dca_class_2017_stack,
            ),
            (
                2018,
                &mut self.dca_class_2018_avg_price,
                &mut self.dca_class_2018_returns,
                &mut self.dca_class_2018_stack,
            ),
            (
                2019,
                &mut self.dca_class_2019_avg_price,
                &mut self.dca_class_2019_returns,
                &mut self.dca_class_2019_stack,
            ),
            (
                2020,
                &mut self.dca_class_2020_avg_price,
                &mut self.dca_class_2020_returns,
                &mut self.dca_class_2020_stack,
            ),
            (
                2021,
                &mut self.dca_class_2021_avg_price,
                &mut self.dca_class_2021_returns,
                &mut self.dca_class_2021_stack,
            ),
            (
                2022,
                &mut self.dca_class_2022_avg_price,
                &mut self.dca_class_2022_returns,
                &mut self.dca_class_2022_stack,
            ),
            (
                2023,
                &mut self.dca_class_2023_avg_price,
                &mut self.dca_class_2023_returns,
                &mut self.dca_class_2023_stack,
            ),
            (
                2024,
                &mut self.dca_class_2024_avg_price,
                &mut self.dca_class_2024_returns,
                &mut self.dca_class_2024_stack,
            ),
            (
                2025,
                &mut self.dca_class_2025_avg_price,
                &mut self.dca_class_2025_returns,
                &mut self.dca_class_2025_stack,
            ),
        ]
        .into_iter()
        .try_for_each(|(year, avg_price, returns, stack)| -> Result<()> {
            let dateindex = DateIndex::try_from(Date::new(year, 1, 1)).unwrap();

            stack.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |v, _, _, starting_indexes, exit| {
                    v.compute_dca_stack_via_from(
                        starting_indexes.dateindex,
                        price.timeindexes_to_close.dateindex.as_ref().unwrap(),
                        dateindex,
                        exit,
                    )?;
                    Ok(())
                },
            )?;

            avg_price.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |v, _, _, starting_indexes, exit| {
                    v.compute_dca_avg_price_via_from(
                        starting_indexes.dateindex,
                        stack.dateindex.as_ref().unwrap(),
                        dateindex,
                        exit,
                    )?;
                    Ok(())
                },
            )?;

            returns.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |v, _, _, starting_indexes, exit| {
                    v.compute_percentage_difference(
                        starting_indexes.dateindex,
                        price.timeindexes_to_close.dateindex.as_ref().unwrap(),
                        avg_price.dateindex.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                },
            )?;

            Ok(())
        })?;

        thread::scope(|s| -> Result<()> {
            [
                (&mut self.indexes_to_1w_sma, 7),
                (&mut self.indexes_to_8d_sma, 8),
                (&mut self.indexes_to_13d_sma, 13),
                (&mut self.indexes_to_21d_sma, 21),
                (&mut self.indexes_to_1m_sma, 30),
                (&mut self.indexes_to_34d_sma, 34),
                (&mut self.indexes_to_55d_sma, 55),
                (&mut self.indexes_to_89d_sma, 89),
                (&mut self.indexes_to_144d_sma, 144),
                (&mut self.indexes_to_200d_sma, 200),
                (&mut self.indexes_to_1y_sma, 365),
                (&mut self.indexes_to_2y_sma, 2 * 365),
                (&mut self.indexes_to_200w_sma, 200 * 7),
                (&mut self.indexes_to_4y_sma, 4 * 365),
            ]
            .into_iter()
            .for_each(|(vecs, sma)| {
                s.spawn(move || -> Result<()> {
                    vecs.compute_all(
                        indexer,
                        indexes,
                        price,
                        starting_indexes,
                        exit,
                        |v, _, _, starting_indexes, exit| {
                            v.compute_sma(
                                starting_indexes.dateindex,
                                price.timeindexes_to_close.dateindex.as_ref().unwrap(),
                                sma,
                                exit,
                            )?;
                            Ok(())
                        },
                    )
                });
            });
            Ok(())
        })?;

        self.indexes_to_200d_sma_x0_8.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_200d_sma
                        .price
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    |(i, v, ..)| (i, v * 0.8),
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_200d_sma_x2_4.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_200d_sma
                        .price
                        .as_ref()
                        .unwrap()
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    |(i, v, ..)| (i, v * 2.4),
                    exit,
                )?;
                Ok(())
            },
        )?;

        Ok(())
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.indexes_to_marketcap.vecs(),
            self.indexes_to_ath.vecs(),
            self.indexes_to_drawdown.vecs(),
            self.indexes_to_days_since_ath.vecs(),
            self.indexes_to_max_days_between_aths.vecs(),
            self.indexes_to_max_years_between_aths.vecs(),
            self.indexes_to_1w_sma.vecs(),
            self.indexes_to_8d_sma.vecs(),
            self.indexes_to_13d_sma.vecs(),
            self.indexes_to_21d_sma.vecs(),
            self.indexes_to_1m_sma.vecs(),
            self.indexes_to_34d_sma.vecs(),
            self.indexes_to_55d_sma.vecs(),
            self.indexes_to_89d_sma.vecs(),
            self.indexes_to_144d_sma.vecs(),
            self.indexes_to_200d_sma.vecs(),
            self.indexes_to_1y_sma.vecs(),
            self.indexes_to_2y_sma.vecs(),
            self.indexes_to_200w_sma.vecs(),
            self.indexes_to_4y_sma.vecs(),
            self.indexes_to_200d_sma_x0_8.vecs(),
            self.indexes_to_200d_sma_x2_4.vecs(),
            self.price_1d_ago.vecs(),
            self.price_1w_ago.vecs(),
            self.price_1m_ago.vecs(),
            self.price_3m_ago.vecs(),
            self.price_6m_ago.vecs(),
            self.price_1y_ago.vecs(),
            self.price_2y_ago.vecs(),
            self.price_3y_ago.vecs(),
            self.price_4y_ago.vecs(),
            self.price_5y_ago.vecs(),
            self.price_6y_ago.vecs(),
            self.price_8y_ago.vecs(),
            self.price_10y_ago.vecs(),
            self._1d_returns.vecs(),
            self._1w_returns.vecs(),
            self._1m_returns.vecs(),
            self._3m_returns.vecs(),
            self._6m_returns.vecs(),
            self._1y_returns.vecs(),
            self._2y_returns.vecs(),
            self._3y_returns.vecs(),
            self._4y_returns.vecs(),
            self._5y_returns.vecs(),
            self._6y_returns.vecs(),
            self._8y_returns.vecs(),
            self._10y_returns.vecs(),
            self._2y_cagr.vecs(),
            self._3y_cagr.vecs(),
            self._4y_cagr.vecs(),
            self._5y_cagr.vecs(),
            self._6y_cagr.vecs(),
            self._8y_cagr.vecs(),
            self._10y_cagr.vecs(),
            self._1w_dca_returns.vecs(),
            self._1m_dca_returns.vecs(),
            self._3m_dca_returns.vecs(),
            self._6m_dca_returns.vecs(),
            self._1y_dca_returns.vecs(),
            self._2y_dca_returns.vecs(),
            self._3y_dca_returns.vecs(),
            self._4y_dca_returns.vecs(),
            self._5y_dca_returns.vecs(),
            self._6y_dca_returns.vecs(),
            self._8y_dca_returns.vecs(),
            self._10y_dca_returns.vecs(),
            self._2y_dca_cagr.vecs(),
            self._3y_dca_cagr.vecs(),
            self._4y_dca_cagr.vecs(),
            self._5y_dca_cagr.vecs(),
            self._6y_dca_cagr.vecs(),
            self._8y_dca_cagr.vecs(),
            self._10y_dca_cagr.vecs(),
            self._1w_dca_avg_price.vecs(),
            self._1m_dca_avg_price.vecs(),
            self._3m_dca_avg_price.vecs(),
            self._6m_dca_avg_price.vecs(),
            self._1y_dca_avg_price.vecs(),
            self._2y_dca_avg_price.vecs(),
            self._3y_dca_avg_price.vecs(),
            self._4y_dca_avg_price.vecs(),
            self._5y_dca_avg_price.vecs(),
            self._6y_dca_avg_price.vecs(),
            self._8y_dca_avg_price.vecs(),
            self._10y_dca_avg_price.vecs(),
            self._1w_dca_stack.vecs(),
            self._1m_dca_stack.vecs(),
            self._3m_dca_stack.vecs(),
            self._6m_dca_stack.vecs(),
            self._1y_dca_stack.vecs(),
            self._2y_dca_stack.vecs(),
            self._3y_dca_stack.vecs(),
            self._4y_dca_stack.vecs(),
            self._5y_dca_stack.vecs(),
            self._6y_dca_stack.vecs(),
            self._8y_dca_stack.vecs(),
            self._10y_dca_stack.vecs(),
            self.dca_class_2025_stack.vecs(),
            self.dca_class_2024_stack.vecs(),
            self.dca_class_2023_stack.vecs(),
            self.dca_class_2022_stack.vecs(),
            self.dca_class_2021_stack.vecs(),
            self.dca_class_2020_stack.vecs(),
            self.dca_class_2019_stack.vecs(),
            self.dca_class_2018_stack.vecs(),
            self.dca_class_2017_stack.vecs(),
            self.dca_class_2016_stack.vecs(),
            self.dca_class_2015_stack.vecs(),
            self.dca_class_2025_avg_price.vecs(),
            self.dca_class_2024_avg_price.vecs(),
            self.dca_class_2023_avg_price.vecs(),
            self.dca_class_2022_avg_price.vecs(),
            self.dca_class_2021_avg_price.vecs(),
            self.dca_class_2020_avg_price.vecs(),
            self.dca_class_2019_avg_price.vecs(),
            self.dca_class_2018_avg_price.vecs(),
            self.dca_class_2017_avg_price.vecs(),
            self.dca_class_2016_avg_price.vecs(),
            self.dca_class_2015_avg_price.vecs(),
            self.dca_class_2025_returns.vecs(),
            self.dca_class_2024_returns.vecs(),
            self.dca_class_2023_returns.vecs(),
            self.dca_class_2022_returns.vecs(),
            self.dca_class_2021_returns.vecs(),
            self.dca_class_2020_returns.vecs(),
            self.dca_class_2019_returns.vecs(),
            self.dca_class_2018_returns.vecs(),
            self.dca_class_2017_returns.vecs(),
            self.dca_class_2016_returns.vecs(),
            self.dca_class_2015_returns.vecs(),
            vec![
                &self.height_to_marketcap,
                &self.height_to_ath,
                &self.height_to_drawdown,
            ],
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
