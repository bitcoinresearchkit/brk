use allocative::Allocative;

use crate::{
    datasets::{AnyDataset, ComputeData, MinInitialStates},
    structs::{AnyBiMap, BiMap},
    utils::{ONE_MONTH_IN_DAYS, ONE_WEEK_IN_DAYS, ONE_YEAR_IN_DAYS},
};

#[derive(Default, Allocative)]
pub struct RatioDataset {
    min_initial_states: MinInitialStates,

    // Computed
    ratio: BiMap<f32>,
    ratio_1w_sma: BiMap<f32>,
    ratio_1m_sma: BiMap<f32>,
    ratio_1y_sma: BiMap<f32>,
    ratio_1y_sma_momentum_oscillator: BiMap<f32>,
    ratio_99p: BiMap<f32>,
    ratio_99_5p: BiMap<f32>,
    ratio_99_9p: BiMap<f32>,
    ratio_1p: BiMap<f32>,
    ratio_0_5p: BiMap<f32>,
    ratio_0_1p: BiMap<f32>,
    price_99p: BiMap<f32>,
    price_99_5p: BiMap<f32>,
    price_99_9p: BiMap<f32>,
    price_1p: BiMap<f32>,
    price_0_5p: BiMap<f32>,
    price_0_1p: BiMap<f32>,
}

impl RatioDataset {
    pub fn import(parent_path: &str, name: &str) -> color_eyre::Result<Self> {
        let f_ratio = |s: &str| format!("{parent_path}/market_price_to_{name}_{s}");
        let f_price = |s: &str| format!("{parent_path}/{name}_{s}");

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            ratio: BiMap::new_bin(1, &f_ratio("ratio")),
            ratio_1w_sma: BiMap::new_bin(1, &f_ratio("ratio_1w_sma")),
            ratio_1m_sma: BiMap::new_bin(1, &f_ratio("ratio_1m_sma")),
            ratio_1y_sma: BiMap::new_bin(1, &f_ratio("ratio_1y_sma")),
            ratio_1y_sma_momentum_oscillator: BiMap::new_bin(
                1,
                &f_ratio("ratio_1y_sma_momentum_oscillator"),
            ),
            ratio_99p: BiMap::new_bin(1, &f_ratio("ratio_99p")),
            ratio_99_5p: BiMap::new_bin(1, &f_ratio("ratio_99_5p")),
            ratio_99_9p: BiMap::new_bin(1, &f_ratio("ratio_99_9p")),
            ratio_1p: BiMap::new_bin(1, &f_ratio("ratio_1p")),
            ratio_0_5p: BiMap::new_bin(1, &f_ratio("ratio_0_5p")),
            ratio_0_1p: BiMap::new_bin(1, &f_ratio("ratio_0_1p")),
            price_99p: BiMap::new_bin(1, &f_price("99p")),
            price_99_5p: BiMap::new_bin(1, &f_price("99_5p")),
            price_99_9p: BiMap::new_bin(1, &f_price("99_9p")),
            price_1p: BiMap::new_bin(1, &f_price("1p")),
            price_0_5p: BiMap::new_bin(1, &f_price("0_5p")),
            price_0_1p: BiMap::new_bin(1, &f_price("0_1p")),
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn compute(
        &mut self,
        &ComputeData { heights, dates }: &ComputeData,
        market_price: &mut BiMap<f32>,
        other_price: &mut BiMap<f32>,
    ) {
        self.ratio.height.multi_insert_divide(
            heights,
            &mut market_price.height,
            &mut other_price.height,
        );

        self.ratio
            .date
            .multi_insert_divide(dates, &mut market_price.date, &mut other_price.date);

        self.ratio_1w_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.ratio,
            ONE_WEEK_IN_DAYS,
        );

        self.ratio_1m_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.ratio,
            ONE_MONTH_IN_DAYS,
        );

        self.ratio_1m_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.ratio,
            ONE_MONTH_IN_DAYS,
        );

        self.ratio_1y_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.ratio,
            ONE_YEAR_IN_DAYS,
        );

        self.ratio_1y_sma_momentum_oscillator
            .height
            .multi_insert_complex_transform(heights, &mut self.ratio.height, |(ratio, height)| {
                (ratio / self.ratio_1y_sma.height.get_or_import(height)) - 1.0
            });

        self.ratio_1y_sma_momentum_oscillator
            .date
            .multi_insert_complex_transform(dates, &mut self.ratio.date, |(ratio, date, _)| {
                (ratio / self.ratio_1y_sma.date.get_or_import(date).unwrap()) - 1.0
            });

        self.ratio.multi_insert_percentile(
            heights,
            dates,
            vec![
                (&mut self.ratio_99p, 0.99),
                (&mut self.ratio_99_5p, 0.995),
                (&mut self.ratio_99_9p, 0.999),
                (&mut self.ratio_1p, 0.1),
                (&mut self.ratio_0_5p, 0.005),
                (&mut self.ratio_0_1p, 0.001),
            ],
            None,
        );

        self.price_99p
            .multi_insert_multiply(heights, dates, market_price, &mut self.ratio_99p);

        self.price_99_5p
            .multi_insert_multiply(heights, dates, market_price, &mut self.ratio_99_5p);

        self.price_99_9p
            .multi_insert_multiply(heights, dates, market_price, &mut self.ratio_99_9p);

        self.price_1p
            .multi_insert_multiply(heights, dates, market_price, &mut self.ratio_1p);

        self.price_0_5p
            .multi_insert_multiply(heights, dates, market_price, &mut self.ratio_0_5p);

        self.price_0_1p
            .multi_insert_multiply(heights, dates, market_price, &mut self.ratio_0_1p);
    }
}

impl AnyDataset for RatioDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }

    fn to_computed_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![
            &self.ratio,
            &self.ratio_1w_sma,
            &self.ratio_1m_sma,
            &self.ratio_1y_sma,
            &self.ratio_1y_sma_momentum_oscillator,
            &self.ratio_99p,
            &self.ratio_99_5p,
            &self.ratio_99_9p,
            &self.ratio_1p,
            &self.ratio_0_5p,
            &self.ratio_0_1p,
            &self.price_99p,
            &self.price_99_5p,
            &self.price_99_9p,
            &self.price_1p,
            &self.price_0_5p,
            &self.price_0_1p,
        ]
    }

    fn to_computed_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![
            &mut self.ratio,
            &mut self.ratio_1w_sma,
            &mut self.ratio_1m_sma,
            &mut self.ratio_1y_sma,
            &mut self.ratio_1y_sma_momentum_oscillator,
            &mut self.ratio_99p,
            &mut self.ratio_99_5p,
            &mut self.ratio_99_9p,
            &mut self.ratio_1p,
            &mut self.ratio_0_5p,
            &mut self.ratio_0_1p,
            &mut self.price_99p,
            &mut self.price_99_5p,
            &mut self.price_99_9p,
            &mut self.price_1p,
            &mut self.price_0_5p,
            &mut self.price_0_1p,
        ]
    }
}
