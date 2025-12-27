use std::path::Path;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec, PAGE_SIZE};

use crate::{
    grouped::{
        ComputedRatioVecsFromDateIndex, ComputedStandardDeviationVecsFromDateIndex,
        ComputedVecsFromDateIndex, Source, StandardDeviationVecsOptions, VecBuilderOptions,
    },
    indexes,
};

use super::Vecs;

impl Vecs {
    pub fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let db = Database::open(&parent_path.join(super::DB_NAME))?;
        db.set_min_len(PAGE_SIZE * 1_000_000)?;

        let version = parent_version + Version::ZERO;
        let v0 = Version::ZERO;
        let v1 = Version::ONE;
        let v2 = Version::TWO;
        let last = VecBuilderOptions::default().add_last();

        macro_rules! computed_di {
            ($name:expr) => {
                ComputedVecsFromDateIndex::forced_import(
                    &db,
                    $name,
                    Source::Compute,
                    version + v0,
                    indexes,
                    last.clone(),
                )?
            };
            ($name:expr, $v:expr) => {
                ComputedVecsFromDateIndex::forced_import(
                    &db,
                    $name,
                    Source::Compute,
                    version + $v,
                    indexes,
                    last.clone(),
                )?
            };
        }
        macro_rules! ratio_di {
            ($name:expr) => {
                ComputedRatioVecsFromDateIndex::forced_import(
                    &db,
                    $name,
                    Source::Compute,
                    version + v0,
                    indexes,
                    true,
                )?
            };
        }
        macro_rules! sd_di {
            ($name:expr, $window:expr, $v:expr) => {
                ComputedStandardDeviationVecsFromDateIndex::forced_import(
                    &db,
                    $name,
                    $window,
                    Source::Compute,
                    version + $v,
                    indexes,
                    StandardDeviationVecsOptions::default(),
                )?
            };
        }
        macro_rules! eager_h {
            ($name:expr, $v:expr) => {
                EagerVec::forced_import(&db, $name, version + $v)?
            };
        }
        macro_rules! eager_di {
            ($name:expr, $v:expr) => {
                EagerVec::forced_import(&db, $name, version + $v)?
            };
        }

