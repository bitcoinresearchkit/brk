use allocative::Allocative;
use struct_iterable::Iterable;

use crate::{
    structs::{BiMap, Config, DateMap, Height, HeightMap, MapKind},
    utils::{ONE_DAY_IN_DAYS, ONE_YEAR_IN_DAYS, THREE_MONTHS_IN_DAYS, TWO_WEEK_IN_DAYS},
};

use super::{AnyDataset, ComputeData, InsertData, MinInitialStates, RatioDataset};

#[derive(Allocative, Iterable)]
pub struct CointimeDataset {
    min_initial_states: MinInitialStates,

    // Inserted
    pub coinblocks_destroyed: HeightMap<f32>,
    pub coinblocks_destroyed_1d_sum: DateMap<f32>,

    // Computed
    pub active_cap: BiMap<f32>,
    pub active_price: BiMap<f32>,
    pub active_price_ratio: RatioDataset,
    pub active_supply: BiMap<f32>,
    pub active_supply_3m_net_change: BiMap<f32>,
    pub active_supply_net_change: BiMap<f32>,
    pub activity_to_vaultedness_ratio: BiMap<f32>,
    pub coinblocks_created: HeightMap<f32>,
    pub coinblocks_created_1d_sum: DateMap<f32>,
    pub coinblocks_stored: HeightMap<f32>,
    pub coinblocks_stored_1d_sum: DateMap<f32>,
    pub cointime_adjusted_velocity: DateMap<f32>,
    pub cointime_adjusted_inflation_rate: DateMap<f32>,
    pub cointime_adjusted_yearly_inflation_rate: DateMap<f32>,
    pub cointime_cap: BiMap<f32>,
    pub cointime_price: BiMap<f32>,
    pub cointime_price_ratio: RatioDataset,
    pub cointime_value_created: HeightMap<f32>,
    pub cointime_value_created_1d_sum: DateMap<f32>,
    pub cointime_value_destroyed: HeightMap<f32>,
    pub cointime_value_destroyed_1d_sum: DateMap<f32>,
    pub cointime_value_stored: HeightMap<f32>,
    pub cointime_value_stored_1d_sum: DateMap<f32>,
    pub concurrent_liveliness: DateMap<f32>,
    pub concurrent_liveliness_2w_median: DateMap<f32>,
    pub cumulative_coinblocks_created: BiMap<f32>,
    pub cumulative_coinblocks_destroyed: BiMap<f32>,
    pub cumulative_coinblocks_stored: BiMap<f32>,
    pub investor_cap: BiMap<f32>,
    pub investorness: BiMap<f32>,
    pub liveliness: BiMap<f32>,
    pub liveliness_net_change: BiMap<f32>,
    pub liveliness_net_change_2w_median: BiMap<f32>,
    pub producerness: BiMap<f32>,
    pub thermo_cap: BiMap<f32>,
    pub thermo_cap_to_investor_cap_ratio: BiMap<f32>,
    pub total_cointime_value_created: BiMap<f32>,
    pub total_cointime_value_destroyed: BiMap<f32>,
    pub total_cointime_value_stored: BiMap<f32>,
    pub true_market_deviation: BiMap<f32>,
    pub true_market_mean: BiMap<f32>,
    pub true_market_mean_ratio: RatioDataset,
    pub true_market_net_unrealized_profit_and_loss: BiMap<f32>,
    pub vaulted_cap: BiMap<f32>,
    pub vaulted_price: BiMap<f32>,
    pub vaulted_price_ratio: RatioDataset,
    pub vaulted_supply: BiMap<f32>,
    pub vaulted_supply_net_change: BiMap<f32>,
    pub vaulted_supply_3m_net_change: BiMap<f32>,
    pub vaultedness: BiMap<f32>,
    pub vaulting_rate: BiMap<f32>,
}

