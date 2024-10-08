use allocative::Allocative;

use crate::{
    datasets::InsertData,
    structs::{AnyBiMap, BiMap, HeightMap},
    utils::{ONE_DAY_IN_S, ONE_MONTH_IN_DAYS, ONE_WEEK_IN_DAYS, ONE_YEAR_IN_DAYS},
};

use super::{AnyDataset, ComputeData, MinInitialStates};

#[derive(Allocative)]
pub struct TransactionDataset {
    min_initial_states: MinInitialStates,

    // Inserted
    pub count: BiMap<usize>,
    pub volume: BiMap<f64>,
    pub volume_in_dollars: BiMap<f32>,
    // Average sent
    // Average sent in dollars
    // Median sent
    // Median sent in dollars
    // Min
    // Max
    // 10th 25th 75th 90th percentiles
    // type
    // version

    // Computed
    pub count_1w_sma: BiMap<f32>,
    pub count_1m_sma: BiMap<f32>,
    pub volume_1w_sma: BiMap<f32>,
    pub volume_1m_sma: BiMap<f32>,
    pub volume_in_dollars_1w_sma: BiMap<f32>,
    pub volume_in_dollars_1m_sma: BiMap<f32>,
    pub annualized_volume: BiMap<f32>,
    pub annualized_volume_in_dollars: BiMap<f32>,
    pub velocity: BiMap<f32>,
    pub transactions_per_second: BiMap<f32>,
    pub transactions_per_second_1w_sma: BiMap<f32>,
    pub transactions_per_second_1m_sma: BiMap<f32>,
}

impl TransactionDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            count: BiMap::new_bin(1, &f("transaction_count")),
            count_1w_sma: BiMap::new_bin(1, &f("transaction_count_1w_sma")),
            count_1m_sma: BiMap::new_bin(1, &f("transaction_count_1m_sma")),
            volume: BiMap::new_bin(1, &f("transaction_volume")),
            volume_1w_sma: BiMap::new_bin(1, &f("transaction_volume_1w_sma")),
            volume_1m_sma: BiMap::new_bin(1, &f("transaction_volume_1m_sma")),
            volume_in_dollars: BiMap::new_bin(1, &f("transaction_volume_in_dollars")),
            volume_in_dollars_1w_sma: BiMap::new_bin(1, &f("transaction_volume_in_dollars_1w_sma")),
            volume_in_dollars_1m_sma: BiMap::new_bin(1, &f("transaction_volume_in_dollars_1m_sma")),
            annualized_volume: BiMap::new_bin(1, &f("annualized_transaction_volume")),
            annualized_volume_in_dollars: BiMap::new_bin(
                2,
                &f("annualized_transaction_volume_in_dollars"),
            ),
            velocity: BiMap::new_bin(1, &f("transaction_velocity")),
            transactions_per_second: BiMap::new_bin(1, &f("transactions_per_second")),
            transactions_per_second_1w_sma: BiMap::new_bin(1, &f("transactions_per_second_1w_sma")),
            transactions_per_second_1m_sma: BiMap::new_bin(1, &f("transactions_per_second_1m_sma")),
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s));

        Ok(s)
    }

    pub fn insert(
        &mut self,
        &InsertData {
            height,
            date,
            amount_sent,
            transaction_count,
            is_date_last_block,
            date_blocks_range,
            block_price,
            ..
        }: &InsertData,
    ) {
        self.count.height.insert(height, transaction_count);

        self.volume.height.insert(height, amount_sent.to_btc());

        self.volume_in_dollars
            .height
            .insert(height, (block_price * amount_sent).to_dollar() as f32);

        if is_date_last_block {
            self.count.date_insert_sum_range(date, date_blocks_range);

            self.volume.date_insert_sum_range(date, date_blocks_range);

            self.volume_in_dollars
                .date_insert_sum_range(date, date_blocks_range);
        }
    }

    pub fn compute(
        &mut self,
        &ComputeData { heights, dates, .. }: &ComputeData,
        circulating_supply: &mut BiMap<f64>,
        block_interval: &mut HeightMap<u32>,
    ) {
        self.count_1w_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.count,
            ONE_WEEK_IN_DAYS,
        );

        self.count_1m_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.count,
            ONE_MONTH_IN_DAYS,
        );

        self.volume_1w_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.volume,
            ONE_WEEK_IN_DAYS,
        );

        self.volume_1m_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.volume,
            ONE_MONTH_IN_DAYS,
        );

        self.volume_in_dollars_1w_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.volume_in_dollars,
            ONE_WEEK_IN_DAYS,
        );

        self.volume_in_dollars_1m_sma.multi_insert_simple_average(
            heights,
            dates,
            &mut self.volume_in_dollars,
            ONE_MONTH_IN_DAYS,
        );

        self.annualized_volume.multi_insert_last_x_sum(
            heights,
            dates,
            &mut self.volume,
            ONE_YEAR_IN_DAYS,
        );

        self.annualized_volume_in_dollars.multi_insert_last_x_sum(
            heights,
            dates,
            &mut self.volume_in_dollars,
            ONE_YEAR_IN_DAYS,
        );

        self.velocity.multi_insert_divide(
            heights,
            dates,
            &mut self.annualized_volume,
            circulating_supply,
        );

        self.transactions_per_second.height.multi_insert_divide(
            heights,
            &mut self.count.height,
            block_interval,
        );

        self.transactions_per_second
            .date
            .multi_insert_simple_transform(dates, &mut self.count.date, |count, _| {
                count as f32 / ONE_DAY_IN_S as f32
            });

        self.transactions_per_second_1w_sma
            .multi_insert_simple_average(
                heights,
                dates,
                &mut self.transactions_per_second,
                ONE_WEEK_IN_DAYS,
            );

        self.transactions_per_second_1m_sma
            .multi_insert_simple_average(
                heights,
                dates,
                &mut self.transactions_per_second,
                ONE_MONTH_IN_DAYS,
            );
    }
}

impl AnyDataset for TransactionDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }

    fn to_inserted_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![&self.count, &self.volume, &self.volume_in_dollars]
    }

    fn to_inserted_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![
            &mut self.count,
            &mut self.volume,
            &mut self.volume_in_dollars,
        ]
    }

    fn to_computed_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![
            &self.count_1w_sma,
            &self.count_1m_sma,
            &self.volume_1w_sma,
            &self.volume_1m_sma,
            &self.volume_in_dollars_1w_sma,
            &self.volume_in_dollars_1m_sma,
            &self.annualized_volume,
            &self.annualized_volume_in_dollars,
            &self.velocity,
            &self.transactions_per_second,
            &self.transactions_per_second_1w_sma,
            &self.transactions_per_second_1m_sma,
        ]
    }

    fn to_computed_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![
            &mut self.count_1w_sma,
            &mut self.count_1m_sma,
            &mut self.volume_1w_sma,
            &mut self.volume_1m_sma,
            &mut self.volume_in_dollars_1w_sma,
            &mut self.volume_in_dollars_1m_sma,
            &mut self.annualized_volume,
            &mut self.annualized_volume_in_dollars,
            &mut self.velocity,
            &mut self.transactions_per_second,
            &mut self.transactions_per_second_1w_sma,
            &mut self.transactions_per_second_1m_sma,
        ]
    }
}