        let this = Self {
            height_to_price_ath: eager_h!("price_ath", v0),
            height_to_price_drawdown: eager_h!("price_drawdown", v0),
            indexes_to_price_ath: computed_di!("price_ath"),
            indexes_to_price_drawdown: computed_di!("price_drawdown"),
            indexes_to_1d_returns_1w_sd: sd_di!("1d_returns_1w_sd", 7, v1),
            indexes_to_1d_returns_1m_sd: sd_di!("1d_returns_1m_sd", 30, v1),
            indexes_to_1d_returns_1y_sd: sd_di!("1d_returns_1y_sd", 365, v1),
            indexes_to_price_1w_volatility: computed_di!("price_1w_volatility", v2),
            indexes_to_price_1m_volatility: computed_di!("price_1m_volatility", v2),
            indexes_to_price_1y_volatility: computed_di!("price_1y_volatility", v2),
            indexes_to_days_since_price_ath: computed_di!("days_since_price_ath"),
            indexes_to_max_days_between_price_aths: computed_di!("max_days_between_price_aths"),
            indexes_to_max_years_between_price_aths: computed_di!("max_years_between_price_aths"),

            indexes_to_price_1w_sma: ratio_di!("price_1w_sma"),
            indexes_to_price_8d_sma: ratio_di!("price_8d_sma"),
            indexes_to_price_13d_sma: ratio_di!("price_13d_sma"),
            indexes_to_price_21d_sma: ratio_di!("price_21d_sma"),
            indexes_to_price_1m_sma: ratio_di!("price_1m_sma"),
            indexes_to_price_34d_sma: ratio_di!("price_34d_sma"),
            indexes_to_price_55d_sma: ratio_di!("price_55d_sma"),
            indexes_to_price_89d_sma: ratio_di!("price_89d_sma"),
            indexes_to_price_144d_sma: ratio_di!("price_144d_sma"),
            indexes_to_price_200d_sma: ratio_di!("price_200d_sma"),
            indexes_to_price_1y_sma: ratio_di!("price_1y_sma"),
            indexes_to_price_2y_sma: ratio_di!("price_2y_sma"),
            indexes_to_price_200w_sma: ratio_di!("price_200w_sma"),
            indexes_to_price_4y_sma: ratio_di!("price_4y_sma"),

            indexes_to_price_1w_ema: ratio_di!("price_1w_ema"),
            indexes_to_price_8d_ema: ratio_di!("price_8d_ema"),
            indexes_to_price_13d_ema: ratio_di!("price_13d_ema"),
            indexes_to_price_21d_ema: ratio_di!("price_21d_ema"),
            indexes_to_price_1m_ema: ratio_di!("price_1m_ema"),
            indexes_to_price_34d_ema: ratio_di!("price_34d_ema"),
            indexes_to_price_55d_ema: ratio_di!("price_55d_ema"),
            indexes_to_price_89d_ema: ratio_di!("price_89d_ema"),
            indexes_to_price_144d_ema: ratio_di!("price_144d_ema"),
            indexes_to_price_200d_ema: ratio_di!("price_200d_ema"),
            indexes_to_price_1y_ema: ratio_di!("price_1y_ema"),
            indexes_to_price_2y_ema: ratio_di!("price_2y_ema"),
            indexes_to_price_200w_ema: ratio_di!("price_200w_ema"),
            indexes_to_price_4y_ema: ratio_di!("price_4y_ema"),

            _1d_price_returns: computed_di!("1d_price_returns"),
            _1w_price_returns: computed_di!("1w_price_returns"),
            _1m_price_returns: computed_di!("1m_price_returns"),
            _3m_price_returns: computed_di!("3m_price_returns"),
            _6m_price_returns: computed_di!("6m_price_returns"),
            _1y_price_returns: computed_di!("1y_price_returns"),
            _2y_price_returns: computed_di!("2y_price_returns"),
            _3y_price_returns: computed_di!("3y_price_returns"),
            _4y_price_returns: computed_di!("4y_price_returns"),
            _5y_price_returns: computed_di!("5y_price_returns"),
            _6y_price_returns: computed_di!("6y_price_returns"),
            _8y_price_returns: computed_di!("8y_price_returns"),
            _10y_price_returns: computed_di!("10y_price_returns"),
            _2y_cagr: computed_di!("2y_cagr"),
            _3y_cagr: computed_di!("3y_cagr"),
            _4y_cagr: computed_di!("4y_cagr"),
            _5y_cagr: computed_di!("5y_cagr"),
            _6y_cagr: computed_di!("6y_cagr"),
            _8y_cagr: computed_di!("8y_cagr"),
            _10y_cagr: computed_di!("10y_cagr"),

            _1w_dca_returns: computed_di!("1w_dca_returns"),
            _1m_dca_returns: computed_di!("1m_dca_returns"),
            _3m_dca_returns: computed_di!("3m_dca_returns"),
            _6m_dca_returns: computed_di!("6m_dca_returns"),
            _1y_dca_returns: computed_di!("1y_dca_returns"),
            _2y_dca_returns: computed_di!("2y_dca_returns"),
            _3y_dca_returns: computed_di!("3y_dca_returns"),
            _4y_dca_returns: computed_di!("4y_dca_returns"),
            _5y_dca_returns: computed_di!("5y_dca_returns"),
            _6y_dca_returns: computed_di!("6y_dca_returns"),
            _8y_dca_returns: computed_di!("8y_dca_returns"),
            _10y_dca_returns: computed_di!("10y_dca_returns"),
            _2y_dca_cagr: computed_di!("2y_dca_cagr"),
            _3y_dca_cagr: computed_di!("3y_dca_cagr"),
            _4y_dca_cagr: computed_di!("4y_dca_cagr"),
            _5y_dca_cagr: computed_di!("5y_dca_cagr"),
            _6y_dca_cagr: computed_di!("6y_dca_cagr"),
            _8y_dca_cagr: computed_di!("8y_dca_cagr"),
            _10y_dca_cagr: computed_di!("10y_dca_cagr"),
            _1w_dca_avg_price: computed_di!("1w_dca_avg_price"),
            _1m_dca_avg_price: computed_di!("1m_dca_avg_price"),
            _3m_dca_avg_price: computed_di!("3m_dca_avg_price"),
            _6m_dca_avg_price: computed_di!("6m_dca_avg_price"),
            _1y_dca_avg_price: computed_di!("1y_dca_avg_price"),
            _2y_dca_avg_price: computed_di!("2y_dca_avg_price"),
            _3y_dca_avg_price: computed_di!("3y_dca_avg_price"),
            _4y_dca_avg_price: computed_di!("4y_dca_avg_price"),
            _5y_dca_avg_price: computed_di!("5y_dca_avg_price"),
            _6y_dca_avg_price: computed_di!("6y_dca_avg_price"),
            _8y_dca_avg_price: computed_di!("8y_dca_avg_price"),
            _10y_dca_avg_price: computed_di!("10y_dca_avg_price"),
            price_1d_ago: computed_di!("price_1d_ago"),
            price_1w_ago: computed_di!("price_1w_ago"),
            price_1m_ago: computed_di!("price_1m_ago"),
            price_3m_ago: computed_di!("price_3m_ago"),
            price_6m_ago: computed_di!("price_6m_ago"),
            price_1y_ago: computed_di!("price_1y_ago"),
            price_2y_ago: computed_di!("price_2y_ago"),
            price_3y_ago: computed_di!("price_3y_ago"),
            price_4y_ago: computed_di!("price_4y_ago"),
            price_5y_ago: computed_di!("price_5y_ago"),
            price_6y_ago: computed_di!("price_6y_ago"),
            price_8y_ago: computed_di!("price_8y_ago"),
            price_10y_ago: computed_di!("price_10y_ago"),
            _1w_dca_stack: computed_di!("1w_dca_stack"),
            _1m_dca_stack: computed_di!("1m_dca_stack"),
            _3m_dca_stack: computed_di!("3m_dca_stack"),
            _6m_dca_stack: computed_di!("6m_dca_stack"),
            _1y_dca_stack: computed_di!("1y_dca_stack"),
            _2y_dca_stack: computed_di!("2y_dca_stack"),
            _3y_dca_stack: computed_di!("3y_dca_stack"),
            _4y_dca_stack: computed_di!("4y_dca_stack"),
            _5y_dca_stack: computed_di!("5y_dca_stack"),
            _6y_dca_stack: computed_di!("6y_dca_stack"),
            _8y_dca_stack: computed_di!("8y_dca_stack"),
            _10y_dca_stack: computed_di!("10y_dca_stack"),

            dca_class_2025_stack: computed_di!("dca_class_2025_stack"),
            dca_class_2024_stack: computed_di!("dca_class_2024_stack"),
            dca_class_2023_stack: computed_di!("dca_class_2023_stack"),
            dca_class_2022_stack: computed_di!("dca_class_2022_stack"),
            dca_class_2021_stack: computed_di!("dca_class_2021_stack"),
            dca_class_2020_stack: computed_di!("dca_class_2020_stack"),
            dca_class_2019_stack: computed_di!("dca_class_2019_stack"),
            dca_class_2018_stack: computed_di!("dca_class_2018_stack"),
            dca_class_2017_stack: computed_di!("dca_class_2017_stack"),
            dca_class_2016_stack: computed_di!("dca_class_2016_stack"),
            dca_class_2015_stack: computed_di!("dca_class_2015_stack"),

            dca_class_2025_avg_price: computed_di!("dca_class_2025_avg_price"),
            dca_class_2024_avg_price: computed_di!("dca_class_2024_avg_price"),
            dca_class_2023_avg_price: computed_di!("dca_class_2023_avg_price"),
            dca_class_2022_avg_price: computed_di!("dca_class_2022_avg_price"),
            dca_class_2021_avg_price: computed_di!("dca_class_2021_avg_price"),
            dca_class_2020_avg_price: computed_di!("dca_class_2020_avg_price"),
            dca_class_2019_avg_price: computed_di!("dca_class_2019_avg_price"),
            dca_class_2018_avg_price: computed_di!("dca_class_2018_avg_price"),
            dca_class_2017_avg_price: computed_di!("dca_class_2017_avg_price"),
            dca_class_2016_avg_price: computed_di!("dca_class_2016_avg_price"),
            dca_class_2015_avg_price: computed_di!("dca_class_2015_avg_price"),

            dca_class_2025_returns: computed_di!("dca_class_2025_returns"),
            dca_class_2024_returns: computed_di!("dca_class_2024_returns"),
            dca_class_2023_returns: computed_di!("dca_class_2023_returns"),
            dca_class_2022_returns: computed_di!("dca_class_2022_returns"),
            dca_class_2021_returns: computed_di!("dca_class_2021_returns"),
            dca_class_2020_returns: computed_di!("dca_class_2020_returns"),
            dca_class_2019_returns: computed_di!("dca_class_2019_returns"),
            dca_class_2018_returns: computed_di!("dca_class_2018_returns"),
            dca_class_2017_returns: computed_di!("dca_class_2017_returns"),
            dca_class_2016_returns: computed_di!("dca_class_2016_returns"),
            dca_class_2015_returns: computed_di!("dca_class_2015_returns"),

            indexes_to_price_200d_sma_x2_4: computed_di!("price_200d_sma_x2_4"),
            indexes_to_price_200d_sma_x0_8: computed_di!("price_200d_sma_x0_8"),
            dateindex_to_price_true_range: eager_di!("price_true_range", v0),
            dateindex_to_price_true_range_2w_sum: eager_di!("price_true_range_2w_sum", v0),
            indexes_to_price_1w_min: computed_di!("price_1w_min", v1),
            indexes_to_price_1w_max: computed_di!("price_1w_max", v1),
            indexes_to_price_2w_min: computed_di!("price_2w_min", v1),
            indexes_to_price_2w_max: computed_di!("price_2w_max", v1),
            indexes_to_price_1m_min: computed_di!("price_1m_min", v1),
            indexes_to_price_1m_max: computed_di!("price_1m_max", v1),
            indexes_to_price_1y_min: computed_di!("price_1y_min", v1),
            indexes_to_price_1y_max: computed_di!("price_1y_max", v1),
            indexes_to_price_2w_choppiness_index: computed_di!("price_2w_choppiness_index", v1),
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
}
