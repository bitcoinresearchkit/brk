use allocative::Allocative;

use crate::{
    datasets::InsertData,
    structs::{AnyBiMap, AnyDateMap, AnyHeightMap, BiMap, HeightMap},
    utils::{
        ONE_DAY_IN_S, ONE_MONTH_IN_DAYS, ONE_WEEK_IN_DAYS, ONE_YEAR_IN_DAYS, TARGET_BLOCKS_PER_DAY,
    },
    DateMap,
};

use super::{AnyDataset, ComputeData, MinInitialStates};

#[derive(Allocative)]
pub struct TransactionDataset {
    min_initial_states: MinInitialStates,

    // Inserted
    pub count: HeightMap<usize>,
    pub count_1d_sum: DateMap<usize>,
    pub volume: HeightMap<f64>,
    pub volume_1d_sum: DateMap<f64>,
    pub volume_in_dollars: HeightMap<f32>,
    pub volume_in_dollars_1d_sum: DateMap<f32>,
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
    pub count_1w_sma: HeightMap<f32>,
    pub count_1d_sum_1w_sma: DateMap<f32>,
    pub count_1m_sma: HeightMap<f32>,
    pub count_1d_sum_1m_sma: DateMap<f32>,
    pub volume_1w_sma: HeightMap<f32>,
    pub volume_1d_sum_1w_sma: DateMap<f32>,
    pub volume_1m_sma: HeightMap<f32>,
    pub volume_1d_sum_1m_sma: DateMap<f32>,
    pub volume_in_dollars_1w_sma: HeightMap<f32>,
    pub volume_in_dollars_1d_sum_1w_sma: DateMap<f32>,
    pub volume_in_dollars_1m_sma: HeightMap<f32>,
    pub volume_in_dollars_1d_sum_1m_sma: DateMap<f32>,
    pub annualized_volume: DateMap<f32>,
    pub annualized_volume_in_dollars: DateMap<f32>,
    pub velocity: DateMap<f32>,
    pub transactions_per_second: BiMap<f32>,
    pub transactions_per_second_1w_sma: BiMap<f32>,
    pub transactions_per_second_1m_sma: BiMap<f32>,
}

