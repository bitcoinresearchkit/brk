use std::{path::Path, thread};

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Date, DateIndex, Dollars, Height, Sats, StoredF32, StoredU16, Version};
use vecdb::{
    Database, EagerVec, Exit, GenericStoredVec, ImportableVec, PAGE_SIZE, PcoVec, TypedVecIterator,
    VecIndex,
};

use crate::{
    grouped::{ComputedStandardDeviationVecsFromDateIndex, Source, StandardDeviationVecsOptions},
    price,
    traits::{ComputeDCAAveragePriceViaLen, ComputeDCAStackViaLen, ComputeDrawdown},
};

use super::{
    Indexes,
    grouped::{ComputedRatioVecsFromDateIndex, ComputedVecsFromDateIndex, VecBuilderOptions},
    indexes,
};

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,

    pub height_to_price_ath: EagerVec<PcoVec<Height, Dollars>>,
    pub height_to_price_drawdown: EagerVec<PcoVec<Height, StoredF32>>,
    pub indexes_to_price_ath: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_price_drawdown: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_days_since_price_ath: ComputedVecsFromDateIndex<StoredU16>,
    pub indexes_to_max_days_between_price_aths: ComputedVecsFromDateIndex<StoredU16>,
    pub indexes_to_max_years_between_price_aths: ComputedVecsFromDateIndex<StoredF32>,

    pub indexes_to_1d_returns_1w_sd: ComputedStandardDeviationVecsFromDateIndex,
    pub indexes_to_1d_returns_1m_sd: ComputedStandardDeviationVecsFromDateIndex,
    pub indexes_to_1d_returns_1y_sd: ComputedStandardDeviationVecsFromDateIndex,
    pub indexes_to_price_1w_volatility: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_price_1m_volatility: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_price_1y_volatility: ComputedVecsFromDateIndex<StoredF32>,

    pub indexes_to_price_1w_min: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_price_1w_max: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_price_2w_min: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_price_2w_max: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_price_1m_min: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_price_1m_max: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_price_1y_min: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_price_1y_max: ComputedVecsFromDateIndex<Dollars>,

    pub dateindex_to_price_true_range: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub dateindex_to_price_true_range_2w_sum: EagerVec<PcoVec<DateIndex, StoredF32>>,
    pub indexes_to_price_2w_choppiness_index: ComputedVecsFromDateIndex<StoredF32>,

    pub indexes_to_price_1w_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_8d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_13d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_21d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_1m_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_34d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_55d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_89d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_144d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_200d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_1y_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_2y_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_200w_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_4y_sma: ComputedRatioVecsFromDateIndex,

    pub indexes_to_price_1w_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_8d_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_13d_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_21d_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_1m_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_34d_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_55d_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_89d_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_144d_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_200d_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_1y_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_2y_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_200w_ema: ComputedRatioVecsFromDateIndex,
    pub indexes_to_price_4y_ema: ComputedRatioVecsFromDateIndex,

    pub indexes_to_price_200d_sma_x2_4: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_price_200d_sma_x0_8: ComputedVecsFromDateIndex<Dollars>,

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

    pub _1d_price_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _1w_price_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _1m_price_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _3m_price_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _6m_price_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _1y_price_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _2y_price_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _3y_price_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _4y_price_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _5y_price_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _6y_price_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _8y_price_returns: ComputedVecsFromDateIndex<StoredF32>,
    pub _10y_price_returns: ComputedVecsFromDateIndex<StoredF32>,
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
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let db = Database::open(&parent_path.join("market"))?;
        db.set_min_len(PAGE_SIZE * 1_000_000)?;

        let version = parent_version + Version::ZERO;

        let this = Self {
            height_to_price_ath: EagerVec::forced_import(
                &db,
                "price_ath",
                version + Version::ZERO,
            )?,
            height_to_price_drawdown: EagerVec::forced_import(
                &db,
                "price_drawdown",
                version + Version::ZERO,
            )?,
            indexes_to_price_ath: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_ath",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_price_drawdown: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_drawdown",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_1d_returns_1w_sd: ComputedStandardDeviationVecsFromDateIndex::forced_import(
                &db,
                "1d_returns_1w_sd",
                7,
                Source::Compute,
                version + Version::ONE,
                indexes,
                StandardDeviationVecsOptions::default(),
            )?,
            indexes_to_1d_returns_1m_sd: ComputedStandardDeviationVecsFromDateIndex::forced_import(
                &db,
                "1d_returns_1m_sd",
                30,
                Source::Compute,
                version + Version::ONE,
                indexes,
                StandardDeviationVecsOptions::default(),
            )?,
            indexes_to_1d_returns_1y_sd: ComputedStandardDeviationVecsFromDateIndex::forced_import(
                &db,
                "1d_returns_1y_sd",
                365,
                Source::Compute,
                version + Version::ONE,
                indexes,
                StandardDeviationVecsOptions::default(),
            )?,
            indexes_to_price_1w_volatility: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_1w_volatility",
                Source::Compute,
                version + Version::TWO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_price_1m_volatility: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_1m_volatility",
                Source::Compute,
                version + Version::TWO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_price_1y_volatility: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_1y_volatility",
                Source::Compute,
                version + Version::TWO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_days_since_price_ath: ComputedVecsFromDateIndex::forced_import(
                &db,
                "days_since_price_ath",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_max_days_between_price_aths: ComputedVecsFromDateIndex::forced_import(
                &db,
                "max_days_between_price_aths",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_max_years_between_price_aths: ComputedVecsFromDateIndex::forced_import(
                &db,
                "max_years_between_price_aths",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,

            indexes_to_price_1w_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_1w_sma",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_8d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_8d_sma",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_13d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_13d_sma",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_21d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_21d_sma",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_1m_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_1m_sma",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_34d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_34d_sma",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_55d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_55d_sma",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_89d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_89d_sma",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_144d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_144d_sma",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_200d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_200d_sma",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_1y_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_1y_sma",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_2y_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_2y_sma",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_200w_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_200w_sma",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_4y_sma: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_4y_sma",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,

            indexes_to_price_1w_ema: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_1w_ema",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_8d_ema: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_8d_ema",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_13d_ema: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_13d_ema",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_21d_ema: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_21d_ema",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_1m_ema: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_1m_ema",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_34d_ema: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_34d_ema",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_55d_ema: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_55d_ema",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_89d_ema: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_89d_ema",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_144d_ema: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_144d_ema",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_200d_ema: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_200d_ema",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_1y_ema: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_1y_ema",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_2y_ema: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_2y_ema",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_200w_ema: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_200w_ema",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,
            indexes_to_price_4y_ema: ComputedRatioVecsFromDateIndex::forced_import(
                &db,
                "price_4y_ema",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                true,
            )?,

            _1d_price_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1d_price_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1w_price_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1w_price_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1m_price_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1m_price_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3m_price_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3m_price_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6m_price_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6m_price_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1y_price_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1y_price_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _2y_price_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "2y_price_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3y_price_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3y_price_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _4y_price_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "4y_price_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _5y_price_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "5y_price_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6y_price_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6y_price_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _8y_price_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "8y_price_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _10y_price_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "10y_price_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _2y_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "2y_cagr",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3y_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3y_cagr",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _4y_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "4y_cagr",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _5y_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "5y_cagr",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6y_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6y_cagr",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _8y_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "8y_cagr",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _10y_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "10y_cagr",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,

            _1w_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1w_dca_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1m_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1m_dca_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3m_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3m_dca_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6m_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6m_dca_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1y_dca_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _2y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "2y_dca_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3y_dca_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _4y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "4y_dca_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _5y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "5y_dca_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6y_dca_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _8y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "8y_dca_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _10y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "10y_dca_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _2y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "2y_dca_cagr",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3y_dca_cagr",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _4y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "4y_dca_cagr",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _5y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "5y_dca_cagr",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6y_dca_cagr",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _8y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "8y_dca_cagr",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _10y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                &db,
                "10y_dca_cagr",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1w_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1w_dca_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1m_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1m_dca_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3m_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3m_dca_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6m_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6m_dca_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1y_dca_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _2y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "2y_dca_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3y_dca_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _4y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "4y_dca_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _5y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "5y_dca_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6y_dca_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _8y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "8y_dca_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _10y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "10y_dca_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_1d_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_1d_ago",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_1w_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_1w_ago",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_1m_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_1m_ago",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_3m_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_3m_ago",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_6m_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_6m_ago",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_1y_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_1y_ago",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_2y_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_2y_ago",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_3y_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_3y_ago",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_4y_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_4y_ago",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_5y_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_5y_ago",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_6y_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_6y_ago",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_8y_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_8y_ago",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            price_10y_ago: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_10y_ago",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1w_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1w_dca_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1m_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1m_dca_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3m_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3m_dca_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6m_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6m_dca_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _1y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "1y_dca_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _2y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "2y_dca_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _3y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "3y_dca_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _4y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "4y_dca_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _5y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "5y_dca_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _6y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "6y_dca_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _8y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "8y_dca_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            _10y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "10y_dca_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,

            dca_class_2025_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2025_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2024_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2024_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2023_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2023_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2022_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2022_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2021_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2021_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2020_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2020_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2019_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2019_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2018_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2018_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2017_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2017_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2016_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2016_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2015_stack: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2015_stack",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,

            dca_class_2025_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2025_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2024_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2024_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2023_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2023_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2022_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2022_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2021_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2021_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2020_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2020_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2019_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2019_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2018_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2018_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2017_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2017_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2016_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2016_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2015_avg_price: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2015_avg_price",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,

            dca_class_2025_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2025_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2024_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2024_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2023_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2023_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2022_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2022_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2021_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2021_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2020_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2020_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2019_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2019_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2018_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2018_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2017_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2017_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2016_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2016_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dca_class_2015_returns: ComputedVecsFromDateIndex::forced_import(
                &db,
                "dca_class_2015_returns",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,

            indexes_to_price_200d_sma_x2_4: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_200d_sma_x2_4",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_price_200d_sma_x0_8: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_200d_sma_x0_8",
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            dateindex_to_price_true_range: EagerVec::forced_import(
                &db,
                "price_true_range",
                version + Version::ZERO,
            )?,
            dateindex_to_price_true_range_2w_sum: EagerVec::forced_import(
                &db,
                "price_true_range_2w_sum",
                version + Version::ZERO,
            )?,
            indexes_to_price_1w_min: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_1w_min",
                Source::Compute,
                version + Version::ONE,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_price_1w_max: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_1w_max",
                Source::Compute,
                version + Version::ONE,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_price_2w_min: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_2w_min",
                Source::Compute,
                version + Version::ONE,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_price_2w_max: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_2w_max",
                Source::Compute,
                version + Version::ONE,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_price_1m_min: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_1m_min",
                Source::Compute,
                version + Version::ONE,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_price_1m_max: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_1m_max",
                Source::Compute,
                version + Version::ONE,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_price_1y_min: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_1y_min",
                Source::Compute,
                version + Version::ONE,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_price_1y_max: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_1y_max",
                Source::Compute,
                version + Version::ONE,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_price_2w_choppiness_index: ComputedVecsFromDateIndex::forced_import(
                &db,
                "price_2w_choppiness_index",
                Source::Compute,
                version + Version::ONE,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            db,
        };

        this.db.retain_regions(
            this.iter_any_exportable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;

        this.db.compact()?;

        Ok(this)
    }

    pub fn compute(
        &mut self,
        price: &price::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.compute_(price, starting_indexes, exit)?;
        self.db.compact()?;
        Ok(())
    }

    fn compute_(
        &mut self,
        price: &price::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_price_ath.compute_all_time_high(
            starting_indexes.height,
            &price.chainindexes_to_price_high.height,
            exit,
        )?;
        self.height_to_price_drawdown.compute_drawdown(
            starting_indexes.height,
            &price.chainindexes_to_price_close.height,
            &self.height_to_price_ath,
            exit,
        )?;

        self.indexes_to_price_ath
            .compute_all(starting_indexes, exit, |v| {
                v.compute_all_time_high(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_high.dateindex.as_ref().unwrap(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_drawdown
            .compute_all(starting_indexes, exit, |v| {
                v.compute_drawdown(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_close.dateindex.as_ref().unwrap(),
                    self.indexes_to_price_ath.dateindex.as_ref().unwrap(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_days_since_price_ath
            .compute_all(starting_indexes, exit, |v| {
                let mut high_iter = price
                    .timeindexes_to_price_high
                    .dateindex
                    .as_ref()
                    .unwrap()
                    .into_iter();
                let mut prev = None;
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_price_ath.dateindex.as_ref().unwrap(),
                    |(i, ath, slf)| {
                        if prev.is_none() {
                            let i = i.to_usize();
                            prev.replace(if i > 0 {
                                slf.get_pushed_or_read_at_unwrap_once(i - 1)
                            } else {
                                StoredU16::default()
                            });
                        }
                        let days = if *high_iter.get_unwrap(i) == ath {
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
            })?;

        self.indexes_to_max_days_between_price_aths
            .compute_all(starting_indexes, exit, |v| {
                let mut prev = None;
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_days_since_price_ath
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    |(i, days, slf)| {
                        if prev.is_none() {
                            let i = i.to_usize();
                            prev.replace(if i > 0 {
                                slf.get_pushed_or_read_at_unwrap_once(i - 1)
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
            })?;

        self.indexes_to_max_years_between_price_aths
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_max_days_between_price_aths
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    |(i, max, ..)| (i, StoredF32::from(*max as f64 / 365.0)),
                    exit,
                )?;
                Ok(())
            })?;

        [
            (1, &mut self.price_1d_ago, &mut self._1d_price_returns, None),
            (7, &mut self.price_1w_ago, &mut self._1w_price_returns, None),
            (
                30,
                &mut self.price_1m_ago,
                &mut self._1m_price_returns,
                None,
            ),
            (
                3 * 30,
                &mut self.price_3m_ago,
                &mut self._3m_price_returns,
                None,
            ),
            (
                6 * 30,
                &mut self.price_6m_ago,
                &mut self._6m_price_returns,
                None,
            ),
            (
                365,
                &mut self.price_1y_ago,
                &mut self._1y_price_returns,
                None,
            ),
            (
                2 * 365,
                &mut self.price_2y_ago,
                &mut self._2y_price_returns,
                Some(&mut self._2y_cagr),
            ),
            (
                3 * 365,
                &mut self.price_3y_ago,
                &mut self._3y_price_returns,
                Some(&mut self._3y_cagr),
            ),
            (
                4 * 365,
                &mut self.price_4y_ago,
                &mut self._4y_price_returns,
                Some(&mut self._4y_cagr),
            ),
            (
                5 * 365,
                &mut self.price_5y_ago,
                &mut self._5y_price_returns,
                Some(&mut self._5y_cagr),
            ),
            (
                6 * 365,
                &mut self.price_6y_ago,
                &mut self._6y_price_returns,
                Some(&mut self._6y_cagr),
            ),
            (
                8 * 365,
                &mut self.price_8y_ago,
                &mut self._8y_price_returns,
                Some(&mut self._8y_cagr),
            ),
            (
                10 * 365,
                &mut self.price_10y_ago,
                &mut self._10y_price_returns,
                Some(&mut self._10y_cagr),
            ),
        ]
        .into_iter()
        .try_for_each(|(days, ago, returns, cagr)| -> Result<()> {
            ago.compute_all(starting_indexes, exit, |v| {
                v.compute_previous_value(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_close.dateindex.as_ref().unwrap(),
                    days,
                    exit,
                )?;
                Ok(())
            })?;

            returns.compute_all(starting_indexes, exit, |v| {
                v.compute_percentage_change(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_close.dateindex.as_ref().unwrap(),
                    days,
                    exit,
                )?;
                Ok(())
            })?;

            if let Some(cagr) = cagr {
                cagr.compute_all(starting_indexes, exit, |v| {
                    v.compute_cagr(
                        starting_indexes.dateindex,
                        returns.dateindex.as_ref().unwrap(),
                        days,
                        exit,
                    )?;
                    Ok(())
                })?;
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
                dca_stack.compute_all(starting_indexes, exit, |v| {
                    v.compute_dca_stack_via_len(
                        starting_indexes.dateindex,
                        price.timeindexes_to_price_close.dateindex.as_ref().unwrap(),
                        days,
                        exit,
                    )?;
                    Ok(())
                })?;

                dca_avg_price.compute_all(starting_indexes, exit, |v| {
                    v.compute_dca_avg_price_via_len(
                        starting_indexes.dateindex,
                        dca_stack.dateindex.as_ref().unwrap(),
                        days,
                        exit,
                    )?;
                    Ok(())
                })?;

                dca_returns.compute_all(starting_indexes, exit, |v| {
                    v.compute_percentage_difference(
                        starting_indexes.dateindex,
                        price.timeindexes_to_price_close.dateindex.as_ref().unwrap(),
                        dca_avg_price.dateindex.as_ref().unwrap(),
                        exit,
                    )?;
                    Ok(())
                })?;

                if let Some(dca_cagr) = dca_cagr {
                    dca_cagr.compute_all(starting_indexes, exit, |v| {
                        v.compute_cagr(
                            starting_indexes.dateindex,
                            dca_returns.dateindex.as_ref().unwrap(),
                            days,
                            exit,
                        )?;
                        Ok(())
                    })?;
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

            stack.compute_all(starting_indexes, exit, |v| {
                v.compute_dca_stack_via_from(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_close.dateindex.as_ref().unwrap(),
                    dateindex,
                    exit,
                )?;
                Ok(())
            })?;

            avg_price.compute_all(starting_indexes, exit, |v| {
                v.compute_dca_avg_price_via_from(
                    starting_indexes.dateindex,
                    stack.dateindex.as_ref().unwrap(),
                    dateindex,
                    exit,
                )?;
                Ok(())
            })?;

            returns.compute_all(starting_indexes, exit, |v| {
                v.compute_percentage_difference(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_close.dateindex.as_ref().unwrap(),
                    avg_price.dateindex.as_ref().unwrap(),
                    exit,
                )?;
                Ok(())
            })?;

            Ok(())
        })?;

        thread::scope(|s| -> Result<()> {
            [
                (
                    &mut self.indexes_to_price_1w_sma,
                    &mut self.indexes_to_price_1w_ema,
                    7,
                ),
                (
                    &mut self.indexes_to_price_8d_sma,
                    &mut self.indexes_to_price_8d_ema,
                    8,
                ),
                (
                    &mut self.indexes_to_price_13d_sma,
                    &mut self.indexes_to_price_13d_ema,
                    13,
                ),
                (
                    &mut self.indexes_to_price_21d_sma,
                    &mut self.indexes_to_price_21d_ema,
                    21,
                ),
                (
                    &mut self.indexes_to_price_1m_sma,
                    &mut self.indexes_to_price_1m_ema,
                    30,
                ),
                (
                    &mut self.indexes_to_price_34d_sma,
                    &mut self.indexes_to_price_34d_ema,
                    34,
                ),
                (
                    &mut self.indexes_to_price_55d_sma,
                    &mut self.indexes_to_price_55d_ema,
                    55,
                ),
                (
                    &mut self.indexes_to_price_89d_sma,
                    &mut self.indexes_to_price_89d_ema,
                    89,
                ),
                (
                    &mut self.indexes_to_price_144d_sma,
                    &mut self.indexes_to_price_144d_ema,
                    144,
                ),
                (
                    &mut self.indexes_to_price_200d_sma,
                    &mut self.indexes_to_price_200d_ema,
                    200,
                ),
                (
                    &mut self.indexes_to_price_1y_sma,
                    &mut self.indexes_to_price_1y_ema,
                    365,
                ),
                (
                    &mut self.indexes_to_price_2y_sma,
                    &mut self.indexes_to_price_2y_ema,
                    2 * 365,
                ),
                (
                    &mut self.indexes_to_price_200w_sma,
                    &mut self.indexes_to_price_200w_ema,
                    200 * 7,
                ),
                (
                    &mut self.indexes_to_price_4y_sma,
                    &mut self.indexes_to_price_4y_ema,
                    4 * 365,
                ),
            ]
            .into_iter()
            .for_each(|(sma, ema, days)| {
                s.spawn(move || -> Result<()> {
                    sma.compute_all(price, starting_indexes, exit, |v| {
                        v.compute_sma(
                            starting_indexes.dateindex,
                            price.timeindexes_to_price_close.dateindex.as_ref().unwrap(),
                            days,
                            exit,
                        )?;
                        Ok(())
                    })?;

                    ema.compute_all(price, starting_indexes, exit, |v| {
                        v.compute_ema(
                            starting_indexes.dateindex,
                            price.timeindexes_to_price_close.dateindex.as_ref().unwrap(),
                            days,
                            exit,
                        )?;
                        Ok(())
                    })
                });
            });
            Ok(())
        })?;

        self.indexes_to_price_200d_sma_x0_8
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_price_200d_sma
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
            })?;

        self.indexes_to_price_200d_sma_x2_4
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_price_200d_sma
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
            })?;

        self.indexes_to_1d_returns_1w_sd.compute_all(
            starting_indexes,
            exit,
            self._1d_price_returns.dateindex.as_ref().unwrap(),
            None as Option<&EagerVec<PcoVec<DateIndex, Dollars>>>,
        )?;
        self.indexes_to_1d_returns_1m_sd.compute_all(
            starting_indexes,
            exit,
            self._1d_price_returns.dateindex.as_ref().unwrap(),
            None as Option<&EagerVec<PcoVec<DateIndex, Dollars>>>,
        )?;
        self.indexes_to_1d_returns_1y_sd.compute_all(
            starting_indexes,
            exit,
            self._1d_price_returns.dateindex.as_ref().unwrap(),
            None as Option<&EagerVec<PcoVec<DateIndex, Dollars>>>,
        )?;

        self.indexes_to_price_1w_volatility
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_1d_returns_1w_sd
                        .sd
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    |(i, v, ..)| (i, (*v * 7.0_f32.sqrt()).into()),
                    exit,
                )?;
                Ok(())
            })?;
        self.indexes_to_price_1m_volatility
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_1d_returns_1m_sd
                        .sd
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    |(i, v, ..)| (i, (*v * 30.0_f32.sqrt()).into()),
                    exit,
                )?;
                Ok(())
            })?;
        self.indexes_to_price_1y_volatility
            .compute_all(starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_1d_returns_1y_sd
                        .sd
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    |(i, v, ..)| (i, (*v * 365.0_f32.sqrt()).into()),
                    exit,
                )?;
                Ok(())
            })?;

        self.dateindex_to_price_true_range.compute_transform3(
            starting_indexes.dateindex,
            price.timeindexes_to_price_open.dateindex.as_ref().unwrap(),
            price.timeindexes_to_price_high.dateindex.as_ref().unwrap(),
            price.timeindexes_to_price_low.dateindex.as_ref().unwrap(),
            |(i, open, high, low, ..)| {
                let high_min_low = **high - **low;
                let high_min_open = (**high - **open).abs();
                let low_min_open = (**low - **open).abs();
                (i, high_min_low.max(high_min_open).max(low_min_open).into())
            },
            exit,
        )?;

        self.dateindex_to_price_true_range_2w_sum.compute_sum(
            starting_indexes.dateindex,
            &self.dateindex_to_price_true_range,
            14,
            exit,
        )?;

        self.indexes_to_price_1w_max
            .compute_all(starting_indexes, exit, |v| {
                v.compute_max(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_high.dateindex.as_ref().unwrap(),
                    7,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_1w_min
            .compute_all(starting_indexes, exit, |v| {
                v.compute_min(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_low.dateindex.as_ref().unwrap(),
                    7,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_2w_max
            .compute_all(starting_indexes, exit, |v| {
                v.compute_max(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_high.dateindex.as_ref().unwrap(),
                    14,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_2w_min
            .compute_all(starting_indexes, exit, |v| {
                v.compute_min(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_low.dateindex.as_ref().unwrap(),
                    14,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_1m_max
            .compute_all(starting_indexes, exit, |v| {
                v.compute_max(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_high.dateindex.as_ref().unwrap(),
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_1m_min
            .compute_all(starting_indexes, exit, |v| {
                v.compute_min(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_low.dateindex.as_ref().unwrap(),
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_1y_max
            .compute_all(starting_indexes, exit, |v| {
                v.compute_max(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_high.dateindex.as_ref().unwrap(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_1y_min
            .compute_all(starting_indexes, exit, |v| {
                v.compute_min(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_low.dateindex.as_ref().unwrap(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_price_2w_choppiness_index
            .compute_all(starting_indexes, exit, |v| {
                let n = 14;
                let log10n = (n as f32).log10();
                v.compute_transform3(
                    starting_indexes.dateindex,
                    &self.dateindex_to_price_true_range_2w_sum,
                    self.indexes_to_price_2w_max.dateindex.as_ref().unwrap(),
                    self.indexes_to_price_2w_min.dateindex.as_ref().unwrap(),
                    |(i, tr_sum, max, min, ..)| {
                        (
                            i,
                            StoredF32::from(
                                100.0 * (*tr_sum / (*max - *min) as f32).log10() / log10n,
                            ),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
