use std::{fs, path::Path, thread};

use brk_core::{Date, DateIndex, Dollars, Sats, StoredF32, StoredUsize, Version};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyCollectableVec, Compressed, Computation, StoredIndex, VecIterator};

use super::{
    Indexes, fetched,
    grouped::{
        ComputedRatioVecsFromDateIndex, ComputedVecsFromDateIndex, StorableVecGeneatorOptions,
    },
    indexes, transactions,
};

const VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct Vecs {
    pub indexes_to_marketcap: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_ath: ComputedVecsFromDateIndex<Dollars>,
    pub indexes_to_drawdown: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_days_since_ath: ComputedVecsFromDateIndex<StoredUsize>,
    pub indexes_to_max_days_between_ath: ComputedVecsFromDateIndex<StoredUsize>,
    pub indexes_to_max_years_between_ath: ComputedVecsFromDateIndex<StoredF32>,

    pub indexes_to_1w_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_8d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_13d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_21d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_1m_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_34d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_55d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_89d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_144d_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_1y_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_2y_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_200w_sma: ComputedRatioVecsFromDateIndex,
    pub indexes_to_4y_sma: ComputedRatioVecsFromDateIndex,

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
        path: &Path,
        version: Version,
        _computation: Computation,
        compressed: Compressed,
    ) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            indexes_to_marketcap: ComputedVecsFromDateIndex::forced_import(
                path,
                "marketcap",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_ath: ComputedVecsFromDateIndex::forced_import(
                path,
                "ath",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_drawdown: ComputedVecsFromDateIndex::forced_import(
                path,
                "drawdown",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_days_since_ath: ComputedVecsFromDateIndex::forced_import(
                path,
                "days_since_ath",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_max_days_between_ath: ComputedVecsFromDateIndex::forced_import(
                path,
                "max_days_between_ath",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_max_years_between_ath: ComputedVecsFromDateIndex::forced_import(
                path,
                "max_years_between_ath",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,

            indexes_to_1w_sma: ComputedRatioVecsFromDateIndex::forced_import(
                path,
                "1w_sma",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_8d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                path,
                "8d_sma",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_13d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                path,
                "13d_sma",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_21d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                path,
                "21d_sma",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_1m_sma: ComputedRatioVecsFromDateIndex::forced_import(
                path,
                "1m_sma",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_34d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                path,
                "34d_sma",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_55d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                path,
                "55d_sma",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_89d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                path,
                "89d_sma",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_144d_sma: ComputedRatioVecsFromDateIndex::forced_import(
                path,
                "144d_sma",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_1y_sma: ComputedRatioVecsFromDateIndex::forced_import(
                path,
                "1y_sma",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_2y_sma: ComputedRatioVecsFromDateIndex::forced_import(
                path,
                "2y_sma",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_200w_sma: ComputedRatioVecsFromDateIndex::forced_import(
                path,
                "200w_sma",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            indexes_to_4y_sma: ComputedRatioVecsFromDateIndex::forced_import(
                path,
                "4y_sma",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,

            _1d_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "1d_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _1w_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "1w_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _1m_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "1m_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _3m_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "3m_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _6m_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "6m_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _1y_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "1y_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _2y_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "2y_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _3y_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "3y_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _4y_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "4y_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _5y_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "5y_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _6y_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "6y_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _8y_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "8y_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _10y_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "10y_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _2y_cagr: ComputedVecsFromDateIndex::forced_import(
                path,
                "2y_cagr",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _3y_cagr: ComputedVecsFromDateIndex::forced_import(
                path,
                "3y_cagr",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _4y_cagr: ComputedVecsFromDateIndex::forced_import(
                path,
                "4y_cagr",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _5y_cagr: ComputedVecsFromDateIndex::forced_import(
                path,
                "5y_cagr",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _6y_cagr: ComputedVecsFromDateIndex::forced_import(
                path,
                "6y_cagr",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _8y_cagr: ComputedVecsFromDateIndex::forced_import(
                path,
                "8y_cagr",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _10y_cagr: ComputedVecsFromDateIndex::forced_import(
                path,
                "10y_cagr",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,

            _1w_dca_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "1w_dca_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _1m_dca_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "1m_dca_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _3m_dca_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "3m_dca_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _6m_dca_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "6m_dca_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _1y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "1y_dca_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _2y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "2y_dca_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _3y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "3y_dca_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _4y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "4y_dca_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _5y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "5y_dca_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _6y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "6y_dca_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _8y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "8y_dca_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _10y_dca_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "10y_dca_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _2y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                path,
                "2y_dca_cagr",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _3y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                path,
                "3y_dca_cagr",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _4y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                path,
                "4y_dca_cagr",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _5y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                path,
                "5y_dca_cagr",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _6y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                path,
                "6y_dca_cagr",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _8y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                path,
                "8y_dca_cagr",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _10y_dca_cagr: ComputedVecsFromDateIndex::forced_import(
                path,
                "10y_dca_cagr",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _1w_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "1w_dca_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _1m_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "1m_dca_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _3m_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "3m_dca_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _6m_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "6m_dca_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _1y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "1y_dca_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _2y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "2y_dca_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _3y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "3y_dca_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _4y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "4y_dca_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _5y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "5y_dca_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _6y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "6y_dca_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _8y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "8y_dca_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _10y_dca_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "10y_dca_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            price_1d_ago: ComputedVecsFromDateIndex::forced_import(
                path,
                "price_1d_ago",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            price_1w_ago: ComputedVecsFromDateIndex::forced_import(
                path,
                "price_1w_ago",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            price_1m_ago: ComputedVecsFromDateIndex::forced_import(
                path,
                "price_1m_ago",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            price_3m_ago: ComputedVecsFromDateIndex::forced_import(
                path,
                "price_3m_ago",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            price_6m_ago: ComputedVecsFromDateIndex::forced_import(
                path,
                "price_6m_ago",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            price_1y_ago: ComputedVecsFromDateIndex::forced_import(
                path,
                "price_1y_ago",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            price_2y_ago: ComputedVecsFromDateIndex::forced_import(
                path,
                "price_2y_ago",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            price_3y_ago: ComputedVecsFromDateIndex::forced_import(
                path,
                "price_3y_ago",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            price_4y_ago: ComputedVecsFromDateIndex::forced_import(
                path,
                "price_4y_ago",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            price_5y_ago: ComputedVecsFromDateIndex::forced_import(
                path,
                "price_5y_ago",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            price_6y_ago: ComputedVecsFromDateIndex::forced_import(
                path,
                "price_6y_ago",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            price_8y_ago: ComputedVecsFromDateIndex::forced_import(
                path,
                "price_8y_ago",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            price_10y_ago: ComputedVecsFromDateIndex::forced_import(
                path,
                "price_10y_ago",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _1w_dca_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "1w_dca_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _1m_dca_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "1m_dca_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _3m_dca_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "3m_dca_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _6m_dca_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "6m_dca_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _1y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "1y_dca_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _2y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "2y_dca_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _3y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "3y_dca_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _4y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "4y_dca_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _5y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "5y_dca_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _6y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "6y_dca_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _8y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "8y_dca_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            _10y_dca_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "10y_dca_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,

            dca_class_2025_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2025_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2024_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2024_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2023_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2023_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2022_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2022_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2021_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2021_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2020_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2020_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2019_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2019_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2018_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2018_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2017_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2017_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2016_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2016_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2015_stack: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2015_stack",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,

            dca_class_2025_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2025_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2024_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2024_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2023_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2023_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2022_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2022_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2021_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2021_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2020_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2020_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2019_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2019_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2018_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2018_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2017_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2017_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2016_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2016_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2015_avg_price: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2015_avg_price",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,

            dca_class_2025_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2025_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2024_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2024_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2023_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2023_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2022_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2022_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2021_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2021_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2020_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2020_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2019_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2019_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2018_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2018_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2017_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2017_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2016_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2016_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
            dca_class_2015_returns: ComputedVecsFromDateIndex::forced_import(
                path,
                "dca_class_2015_returns",
                true,
                version + VERSION + Version::ZERO,
                compressed,
                StorableVecGeneatorOptions::default().add_last(),
            )?,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        fetched: &fetched::Vecs,
        transactions: &mut transactions::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()> {
        self.indexes_to_marketcap.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                let mut cumulative_subsidy_in_btc = transactions
                    .indexes_to_subsidy
                    .bitcoin
                    .dateindex
                    .unwrap_cumulative()
                    .into_iter();
                v.compute_transform(
                    starting_indexes.dateindex,
                    fetched.timeindexes_to_close.dateindex.as_ref().unwrap(),
                    |(i, close, ..)| {
                        let supply = cumulative_subsidy_in_btc.unwrap_get_inner(i);
                        (i, *close * supply)
                    },
                    exit,
                )
            },
        )?;

        self.indexes_to_ath.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                let mut prev = None;
                v.compute_transform(
                    starting_indexes.dateindex,
                    fetched.timeindexes_to_high.dateindex.as_ref().unwrap(),
                    |(i, high, slf)| {
                        if prev.is_none() {
                            let i = i.unwrap_to_usize();
                            prev.replace(if i > 0 {
                                slf.into_iter().unwrap_get_inner_(i - 1)
                            } else {
                                Dollars::ZERO
                            });
                        }
                        let ath = prev.unwrap().max(*high);
                        prev.replace(ath);
                        (i, ath)
                    },
                    exit,
                )
            },
        )?;

        self.indexes_to_drawdown.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                let mut close_iter = fetched
                    .timeindexes_to_close
                    .dateindex
                    .as_ref()
                    .unwrap()
                    .into_iter();

                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_ath.dateindex.as_ref().unwrap(),
                    |(i, ath, ..)| {
                        if ath == Dollars::ZERO {
                            return (i, StoredF32::default());
                        }
                        let close = *close_iter.unwrap_get_inner(i);
                        let drawdown = StoredF32::from((*ath - *close) / *ath * -100.0);
                        (i, drawdown)
                    },
                    exit,
                )
            },
        )?;

        self.indexes_to_days_since_ath.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                let mut high_iter = fetched
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
                                StoredUsize::default()
                            });
                        }
                        let days = if *high_iter.unwrap_get_inner(i) == ath {
                            StoredUsize::default()
                        } else {
                            prev.unwrap() + StoredUsize::from(1)
                        };
                        prev.replace(days);
                        (i, days)
                    },
                    exit,
                )
            },
        )?;

        self.indexes_to_max_days_between_ath.compute_all(
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
                                StoredUsize::ZERO
                            });
                        }
                        let max = prev.unwrap().max(days);
                        prev.replace(max);
                        (i, max)
                    },
                    exit,
                )
            },
        )?;

        self.indexes_to_max_years_between_ath.compute_all(
            indexer,
            indexes,
            starting_indexes,
            exit,
            |v, _, _, starting_indexes, exit| {
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_max_days_between_ath
                        .dateindex
                        .as_ref()
                        .unwrap(),
                    |(i, max, ..)| (i, StoredF32::from(*max as f64 / 365.0)),
                    exit,
                )
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
        .try_for_each(|(days, ago, returns, cagr)| -> color_eyre::Result<()> {
            ago.compute_all(
                indexer,
                indexes,
                starting_indexes,
                exit,
                |v, _, _, starting_indexes, exit| {
                    v.compute_previous_value(
                        starting_indexes.dateindex,
                        fetched.timeindexes_to_close.dateindex.as_ref().unwrap(),
                        days,
                        exit,
                    )
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
                        fetched.timeindexes_to_close.dateindex.as_ref().unwrap(),
                        days,
                        exit,
                    )
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
                        )
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
            |(days, dca_stack, dca_avg_price, dca_returns, dca_cagr)| -> color_eyre::Result<()> {
                dca_stack.compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |v, _, _, starting_indexes, exit| {
                        v.compute_dca_stack_via_len(
                            starting_indexes.dateindex,
                            fetched.timeindexes_to_close.dateindex.as_ref().unwrap(),
                            days,
                            exit,
                        )
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
                        )
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
                            fetched.timeindexes_to_close.dateindex.as_ref().unwrap(),
                            dca_avg_price.dateindex.as_ref().unwrap(),
                            exit,
                        )
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
                            )
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
        .try_for_each(
            |(year, avg_price, returns, stack)| -> color_eyre::Result<()> {
                let dateindex = DateIndex::try_from(Date::new(year, 1, 1)).unwrap();

                stack.compute_all(
                    indexer,
                    indexes,
                    starting_indexes,
                    exit,
                    |v, _, _, starting_indexes, exit| {
                        v.compute_dca_stack_via_from(
                            starting_indexes.dateindex,
                            fetched.timeindexes_to_close.dateindex.as_ref().unwrap(),
                            dateindex,
                            exit,
                        )
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
                        )
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
                            fetched.timeindexes_to_close.dateindex.as_ref().unwrap(),
                            avg_price.dateindex.as_ref().unwrap(),
                            exit,
                        )
                    },
                )?;

                Ok(())
            },
        )?;

        thread::scope(|s| -> color_eyre::Result<()> {
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
                (&mut self.indexes_to_1y_sma, 365),
                (&mut self.indexes_to_2y_sma, 2 * 365),
                (&mut self.indexes_to_200w_sma, 200 * 7),
                (&mut self.indexes_to_4y_sma, 4 * 365),
            ]
            .into_iter()
            .for_each(|(vecs, sma)| {
                s.spawn(move || -> color_eyre::Result<()> {
                    vecs.compute_all(
                        indexer,
                        indexes,
                        fetched,
                        starting_indexes,
                        exit,
                        |v, _, _, starting_indexes, exit| {
                            v.compute_sma(
                                starting_indexes.dateindex,
                                fetched.timeindexes_to_close.dateindex.as_ref().unwrap(),
                                sma,
                                exit,
                            )
                        },
                    )
                });
            });
            Ok(())
        })
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        [
            self.indexes_to_marketcap.vecs(),
            self.indexes_to_ath.vecs(),
            self.indexes_to_drawdown.vecs(),
            self.indexes_to_days_since_ath.vecs(),
            self.indexes_to_max_days_between_ath.vecs(),
            self.indexes_to_max_years_between_ath.vecs(),
            self.indexes_to_1w_sma.vecs(),
            self.indexes_to_8d_sma.vecs(),
            self.indexes_to_13d_sma.vecs(),
            self.indexes_to_21d_sma.vecs(),
            self.indexes_to_1m_sma.vecs(),
            self.indexes_to_34d_sma.vecs(),
            self.indexes_to_55d_sma.vecs(),
            self.indexes_to_89d_sma.vecs(),
            self.indexes_to_144d_sma.vecs(),
            self.indexes_to_1y_sma.vecs(),
            self.indexes_to_2y_sma.vecs(),
            self.indexes_to_200w_sma.vecs(),
            self.indexes_to_4y_sma.vecs(),
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
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}