impl TransactionDataset {
    pub fn import(parent_path: &str) -> color_eyre::Result<Self> {
        let f = |s: &str| format!("{parent_path}/{s}");

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            count: HeightMap::new_bin(1, &f("transaction_count")),
            count_1d_sum: DateMap::new_bin(1, &f("transaction_count_1d_sum")),
            count_1w_sma: HeightMap::new_bin(1, &f("transaction_count_1w_sma")),
            count_1d_sum_1w_sma: DateMap::new_bin(1, &f("transaction_count_1d_sum_1w_sma")),
            count_1m_sma: HeightMap::new_bin(1, &f("transaction_count_1m_sma")),
            count_1d_sum_1m_sma: DateMap::new_bin(1, &f("transaction_count_1d_sum_1m_sma")),
            volume: HeightMap::new_bin(1, &f("transaction_volume")),
            volume_1d_sum: DateMap::new_bin(1, &f("transaction_volume_1d_sum")),
            volume_1w_sma: HeightMap::new_bin(1, &f("transaction_volume_1w_sma")),
            volume_1d_sum_1w_sma: DateMap::new_bin(1, &f("transaction_volume_1d_sum_1w_sma")),
            volume_1m_sma: HeightMap::new_bin(1, &f("transaction_volume_1m_sma")),
            volume_1d_sum_1m_sma: DateMap::new_bin(1, &f("transaction_volume_1d_sum_1m_sma")),
            volume_in_dollars: HeightMap::new_bin(1, &f("transaction_volume_in_dollars")),
            volume_in_dollars_1d_sum: DateMap::new_bin(
                1,
                &f("transaction_volume_in_dollars_1d_sum"),
            ),
            volume_in_dollars_1w_sma: HeightMap::new_bin(
                1,
                &f("transaction_volume_in_dollars_1w_sma"),
            ),
            volume_in_dollars_1d_sum_1w_sma: DateMap::new_bin(
                1,
                &f("transaction_volume_in_dollars_1d_sum_1w_sma"),
            ),
            volume_in_dollars_1m_sma: HeightMap::new_bin(
                1,
                &f("transaction_volume_in_dollars_1m_sma"),
            ),
            volume_in_dollars_1d_sum_1m_sma: DateMap::new_bin(
                1,
                &f("transaction_volume_in_dollars_1d_sum_1m_sma"),
            ),
            annualized_volume: DateMap::new_bin(1, &f("annualized_transaction_volume")),
            annualized_volume_in_dollars: DateMap::new_bin(
                2,
                &f("annualized_transaction_volume_in_dollars"),
            ),
            velocity: DateMap::new_bin(1, &f("transaction_velocity")),
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
        self.count.insert(height, transaction_count);

        self.volume.insert(height, amount_sent.to_btc());

        self.volume_in_dollars
            .insert(height, (block_price * amount_sent).to_dollar() as f32);

        if is_date_last_block {
            self.count_1d_sum
                .insert(date, self.count.sum_range(date_blocks_range));

            self.volume_1d_sum
                .insert(date, self.volume.sum_range(date_blocks_range));

            self.volume_in_dollars_1d_sum
                .insert(date, self.volume_in_dollars.sum_range(date_blocks_range));
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
            &mut self.count,
            TARGET_BLOCKS_PER_DAY * ONE_WEEK_IN_DAYS,
        );
        self.count_1d_sum_1w_sma.multi_insert_simple_average(
            dates,
            &mut self.count_1d_sum,
            ONE_WEEK_IN_DAYS,
        );

        self.count_1m_sma.multi_insert_simple_average(
            heights,
            &mut self.count,
            TARGET_BLOCKS_PER_DAY * ONE_MONTH_IN_DAYS,
        );
        self.count_1d_sum_1m_sma.multi_insert_simple_average(
            dates,
            &mut self.count_1d_sum,
            ONE_MONTH_IN_DAYS,
        );

        self.volume_1w_sma.multi_insert_simple_average(
            heights,
            &mut self.volume,
            TARGET_BLOCKS_PER_DAY * ONE_WEEK_IN_DAYS,
        );
        self.volume_1d_sum_1w_sma.multi_insert_simple_average(
            dates,
            &mut self.volume_1d_sum,
            ONE_WEEK_IN_DAYS,
        );

        self.volume_1m_sma.multi_insert_simple_average(
            heights,
            &mut self.volume,
            TARGET_BLOCKS_PER_DAY * ONE_MONTH_IN_DAYS,
        );
        self.volume_1d_sum_1m_sma.multi_insert_simple_average(
            dates,
            &mut self.volume_1d_sum,
            ONE_MONTH_IN_DAYS,
        );

        self.volume_in_dollars_1w_sma.multi_insert_simple_average(
            heights,
            &mut self.volume_in_dollars,
            TARGET_BLOCKS_PER_DAY * ONE_WEEK_IN_DAYS,
        );
        self.volume_in_dollars_1d_sum_1w_sma
            .multi_insert_simple_average(
                dates,
                &mut self.volume_in_dollars_1d_sum,
                ONE_WEEK_IN_DAYS,
            );

        self.volume_in_dollars_1m_sma.multi_insert_simple_average(
            heights,
            &mut self.volume_in_dollars,
            TARGET_BLOCKS_PER_DAY * ONE_MONTH_IN_DAYS,
        );
        self.volume_in_dollars_1d_sum_1m_sma
            .multi_insert_simple_average(
                dates,
                &mut self.volume_in_dollars_1d_sum,
                ONE_MONTH_IN_DAYS,
            );

        self.annualized_volume.multi_insert_last_x_sum(
            dates,
            &mut self.volume_1d_sum,
            ONE_YEAR_IN_DAYS,
        );

        self.annualized_volume_in_dollars.multi_insert_last_x_sum(
            dates,
            &mut self.volume_in_dollars_1d_sum,
            ONE_YEAR_IN_DAYS,
        );

        self.velocity.multi_insert_divide(
            dates,
            &mut self.annualized_volume,
            &mut circulating_supply.date,
        );

        self.transactions_per_second.height.multi_insert_divide(
            heights,
            &mut self.count,
            block_interval,
        );

        self.transactions_per_second
            .date
            .multi_insert_simple_transform(dates, &mut self.count_1d_sum, |count, date| {
                count as f32 / (date.get_day_completion() as f32 * ONE_DAY_IN_S as f32)
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

    fn to_inserted_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![&self.count, &self.volume, &self.volume_in_dollars]
    }

    fn to_inserted_mut_height_map_vec(&mut self) -> Vec<&mut dyn AnyHeightMap> {
        vec![
            &mut self.count,
            &mut self.volume,
            &mut self.volume_in_dollars,
        ]
    }

    fn to_inserted_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![
            &self.count_1d_sum,
            &self.volume_1d_sum,
            &self.volume_in_dollars_1d_sum,
        ]
    }

    fn to_inserted_mut_date_map_vec(&mut self) -> Vec<&mut dyn AnyDateMap> {
        vec![
            &mut self.count_1d_sum,
            &mut self.volume_1d_sum,
            &mut self.volume_in_dollars_1d_sum,
        ]
    }

    fn to_computed_bi_map_vec(&self) -> Vec<&(dyn AnyBiMap + Send + Sync)> {
        vec![
            &self.transactions_per_second,
            &self.transactions_per_second_1w_sma,
            &self.transactions_per_second_1m_sma,
        ]
    }

    fn to_computed_mut_bi_map_vec(&mut self) -> Vec<&mut dyn AnyBiMap> {
        vec![
            &mut self.transactions_per_second,
            &mut self.transactions_per_second_1w_sma,
            &mut self.transactions_per_second_1m_sma,
        ]
    }

    fn to_computed_height_map_vec(&self) -> Vec<&(dyn AnyHeightMap + Send + Sync)> {
        vec![
            &self.count_1w_sma,
            &self.count_1m_sma,
            &self.volume_1w_sma,
            &self.volume_1m_sma,
            &self.volume_in_dollars_1w_sma,
            &self.volume_in_dollars_1m_sma,
        ]
    }

    fn to_computed_mut_height_map_vec(&mut self) -> Vec<&mut dyn AnyHeightMap> {
        vec![
            &mut self.count_1w_sma,
            &mut self.count_1m_sma,
            &mut self.volume_1w_sma,
            &mut self.volume_1m_sma,
            &mut self.volume_in_dollars_1w_sma,
            &mut self.volume_in_dollars_1m_sma,
        ]
    }

    fn to_computed_date_map_vec(&self) -> Vec<&(dyn AnyDateMap + Send + Sync)> {
        vec![
            &self.count_1d_sum_1w_sma,
            &self.count_1d_sum_1m_sma,
            &self.volume_1d_sum_1w_sma,
            &self.volume_1d_sum_1m_sma,
            &self.volume_in_dollars_1d_sum_1w_sma,
            &self.volume_in_dollars_1d_sum_1m_sma,
            &self.annualized_volume,
            &self.annualized_volume_in_dollars,
            &self.velocity,
        ]
    }

    fn to_computed_mut_date_map_vec(&mut self) -> Vec<&mut dyn AnyDateMap> {
        vec![
            &mut self.count_1d_sum_1w_sma,
            &mut self.count_1d_sum_1m_sma,
            &mut self.volume_1d_sum_1w_sma,
            &mut self.volume_1d_sum_1m_sma,
            &mut self.volume_in_dollars_1d_sum_1w_sma,
            &mut self.volume_in_dollars_1d_sum_1m_sma,
            &mut self.annualized_volume,
            &mut self.annualized_volume_in_dollars,
            &mut self.velocity,
        ]
    }
}