impl CointimeDataset {
    pub fn import(config: &Config) -> color_eyre::Result<Self> {
        let path_dataset = config.path_datasets();
        let f = |s: &str| path_dataset.join(s);

        let mut s = Self {
            min_initial_states: MinInitialStates::default(),

            // Inserted
            coinblocks_destroyed: HeightMap::new_bin(
                1,
                MapKind::Inserted,
                &f("coinblocks_destroyed"),
            ),
            coinblocks_destroyed_1d_sum: DateMap::new_bin(
                1,
                MapKind::Inserted,
                &f("coinblocks_destroyed_1d_sum"),
            ),

            // Computed
            active_cap: BiMap::new_bin(1, MapKind::Computed, &f("active_cap")),
            active_price: BiMap::new_bin(1, MapKind::Computed, &f("active_price")),
            active_price_ratio: RatioDataset::import(&path_dataset, "active_price", config)?,
            active_supply: BiMap::new_bin(1, MapKind::Computed, &f("active_supply")),
            active_supply_3m_net_change: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("active_supply_3m_net_change"),
            ),
            active_supply_net_change: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("active_supply_net_change"),
            ),
            activity_to_vaultedness_ratio: BiMap::new_bin(
                2,
                MapKind::Computed,
                &f("activity_to_vaultedness_ratio"),
            ),
            coinblocks_created: HeightMap::new_bin(1, MapKind::Computed, &f("coinblocks_created")),
            coinblocks_created_1d_sum: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("coinblocks_created_1d_sum"),
            ),

            coinblocks_stored: HeightMap::new_bin(1, MapKind::Computed, &f("coinblocks_stored")),
            coinblocks_stored_1d_sum: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("coinblocks_stored_1d_sum"),
            ),
            cointime_adjusted_velocity: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("cointime_adjusted_velocity"),
            ),
            cointime_adjusted_inflation_rate: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("cointime_adjusted_inflation_rate"),
            ),
            cointime_adjusted_yearly_inflation_rate: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("cointime_adjusted_yearly_inflation_rate"),
            ),
            cointime_cap: BiMap::new_bin(1, MapKind::Computed, &f("cointime_cap")),
            cointime_price: BiMap::new_bin(1, MapKind::Computed, &f("cointime_price")),
            cointime_price_ratio: RatioDataset::import(&path_dataset, "cointime_price", config)?,
            cointime_value_created: HeightMap::new_bin(
                1,
                MapKind::Computed,
                &f("cointime_value_created"),
            ),
            cointime_value_created_1d_sum: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("cointime_value_created_1d_sum"),
            ),
            cointime_value_destroyed: HeightMap::new_bin(
                1,
                MapKind::Computed,
                &f("cointime_value_destroyed"),
            ),
            cointime_value_destroyed_1d_sum: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("cointime_value_destroyed_1d_sum"),
            ),
            cointime_value_stored: HeightMap::new_bin(
                1,
                MapKind::Computed,
                &f("cointime_value_stored"),
            ),
            cointime_value_stored_1d_sum: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("cointime_value_stored_1d_sum"),
            ),
            concurrent_liveliness: DateMap::new_bin(
                1,
                MapKind::Computed,
                &f("concurrent_liveliness"),
            ),
            concurrent_liveliness_2w_median: DateMap::new_bin(
                2,
                MapKind::Computed,
                &f("concurrent_liveliness_2w_median"),
            ),
            cumulative_coinblocks_created: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("cumulative_coinblocks_created"),
            ),
            cumulative_coinblocks_destroyed: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("cumulative_coinblocks_destroyed"),
            ),
            cumulative_coinblocks_stored: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("cumulative_coinblocks_stored"),
            ),
            investor_cap: BiMap::new_bin(1, MapKind::Computed, &f("investor_cap")),
            investorness: BiMap::new_bin(1, MapKind::Computed, &f("investorness")),
            liveliness: BiMap::new_bin(1, MapKind::Computed, &f("liveliness")),
            liveliness_net_change: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("liveliness_net_change"),
            ),
            liveliness_net_change_2w_median: BiMap::new_bin(
                3,
                MapKind::Computed,
                &f("liveliness_net_change_2w_median"),
            ),
            producerness: BiMap::new_bin(1, MapKind::Computed, &f("producerness")),
            thermo_cap: BiMap::new_bin(1, MapKind::Computed, &f("thermo_cap")),
            thermo_cap_to_investor_cap_ratio: BiMap::new_bin(
                2,
                MapKind::Computed,
                &f("thermo_cap_to_investor_cap_ratio"),
            ),
            total_cointime_value_created: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("total_cointime_value_created"),
            ),
            total_cointime_value_destroyed: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("total_cointime_value_destroyed"),
            ),
            total_cointime_value_stored: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("total_cointime_value_stored"),
            ),
            true_market_deviation: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("true_market_deviation"),
            ),
            true_market_mean: BiMap::new_bin(1, MapKind::Computed, &f("true_market_mean")),
            true_market_mean_ratio: RatioDataset::import(
                &path_dataset,
                "true_market_mean",
                config,
            )?,
            true_market_net_unrealized_profit_and_loss: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("true_market_net_unrealized_profit_and_loss"),
            ),
            vaulted_cap: BiMap::new_bin(1, MapKind::Computed, &f("vaulted_cap")),
            vaulted_price: BiMap::new_bin(1, MapKind::Computed, &f("vaulted_price")),
            vaulted_price_ratio: RatioDataset::import(&path_dataset, "vaulted_price", config)?,
            vaulted_supply: BiMap::new_bin(1, MapKind::Computed, &f("vaulted_supply")),
            vaulted_supply_3m_net_change: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("vaulted_supply_3m_net_change"),
            ),
            vaulted_supply_net_change: BiMap::new_bin(
                1,
                MapKind::Computed,
                &f("vaulted_supply_net_change"),
            ),
            vaultedness: BiMap::new_bin(1, MapKind::Computed, &f("vaultedness")),
            vaulting_rate: BiMap::new_bin(1, MapKind::Computed, &f("vaulting_rate")),
        };

        s.min_initial_states
            .consume(MinInitialStates::compute_from_dataset(&s, config));

        Ok(s)
    }

    pub fn insert(
        &mut self,
        &InsertData {
            height,
            date,
            satblocks_destroyed,
            date_blocks_range,
            is_date_last_block,
            ..
        }: &InsertData,
    ) {
        self.coinblocks_destroyed
            .insert(height, satblocks_destroyed.to_btc() as f32);

        if is_date_last_block {
            self.coinblocks_destroyed_1d_sum
                .insert(date, self.coinblocks_destroyed.sum_range(date_blocks_range));
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        compute_data: &ComputeData,
        first_height: &mut DateMap<Height>,
        last_height: &mut DateMap<Height>,
        closes: &mut BiMap<f32>,
        circulating_supply: &mut BiMap<f64>,
        realized_cap: &mut BiMap<f32>,
        realized_price: &mut BiMap<f32>,
        inflation_rate: &mut DateMap<f64>,
        yearly_inflation_rate: &mut DateMap<f64>,
        annualized_transaction_volume: &mut DateMap<f32>,
        cumulative_subsidy_in_dollars: &mut BiMap<f32>,
    ) {
        let &ComputeData { heights, dates, .. } = compute_data;

        self.cumulative_coinblocks_destroyed
            .height
            .multi_insert_cumulative(heights, &mut self.coinblocks_destroyed);
        self.cumulative_coinblocks_destroyed
            .date
            .multi_insert_cumulative(dates, &mut self.coinblocks_destroyed_1d_sum);

        self.coinblocks_created.multi_insert_simple_transform(
            heights,
            &mut circulating_supply.height,
            |circulating_supply, _| circulating_supply as f32,
        );
        self.coinblocks_created_1d_sum.multi_insert_sum_range(
            dates,
            &self.coinblocks_created,
            first_height,
            last_height,
        );

        self.cumulative_coinblocks_created
            .height
            .multi_insert_cumulative(heights, &mut self.coinblocks_created);
        self.cumulative_coinblocks_created
            .date
            .multi_insert_cumulative(dates, &mut self.coinblocks_created_1d_sum);

        self.coinblocks_stored.multi_insert_subtract(
            heights,
            &mut self.coinblocks_created,
            &mut self.coinblocks_destroyed,
        );
        self.coinblocks_stored_1d_sum.multi_insert_sum_range(
            dates,
            &self.coinblocks_stored,
            first_height,
            last_height,
        );

        self.cumulative_coinblocks_stored
            .height
            .multi_insert_cumulative(heights, &mut self.coinblocks_stored);
        self.cumulative_coinblocks_stored
            .date
            .multi_insert_cumulative(dates, &mut self.coinblocks_stored_1d_sum);

        self.liveliness.multi_insert_divide(
            heights,
            dates,
            &mut self.cumulative_coinblocks_destroyed,
            &mut self.cumulative_coinblocks_created,
        );

        self.vaultedness.multi_insert_simple_transform(
            heights,
            dates,
            &mut self.liveliness,
            &|liveliness| 1.0 - liveliness,
        );

        self.activity_to_vaultedness_ratio.multi_insert_divide(
            heights,
            dates,
            &mut self.liveliness,
            &mut self.vaultedness,
        );

        self.concurrent_liveliness.multi_insert_divide(
            dates,
            &mut self.coinblocks_destroyed_1d_sum,
            &mut self.coinblocks_created_1d_sum,
        );

        self.concurrent_liveliness_2w_median.multi_insert_median(
            dates,
            &mut self.concurrent_liveliness,
            Some(TWO_WEEK_IN_DAYS),
        );

        self.liveliness_net_change.multi_insert_net_change(
            heights,
            dates,
            &mut self.liveliness,
            ONE_DAY_IN_DAYS,
        );

        self.liveliness_net_change_2w_median
            .multi_insert_net_change(heights, dates, &mut self.liveliness, TWO_WEEK_IN_DAYS);

        self.vaulted_supply.multi_insert_multiply(
            heights,
            dates,
            &mut self.vaultedness,
            circulating_supply,
        );

        self.vaulted_supply_net_change.multi_insert_net_change(
            heights,
            dates,
            &mut self.vaulted_supply,
            ONE_DAY_IN_DAYS,
        );

        self.vaulted_supply_3m_net_change.multi_insert_net_change(
            heights,
            dates,
            &mut self.vaulted_supply,
            THREE_MONTHS_IN_DAYS,
        );

        self.vaulting_rate.multi_insert_simple_transform(
            heights,
            dates,
            &mut self.vaulted_supply,
            &|vaulted_supply| vaulted_supply * ONE_YEAR_IN_DAYS as f32,
        );

        self.active_supply.multi_insert_multiply(
            heights,
            dates,
            &mut self.liveliness,
            circulating_supply,
        );

        self.active_supply_net_change.multi_insert_net_change(
            heights,
            dates,
            &mut self.active_supply,
            ONE_DAY_IN_DAYS,
        );

        self.active_supply_3m_net_change.multi_insert_net_change(
            heights,
            dates,
            &mut self.active_supply,
            THREE_MONTHS_IN_DAYS,
        );

        // TODO: Do these
        // let min_vaulted_supply = ;
        // let max_active_supply = ;

        self.cointime_adjusted_inflation_rate.multi_insert_multiply(
            dates,
            &mut self.activity_to_vaultedness_ratio.date,
            inflation_rate,
        );

        self.cointime_adjusted_yearly_inflation_rate
            .multi_insert_multiply(
                dates,
                &mut self.activity_to_vaultedness_ratio.date,
                yearly_inflation_rate,
            );

        self.cointime_adjusted_velocity.multi_insert_divide(
            dates,
            annualized_transaction_volume,
            &mut self.active_supply.date,
        );

        // TODO:
        // const activeSupplyChangeFromTransactions90dChange =
        //     createNetChangeLazyDataset(activeSupplyChangeFromTransactions, 90);
        //   const activeSupplyChangeFromIssuance = createMultipliedLazyDataset(
        //     lastSubsidy,
        //     liveliness,
        //   );

        self.thermo_cap.multi_insert_simple_transform(
            heights,
            dates,
            cumulative_subsidy_in_dollars,
            &|cumulative_subsidy_in_dollars| cumulative_subsidy_in_dollars,
        );

        self.investor_cap
            .multi_insert_subtract(heights, dates, realized_cap, &mut self.thermo_cap);

        self.thermo_cap_to_investor_cap_ratio
            .multi_insert_percentage(heights, dates, &mut self.thermo_cap, &mut self.investor_cap);

        // TODO:
        // const activeSupplyChangeFromIssuance90dChange = createNetChangeLazyDataset(
        //   activeSupplyChangeFromIssuance,
        //   90,
        // );

        self.active_price
            .multi_insert_divide(heights, dates, realized_price, &mut self.liveliness);

        self.active_cap.height.multi_insert_multiply(
            heights,
            &mut self.active_supply.height,
            &mut closes.height,
        );
        self.active_cap.date.multi_insert_multiply(
            dates,
            &mut self.active_supply.date,
            &mut closes.date,
        );

        self.vaulted_price.multi_insert_divide(
            heights,
            dates,
            realized_price,
            &mut self.vaultedness,
        );

        self.vaulted_cap.height.multi_insert_multiply(
            heights,
            &mut self.vaulted_supply.height,
            &mut closes.height,
        );

        self.vaulted_cap.date.multi_insert_multiply(
            dates,
            &mut self.vaulted_supply.date,
            &mut closes.date,
        );

        self.true_market_mean.multi_insert_divide(
            heights,
            dates,
            &mut self.investor_cap,
            &mut self.active_supply,
        );

        self.true_market_deviation.multi_insert_divide(
            heights,
            dates,
            &mut self.active_cap,
            &mut self.investor_cap,
        );

        self.true_market_net_unrealized_profit_and_loss
            .height
            .multi_insert_complex_transform(
                heights,
                &mut self.active_cap.height,
                |(active_cap, height, ..)| {
                    let investor_cap = self.investor_cap.height.get(height).unwrap();

                    (active_cap - investor_cap) / active_cap
                },
            );
        self.true_market_net_unrealized_profit_and_loss
            .date
            .multi_insert_complex_transform(
                dates,
                &mut self.active_cap.date,
                |(active_cap, date, _, _)| {
                    let investor_cap = self.investor_cap.date.get(date).unwrap();
                    (active_cap - investor_cap) / active_cap
                },
            );

        self.investorness
            .multi_insert_divide(heights, dates, &mut self.investor_cap, realized_cap);

        self.producerness
            .multi_insert_divide(heights, dates, &mut self.thermo_cap, realized_cap);

        self.cointime_value_destroyed.multi_insert_multiply(
            heights,
            &mut self.coinblocks_destroyed,
            &mut closes.height,
        );
        self.cointime_value_destroyed_1d_sum.multi_insert_multiply(
            dates,
            &mut self.coinblocks_destroyed_1d_sum,
            &mut closes.date,
        );

        self.cointime_value_created.multi_insert_multiply(
            heights,
            &mut self.coinblocks_created,
            &mut closes.height,
        );
        self.cointime_value_created_1d_sum.multi_insert_multiply(
            dates,
            &mut self.coinblocks_created_1d_sum,
            &mut closes.date,
        );

        self.cointime_value_stored.multi_insert_multiply(
            heights,
            &mut self.coinblocks_stored,
            &mut closes.height,
        );
        self.cointime_value_stored_1d_sum.multi_insert_multiply(
            dates,
            &mut self.coinblocks_stored_1d_sum,
            &mut closes.date,
        );

        self.total_cointime_value_created
            .height
            .multi_insert_cumulative(heights, &mut self.cointime_value_created);
        self.total_cointime_value_created
            .date
            .multi_insert_cumulative(dates, &mut self.cointime_value_created_1d_sum);

        self.total_cointime_value_destroyed
            .height
            .multi_insert_cumulative(heights, &mut self.cointime_value_destroyed);
        self.total_cointime_value_destroyed
            .date
            .multi_insert_cumulative(dates, &mut self.cointime_value_destroyed_1d_sum);

        self.total_cointime_value_stored
            .height
            .multi_insert_cumulative(heights, &mut self.cointime_value_stored);
        self.total_cointime_value_stored
            .date
            .multi_insert_cumulative(dates, &mut self.cointime_value_stored_1d_sum);

        self.cointime_price.multi_insert_divide(
            heights,
            dates,
            &mut self.total_cointime_value_destroyed,
            &mut self.cumulative_coinblocks_stored,
        );

        self.cointime_cap.multi_insert_multiply(
            heights,
            dates,
            &mut self.cointime_price,
            circulating_supply,
        );

        self.active_price_ratio
            .compute(compute_data, closes, &mut self.active_price);

        self.cointime_price_ratio
            .compute(compute_data, closes, &mut self.cointime_price);

        self.true_market_mean_ratio
            .compute(compute_data, closes, &mut self.true_market_mean);

        self.vaulted_price_ratio
            .compute(compute_data, closes, &mut self.vaulted_price);
    }
}

impl AnyDataset for CointimeDataset {
    fn get_min_initial_states(&self) -> &MinInitialStates {
        &self.min_initial_states
    }
}
